//! Story 125.2: independent protocol expectations for the legacy 8-bit AXI bank.
//! Raw-channel driver; this does not claim cocotbext-axi BFM coverage.
//! Every frame drives once, checks before the edge, ticks once, then checks after.
use bitloom_hir::PortValues;
use bitloom_prelude::{Elaboratable, ip::Axi4LiteSlave};
use bitloom_sim::{Sim, TickEngine};
use std::{fmt::Write as _, fs, path::PathBuf, process::Command};

const INPUTS: [(&str, u32); 10] = [
    ("rst", 1),
    ("s_axi_awaddr", 8),
    ("s_axi_awvalid", 1),
    ("s_axi_wdata", 32),
    ("s_axi_wstrb", 4),
    ("s_axi_wvalid", 1),
    ("s_axi_bready", 1),
    ("s_axi_araddr", 8),
    ("s_axi_arvalid", 1),
    ("s_axi_rready", 1),
];
const OUTPUTS: [(&str, u32); 8] = [
    ("s_axi_awready", 1),
    ("s_axi_wready", 1),
    ("s_axi_bresp", 2),
    ("s_axi_bvalid", 1),
    ("s_axi_arready", 1),
    ("s_axi_rdata", 32),
    ("s_axi_rresp", 2),
    ("s_axi_rvalid", 1),
];
#[derive(Debug)]
struct Frame {
    label: &'static str,
    inputs: Vec<(&'static str, u64)>,
    pre: Vec<(&'static str, u64)>,
    post: Vec<(&'static str, u64)>,
}
fn frame(
    label: &'static str,
    inputs: &[(&'static str, u64)],
    pre: &[(&'static str, u64)],
    post: &[(&'static str, u64)],
) -> Frame {
    Frame {
        label,
        inputs: inputs.to_vec(),
        pre: pre.to_vec(),
        post: post.to_vec(),
    }
}
fn readback(frames: &mut Vec<Frame>, addr: u64, expected: u64) {
    frames.push(frame(
        "readback",
        &[("s_axi_arvalid", 1), ("s_axi_araddr", addr)],
        &[("s_axi_arready", 1)],
        &[
            ("s_axi_rvalid", 1),
            ("s_axi_rdata", expected),
            ("s_axi_rresp", 0),
        ],
    ));
    frames.push(frame(
        "consume read once",
        &[("s_axi_rready", 1)],
        &[("s_axi_rvalid", 1)],
        &[("s_axi_rvalid", 0)],
    ));
}
fn frames(scenario: &str) -> Vec<Frame> {
    let mut f = vec![frame(
        "reset",
        &[("rst", 1)],
        &[],
        &[("s_axi_bvalid", 0), ("s_axi_rvalid", 0)],
    )];
    match scenario {
        "aw_first" => {
            f.push(frame(
                "AW accepted at original address",
                &[("s_axi_awaddr", 4), ("s_axi_awvalid", 1)],
                &[("s_axi_awready", 1)],
                &[("s_axi_bvalid", 0)],
            ));
            f.push(frame(
                "AW withdrawn and address poisoned",
                &[("s_axi_awaddr", 12)],
                &[],
                &[("s_axi_bvalid", 0)],
            ));
            f.push(frame(
                "delayed W must produce B",
                &[
                    ("s_axi_awaddr", 12),
                    ("s_axi_wvalid", 1),
                    ("s_axi_wdata", 0x11223344),
                    ("s_axi_wstrb", 15),
                ],
                &[("s_axi_wready", 1)],
                &[("s_axi_bvalid", 1), ("s_axi_bresp", 0)],
            ));
        }
        "w_first" => {
            f.push(frame(
                "W accepted with partial strobe",
                &[
                    ("s_axi_wvalid", 1),
                    ("s_axi_wdata", 0x11223344),
                    ("s_axi_wstrb", 5),
                ],
                &[("s_axi_wready", 1)],
                &[("s_axi_bvalid", 0)],
            ));
            f.push(frame(
                "W withdrawn and payload poisoned",
                &[("s_axi_wdata", 0xdeadbeef), ("s_axi_wstrb", 10)],
                &[],
                &[("s_axi_bvalid", 0)],
            ));
            f.push(frame(
                "delayed AW must produce B",
                &[
                    ("s_axi_awaddr", 4),
                    ("s_axi_awvalid", 1),
                    ("s_axi_wdata", 0xdeadbeef),
                    ("s_axi_wstrb", 10),
                ],
                &[("s_axi_awready", 1)],
                &[("s_axi_bvalid", 1), ("s_axi_bresp", 0)],
            ));
        }
        "concurrent" => {
            f.push(frame(
                "seed old value",
                &[
                    ("s_axi_awaddr", 4),
                    ("s_axi_awvalid", 1),
                    ("s_axi_wvalid", 1),
                    ("s_axi_wdata", 0xaabbccdd),
                    ("s_axi_wstrb", 15),
                ],
                &[("s_axi_awready", 1), ("s_axi_wready", 1)],
                &[("s_axi_bvalid", 1)],
            ));
            f.push(frame(
                "consume seed response",
                &[("s_axi_bready", 1)],
                &[("s_axi_bvalid", 1)],
                &[("s_axi_bvalid", 0)],
            ));
            f.push(frame(
                "concurrent same-address read must produce R and B",
                &[
                    ("s_axi_awaddr", 4),
                    ("s_axi_awvalid", 1),
                    ("s_axi_wvalid", 1),
                    ("s_axi_wdata", 0x11223344),
                    ("s_axi_wstrb", 15),
                    ("s_axi_araddr", 4),
                    ("s_axi_arvalid", 1),
                ],
                &[
                    ("s_axi_awready", 1),
                    ("s_axi_wready", 1),
                    ("s_axi_arready", 1),
                ],
                &[
                    ("s_axi_bvalid", 1),
                    ("s_axi_rvalid", 1),
                    ("s_axi_rdata", 0xaabbccdd),
                    ("s_axi_bresp", 0),
                    ("s_axi_rresp", 0),
                ],
            ));
            f.push(frame(
                "hold concurrent read snapshot",
                &[],
                &[],
                &[
                    ("s_axi_rvalid", 1),
                    ("s_axi_rdata", 0xaabbccdd),
                    ("s_axi_bvalid", 1),
                ],
            ));
            f.push(frame(
                "consume concurrent read",
                &[("s_axi_rready", 1)],
                &[("s_axi_rvalid", 1)],
                &[("s_axi_rvalid", 0), ("s_axi_bvalid", 1)],
            ));
        }
        _ => panic!("unknown scenario"),
    }
    f.push(frame(
        "hold B under backpressure",
        &[],
        &[],
        &[("s_axi_bvalid", 1), ("s_axi_bresp", 0)],
    ));
    f.push(frame(
        "consume B once",
        &[("s_axi_bready", 1)],
        &[("s_axi_bvalid", 1)],
        &[("s_axi_bvalid", 0)],
    ));
    f.push(frame(
        "no duplicate responses",
        &[("s_axi_bready", 1), ("s_axi_rready", 1)],
        &[],
        &[("s_axi_bvalid", 0), ("s_axi_rvalid", 0)],
    ));
    readback(
        &mut f,
        4,
        if scenario == "w_first" {
            0x00220044
        } else {
            0x11223344
        },
    );
    readback(&mut f, 12, 0);
    f
}
fn artifact_dir(scenario: &str, backend: &str) -> PathBuf {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr193-axi")
        .join(scenario)
        .join(backend);
    fs::create_dir_all(&p).unwrap();
    p
}
fn testbench(frames: &[Frame]) -> String {
    let mut tb = String::from("module tb; reg clk=0;\n");
    for (name, width) in INPUTS {
        writeln!(tb, "reg [{}:0] {name}=0;", width - 1).unwrap();
    }
    for (name, width) in OUTPUTS {
        writeln!(tb, "wire [{}:0] {name};", width - 1).unwrap();
    }
    tb.push_str("Axi4LiteSlave dut(.*); initial begin\n");
    for (index, f) in frames.iter().enumerate() {
        writeln!(tb, "// Frame {index}: {}", f.label).unwrap();
        for (name, _) in INPUTS {
            let value = f
                .inputs
                .iter()
                .find(|(n, _)| *n == name)
                .map_or(0, |(_, v)| *v);
            writeln!(tb, "{name}=64'd{value};").unwrap();
        }
        tb.push_str("#5;\n");
        for (phase, checks) in [("pre", &f.pre), ("post", &f.post)] {
            if phase == "post" {
                tb.push_str("clk=1; #5;\n");
            }
            for (name, value) in checks {
                writeln!(tb, "$display(\"frame {index} {phase} {name}=%h expected={value}\", {name});\nif ({name} !== 64'd{value}) $fatal(1, \"PROTOCOL frame {index} {} {phase} {name}: expected {value}, got %h\", {name});", f.label).unwrap();
            }
        }
        tb.push_str("clk=0; #5;\n");
    }
    tb.push_str("$display(\"PROTOCOL PASS\"); $finish; end endmodule\n");
    tb
}
fn run(scenario: &str, engine: Option<TickEngine>) {
    let backend = match engine {
        Some(TickEngine::Interpreter) => "interpreter",
        Some(TickEngine::Compiled) => "compiled",
        None => "rtl",
    };
    let dir = artifact_dir(scenario, backend);
    let h = Axi4LiteSlave::elaborate().expect("elaborate legacy bank");
    let f = frames(scenario);
    fs::write(
        dir.join("design.v"),
        &bitloom_vlog::emit(&h).files[0].contents,
    )
    .unwrap();
    fs::write(dir.join("tb.sv"), testbench(&f)).unwrap();
    fs::write(dir.join("frames.txt"), format!("{f:#?}\n")).unwrap();
    if let Some(engine) = engine {
        let mut sim = Sim::with_engine(h, engine);
        let mut log = String::new();
        for (index, frame) in f.iter().enumerate() {
            let mut inputs = PortValues::default();
            for (name, _) in INPUTS {
                inputs.set(name, 0);
            }
            for (name, value) in &frame.inputs {
                inputs.set(*name, *value);
            }
            sim.set_inputs(inputs);
            sim.settle();
            for (phase, checks) in [("pre", &frame.pre), ("post", &frame.post)] {
                if phase == "post" {
                    sim.tick();
                }
                for (name, expected) in checks {
                    let actual = sim.ports().get(name);
                    writeln!(
                        log,
                        "frame {index} {} {phase} {name}={actual:?} expected={expected}",
                        frame.label
                    )
                    .unwrap();
                    fs::write(dir.join("native.log"), &log).unwrap();
                    assert_eq!(
                        actual,
                        Some(*expected),
                        "PROTOCOL {scenario}/{backend} frame {index} {} {phase} {name}; evidence {}",
                        frame.label,
                        dir.display()
                    );
                }
            }
        }
    } else {
        // Explicit RTL tests always require tools: missing tools never count as red evidence.
        // BITLOOM_REQUIRE_RTL=1 is accepted by the documented strict invocation.
        let compile = Command::new("iverilog")
            .current_dir(&dir)
            .args([
                "-g2012",
                "-s",
                "tb",
                "-o",
                "simulation",
                "design.v",
                "tb.sv",
            ])
            .output()
            .expect("TOOL ERROR: iverilog required on PATH for explicit RTL test");
        fs::write(
            dir.join("compile.log"),
            format!(
                "status={}\n{}{}",
                compile.status,
                String::from_utf8_lossy(&compile.stdout),
                String::from_utf8_lossy(&compile.stderr)
            ),
        )
        .unwrap();
        assert!(
            compile.status.success(),
            "TOOL ERROR: RTL compile failed; see {}",
            dir.display()
        );
        let output = Command::new("vvp")
            .current_dir(&dir)
            .arg("simulation")
            .output()
            .expect("TOOL ERROR: vvp required on PATH");
        let log = format!(
            "status={}\n{}{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        fs::write(dir.join("rtl.log"), &log).unwrap();
        assert!(
            output.status.success(),
            "RTL {scenario}: {log}; evidence {}",
            dir.display()
        );
        assert!(
            log.contains("PROTOCOL PASS"),
            "TOOL ERROR: RTL did not complete protocol driver"
        );
    }
}
#[test]
fn legacy_axi_emits_protocol_fixture() {
    let h = Axi4LiteSlave::elaborate().unwrap();
    let v = bitloom_vlog::emit(&h).files[0].contents.clone();
    assert!(v.contains("module Axi4LiteSlave"));
    for (name, _) in INPUTS.into_iter().chain(OUTPUTS) {
        assert!(v.contains(name));
    }
    for scenario in ["aw_first", "w_first", "concurrent"] {
        let d = artifact_dir(scenario, "emit");
        fs::write(d.join("design.v"), &v).unwrap();
        fs::write(d.join("tb.sv"), testbench(&frames(scenario))).unwrap();
    }
}
macro_rules! red_case {
    ($name:ident, $scenario:literal, $engine:expr) => {
        #[test]
        #[ignore = "125.2 reproduces unfixed AW/W or concurrent AR transaction loss; enable in 125.3"]
        fn $name() { run($scenario, $engine); }
    };
}
red_case!(
    aw_first_interpreter,
    "aw_first",
    Some(TickEngine::Interpreter)
);
red_case!(aw_first_compiled, "aw_first", Some(TickEngine::Compiled));
red_case!(aw_first_rtl, "aw_first", None);
red_case!(
    w_first_interpreter,
    "w_first",
    Some(TickEngine::Interpreter)
);
red_case!(w_first_compiled, "w_first", Some(TickEngine::Compiled));
red_case!(w_first_rtl, "w_first", None);
red_case!(
    concurrent_interpreter,
    "concurrent",
    Some(TickEngine::Interpreter)
);
red_case!(
    concurrent_compiled,
    "concurrent",
    Some(TickEngine::Compiled)
);
red_case!(concurrent_rtl, "concurrent", None);
