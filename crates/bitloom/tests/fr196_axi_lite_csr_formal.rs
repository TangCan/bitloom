//! Story127.3 ATDD: real emitted RTL, port-only oracle, separate prove/cover and synthesis.
//! Dedicated tool gates
//! use --ignored; an ordinary workspace ignored result is not formal evidence.
use bitloom_prelude::{Elaboratable, ip::AxiLiteCsrBridge};
use serde_json::{Value, json};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Instant,
};

fn run(dir: &Path, tool: &str, args: &[&str], label: &str) {
    let log = fs::File::create(dir.join(format!("{label}.log"))).unwrap();
    let start = Instant::now();
    let status = Command::new("timeout")
        .args(["--kill-after=5s", "180s", tool])
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .expect("required formal/synthesis runner");
    fs::write(
        dir.join(format!("{label}-execution.json")),
        serde_json::to_string_pretty(&json!({
            "tool":tool,"args":args,"timeout_seconds":180,"exit_code":status.code(),
            "elapsed_seconds":start.elapsed().as_secs_f64(),"success":status.success()
        }))
        .unwrap(),
    )
    .unwrap();
    assert!(
        status.success(),
        "{tool} {args:?}: {status}; artifacts={}",
        dir.display()
    );
}
fn design(stage: &str) -> (PathBuf, String, String) {
    let hir = AxiLiteCsrBridge::elaborate().unwrap();
    let top = hir.circuit().modules[0].name.clone();
    assert_eq!(top, "AxiLiteCsrBridge");
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr196-bridge-formal")
        .join(format!("{stage}-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let rtl = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(dir.join("design.v"), &rtl).unwrap();
    (dir, top, rtl)
}

// Oracle storage is written only by external handshakes. No knowledge of the
// DUT FSM or internal signal names. Counts are modular supplements to exact slots.
const OBSERVER: &str = r#"
reg f_past = 0;
reg f_aw = 0, f_w = 0, f_ar = 0;
reg [15:0] f_awaddr, f_araddr;
reg [31:0] f_wdata;
reg [3:0] f_wstrb;
reg f_exec = 0, f_owner = 0, f_prefer_write = 0;
reg f_b = 0, f_r = 0;
reg [1:0] f_bresp, f_rresp;
reg [31:0] f_rdata;
reg [31:0] f_naw=0, f_nw=0, f_nar=0, f_nwr=0, f_nrd=0;
reg [31:0] f_caw=0, f_cw=0, f_car=0;
reg [31:0] f_submit=0, f_rsp=0, f_cexec=0;
reg [31:0] f_nbmake=0, f_nrmake=0, f_nb=0, f_nr=0, f_cb=0, f_cr=0;
reg f_seen_contention=0, f_read_during_bstall=0, f_write_during_rstall=0, f_seen_cancel=0;
reg f_nonzero_write_slverr=0, f_nonzero_write_decerr=0;
wire f_awtake=s_axi_awvalid && s_axi_awready;
wire f_wtake=s_axi_wvalid && s_axi_wready;
wire f_artake=s_axi_arvalid && s_axi_arready;
wire f_commit=csr_req_valid && csr_req_ready;
wire f_response=csr_rsp_valid && csr_rsp_ready;
wire f_btake=s_axi_bvalid && s_axi_bready;
wire f_rtake=s_axi_rvalid && s_axi_rready;
wire f_we=f_aw && f_w && !f_b;
wire f_re=f_ar && !f_r;
always @(posedge clk) begin
  f_past <= 1;
  if (!f_past) assume(rst);
  if (f_past) begin
    // Strengthening for this conservative implementation: these are proved
    // port-level obligations, never assumptions about the bridge. They expose
    // dormant occupancy to induction even during unbounded AXI/CSR stalls.
    assert(s_axi_awready == !f_aw);
    assert(s_axi_wready == !f_w);
    assert(s_axi_arready == !f_ar);
    assert(s_axi_bvalid == f_b);
    assert(s_axi_rvalid == f_r);
    assert(csr_rsp_ready == (f_exec && !rst));
    if (f_exec) begin
      if (f_owner) assert(!f_b); else assert(!f_r);
    end
    // Independent accounting includes partial AW/W cancellation, not fake writes.
    assert(f_naw == f_nwr + f_caw + f_aw);
    assert(f_nw == f_nwr + f_cw + f_w);
    assert(f_nar == f_nrd + f_car + f_ar);
    assert(f_submit == f_rsp + f_cexec + f_exec);
    assert(f_nbmake == f_nb + f_cb + f_b);
    assert(f_nrmake == f_nr + f_cr + f_r);
    if ($past(rst)) begin
      assert(!s_axi_bvalid && !s_axi_rvalid);
      assert(!f_aw && !f_w && !f_ar && !f_exec && !f_b && !f_r);
    end
    if (!rst) begin
      // Legal AXI producers: only these input holds are assumptions.
      if ($past(!rst && s_axi_awvalid && !s_axi_awready)) begin
        assume(s_axi_awvalid);
        assume({s_axi_awaddr,s_axi_awprot} == $past({s_axi_awaddr,s_axi_awprot}));
      end
      if ($past(!rst && s_axi_wvalid && !s_axi_wready)) begin
        assume(s_axi_wvalid);
        assume({s_axi_wdata,s_axi_wstrb} == $past({s_axi_wdata,s_axi_wstrb}));
      end
      if ($past(!rst && s_axi_arvalid && !s_axi_arready)) begin
        assume(s_axi_arvalid);
        assume({s_axi_araddr,s_axi_arprot} == $past({s_axi_araddr,s_axi_arprot}));
      end
      // Causal single response peer; no response-latency bound in safety.
      // No restrictions on BREADY/RREADY (arbitrarily long AXI backpressure).
      if (!f_exec) assume(!csr_rsp_valid);
      if (csr_rsp_valid) begin
        assume(csr_error != 2'b01);
        if (!f_owner && csr_error != 0) assume(csr_rdata == 0);
      end
      if ($past(!rst && csr_rsp_valid && !csr_rsp_ready)) begin
        assume(csr_rsp_valid);
        assume({csr_rdata,csr_error} == $past({csr_rdata,csr_error}));
      end
      // Independent next-edge progress: when no offer or execution occupies
      // the CSR interface and a captured class has response capacity, an offer
      // must appear on the next edge. This does not assume downstream readiness.
      if ($past(!rst && !f_exec && !csr_req_valid && (f_we || f_re)))
        assert(csr_req_valid);
      // Bridge obligations are ASSERTS, never environmental assumptions.
      if (f_awtake) assert(!f_aw);
      if (f_wtake) assert(!f_w);
      if (f_artake) assert(!f_ar);
      if (csr_req_valid) begin
        assert(!f_exec);
        if (csr_write) begin
          assert(f_aw && f_w && !f_b);
          assert({csr_addr,csr_wdata,csr_wstrb} == {f_awaddr,f_wdata,f_wstrb});
        end else begin
          assert(f_ar && !f_r);
          assert(csr_addr == f_araddr);
        end
        // Arbitration is checked on first observed offer only; later arrivals
        // cannot change it. Eligibility is measured before the offer appeared.
        if (!$past(!rst && csr_req_valid && !csr_req_ready) &&
            $past(!rst && !f_exec && f_we && f_re))
          assert(csr_write == f_prefer_write);
      end
      if ($past(!rst && csr_req_valid && !csr_req_ready)) begin
        assert(csr_req_valid);
        assert({csr_write,csr_addr,csr_wdata,csr_wstrb} ==
               $past({csr_write,csr_addr,csr_wdata,csr_wstrb}));
      end
      if (f_commit) assert(!f_exec);
      if (f_response) begin
        assert(f_exec);
        if (f_owner) assert(!f_b); else assert(!f_r);
      end
      if (s_axi_bvalid) begin assert(f_b); assert(s_axi_bresp == f_bresp); end
      if (s_axi_rvalid) begin
        assert(f_r); assert({s_axi_rdata,s_axi_rresp} == {f_rdata,f_rresp});
      end
      if ($past(!rst && s_axi_bvalid && !s_axi_bready)) begin
        assert(s_axi_bvalid); assert(s_axi_bresp == $past(s_axi_bresp));
      end
      if ($past(!rst && s_axi_rvalid && !s_axi_rready)) begin
        assert(s_axi_rvalid);
        assert({s_axi_rdata,s_axi_rresp} == $past({s_axi_rdata,s_axi_rresp}));
      end
      cover(f_seen_contention && f_nwr != 0 && f_nrd != 0 && f_nb != 0 && f_nr != 0);
      cover(f_read_during_bstall && f_btake);
      cover(f_write_during_rstall && f_rtake);
      cover(f_seen_cancel && f_commit);
      cover(f_b && f_r && !s_axi_bready && !s_axi_rready);
      // Each witness must consume a causal failed WRITE response with nonzero
      // ignored RDATA, then consume that same transaction's correct B response.
      cover(f_nonzero_write_slverr && f_btake && s_axi_bresp == 2'b10);
      cover(f_nonzero_write_decerr && f_btake && s_axi_bresp == 2'b11);
    end
  end
  if (rst) begin
    if (f_aw || f_w || f_ar || f_exec || f_b || f_r) f_seen_cancel <= 1;
    f_caw <= f_caw + f_aw; f_cw <= f_cw + f_w; f_car <= f_car + f_ar;
    f_cexec <= f_cexec + f_exec; f_cb <= f_cb + f_b; f_cr <= f_cr + f_r;
    f_aw<=0; f_w<=0; f_ar<=0; f_exec<=0; f_owner<=0;
    f_b<=0; f_r<=0; f_prefer_write<=0;
    f_seen_contention<=0; f_read_during_bstall<=0; f_write_during_rstall<=0;
    f_nonzero_write_slverr<=0; f_nonzero_write_decerr<=0;
  end else begin
    if (f_we && f_re && !f_exec) f_seen_contention<=1;
    // Witness opposite-class progress DURING this blocked response, then its
    // recovery. Clear on consumption so earlier traffic cannot satisfy a cover.
    if (s_axi_bvalid && !s_axi_bready && f_commit && !csr_write)
      f_read_during_bstall<=1;
    if (s_axi_rvalid && !s_axi_rready && f_commit && csr_write)
      f_write_during_rstall<=1;
    if (f_btake) f_read_during_bstall<=0;
    if (f_rtake) f_write_during_rstall<=0;
    if (f_awtake) begin f_aw<=1; f_awaddr<=s_axi_awaddr; f_naw<=f_naw+1; end
    if (f_wtake) begin f_w<=1; f_wdata<=s_axi_wdata; f_wstrb<=s_axi_wstrb; f_nw<=f_nw+1; end
    if (f_artake) begin f_ar<=1; f_araddr<=s_axi_araddr; f_nar<=f_nar+1; end
    if (f_commit) begin
      f_exec<=1; f_owner<=csr_write; f_prefer_write<=!csr_write;
      f_submit<=f_submit+1;
      if (csr_write) begin f_aw<=0; f_w<=0; f_nwr<=f_nwr+1; end
      else begin f_ar<=0; f_nrd<=f_nrd+1; end
    end
    if (f_response) begin
      f_exec<=0; f_rsp<=f_rsp+1;
      if (f_exec && f_owner) begin
        f_nonzero_write_slverr <= csr_error == 2'b10 && csr_rdata != 0;
        f_nonzero_write_decerr <= csr_error == 2'b11 && csr_rdata != 0;
      end
      if (f_owner) begin f_b<=1; f_bresp<=csr_error; f_nbmake<=f_nbmake+1; end
      else begin f_r<=1; f_rresp<=csr_error; f_rdata<=csr_rdata; f_nrmake<=f_nrmake+1; end
    end
    if (f_btake) begin
      f_b<=0; f_nb<=f_nb+1;
      f_nonzero_write_slverr<=0; f_nonzero_write_decerr<=0;
    end
    if (f_rtake) begin f_r<=0; f_nr<=f_nr+1; end
  end
end
"#;

// Separate implementation correspondence lemmas strengthen induction. The
// reference state above is still updated exclusively from port handshakes;
// no DUT state influences its expected payloads or accounting. Every lemma is
// an assertion checked by the same basecase and induction, never an assumption.
const INDUCTION_LEMMAS: &str = r#"
always @(posedge clk) if (f_past) begin
  assert(aw_full == f_aw && w_full == f_w && ar_full == f_ar);
  if (f_aw) assert(aw_addr == f_awaddr);
  if (f_w) assert({w_data,w_strb} == {f_wdata,f_wstrb});
  if (f_ar) assert(ar_addr == f_araddr);
  assert(exec == f_exec);
  if (f_exec) assert(owner == f_owner);
  assert(prefer_write == f_prefer_write);
  assert(b_full == f_b && r_full == f_r);
  if (f_b) assert(b_error == f_bresp);
  if (f_r) assert({r_data,r_error} == {f_rdata,f_rresp});
  if (offer) begin
    assert(!exec);
    if (offer_write) begin
      assert(f_aw && f_w && !f_b);
      assert({offer_data,offer_strb} == {f_wdata,f_wstrb});
    end else assert(f_ar && !f_r);
  end
end
"#;

#[test]
#[ignore = "requires implemented bridge and real SBY/Yosys/Z3; CI must explicitly run --ignored"]
fn p0_port_oracle_safety_induction_and_nonvacuous_covers() {
    let (dir, top, rtl) = design("proof");
    assert_eq!(rtl.matches("endmodule").count(), 1);
    fs::write(dir.join("observer.sv"), OBSERVER).unwrap();
    fs::write(dir.join("induction-lemmas.sv"), INDUCTION_LEMMAS).unwrap();
    fs::write(
        dir.join("design_formal.v"),
        rtl.replace(
            "endmodule",
            &format!("{OBSERVER}\n{INDUCTION_LEMMAS}\nendmodule"),
        ),
    )
    .unwrap();
    fs::write(dir.join("bridge.sby"), format!("[tasks]\nprove\ncover\n[options]\nprove: mode prove\ncover: mode cover\nprove: depth 8\ncover: depth 64\n[engines]\nsmtbmc --unroll z3\n[script]\nread -formal design_formal.v\nprep -top {top}\n[files]\ndesign_formal.v\n")).unwrap();
    for (tool, arg) in [("yosys", "-V"), ("sby", "--version"), ("z3", "-version")] {
        run(&dir, tool, &[arg], &format!("{tool}-version"));
    }
    for task in ["prove", "cover"] {
        run(&dir, "sby", &["-f", "bridge.sby", task], task);
        let status = fs::read_to_string(dir.join(format!("bridge_{task}/status"))).unwrap();
        assert_eq!(
            status.split_whitespace().next(),
            Some("PASS"),
            "{task}: {status}"
        );
    }
}

fn bit(v: &Value) -> Option<u64> {
    if let Some(b) = v.as_u64() {
        Some(b)
    } else {
        assert!(
            matches!(v.as_str(), Some("0" | "1")),
            "unknown/nonbinary structural constant {v}"
        );
        None
    }
}
fn path_to_input(
    b: u64,
    inputs: &HashSet<u64>,
    drivers: &HashMap<u64, Vec<u64>>,
    seen: &mut HashSet<u64>,
) -> bool {
    if inputs.contains(&b) {
        return true;
    }
    assert!(seen.insert(b), "combinational cycle {b}");
    let sources = drivers
        .get(&b)
        .unwrap_or_else(|| panic!("undriven bit {b}"));
    let found = sources
        .iter()
        .any(|s| path_to_input(*s, inputs, drivers, seen));
    seen.remove(&b);
    found
}
fn registered(json_text: &str, top: &str) -> usize {
    let value: Value = serde_json::from_str(json_text).unwrap();
    let module = &value["modules"][top];
    let ports = module["ports"].as_object().unwrap();
    let inputs: HashSet<u64> = ports
        .values()
        .filter(|p| p["direction"] == "input")
        .flat_map(|p| p["bits"].as_array().unwrap().iter().filter_map(bit))
        .collect();
    let clock = &ports["clk"];
    assert_eq!(clock["direction"], "input", "dedicated clk must be input");
    let clock_bits = clock["bits"].as_array().expect("dedicated clk bits");
    assert_eq!(clock_bits.len(), 1, "dedicated clk must be one bit");
    let clock_bit = bit(&clock_bits[0]).expect("clk cannot be constant");
    let mut drivers = HashMap::new();
    let mut ff_count = 0;
    for cell in module["cells"].as_object().unwrap().values() {
        let kind = cell["type"].as_str().unwrap();
        // Exact list: async-reset FF, latches, memories and unknown variants fail closed.
        let ff = matches!(
            kind,
            "$_DFF_P_"
                | "$_DFF_N_"
                | "$_DFFE_PP_"
                | "$_DFFE_PN_"
                | "$_DFFE_NP_"
                | "$_DFFE_NN_"
                | "$_SDFF_PP0_"
                | "$_SDFF_PP1_"
                | "$_SDFF_PN0_"
                | "$_SDFF_PN1_"
                | "$_SDFFE_PP0P_"
                | "$_SDFFE_PP0N_"
                | "$_SDFFE_PP1P_"
                | "$_SDFFE_PP1N_"
                | "$_SDFFCE_PP0P_"
                | "$_SDFFCE_PP0N_"
                | "$_SDFFCE_PP1P_"
                | "$_SDFFCE_PP1N_"
        );
        let comb = matches!(
            kind,
            "$_BUF_"
                | "$_NOT_"
                | "$_AND_"
                | "$_NAND_"
                | "$_OR_"
                | "$_NOR_"
                | "$_XOR_"
                | "$_XNOR_"
                | "$_ANDNOT_"
                | "$_ORNOT_"
                | "$_MUX_"
                | "$_NMUX_"
        );
        assert!(ff || comb, "unsupported/latch/unknown cell {kind}");
        ff_count += usize::from(ff);
        let connections = cell["connections"].as_object().unwrap();
        let sources = if ff {
            assert_eq!(
                cell["port_directions"]["C"], "input",
                "FF missing input clock port"
            );
            let clocks = connections
                .get("C")
                .and_then(Value::as_array)
                .expect("FF missing C connection");
            assert_eq!(clocks.len(), 1, "FF clock must have one bit");
            assert_eq!(
                bit(&clocks[0]),
                Some(clock_bit),
                "FF clock must be the dedicated clk input"
            );
            vec![]
        } else {
            connections
                .iter()
                .filter(|(p, _)| cell["port_directions"][*p] == "input")
                .flat_map(|(_, b)| b.as_array().unwrap().iter().filter_map(bit))
                .collect()
        };
        for (_, bits) in connections
            .iter()
            .filter(|(p, _)| cell["port_directions"][*p] == "output")
        {
            for b in bits.as_array().unwrap().iter().filter_map(bit) {
                assert!(
                    drivers.insert(b, sources.clone()).is_none(),
                    "multiple driver {b}"
                );
            }
        }
    }
    assert!(ff_count > 0, "no sequential state");
    for name in [
        "s_axi_awready",
        "s_axi_wready",
        "s_axi_arready",
        "s_axi_bvalid",
        "s_axi_rvalid",
    ] {
        for b in ports[name]["bits"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(bit)
        {
            assert!(
                !path_to_input(b, &inputs, &drivers, &mut HashSet::new()),
                "input (including rst) path to {name}"
            );
        }
    }
    ff_count
}
#[test]
#[ignore = "requires implemented bridge and real Yosys; CI must explicitly run --ignored"]
fn p0_original_rtl_synthesis_check_and_all_five_registered_boundaries() {
    let (dir, top, _) = design("synthesis");
    run(&dir, "yosys", &["-V"], "yosys-version");
    let script = format!(
        "read_verilog design.v; hierarchy -check -top {top}; proc; check -assert; synth -top {top}; check -assert; stat; write_json synthesis.json"
    );
    run(&dir, "yosys", &["-p", &script], "synthesis");
    let count = registered(
        &fs::read_to_string(dir.join("synthesis.json")).unwrap(),
        &top,
    );
    fs::write(
        dir.join("boundary-result.json"),
        json!({"registered_outputs":5,"flip_flops":count,"original_rtl_only":true}).to_string(),
    )
    .unwrap();
}

#[test]
fn p1_structural_observer_rejects_reset_paths_latches_unknowns_and_missing_drivers() {
    fn fixture(kind: &str, output: Value) -> String {
        json!({"modules":{"T":{"ports":{
            "rst":{"direction":"input","bits":[1]},
            "clk":{"direction":"input","bits":[3]},
            "s_axi_awready":{"direction":"output","bits":[output]},
            "s_axi_wready":{"direction":"output","bits":[2]},
            "s_axi_arready":{"direction":"output","bits":[2]},
            "s_axi_bvalid":{"direction":"output","bits":[2]},
            "s_axi_rvalid":{"direction":"output","bits":[2]}},
            "cells":{"ff":{"type":kind,"port_directions":{"C":"input","D":"input","Q":"output"},"connections":{"C":[3],"D":[1],"Q":[2]}}}}}}).to_string()
    }
    for (kind, output) in [
        ("$_DFF_P_", json!(1)),
        ("$_DLATCH_P_", json!(2)),
        ("mystery", json!(2)),
        ("$_DFF_FAKE_", json!(2)),
        ("$_DFF_P_", json!(9)),
        ("$_DFF_P_", json!("x")),
    ] {
        assert!(std::panic::catch_unwind(|| registered(&fixture(kind, output), "T")).is_err());
    }
    for connection in [Some(json!([1])), None] {
        let mut bad: Value = serde_json::from_str(&fixture("$_DFF_P_", json!(2))).unwrap();
        let ports = bad["modules"]["T"]["cells"]["ff"]["connections"]
            .as_object_mut()
            .unwrap();
        if let Some(connection) = connection {
            ports.insert("C".into(), connection);
        } else {
            ports.remove("C");
        }
        let failure = std::panic::catch_unwind(|| registered(&bad.to_string(), "T")).unwrap_err();
        let message = failure
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| failure.downcast_ref::<&str>().copied())
            .unwrap_or("");
        assert!(
            message.contains("FF clock must be the dedicated clk input")
                || message.contains("FF missing C connection"),
            "{message}"
        );
    }
    assert_eq!(registered(&fixture("$_DFF_P_", json!(2)), "T"), 1);
}

#[test]
#[ignore = "requires real SBY/Yosys/Z3; checks assertion counterexamples, not tool errors"]
fn p0_port_oracle_kills_pairing_routing_and_offer_hold_rtl_mutants() {
    let (dir, top, rtl) = design("mutations");
    for (label, from, to) in [
        ("control", "", ""),
        (
            "pairing",
            "assign csr_addr = (offer_write ? aw_addr : ar_addr);",
            "assign csr_addr = (offer_write ? ar_addr : ar_addr);",
        ),
        (
            "routing",
            "assign write_response = (response & owner);",
            "assign write_response = (response & read_owner);",
        ),
        (
            "offer-hold",
            "assign offer_next = (commit ? zero : offer_started);",
            "assign offer_next = (offer ? zero : offer_started);",
        ),
    ] {
        let case = dir.join(label);
        fs::create_dir_all(&case).unwrap();
        let source = if label == "control" {
            rtl.clone()
        } else {
            assert_eq!(
                rtl.matches(from).count(),
                1,
                "mutation must match exactly once"
            );
            rtl.replace(from, to)
        };
        // Mutation sensitivity is established using the independent port oracle
        // alone: implementation correspondence lemmas cannot kill a mutant.
        fs::write(
            case.join("design_formal.v"),
            source.replace("endmodule", &format!("{OBSERVER}\nendmodule")),
        )
        .unwrap();
        fs::write(case.join("mutation.sby"),format!("[options]\nmode bmc\ndepth 10\n[engines]\nsmtbmc --unroll z3\n[script]\nread -formal design_formal.v\nprep -top {top}\n[files]\ndesign_formal.v\n")).unwrap();
        let log = fs::File::create(case.join("mutation.log")).unwrap();
        let start = Instant::now();
        let status = Command::new("timeout")
            .args(["--kill-after=5s", "180s", "sby", "-f", "mutation.sby"])
            .current_dir(&case)
            .stdout(Stdio::from(log.try_clone().unwrap()))
            .stderr(Stdio::from(log))
            .status()
            .unwrap();
        fs::write(case.join("execution.json"),json!({"command":["timeout","--kill-after=5s","180s","sby","-f","mutation.sby"],"exit_code":status.code(),"elapsed_seconds":start.elapsed().as_secs_f64(),"oracle":"port-only; no implementation lemmas","depth":10}).to_string()).unwrap();
        let result = fs::read_to_string(case.join("mutation/status")).unwrap();
        if label == "control" {
            assert_eq!(status.code(), Some(0));
            assert_eq!(result.split_whitespace().next(), Some("PASS"));
        } else {
            assert_eq!(
                status.code(),
                Some(2),
                "{label}: expected assertion FAIL, artifacts {}",
                case.display()
            );
            assert_eq!(result.split_whitespace().next(), Some("FAIL"));
            let log = fs::read_to_string(case.join("mutation.log")).unwrap();
            assert!(
                log.contains("Assert failed") && log.contains("Status: failed"),
                "{log}"
            );
            let trace = fs::read_to_string(case.join("mutation/engine_0/trace.vcd")).unwrap();
            assert!(trace.contains("$enddefinitions") && trace.contains("#0"));
        }
    }
}
