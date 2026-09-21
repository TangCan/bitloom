//! Story128.4: independent GPIO synchronization and CSR recurrence proof, reachability, raw synthesis.
use bitloom_prelude::{Elaboratable, ip::GpioCsr};
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
    let hir = GpioCsr::elaborate().unwrap();
    let source = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-gpio-formal")
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
reg started=0, pending=0, cancelled=0;
reg [3:0] stall_count=0, age=0;
reg initial_high=0;
reg [31:0] directions=0, outputs=0, flags=0, sample1=0, sample2=0, previous=0;
reg [31:0] response_data=0;
reg [1:0] response_error=0;
wire submit=req_valid && !pending && !rst;
wire [31:0] byte_mask={{8{wstrb[3]}},{8{wstrb[2]}},{8{wstrb[1]}},{8{wstrb[0]}}};
wire [31:0] selected=wdata & byte_mask;
wire [31:0] edges=sample2 & ~previous & ~directions;
wire clear=submit && write && addr==20;
wire dir_write=submit && write && addr==0;
reg [31:0] read_snapshot;
reg [1:0] response_code;
always @* begin
 response_code=0; read_snapshot=0;
 case(addr)
  0: if(!write) read_snapshot=directions;
  4: if(!write) read_snapshot=outputs;
  8: if(write) response_code=2; else read_snapshot=sample2;
  12,16: if(!write) response_code=2;
  20: if(!write) read_snapshot=flags;
  default: response_code=2;
 endcase
end
always @(posedge clk) begin
 started<=1;
 if(!started) assume(rst);
 if(started) begin
  assert(req_ready==(!pending && !rst)); assert(rsp_valid==pending);
  assert(rdata==response_data); assert(error==response_error);
  gpio_output: assert(pad_out==(outputs & directions));
  assert(pad_oe==directions);
  gpio_raw: assert(raw_event==(!rst && |edges));
  gpio_events: assert(rise_event_value==flags);
  gpio_dir: assert(dir_value==directions);
  gpio_out: assert(out_r==outputs);
  gpio_input: assert(in_value==sample2);
  assert(sync1==sample1); assert(sync2==sample2); assert(history==previous);
  if($past(rsp_valid && !rsp_ready && !rst)) begin
   assert(rsp_valid); assert(rdata==$past(rdata)); assert(error==$past(error));
  end
  if($past(req_valid && !req_ready && !rst) && !rst) begin
   assume(req_valid); assume(write==$past(write)); assume(addr==$past(addr));
   assume(wdata==$past(wdata)); assume(wstrb==$past(wstrb));
  end
  cover(!rst && edges[31]);
  cover(!rst && edges[31] && dir_write && selected[31]);
  cover(!rst && directions[31] && sample2[31] && !previous[31] && dir_write && byte_mask[31] && !wdata[31]);
  cover(!rst && edges[31] && clear && selected[31]);
  cover(!rst && edges[31] && clear && selected[0] && !selected[31]);
  cover(!rst && stall_count>=5 && pending && |edges);
  cover(!rst && cancelled && submit);
  cover(!rst && submit && addr==8 && !write && sample2[31]);
  cover(!rst && submit && addr==20 && !write && |edges);
  cover(!rst && age==2 && initial_high && edges[31]);
  if(rst && pending) cancelled<=1;
 end
 if(rst) begin
  pending<=0; directions<=0; outputs<=0; flags<=0;
  sample1<=0; sample2<=0; previous<=0;
  response_data<=0; response_error<=0; stall_count<=0; age<=0; initial_high<=pad_in[31];
 end else begin
  sample1<=pad_in; sample2<=sample1; previous<=sample2;
  if(age<15) age<=age+1;
  if(!pad_in[31]) initial_high<=0;
  if(pending && !rsp_ready && stall_count<15) stall_count<=stall_count+1;
  if(pending && rsp_ready) begin pending<=0; stall_count<=0; end
  if(submit) begin pending<=1; response_data<=read_snapshot; response_error<=response_code; end
  if(dir_write) directions<=(directions & ~byte_mask) | selected;
  if(submit && write) begin
   case(addr)
    4: outputs<=(outputs & ~byte_mask) | selected;
    12: outputs<=outputs | selected;
    16: outputs<=outputs & ~selected;
   endcase
  end
  flags <= (flags & ~(clear ? selected : 32'b0)) | edges;
 end
end
"#;
#[test]
#[ignore = "dedicated real SBY/Yosys/Z3 gate; run --ignored"]
fn p0_gpio_safety_and_cover() {
    let (dir, source) = design("proof");
    let start = source.find("module GpioCsr ").expect("top module");
    let end = start + source[start..].find("endmodule").unwrap();
    let mut formal = source.clone();
    formal.insert_str(end, OBSERVER);
    fs::write(dir.join("observer.sv"), OBSERVER).unwrap();
    fs::write(dir.join("formal.v"), formal).unwrap();
    fs::write(dir.join("gpio.sby"),"[tasks]\nprove\ncover\n[options]\nprove: mode prove\ncover: mode cover\nprove: depth 16\ncover: depth 24\n[engines]\nsmtbmc --unroll z3\n[script]\nread -formal formal.v\nprep -top GpioCsr -flatten\n[files]\nformal.v\n").unwrap();
    for (tool, arg) in [("sby", "--version"), ("yosys", "-V"), ("z3", "-version")] {
        run(&dir, tool, &[arg], tool);
    }
    for task in ["prove", "cover"] {
        run(&dir, "sby", &["-f", "gpio.sby", task], task);
        assert_eq!(
            fs::read_to_string(dir.join(format!("gpio_{task}/status")))
                .unwrap()
                .split_whitespace()
                .next(),
            Some("PASS")
        );
    }
    println!("GpioCsr safety/cover {}", dir.display());
}
#[test]
#[ignore = "dedicated real Yosys gate; run --ignored"]
fn p0_gpio_original_synthesis() {
    let (dir, _) = design("synthesis");
    run(&dir, "yosys", &["-V"], "version");
    run(
        &dir,
        "yosys",
        &[
            "-p",
            "read_verilog design.v; hierarchy -check -top GpioCsr; proc; check -assert; synth -top GpioCsr -flatten; check -assert; stat; write_json synthesis.json",
        ],
        "synthesis",
    );
    let net: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join("synthesis.json")).unwrap()).unwrap();
    let cells = net["modules"]["GpioCsr"]["cells"].as_object().unwrap();
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
        "GpioCsr raw synthesis cells={} {}",
        cells.len(),
        dir.display()
    );
}

