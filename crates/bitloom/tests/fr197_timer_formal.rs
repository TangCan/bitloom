//! Story128.2: independent Timer32 recurrence proof, reachability, raw synthesis.
use bitloom_prelude::{Elaboratable, ip::Timer};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
fn run(dir: &Path, tool: &str, args: &[&str], label: &str) {
    let log = fs::File::create(dir.join(format!("{label}.log"))).unwrap();
    let status = Command::new("timeout")
        .args(["--kill-after=5s", "180s", tool])
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .unwrap();
    fs::write(dir.join(format!("{label}.exit")), format!("{status}\n")).unwrap();
    assert!(
        status.success(),
        "{tool} {args:?}: {status}; {}",
        dir.display()
    );
}
fn design(stage: &str) -> (PathBuf, String) {
    let hir = Timer::elaborate().unwrap();
    let source = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-timer-formal")
        .join(format!("{stage}-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("design.v"), &source).unwrap();
    (dir, source)
}
// Independent bus-level reference. State correspondence assertions strengthen
// induction; no assumption ties the observer state to the DUT or requires ready.
const OBSERVER: &str = r#"
reg started=0, pending=0, ev=0, stalled=0, cancelled=0;
reg [1:0] control=0, response_error=0;
reg [31:0] counter=0, compare_reg=0, response_data=0;
wire submit=req_valid && !pending && !rst;
wire [31:0] bytes={{8{wstrb[3]}},{8{wstrb[2]}},{8{wstrb[1]}},{8{wstrb[0]}}};
wire wc=submit && write && addr==0 && wstrb[0];
wire wn=submit && write && addr==4 && (|wstrb);
wire wp=submit && write && addr==8 && (|wstrb);
wire clear=submit && write && addr==12 && wstrb[0] && wdata[0];
wire [31:0] inc=counter+32'd1;
wire hit=!rst && control[0] && !(wc || wn || wp) && inc==compare_reg;
reg [31:0] read_snapshot;
reg [1:0] response_code;
always @* begin
  response_code=0; read_snapshot=0;
  case(addr)
    0: if(!write) read_snapshot={30'b0,control};
    4: if(!write) read_snapshot=counter;
    8: if(!write) read_snapshot=compare_reg;
    12: if(!write) read_snapshot={31'b0,ev};
    default: response_code=2;
  endcase
end
always @(posedge clk) begin
  started<=1;
  if(!started) assume(rst);
  if(started) begin
    assert(req_ready==(!pending && !rst));
    assert(rsp_valid==pending);
    assert(rdata==response_data); assert(error==response_error);
    assert(match_event==hit);
    assert(ctrl_value=={30'b0,control}); assert(count_value==counter);
    assert(compare_value==compare_reg); assert(EVENT_value=={31'b0,ev});
    if($past(rsp_valid && !rsp_ready && !rst)) begin
      assert(rsp_valid); assert(rdata==$past(rdata)); assert(error==$past(error));
    end
    if($past(req_valid && !req_ready && !rst) && !rst) begin
      assume(req_valid); assume(write==$past(write)); assume(addr==$past(addr));
      assume(wdata==$past(wdata)); assume(wstrb==$past(wstrb));
    end
    cover(hit);
    cover(hit && compare_reg==0 && counter==32'hffffffff);
    cover(hit && clear);
    cover(control[0] && inc==compare_reg && wc);
    cover(control[0] && inc==compare_reg && wn);
    cover(control[0] && inc==compare_reg && wp);
    cover(hit && submit && write && addr==0 && !wstrb[0] && (|wstrb));
    cover(stalled && pending && rsp_ready && !rst);
    cover(cancelled && submit);
    cover(hit && control[1] && $past(hit && control[1]));
    if(rst && pending) cancelled<=1;
  end
  if(rst) begin
    pending<=0; ev<=0; control<=0; counter<=0; compare_reg<=0;
    response_data<=0; response_error<=0; stalled<=0;
  end else begin
    if(pending && !rsp_ready) stalled<=1;
    if(pending && rsp_ready) begin pending<=0; stalled<=0; end
    if(submit) begin pending<=1; response_data<=read_snapshot; response_error<=response_code; end
    if(wc) control<=wdata[1:0];
    else if(hit && !control[1]) control<={control[1],1'b0};
    if(wn) counter<=(counter & ~bytes) | (wdata & bytes);
    else if(control[0] && !(wc || wp)) begin
      if(hit && control[1]) counter<=0; else counter<=inc;
    end
    if(wp) compare_reg<=(compare_reg & ~bytes) | (wdata & bytes);
    ev <= (ev && !clear) || hit;
  end
end
"#;
#[test]
#[ignore = "dedicated real SBY/Yosys/Z3 gate; run --ignored"]
fn p0_timer_safety_and_cover() {
    let (dir, source) = design("proof");
    let start = source.find("module Timer ").expect("top module");
    let end = start + source[start..].find("endmodule").unwrap();
    let mut formal = source.clone();
    formal.insert_str(end, OBSERVER);
    fs::write(dir.join("observer.sv"), OBSERVER).unwrap();
    fs::write(dir.join("formal.v"), formal).unwrap();
    fs::write(dir.join("timer.sby"),"[tasks]\nprove\ncover\n[options]\nprove: mode prove\ncover: mode cover\nprove: depth 16\ncover: depth 24\n[engines]\nsmtbmc --unroll z3\n[script]\nread -formal formal.v\nprep -top Timer -flatten\n[files]\nformal.v\n").unwrap();
    for (tool, arg) in [("sby", "--version"), ("yosys", "-V"), ("z3", "-version")] {
        run(&dir, tool, &[arg], tool);
    }
    for task in ["prove", "cover"] {
        run(&dir, "sby", &["-f", "timer.sby", task], task);
        assert_eq!(
            fs::read_to_string(dir.join(format!("timer_{task}/status")))
                .unwrap()
                .split_whitespace()
                .next(),
            Some("PASS")
        );
    }
    println!("Timer safety/cover {}", dir.display());
}
#[test]
#[ignore = "dedicated real Yosys gate; run --ignored"]
fn p0_timer_original_synthesis() {
    let (dir, _) = design("synthesis");
    run(&dir, "yosys", &["-V"], "version");
    run(
        &dir,
        "yosys",
        &[
            "-p",
            "read_verilog design.v; hierarchy -check -top Timer; proc; check -assert; synth -top Timer -flatten; check -assert; stat; write_json synthesis.json",
        ],
        "synthesis",
    );
    let net: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join("synthesis.json")).unwrap()).unwrap();
    let cells = net["modules"]["Timer"]["cells"].as_object().unwrap();
    assert!(
        cells
            .values()
            .any(|v| v["type"].as_str().unwrap().contains("DFF"))
    );
    assert!(
        !cells
            .values()
            .any(|v| v["type"].as_str().unwrap().contains("LATCH"))
    );
    println!(
        "Timer raw synthesis cells={} {}",
        cells.len(),
        dir.display()
    );
}

#[test]
#[ignore = "dedicated real SBY counterexample gate; run --ignored"]
fn p0_timer_observer_rejects_representative_rtl_faults() {
    let (dir, source) = design("mutations");
    // The observer equations and assumptions are identical for all runs. Labels
    // identify a reachable property failure, never a parser/solver error.
    let observer = OBSERVER
        .replace(
            "assert(match_event==hit);",
            "timer_match: assert(match_event==hit);",
        )
        .replace(
            "assert(count_value==counter);",
            "timer_count: assert(count_value==counter);",
        )
        .replace(
            "assert(EVENT_value=={31'b0,ev});",
            "timer_event: assert(EVENT_value=={31'b0,ev});",
        );
    fs::write(dir.join("observer.sv"), &observer).unwrap();
    let mutations = [
        (
            "config-suppression",
            "  assign match_event = (advance & equal);",
            "  assign match_event = (running & equal);",
            "timer_match",
        ),
        (
            "event-clear-priority",
            "  assign _csr_n129 = (_csr_n127 | _csr_n128);",
            "  assign _csr_n129 = ((_csr_n105 | _csr_n128) & _csr_n126);",
            "timer_event",
        ),
        (
            "counter-lost-bit31",
            "  assign count_next = (count_write_commit ? count_candidate : count_running);",
            "  assign count_next = (count_write_commit ? count_candidate : count_running) & 32'h7fffffff;",
            "timer_count",
        ),
    ];
    let mut cases = vec![("control", source.clone(), "")];
    for (name, old, new, property) in mutations {
        assert_eq!(source.matches(old).count(), 1, "mutation site {name}");
        let mutant = source.replacen(old, new, 1);
        assert_ne!(mutant, source, "mutation must change DUT RTL");
        fs::write(
            dir.join(format!("{name}-mutation.txt")),
            format!(
                "DUT original: {old}\nDUT mutation: {new}\nExpected failed property: {property}\n"
            ),
        )
        .unwrap();
        cases.push((name, mutant, property));
    }
    for (name, rtl, property) in cases {
        fs::write(dir.join(format!("{name}-dut.v")), &rtl).unwrap();
        let start = rtl.find("module Timer ").unwrap();
        let end = start + rtl[start..].find("endmodule").unwrap();
        let mut formal = rtl;
        formal.insert_str(end, &observer);
        fs::write(dir.join(format!("{name}.v")), formal).unwrap();
        fs::write(dir.join(format!("{name}.sby")), format!("[options]\nmode prove\ndepth 16\n[engines]\nsmtbmc --unroll z3\n[script]\nread -formal {name}.v\nprep -top Timer -flatten\n[files]\n{name}.v\n")).unwrap();
        let file = format!("{name}.sby");
        if name == "control" {
            run(&dir, "sby", &["-f", &file], "control");
            assert_eq!(
                fs::read_to_string(dir.join("control/status"))
                    .unwrap()
                    .split_whitespace()
                    .next(),
                Some("PASS")
            );
            continue;
        }
        let log = fs::File::create(dir.join(format!("{name}.log"))).unwrap();
        let status = Command::new("timeout")
            .args(["--kill-after=5s", "180s", "sby", "-f", &file])
            .current_dir(&dir)
            .stdout(Stdio::from(log.try_clone().unwrap()))
            .stderr(Stdio::from(log))
            .status()
            .unwrap();
        fs::write(dir.join(format!("{name}.exit")), format!("{status}\n")).unwrap();
        assert_eq!(
            status.code(),
            Some(2),
            "expected SBY FAIL, not ERROR/UNKNOWN/timeout: {}",
            dir.display()
        );
        assert_eq!(
            fs::read_to_string(dir.join(format!("{name}/status")))
                .unwrap()
                .split_whitespace()
                .next(),
            Some("FAIL")
        );
        let base =
            fs::read_to_string(dir.join(format!("{name}/engine_0/logfile_basecase.txt"))).unwrap();
        assert!(
            base.lines()
                .any(|line| line.contains("Assert failed") && line.contains(property)),
            "expected reachable {property}: {base}"
        );
        let trace = fs::read_to_string(dir.join(format!("{name}/engine_0/trace.vcd"))).unwrap();
        assert!(trace.contains("$enddefinitions") && trace.contains("#0"));
    }
    println!(
        "Timer original PASS and three representative DUT mutation counterexamples PASS {}",
        dir.display()
    );
}
