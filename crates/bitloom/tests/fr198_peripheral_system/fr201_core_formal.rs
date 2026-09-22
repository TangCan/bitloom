use super::{fr201_backend_matrix::emit_direct, system};
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

const OBSERVER: &str = r#"
`ifdef FR201_RESPONSE_FORMAL
  reg fr201_past_valid = 0;
`endif
  always @(*) begin
    assert(i_raw_events == irq_events);
    assert(irq_events[0] == t_match_event);
    assert(irq_events[1] == u_raw_events[0]);
    assert(irq_events[2] == u_raw_events[1]);
    assert(irq_events[3] == (u_raw_events[2] | u_raw_events[3]));
    assert(irq_events[4] == g_raw_event);
  end
`ifdef FR201_RESPONSE_FORMAL
  always @(posedge clk) begin
    fr201_past_valid <= 1;
    if (!fr201_past_valid) assume(rst);
    if (fr201_past_valid && $past(rst)) begin
      assume(!u_rsp_valid && !g_rsp_valid && !t_rsp_valid && !i_rsp_valid);
    end
    if (fr201_past_valid && $past(u_rsp_valid && !u_rsp_ready && !rst) && !rst) begin
      assume(u_rsp_valid && u_rdata == $past(u_rdata) && u_error == $past(u_error));
    end
    if (fr201_past_valid && $past(g_rsp_valid && !g_rsp_ready && !rst) && !rst) begin
      assume(g_rsp_valid && g_rdata == $past(g_rdata) && g_error == $past(g_error));
    end
    if (fr201_past_valid && $past(t_rsp_valid && !t_rsp_ready && !rst) && !rst) begin
      assume(t_rsp_valid && t_rdata == $past(t_rdata) && t_error == $past(t_error));
    end
    if (fr201_past_valid && $past(i_rsp_valid && !i_rsp_ready && !rst) && !rst) begin
      assume(i_rsp_valid && i_rdata == $past(i_rdata) && i_error == $past(i_error));
    end
    if (fr201_past_valid && $past(rst)) begin
      assert(!axi_s_axi_bvalid);
      assert(!axi_s_axi_rvalid);
      assert(!csr_rsp_valid);
    end
    if (fr201_past_valid && $past(axi_s_axi_bvalid && !axi_s_axi_bready && !rst) && !rst) begin
      assert(axi_s_axi_bvalid);
      assert(axi_s_axi_bresp == $past(axi_s_axi_bresp));
    end
    if (fr201_past_valid && $past(axi_s_axi_rvalid && !axi_s_axi_rready && !rst) && !rst) begin
      assert(axi_s_axi_rvalid);
      assert(axi_s_axi_rdata == $past(axi_s_axi_rdata));
      assert(axi_s_axi_rresp == $past(axi_s_axi_rresp));
    end
    if (fr201_past_valid && $past(csr_rsp_valid && !csr_rsp_ready && !rst) && !rst) begin
      assert(csr_rsp_valid);
      assert(csr_rdata == $past(csr_rdata));
      assert(csr_error == $past(csr_error));
    end
    cover(!rst && irq_events != 0);
    cover(fr201_past_valid && !rst && csr_rsp_valid);
  end
`else
  always @(posedge clk) cover(!rst && irq_events != 0);