#[test]
#[ignore = "dedicated real SBY counterexample gate; run --ignored"]
fn p0_gpio_observer_rejects_representative_rtl_faults() {
    let (dir, source) = design("mutations");
    // The observer equations and assumptions are identical for all runs. Labels
    // identify a reachable property failure, never a parser/solver error.
    let observer = OBSERVER.to_owned();
    fs::write(dir.join("observer.sv"), &observer).unwrap();
    let mutations = [
        (
            "input-is-output",
            "  assign in_value = sync2;",
            "  assign in_value = out_r;",
            "gpio_input",
        ),
        (
            "event-clear-priority",
            "    .rise_bits(rise_bits)",
            "    .rise_bits(rise_bits & ~(rise_event_write_commit ? rise_event_candidate : 32'b0))",
            "gpio_events",
        ),
        (
            "set-ignores-strobes",
            "  assign set_value = (out_r | set_candidate);",
            "  assign set_value = (out_r | wdata);",
            "gpio_out",
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
        let start = rtl.find("module GpioCsr ").unwrap();
        let end = start + rtl[start..].find("endmodule").unwrap();
        let mut formal = rtl;
        formal.insert_str(end, &observer);
        fs::write(dir.join(format!("{name}.v")), formal).unwrap();
        fs::write(dir.join(format!("{name}.sby")), format!("[options]\nmode prove\ndepth 16\n[engines]\nsmtbmc --unroll z3\n[script]\nread -formal {name}.v\nprep -top GpioCsr -flatten\n[files]\n{name}.v\n")).unwrap();
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
        "GpioCsr original PASS and three representative DUT mutation counterexamples PASS {}",
        dir.display()
    );
}
