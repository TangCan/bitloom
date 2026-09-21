use super::*;
use std::{fmt::Write as _, fs};
#[path = "common.rs"]
#[allow(dead_code)]
mod common;

#[test]
fn p0_two_actual_rtl_instances_keep_independent_owners_and_cancellation() {
    let mut s = ElaborateSession::new("DualDecoder");
    CsrDecoder::define_module(&mut s, "Decoder").unwrap();
    CsrDecoder::define_module(&mut s, "Decoder").unwrap();
    let sp = Span::default();
    s.begin_module("DualDecoder", sp);
    s.add_input("clk", GroundType::Clock, sp);
    s.add_input("rst", GroundType::Reset, sp);
    for inst in ["a", "b"] {
        let mut con = vec![];
        for (n, (d, ty)) in ports() {
            if n == "clk" || n == "rst" {
                con.push((n.clone(), n));
                continue;
            }
            let wire = format!("{inst}_{n}");
            if d == PortDirection::Input {
                s.add_input(&wire, ty, sp);
            } else {
                s.add_output(&wire, ty, sp);
            }
            con.push((n, wire));
        }
        s.add_instance(inst, "Decoder", con, vec![], sp);
    }
    s.end_module();
    let hir = s.finish().unwrap();
    assert_eq!(hir.circuit().modules.len(), 2);
    let mut tb = String::from("module tb;\nreg clk=0,rst=0;\n");
    for inst in ["a", "b"] {
        for (n, (d, ty)) in ports() {
            if n == "clk" || n == "rst" {
                continue;
            }
            let width = match ty {
                GroundType::UInt { width } => width,
                _ => 1,
            };
            writeln!(
                tb,
                "{} [{}:0] {inst}_{n}{};",
                if d == PortDirection::Input {
                    "reg"
                } else {
                    "wire"
                },
                width - 1,
                if d == PortDirection::Input { "=0" } else { "" }
            )
            .unwrap();
        }
    }
    tb.push_str(
        r#"
DualDecoder dut(.*);
task tick;begin #2;clk=1;#2;clk=0;#2;end endtask
initial begin
 rst=1;tick();rst=0;
 a_req_valid=1;a_addr=16'h0104;a_gpio_req_ready=1;
 b_req_valid=1;b_addr=16'hffff;
 #1;if(!a_req_ready||!b_req_ready||!a_gpio_req_valid)$fatal(1,"independent accepts");
 tick();a_req_valid=0;b_req_valid=0;
 a_gpio_rsp_valid=1;a_gpio_rdata=32'h12345678;a_gpio_error=0;
 a_uart_rsp_valid=1;a_uart_rdata=32'hdeadbeef;a_addr=16'h0000;
 #1;if(!a_rsp_valid||a_rdata!==32'h12345678||!b_rsp_valid||b_error!==3)$fatal(1,"owner isolation");
 repeat(31)tick();
 // Consume A while B continues holding its independent miss response.
 a_rsp_ready=1;tick();a_rsp_ready=0;a_gpio_rsp_valid=0;a_uart_rsp_valid=0;
 if(a_rsp_valid||!b_rsp_valid||b_error!==3)$fatal(1,"independent consumption");
 b_rsp_ready=1;tick();b_rsp_ready=0;
 repeat(31)begin tick();if(a_rsp_valid||b_rsp_valid)$fatal(1,"late duplicate");end
 a_req_valid=1;a_addr=16'h0207;a_timer_req_ready=1;
 #1;if(!a_timer_req_valid||a_timer_addr!==7||!a_req_ready)$fatal(1,"local low bits");
 tick();a_req_valid=0;a_timer_rsp_valid=1;a_timer_error=2;a_timer_rdata=0;
 #1;if(!a_rsp_valid||a_error!==2||a_rdata!==0)$fatal(1,"leaf error routing");
 a_rsp_ready=1;tick();a_timer_rsp_valid=0;
 a_rsp_ready=0;a_req_valid=1;a_addr=16'h0004;a_uart_req_ready=1;
 b_req_valid=1;b_addr=16'hffff;tick();a_req_valid=0;b_req_valid=0;a_uart_rsp_valid=1;
 #1;if(!a_rsp_valid||!b_rsp_valid)$fatal(1,"both pending before common reset");
 rst=1;tick();rst=0;a_uart_rsp_valid=0;
 repeat(31)begin tick();if(a_rsp_valid||b_rsp_valid)$fatal(1,"common reset replay");end
 $display("DUAL RTL PASS");$finish;
end
endmodule
"#,
    );
    let dir = common::dir("dual");
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
    common::run(
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
    assert!(common::run(&dir, "vvp", &["simulation"], "run").contains("DUAL RTL PASS"));
}
