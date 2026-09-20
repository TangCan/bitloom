//! Stories 125.2/125.3: independent protocol expectations for the legacy 8-bit AXI bank.
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
macro_rules! protocol_case {
    ($name:ident, $scenario:literal, $engine:expr) => {
        #[test]
        fn $name() {
            run($scenario, $engine);
        }
    };
}
protocol_case!(
    aw_first_interpreter,
    "aw_first",
    Some(TickEngine::Interpreter)
);
protocol_case!(aw_first_compiled, "aw_first", Some(TickEngine::Compiled));
protocol_case!(aw_first_rtl, "aw_first", None);
protocol_case!(
    w_first_interpreter,
    "w_first",
    Some(TickEngine::Interpreter)
);
protocol_case!(w_first_compiled, "w_first", Some(TickEngine::Compiled));
protocol_case!(w_first_rtl, "w_first", None);
protocol_case!(
    concurrent_interpreter,
    "concurrent",
    Some(TickEngine::Interpreter)
);
protocol_case!(
    concurrent_compiled,
    "concurrent",
    Some(TickEngine::Compiled)
);
protocol_case!(concurrent_rtl, "concurrent", None);

// Independent transaction-level oracle: queues represent accepted bus messages;
// byte storage defines architectural contents without copying the DUT's muxes.
#[derive(Default, Debug)]
struct Scoreboard {
    bytes: [u8; 16],
    aw: std::collections::VecDeque<u64>,
    w: std::collections::VecDeque<(u64, u64)>,
    b: std::collections::VecDeque<()>,
    r: std::collections::VecDeque<u64>,
    last_read: u64,
    accepted: [usize; 3],
    committed: usize,
    consumed: [usize; 2],
    cancelled: [usize; 4], // AW, W, B, R
    // AW-first, W-first, simultaneous AW/W, AR+write, write with stalled R.
    intersections: [usize; 5],
    verification_reads: usize,
}
impl Scoreboard {
    fn outputs(&self) -> Vec<(&'static str, u64)> {
        vec![
            (
                "s_axi_awready",
                (self.aw.is_empty() && self.b.is_empty()) as u64,
            ),
            (
                "s_axi_wready",
                (self.w.is_empty() && self.b.is_empty()) as u64,
            ),
            ("s_axi_bresp", 0),
            ("s_axi_bvalid", (!self.b.is_empty()) as u64),
            ("s_axi_arready", self.r.is_empty() as u64),
            ("s_axi_rdata", self.last_read),
            ("s_axi_rresp", 0),
            ("s_axi_rvalid", (!self.r.is_empty()) as u64),
        ]
    }
    fn read(&self, addr: u64) -> u64 {
        if addr <= 12 && addr % 4 == 0 {
            u32::from_le_bytes(
                self.bytes[addr as usize..addr as usize + 4]
                    .try_into()
                    .unwrap(),
            ) as u64
        } else {
            0
        }
    }
    fn push(&mut self, out: &mut Vec<Frame>, label: &'static str, inputs: &[(&'static str, u64)]) {
        let get = |name| {
            inputs
                .iter()
                .find(|(n, _)| *n == name)
                .map_or(0, |(_, v)| *v)
        };
        let pre = self.outputs();
        if get("rst") != 0 {
            self.cancelled[0] += self.aw.len();
            self.aw.clear();
            self.cancelled[1] += self.w.len();
            self.w.clear();
            self.cancelled[2] += self.b.len();
            self.b.clear();
            self.cancelled[3] += self.r.len();
            self.r.clear();
            self.bytes = [0; 16];
            self.last_read = 0;
        } else {
            let ready = |name| pre.iter().find(|(n, _)| *n == name).unwrap().1 != 0;
            let aw_accept = get("s_axi_awvalid") != 0 && ready("s_axi_awready");
            let w_accept = get("s_axi_wvalid") != 0 && ready("s_axi_wready");
            let ar_accept = get("s_axi_arvalid") != 0 && ready("s_axi_arready");
            let r_stalled = !self.r.is_empty() && get("s_axi_rready") == 0;
            if self.aw.is_empty() && self.w.is_empty() {
                if aw_accept && !w_accept {
                    self.intersections[0] += 1;
                }
                if w_accept && !aw_accept {
                    self.intersections[1] += 1;
                }
                if aw_accept && w_accept {
                    self.intersections[2] += 1;
                }
            }
            if get("s_axi_bready") != 0 && self.b.pop_front().is_some() {
                self.consumed[0] += 1;
            }
            if get("s_axi_rready") != 0 && self.r.pop_front().is_some() {
                self.consumed[1] += 1;
            }
            // Read messages snapshot the architectural bytes before write commits.
            if get("s_axi_arvalid") != 0 && ready("s_axi_arready") {
                self.last_read = self.read(get("s_axi_araddr"));
                self.r.push_back(self.last_read);
                self.accepted[2] += 1;
            }
            if get("s_axi_awvalid") != 0 && ready("s_axi_awready") {
                self.aw.push_back(get("s_axi_awaddr"));
                self.accepted[0] += 1;
            }
            if get("s_axi_wvalid") != 0 && ready("s_axi_wready") {
                self.w.push_back((get("s_axi_wdata"), get("s_axi_wstrb")));
                self.accepted[1] += 1;
            }
            if !self.aw.is_empty() && !self.w.is_empty() {
                self.intersections[3] += ar_accept as usize;
                self.intersections[4] += r_stalled as usize;
                assert!(self.b.is_empty(), "response overwrite in oracle");
                let addr = self.aw.pop_front().unwrap();
                let (data, strb) = self.w.pop_front().unwrap();
                if addr <= 12 && addr % 4 == 0 {
                    for (lane, byte) in (data as u32).to_le_bytes().iter().enumerate() {
                        if strb & (1 << lane) != 0 {
                            self.bytes[addr as usize + lane] = *byte;
                        }
                    }
                }
                self.b.push_back(());
                self.committed += 1;
            }
        }
        assert_eq!(
            self.accepted[0],
            self.committed + self.cancelled[0] + self.aw.len()
        );
        assert_eq!(
            self.accepted[1],
            self.committed + self.cancelled[1] + self.w.len()
        );
        assert_eq!(
            self.committed,
            self.consumed[0] + self.cancelled[2] + self.b.len()
        );
        assert_eq!(
            self.accepted[2],
            self.consumed[1] + self.cancelled[3] + self.r.len()
        );
        out.push(Frame {
            label,
            inputs: inputs.to_vec(),
            pre,
            post: self.outputs(),
        });
    }
}

