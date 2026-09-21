//! Story 128.3 P0 ATDD: independent per-source IRQ state and real RTL execution.
//! Missing Irq import is interface RED only; the oracle is not extracted from HIR.
use bitloom_prelude::{Elaboratable, FrozenHir, ip::Irq};
use std::{
    collections::BTreeMap,
    fmt::Write as _,
    fs,
    path::Path,
    process::{Command, Stdio},
};

#[derive(Clone, Copy, Debug, Default)]
struct Input {
    rst: bool,
    valid: bool,
    write: bool,
    addr: u16,
    data: u32,
    strb: u8,
    ready: bool,
    raw: u8,
}
#[derive(Clone, Copy, Debug)]
struct Response {
    data: u32,
    error: u32,
}
#[derive(Default)]
struct Oracle {
    latched: [bool; 5],
    enabled: [bool; 5],
    response: Option<Response>,
    accepted: usize,
    consumed: usize,
    cancelled: usize,
    hw: [usize; 5],
    test: [usize; 5],
    collisions: [usize; 5],
    strobes: [[usize; 16]; 3],
    raw_reads: [usize; 32],
}
type Values = BTreeMap<&'static str, u64>;
struct Frame {
    i: Input,
    before: Values,
    after: Values,
}
fn word(bits: &[bool; 5]) -> u32 {
    bits.iter()
        .enumerate()
        .map(|(n, b)| u32::from(*b) << n)
        .sum()
}
impl Oracle {
    fn commit(&self, i: Input) -> bool {
        !i.rst && self.response.is_none() && i.valid
    }
    fn outputs(&self, i: Input) -> Values {
        let mut v = Values::from([
            ("req_ready", u64::from(!i.rst && self.response.is_none())),
            ("rsp_valid", u64::from(self.response.is_some())),
            (
                "irq",
                u64::from((0..5).any(|n| self.latched[n] && self.enabled[n])),
            ),
        ]);
        if let Some(r) = self.response {
            v.insert("rdata", r.data.into());
            v.insert("error", r.error.into());
        }
        v
    }
    fn frame(&mut self, i: Input) -> Frame {
        assert!(i.raw < 32 && i.strb < 16);
        let before = self.outputs(i);
        if i.rst {
            self.cancelled += usize::from(self.response.is_some());
            self.latched.fill(false);
            self.enabled.fill(false);
            self.response = None;
        } else {
            let commit = self.commit(i);
            let write_allowed = commit && i.write && matches!(i.addr, 0 | 4 | 8);
            let selected = write_allowed && i.strb & 1 != 0;
            if commit {
                // Read the old abstract state before applying this edge's events.
                let read = match i.addr {
                    0 => Some(word(&self.latched)),
                    4 => Some(word(&self.enabled)),
                    12 => Some(i.raw.into()),
                    _ => None,
                };
                let legal = if i.write {
                    matches!(i.addr, 0 | 4 | 8)
                } else {
                    read.is_some()
                };
                self.response = Some(Response {
                    data: if !i.write && legal { read.unwrap() } else { 0 },
                    error: if legal { 0 } else { 2 },
                });
                self.accepted += 1;
                if write_allowed {
                    self.strobes[(i.addr / 4) as usize][i.strb as usize] += 1;
                }
                if !i.write && i.addr == 12 {
                    self.raw_reads[i.raw as usize] += 1;
                }
            } else if self.response.is_some() && i.ready {
                self.response = None;
                self.consumed += 1;
            }
            // Independent event-list oracle: clear one source, then deliver events.
            // There is no DUT bit-vector next-state expression or descriptor dependency.
            for source in 0..5 {
                let software_bit = i.data & (1 << source) != 0;
                let hardware = i.raw & (1 << source) != 0;
                if selected && i.addr == 4 {
                    self.enabled[source] = software_bit;
                }
                if selected && i.addr == 0 && software_bit {
                    self.latched[source] = false;
                    if hardware {
                        self.collisions[source] += 1;
                    }
                }
                if selected && i.addr == 8 && software_bit {
                    self.latched[source] = true;
                    self.test[source] += 1;
                }
                if hardware {
                    self.latched[source] = true;
                    self.hw[source] += 1;
                }
            }
        }
        assert_eq!(
            self.accepted,
            self.consumed + self.cancelled + usize::from(self.response.is_some())
        );
        Frame {
            i,
            before,
            after: self.outputs(i),
        }
    }
}
#[derive(Default)]
struct Trace {
    m: Oracle,
    frames: Vec<Frame>,
    tags: BTreeMap<&'static str, usize>,
}
impl Trace {
    fn step(&mut self, i: Input) {
        self.frames.push(self.m.frame(i));
    }
    fn mark(&mut self, tag: &'static str) {
        *self.tags.entry(tag).or_default() += 1;
    }
    fn reset(&mut self) {
        self.step(Input {
            rst: true,
            ..Input::default()
        });
    }
    fn request(&mut self, i: Input, stall: usize) {
        assert!(self.m.response.is_none());
        self.step(Input { valid: true, ..i });
        for _ in 0..stall {
            self.step(Input::default());
        }
        self.step(Input {
            ready: true,
            ..Input::default()
        });
    }
    fn write(&mut self, addr: u16, data: u32, strb: u8) {
        self.request(
            Input {
                write: true,
                addr,
                data,
                strb,
                ..Input::default()
            },
            0,
        );
    }
    fn read(&mut self, addr: u16) {
        self.request(
            Input {
                addr,
                ..Input::default()
            },
            0,
        );
    }
    fn inspect(&mut self) {
        for addr in [0, 4, 12] {
            self.read(addr);
        }
    }
}
fn random(seed: &mut u64) -> u32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed as u32
}
fn trace(seed0: u64) -> Trace {
    let mut t = Trace::default();
    t.reset();
    t.inspect();
    // Exhaust every hardware event word against every enable word. Pending is read
    // back, so an implementation that masks at event capture is observable.
    for raw in 0..32 {
        for mask in 0..32 {
            t.reset();
            t.write(4, mask, 1);
            t.step(Input {
                raw,
                ..Input::default()
            });
            assert_eq!(word(&t.m.latched), u32::from(raw));
            t.read(0);
            t.write(4, 0, 1);
            t.read(0);
            t.write(4, 31, 1);
            t.request(
                Input {
                    addr: 12,
                    raw,
                    ..Input::default()
                },
                3,
            );
            t.inspect();
            t.mark("all_source_mask_pairs");
        }
    }
    // All byte strobes, all writable registers, zero / valid / reserved bits.
    for addr in [0, 4, 8] {
        for strb in 0..16 {
            for data in [0, 31, 0xffff_ffe0, 0xffff_ffff, 0x805a_0015] {
                // TEST must start with unset bits, otherwise a lost or wrongly
                // selected injection is hidden by sticky all-ones PENDING.
                let initial_states: &[u32] = if addr == 8 { &[0, 0b01010] } else { &[31] };
                for &initial in initial_states {
                    t.reset();
                    t.write(4, 31, 1);
                    // Seed PENDING through hardware, independent of TEST.
                    t.step(Input {
                        raw: initial as u8,
                        ..Input::default()
                    });
                    t.write(addr, data, strb);
                    if addr == 8 {
                        let expected = if strb & 1 != 0 {
                            initial | (data & 31)
                        } else {
                            initial
                        };
                        assert_eq!(word(&t.m.latched), expected);
                        t.mark(if initial == 0 {
                            "test_wstrb_empty"
                        } else {
                            "test_wstrb_mixed"
                        });
                    }
                    t.inspect();
                    t.mark("all_wstrb_register_patterns");
                }
            }
        }
    }
    for source in 0..5 {
        let bit = 1u8 << source;
        t.reset();
        t.step(Input {
            raw: bit,
            ..Input::default()
        });
        t.read(0);
        t.write(4, bit.into(), 1);
        t.mark("disabled_capture_then_enable");
        // Repetition is sticky, not a counter: one clear removes all past pulses.
        for _ in 0..4 {
            t.step(Input {
                raw: bit,
                ..Input::default()
            });
        }
        t.write(0, bit.into(), 1);
        t.inspect();
        t.mark("repeat_not_counted");
        // Consecutive high cycles still supply events, including the clear edge.
        t.step(Input {
            raw: bit,
            ..Input::default()
        });
        t.request(
            Input {
                write: true,
                addr: 0,
                data: bit.into(),
                strb: 1,
                raw: bit,
                ..Input::default()
            },
            2,
        );
        t.read(0);
        t.mark("same_source_clear_collision");
        t.request(
            Input {
                write: true,
                addr: 0,
                data: bit.into(),
                strb: 1,
                raw: 1 << ((source + 1) % 5),
                ..Input::default()
            },
            0,
        );
        t.read(0);
        t.mark("other_source_clear_collision");
        t.write(0, 31, 1);
        t.request(
            Input {
                write: true,
                addr: 8,
                data: bit.into(),
                strb: 1,
                raw: bit,
                ..Input::default()
            },
            2,
        );
        t.inspect();
        t.mark("same_source_test_hardware");
        t.write(0, 31, 1);
        t.request(
            Input {
                write: true,
                addr: 8,
                data: bit.into(),
                strb: 1,
                raw: 1 << ((source + 1) % 5),
                ..Input::default()
            },
            2,
        );
        t.inspect();
        t.mark("other_source_test_hardware");
        // Request edge captures OLD pending, but its hardware event is visible later.
        t.write(0, 31, 1);
        t.request(
            Input {
                addr: 0,
                raw: bit,
                ..Input::default()
            },
            2,
        );
        t.read(0);
        t.mark("pending_old_snapshot");
        t.write(0, 31, 1);
        t.request(
            Input {
                addr: 12,
                raw: bit,
                ..Input::default()
            },
            5,
        );
        t.inspect();
        t.mark("raw_snapshot_stall");
        t.write(0, 31, 1);
        t.write(8, bit.into(), 1);
        t.read(12);
        t.read(0);
        t.mark("test_excluded_from_raw");
    }
    // Idle TEST candidates must never become events.
    t.reset();
    t.write(4, 31, 1);
    for strb in 0..16 {
        t.step(Input {
            addr: 8,
            write: true,
            data: u32::MAX,
            strb,
            ..Input::default()
        });
        t.inspect();
        t.mark("idle_test_candidate");
    }
    // Queue a held TEST behind a read response. It cannot commit on the consume
    // edge, and cannot turn into events until the following available cycle.
    t.reset();
    t.write(4, 31, 1);
    t.step(Input {
        valid: true,
        addr: 0,
        ..Input::default()
    });
    let held = Input {
        valid: true,
        write: true,
        addr: 8,
        data: 1,
        strb: 1,
        ..Input::default()
    };
    for _ in 0..5 {
        t.step(held);
    }
    t.step(Input {
        ready: true,
        ..held
    });
    t.step(held);
    // Keep its committed request payload visible while stalling the response.
    for _ in 0..5 {
        t.step(Input {
            addr: 8,
            write: true,
            data: 1,
            strb: 1,
            ..Input::default()
        });
    }
    t.step(Input {
        ready: true,
        ..Input::default()
    });
    t.write(0, 31, 1);
    t.inspect();
    t.mark("held_test_and_consume_no_refill");
    // Changes in RAW during a stalled response must neither alter the snapshot
    // nor prevent hardware event capture.
    t.reset();
    t.write(4, 31, 1);
    t.step(Input {
        valid: true,
        addr: 12,
        raw: 1,
        ..Input::default()
    });
    for source in 1..5 {
        t.step(Input {
            raw: 1 << source,
            ..Input::default()
        });
    }
    t.step(Input {
        ready: true,
        ..Input::default()
    });
    t.inspect();
    t.mark("backpressure_accepts_hardware");
    // Permission wins even with WSTRB zero; addresses are never truncated.
    for addr in [
        0, 4, 8, 12, 1, 2, 3, 5, 9, 13, 16, 0x300, 0x304, 0x8000, 0xfffc, 0xffff,
    ] {
        for write in [false, true] {
            for strb in [0, 1, 15] {
                t.reset();
                t.write(4, 31, 1);
                t.request(
                    Input {
                        addr,
                        write,
                        data: u32::MAX,
                        strb,
                        raw: 16,
                        ..Input::default()
                    },
                    2,
                );
                t.inspect();
                t.mark("permissions_holes_aliases_with_hardware");
            }
        }
    }
    // Cancel both read and write responses, with valid TEST/clear and events at
    // reset. No stale response or reset-edge event may escape afterwards.
    for addr in [0, 8, 12] {
        for write in [false, true] {
            t.reset();
            t.write(4, 31, 1);
            t.write(8, 31, 1);
            t.step(Input {
                valid: true,
                addr,
                write,
                data: 31,
                strb: 1,
                ..Input::default()
            });
            t.step(Input {
                raw: 31,
                ..Input::default()
            });
            t.step(Input {
                rst: true,
                valid: true,
                addr: 8,
                write: true,
                data: 31,
                strb: 1,
                raw: 31,
                ..Input::default()
            });
            t.inspect();
            t.step(Input {
                raw: 1,
                ..Input::default()
            });
            t.inspect();
            t.mark("reset_cancels_stalled_response");
        }
    }
    for addr in [0, 8] {
        t.reset();
        t.write(4, 31, 1);
        t.write(8, 31, 1);
        t.step(Input {
            rst: true,
            valid: true,
            write: true,
            addr,
            data: 31,
            strb: 1,
            raw: 31,
            ..Input::default()
        });
        t.inspect();
        t.mark("reset_beats_available_commit");
    }
    let mut seed = seed0;
    let mut held: Option<Input> = None;
    for n in 0..3200 {
        if held.is_none() && random(&mut seed) & 3 != 0 {
            let r = random(&mut seed);
            held = Some(Input {
                valid: true,
                write: r & 1 != 0,
                addr: [0, 4, 8, 12, 1, 16, 0x300, 0x8000][((r >> 1) % 8) as usize],
                data: random(&mut seed),
                strb: ((r >> 8) & 15) as u8,
                ..Input::default()
            });
        }
        let i = Input {
            raw: (random(&mut seed) & 31) as u8,
            ready: random(&mut seed) & 3 != 0,
            rst: n == 733 || n == 1733 || n == 2733,
            ..held.unwrap_or_default()
        };
        let accepted = t.m.commit(i);
        t.step(i);
        if accepted || i.rst {
            held = None;
        }
    }
    while held.is_some() || t.m.response.is_some() {
        let i = Input {
            ready: true,
            ..held.unwrap_or_default()
        };
        let accepted = t.m.commit(i);
        t.step(i);
        if accepted {
            held = None;
        }
    }
    t.inspect();
    for pair in t.frames.windows(2) {
        let a = &pair[0];
        let b = &pair[1];
        if a.i.valid && a.before["req_ready"] == 0 && !a.i.rst && !b.i.rst {
            assert!(b.i.valid, "producer dropped held request");
            assert_eq!(
                (a.i.write, a.i.addr, a.i.data, a.i.strb),
                (b.i.write, b.i.addr, b.i.data, b.i.strb)
            );
        }
    }
    assert_eq!(t.tags["all_source_mask_pairs"], 1024);
    assert_eq!(t.tags["all_wstrb_register_patterns"], 320);
    assert_eq!(t.tags["test_wstrb_empty"], 80);
    assert_eq!(t.tags["test_wstrb_mixed"], 80);
    for tag in [
        "disabled_capture_then_enable",
        "repeat_not_counted",
        "same_source_clear_collision",
        "other_source_clear_collision",
        "same_source_test_hardware",
        "other_source_test_hardware",
        "pending_old_snapshot",
        "raw_snapshot_stall",
        "test_excluded_from_raw",
    ] {
        assert_eq!(t.tags[tag], 5, "{tag}");
    }
    assert_eq!(t.tags["idle_test_candidate"], 16);
    assert_eq!(t.tags["held_test_and_consume_no_refill"], 1);
    assert_eq!(t.tags["backpressure_accepts_hardware"], 1);
    assert_eq!(t.tags["permissions_holes_aliases_with_hardware"], 96);
    assert_eq!(t.tags["reset_cancels_stalled_response"], 6);
    assert_eq!(t.tags["reset_beats_available_commit"], 2);
    assert!(t.m.strobes.iter().flatten().all(|n| *n > 0));
    assert!(
        t.m.hw
            .iter()
            .chain(&t.m.test)
            .chain(&t.m.collisions)
            .all(|n| *n > 0)
    );
    assert!(t.m.raw_reads.iter().all(|n| *n > 0));
    assert!(t.m.cancelled >= 6);
    assert_eq!(t.m.accepted, t.m.consumed + t.m.cancelled);
    t
}
fn check(tb: &mut String, v: &Values, cycle: usize, phase: &str) {
    for (name, value) in v {
        writeln!(tb,"if ({name} !== 64'h{value:016x}) $fatal(1,\"cycle={cycle} {phase} {name} expected={value:x} got=%h\",{name});").unwrap();
    }
}
fn testbench(hir: &FrozenHir, frames: &[Frame]) -> String {
    let top = &hir.circuit().name;
    let mut tb = String::from(
        "module tb;reg clk=0,rst=0,req_valid=0,write=0,rsp_ready=0;reg [15:0] addr=0;reg [31:0] wdata=0;reg [3:0] wstrb=0;reg [4:0] raw_events=0;wire req_ready,rsp_valid,irq;wire [31:0] rdata;wire [1:0] error;\n",
    );
    writeln!(
        tb,
        "{top} dut(.*);initial begin $dumpfile(\"trace.vcd\");$dumpvars(0,tb);"
    )
    .unwrap();
    for (n, f) in frames.iter().enumerate() {
        let i = f.i;
        writeln!(tb,"rst={};req_valid={};write={};addr=16'h{:04x};wdata=32'h{:08x};wstrb={};rsp_ready={};raw_events={};#1;",i.rst as u8,i.valid as u8,i.write as u8,i.addr,i.data,i.strb,i.ready as u8,i.raw).unwrap();
        if n != 0 {
            check(&mut tb, &f.before, n, "before");
        }
        tb.push_str("clk=1;#1;\n");
        check(&mut tb, &f.after, n, "after");
        tb.push_str("clk=0;#1;\n");
    }
    tb.push_str("$display(\"FR197 IRQ PASS\");$finish;end endmodule\n");
    tb
}
fn run(dir: &Path, tool: &str, args: &[&str], log_name: &str) {
    let log = fs::File::create(dir.join(log_name)).unwrap();
    let start_utc_unix_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let status = Command::new("timeout")
        .args(["--kill-after=5s", "60s", tool])
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .expect("GNU timeout and RTL tools required");
    fs::write(dir.join(format!("{log_name}.json")), serde_json::json!({"tool":tool,"args":args,"exit_code":status.code(),"start_utc_unix_ms":start_utc_unix_ms,"end_utc_unix_ms":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()}).to_string()).unwrap();
    let output = fs::read_to_string(dir.join(log_name)).unwrap();
    assert!(
        status.success(),
        "{tool} {args:?} failed ({status}) artifacts={}\n{output}",
        dir.display()
    );
}
#[test]
fn p0_irq_direct_rtl_independent_five_source_directed_and_seeded_oracle() {
    let hir = Irq::elaborate().unwrap();
    let design = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    for seed in [0x1283_1970_5511, 0xdead_beef_1283, 0x7359_2401_ffff] {
        let t = trace(seed);
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/fr197-irq")
            .join(format!("direct-{seed:x}-{}-{unique}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("design.v"), &design).unwrap();
        fs::write(dir.join("tb.sv"), testbench(&hir, &t.frames)).unwrap();
        let accounting = format!(
            "seed={seed:x} frames={} accepted={} consumed={} cancelled={} hardware={:?} test={:?} collisions={:?} strobes={:?} raw_reads={:?} directed={:?}\n",
            t.frames.len(),
            t.m.accepted,
            t.m.consumed,
            t.m.cancelled,
            t.m.hw,
            t.m.test,
            t.m.collisions,
            t.m.strobes,
            t.m.raw_reads,
            t.tags
        );
        fs::write(dir.join("accounting.log"), &accounting).unwrap();
        fs::write(dir.join("commands.log"),"timeout --kill-after=5s 60s iverilog -V\ntimeout --kill-after=5s 60s vvp -V\ntimeout --kill-after=5s 60s iverilog -g2012 -s tb -o simulation design.v tb.sv\ntimeout --kill-after=5s 60s vvp simulation\n").unwrap();
        run(&dir, "iverilog", &["-V"], "iverilog-version.log");
        run(&dir, "vvp", &["-V"], "vvp-version.log");
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
            "compile.log",
        );
        run(&dir, "vvp", &["simulation"], "run.log");
        assert!(
            fs::read_to_string(dir.join("run.log"))
                .unwrap()
                .contains("FR197 IRQ PASS")
        );
        println!("{accounting}artifacts={}", dir.display());
    }
}

fn backend_run(dir: &Path, tool: &str, args: &[&str], label: &str) -> String {
    let log = fs::File::create(dir.join(format!("{label}.log"))).unwrap();
    let start_utc_unix_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let status = Command::new("timeout")
        .args(["--kill-after=5s", "60s", tool])
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .expect("required real RTL tool (missing tool is not a pass)");
    fs::write(
        dir.join(format!("{label}.json")),
        serde_json::json!({"tool":tool,"args":args,"exit_code":status.code(), "start_utc_unix_ms":start_utc_unix_ms, "end_utc_unix_ms":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()}).to_string(),
    )
    .unwrap();
    let output = fs::read_to_string(dir.join(format!("{label}.log"))).unwrap();
    assert!(
        status.success(),
        "{tool} {status}: {output}; {}",
        dir.display()
    );
    output
}

fn vector_backends(hir: &FrozenHir, top_name: &str, tb: &str, dir: &Path) {
    let tools = std::env::var("RHDL_FIRTOOL_PATH").expect("pinned firtool directory");
    let firtool = format!("{tools}/firtool");
    assert!(
        backend_run(dir, &firtool, &["--version"], "firtool-version").contains("firtool-1.159.0")
    );
    fs::write(
        dir.join("design.fir"),
        &bitloom_firrtl::emit(hir).files[0].contents,
    )
    .unwrap();
    backend_run(
        dir,
        &firtool,
        &[
            "design.fir",
            "--verilog",
            "--disable-all-randomization",
            "--lowering-options=disallowLocalVariables",
            "-o",
            "firrtl.v",
        ],
        "firrtl-lower",
    );
    backend_run(
        dir,
        "iverilog",
        &[
            "-g2012",
            "-s",
            "tb",
            "-o",
            "firrtl-sim",
            "firrtl.v",
            "tb.sv",
        ],
        "firrtl-compile",
    );
    assert!(backend_run(dir, "vvp", &["firrtl-sim"], "firrtl-simulate").contains("FR197 IRQ PASS"));
    fs::rename(dir.join("trace.vcd"), dir.join("firrtl.vcd")).unwrap();
    fs::create_dir_all(dir.join("src/main/scala")).unwrap();
    fs::create_dir_all(dir.join("project")).unwrap();
    fs::write(
        dir.join("src/main/scala/Design.scala"),
        &bitloom_firrtl::emit_chisel(hir).unwrap().files[0].contents,
    )
    .unwrap();
    fs::write(dir.join("src/main/scala/Main.scala"), format!("object PairMain extends App {{ circt.stage.ChiselStage.emitSystemVerilogFile(new {top_name}, args=Array(\"--target-dir\",\"chisel\"), firtoolOpts=Array(\"--disable-all-randomization\",\"--lowering-options=disallowLocalVariables\")) }}\n")).unwrap();
    fs::write(dir.join("build.sbt"), "scalaVersion := \"2.13.18\"\nlibraryDependencies += \"org.chipsalliance\" %% \"chisel\" % \"7.15.0\"\naddCompilerPlugin(\"org.chipsalliance\" % \"chisel-plugin\" % \"7.15.0\" cross CrossVersion.full)\n").unwrap();
    fs::write(
        dir.join("project/build.properties"),
        "sbt.version=1.10.11\n",
    )
    .unwrap();
    let log = fs::File::create(dir.join("sbt.log")).unwrap();
    let start_utc_unix_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let status = Command::new("timeout")
        .args([
            "--kill-after=5s",
            "240s",
            "sbt",
            "-batch",
            "runMain PairMain",
        ])
        .env("CHISEL_FIRTOOL_PATH", &tools)
        .current_dir(dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .unwrap();
    fs::write(dir.join("sbt.exit"), format!("{status}\n")).unwrap();
    fs::write(dir.join("sbt.json"),serde_json::json!({"command":["timeout","--kill-after=5s","240s","sbt","-batch","runMain PairMain"],"CHISEL_FIRTOOL_PATH":tools,"exit_code":status.code(),"start_utc_unix_ms":start_utc_unix_ms,"end_utc_unix_ms":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()}).to_string()).unwrap();
    assert!(status.success(), "JVM pair failed: {}", dir.display());
    let top = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == top_name)
        .unwrap();
    let connections = top
        .ports
        .iter()
        .map(|p| {
            let name = match p.name.as_str() {
                "clk" => "clock".into(),
                "rst" => "reset".into(),
                n => format!("io_{n}"),
            };
            format!(".{name}({})", p.name)
        })
        .collect::<Vec<_>>()
        .join(", ");
    let chisel = dir.join("chisel");
    fs::write(
        chisel.join("tb.sv"),
        tb.replace("dut(.*)", &format!("dut({connections})")),
    )
    .unwrap();
    backend_run(
        &chisel,
        "iverilog",
        &[
            "-g2012",
            "-I",
            ".",
            "-s",
            "tb",
            "-o",
            "simulation",
            "-f",
            "filelist.f",
            "tb.sv",
        ],
        "compile",
    );
    assert!(backend_run(&chisel, "vvp", &["simulation"], "simulate").contains("FR197 IRQ PASS"));
    println!(
        "IRQ vectors FIRRTL and JVM actual RTL PASS {}",
        dir.display()
    );
}

#[test]
#[ignore = "dedicated pinned firtool and JVM full independent vector gate"]
fn p1_irq_firrtl_chisel_same_independent_vectors() {
    let hir = Irq::elaborate().unwrap();
    // Concatenating all three complete seed suites preserves every directed
    // case and accounting check while compiling each backend only once.
    let mut frames = vec![];
    let mut accounting = String::new();
    for seed in [0x1283_1970_5511, 0xdead_beef_1283, 0x7359_2401_ffff] {
        let t = trace(seed);
        writeln!(
            accounting,
            "seed={seed:x} frames={} accepted={} consumed={} cancelled={} directed={:?}",
            t.frames.len(),
            t.m.accepted,
            t.m.consumed,
            t.m.cancelled,
            t.tags
        )
        .unwrap();
        let mut suite = t.frames;
        // At a suite boundary only the reset post-edge state is comparable:
        // the DUT still holds the previous suite state before this reset.
        suite[0].before.clear();
        frames.extend(suite);
    }
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-irq")
        .join(format!("backends-{}-{stamp}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let tb = testbench(&hir, &frames);
    fs::write(dir.join("tb.sv"), &tb).unwrap();
    fs::write(dir.join("accounting.log"), &accounting).unwrap();
    vector_backends(&hir, "Irq", &tb, &dir);
    println!("{accounting}");
}

/// [P0] A passing DUT and two compiling DUT faults exercise this scoreboard itself.
#[test]
fn p0_irq_direct_scoreboard_rejects_source_mapping_and_test_byte_faults() {
    let hir = Irq::elaborate().unwrap();
    let source = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let mut t = Trace::default();
    // Each hardware source starts empty and disabled. Readback, rather than
    // irq's reduction, distinguishes a GPIO event from a Timer event.
    for source in 0..5 {
        t.reset();
        t.step(Input {
            raw: 1 << source,
            ..Input::default()
        });
        t.read(0);
    }
    let first_test_read = t.frames.len() + 3;
    // Empty pending makes ignored-byte injection observable. The last two
    // strobes also require real injection, preventing an inert control DUT.
    for source in 0..5 {
        for strb in [0, 2, 4, 8, 1, 15] {
            t.reset();
            t.write(8, 1 << source, strb);
            t.read(0);
        }
    }
    assert_eq!(t.frames.len(), 170);
    assert_eq!(t.m.accepted, t.m.consumed);
    assert_eq!(t.frames[18].after["rdata"], 16);
    assert_eq!(t.frames[first_test_read].after["rdata"], 0);
    let tb = testbench(&hir, &t.frames);
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-irq")
        .join(format!("scoreboard-faults-{}-{unique}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("original.v"), &source).unwrap();
    fs::write(dir.join("tb.sv"), &tb).unwrap();
    let mut vectors = String::new();
    for (cycle, frame) in t.frames.iter().enumerate() {
        writeln!(
            vectors,
            "cycle={cycle} input={:?} before={:?} after={:?}",
            frame.i, frame.before, frame.after
        )
        .unwrap();
    }
    fs::write(dir.join("vectors.log"), vectors).unwrap();
    run(&dir, "iverilog", &["-V"], "iverilog-version.log");
    run(&dir, "vvp", &["-V"], "vvp-version.log");
    let mutations = [
        (
            "gpio-aliases-timer",
            "  assign raw_value = {zero27, raw_events};",
            "  assign raw_value = {zero27, 1'b0, raw_events[3:1], (raw_events[0] | raw_events[4])};",
            "cycle=18 after rdata expected=10 got=00000001".to_owned(),
        ),
        (
            "test-bypasses-byte-selection",
            "  assign test_bits = (test_write_commit ? test_candidate : zero32);",
            "  assign test_bits = ((req_valid && req_ready && write && addr == 16'h0008) ? (wdata & 32'h0000001f) : zero32);",
            format!("cycle={first_test_read} after rdata expected=0 got=00000001"),
        ),
    ];
    let mut cases = vec![("control", source.clone(), None)];
    for (name, old, new, expected) in mutations {
        assert_eq!(
            source.matches(old).count(),
            1,
            "unique mutation site {name}"
        );
        let mutant = source.replacen(old, new, 1);
        assert_ne!(source, mutant);
        fs::write(
            dir.join(format!("{name}-mutation.json")),
            serde_json::json!({"name":name,"old":old,"new":new,"site_count":1,"expected_fatal":expected,"expected_vvp_exit":1}).to_string(),
        ).unwrap();
        cases.push((name, mutant, Some(expected)));
    }
    for (name, rtl, expected) in cases {
        let case = dir.join(name);
        fs::create_dir(&case).unwrap();
        fs::write(case.join("design.v"), rtl).unwrap();
        fs::write(case.join("tb.sv"), &tb).unwrap();
        assert_eq!(
            fs::read(case.join("tb.sv")).unwrap(),
            fs::read(dir.join("tb.sv")).unwrap()
        );
        fs::write(case.join("commands.log"), "timeout --kill-after=5s 60s iverilog -g2012 -s tb -o simulation design.v tb.sv\ntimeout --kill-after=5s 60s vvp simulation\n").unwrap();
        // A compile error never reaches the negative-control check.
        run(
            &case,
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
            "compile.log",
        );
        if let Some(expected) = expected {
            let log = fs::File::create(case.join("run.log")).unwrap();
            let start = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let status = Command::new("timeout")
                .args(["--kill-after=5s", "60s", "vvp", "simulation"])
                .current_dir(&case)
                .stdout(Stdio::from(log.try_clone().unwrap()))
                .stderr(Stdio::from(log))
                .status()
                .expect("GNU timeout and vvp required");
            fs::write(case.join("run.log.json"), serde_json::json!({
                "command":["timeout","--kill-after=5s","60s","vvp","simulation"],
                "exit_code":status.code(),"start_utc_unix_ms":start,
                "end_utc_unix_ms":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
                "expected_fatal":expected
            }).to_string()).unwrap();
            let output = fs::read_to_string(case.join("run.log")).unwrap();
            assert_eq!(
                status.code(),
                Some(1),
                "expected vvp assertion, not timeout/signal/tool failure: {}\n{output}",
                case.display()
            );
            let failures: Vec<_> = output
                .lines()
                .filter(|line| line.starts_with("FATAL: tb.sv:"))
                .collect();
            assert_eq!(failures.len(), 1, "expected one scoreboard fatal: {output}");
            assert!(
                failures[0].ends_with(&expected),
                "wrong failure, expected {expected}: {output}"
            );
            assert!(
                !output.contains("FR197 IRQ PASS"),
                "fault escaped scoreboard"
            );
        } else {
            run(&case, "vvp", &["simulation"], "run.log");
            assert!(
                fs::read_to_string(case.join("run.log"))
                    .unwrap()
                    .contains("FR197 IRQ PASS")
            );
        }
        let vcd = fs::read_to_string(case.join("trace.vcd")).unwrap();
        assert!(
            vcd.contains("$enddefinitions") && vcd.contains("#0"),
            "missing waveform: {}",
            case.display()
        );
    }
    println!(
        "IRQ direct scoreboard original PASS; source and TEST-byte faults detected at prescribed readback assertions: {}",
        dir.display()
    );
}