`endif
"#;

fn root() -> PathBuf {
    let base = std::env::var_os("BITLOOM_FR201_FORMAL_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/fr201-core-formal")
        });
    base.join(format!(
        "run-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

fn run(dir: &Path, program: &str, args: &[&str], label: &str) -> Output {
    let result = Command::new("timeout")
        .args(["--kill-after=5s", "180s", program])
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap_or_else(|e| panic!("required {program} failed to start: {e}"));
    fs::write(
        dir.join(format!("{label}.log")),
        [&result.stdout[..], &result.stderr[..]].concat(),
    )
    .unwrap();
    result
}

fn instrument(source: &str) -> String {
    let boundary = source.find("\nmodule Fr198Axi(").expect("boundary module");
    let end = source[..boundary]
        .rfind("endmodule")
        .expect("core endmodule");
    let mut observed = source.to_string();
    observed.insert_str(end, OBSERVER);
    observed
}

fn write_sby(dir: &Path) {
    fs::write(
        dir.join("core.sby"),
        r#"[tasks]
prove
cover

[options]
prove: mode prove
prove: depth 4
cover: mode cover
cover: depth 2

[engines]
smtbmc z3

[script]
read -formal design.v
hierarchy -top Fr198AxiCore
select -clear
blackbox Fr198Uart Fr198Gpio Fr198Timer Fr198Irq
setattr -mod -unset blackbox =A:blackbox=1
prep -top Fr198AxiCore

[files]
design.v
"#,
    )
    .unwrap();
    fs::write(
        dir.join("negative.sby"),
        r#"[options]
mode prove
depth 2

[engines]
smtbmc z3

[script]
read -formal mutant.v
hierarchy -top Fr198AxiCore
select -clear
blackbox Fr198Uart Fr198Gpio Fr198Timer Fr198Irq
setattr -mod -unset blackbox =A:blackbox=1
prep -top Fr198AxiCore

[files]
mutant.v
"#,
    )
    .unwrap();
    fs::write(
        dir.join("response.sby"),
        r#"[options]
mode bmc
depth 8

[engines]
smtbmc --unroll z3

[script]
read -formal -D FR201_RESPONSE_FORMAL design.v
hierarchy -top Fr198AxiCore
select -clear
blackbox Fr198Uart Fr198Gpio Fr198Timer Fr198Irq
setattr -mod -unset blackbox =A:blackbox=1
prep -top Fr198AxiCore

[files]
design.v
"#,
    )
    .unwrap();
}

#[test]
#[ignore = "dedicated system-level SymbiYosys prove/cover/negative-control gate"]
fn fr201_complete_system_formal_and_negative_control() {
    let dir = root();
    fs::create_dir_all(&dir).unwrap();
    let hir = system(true);
    let raw_path = emit_direct(&hir, &dir);
    let raw = fs::read_to_string(raw_path).unwrap();
    let observed = instrument(&raw);
    fs::write(dir.join("design.v"), &observed).unwrap();
    let assignment = "assign i_raw_events = irq_events;";
    assert_eq!(observed.matches(assignment).count(), 1, "mutation site");
    fs::write(
        dir.join("mutant.v"),
        observed.replacen(assignment, "assign i_raw_events = ~irq_events;", 1),
    )
    .unwrap();
    write_sby(&dir);
    let sby = std::env::var("BITLOOM_SBY").unwrap_or_else(|_| "sby".into());
    let prove = run(&dir, &sby, &["-f", "core.sby", "prove"], "prove");
    assert!(
        prove.status.success(),
        "prove failed: {}",
        String::from_utf8_lossy(&prove.stderr)
    );
    let response = run(&dir, &sby, &["-f", "response.sby"], "response");
    assert!(
        response.status.success(),
        "response/reset BMC failed: {}",
        String::from_utf8_lossy(&response.stderr)
    );
    let cover = run(&dir, &sby, &["-f", "core.sby", "cover"], "cover");
    assert!(
        cover.status.success(),
        "cover failed: {}",
        String::from_utf8_lossy(&cover.stderr)
    );
    let negative = run(&dir, &sby, &["-f", "negative.sby"], "negative");
    assert!(
        !negative.status.success(),
        "negative control unexpectedly passed"
    );
    let negative_text = [&negative.stdout[..], &negative.stderr[..]].concat();
    assert!(
        String::from_utf8_lossy(&negative_text).contains("Assert failed in Fr198AxiCore")
            && String::from_utf8_lossy(&negative_text).contains("DONE (FAIL")
            && dir.join("negative/engine_0/trace.vcd").is_file(),
        "negative control did not produce an assertion counterexample and VCD"
    );
    fs::write(
        dir.join("evidence.json"),
        serde_json::to_vec_pretty(&json!({
            "prove": {"status": "PASS", "depth": 4, "exit_code": prove.status.code()},
            "response_reset_bmc": {"status": "PASS", "depth": 8, "exit_code": response.status.code()},
            "cover": {"status": "PASS", "depth": 2, "exit_code": cover.status.code()},
            "negative_control": {"status": "EXPECTED_FAIL", "exit_code": negative.status.code(), "counterexample_vcd": true},
            "assumptions": "blackboxed CSR leaves clear rsp_valid after reset and hold rsp_valid/rdata/error while blocked",
            "limits": "bounded complete-system reset/response/IRQ integration with four peripheral leaves blackboxed; not a full AXI or UART proof"
        }))
        .unwrap(),
    )
    .unwrap();
    println!("FR201 formal PASS artifacts={}", dir.display());
}
