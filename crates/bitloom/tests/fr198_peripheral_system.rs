//! FR198: two real four-peripheral compositions with independent RTL smoke vectors.
use bitloom_hir::PortDirection;
use bitloom_prelude::{
    Elaboratable, ElaborateSession, FrozenHir, GroundType, Span,
    ip::{AxiLiteCsrBridge, CsrDecoder, GpioCsr, Irq, Timer, UartCsr},
};
use sha2::{Digest, Sha256};
use std::{fs, path::PathBuf, process::Command, time::Instant};

const RANDOM_SEEDS: [u32; 16] = [
    0x1020_3040,
    0x89ab_cdef,
    0x1357_9bdf,
    0x2468_ace1,
    0xdead_beef,
    0xc001_d00d,
    0x3141_5926,
    0x2718_2818,
    0x0bad_f00d,
    0x55aa_aa55,
    0x7654_3210,
    0xfedc_ba98,
    0x1122_3344,
    0xa5a5_5a5a,
    0x7f4a_7c15,
    0x6d2b_79f5,
];

fn standalone(name: &str) -> FrozenHir {
    match name {
        "bridge" => AxiLiteCsrBridge::elaborate().unwrap(),
        "decoder" => CsrDecoder::elaborate().unwrap(),
        "uart" => UartCsr::elaborate().unwrap(),
        "gpio" => GpioCsr::elaborate().unwrap(),
        "timer" => Timer::elaborate().unwrap(),
        "irq" => Irq::elaborate().unwrap(),
        _ => unreachable!(),
    }
}
fn module_ports(kind: &str, _module: &str) -> Vec<bitloom_hir::Port> {
    let h = standalone(kind);
    let base = match kind {
        "bridge" => "AxiLiteCsrBridge",
        "decoder" => "CsrDecoder",
        "uart" => "UartCsr",
        "gpio" => "GpioCsr",
        "timer" => "Timer",
        "irq" => "Irq",
        _ => unreachable!(),
    };
    h.circuit()
        .modules
        .iter()
        .find(|m| m.name == base)
        .unwrap()
        .ports
        .clone()
}
fn width(ty: &GroundType) -> u32 {
    match ty {
        GroundType::UInt { width } => *width,
        GroundType::Clock | GroundType::Reset => 1,
        GroundType::SInt { width } => *width,
        GroundType::Bool | GroundType::Analog => 1,
    }
}
fn is_clockish(n: &str) -> bool {
    n == "clk" || n == "rst"
}
fn add_signal(s: &mut ElaborateSession, n: &str, p: &bitloom_hir::Port, external: bool, sp: Span) {
    if is_clockish(n) {
        return;
    }
    if external {
        match p.direction {
            PortDirection::Input => s.add_input(n, p.ty.clone(), sp),
            PortDirection::Output => s.add_output(n, p.ty.clone(), sp),
            PortDirection::InOut => s.declare_wire(
                n,
                GroundType::UInt {
                    width: width(&p.ty),
                },
                sp,
            ),
        };
    } else {
        s.declare_wire(
            n,
            GroundType::UInt {
                width: width(&p.ty),
            },
            sp,
        );
    }
}
fn connect_ports(
    s: &mut ElaborateSession,
    kind: &str,
    module: &str,
    prefix: &str,
    external: bool,
    sp: Span,
) -> Vec<(String, String)> {
    let mut c = vec![];
    for p in module_ports(kind, module) {
        let n = if is_clockish(&p.name) {
            p.name.clone()
        } else {
            format!("{prefix}{}", p.name)
        };
        let expose = external
            && match kind {
                "bridge" => !p.name.starts_with("csr_"),
                "decoder" => {
                    !p.name.contains("_")
                        || [
                            "req_valid",
                            "write",
                            "addr",
                            "wdata",
                            "wstrb",
                            "rsp_ready",
                            "req_ready",
                            "rsp_valid",
                            "rdata",
                            "error",
                        ]
                        .contains(&p.name.as_str())
                }
                "uart" => ["rx", "tx"].contains(&p.name.as_str()),
                "gpio" => ["pad_in", "pad_out", "pad_oe"].contains(&p.name.as_str()),
                "timer" => false,
                "irq" => ["irq"].contains(&p.name.as_str()),
                _ => false,
            };
        add_signal(s, &n, &p, expose, sp);
        c.push((p.name.clone(), n));
    }
    c
}
fn add_shared_io(s: &mut ElaborateSession, sp: Span) {
    s.add_input("clk", GroundType::Clock, sp);
    s.add_input("rst", GroundType::Reset, sp);
}
fn system(axi: bool) -> FrozenHir {
    let mut s = ElaborateSession::new(if axi {
        "Fr198AxiCore"
    } else {
        "Fr198DirectCore"
    });
    AxiLiteCsrBridge::define_module(&mut s, "Fr198Bridge").unwrap();
    CsrDecoder::define_module(&mut s, "Fr198Decoder").unwrap();
    UartCsr::define_module(&mut s, "Fr198Uart").unwrap();
    GpioCsr::define_module(&mut s, "Fr198Gpio").unwrap();
    Timer::define_module(&mut s, "Fr198Timer").unwrap();
    Irq::define_module(&mut s, "Fr198Irq").unwrap();
    let sp = Span::default();
    s.begin_module(
        if axi {
            "Fr198AxiCore"
        } else {
            "Fr198DirectCore"
        },
        sp,
    );
    add_shared_io(&mut s, sp);
    let bridge = if axi {
        Some(connect_ports(
            &mut s,
            "bridge",
            "AxiLiteCsrBridge",
            "axi_",
            true,
            sp,
        ))
    } else {
        None
    };
    let decoder = connect_ports(&mut s, "decoder", "CsrDecoder", "csr_", !axi, sp);
    // The decoder request/response interface is the direct topology's public bus,
    // or the internal bridge bus in the AXI topology.
    s.begin_combinational(sp);
    if let Some(b) = &bridge {
        // Bridge and decoder pins are connected by named top wires; preserve pin direction.
        let bridge_to_decoder = [
            "csr_req_valid",
            "csr_write",
            "csr_addr",
            "csr_wdata",
            "csr_wstrb",
            "csr_rsp_ready",
        ];
        for (bp, bn) in b {
            if bp.starts_with("csr_") {
                let dn = format!("csr_{}", &bp[4..]);
                if bridge_to_decoder.contains(&bp.as_str()) {
                    s.assign_net(&dn, bn, sp);
                } else {
                    s.assign_net(bn, &dn, sp);
                }
            }
        }
    }
    let mut leaf_conns = vec![];
    for (kind, module, prefix) in [
        ("uart", "Fr198Uart", "u_"),
        ("gpio", "Fr198Gpio", "g_"),
        ("timer", "Fr198Timer", "t_"),
        ("irq", "Fr198Irq", "i_"),
    ] {
        leaf_conns.push((
            kind.to_string(),
            connect_ports(&mut s, kind, module, prefix, true, sp),
        ));
    }
    // Wire decoder leaf channels to their matching real leaf instance.
    for (kind, lc) in &leaf_conns {
        let pfx = match kind.as_str() {
            "uart" => "uart_",
            "gpio" => "gpio_",
            "timer" => "timer_",
            "irq" => "irq_",
            _ => unreachable!(),
        };
        for (port, ln) in lc {
            if is_clockish(port) || port == "pad_in" || port == "rx" {
                continue;
            }
            let decoder_port = format!("{}{}", pfx, port);
            let dn = format!("csr_{decoder_port}");
            if module_ports("decoder", "CsrDecoder")
                .iter()
                .any(|p| p.name == decoder_port)
            {
                let leaf_outputs = [
                    "req_ready",
                    "rsp_valid",
                    "rdata",
                    "error",
                    "tx",
                    "raw_events",
                    "pad_out",
                    "pad_oe",
                    "match_event",
                    "irq",
                ];
                if leaf_outputs.contains(&port.as_str()) {
                    s.assign_net(&dn, ln, sp);
                } else {
                    s.assign_net(ln, &dn, sp);
                }
            }
        }
    }
    // Event aggregation: raw UART events plus timer and GPIO become the five raw IRQ inputs.
    s.declare_wire("uart_error_event", GroundType::UInt { width: 1 }, sp);
    s.declare_wire("irq_mid_a", GroundType::UInt { width: 2 }, sp);
    s.declare_wire("irq_mid_b", GroundType::UInt { width: 3 }, sp);
    s.declare_wire("irq_mid_c", GroundType::UInt { width: 4 }, sp);
    s.declare_wire("irq_events", GroundType::UInt { width: 5 }, sp);
    s.declare_wire("u_rx_event", GroundType::UInt { width: 1 }, sp);
    s.declare_wire("u_tx_event", GroundType::UInt { width: 1 }, sp);
    s.declare_wire("u_err_event", GroundType::UInt { width: 1 }, sp);
    s.declare_wire("u_framing_event", GroundType::UInt { width: 1 }, sp);
    s.assign_slice("u_rx_event", "u_raw_events", 0, 1, sp);
    s.assign_slice("u_tx_event", "u_raw_events", 1, 1, sp);
    s.assign_slice("u_err_event", "u_raw_events", 2, 1, sp);
    s.assign_slice("u_framing_event", "u_raw_events", 3, 1, sp);
    s.assign_or("uart_error_event", "u_err_event", "u_framing_event", sp);
    s.assign_concat("irq_mid_a", "u_rx_event", "t_match_event", sp);
    s.assign_concat("irq_mid_b", "u_tx_event", "irq_mid_a", sp);
    s.assign_concat("irq_mid_c", "uart_error_event", "irq_mid_b", sp);
    s.assign_concat("irq_events", "g_raw_event", "irq_mid_c", sp);
    for (kind, lc) in &leaf_conns {
        let module = match kind.as_str() {
            "uart" => "Fr198Uart",
            "gpio" => "Fr198Gpio",
            "timer" => "Fr198Timer",
            "irq" => "Fr198Irq",
            _ => unreachable!(),
        };
        let mut c = lc.clone();
        if kind == "irq" {
            for x in &mut c {
                if x.0 == "raw_events" {
                    x.1 = "irq_events".into();
                }
            }
        }
        s.add_instance(kind.clone(), module, c, vec![], sp);
    }
    if let Some(c) = bridge {
        s.add_instance("bridge", "Fr198Bridge", c, vec![], sp);
    }
    s.add_instance("decoder", "Fr198Decoder", decoder, vec![], sp);
    s.end_process();
    s.end_module();
    s.finish().unwrap()
}
fn emit_compile(label: &str, hir: &FrozenHir, tb: &str) {
    let artifact_root = std::env::var_os("BITLOOM_FR198_ARTIFACT_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/fr198"));
    let dir = artifact_root.join(format!("{label}-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let files = bitloom_vlog::emit(hir).files;
    let mut design = files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    design.push_str(if label == "axi" {
        AXI_RESET_BOUNDARY
    } else {
        DIRECT_RESET_BOUNDARY
    });
    fs::write(dir.join("design.v"), design).unwrap();
    fs::write(dir.join("tb.sv"), tb).unwrap();
    for seed in RANDOM_SEEDS {
        assert!(
            tb.contains(&format!("32'h{seed:08x}")),
            "{label} testbench is missing frozen random seed {seed:08x}"
        );
    }
    let top = if label == "axi" {
        "Fr198Axi"
    } else {
        "Fr198Direct"
    };
    let tool_identity = |tool: &str, version_arg: &str| {
        let path = Command::new("which")
            .arg(tool)
            .output()
            .unwrap_or_else(|_| panic!("required which executable is missing"));
        assert!(
            path.status.success(),
            "required {tool} executable is missing"
        );
        let version = Command::new(tool)
            .arg(version_arg)
            .output()
            .unwrap_or_else(|_| panic!("required {tool} executable is missing"));
        assert!(
            version.status.success(),
            "{tool} identity failed: {}",
            String::from_utf8_lossy(&version.stderr)
        );
        serde_json::json!({
            "path": String::from_utf8_lossy(&path.stdout).trim(),
            "stdout": String::from_utf8_lossy(&version.stdout),
            "stderr": String::from_utf8_lossy(&version.stderr),
            "exit_code": version.status.code(),
        })
    };
    let tools = serde_json::json!({
        "iverilog": tool_identity("iverilog", "-V"),
        "vvp": tool_identity("vvp", "-V"),
        "yosys": tool_identity("yosys", "-V"),
    });
    let started = Instant::now();
    let v = Command::new("iverilog")
        .args([
            "-g2012",
            "-s",
            "tb",
            "-o",
            "simulation",
            "design.v",
            "tb.sv",
        ])
        .current_dir(&dir)
        .output()
        .expect("required iverilog executable is missing");
    assert!(
        v.status.success(),
        "{label} iverilog failed: {}",
        String::from_utf8_lossy(&v.stderr)
    );
    fs::write(
        dir.join("compile.log"),
        [&v.stdout[..], &v.stderr[..]].concat(),
    )
    .unwrap();
    let r = Command::new("timeout")
        .args(["30s", "vvp", "simulation"])
        .current_dir(&dir)
        .output()
        .expect("required timeout executable is missing");
    let stdout = String::from_utf8_lossy(&r.stdout);
    let expected_marker = format!("FR198 PASS topology={label}");
    let assertions = stdout
        .split_whitespace()
        .find_map(|word| word.strip_prefix("assertions="))
        .and_then(|word| word.parse::<u32>().ok())
        .unwrap_or(0);
    let transactions = stdout
        .split_whitespace()
        .find_map(|word| word.strip_prefix("transactions="))
        .and_then(|word| word.parse::<u32>().ok())
        .unwrap_or(0);
    let random_transactions = stdout
        .split_whitespace()
        .find_map(|word| word.strip_prefix("random_transactions="))
        .and_then(|word| word.parse::<u32>().ok())
        .unwrap_or(0);
    assert!(
        r.status.success()
            && stdout.contains(&expected_marker)
            && assertions >= 250
            && transactions >= 60
            && random_transactions == 16_000,
        "{label} simulation failed: {}{}",
        stdout,
        String::from_utf8_lossy(&r.stderr)
    );
    let vcd = dir.join(format!("{label}.vcd"));
    assert!(
        fs::metadata(&vcd).is_ok_and(|m| m.len() > 1024),
        "{label} simulation did not produce a non-empty VCD"
    );
    fs::write(dir.join("run.log"), [&r.stdout[..], &r.stderr[..]].concat()).unwrap();
    let y = Command::new("yosys")
        .args([
            "-p",
            &format!("read_verilog -sv design.v; hierarchy -check -top {top}; proc; check -assert"),
        ])
        .current_dir(&dir)
        .output()
        .expect("yosys required");
    assert!(
        y.status.success(),
        "{label} yosys failed: {}",
        String::from_utf8_lossy(&y.stderr)
    );
    fs::write(
        dir.join("yosys.log"),
        [&y.stdout[..], &y.stderr[..]].concat(),
    )
    .unwrap();
    let sha = |bytes: &[u8]| format!("{:x}", Sha256::digest(bytes));
    fs::write(
        dir.join("evidence.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "topology": label,
            "commands": {
                "compile": ["iverilog", "-g2012", "-s", "tb", "-o", "simulation", "design.v", "tb.sv"],
                "simulate": ["timeout", "30s", "vvp", "simulation"],
                "structure": ["yosys", "-p", format!("read_verilog -sv design.v; hierarchy -check -top {top}; proc; check -assert")],
            },
            "exit_codes": {
                "compile": v.status.code(), "simulate": r.status.code(), "structure": y.status.code()
            },
            "assertions": assertions,
            "transactions": transactions,
            "random_transactions": random_transactions,
            "random_seeds": RANDOM_SEEDS.iter().map(|seed| format!("{seed:08x}")).collect::<Vec<_>>(),
            "elapsed_ms": started.elapsed().as_millis(),
            "sha256": {"design.v": sha(fs::read(dir.join("design.v")).unwrap().as_slice()), "tb.sv": sha(tb.as_bytes())},
            "vcd_bytes": fs::metadata(vcd).unwrap().len(),
            "tools": tools,
        }))
        .unwrap(),
    )
    .unwrap();
    println!("FR198 {label} PASS artifacts={}", dir.display());
}
const AXI_TB: &str = include_str!("fr198_peripheral_system/axi_tb.sv");
const CSR_TB: &str = include_str!("fr198_peripheral_system/direct_tb.sv");
const AXI_RESET_BOUNDARY: &str = r#"
module Fr198Axi(
 input clk,input aresetn,
 input [15:0] axi_s_axi_awaddr,input [2:0] axi_s_axi_awprot,input axi_s_axi_awvalid,output axi_s_axi_awready,
 input [31:0] axi_s_axi_wdata,input [3:0] axi_s_axi_wstrb,input axi_s_axi_wvalid,output axi_s_axi_wready,
 input axi_s_axi_bready,output axi_s_axi_bvalid,output [1:0] axi_s_axi_bresp,
 input [15:0] axi_s_axi_araddr,input [2:0] axi_s_axi_arprot,input axi_s_axi_arvalid,output axi_s_axi_arready,
 input axi_s_axi_rready,output axi_s_axi_rvalid,output [31:0] axi_s_axi_rdata,output [1:0] axi_s_axi_rresp,
 input u_rx,output u_tx,input [31:0] g_pad_in,output [31:0] g_pad_out,output [31:0] g_pad_oe,output i_irq);
 wire core_reset = ~aresetn;
 Fr198AxiCore core(.clk(clk),.rst(core_reset),.*);
