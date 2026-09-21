//! Story127.2: dedicated real CSR safety/cover and raw synthesis gates.
//! ATDD API compilation RED is not proof of any property below.
//! Run required dedicated gate with --ignored (matching FR195 CI layout).
use bitloom_prelude::ip::{CsrAccess, CsrBlock, CsrField, CsrOwner, CsrRegister};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

fn bank() -> CsrBlock {
    let mut registers = Vec::new();
    for (name, offset, access, owner, mask, event, read_reject, write_reject) in [
        (
            "control",
            0,
            CsrAccess::Rw,
            CsrOwner::Leaf,
            0x80ff00ff,
            None,
            false,
            false,
        ),
        (
            "status",
            4,
            CsrAccess::Ro,
            CsrOwner::External,
            0xff,
            None,
            true,
            false,
        ),
        (
            "tx",
            8,
            CsrAccess::Wo,
            CsrOwner::None,
            0xff,
            None,
            false,
            true,
        ),
        (
            "events",
            12,
            CsrAccess::W1c,
            CsrOwner::Leaf,
            0x8000000f,
            Some("hw_events"),
            false,
            false,
        ),
        (
            "counter",
            16,
            CsrAccess::Rw,
            CsrOwner::External,
            0xffffffff,
            None,
            false,
            true,
        ),
    ] {
        registers.push(CsrRegister {
            name: name.into(),
            offset,
            reset: 0,
            access,
            owner,
            event: event.map(str::to_owned),
            read_reject,
            write_reject,
            fields: vec![CsrField {
                name: "bits".into(),
                mask,
                reset: 0,
                access,
            }],
        });
    }
    for register in &mut registers {
        if matches!(register.name.as_str(), "control" | "events") {
            register.read_reject = true;
            register.write_reject = true;
        }
        if register.name == "counter" {
            register.read_reject = true;
        }
    }
    CsrBlock {
        name: "Probe".into(),
        registers,
    }
}

fn run(dir: &Path, tool: &str, args: &[&str], log_name: &str) {
    let log = fs::File::create(dir.join(log_name)).unwrap();
    let status = Command::new("timeout")
        .args(["--kill-after=5s", "180s", tool])
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .expect("required tool must exist; never skip");
    fs::write(dir.join(format!("{log_name}.exit")), format!("{status}\n")).unwrap();
    assert!(
        status.success(),
        "{tool} {args:?}: {status}; {}",
        dir.display()
    );
}