fn matrix_frames() -> (Vec<Frame>, Scoreboard) {
    let mut q = Scoreboard::default();
    let mut f = Vec::new();
    q.push(&mut f, "reset", &[("rst", 1)]);
    for addr in [0, 4, 8, 12, 1, 3, 13, 16, 252, 255] {
        for strb in 0..16 {
            for gap in [0, 1, 7, 31] {
                for aw_first in [true, false] {
                    let aw = [("s_axi_awvalid", 1), ("s_axi_awaddr", addr)];
                    let w = [
                        ("s_axi_wvalid", 1),
                        ("s_axi_wdata", 0xa1b2_c3d4 ^ (strb * 0x0101_0101)),
                        ("s_axi_wstrb", strb),
                    ];
                    if gap == 0 {
                        let mut i = aw.to_vec();
                        i.extend(w);
                        i.extend([("s_axi_arvalid", 1), ("s_axi_araddr", addr)]);
                        q.push(&mut f, "same-edge write/read old value", &i);
                    } else {
                        q.push(&mut f, "first channel", if aw_first { &aw } else { &w });
                        for _ in 1..gap {
                            q.push(
                                &mut f,
                                "poison unaccepted pins",
                                &[
                                    ("s_axi_awaddr", 255),
                                    ("s_axi_wdata", 0xdeadbeef),
                                    ("s_axi_wstrb", 15),
                                ],
                            );
                        }
                        q.push(&mut f, "second channel", if aw_first { &w } else { &aw });
                    }
                    // Continuously attempt another transaction while both responses stall.
                    // Hold the master payload until its eventual acceptance or reset.
                    for _ in 0..3 {
                        q.push(
                            &mut f,
                            "blocked attempts",
                            &[
                                ("s_axi_awvalid", 1),
                                ("s_axi_awaddr", 252),
                                ("s_axi_wvalid", 1),
                                ("s_axi_wdata", 0xffffffff),
                                ("s_axi_wstrb", 15),
                                ("s_axi_arvalid", 1),
                                ("s_axi_araddr", addr),
                            ],
                        );
                    }
                    q.push(
                        &mut f,
                        "drain responses",
                        &[
                            ("s_axi_bready", 1),
                            ("s_axi_rready", 1),
                            ("s_axi_awvalid", 1),
                            ("s_axi_awaddr", 252),
                            ("s_axi_wvalid", 1),
                            ("s_axi_wdata", 0xffffffff),
                            ("s_axi_wstrb", 15),
                            ("s_axi_arvalid", 1),
                            ("s_axi_araddr", addr),
                        ],
                    );
                    q.push(
                        &mut f,
                        "accept held attempts",
                        &[
                            ("s_axi_awvalid", 1),
                            ("s_axi_awaddr", 252),
                            ("s_axi_wvalid", 1),
                            ("s_axi_wdata", 0xffffffff),
                            ("s_axi_wstrb", 15),
                            ("s_axi_arvalid", 1),
                            ("s_axi_araddr", addr),
                        ],
                    );
                    q.push(
                        &mut f,
                        "drain held attempts",
                        &[("s_axi_bready", 1), ("s_axi_rready", 1)],
                    );
                    q.push(
                        &mut f,
                        "read committed bank",
                        &[("s_axi_arvalid", 1), ("s_axi_araddr", addr)],
                    );
                    q.push(&mut f, "consume read", &[("s_axi_rready", 1)]);
                }
            }
        }
    }
    // Reset in every partially accepted / stalled state; then prove no ghost
    // response and a fresh complete write/read works without any old pairing.
    for phase in 0..5 {
        let mut i = vec![];
        if phase != 1 && phase != 3 {
            i.extend([("s_axi_awvalid", 1), ("s_axi_awaddr", 4)]);
        }
        if phase != 0 && phase != 3 {
            i.extend([
                ("s_axi_wvalid", 1),
                ("s_axi_wdata", 0x12345678),
                ("s_axi_wstrb", 15),
            ]);
        }
        if phase >= 3 {
            i.extend([("s_axi_arvalid", 1), ("s_axi_araddr", 4)]);
        }
        q.push(&mut f, "reset setup", &i);
        q.push(
            &mut f,
            "reset cancels pending",
            &[
                ("rst", 1),
                ("s_axi_awvalid", 1),
                ("s_axi_wvalid", 1),
                ("s_axi_arvalid", 1),
                ("s_axi_bready", 1),
                ("s_axi_rready", 1),
                ("s_axi_wdata", 0xdeadbeef),
                ("s_axi_wstrb", 15),
            ],
        );
        for _ in 0..3 {
            q.push(
                &mut f,
                "no ghost after reset",
                &[("s_axi_bready", 1), ("s_axi_rready", 1)],
            );
        }
        for addr in [0, 4, 8, 12] {
            q.push(
                &mut f,
                "reset cleared bank",
                &[("s_axi_arvalid", 1), ("s_axi_araddr", addr)],
            );
            q.push(&mut f, "consume reset read", &[("s_axi_rready", 1)]);
        }
        let fresh_aw = [("s_axi_awvalid", 1), ("s_axi_awaddr", 4)];
        let fresh_w = [
            ("s_axi_wvalid", 1),
            ("s_axi_wdata", 0x55aa55aa),
            ("s_axi_wstrb", 15),
        ];
        // Send the opposite half first: a stale pre-reset capture must not pair.
        q.push(
            &mut f,
            "fresh first half cannot pair with cancelled half",
            if phase == 0 { &fresh_w } else { &fresh_aw },
        );
        q.push(&mut f, "still no ghost pairing", &[]);
        q.push(
            &mut f,
            "fresh second half",
            if phase == 0 { &fresh_aw } else { &fresh_w },
        );
        q.push(
            &mut f,
            "fresh read during blocked B",
            &[("s_axi_arvalid", 1), ("s_axi_araddr", 4)],
        );
        q.push(
            &mut f,
            "consume fresh responses",
            &[("s_axi_bready", 1), ("s_axi_rready", 1)],
        );
    }
    for aw_first in [true, false] {
        let aw = [("s_axi_awvalid", 1), ("s_axi_awaddr", 12)];
        let w = [
            ("s_axi_wvalid", 1),
            ("s_axi_wdata", 0xdeadbeef),
            ("s_axi_wstrb", 15),
        ];
        q.push(
            &mut f,
            "half before held reset",
            if aw_first { &aw } else { &w },
        );
        let mut reset = vec![
            ("rst", 1),
            ("s_axi_awvalid", 1),
            ("s_axi_awaddr", 12),
            ("s_axi_wvalid", 1),
            ("s_axi_wdata", 0xdeadbeef),
            ("s_axi_wstrb", 15),
            ("s_axi_arvalid", 1),
            ("s_axi_araddr", 12),
            ("s_axi_bready", 1),
            ("s_axi_rready", 1),
        ];
        for _ in 0..3 {
            q.push(&mut f, "held reset beats half completion and ready", &reset);
        }
        reset[0].1 = 0;
        reset[2].1 = 0; // fresh write at address 0, not cancelled address 12
        reset[4].1 = 0xaabbccdd;
        reset[7].1 = 0;
        q.push(&mut f, "fresh transaction on first released edge", &reset);
        q.push(
            &mut f,
            "consume fresh release responses",
            &[("s_axi_bready", 1), ("s_axi_rready", 1)],
        );
        for addr in [0, 4, 8, 12] {
            q.push(
                &mut f,
                "verify reset precedence bank",
                &[("s_axi_arvalid", 1), ("s_axi_araddr", addr)],
            );
            q.push(
                &mut f,
                "consume reset precedence read",
                &[("s_axi_rready", 1)],
            );
        }
    }
    assert!(q.cancelled.iter().all(|n| *n > 0));
    (f, q)
}

