//! Story 128.4 GPIO32 independent cycle oracle; no DUT-derived golden.
use bitloom_prelude::{Elaboratable, FrozenHir, ip::GpioCsr};
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
    pad: u32,
}
#[derive(Clone, Copy, Debug)]
struct Response {
    data: u32,
    error: u32,
}
#[derive(Default)]
struct Oracle {
    dir: u32,
    out: u32,
    event: u32,
    s1: u32,
    s2: u32,
    history: u32,
    response: Option<Response>,
    accepted: usize,
    consumed: usize,
    cancelled: usize,
    strobes: [[usize; 16]; 5],
    rises: [usize; 32],
    collisions: [usize; 32],
}
type Values = BTreeMap<&'static str, u64>;
struct Frame {
    i: Input,
    before: Values,
    after: Values,
}
impl Oracle {
    fn commit(&self, i: Input) -> bool {
        !i.rst && i.valid && self.response.is_none()
    }
    fn rising(&self) -> u32 {
        self.s2 & !self.history & !self.dir
    }
    fn outputs(&self, i: Input) -> Values {
        let mut v = Values::from([
            ("req_ready", u64::from(!i.rst && self.response.is_none())),
            ("rsp_valid", u64::from(self.response.is_some())),
            ("pad_out", u64::from(self.out & self.dir)),
            ("pad_oe", u64::from(self.dir)),
            ("raw_event", u64::from(!i.rst && self.rising() != 0)),
        ]);
        if let Some(r) = self.response {
            v.insert("rdata", r.data.into());
            v.insert("error", r.error.into());
        }
        v
    }
    fn frame(&mut self, i: Input) -> Frame {
        assert!(i.strb < 16);
        let before = self.outputs(i);
        if i.rst {
            self.cancelled += usize::from(self.response.is_some());
            self.dir = 0;
            self.out = 0;
            self.event = 0;
            self.s1 = 0;
            self.s2 = 0;
            self.history = 0;
            self.response = None;
        } else {
            let rise = self.rising();
            let commit = self.commit(i);
            let writable = matches!(i.addr, 0 | 4 | 12 | 16 | 20);
            let selected = commit && i.write && writable;
            let old_s1 = self.s1;
            let old_s2 = self.s2;
            if commit {
                let read = match i.addr {
                    0 => Some(self.dir),
                    4 => Some(self.out),
                    8 => Some(self.s2),
                    20 => Some(self.event),
                    _ => None,
                };
                let legal = if i.write { writable } else { read.is_some() };
                self.response = Some(Response {
                    data: if !i.write && legal { read.unwrap() } else { 0 },
                    error: if legal { 0 } else { 2 },
                });
                self.accepted += 1;
                if selected {
                    let index = match i.addr {
                        0 => 0,
                        4 => 1,
                        12 => 2,
                        16 => 3,
                        20 => 4,
                        _ => unreachable!(),
                    };
                    self.strobes[index][i.strb as usize] += 1;
                }
            } else if self.response.is_some() && i.ready {
                self.response = None;
                self.consumed += 1;
            }
            // Per-pin operations, not a descriptor or emitted expression as a reference.
            for bit in 0..32 {
                let mask = 1u32 << bit;
                let data = i.data & mask != 0;
                let byte = i.strb & (1 << (bit / 8)) != 0;
                if selected && byte {
                    match i.addr {
                        0 => {
                            if data {
                                self.dir |= mask
                            } else {
                                self.dir &= !mask
                            }
                        }
                        4 => {
                            if data {
                                self.out |= mask
                            } else {
                                self.out &= !mask
                            }
                        }
                        12 => {
                            if data {
                                self.out |= mask
                            }
                        }
                        16 => {
                            if data {
                                self.out &= !mask
                            }
                        }
                        20 => {
                            if data {
                                self.event &= !mask;
                                if rise & mask != 0 {
                                    self.collisions[bit] += 1;
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                if rise & mask != 0 {
                    self.event |= mask;
                    self.rises[bit] += 1;
                }
            }
            self.s1 = i.pad;
            self.s2 = old_s1;
            self.history = old_s2;
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
    pad: u32,
}
impl Trace {
    fn step(&mut self, i: Input) {
        self.pad = i.pad;
        self.frames.push(self.m.frame(i));
    }
    fn idle(&mut self) {
        self.step(Input {
            pad: self.pad,
            ..Input::default()
        });
    }
    fn mark(&mut self, s: &'static str) {
        *self.tags.entry(s).or_default() += 1;
    }
    fn reset(&mut self) {
        self.step(Input {
            rst: true,
            ..Input::default()
        });
    }
    fn level(&mut self, pad: u32, n: usize) {
        for _ in 0..n {
            self.step(Input {
                pad,
                ..Input::default()
            });
        }
    }
    fn request(&mut self, i: Input, stall: usize) {
        assert!(self.m.response.is_none());
        self.step(Input {
            valid: true,
            pad: self.pad,
            ..i
        });
        for _ in 0..stall {
            self.idle();
        }
        self.step(Input {
            ready: true,
            pad: self.pad,
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
        for addr in [0, 4, 8, 20] {
            self.read(addr);
        }
    }
    // Exactly the event sampling edge is next; request functions must not add a wait.
    fn prepare_rise(&mut self, pad: u32) {
        self.level(0, 3);
        self.level(pad, 2);
        assert_eq!(self.m.s2, pad);
        assert_eq!(self.m.history, 0);
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
    // Every pin is observed at every distinct pipeline phase, including pin31.
    for bit in 0..32 {
        let b = 1u32 << bit;
        t.reset();
        t.level(b, 1);
        assert_eq!((t.m.s1, t.m.s2, t.m.event), (b, 0, 0));
        t.step(Input {
            valid: true,
            addr: 8,
            pad: b,
            ..Input::default()
        });
        assert_eq!(t.m.response.unwrap().data, 0);
        assert_eq!((t.m.s2, t.m.history, t.m.event), (b, 0, 0));
        assert_eq!(t.m.outputs(Input::default())["raw_event"], 1);
        t.step(Input {
            ready: true,
            pad: b,
            ..Input::default()
        });
        assert_eq!(t.m.event, b);
        assert_eq!(t.m.rising(), 0);
        t.read(8);
        assert_eq!(t.m.s2, b);
        t.write(20, b, 15);
        t.level(b, 5);
        assert_eq!(t.m.event, 0);
        t.read(20);
        t.level(0, 4);
        assert_eq!(t.m.event, 0);
        t.prepare_rise(b);
        t.idle();
        assert_eq!(t.m.event, b);
        t.inspect();
        t.mark("every_pin_pipeline_initial_high_repeat_fall");
        // Same-bit W1C on the precise sampling edge, from an already sticky bit.
        t.prepare_rise(b);
        t.write(20, b, 15);
        assert_eq!(t.m.event, b);
        t.read(20);
        t.mark("same_bit_set_beats_clear");
        let other = 1u32 << ((bit + 1) % 32);
        t.prepare_rise(other);
        t.write(20, b, 15);
        assert_eq!(t.m.event, other);
        t.read(20);
        t.mark("other_bit_clear_and_rise");
        // Input -> output DIR write must capture according to OLD input direction.
        t.reset();
        t.prepare_rise(b);
        t.write(0, b, 15);
        assert_eq!(t.m.event, b);
        t.inspect();
        t.mark("old_dir_input_to_output");
        // Output -> input at the event edge must not capture this edge or synthesize one later.
        t.reset();
        t.write(0, b, 15);
        t.prepare_rise(b);
        t.write(0, 0, 15);
        assert_eq!(t.m.event, 0);
        t.level(b, 4);
        assert_eq!(t.m.event, 0);
        t.inspect();
        t.mark("old_dir_output_to_input");
        // Read event snapshot is old even though a new rise is captured at this edge.
        t.reset();
        t.prepare_rise(b);
        t.step(Input {
            valid: true,
            addr: 20,
            pad: b,
            ..Input::default()
        });
        assert_eq!(t.m.response.unwrap().data, 0);
        assert_eq!(t.m.event, b);
        t.step(Input {
            ready: true,
            pad: b,
            ..Input::default()
        });
        t.read(20);
        t.mark("event_old_snapshot");
    }
    // Full WSTRB matrix. SET starts empty AND mixed; CLEAR starts full AND mixed.
    for addr in [0, 4, 12, 16, 20] {
        for strb in 0..16 {
            for data in [0, 0xffff_ffff, 0xa55a_3cc3, 0x8000_0001, 0x0180_0180] {
                let states: &[u32] = match addr {
                    12 => &[0, 0x5aa5_c33c],
                    16 => &[u32::MAX, 0x5aa5_c33c],
                    _ => &[0xa55a_3cc3],
                };
                for &initial in states {
                    t.reset();
                    if addr == 0 {
                        t.write(0, initial, 15);
                    } else if addr == 20 {
                        t.level(initial, 3);
                        t.level(0, 3);
                    } else {
                        t.write(4, initial, 15);
                    }
                    t.write(addr, data, strb);
                    t.inspect();
                    t.mark("full_strobe_matrix");
                    if addr == 12 {
                        t.mark(if initial == 0 {
                            "set_empty"
                        } else {
                            "set_mixed"
                        });
                    }
                    if addr == 16 {
                        t.mark(if initial == u32::MAX {
                            "clear_full"
                        } else {
                            "clear_mixed"
                        });
                    }
                }
            }
        }
    }
    // W1C collision coverage for every strobe, not just full-byte strobes.
    for strb in 0..16 {
        t.reset();
        t.level(u32::MAX, 3);
        t.level(0, 3);
        t.prepare_rise(0x8101_0181);
        t.write(20, u32::MAX, strb);
        t.inspect();
        t.mark("w1c_strobes_with_new_rise");
    }
    // IN must be the two-stage actual pin word even when OUT and DIR disagree.
    for dir in [0, u32::MAX, 0xaaaa_aaaa, 0x5555_5555] {
        for pad in [0, u32::MAX, 0x8000_0001, 0x0180_0180] {
            t.reset();
            t.write(4, !pad, 15);
            t.write(0, dir, 15);
            t.level(pad, 3);
            t.read(8);
            t.inspect();
            t.mark("in_actual_pins_all_directions");
        }
    }
    // Invalid/idle candidates cannot update OUT; each operation has observable initial state.
    for addr in [12, 16] {
        for strb in 0..16 {
            t.reset();
            t.write(4, if addr == 12 { 0 } else { u32::MAX }, 15);
            t.step(Input {
                write: true,
                addr,
                data: u32::MAX,
                strb,
                ..Input::default()
            });
            t.inspect();
            t.mark("idle_set_clear");
        }
    }
    // A legal held request cannot refill at response-consume edge.
    for addr in [12, 16] {
        t.reset();
        t.write(4, if addr == 12 { 0 } else { u32::MAX }, 15);
        t.step(Input {
            valid: true,
            addr: 4,
            ..Input::default()
        });
        let held = Input {
            valid: true,
            write: true,
            addr,
            data: 0x8101_0181,
            strb: 15,
            ..Input::default()
        };
        for _ in 0..5 {
            t.step(held);
        }
        let accepted = t.m.accepted;
        t.step(Input {
            ready: true,
            ..held
        });
        assert_eq!(t.m.accepted, accepted);
        t.step(held);
        assert_eq!(t.m.accepted, accepted + 1);
        for _ in 0..5 {
            t.step(Input {
                write: true,
                addr,
                data: 0x8101_0181,
                strb: 15,
                ..Input::default()
            });
        }
        t.step(Input {
            ready: true,
            ..Input::default()
        });
        t.inspect();
        t.mark("held_write_consume_no_refill");
    }
    // Stalled read snapshot stays unchanged while new hardware events continue.
    t.reset();
    t.step(Input {
        valid: true,
        addr: 8,
        ..Input::default()
    });
    for bit in 0..32 {
        t.level(1 << bit, 3);
        assert_eq!(t.m.response.unwrap().data, 0);
    }
    t.step(Input {
        ready: true,
        pad: t.pad,
        ..Input::default()
    });
    assert_eq!(t.m.event, u32::MAX);
    t.inspect();
    t.mark("backpressure_natural_events");
    for addr in [
        0, 4, 8, 12, 16, 20, 1, 2, 3, 5, 9, 13, 17, 21, 24, 0xfc, 0x100, 0x104, 0x8000, 0xfffc,
        0xffff,
    ] {
        for write in [false, true] {
            for strb in [0, 1, 15] {
                t.reset();
                t.write(4, 0xa55a_3cc3, 15);
                t.prepare_rise(0x8000_0001);
                t.request(
                    Input {
                        addr,
                        write,
                        data: u32::MAX,
                        strb,
                        ..Input::default()
                    },
                    2,
                );
                t.inspect();
                t.mark("permissions_aliases_with_rise");
            }
        }
    }
    // Reset cancels both read and write responses with partially propagated pin input.
    for addr in [4, 8, 12, 16, 20] {
        for write in [false, true] {
            t.reset();
            t.write(4, u32::MAX, 15);
            t.level(1, 3);
            t.write(0, 0xaaaa_aaaa, 15);
            t.step(Input {
                valid: true,
                addr,
                write,
                data: u32::MAX,
                strb: 15,
                pad: 0x8000_0000,
                ..Input::default()
            });
            t.step(Input {
                pad: 0x8000_0000,
                ..Input::default()
            });
            t.step(Input {
                rst: true,
                valid: true,
                write: true,
                addr: 12,
                data: u32::MAX,
                strb: 15,
                pad: u32::MAX,
                ..Input::default()
            });
            assert_eq!(
                (t.m.dir, t.m.out, t.m.event, t.m.s1, t.m.s2, t.m.history),
                (0, 0, 0, 0, 0, 0)
            );
            t.level(u32::MAX, 3);
            assert_eq!(t.m.event, u32::MAX);
            t.inspect();
            t.mark("reset_cancels_response");
        }
    }
    for addr in [0, 4, 12, 16, 20] {
        t.reset();
        t.prepare_rise(u32::MAX);
        t.step(Input {
            rst: true,
            valid: true,
            write: true,
            addr,
            data: u32::MAX,
            strb: 15,
            pad: u32::MAX,
            ..Input::default()
        });
        t.inspect();
        t.mark("reset_beats_available_commit_and_rise");
    }
    let mut seed = seed0;
    let mut held: Option<Input> = None;
    for n in 0..3600 {
        if held.is_none() && random(&mut seed) & 3 != 0 {
            let r = random(&mut seed);
            held = Some(Input {
                valid: true,
                write: r & 1 != 0,
                addr: [0, 4, 8, 12, 16, 20, 1, 24, 0x100, 0x8000][((r >> 1) % 10) as usize],
                data: random(&mut seed),
                strb: ((r >> 8) & 15) as u8,
                ..Input::default()
            });
        }
        let i = Input {
            pad: random(&mut seed),
            ready: random(&mut seed) & 3 != 0,
            rst: n == 731 || n == 1731 || n == 2731,
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
            pad: t.pad,
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
            assert!(b.i.valid);
            assert_eq!(
                (a.i.write, a.i.addr, a.i.data, a.i.strb),
                (b.i.write, b.i.addr, b.i.data, b.i.strb)
            );
        }
    }
    for tag in [
        "every_pin_pipeline_initial_high_repeat_fall",
        "same_bit_set_beats_clear",
        "other_bit_clear_and_rise",
        "old_dir_input_to_output",
        "old_dir_output_to_input",
        "event_old_snapshot",
    ] {
        assert_eq!(t.tags[tag], 32, "{tag}");
    }
    assert_eq!(t.tags["full_strobe_matrix"], 560);
    for tag in ["set_empty", "set_mixed", "clear_full", "clear_mixed"] {
        assert_eq!(t.tags[tag], 80);
    }
    for (tag, count) in [
        ("w1c_strobes_with_new_rise", 16),
        ("in_actual_pins_all_directions", 16),
        ("idle_set_clear", 32),
        ("held_write_consume_no_refill", 2),
        ("backpressure_natural_events", 1),
        ("permissions_aliases_with_rise", 126),
        ("reset_cancels_response", 10),
        ("reset_beats_available_commit_and_rise", 5),
    ] {
        assert_eq!(t.tags[tag], count, "{tag}");
    }
    assert!(t.m.strobes.iter().flatten().all(|n| *n > 0));
    assert!(t.m.rises.iter().chain(&t.m.collisions).all(|n| *n > 0));
    assert!(t.m.cancelled >= 10);
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
        "module tb;reg clk=0,rst=0,req_valid=0,write=0,rsp_ready=0;reg [15:0] addr=0;reg [31:0] wdata=0;reg [3:0] wstrb=0;reg [31:0] pad_in=0;wire [31:0] pad_out,pad_oe;wire req_ready,rsp_valid,raw_event;wire [31:0] rdata;wire [1:0] error;\n",
    );
    writeln!(
        tb,
        "{top} dut(.*);initial begin $dumpfile(\"trace.vcd\");$dumpvars(0,tb);"
    )
    .unwrap();
    for (n, f) in frames.iter().enumerate() {
        let i = f.i;
        writeln!(tb,"rst={};req_valid={};write={};addr=16'h{:04x};wdata=32'h{:08x};wstrb={};rsp_ready={};pad_in=32'h{:08x};#1;",i.rst as u8,i.valid as u8,i.write as u8,i.addr,i.data,i.strb,i.ready as u8,i.pad).unwrap();
        if n != 0 {
            check(&mut tb, &f.before, n, "before");
        }
        tb.push_str("clk=1;#1;\n");
        check(&mut tb, &f.after, n, "after");
        tb.push_str("clk=0;#1;\n");
    }
    tb.push_str("$display(\"FR197 GPIO PASS\");$finish;end endmodule\n");
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
fn p0_gpio_direct_rtl_independent_32_pin_directed_and_seeded_oracle() {
    let hir = GpioCsr::elaborate().unwrap();
    let design = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    for seed in [0x1284_1970_5511, 0xdead_beef_1284, 0x8359_2401_ffff] {
        let t = trace(seed);
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/fr197-gpio")
            .join(format!("direct-{seed:x}-{}-{unique}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("design.v"), &design).unwrap();
        fs::write(dir.join("tb.sv"), testbench(&hir, &t.frames)).unwrap();
        let accounting = format!(
            "seed={seed:x} frames={} accepted={} consumed={} cancelled={} rises={:?} collisions={:?} strobes={:?} directed={:?}\n",
            t.frames.len(),
            t.m.accepted,
            t.m.consumed,
            t.m.cancelled,
            t.m.rises,
            t.m.collisions,
            t.m.strobes,
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
                .contains("FR197 GPIO PASS")
        );
        println!("{accounting}artifacts={}", dir.display());
    }
}

#[test]
fn p0_gpio_independent_scoreboard_detects_input_output_mutation() {
    let hir = GpioCsr::elaborate().unwrap();
    let source = bitloom_vlog::emit(&hir)
        .files
        .into_iter()
        .map(|f| f.contents)
        .collect::<Vec<_>>()
        .join("\n");
    let original = "  assign in_value = sync2;";
    assert_eq!(source.matches(original).count(), 1);
    let mutated = source.replacen(original, "  assign in_value = out_r;", 1);
    // Same existing oracle and comparison generator as the full direct suite.
    // No DUT-derived expectations or mutation-specific golden changes.
    let mut t = Trace::default();
    t.reset();
    t.level(0x80000000, 3);
    t.request(
        Input {
            addr: 8,
            ..Input::default()
        },
        0,
    );
    let tb = testbench(&hir, &t.frames);
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-gpio")
        .join(format!(
            "scoreboard-mutation-{}-{stamp}",
            std::process::id()
        ));
    for (name, rtl) in [("control", source), ("input-is-output", mutated)] {
        let dir = root.join(name);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("design.v"), rtl).unwrap();
        fs::write(dir.join("tb.sv"), &tb).unwrap();
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
        let log = fs::File::create(dir.join("run.log")).unwrap();
        let start = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let status = Command::new("timeout")
            .args(["--kill-after=5s", "60s", "vvp", "simulation"])
            .current_dir(&dir)
            .stdout(Stdio::from(log.try_clone().unwrap()))
            .stderr(Stdio::from(log))
            .status()
            .unwrap();
        fs::write(dir.join("run-command.json"), serde_json::json!({
            "command":["timeout","--kill-after=5s","60s","vvp","simulation"],
            "start_utc_unix_ms": start,
            "end_utc_unix_ms": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            "exit_code": status.code()
        }).to_string()).unwrap();
        let output = fs::read_to_string(dir.join("run.log")).unwrap();
        if name == "control" {
            assert!(
                status.success() && output.contains("FR197 GPIO PASS"),
                "{output}"
            );
        } else {
            assert_eq!(
                status.code(),
                Some(1),
                "expected behavioral fatal, not timeout/tool failure: {output}"
            );
            assert!(
                output.contains("cycle=4 after rdata expected=80000000 got=00000000"),
                "{output}"
            );
            assert!(!output.contains("FR197 GPIO PASS"));
        }
        let trace = fs::read_to_string(dir.join("trace.vcd")).unwrap();
        assert!(trace.contains("$enddefinitions") && trace.contains("#0"));
    }
    println!(
        "GPIO scoreboard control PASS and IN=OUT behavioral counterexample PASS {}",
        root.display()
    );
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
    assert!(
        backend_run(dir, "vvp", &["firrtl-sim"], "firrtl-simulate").contains("FR197 GPIO PASS")
    );
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
    assert!(backend_run(&chisel, "vvp", &["simulation"], "simulate").contains("FR197 GPIO PASS"));
    println!(
        "GPIO vectors FIRRTL and JVM actual RTL PASS {}",
        dir.display()
    );
}

#[test]
#[ignore = "dedicated pinned firtool and JVM full independent vector gate"]
fn p1_gpio_firrtl_chisel_same_independent_vectors() {
    let hir = GpioCsr::elaborate().unwrap();
    // Concatenating all three complete seed suites preserves every directed
    // case and accounting check while compiling each backend only once.
    let mut frames = vec![];
    let mut accounting = String::new();
    for seed in [0x1284_1970_5511, 0xdead_beef_1284, 0x8359_2401_ffff] {
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
        .join("../../target/fr197-gpio")
        .join(format!("backends-{}-{stamp}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let tb = testbench(&hir, &frames);
    fs::write(dir.join("tb.sv"), &tb).unwrap();
    fs::write(dir.join("accounting.log"), &accounting).unwrap();
    vector_backends(&hir, "GpioCsr", &tb, &dir);
    println!("{accounting}");
}