endmodule
"#;
const DIRECT_RESET_BOUNDARY: &str = r#"
module Fr198Direct(
 input clk,input aresetn,
 input csr_req_valid,input csr_write,input [15:0] csr_addr,input [31:0] csr_wdata,input [3:0] csr_wstrb,input csr_rsp_ready,
 output csr_req_ready,output csr_rsp_valid,output [31:0] csr_rdata,output [1:0] csr_error,
 input u_rx,output u_tx,input [31:0] g_pad_in,output [31:0] g_pad_out,output [31:0] g_pad_oe,output i_irq);
 wire core_reset = ~aresetn;
 Fr198DirectCore core(.clk(clk),.rst(core_reset),.*);
endmodule
"#;
#[test]
fn fr198_axi_four_leaf_hierarchy_emits_and_runs() {
    let h = system(true);
    assert!(h.circuit().modules.iter().any(|m| m.name == "Fr198AxiCore"));
    emit_compile("axi", &h, AXI_TB);
}
#[test]
fn fr198_direct_csr_four_leaf_hierarchy_emits_and_runs() {
    let h = system(false);
    assert!(
        h.circuit()
            .modules
            .iter()
            .any(|m| m.name == "Fr198DirectCore")
    );
    emit_compile("direct", &h, CSR_TB);
}
#[test]
fn fr198_both_topologies_have_shared_real_leaf_definitions() {
    let a = system(true);
    let d = system(false);
    for n in [
        "Fr198Uart",
        "Fr198Gpio",
        "Fr198Timer",
        "Fr198Irq",
        "Fr198Decoder",
    ] {
        assert!(a.circuit().modules.iter().any(|m| m.name == n));
        assert!(d.circuit().modules.iter().any(|m| m.name == n));
    }
    assert!(a.circuit().modules.iter().any(|m| m.name == "Fr198Bridge"));
}