fn random_frames(seed: u64) -> (Vec<Frame>, Scoreboard) {
    let mut state = seed;
    let mut rng = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let mut q = Scoreboard::default();
    let mut f = Vec::new();
    q.push(&mut f, "random reset", &[("rst", 1)]);
    let addresses = [0, 4, 8, 12, 1, 16, 255];
    let transactions: Vec<_> = (0..1000)
        .map(|_| {
            (
                addresses[rng() as usize % addresses.len()],
                rng() & 0xffffffff,
                rng() & 15,
                addresses[rng() as usize % addresses.len()],
            )
        })
        .collect();
    let mut delay = [0usize; 3];
    for cycle in 0..100_000 {
        let mut input = vec![
            ("s_axi_bready", (rng() % 4 == 0 || cycle % 17 == 0) as u64),
            ("s_axi_rready", (rng() % 4 == 0 || cycle % 19 == 0) as u64),
        ];
        for channel in 0..3 {
            if delay[channel] > 0 {
                delay[channel] -= 1;
            }
            let index = q.accepted[channel];
            if index < transactions.len() && delay[channel] == 0 {
                let (addr, data, strb, araddr) = transactions[index];
                match channel {
                    0 => input.extend([("s_axi_awvalid", 1), ("s_axi_awaddr", addr)]),
                    1 => input.extend([
                        ("s_axi_wvalid", 1),
                        ("s_axi_wdata", data),
                        ("s_axi_wstrb", strb),
                    ]),
                    _ => input.extend([("s_axi_arvalid", 1), ("s_axi_araddr", araddr)]),
                }
            }
        }
        let before = q.accepted;
        q.push(&mut f, "seeded independent channels", &input);
        for channel in 0..3 {
            if before[channel] != q.accepted[channel] {
                delay[channel] = (rng() % 32) as usize;
            }
        }
        if q.consumed == [1000, 1000] {
            for _ in 0..4 {
                q.push(
                    &mut f,
                    "no duplicate after drain",
                    &[("s_axi_bready", 1), ("s_axi_rready", 1)],
                );
            }
            assert_eq!(q.committed, 1000);
            assert_eq!(q.accepted, [1000; 3]);
            assert!(
                q.intersections.iter().all(|n| *n > 0),
                "seed {seed:#x} missed scheduling intersection: {:?}",
                q.intersections
            );
            for addr in [0, 4, 8, 12] {
                q.push(
                    &mut f,
                    "final bank audit after workload drained",
                    &[("s_axi_arvalid", 1), ("s_axi_araddr", addr)],
                );
                q.push(&mut f, "consume final audit read", &[("s_axi_rready", 1)]);
                q.verification_reads += 1;
            }
            assert_eq!(q.consumed, [1000, 1004]);
            return (f, q);
        }
    }
    panic!("seed {seed:#x}: fair-ready timeout; {q:?}");
}

