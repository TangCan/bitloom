//! Actual emitted bridge/decoder/four-CsrBlock hierarchy. Peers are not peripherals.
#[path = "fr196_csr_decoder/common.rs"]
mod common;
use bitloom_hir::PortDirection;
use bitloom_prelude::{GroundType, ip::CsrAccess};
use common::*;
use std::{fmt::Write as _, fs};
#[test]
fn p0_real_hierarchy_routes_errors_partial_write_and_common_reset() {
    let hir = hierarchy();
    let mut tb = String::from("module tb;\n");
    for p in &hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == "DecoderSystem")
        .unwrap()
        .ports
    {
        let w = match p.ty {
            GroundType::UInt { width } => width,
            _ => 1,
        };
        let input = p.direction == PortDirection::Input;
        writeln!(
            tb,
            "{} [{}:0] {}{};",
            if input { "reg" } else { "wire" },
            w - 1,
            p.name,
            if input { "=0" } else { "" }
        )
        .unwrap();
    }
    tb.push_str("DecoderSystem dut(.*);\n");
    // EVENT uses the formal uppercase spelling because lowercase event is a reserved SV identifier.
    // The external peer owns RW state. Update only on real leaf commit with
    // the emitted candidate; hardware protocol/address assertions stay independent.
    tb.push_str("always @(posedge clk) begin\n");
    for b in banks() {
        for r in b.registers {
            if r.access == CsrAccess::Rw {
                writeln!(tb,"if(rst) {0}_{1}_value<=0; else if({0}_{1}_write_commit) {0}_{1}_value<={0}_{1}_candidate;",b.name,r.name).unwrap();
            }
        }
    }
    tb.push_str("end\n");
    tb.push_str(r#"
integer submits=0, responses=0, bs=0, rs=0, leaf_submits=0, gpio_out_writes=0;
integer before_s,before_l,before_b,before_r,before_w,k;
task tick;begin
 #2;
 if(!rst)begin
  if(csr_req_valid&&csr_req_ready)submits=submits+1;
  if(csr_rsp_valid&&csr_rsp_ready)responses=responses+1;
  if(s_axi_bvalid&&s_axi_bready)bs=bs+1;
  if(s_axi_rvalid&&s_axi_rready)rs=rs+1;
  leaf_submits=leaf_submits+(uart_req_valid&&uart_req_ready)+(gpio_req_valid&&gpio_req_ready)+(timer_req_valid&&timer_req_ready)+(irq_req_valid&&irq_req_ready);
  gpio_out_writes=gpio_out_writes+gpio_out_write_commit;
 end
 // External peer samples pre-edge values and updates in NBA, like real registers.
 clk=1;#2;clk=0;#2;
end endtask
task aw(input[15:0] a);integer age;reg taken;begin
 s_axi_awaddr=a;s_axi_awvalid=1;age=0;taken=0;
 while(!taken)begin #1;taken=s_axi_awready;tick();age=age+1;if(age>128)$fatal(1,"AW timeout");end
 s_axi_awvalid=0;
end endtask
task w(input[31:0] d,input[3:0] st);integer age;reg taken;begin
 s_axi_wdata=d;s_axi_wstrb=st;s_axi_wvalid=1;age=0;taken=0;
 while(!taken)begin #1;taken=s_axi_wready;tick();age=age+1;if(age>128)$fatal(1,"W timeout");end
 s_axi_wvalid=0;
end endtask
task b(input[1:0] err);integer age,start_b;begin
 age=0;start_b=bs;while(!s_axi_bvalid)begin tick();age=age+1;if(age>128)$fatal(1,"B timeout");end
 repeat(7)begin if(!s_axi_bvalid||s_axi_bresp!==err)$fatal(1,"B error/hold");tick();end
 s_axi_bready=1;tick();s_axi_bready=0;if(bs!=start_b+1)$fatal(1,"B duplicate/drop");
end endtask
task wr(input[15:0] a,input[31:0] d,input[1:0] err);begin aw(a);w(d,15);b(err);end endtask
task rd(input[15:0] a,input[31:0] data,input[1:0] err);integer age,start_r;reg taken;begin
 s_axi_araddr=a;s_axi_arvalid=1;age=0;taken=0;start_r=rs;
 while(!taken)begin #1;taken=s_axi_arready;tick();age=age+1;if(age>128)$fatal(1,"AR timeout");end
 s_axi_arvalid=0;age=0;
 while(!s_axi_rvalid)begin tick();age=age+1;if(age>128)$fatal(1,"R timeout");end
 repeat(7)begin if(!s_axi_rvalid||s_axi_rresp!==err||s_axi_rdata!==data)$fatal(1,"R addr=%h got=%h err=%h want=%h err=%h",a,s_axi_rdata,s_axi_rresp,data,err);tick();end
 s_axi_rready=1;tick();s_axi_rready=0;if(rs!=start_r+1)$fatal(1,"R duplicate/drop");
end endtask
initial begin
 rst=1;tick();rst=0;
 // Four identical local offsets keep independent values (IRQ enable mask=31).
 wr(16'h0004,32'h12345678,0);wr(16'h0104,32'h89abcdef,0);
 wr(16'h0204,32'h55aa33cc,0);wr(16'h0304,32'h1b,0);
 rd(16'h0304,32'h1b,0);rd(16'h0104,32'h89abcdef,0);
 rd(16'h0004,32'h12345678,0);rd(16'h0204,32'h55aa33cc,0);
 // Real CsrBlock holes and low byte bits remain SLVERR, never DECERR.
 rd(16'h0018,0,2);rd(16'h0118,0,2);rd(16'h0210,0,2);rd(16'h0310,0,2);
 rd(16'h00ff,0,2);rd(16'h01ff,0,2);rd(16'h02ff,0,2);rd(16'h03ff,0,2);
 rd(16'h0105,0,2);rd(16'h0106,0,2);rd(16'h0107,0,2);
 before_l=leaf_submits;
 rd(16'h0400,0,3);rd(16'h0401,0,3);rd(16'h8104,0,3);rd(16'hffff,0,3);
 wr(16'h8000,32'hffffffff,3);
 if(leaf_submits!=before_l)$fatal(1,"miss touched a leaf");
 rd(16'h0104,32'h89abcdef,0);
 // Partial AW cannot starve a qualified read; payload waits in bridge capture.
 before_w=gpio_out_writes;aw(16'h0104);
 rd(16'h0004,32'h12345678,0);
 if(gpio_out_writes!=before_w)$fatal(1,"partial AW committed");
 w(32'hcafebabe,15);b(0);
 if(gpio_out_writes!=before_w+1)$fatal(1,"write side effect count");
 rd(16'h0104,32'hcafebabe,0);
 if(submits!=responses||responses!=bs+rs)$fatal(1,"drained ledger");
 // Cancel captured-only AW under common reset; captured AW must not replay.
 aw(16'h0104);before_s=submits;before_b=bs;before_r=rs;before_w=gpio_out_writes;
 rst=1;tick();rst=0;s_axi_bready=1;s_axi_rready=1;repeat(12)tick();s_axi_bready=0;s_axi_rready=0;
 if(submits!=before_s||bs!=before_b||rs!=before_r||gpio_out_writes!=before_w)$fatal(1,"reset replay");
 if(s_axi_bvalid||s_axi_rvalid||gpio_out_value!==0)$fatal(1,"common reset failed");
 rd(16'h0204,0,0);wr(16'h0304,7,0);rd(16'h0304,7,0);
 before_s=submits;before_b=bs;before_r=rs;s_axi_bready=1;s_axi_rready=1;repeat(31)tick();
 if(submits!=before_s||bs!=before_b||rs!=before_r)$fatal(1,"late duplicate");
 $display("FR196 DECODER PASS submits=%0d responses=%0d leaf=%0d B=%0d R=%0d",submits,responses,leaf_submits,bs,rs);$finish;
end
initial begin #1000000;$fatal(1,"watchdog");end
endmodule
"#);
    let dir = dir("rtl");
    let rtl = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(dir.join("design.v"), rtl).unwrap();
    fs::write(dir.join("tb.sv"), tb).unwrap();
    run(&dir, "iverilog", &["-V"], "iverilog-version");
    run(&dir, "vvp", &["-V"], "vvp-version");
    run(
        &dir,
        "iverilog",
        &[
            "-g2012",
            "-s",
            "tb",
            "-o",
            "simulation",
            "design.v",
            "tb.sv",
        ],
        "compile",
    );
    assert!(run(&dir, "vvp", &["simulation"], "run").contains("FR196 DECODER PASS"));
}
#[test]
fn p1_four_local_headers_repeat_and_compile_with_independent_global_goldens() {
    let dir = dir("headers");
    let banks = banks();
    // Handwritten golden offsets/masks and bases: not derived from CsrBlock.
    // Absolute addresses and software-facing semantics are independent literals.
    let expected: [(&str, u32, &[(&str, u32, u32, u32, &str, &str)]); 4] = [
        (
            "UART",
            0x0000,
            &[
                ("CTRL", 0, 1, 0x0000, "RW", "External"),
                ("BAUD_DIV", 4, 0xffffffff, 0x0004, "RW", "External"),
                ("STATUS", 8, 15, 0x0008, "RO", "External"),
                ("TX_DATA", 12, 255, 0x000c, "WO", "None"),
                ("RX_DATA", 16, 255, 0x0010, "RO", "External"),
                ("EVENT", 20, 15, 0x0014, "W1C", "Leaf"),
            ],
        ),
        (
            "GPIO",
            0x0100,
            &[
                ("DIR", 0, 0xffffffff, 0x0100, "RW", "External"),
                ("OUT", 4, 0xffffffff, 0x0104, "RW", "External"),
                ("IN", 8, 0xffffffff, 0x0108, "RO", "External"),
                ("SET", 12, 0xffffffff, 0x010c, "WO", "None"),
                ("CLEAR", 16, 0xffffffff, 0x0110, "WO", "None"),
                ("RISE_EVENT", 20, 0xffffffff, 0x0114, "W1C", "Leaf"),
            ],
        ),
        (
            "TIMER",
            0x0200,
            &[
                ("CTRL", 0, 3, 0x0200, "RW", "External"),
                ("COUNT", 4, 0xffffffff, 0x0204, "RW", "External"),
                ("COMPARE", 8, 0xffffffff, 0x0208, "RW", "External"),
                ("EVENT", 12, 1, 0x020c, "W1C", "Leaf"),
            ],
        ),
        (
            "IRQ",
            0x0300,
            &[
                ("PENDING", 0, 31, 0x0300, "W1C", "Leaf"),
                ("ENABLE", 4, 31, 0x0304, "RW", "External"),
                ("TEST", 8, 31, 0x0308, "WO", "None"),
                ("RAW", 12, 31, 0x030c, "RO", "External"),
            ],
        ),
    ];
    let mut c = String::from(
        "#include <stdint.h>\n#include \"uart.h\"\n#include \"gpio.h\"\n#include \"timer.h\"\n#include \"irq.h\"\n",
    );
    for (j, b) in banks.iter().enumerate() {
        let h = b.emit_c_header().unwrap();
        assert_eq!(h, b.emit_c_header().unwrap());
        let markdown = b.emit_markdown().unwrap();
        assert_eq!(markdown, b.emit_markdown().unwrap());
        fs::write(dir.join(format!("{}.md", LEAVES[j])), &markdown).unwrap();
        fs::write(dir.join(format!("{}.h", LEAVES[j])), h).unwrap();
        let (prefix, base, regs) = expected[j];
        writeln!(c, "#define TEST_{prefix}_BASE UINT32_C(0x{base:04x})").unwrap();
        let rows: Vec<_> = markdown
            .lines()
            .filter_map(|line| {
                let columns: Vec<_> = line.split('|').map(str::trim).collect();
                (columns.len() == 11 && columns[2].starts_with("0x")).then_some(columns)
            })
            .collect();
        assert_eq!(rows.len(), regs.len(), "{prefix} register row count");
        for &(reg, offset, mask, absolute, access, owner) in regs {
            let markdown_name = if reg == "EVENT" {
                reg.to_owned()
            } else {
                reg.to_ascii_lowercase()
            };
            let matching: Vec<_> = rows.iter().filter(|row| row[1] == markdown_name).collect();
            assert_eq!(
                matching.len(),
                1,
                "{prefix} {reg}: unique Markdown register row"
            );
            assert_eq!(
                matching[0][2],
                format!("0x{offset:04x}"),
                "{prefix} {reg} Markdown offset"
            );
            assert_eq!(
                matching[0][3],
                format!("0x{mask:08x}"),
                "{prefix} {reg} Markdown mask"
            );
            assert_eq!(
                matching[0][4], "0x00000000",
                "{prefix} {reg} Markdown reset"
            );
            assert_eq!(matching[0][5], access, "{prefix} {reg} Markdown access");
            assert_eq!(matching[0][6], owner, "{prefix} {reg} Markdown owner");
            writeln!(c,"_Static_assert({prefix}_{reg}_OFFSET == UINT32_C(0x{offset:x}),\"local\");\n_Static_assert({prefix}_{reg}_MASK == UINT32_C(0x{mask:x}),\"mask\");\n_Static_assert(TEST_{prefix}_BASE + {prefix}_{reg}_OFFSET == UINT32_C(0x{absolute:04x}),\"global\");").unwrap();
        }
        writeln!(c,"_Static_assert(TEST_{prefix}_BASE + UINT32_C(0xff) == UINT32_C(0x{:x}),\"window-end\");",[0xff,0x1ff,0x2ff,0x3ff][j]).unwrap();
    }
    c.push_str("int main(void) { return 0; }\n");
    fs::write(dir.join("consumer.c"), c).unwrap();
    run(&dir, "cc", &["--version"], "cc-version");
    run(
        &dir,
        "cc",
        &[
            "-std=c11",
            "-Wall",
            "-Wextra",
            "-Werror",
            "consumer.c",
            "-o",
            "consumer",
        ],
        "compile",
    );
    run(&dir, "./consumer", &[], "consume");
}

#[test]
fn p0_independent_scoreboard_directed_random_and_concurrent_real_rtl() {
    let hir = hierarchy();
    let mut tb = bench(&hir);
    tb.push_str(include_str!("fr196_csr_decoder/scoreboard.sv"));
    tb.push_str(include_str!("fr196_csr_decoder/stress.sv"));
    let dir = dir("stress");
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
    fs::write(dir.join("tb.sv"), tb).unwrap();
    run(
        &dir,
        "iverilog",
        &[
            "-g2012",
            "-s",
            "tb",
            "-o",
            "simulation",
            "design.v",
            "tb.sv",
        ],
        "compile",
    );
    for seed in [
        1u32, 7, 31, 127, 1021, 4093, 65521, 1048573, 0x1234567, 0x2345678, 0x3456789, 0x456789a,
        0x56789ab, 0x6789abc, 0x789abcd, 0x1234abcd,
    ] {
        let out = run(
            &dir,
            "vvp",
            &["simulation", &format!("+seed={seed}")],
            &format!("seed-{seed}"),
        );
        assert!(out.contains("STRESS PASS"));
        eprintln!("{}", out.trim());
    }
}

#[test]
fn p0_real_monitor_rejects_changed_stalled_payload_and_consume_refill() {
    use std::process::{Command, Stdio};
    for (label, stimulus, expected) in [
        ("control", "transact(1,'h0104,42,15,7,0,7);", None),
        (
            "aw-change",
            "send_aw('h0104);s_axi_awvalid=1;s_axi_awaddr='h0204;tick();s_axi_awaddr='h0304;tick();",
            Some("MONITOR AW"),
        ),
        (
            "aw-accept-change",
            "send_aw('h0104);s_axi_awvalid=1;s_axi_awaddr='h0204;tick();force s_axi_awready=1;s_axi_awaddr='h0304;tick();",
            Some("MONITOR AW"),
        ),
        (
            "w-change",
            "send_w(1,15);s_axi_wvalid=1;s_axi_wdata=2;tick();s_axi_wdata=3;tick();",
            Some("MONITOR W"),
        ),
        (
            "w-accept-change",
            "send_w(1,15);s_axi_wvalid=1;s_axi_wdata=2;tick();force s_axi_wready=1;s_axi_wdata=3;tick();",
            Some("MONITOR W"),
        ),
        (
            "ar-accept-change",
            "send_ar('h0104);s_axi_arvalid=1;s_axi_araddr='h0204;tick();force s_axi_arready=1;s_axi_araddr='h0304;tick();",
            Some("MONITOR AR"),
        ),
        (
            "ar-change",
            "send_ar('h0104);s_axi_arvalid=1;s_axi_araddr='h0204;tick();s_axi_araddr='h0304;tick();",
            Some("MONITOR AR"),
        ),
        (
            "bubble",
            "force csr_req_valid=1;force csr_req_ready=1;force csr_rsp_valid=1;force csr_rsp_ready=1;tick();",
            Some("CSR consume/refill bubble"),
        ),
    ] {
        let hir = hierarchy();
        let mut tb = bench(&hir);
        tb.push_str(include_str!("fr196_csr_decoder/scoreboard.sv"));
        tb.push_str(&format!("initial begin reset_all();{stimulus}$display(\"CONTROL PASS\");$finish;end\nendmodule\n"));
        let dir = dir(&format!("monitor-{label}"));
        fs::write(dir.join("tb.sv"), tb).unwrap();
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
            "iverilog",
            &[
                "-g2012",
                "-s",
                "tb",
                "-o",
                "simulation",
                "design.v",
                "tb.sv",
            ],
            "compile",
        );
        let log = fs::File::create(dir.join("run.log")).unwrap();
        let started = std::time::Instant::now();
        let status = Command::new("timeout")
            .args(["60s", "vvp", "simulation"])
            .current_dir(&dir)
            .stdout(Stdio::from(log.try_clone().unwrap()))
            .stderr(Stdio::from(log))
            .status()
            .unwrap();
        let output = fs::read_to_string(dir.join("run.log")).unwrap();
        fs::write(dir.join("run.json"),serde_json::json!({"command":["timeout","60s","vvp","simulation"],"exit_code":status.code(),"elapsed_seconds":started.elapsed().as_secs_f64(),"expected_rejection":expected}).to_string()).unwrap();
        if let Some(message) = expected {
            assert_eq!(status.code(), Some(1), "{output}");
            assert!(output.contains(message), "{output}");
        } else {
            assert!(status.success(), "{output}");
            assert!(output.contains("CONTROL PASS"));
        }
    }
}

#[test]
fn p1_seven_module_native_and_generated_models_explicitly_reject_hierarchy() {
    use bitloom_sim::{GeneratedFunctional, Sim, TickEngine};
    let hir = hierarchy();
    let mut errors = vec![];
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        errors.push(
            std::panic::catch_unwind(|| Sim::with_engine(hir.clone(), engine))
                .err()
                .expect("hierarchy must reject"),
        );
    }
    errors.push(
        std::panic::catch_unwind(|| GeneratedFunctional::from_hir(&hir))
            .err()
            .expect("hierarchy must reject"),
    );
    for payload in errors {
        let message = payload
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| payload.downcast_ref::<&str>().copied())
            .unwrap_or("");
        assert!(
            message.contains("hierarchical simulation is unsupported"),
            "unexpected panic: {message}"
        );
    }
}
