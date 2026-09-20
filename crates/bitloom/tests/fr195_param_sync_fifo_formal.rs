//! Story 126.4 / AC8: real emitted RTL proofs, non-vacuous covers and synthesis.
//! Dedicated tool tests must be run with --ignored; compilation is the API red gate.
use bitloom_prelude::{Elaboratable, ip::ParamSyncFifo};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

fn run(dir: &Path, tool: &str, args: &[&str], name: &str) {
    let log = fs::File::create(dir.join(name)).unwrap();
    let status = Command::new("timeout")
        .args(["--kill-after=5s", "180s", tool])
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .expect("required formal/synthesis runner must exist");
    assert!(
        status.success(),
        "{tool} {args:?}: {status}; artifacts={}",
        dir.display()
    );
}

fn design<const W: u32, const D: u32>(stage: &str) -> (PathBuf, String, String) {
    let hir = ParamSyncFifo::<W, D>::elaborate().unwrap();
    let top = hir.circuit().modules[0].name.clone();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr195-fifo-formal")
        .join(format!("{stage}-w{W}-d{D}-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let source = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(dir.join("design.v"), &source).unwrap();
    (dir, top, source)
}

// Reference state depends only on observed port handshakes. There are no internal
// assumptions, fairness assumptions, or reset-every-cycle constraints. Once the
// DUT exists, extra assertions (never assumptions) may strengthen induction.
fn observer(w: u32, d: u32) -> String {
    let simultaneous = if d > 1 {
        "cover(!rst && !flush && ref_pending > 0 && ref_pending < DEPTH && ref_push && ref_pop);"
    } else {
        "assert(!(ref_push && ref_pop));"
    };
    // Strengthen induction by relating every live slot to the independent
    // port-driven queue. These are proved assertions, never assumed state.
    let mut invariants = "assert(count == ref_pending);\n".to_owned();
    for i in 0..d {
        invariants.push_str(&format!(
            "if (ref_pending > {i}) assert(slot_{i} == ref_queue[{i}]);\n"
        ));
    }
    format!(
        r#"
  localparam DEPTH = {d};
  reg ref_past_valid = 0;
  integer ref_pending = 0;
  reg [{last}:0] ref_queue [0:DEPTH-1];
  reg ref_seen_full = 0;
  reg [31:0] ref_accepted = 0, ref_delivered = 0, ref_cancelled = 0;
  wire ref_push = input_valid && input_ready;
  wire ref_pop = output_valid && output_ready;
  integer ref_i;
  always @(posedge clk) begin
    ref_past_valid <= 1;
    if (!ref_past_valid) assume(rst);
    if (ref_past_valid) begin
      {invariants}
      assert(ref_pending >= 0 && ref_pending <= DEPTH);
      assert(input_ready == (ref_pending < DEPTH));
      assert(output_valid == (ref_pending != 0));
      if (output_valid) assert(output_data == ref_queue[0]);
      assert(!ref_push || ref_pending < DEPTH);
      assert(!ref_pop || ref_pending > 0);
      // Modular counters supplement the exact bounded queue occupancy/order proof.
      assert(ref_accepted == ref_delivered + ref_cancelled + ref_pending);
      if ($past(rst || flush)) begin
        assert(ref_pending == 0);
        assert(!output_valid);
        assert(input_ready);
      end
      if ($past(output_valid && !output_ready && !rst && !flush)) begin
        assert(output_valid);
        assert(output_data == $past(output_data));
      end
      // Protocol legal producer; cancellation releases the outstanding offer.
      if ($past(input_valid && !input_ready && !rst && !flush) && !rst && !flush) begin
        assume(input_valid);
        assume(input_data == $past(input_data));
      end
      cover(ref_seen_full && ref_pending == 0 && !rst && !flush);
      cover($past(ref_pending == DEPTH && rst) && ref_pending == 0 && !rst && !flush && ref_push);
      cover($past(ref_pending == DEPTH && !rst && flush) && ref_pending == 0 && !rst && !flush && ref_push);
      cover($past(ref_pending > 0 && rst && flush) && ref_pending == 0 && !rst && !flush && ref_push);
      {simultaneous}
    end
    if (rst || flush) begin
      ref_cancelled <= ref_cancelled + ref_pending;
      ref_pending <= 0;
      ref_seen_full <= 0;
    end else begin
      if (ref_pending == DEPTH) ref_seen_full <= 1;
      if (ref_pop) begin
        for (ref_i = 0; ref_i < DEPTH-1; ref_i = ref_i+1)
          ref_queue[ref_i] <= ref_queue[ref_i+1];
        ref_delivered <= ref_delivered + 1;
      end
      if (ref_push) begin
        ref_queue[ref_pending - (ref_pop ? 1 : 0)] <= input_data;
        ref_accepted <= ref_accepted + 1;
      end
      case ({{ref_push, ref_pop}})
        2'b10: ref_pending <= ref_pending + 1;
        2'b01: ref_pending <= ref_pending - 1;
      endcase
    end
  end
"#,
        last = w - 1
    )
}

fn formal<const D: u32>() {
    let (dir, top, source) = design::<1, D>("prove");
    let observer = observer(1, D);
    fs::write(dir.join("observer.sv"), &observer).unwrap();
    assert_eq!(source.matches("endmodule").count(), 1);
    fs::write(
        dir.join("design_formal.v"),
        source.replace("endmodule", &format!("{observer}\nendmodule")),
    )
    .unwrap();
    fs::write(dir.join("fifo.sby"), format!("[tasks]\nprove\ncover\n[options]\nprove: mode prove\ncover: mode cover\nprove: depth 12\ncover: depth 24\n[engines]\nsmtbmc --unroll z3\n[script]\nread -formal design_formal.v\nprep -top {top}\n[files]\ndesign_formal.v\n")).unwrap();
    for (tool, arg) in [("yosys", "-V"), ("sby", "--version"), ("z3", "-version")] {
        run(&dir, tool, &[arg], &format!("{tool}-version.log"));
    }
    fs::write(dir.join("commands.log"), "timeout --kill-after=5s 180s sby -f fifo.sby prove\ntimeout --kill-after=5s 180s sby -f fifo.sby cover\n").unwrap();
    for task in ["prove", "cover"] {
        run(
            &dir,
            "sby",
            &["-f", "fifo.sby", task],
            &format!("{task}.log"),
        );
        let status = fs::read_to_string(dir.join(format!("fifo_{task}/status"))).unwrap();
        assert_eq!(
            status.split_whitespace().next(),
            Some("PASS"),
            "{task}: {status}"
        );
    }
    println!(
        "FR195 FIFO W1 D{D} prove+cover PASS artifacts={}",
        dir.display()
    );
}

fn synthesis<const W: u32, const D: u32>() {
    let (dir, top, _) = design::<W, D>("synthesis");
    run(&dir, "yosys", &["-V"], "yosys-version.log");
    // Only the uninstrumented original RTL enters synthesis. check -assert catches
    // undriven/conflicting drivers before and after mapping; no PPA/BRAM claim.
    let script = format!(
        "read_verilog design.v; hierarchy -check -top {top}; proc; check -assert; synth -top {top}; check -assert; stat; write_json synthesis.json"
    );
    fs::write(
        dir.join("commands.log"),
        format!("timeout --kill-after=5s 180s yosys -p '{script}'\n"),
    )
    .unwrap();
    run(&dir, "yosys", &["-p", &script], "synthesis.log");
    let netlist: Value =
        serde_json::from_str(&fs::read_to_string(dir.join("synthesis.json")).unwrap()).unwrap();
    let module = &netlist["modules"][&top];
    let cells = module["cells"].as_object().unwrap();
    assert!(!cells.is_empty(), "FIFO must retain sequential state");
    let mut statistics: BTreeMap<String, usize> = BTreeMap::new();
    let mut drivers = BTreeMap::new();
    let mut ff_count = 0;
    for cell in cells.values() {
        let kind = cell["type"].as_str().unwrap();
        let ff = ["$_DFF_", "$_DFFE_", "$_SDFF_", "$_SDFFE_", "$_SDFFCE_"]
            .iter()
            .any(|prefix| kind.starts_with(prefix));
        let combinational = matches!(
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
        assert!(ff || combinational, "unsupported/latch/memory cell: {kind}");
        ff_count += usize::from(ff);
        *statistics.entry(kind.into()).or_default() += 1;
        for (port, bits) in cell["connections"].as_object().unwrap() {
            if cell["port_directions"][port] == "output" {
                for bit in bits.as_array().unwrap().iter().filter_map(Value::as_u64) {
                    assert!(
                        drivers.insert(bit, kind).is_none(),
                        "multiple drivers at {bit}"
                    );
                }
            }
        }
    }
    assert!(ff_count > 0, "FIFO must retain edge-triggered registers");
    fs::write(
        dir.join("cell-statistics.json"),
        serde_json::to_string_pretty(&json!({
            "width":W,"depth":D,"cells":cells.len(),"flip_flop_cells":ff_count,
            "cell_types":statistics,"original_rtl_only":true,"ppa_claim":false,"bram_claim":false
        }))
        .unwrap(),
    )
    .unwrap();
    println!(
        "FR195 FIFO synthesis W{W} D{D} cells={} PASS artifacts={}",
        cells.len(),
        dir.display()
    );
}

#[test]
#[ignore = "real SBY/Yosys/Z3 required; strict dedicated gate runs --ignored"]
fn p0_fifo_width1_depth1_safety_and_nonvacuous_cover() {
    formal::<1>();
}
#[test]
#[ignore = "real SBY/Yosys/Z3 required; strict dedicated gate runs --ignored"]
fn p0_fifo_width1_depth2_safety_and_nonvacuous_cover() {
    formal::<2>();
}
#[test]
#[ignore = "real SBY/Yosys/Z3 required; strict dedicated gate runs --ignored"]
fn p0_fifo_width1_depth3_safety_and_nonvacuous_cover() {
    formal::<3>();
}
#[test]
#[ignore = "real Yosys required; strict dedicated gate runs --ignored"]
fn p0_original_rtl_synthesis_width1_depth1() {
    synthesis::<1, 1>();
}
#[test]
#[ignore = "real Yosys required; strict dedicated gate runs --ignored"]
fn p0_original_rtl_synthesis_width8_depth3() {
    synthesis::<8, 3>();
}
#[test]
#[ignore = "real Yosys required; strict dedicated gate runs --ignored"]
fn p0_original_rtl_synthesis_width64_depth16() {
    synthesis::<64, 16>();
}