fn verify_matrix(name: &str, frames: &[Frame], accounting: &Scoreboard) {
    use bitloom_sim::{AbstractionView, Axi4LiteSlaveFunctional};
    let dir = artifact_dir(name, "golden");
    fs::write(
        dir.join("accounting.txt"),
        format!("cycles={}\n{accounting:#?}\n", frames.len()),
    )
    .unwrap();
    let hir = Axi4LiteSlave::elaborate().unwrap();
    let mut vectors = String::new();
    for frame in frames {
        for (port, _) in INPUTS {
            let value = frame
                .inputs
                .iter()
                .find(|(n, _)| *n == port)
                .map_or(0, |(_, v)| *v);
            write!(vectors, "{value:x} ").unwrap();
        }
        for checks in [&frame.pre, &frame.post] {
            for (_, value) in checks {
                write!(vectors, "{value:x} ").unwrap();
            }
        }
        vectors.push('\n');
    }
    fs::write(dir.join("vectors.hex"), vectors).unwrap();
    fs::write(
        dir.join("design.v"),
        &bitloom_vlog::emit(&hir).files[0].contents,
    )
    .unwrap();
    // Constant-size RTL driver reads independent oracle vectors, never DUT output.
    let mut tb =
        String::from("module tb; reg clk=0; integer fd,n,cycle; reg [63:0] expected [0:15];\n");
    for (port, width) in INPUTS {
        writeln!(tb, "reg [{}:0] {port}=0;", width - 1).unwrap();
    }
    for (port, width) in OUTPUTS {
        writeln!(tb, "wire [{}:0] {port};", width - 1).unwrap();
    }
    tb.push_str(
        "Axi4LiteSlave dut(.*); initial begin fd=$fopen(\"vectors.hex\",\"r\"); if (!fd) $fatal;\n",
    );
    writeln!(
        tb,
        "for(cycle=0;cycle<{};cycle=cycle+1) begin",
        frames.len()
    )
    .unwrap();
    for (port, _) in INPUTS {
        writeln!(tb, "n=$fscanf(fd,\"%h\",{port}); if(n!=1) $fatal;").unwrap();
    }
    tb.push_str(
        "for(n=0;n<16;n=n+1) begin if($fscanf(fd,\"%h\",expected[n])!=1) $fatal; end #5;\n",
    );
    for phase in 0..2 {
        if phase == 1 {
            tb.push_str("clk=1; #5;\n");
        }
        for (j, (port, _)) in OUTPUTS.iter().enumerate() {
            writeln!(tb, "if((cycle != 0 || {phase} != 0) && {port} !== expected[{}]) $fatal(1,\"cycle %0d phase {phase} {port}=%h expected=%h\",cycle,{port},expected[{}]);", phase*8+j, phase*8+j).unwrap();
        }
    }
    tb.push_str("clk=0; #5; end $display(\"PROTOCOL PASS\"); $finish; end endmodule\n");
    fs::write(dir.join("tb.sv"), tb).unwrap();
    // Persist the complete independent reproducer before any DUT assertion.
    let mut sims = [
        Sim::with_engine(hir.clone(), TickEngine::Interpreter),
        Sim::with_engine(hir.clone(), TickEngine::Compiled),
    ];
    let mut fl = Axi4LiteSlaveFunctional::new();
    for (index, frame) in frames.iter().enumerate() {
        let mut inputs = PortValues::default();
        for (port, _) in INPUTS {
            inputs.set(port, 0);
        }
        for (port, value) in &frame.inputs {
            inputs.set(*port, *value);
        }
        for (engine, sim) in sims.iter_mut().enumerate() {
            sim.set_inputs(inputs.clone());
            sim.settle();
            for (phase, checks) in [("pre", &frame.pre), ("post", &frame.post)] {
                if phase == "post" {
                    sim.tick();
                }
                for (port, expected) in checks {
                    assert_eq!(
                        sim.ports().get(port),
                        Some(*expected),
                        "{name} frame {index} {} engine {engine} {phase} {port}",
                        frame.label
                    );
                }
            }
        }
        let actual = fl.cycle(&inputs);
        for (port, expected) in &frame.post {
            assert_eq!(
                actual.get(port),
                Some(*expected),
                "{name} frame {index} FL {port}"
            );
        }
    }
    for (tool, args) in [
        (
            "iverilog",
            vec![
                "-g2012",
                "-s",
                "tb",
                "-o",
                "simulation",
                "design.v",
                "tb.sv",
            ],
        ),
        ("vvp", vec!["simulation"]),
    ] {
        let output = Command::new(tool)
            .current_dir(&dir)
            .args(args)
            .output()
            .expect("required real RTL tool missing");
        let log = format!(
            "status={}\n{}{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        fs::write(dir.join(format!("{tool}.log")), &log).unwrap();
        assert!(output.status.success(), "{name} {tool}: {log}");
        if tool == "vvp" {
            assert!(log.contains("PROTOCOL PASS"));
        }
    }
    eprintln!(
        "{name}: {} cycles, {} writes, {} reads; interpreter/compiled/FL/direct RTL PASS",
        frames.len(),
        accounting.committed,
        accounting.accepted[2]
    );
}
#[test]
fn required_directed_protocol_matrix() {
    let (frames, accounting) = matrix_frames();
    verify_matrix("directed", &frames, &accounting);
}
#[test]
fn required_random_protocol_matrix_16_seeds_1000_transactions() {
    for index in 0..16u64 {
        let seed = 0x1253_1930_0000_0001 + index;
        let (frames, accounting) = random_frames(seed);
        verify_matrix(&format!("seed-{seed:016x}"), &frames, &accounting);
    }
}
