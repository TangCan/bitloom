//! Story128.3: independent five-source IRQ recurrence proof, reachability, raw synthesis.
use bitloom_prelude::{Elaboratable, ip::Irq};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
fn run(dir: &Path, tool: &str, args: &[&str], label: &str) {
    let log = fs::File::create(dir.join(format!("{label}.log"))).unwrap();
    let start_utc_unix_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let status = Command::new("timeout")
        .args(["--kill-after=5s", "180s", tool])
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .unwrap();
    fs::write(dir.join(format!("{label}.exit")), format!("{status}\n")).unwrap();
    fs::write(dir.join(format!("{label}-command.json")),serde_json::json!({"tool":tool,"args":args,"exit_code":status.code(),"start_utc_unix_ms":start_utc_unix_ms,"end_utc_unix_ms":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()}).to_string()).unwrap();
    assert!(
        status.success(),
        "{tool} {args:?}: {status}; {}",
        dir.display()
    );
}
fn design(stage: &str) -> (PathBuf, String) {
    let hir = Irq::elaborate().unwrap();
    let source = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-irq-formal")
        .join(format!(
            "{stage}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("design.v"), &source).unwrap();
    (dir, source)
}
// Independent bus-level reference. State correspondence assertions strengthen
// induction; no assumption ties the observer state to the DUT or requires ready.
const OBSERVER: &str = r#"
reg started=0, pending=0, stalled=0, cancelled=0;
reg [4:0] flags=0, enables=0;
reg [31:0] response_data=0;
reg [1:0] response_error=0;
wire submit=req_valid && !pending && !rst;
wire clear=submit && write && addr==0 && wstrb[0];
wire mask_write=submit && write && addr==4 && wstrb[0];
wire inject=submit && write && addr==8 && wstrb[0];
reg [31:0] read_snapshot;
reg [1:0] response_code;
always @* begin
 response_code=0; read_snapshot=0;
 case(addr)
  0: if(!write) read_snapshot={27'b0,flags};
  4: if(!write) read_snapshot={27'b0,enables};
  8: if(!write) response_code=2;
  12: if(write) response_code=2; else read_snapshot={27'b0,raw_events};
  default: response_code=2;
 endcase
end
integer n;
always @(posedge clk) begin
 started<=1;
 if(!started) assume(rst);
 if(started) begin
  assert(req_ready==(!pending && !rst)); assert(rsp_valid==pending);
  assert(rdata==response_data); assert(error==response_error);
  assert(irq==(|(flags & enables)));
  irq_pending: assert(pending_value=={27'b0,flags});
  irq_enable: assert(enable_value=={27'b0,enables});
  irq_raw: assert(raw_value=={27'b0,raw_events});
  if($past(rsp_valid && !rsp_ready && !rst)) begin
   assert(rsp_valid); assert(rdata==$past(rdata)); assert(error==$past(error));
  end
  if($past(req_valid && !req_ready && !rst) && !rst) begin
   assume(req_valid); assume(write==$past(write)); assume(addr==$past(addr));
   assume(wdata==$past(wdata)); assume(wstrb==$past(wstrb));
  end
  for(integer b=0;b<5;b=b+1) begin
   cover(!rst && raw_events[b] && !enables[b]);
   cover(!rst && raw_events[b] && clear && wdata[b]);
   cover(!rst && raw_events[b] && inject && wdata[b]);
   cover(!rst && flags[b] && !enables[b] && mask_write && wdata[b]);
  end
  cover(!rst && stalled && pending && |raw_events);
  cover(!rst && cancelled && submit);
  cover(!rst && submit && addr==12 && !write && |raw_events);
  if(rst && pending) cancelled<=1;
 end
 if(rst) begin
  pending<=0; flags<=0; enables<=0; response_data<=0; response_error<=0; stalled<=0;
 end else begin
  if(pending && !rsp_ready) stalled<=1;
  if(pending && rsp_ready) begin pending<=0; stalled<=0; end
  if(submit) begin pending<=1; response_data<=read_snapshot; response_error<=response_code; end
  if(mask_write) enables<=wdata[4:0];
  for(n=0;n<5;n=n+1) begin
   if(raw_events[n] || (inject && wdata[n])) flags[n]<=1;
   else if(clear && wdata[n]) flags[n]<=0;
  end
 end
end
"#;
#[test]
#[ignore = "dedicated real SBY/Yosys/Z3 gate; run --ignored"]
fn p0_irq_safety_and_cover() {
    let (dir, source) = design("proof");
    let start = source.find("module Irq ").expect("top module");
    let end = start + source[start..].find("endmodule").unwrap();
    let mut formal = source.clone();
    formal.insert_str(end, OBSERVER);
    fs::write(dir.join("observer.sv"), OBSERVER).unwrap();
    fs::write(dir.join("formal.v"), formal).unwrap();
    fs::write(dir.join("irq.sby"),"[tasks]\nprove\ncover\n[options]\nprove: mode prove\ncover: mode cover\nprove: depth 16\ncover: depth 24\n[engines]\nsmtbmc --unroll z3\n[script]\nread -formal formal.v\nprep -top Irq -flatten\n[files]\nformal.v\n").unwrap();
    for (tool, arg) in [("sby", "--version"), ("yosys", "-V"), ("z3", "-version")] {
        run(&dir, tool, &[arg], tool);
    }
    for task in ["prove", "cover"] {
        run(&dir, "sby", &["-f", "irq.sby", task], task);
        assert_eq!(
            fs::read_to_string(dir.join(format!("irq_{task}/status")))
                .unwrap()
                .split_whitespace()
                .next(),
            Some("PASS")
        );
    }
    println!("Irq safety/cover {}", dir.display());
}
#[test]
#[ignore = "dedicated real Yosys gate; run --ignored"]
fn p0_irq_original_synthesis() {
    let (dir, _) = design("synthesis");
    run(&dir, "yosys", &["-V"], "version");
    run(
        &dir,
        "yosys",
        &[
            "-p",
            "read_verilog design.v; hierarchy -check -top Irq; proc; check -assert; synth -top Irq -flatten; check -assert; stat; write_json synthesis.json",
        ],
        "synthesis",
    );
    let net: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join("synthesis.json")).unwrap()).unwrap();
    let cells = net["modules"]["Irq"]["cells"].as_object().unwrap();
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
    println!("Irq raw synthesis cells={} {}", cells.len(), dir.display());
}

#[test]
#[ignore = "dedicated real SBY counterexample gate; run --ignored"]
fn p0_irq_observer_rejects_representative_rtl_faults() {
    let (dir, source) = design("mutations");
    // The observer equations and assumptions are identical for all runs. Labels
    // identify a reachable property failure, never a parser/solver error.
    let observer = OBSERVER.to_owned();
    fs::write(dir.join("observer.sv"), &observer).unwrap();
    let mutations = [
        (
            "event-clear-priority",
            "  assign event_bits = (raw_value | test_bits);",
            "  assign event_bits = (raw_value | test_bits) & ~(pending_write_commit ? pending_candidate : 32'b0);",
            "irq_pending",
        ),
        (
            "test-without-commit",
            "  assign test_bits = (test_write_commit ? test_candidate : zero32);",
            "  assign test_bits = test_candidate;",
            "irq_pending",
        ),
        (
            "raw-includes-software",
            "  assign raw_value = {zero27, raw_events};",
            "  assign raw_value = {zero27, raw_events} | test_bits;",
            "irq_raw",
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
        let start = rtl.find("module Irq ").unwrap();
        let end = start + rtl[start..].find("endmodule").unwrap();
        let mut formal = rtl;
        formal.insert_str(end, &observer);
        fs::write(dir.join(format!("{name}.v")), formal).unwrap();
        fs::write(dir.join(format!("{name}.sby")), format!("[options]\nmode prove\ndepth 16\n[engines]\nsmtbmc --unroll z3\n[script]\nread -formal {name}.v\nprep -top Irq -flatten\n[files]\n{name}.v\n")).unwrap();
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
        let start_utc_unix_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let status = Command::new("timeout")
            .args(["--kill-after=5s", "180s", "sby", "-f", &file])
            .current_dir(&dir)
            .stdout(Stdio::from(log.try_clone().unwrap()))
            .stderr(Stdio::from(log))
            .status()
            .unwrap();
        fs::write(dir.join(format!("{name}.exit")), format!("{status}\n")).unwrap();
        fs::write(dir.join(format!("{name}-command.json")),serde_json::json!({"command":["timeout","--kill-after=5s","180s","sby","-f",file.as_str()],"exit_code":status.code(),"start_utc_unix_ms":start_utc_unix_ms,"end_utc_unix_ms":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()}).to_string()).unwrap();
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
        "Irq original PASS and three representative DUT mutation counterexamples PASS {}",
        dir.display()
    );
}