fn design(stage: &str) -> (PathBuf, String) {
    let hir = bank().elaborate("CsrFormal").unwrap();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr196-csr-formal")
        .join(format!("{stage}-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let source = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(dir.join("design.v"), &source).unwrap();
    (dir, source)
}

// Independent port-only reference. No DUT internal names, no ready fairness,
// no assumptions on DUT outputs. Any induction strengthening must be assertions.
const OBSERVER: &str = r#"
reg ref_started = 0;
reg ref_pending = 0;
reg [31:0] ref_control = 0, ref_events = 0, ref_data = 0;
reg [1:0] ref_error = 0;
reg ref_stalled = 0;
// Cover-only history survives the cancellation reset; no new assumptions.
reg ref_cancelled = 0;
wire ref_submit = req_valid && !ref_pending && !rst;
wire [31:0] ref_bytes = {{8{wstrb[3]}}, {8{wstrb[2]}}, {8{wstrb[1]}}, {8{wstrb[0]}}};
reg [1:0] ref_next_error;
reg [31:0] ref_next_data;
reg ref_rd_control, ref_rd_status, ref_rd_events, ref_rd_counter;
reg ref_wr_control, ref_wr_tx, ref_wr_events, ref_wr_counter;
always @* begin
  ref_next_error = 0;
  ref_next_data = 0;
  ref_rd_control=0; ref_rd_status=0; ref_rd_events=0; ref_rd_counter=0;
  ref_wr_control=0; ref_wr_tx=0; ref_wr_events=0; ref_wr_counter=0;
  case(addr)
    16'h0000: if(write) begin
                if ((|(ref_bytes & 32'h80ff00ff)) && control_write_reject) ref_next_error=2;
                else ref_wr_control=|(ref_bytes & 32'h80ff00ff);
              end else if(control_read_reject) ref_next_error=2;
              else begin ref_rd_control=1; ref_next_data=ref_control; end
    16'h0004: if(write || status_read_reject) ref_next_error=2;
              else begin ref_rd_status=1; ref_next_data=status_value & 32'hff; end
    16'h0008: if(!write || ((|(ref_bytes & 32'hff)) && tx_write_reject)) ref_next_error=2;
              else ref_wr_tx=|(ref_bytes & 32'hff);
    16'h000c: if(write) begin
                if ((|(ref_bytes & 32'h8000000f)) && events_write_reject) ref_next_error=2;
                else ref_wr_events=|(ref_bytes & 32'h8000000f);
              end else if(events_read_reject) ref_next_error=2;
              else begin ref_rd_events=1; ref_next_data=ref_events; end
    16'h0010: if(write) begin
                if((|ref_bytes) && counter_write_reject) ref_next_error=2;
                else ref_wr_counter=|ref_bytes;
              end else if(counter_read_reject) ref_next_error=2;
              else begin ref_rd_counter=1; ref_next_data=counter_value; end
    default: ref_next_error=2;
  endcase
end
always @(posedge clk) begin
  ref_started <= 1;
  if(ref_started && rst && ref_pending) ref_cancelled <= 1;
  if(!ref_started) assume(rst);
  if(ref_started) begin
    assert(req_ready == (!ref_pending && !rst));
    assert(rsp_valid == ref_pending);
    if(ref_pending) begin assert(rdata==ref_data); assert(error==ref_error); end
    assert(control_value==ref_control);
    assert(events_value==ref_events);
    assert(control_read_commit == (ref_submit && ref_rd_control));
    assert(status_read_commit == (ref_submit && ref_rd_status));
    assert(events_read_commit == (ref_submit && ref_rd_events));
    assert(counter_read_commit == (ref_submit && ref_rd_counter));
    assert(!tx_read_commit && !status_write_commit);
    assert(control_write_commit == (ref_submit && ref_wr_control));
    assert(tx_write_commit == (ref_submit && ref_wr_tx));
    assert(events_write_commit == (ref_submit && ref_wr_events));
    assert(counter_write_commit == (ref_submit && ref_wr_counter));
    assert(control_write_mask==(ref_bytes & 32'h80ff00ff));
    assert(tx_write_mask==(ref_bytes & 32'hff));
    assert(events_write_mask==(ref_bytes & 32'h8000000f));
    assert(counter_write_mask==ref_bytes);
    assert(counter_candidate==((counter_value & ~ref_bytes) | (wdata & ref_bytes)));
    assert(control_candidate==(((ref_control & ~ref_bytes) | (wdata & ref_bytes)) & 32'h80ff00ff));
    assert(events_candidate==(wdata & ref_bytes & 32'h8000000f));
    assert(tx_candidate==(wdata & ref_bytes & 32'hff));
    if($past(rsp_valid && !rsp_ready && !rst)) begin
      assert(rsp_valid); assert(rdata==$past(rdata)); assert(error==$past(error));
    end
    if($past(req_valid && !req_ready && !rst) && !rst) begin
      assume(req_valid);
      assume(write==$past(write)); assume(addr==$past(addr));
      assume(wdata==$past(wdata)); assume(wstrb==$past(wstrb));
    end
    cover(ref_submit && ref_wr_control);
    cover(ref_submit && ref_next_error==2);
    cover(ref_cancelled && ref_submit && ref_next_error==0);
    cover(ref_stalled && rsp_valid && rsp_ready && !rst);
    cover(ref_submit && ref_wr_events && |(hw_events & wdata & ref_bytes & 32'h8000000f));
  end
  if(rst) begin
    ref_pending<=0; ref_control<=0; ref_events<=0; ref_data<=0; ref_error<=0; ref_stalled<=0;
  end else begin
    if(ref_pending && !rsp_ready) ref_stalled<=1;
    if(ref_pending && rsp_ready) begin ref_pending<=0; ref_stalled<=0; end
    if(ref_submit) begin
      ref_pending<=1; ref_data<=ref_next_data; ref_error<=ref_next_error;
    end
    if(ref_submit && ref_wr_control)
      ref_control<=((ref_control & ~ref_bytes) | (wdata & ref_bytes)) & 32'h80ff00ff;
    ref_events <= (ref_events & ~(ref_submit && ref_wr_events ? (wdata & ref_bytes) : 32'b0))
                   | (hw_events & 32'h8000000f);
  end
end
"#;

#[test]
#[ignore = "real SBY/Yosys/Z3 required; strict dedicated gate runs --ignored"]
fn p0_csr_real_safety_and_nonvacuous_cover() {
    let (dir, source) = design("proof");
    assert_eq!(source.matches("endmodule").count(), 1);
    fs::write(dir.join("observer.sv"), OBSERVER).unwrap();
    fs::write(
        dir.join("design_formal.v"),
        source.replace("endmodule", &format!("{OBSERVER}\nendmodule")),
    )
    .unwrap();
    fs::write(dir.join("csr.sby"), "[tasks]\nprove\ncover\n[options]\nprove: mode prove\ncover: mode cover\nprove: depth 16\ncover: depth 24\n[engines]\nsmtbmc --unroll z3\n[script]\nread -formal design_formal.v\nprep -top CsrFormal\n[files]\ndesign_formal.v\n").unwrap();
    for (tool, arg) in [("yosys", "-V"), ("sby", "--version"), ("z3", "-version")] {
        run(&dir, tool, &[arg], &format!("{tool}-version.log"));
    }
    fs::write(dir.join("commands.log"), "timeout --kill-after=5s 180s sby -f csr.sby prove\ntimeout --kill-after=5s 180s sby -f csr.sby cover\n").unwrap();
    for task in ["prove", "cover"] {
        run(
            &dir,
            "sby",
            &["-f", "csr.sby", task],
            &format!("{task}.log"),
        );
        let status = fs::read_to_string(dir.join(format!("csr_{task}/status"))).unwrap();
        assert_eq!(
            status.split_whitespace().next(),
            Some("PASS"),
            "{task}: {status}"
        );
    }
    println!("FR196 CSR proof/cover artifacts={}", dir.display());
}

#[test]
#[ignore = "real SBY/Yosys/Z3 required; strict dedicated gate runs --ignored"]
fn p0_csr_formal_rejects_lost_event_output_with_counterexample() {
    // [P0] Prove the original and falsify an actual generated-RTL mutation
    // with the same port-only oracle. No assumption or reference-model change
    // may hide the loss of a naturally arriving hardware event.
    let (dir, source) = design("lost-event-mutation");
    assert_eq!(source.matches("endmodule").count(), 1);
    let assignments: Vec<_> = source
        .lines()
        .filter(|line| line.trim_start().starts_with("assign events_value = "))
        .collect();
    assert_eq!(
        assignments.len(),
        1,
        "mutation must touch exactly one output"
    );
    let mutant = source.replacen(assignments[0], "  assign events_value = 32'b0;", 1);
    assert_ne!(mutant, source, "mutation must change the DUT");
    let property = "csr_events_value_matches_reference";
    let assertion = "assert(events_value==ref_events);";
    assert_eq!(OBSERVER.matches(assertion).count(), 1);
    let observer = OBSERVER.replace(assertion, &format!("{property}: {assertion}"));
    fs::write(dir.join("observer.sv"), &observer).unwrap();
    fs::write(dir.join("mutant.v"), &mutant).unwrap();
    fs::write(
        dir.join("mutation.txt"),
        format!("Original: {}\nMutation: assign events_value = 32'b0;\nExpected failed property: {property}\n", assignments[0].trim()),
    )
    .unwrap();
    for (tool, arg) in [("yosys", "-V"), ("sby", "--version"), ("z3", "-version")] {
        run(&dir, tool, &[arg], &format!("{tool}-version.log"));
    }
    // Each task is force-recreated by SBY, so stale status/trace files cannot
    // satisfy the assertions even when a process ID is reused in a later run.
    for (name, rtl) in [("control", &source), ("mutant", &mutant)] {
        fs::write(
            dir.join(format!("{name}_formal.v")),
            rtl.replace("endmodule", &format!("{observer}\nendmodule")),
        )
        .unwrap();
        fs::write(
            dir.join(format!("{name}.sby")),
            format!("[options]\nmode prove\ndepth 16\n[engines]\nsmtbmc --unroll z3\n[script]\nread -formal {name}_formal.v\nprep -top CsrFormal\n[files]\n{name}_formal.v\n"),
        )
        .unwrap();
    }
    fs::write(dir.join("commands.log"), "timeout --kill-after=5s 180s sby -f control.sby\ntimeout --kill-after=5s 180s sby -f mutant.sby\n").unwrap();
    run(&dir, "sby", &["-f", "control.sby"], "control.log");
    let control_status = fs::read_to_string(dir.join("control/status")).unwrap();
    assert_eq!(control_status.split_whitespace().next(), Some("PASS"));

    let log = fs::File::create(dir.join("mutant.log")).unwrap();
    let exit = Command::new("timeout")
        .args(["--kill-after=5s", "180s", "sby", "-f", "mutant.sby"])
        .current_dir(&dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .expect("required tool must exist; missing tools are not a counterexample");
    fs::write(dir.join("mutant.log.exit"), format!("{exit}\n")).unwrap();
    // Pinned SBY assigns FAIL=2, ERROR=16; timeout/missing tool exits are
    // distinct too. A parser failure or an induction-only UNKNOWN is no kill.
    assert_eq!(exit.code(), Some(2), "expected SBY FAIL; {}", dir.display());
    let status = fs::read_to_string(dir.join("mutant/status")).unwrap();
    assert_eq!(status.split_whitespace().next(), Some("FAIL"));
    let log = fs::read_to_string(dir.join("mutant/engine_0/logfile_basecase.txt")).unwrap();
    assert!(
        log.lines()
            .any(|line| line.contains("Assert failed") && line.contains(property)),
        "the reachable basecase must fail the event-output property: {log}"
    );
    assert!(log.contains("Status: failed"), "{log}");
    let trace = fs::read_to_string(dir.join("mutant/engine_0/trace.vcd")).unwrap();
    assert!(
        !trace.is_empty(),
        "solver must emit a nonempty counterexample"
    );
    assert!(trace.contains("$enddefinitions") && trace.contains("#0"));
    assert!(trace.contains("events_value") && trace.contains("ref_events"));
    println!(
        "FR196 CSR original PASS / lost-event mutation FAIL artifacts={}",
        dir.display()
    );
}

#[test]
#[ignore = "real Yosys required; strict dedicated gate runs --ignored"]
fn p0_csr_original_rtl_synthesis_check() {
    let (dir, _) = design("synthesis");
    run(&dir, "yosys", &["-V"], "yosys-version.log");
    let script = "read_verilog design.v; hierarchy -check -top CsrFormal; proc; check -assert; synth -top CsrFormal; check -assert; stat; write_json synthesis.json";
    fs::write(
        dir.join("commands.log"),
        format!("timeout --kill-after=5s 180s yosys -p '{script}'\n"),
    )
    .unwrap();
    run(&dir, "yosys", &["-p", script], "synthesis.log");
    let netlist: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join("synthesis.json")).unwrap()).unwrap();
    let cells = netlist["modules"]["CsrFormal"]["cells"]
        .as_object()
        .unwrap();
    assert!(!cells.is_empty());
    assert!(
        cells
            .values()
            .any(|c| c["type"].as_str().unwrap().contains("DFF"))
    );
    assert!(
        !cells
            .values()
            .any(|c| c["type"].as_str().unwrap().contains("LATCH"))
    );
    println!(
        "FR196 CSR uninstrumented synthesis artifacts={}",
        dir.display()
    );
}

#[test]
#[ignore = "real Yosys required; strict dedicated gate runs --ignored"]
fn p0_each_supported_singleton_original_rtl_synthesis() {
    // All five supported access/owner shapes; no multi-register Probe observer
    // is inserted or claimed for these synthesis-only specialization checks.
    for (index, register) in bank().registers.into_iter().enumerate() {
        let stage = format!("singleton-{index}");
        let leaf_owned = register.owner == CsrOwner::Leaf;
        let hir = CsrBlock {
            name: "Singleton".into(),
            registers: vec![register],
        }
        .elaborate("SingletonCsr")
        .unwrap();
        // Three response registers, and exactly one additional register only
        // for a leaf-owned RW/W1C. External and WO do not duplicate state.
        assert_eq!(
            hir.circuit().modules[0]
                .body
                .iter()
                .filter(|stmt| matches!(stmt, bitloom_hir::Stmt::RegDecl { .. }))
                .count(),
            3 + usize::from(leaf_owned)
        );
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/fr196-csr-formal")
            .join(format!("{stage}-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let source = bitloom_vlog::emit(&hir)
            .files
            .iter()
            .map(|f| f.contents.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(!source.contains("ref_pending"));
        fs::write(dir.join("design.v"), source).unwrap();
        let script = "read_verilog design.v; hierarchy -check -top SingletonCsr; proc; check -assert; synth -top SingletonCsr; check -assert; stat; write_json synthesis.json";
        fs::write(
            dir.join("commands.log"),
            format!("timeout --kill-after=5s 180s yosys -p '{script}'\n"),
        )
        .unwrap();
        run(&dir, "yosys", &["-V"], "yosys-version.log");
        run(&dir, "yosys", &["-p", script], "synthesis.log");
        let netlist: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(dir.join("synthesis.json")).unwrap()).unwrap();
        let cells = netlist["modules"]["SingletonCsr"]["cells"]
            .as_object()
            .unwrap();
        assert!(
            cells
                .values()
                .any(|c| c["type"].as_str().unwrap().contains("DFF")),
            "response storage must remain"
        );
        assert!(
            !cells
                .values()
                .any(|c| c["type"].as_str().unwrap().contains("LATCH"))
        );
        println!(
            "FR196 singleton {index} original RTL synthesis PASS artifacts={}",
            dir.display()
        );
    }
}
