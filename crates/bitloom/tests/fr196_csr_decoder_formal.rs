//! Port-contract safety proved with asserted implementation correspondence lemmas.
//! Mutation BMC separately removes those lemmas and uses only the port observer.
//! Cover reachability and raw RTL synthesis are independent verification steps.
use bitloom_prelude::{Elaboratable, ip::CsrDecoder};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::Instant,
};
fn design(label: &str) -> (PathBuf, String) {
    let hir = CsrDecoder::elaborate().unwrap();
    let top = hir.circuit().modules[0].name.clone();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr196-decoder-formal")
        .join(format!("{label}-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let rtl = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(dir.join("design.v"), rtl).unwrap();
    (dir, top)
}
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
        .expect("real tool required");
    fs::write(dir.join(format!("{label}-execution.json")),serde_json::json!({"tool":tool,"args":args,"exit_code":status.code(),"elapsed_seconds":start.elapsed().as_secs_f64()}).to_string()).unwrap();
    assert!(
        status.success(),
        "{tool} failed: {status}; {}",
        dir.display()
    );
}
const OBSERVER: &str = r#"
reg f_pending=0, f_miss=0, f_write=0;
reg [1:0] f_owner=0;
reg [3:0] f_leaf_pending=0;
reg f_stalled=0, f_reset_cancel=0;
reg [1:0] f_cancel_kind=0, f_recovery_kind=0;
reg f_recovery_pending=0;
wire [3:0] f_lv={irq_rsp_valid,timer_rsp_valid,gpio_rsp_valid,uart_rsp_valid};
wire [3:0] f_lr={irq_rsp_ready,timer_rsp_ready,gpio_rsp_ready,uart_rsp_ready};
wire [31:0] f_expected_data=f_owner==0?uart_rdata:f_owner==1?gpio_rdata:f_owner==2?timer_rdata:irq_rdata;
wire [1:0] f_expected_error=f_owner==0?uart_error:f_owner==1?gpio_error:f_owner==2?timer_error:irq_error;

reg f_past=0;
wire [3:0] f_select={irq_req_valid,timer_req_valid,gpio_req_valid,uart_req_valid};
wire [3:0] f_expected_select=(!rst && !f_pending && req_valid) ?
  (addr<=16'h00ff ? 4'b0001 : addr<=16'h01ff ? 4'b0010 :
   addr<=16'h02ff ? 4'b0100 : addr<=16'h03ff ? 4'b1000 : 4'b0000) : 4'b0000;
wire [3:0] f_commit={irq_req_valid&&irq_req_ready,timer_req_valid&&timer_req_ready,gpio_req_valid&&gpio_req_ready,uart_req_valid&&uart_req_ready};
always @(posedge clk) begin
  f_past<=1;
  if (!f_past) assume(rst);
  if (f_past) begin
    // Induction lemmas are assertions, never restrictions on the environment.
    assert(busy==f_pending);
    if (f_pending) begin assert(miss==f_miss); if(!f_miss) assert(owner==f_owner); end
    assert(f_select==f_expected_select);
    assert((f_select & (f_select-1'b1)) == 0);
    if (rst) begin
      assert(!req_ready && f_select==0);
      assert(!uart_rsp_ready && !gpio_rsp_ready && !timer_rsp_ready && !irq_rsp_ready);
    end else begin
      if ($past(!rst && req_valid && !req_ready)) begin
        assume(req_valid);
        assume({write,addr,wdata,wstrb}==$past({write,addr,wdata,wstrb}));
      end
      if (addr>=16'h0400) assert(f_select==0);
      if (req_valid && req_ready) begin
        if (addr<16'h0400) assert(f_commit!=0); else assert(f_commit==0);
      end
      if (f_commit!=0) assert(req_valid && req_ready);
      if (uart_req_valid) begin
        assert(addr<=16'h00ff); assert(uart_addr==addr);
        assert({uart_write,uart_wdata,uart_wstrb}=={write,wdata,wstrb});
      end
      if (gpio_req_valid) begin
        assert(addr>=16'h0100 && addr<=16'h01ff); assert(gpio_addr==addr-16'h0100);
        assert({gpio_write,gpio_wdata,gpio_wstrb}=={write,wdata,wstrb});
      end
      if (timer_req_valid) begin
        assert(addr>=16'h0200 && addr<=16'h02ff); assert(timer_addr==addr-16'h0200);
        assert({timer_write,timer_wdata,timer_wstrb}=={write,wdata,wstrb});
      end
      if (irq_req_valid) begin
        assert(addr>=16'h0300 && addr<=16'h03ff); assert(irq_addr==addr-16'h0300);
        assert({irq_write,irq_wdata,irq_wstrb}=={write,wdata,wstrb});
      end
      // Only protocol assumptions: no response without a prior leaf commit,
      // and an offered response remains unchanged until consumption/reset.
      assume((f_lv & ~f_leaf_pending)==0);
      if(uart_rsp_valid) assume(uart_error!=1);
      if(gpio_rsp_valid) assume(gpio_error!=1);
      if(timer_rsp_valid) assume(timer_error!=1);
      if(irq_rsp_valid) assume(irq_error!=1);
      if (f_pending && !f_write && !f_miss && f_lv[f_owner] && f_expected_error!=0)
        assume(f_expected_data==0);
      if ($past(!rst && uart_rsp_valid && !uart_rsp_ready)) assume({uart_rsp_valid,uart_rdata,uart_error}==$past({uart_rsp_valid,uart_rdata,uart_error}));
      if ($past(!rst && gpio_rsp_valid && !gpio_rsp_ready)) assume({gpio_rsp_valid,gpio_rdata,gpio_error}==$past({gpio_rsp_valid,gpio_rdata,gpio_error}));
      if ($past(!rst && timer_rsp_valid && !timer_rsp_ready)) assume({timer_rsp_valid,timer_rdata,timer_error}==$past({timer_rsp_valid,timer_rdata,timer_error}));
      if ($past(!rst && irq_rsp_valid && !irq_rsp_ready)) assume({irq_rsp_valid,irq_rdata,irq_error}==$past({irq_rsp_valid,irq_rdata,irq_error}));
      assert((f_commit & (f_commit-1'b1))==0);
      if (f_pending) begin
        assert(!req_ready && f_select==0);
        if (f_miss) begin
          assert(rsp_valid && error==3 && rdata==0 && f_lr==0);
          assert(f_leaf_pending==0);
        end else begin
          assert(f_leaf_pending==(4'b1<<f_owner));
          assert(rsp_valid==f_lv[f_owner]);
          assert(rdata==f_expected_data && error==f_expected_error);
          assert(f_lr==(rsp_ready?(4'b1<<f_owner):0));
        end
      end else begin
        assert(!rsp_valid && f_lr==0 && f_leaf_pending==0);
        if (addr<16'h0100) assert(req_ready==uart_req_ready);
        else if (addr<16'h0200) assert(req_ready==gpio_req_ready);
        else if (addr<16'h0300) assert(req_ready==timer_req_ready);
        else if (addr<16'h0400) assert(req_ready==irq_req_ready);
        else assert(req_ready);
      end
      if ($past(!rst && rsp_valid && !rsp_ready))
        assert({rsp_valid,rdata,error}==$past({rsp_valid,rdata,error}));
      if ($past(rst)) assert(!rsp_valid && f_select==(req_valid && addr<16'h0400 ? (4'b1 << addr[9:8]):0));
      cover(f_stalled && rsp_valid && rsp_ready);
      cover(f_pending && !f_miss && f_owner==0 && rsp_valid && rsp_ready);
      cover(f_pending && !f_miss && f_owner==1 && rsp_valid && rsp_ready);
      cover(f_pending && !f_miss && f_owner==2 && rsp_valid && rsp_ready);
      cover(f_pending && !f_miss && f_owner==3 && rsp_valid && rsp_ready);
      cover(f_pending && !f_miss && f_owner==0 && f_stalled && rsp_valid && rsp_ready);
      cover(f_pending && !f_miss && f_owner==1 && f_stalled && rsp_valid && rsp_ready);
      cover(f_pending && !f_miss && f_owner==2 && f_stalled && rsp_valid && rsp_ready);
      cover(f_pending && !f_miss && f_owner==3 && f_stalled && rsp_valid && rsp_ready);
      // These flags are armed only by cancel -> later accept, and cleared on
      // any reset/consumption. A canceled old response cannot satisfy completion.
      cover(f_recovery_pending && f_recovery_kind==1 && f_pending && !f_miss && rsp_valid && rsp_ready);
      cover(f_recovery_pending && f_recovery_kind==2 && f_pending && !f_miss && rsp_valid && rsp_ready);
      cover(f_reset_cancel && req_valid && req_ready && addr<16'h0400);
      cover(f_commit[0]); cover(f_commit[1]); cover(f_commit[2]); cover(f_commit[3]);
      cover(req_valid && req_ready && addr==16'hffff);
    end
  end
  if (rst) begin
    if (f_pending) begin f_reset_cancel<=1; f_cancel_kind<=f_miss ? 2 : 1; end
    f_recovery_pending<=0; f_recovery_kind<=0;
    f_pending<=0; f_miss<=0; f_owner<=0; f_write<=0; f_leaf_pending<=0; f_stalled<=0;
  end else begin
    f_leaf_pending <= (f_leaf_pending | f_commit) & ~(f_lv & f_lr);
    if(req_valid && req_ready) begin
      assert(!f_pending);
      f_recovery_pending<=f_cancel_kind!=0; f_recovery_kind<=f_cancel_kind; f_cancel_kind<=0;
      f_pending<=1; f_miss<=addr>=16'h0400; f_owner<=addr[9:8]; f_write<=write;
    end
    if(rsp_valid && rsp_ready) begin assert(f_pending); f_pending<=0; f_stalled<=0; f_recovery_pending<=0; end
    if(rsp_valid && !rsp_ready) f_stalled<=1;
  end

end
"#;
#[test]
#[ignore = "requires real SBY/Yosys/Z3"]
fn p0_owner_response_inductive_safety_with_asserted_lemmas_and_cover() {
    let (dir, top) = design("routing");
    run(&dir, "yosys", &["-V"], "yosys-version");
    run(&dir, "sby", &["--version"], "sby-version");
    run(&dir, "z3", &["--version"], "z3-version");
    let rtl = fs::read_to_string(dir.join("design.v")).unwrap();
    assert_eq!(rtl.matches("endmodule").count(), 1);
    fs::write(
        dir.join("observed.v"),
        rtl.replace("endmodule", &format!("{OBSERVER}\nendmodule")),
    )
    .unwrap();
    for mode in ["prove", "cover"] {
        fs::write(dir.join(format!("{mode}.sby")),format!("[options]\nmode {mode}\ndepth 8\n[engines]\nsmtbmc --unroll z3\n[script]\nread -formal observed.v\nprep -top {top}\n[files]\nobserved.v\n")).unwrap();
        run(&dir, "sby", &["-f", &format!("{mode}.sby")], mode);
        let status = fs::read_to_string(dir.join(mode).join("status")).unwrap();
        assert_eq!(status.split_whitespace().next(), Some("PASS"));
    }
}
#[test]
#[ignore = "requires real Yosys"]
fn p0_original_decoder_synthesis_check() {
    let (dir, top) = design("synthesis");
    run(&dir, "yosys", &["-V"], "yosys-version");
    let script = format!(
        "read_verilog design.v; hierarchy -check -top {top}; proc; check -assert; synth -top {top}; check -assert; stat; write_json synthesis.json"
    );
    run(&dir, "yosys", &["-p", &script], "synthesis");
    let net: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join("synthesis.json")).unwrap()).unwrap();
    let cells = net["modules"][&top]["cells"].as_object().unwrap();
    assert!(!cells.is_empty());
    structure::registered(
        &fs::read_to_string(dir.join("synthesis.json")).unwrap(),
        &top,
        &[],
    );
    for cell in cells.values() {
        let kind = cell["type"].as_str().unwrap();
        assert!(!kind.to_ascii_lowercase().contains("latch"), "latch {kind}");
    }
}

#[path = "fr196_csr_decoder/common.rs"]
#[allow(dead_code)]
mod common;
#[path = "fr196_csr_decoder/structure.rs"]
mod structure;
#[test]
#[ignore = "requires real Yosys"]
fn p0_original_hierarchy_synthesis_and_five_registered_boundaries() {
    let hir = common::hierarchy();
    let dir = common::dir("hierarchy-synthesis");
    fs::write(
        dir.join("design.v"),
        bitloom_vlog::emit(&hir)
            .files
            .iter()
            .map(|f| f.contents.as_str())
            .collect::<Vec<_>>()
            .join("\n"),
    )
    .unwrap();
    run(
        &dir,
        "yosys",
        &[
            "-p",
            "read_verilog design.v; hierarchy -check -top DecoderSystem; proc; check -assert; synth -flatten -top DecoderSystem; check -assert; stat; write_json synthesis.json",
        ],
        "synthesis",
    );
    let count = structure::registered(
        &fs::read_to_string(dir.join("synthesis.json")).unwrap(),
        "DecoderSystem",
        &[
            "s_axi_awready",
            "s_axi_wready",
            "s_axi_arready",
            "s_axi_bvalid",
            "s_axi_rvalid",
        ],
    );
    fs::write(
        dir.join("boundary-result.json"),
        serde_json::json!({"flip_flops":count,"registered_outputs":5,"original_rtl_only":true})
            .to_string(),
    )
    .unwrap();
}

#[test]
#[ignore = "requires real SBY/Yosys/Z3"]
fn p0_port_only_oracle_kills_cross_owner_dual_select_and_high_alias_mutants() {
    let (dir, top) = design("mutations");
    let rtl = fs::read_to_string(dir.join("design.v")).unwrap();
    let observer=OBSERVER.replace("    assert(busy==f_pending);", "").replace("    if (f_pending) begin assert(miss==f_miss); if(!f_miss) assert(owner==f_owner); end", "");
    assert!(
        !observer.contains("assert(owner")
            && !observer.contains("assert(busy")
            && !observer.contains("assert(miss")
    );
    for (label, from, to) in [
        ("control", "", ""),
        (
            "miss-drops-under-stall",
            "assign busy_next = (consume ? zero : busy_started);",
            "assign busy_next = ((consume | miss_valid) ? zero : busy_started);",
        ),
        (
            "reset-retains-pending",
            "busy <= 0;",
            "busy <= (mutation_active & busy);",
        ),
        (
            "ready-gated-valid",
            "assign uart_req_valid = (offering & uart_hit);",
            "assign uart_req_valid = (offering & uart_hit & uart_req_ready);",
        ),
        (
            "cross-owner",
            "assign data_2 = (gpio_owned ? gpio_rdata : data_1);",
            "assign data_2 = (gpio_owned ? uart_rdata : data_1);",
        ),
        (
            "dual-select",
            "assign gpio_req_valid = (offering & gpio_hit);",
            "assign gpio_req_valid = (offering & (gpio_hit | uart_hit));",
        ),
        (
            "high-alias",
            "assign page = addr[15:8];",
            "assign page = {6'b0,addr[9:8]};",
        ),
    ] {
        let case = dir.join(label);
        fs::create_dir_all(&case).unwrap();
        let mut source = if label == "control" {
            rtl.clone()
        } else {
            assert_eq!(rtl.matches(from).count(), 1);
            rtl.replace(from, to)
        };
        if label == "reset-retains-pending" {
            // Preserve the initial reset, then inject failure to clear an
            // actually acquired pending state on a later reset.
            source = source.replace("  always @(posedge clk) begin", "  reg mutation_active = 0;\n  always @(posedge clk) mutation_active <= mutation_active | !rst;\n  always @(posedge clk) begin");
        }
        fs::write(
            case.join("observed.v"),
            source.replace("endmodule", &format!("{observer}\nendmodule")),
        )
        .unwrap();
        fs::write(case.join("mutation.sby"),format!("[options]\nmode bmc\ndepth 10\n[engines]\nsmtbmc --unroll z3\n[script]\nread -formal observed.v\nprep -top {top}\n[files]\nobserved.v\n")).unwrap();
        let log = fs::File::create(case.join("mutation.log")).unwrap();
        let start = Instant::now();
        let status = Command::new("timeout")
            .args(["--kill-after=5s", "180s", "sby", "-f", "mutation.sby"])
            .current_dir(&case)
            .stdout(Stdio::from(log.try_clone().unwrap()))
            .stderr(Stdio::from(log))
            .status()
            .unwrap();
        fs::write(case.join("execution.json"),serde_json::json!({"tool":"sby","args":["-f","mutation.sby"],"exit_code":status.code(),"elapsed_seconds":start.elapsed().as_secs_f64(),"oracle":"port-only","depth":10}).to_string()).unwrap();
        let result = fs::read_to_string(case.join("mutation/status")).unwrap();
        if label == "control" {
            assert_eq!(status.code(), Some(0));
            assert_eq!(result.split_whitespace().next(), Some("PASS"));
        } else {
            assert_eq!(status.code(), Some(2), "{}", case.display());
            assert_eq!(result.split_whitespace().next(), Some("FAIL"));
            assert!(
                fs::read_to_string(case.join("mutation.log"))
                    .unwrap()
                    .contains("Assert failed")
            );
            assert!(case.join("mutation/engine_0/trace.vcd").exists());
        }
    }
}
