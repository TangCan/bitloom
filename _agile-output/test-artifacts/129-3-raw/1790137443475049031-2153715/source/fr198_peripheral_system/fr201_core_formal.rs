use super::{
    fr201_backend_matrix::{commands, emit_direct, file_sha, identity, run_tool, source_hashes},
    system,
};
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Output,
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
`ifdef FR201_REQUEST_FORMAL
  // Independent accepted-channel credits, scoped to the current reset epoch.
  reg fr201_aw_credit = 0, fr201_w_credit = 0, fr201_ar_credit = 0;
  reg fr201_epoch_accept = 0;
  reg fr201_cancelled_pending = 0;
  always @(posedge clk) begin
    if (rst) begin
      fr201_aw_credit <= 0;
      fr201_w_credit <= 0;
      fr201_ar_credit <= 0;
      fr201_epoch_accept <= 0;
      fr201_cancelled_pending <= fr201_past_valid && $past(csr_req_valid && !csr_req_ready && !rst);
    end else begin
      if (axi_s_axi_awvalid && axi_s_axi_awready) fr201_aw_credit <= 1;
      if (axi_s_axi_wvalid && axi_s_axi_wready) fr201_w_credit <= 1;
      if (axi_s_axi_arvalid && axi_s_axi_arready) fr201_ar_credit <= 1;
      if ((axi_s_axi_awvalid && axi_s_axi_awready) ||
          (axi_s_axi_wvalid && axi_s_axi_wready) ||
          (axi_s_axi_arvalid && axi_s_axi_arready)) fr201_epoch_accept <= 1;
      if (csr_req_valid && csr_req_ready) begin
        if (csr_write) begin fr201_aw_credit <= 0; fr201_w_credit <= 0; end
        else fr201_ar_credit <= 0;
      end
    end
    if (fr201_past_valid && !rst && csr_req_valid) begin
      if (csr_write) assert(fr201_aw_credit && fr201_w_credit);
      else assert(fr201_ar_credit);
    end
    if (fr201_past_valid && $past(csr_req_valid && !csr_req_ready && !rst) && !rst) begin
      assert(csr_req_valid);
      assert({csr_write,csr_addr,csr_wdata,csr_wstrb} == $past({csr_write,csr_addr,csr_wdata,csr_wstrb}));
    end
    if (fr201_past_valid && $past(rst)) begin
      assert(!csr_req_valid);
      assert(!u_req_valid && !g_req_valid && !t_req_valid && !i_req_valid);
    end
    if (fr201_past_valid && !rst && !fr201_epoch_accept) begin
      assert(!csr_req_valid && !csr_rsp_valid && !axi_s_axi_bvalid && !axi_s_axi_rvalid);
    end
    cover(fr201_past_valid && $past(csr_req_valid && !csr_req_ready && !rst) && csr_req_valid && !csr_req_ready && !rst);
    cover(fr201_past_valid && $past(csr_req_valid && !csr_req_ready && !rst) && csr_req_valid && csr_req_ready && !rst);
    cover(fr201_cancelled_pending && !rst && !fr201_epoch_accept && !csr_req_valid && !csr_rsp_valid);
  end
  always @(posedge clk) begin
    if (fr201_past_valid && $past(u_req_valid && !u_req_ready && !rst) && !rst) begin
      assert(u_req_valid);
      assert({u_write,u_addr,u_wdata,u_wstrb} == $past({u_write,u_addr,u_wdata,u_wstrb}));
    end
  end
  always @(posedge clk) begin
    if (fr201_past_valid && $past(g_req_valid && !g_req_ready && !rst) && !rst) begin
      assert(g_req_valid);
      assert({g_write,g_addr,g_wdata,g_wstrb} == $past({g_write,g_addr,g_wdata,g_wstrb}));
    end
  end
  always @(posedge clk) begin
    if (fr201_past_valid && $past(t_req_valid && !t_req_ready && !rst) && !rst) begin
      assert(t_req_valid);
      assert({t_write,t_addr,t_wdata,t_wstrb} == $past({t_write,t_addr,t_wdata,t_wstrb}));
    end
  end
  always @(posedge clk) begin
    if (fr201_past_valid && $past(i_req_valid && !i_req_ready && !rst) && !rst) begin
      assert(i_req_valid);
      assert({i_write,i_addr,i_wdata,i_wstrb} == $past({i_write,i_addr,i_wdata,i_wstrb}));
    end
  end
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
    run_tool(dir, program, args, label, 180)
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
    for (name, mode, source) in [
        ("request", "bmc", "design.v"),
        ("request_cover", "cover", "design.v"),
        ("negative_request", "bmc", "mutant_request.v"),
        ("negative_reset", "bmc", "mutant_reset.v"),
    ] {
        fs::write(
            dir.join(format!("{name}.sby")),
            format!(
                r#"[options]
mode {mode}
depth 8

[engines]
smtbmc --unroll z3

[script]
read -formal -D FR201_RESPONSE_FORMAL -D FR201_REQUEST_FORMAL {source}
hierarchy -top Fr198AxiCore
select -clear
blackbox Fr198Uart Fr198Gpio Fr198Timer Fr198Irq
setattr -mod -unset blackbox =A:blackbox=1
prep -top Fr198AxiCore

[files]
{source}
"#
            ),
        )
        .unwrap();
    }
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
    let data_assignment = "assign csr_wdata = axi_csr_wdata;";
    assert_eq!(
        observed.matches(data_assignment).count(),
        1,
        "request mutation site"
    );
    fs::write(
        dir.join("mutant_request.v"),
        observed.replacen(
            data_assignment,
            "assign csr_wdata = axi_csr_wdata ^ {32{u_rx}};",
            1,
        ),
    )
    .unwrap();
    let valid_assignment = "assign csr_req_valid = axi_csr_req_valid;";
    assert_eq!(
        observed.matches(valid_assignment).count(),
        1,
        "reset mutation site"
    );
    fs::write(dir.join("mutant_reset.v"), observed.replacen(valid_assignment, r#"
reg fr201_mutant_replay = 0;
always @(posedge clk) fr201_mutant_replay <= rst && fr201_past_valid && $past(csr_req_valid && !csr_req_ready && !rst);
assign csr_req_valid = axi_csr_req_valid | fr201_mutant_replay;
"#, 1)).unwrap();
    write_sby(&dir);
    let sby = std::env::var("BITLOOM_SBY").unwrap_or_else(|_| "sby".into());
    let tools = json!({
        "sby": identity(&dir, &sby, "--version", "sby-version"),
        "yosys": identity(&dir, "yosys", "-V", "yosys-version"),
        "z3": identity(&dir, "z3", "--version", "z3-version"),
    });
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
    let request = run(&dir, &sby, &["-f", "request.sby"], "request");
    assert!(
        request.status.success(),
        "request/reset BMC failed: {}",
        String::from_utf8_lossy(&request.stdout)
    );
    let request_cover = run(&dir, &sby, &["-f", "request_cover.sby"], "request-cover");
    assert!(
        request_cover.status.success(),
        "request/reset cover failed: {}",
        String::from_utf8_lossy(&request_cover.stdout)
    );
    let mut request_witnesses = serde_json::Map::new();
    for entry in fs::read_dir(dir.join("request_cover/engine_0")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|extension| extension == "vcd") {
            assert!(
                fs::metadata(&path).unwrap().len() > 0,
                "empty request/reset cover witness"
            );
            request_witnesses.insert(
                path.strip_prefix(&dir).unwrap().to_string_lossy().into(),
                json!(file_sha(&path)),
            );
        }
    }
    assert!(
        request_witnesses.len() >= 5,
        "all five response/IRQ/request/reset covers must have actual VCD witnesses"
    );
    let mut controls = serde_json::Map::new();
    controls.insert("irq_wiring".into(), json!({"status":"EXPECTED_FAIL", "exit_code":negative.status.code(), "counterexample_vcd":true}));
    for (name, task) in [
        ("request_hold", "negative_request"),
        ("reset_cancel", "negative_reset"),
    ] {
        let result = run(&dir, &sby, &["-f", &format!("{task}.sby")], task);
        let text = String::from_utf8_lossy(&result.stdout).to_string()
            + &String::from_utf8_lossy(&result.stderr);
        let trace = dir.join(format!("{task}/engine_0/trace.vcd"));
        assert!(
            !result.status.success()
                && text.contains("Assert failed in Fr198AxiCore")
                && text.contains("DONE (FAIL")
                && fs::metadata(&trace).is_ok_and(|m| m.len() > 0),
            "{name} must fail a real assertion with counterexample: {text}"
        );
        controls.insert(
            name.into(),
            json!({"status":"EXPECTED_FAIL", "exit_code":result.status.code(),
            "counterexample_vcd":true, "trace_sha256":file_sha(&trace)}),
        );
    }
    fs::write(
        dir.join("evidence.json"),
        serde_json::to_vec_pretty(&json!({
            "prove": {"status": "PASS", "depth": 4, "exit_code": prove.status.code()},
            "response_reset_bmc": {"status": "PASS", "depth": 8, "exit_code": response.status.code()},
            "cover": {"status": "PASS", "depth": 2, "exit_code": cover.status.code()},
            "negative_control": {"status": "EXPECTED_FAIL", "exit_code": negative.status.code(), "counterexample_vcd": true},
            "request_reset_bmc": {"status":"PASS", "depth":8, "exit_code":request.status.code()},
            "request_reset_cover": {"status":"PASS", "depth":8, "exit_code":request_cover.status.code(), "witnesses":"blocked request, held-to-commit request, reset cancellation after blocked request", "witness_vcd":request_witnesses},
            "negative_controls": controls,
            "source_sha256": source_hashes(&dir),
            "commands": commands(&dir),
            "tools": tools,
            "assumptions": "blackboxed CSR leaves clear rsp_valid after reset and hold rsp_valid/rdata/error while blocked",
            "limits": "depth-8 bounded complete-system request hold/accepted-channel credits/reset cancellation/response and IRQ integration; four peripheral leaves blackboxed; cancellation checked until fresh epoch inputs, not full AXI/UART liveness"
        }))
        .unwrap(),
    )
    .unwrap();
    println!("FR201 formal PASS artifacts={}", dir.display());
}
