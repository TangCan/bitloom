//! Story 128.2 P0 active ATDD: independent full-width software oracle vs real RTL.
//! Missing Timer API is the initial compile RED, not evidence of behavioral RED.
use bitloom_prelude::{Elaboratable, FrozenHir, ip::Timer};
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
}
#[derive(Clone, Copy, Debug)]
struct Response {
    data: u32,
    error: u32,
}
#[derive(Default)]
struct Oracle {
    ctrl: u32,
    count: u32,
    compare: u32,
    event: bool,
    pending: Option<Response>,
    accepted: usize,
    consumed: usize,
    cancelled: usize,
    matches: usize,
    suppressed: usize,
    clear_collision: usize,
    strobes: [usize; 16],
}
type Values = BTreeMap<&'static str, u64>;
struct Frame {
    i: Input,
    before: Values,
    after: Values,
}
fn bytes(strobe: u8) -> u32 {
    (0..4)
        .filter(|b| strobe & (1 << b) != 0)
        .fold(0, |mask, b| mask | (0xff << (8 * b)))
}
impl Oracle {
    fn commit(&self, i: Input) -> bool {
        !i.rst && self.pending.is_none() && i.valid
    }
    fn mask(i: Input) -> u32 {
        bytes(i.strb)
            & match i.addr {
                0 => 3,
                4 | 8 => u32::MAX,
                12 => 1,
                _ => 0,
            }
    }
    fn config_write(&self, i: Input) -> bool {
        self.commit(i) && i.write && matches!(i.addr, 0 | 4 | 8) && Self::mask(i) != 0
    }
    fn hit(&self, i: Input) -> bool {
        !i.rst
            && !self.config_write(i)
            && self.ctrl & 1 != 0
            && self.count.wrapping_add(1) == self.compare
    }
    fn outputs(&self, i: Input) -> Values {
        let mut v = Values::from([
            ("req_ready", u64::from(!i.rst && self.pending.is_none())),
            ("rsp_valid", u64::from(self.pending.is_some())),
            ("match_event", u64::from(self.hit(i))),
        ]);
        if let Some(r) = self.pending {
            v.insert("rdata", r.data.into());
            v.insert("error", r.error.into());
        }
        v
    }
    fn frame(&mut self, i: Input) -> Frame {
        let before = self.outputs(i);
        let hit = self.hit(i);
        let config = self.config_write(i);
        if i.rst {
            self.cancelled += usize::from(self.pending.is_some());
            self.ctrl = 0;
            self.count = 0;
            self.compare = 0;
            self.event = false;
            self.pending = None;
        } else {
            let mut clear = false;
            // Response captures OLD software-visible state, independent of the tick below.
            if self.commit(i) {
                let old = match i.addr {
                    0 => Some(self.ctrl),
                    4 => Some(self.count),
                    8 => Some(self.compare),
                    12 => Some(self.event as u32),
                    _ => None,
                };
                self.pending = Some(Response {
                    data: if i.write { 0 } else { old.unwrap_or(0) },
                    error: if old.is_some() { 0 } else { 2 },
                });
                self.accepted += 1;
                if i.write {
                    self.strobes[i.strb as usize] += 1;
                    let mask = Self::mask(i);
                    let merge = |v: u32| (v & !mask) | (i.data & mask);
                    match i.addr {
                        0 => self.ctrl = merge(self.ctrl),
                        4 => self.count = merge(self.count),
                        8 => self.compare = merge(self.compare),
                        12 => clear = i.data & mask & 1 != 0,
                        _ => {}
                    }
                }
            } else if self.pending.is_some() && i.ready {
                self.pending = None;
                self.consumed += 1;
            }
            if config {
                self.suppressed += 1;
            } else if self.ctrl & 1 != 0 {
                self.count = self.count.wrapping_add(1);
                if hit {
                    if self.ctrl & 2 != 0 {
                        self.count = 0;
                    } else {
                        self.ctrl &= !1;
                    }
                }
            }
            self.matches += usize::from(hit);
            self.clear_collision += usize::from(hit && clear);
            self.event = (self.event && !clear) || hit;
        }
        assert_eq!(
            self.accepted,
            self.consumed + self.cancelled + usize::from(self.pending.is_some())
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
    fn idle(&mut self, n: usize) {
        for _ in 0..n {
            self.step(Input::default());
        }
    }
    fn mark(&mut self, s: &'static str) {
        *self.tags.entry(s).or_default() += 1;
    }
    fn request(&mut self, i: Input, stall: usize) {
        assert!(self.m.pending.is_none());
        self.step(Input { valid: true, ..i });
        self.idle(stall);
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
    fn reset(&mut self) {
        self.step(Input {
            rst: true,
            ..Input::default()
        });
    }
    fn inspect(&mut self) {
        for a in [0, 4, 8, 12] {
            self.read(a);
        }
    }
    // Enabling consumes one automatic edge at response consumption. COMPARE=2
    // therefore positions the immediately following accepted request at a match.
    fn imminent(&mut self) {
        self.reset();
        self.write(8, 2, 15);
        self.write(0, 3, 1);
        assert_eq!((self.m.count, self.m.compare, self.m.ctrl), (1, 2, 3));
        assert!(self.m.hit(Input::default()));
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
    // Exhaustive byte lanes on every legal register; observe through bus reads.
    for addr in [0, 4, 8, 12] {
        for strb in 0..16 {
            for data in [0, 0xffff_ffff, 0x805a_00a5] {
                t.reset();
                t.write(addr, 0x1234_5678, 15);
                t.write(addr, data, strb);
                t.inspect();
            }
        }
    }
    t.mark("all_strobes_high_bits");
    for periodic in [false, true] {
        for compare in [1, 2, 17] {
            t.reset();
            t.write(8, compare, 15);
            t.write(0, if periodic { 3 } else { 1 }, 1);
            t.idle(40);
            t.inspect();
            if !periodic {
                assert_eq!(t.m.ctrl, 0);
                assert_eq!(t.m.count, compare);
                t.write(0, 1, 1);
                t.idle(4);
                t.inspect();
            }
        }
    }
    t.mark("periodic_oneshot_1_2_reenable");
    for periodic in [false, true] {
        t.reset();
        t.write(4, 0xffff_fffc, 15);
        t.write(0, if periodic { 3 } else { 1 }, 1);
        t.idle(5);
        t.inspect();
    }
    t.mark("full32_wrap_compare0");
    for periodic in [false, true] {
        t.reset();
        t.write(8, u32::MAX, 15);
        t.write(4, u32::MAX - 2, 15);
        t.write(0, if periodic { 3 } else { 1 }, 1);
        assert_eq!(t.m.count, u32::MAX - 1);
        let matches = t.m.matches;
        t.idle(1);
        assert_eq!(t.m.matches, matches + 1);
        assert_eq!(t.m.count, if periodic { 0 } else { u32::MAX });
        assert_eq!(t.m.ctrl, if periodic { 3 } else { 0 });
        t.inspect();
    }
    t.mark("compare_maximum_oneshot_periodic");
    // Effective same-value and zero writes at would-be match suppress exactly that edge.
    for (addr, same) in [(0, 3), (4, 1), (8, 2)] {
        for value in [same, 0] {
            for strb in [1, 15] {
                t.imminent();
                let matches = t.m.matches;
                t.step(Input {
                    valid: true,
                    write: true,
                    addr,
                    data: value,
                    strb,
                    ..Input::default()
                });
                assert_eq!(t.m.matches, matches);
                t.step(Input {
                    ready: true,
                    ..Input::default()
                });
                t.inspect();
            }
        }
    }
    // Upper byte writes of full-width count/compare remain effective, even same value 0.
    for addr in [4, 8] {
        for strb in [2, 4, 8] {
            t.imminent();
            let matches = t.m.matches;
            t.write(addr, 0, strb);
            assert_eq!(t.frames[t.frames.len() - 2].before["match_event"], 0);
            assert!(t.m.matches <= matches + 1);
            t.inspect();
        }
    }
    t.mark("effective_write_suppression");
    for (addr, strb) in [(0, 0), (0, 2), (0, 4), (0, 8), (0, 14), (4, 0), (8, 0)] {
        t.imminent();
        let matches = t.m.matches;
        t.write(addr, u32::MAX, strb);
        assert_eq!(t.m.matches, matches + 1);
        t.inspect();
    }
    t.mark("zero_effective_mask_does_not_suppress");
    t.reset();
    t.write(4, 100, 15);
    t.write(8, 200, 15);
    t.write(0, 3, 1);
    t.write(8, 2, 15);
    let matches = t.m.matches;
    t.idle(10);
    assert_eq!(matches, t.m.matches);
    t.inspect();
    // Short independently labeled wrap path, not a claim to simulate 2^32 clocks.
    t.write(4, u32::MAX - 1, 15);
    t.idle(4);
    t.inspect();
    t.mark("compare_below_count_then_wrap");
    for (data, strb) in [(1, 1), (0, 1), (1, 2), (u32::MAX, 15)] {
        t.imminent();
        t.write(12, data, strb);
        t.read(12);
        t.inspect();
    }
    t.reset();
    t.write(8, 1, 15);
    t.write(0, 3, 1);
    let matches = t.m.matches;
    t.idle(12);
    assert_eq!(t.m.matches, matches + 12);
    t.write(12, 1, 1);
    assert!(t.m.event);
    t.write(0, 0, 1);
    for (data, strb, expected) in [(0, 1, true), (1, 2, true), (1, 1, false)] {
        t.write(12, data, strb);
        assert_eq!(t.m.event, expected);
        t.read(12);
    }
    t.idle(5);
    t.inspect();
    t.mark("raw_pulse_sticky_and_w1c_set_wins");
    for addr in [
        0, 4, 8, 12, 1, 2, 3, 5, 16, 0x200, 0x204, 0x208, 0x20c, 0x8000, 0xffff,
    ] {
        for write in [false, true] {
            t.imminent();
            t.request(
                Input {
                    addr,
                    write,
                    data: u32::MAX,
                    strb: 15,
                    ..Input::default()
                },
                3,
            );
            t.inspect();
        }
    }
    t.mark("snapshot_errors_and_no_system_alias");
    // Held requests remain stable until accepted, including the consume bubble.
    t.reset();
    t.write(8, 2, 15);
    t.write(0, 3, 1);
    t.step(Input {
        valid: true,
        addr: 4,
        ..Input::default()
    });
    let held = Input {
        valid: true,
        write: true,
        addr: 12,
        data: 1,
        strb: 1,
        ..Input::default()
    };
    for _ in 0..37 {
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
    t.step(Input {
        ready: true,
        ..Input::default()
    });
    t.inspect();
    t.mark("long_stall_snapshot_consume_no_refill");
    // Reset beats an imminent natural hit, accepted-looking config/clear, and live response.
    for addr in [0, 4, 8, 12] {
        t.imminent();
        t.step(Input {
            rst: true,
            valid: true,
            write: true,
            addr,
            data: u32::MAX,
            strb: 15,
            ..Input::default()
        });
        t.inspect();
        t.imminent();
        t.step(Input {
            valid: true,
            addr: 4,
            ..Input::default()
        });
        t.idle(7);
        t.step(Input {
            rst: true,
            valid: true,
            write: true,
            addr,
            data: 1,
            strb: 1,
            ..Input::default()
        });
        t.inspect();
    }
    t.mark("reset_match_write_clear_stalled_response");
    let mut seed = seed0;
    let mut held = None;
    for n in 0..2400 {
        if held.is_none() && random(&mut seed) & 3 != 0 {
            let r = random(&mut seed);
            held = Some(Input {
                valid: true,
                write: r & 1 != 0,
                addr: [0, 4, 8, 12, 1, 0x200, 0x8000][((r >> 1) % 7) as usize],
                data: random(&mut seed),
                strb: ((r >> 8) & 15) as u8,
                ..Input::default()
            });
        }
        let i = Input {
            ready: random(&mut seed) & 3 != 0,
            rst: n == 733 || n == 1733,
            ..held.unwrap_or_default()
        };
        let accepted = t.m.commit(i);
        t.step(i);
        if accepted || i.rst {
            held = None;
        }
    }
    while held.is_some() || t.m.pending.is_some() {
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
            assert!(b.i.valid);
            assert_eq!(
                (a.i.write, a.i.addr, a.i.data, a.i.strb),
                (b.i.write, b.i.addr, b.i.data, b.i.strb)
            );
        }
    }
    assert!(t.m.strobes.iter().all(|n| *n > 0));
    assert!(t.m.cancelled >= 4);
    assert!(t.m.clear_collision > 0);
    assert!(t.m.matches > 50);
    assert_eq!(t.m.accepted, t.m.consumed + t.m.cancelled);
    println!(
        "FR197 seed={seed0:x} frames={} accepted={} consumed={} cancelled={} matches={} suppressed={} clear_collisions={} strobes={:?} directed={:?}",
        t.frames.len(),
        t.m.accepted,
        t.m.consumed,
        t.m.cancelled,
        t.m.matches,
        t.m.suppressed,
        t.m.clear_collision,
        t.m.strobes,
        t.tags
    );
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
        "module tb;reg clk=0,rst=0,req_valid=0,write=0,rsp_ready=0;reg [15:0] addr=0;reg [31:0] wdata=0;reg [3:0] wstrb=0;wire req_ready,rsp_valid,match_event;wire [31:0] rdata;wire [1:0] error;\n",
    );
    writeln!(
        tb,
        "{top} dut(.*);initial begin $dumpfile(\"trace.vcd\");$dumpvars(0,tb);"
    )
    .unwrap();
    for (n, f) in frames.iter().enumerate() {
        let i = f.i;
        writeln!(tb,"rst={};req_valid={};write={};addr=16'h{:04x};wdata=32'h{:08x};wstrb={};rsp_ready={};#1;",i.rst as u8,i.valid as u8,i.write as u8,i.addr,i.data,i.strb,i.ready as u8).unwrap();
        if n != 0 {
            check(&mut tb, &f.before, n, "before");
        }
        tb.push_str("clk=1;#1;\n");
        check(&mut tb, &f.after, n, "after");
        tb.push_str("clk=0;#1;\n");
    }
    tb.push_str("$display(\"FR197 TIMER PASS\");$finish;end endmodule\n");
    tb
}
fn run(dir: &Path, tool: &str, args: &[&str], log_name: &str) {
    let log = fs::File::create(dir.join(log_name)).unwrap();
    let status = Command::new("timeout")
        .args(["--kill-after=5s", "60s", tool])
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .expect("GNU timeout and RTL tools required");
    let output = fs::read_to_string(dir.join(log_name)).unwrap();
    assert!(
        status.success(),
        "{tool} {args:?} failed ({status}) artifacts={}\n{output}",
        dir.display()
    );
}
#[test]
fn p0_timer_direct_rtl_independent_full32_directed_and_seeded_oracle() {
    let hir = Timer::elaborate().unwrap();
    let design = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    for seed in [0x1282_1970_5511, 0xdead_beef_1282, 0x7359_2401_ffff] {
        let t = trace(seed);
        let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/fr197-timer")
            .join(format!("direct-{seed:x}-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("design.v"), &design).unwrap();
        fs::write(dir.join("tb.sv"), testbench(&hir, &t.frames)).unwrap();
        fs::write(dir.join("accounting.log"),format!("seed={seed:x} frames={} accepted={} consumed={} cancelled={} matches={} suppressed={} clear_collisions={} strobes={:?} directed={:?}\n",t.frames.len(),t.m.accepted,t.m.consumed,t.m.cancelled,t.m.matches,t.m.suppressed,t.m.clear_collision,t.m.strobes,t.tags)).unwrap();
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
                .contains("FR197 TIMER PASS")
        );
        println!("artifacts={}", dir.display());
    }
}

// Regression for the backend defects first exposed by Timer's CSR leaf.
// Execute full-u64 literals, initialized memory, and both Reset operand orders.
fn chisel_type_regression() -> FrozenHir {
    use bitloom_prelude::{ElaborateSession, GroundType, Span};
    let mut s = ElaborateSession::new("ChiselTypeRegression");
    let p = Span::default();
    s.begin_module("ChiselTypeRegression", p);
    s.add_input("clk", GroundType::Clock, p);
    s.add_input("rst", GroundType::Reset, p);
    for name in ["bit_in", "address"] {
        s.add_input(name, GroundType::UInt { width: 1 }, p);
    }
    for name in ["high", "maximum", "memory"] {
        s.add_output(name, GroundType::UInt { width: 64 }, p);
    }
    for name in [
        "and_left",
        "and_right",
        "or_left",
        "or_right",
        "xor_left",
        "xor_right",
    ] {
        s.add_output(name, GroundType::UInt { width: 1 }, p);
    }
    s.declare_mem_with_init("rom", 2, 64, vec![0x8000_0000_0000_0000, u64::MAX], p);
    s.begin_combinational(p);
    s.assign_lit("high", 0x8000_0000_0000_0000, p);
    s.assign_lit("maximum", u64::MAX, p);
    s.assign_mem_read("memory", "rom", "address", p);
    s.assign_and("and_left", "rst", "bit_in", p);
    s.assign_and("and_right", "bit_in", "rst", p);
    s.assign_or("or_left", "rst", "bit_in", p);
    s.assign_or("or_right", "bit_in", "rst", p);
    s.assign_xor("xor_left", "rst", "bit_in", p);
    s.assign_xor("xor_right", "bit_in", "rst", p);
    s.end_process();
    s.end_module();
    s.finish().unwrap()
}

const CHISEL_TYPE_TB: &str = r#"
module tb;
reg clk=0,rst=1,bit_in=0,address=0;
wire [63:0] high,maximum,memory;
wire and_left,and_right,or_left,or_right,xor_left,xor_right;
ChiselTypeRegression dut(.clock(clk),.reset(rst),.io_bit_in(bit_in),.io_address(address),
 .io_high(high),.io_maximum(maximum),.io_memory(memory),.io_and_left(and_left),
 .io_and_right(and_right),.io_or_left(or_left),.io_or_right(or_right),.io_xor_left(xor_left),.io_xor_right(xor_right));
integer r,b,a;
initial begin
 $dumpfile("types.vcd"); $dumpvars(0,tb);
 #1; clk=1; #1; clk=0; // Initialize both full-width memory words.
 for(r=0;r<2;r=r+1) for(b=0;b<2;b=b+1) for(a=0;a<2;a=a+1) begin
  rst=r; bit_in=b; address=a; #1;
  if(high !== 64'h8000000000000000 || maximum !== 64'hffffffffffffffff) $fatal(1,"u64 literals");
  if(memory !== (a ? 64'hffffffffffffffff : 64'h8000000000000000)) $fatal(1,"u64 memory address %d",a);
  if(and_left !== (r[0]&b[0]) || and_right !== (b[0]&r[0])) $fatal(1,"Reset AND");
  if(or_left !== (r[0]|b[0]) || or_right !== (b[0]|r[0])) $fatal(1,"Reset OR");
  if(xor_left !== (r[0]^b[0]) || xor_right !== (b[0]^r[0])) $fatal(1,"Reset XOR");
 end
 $display("CHISEL FULL U64 MEMORY RESET OPERANDS PASS"); $finish;
end
endmodule
"#;

#[test]
#[ignore = "dedicated pinned firtool/JVM gate; run --ignored with RHDL_FIRTOOL_PATH"]
fn p1_timer_firrtl_chisel_same_independent_vectors() {
    let hir = Timer::elaborate().unwrap();
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-timer-backends")
        .join(format!("jvm-{}", std::process::id()));
    fs::create_dir_all(root.join("src/main/scala")).unwrap();
    fs::create_dir_all(root.join("project")).unwrap();
    let firtool_dir =
        std::env::var("RHDL_FIRTOOL_PATH").expect("pinned firtool directory required");
    let firtool = format!("{firtool_dir}/firtool");
    run(&root, &firtool, &["--version"], "firtool-version.log");
    assert!(
        fs::read_to_string(root.join("firtool-version.log"))
            .unwrap()
            .contains("firtool-1.159.0")
    );
    let scala = bitloom_firrtl::emit_chisel(&hir).unwrap().files[0]
        .contents
        .clone();
    fs::write(root.join("src/main/scala/Design.scala"), scala).unwrap();
    let regression = bitloom_firrtl::emit_chisel(&chisel_type_regression()).unwrap();
    fs::write(
        root.join("src/main/scala/Types.scala"),
        &regression.files[0].contents,
    )
    .unwrap();
    fs::write(root.join("src/main/scala/Main.scala"),"object TimerMain extends App { circt.stage.ChiselStage.emitSystemVerilogFile(new Timer, args=Array(\"--target-dir\",\"chisel\"), firtoolOpts=Array(\"--disable-all-randomization\",\"--lowering-options=disallowLocalVariables\")); circt.stage.ChiselStage.emitSystemVerilogFile(new ChiselTypeRegression, args=Array(\"--target-dir\",\"types\"), firtoolOpts=Array(\"--disable-all-randomization\",\"--lowering-options=disallowLocalVariables\")) }\n").unwrap();
    fs::write(root.join("build.sbt"),"scalaVersion := \"2.13.18\"\nlibraryDependencies += \"org.chipsalliance\" %% \"chisel\" % \"7.15.0\"\naddCompilerPlugin(\"org.chipsalliance\" % \"chisel-plugin\" % \"7.15.0\" cross CrossVersion.full)\n").unwrap();
    fs::write(
        root.join("project/build.properties"),
        "sbt.version=1.10.11\n",
    )
    .unwrap();
    run(&root, "java", &["-version"], "java-version.log");
    // Set the child environment, never mutate the test process environment.
    let log = fs::File::create(root.join("sbt.log")).unwrap();
    let status = Command::new("timeout")
        .args([
            "--kill-after=5s",
            "240s",
            "sbt",
            "-batch",
            "runMain TimerMain",
        ])
        .env("CHISEL_FIRTOOL_PATH", &firtool_dir)
        .current_dir(&root)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .unwrap();
    fs::write(root.join("sbt.exit"), format!("{status}\n")).unwrap();
    assert!(status.success(), "JVM failure: {}", root.display());
    let types_dir = root.join("types");
    fs::write(types_dir.join("tb.sv"), CHISEL_TYPE_TB).unwrap();
    run(
        &types_dir,
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
        "compile.log",
    );
    run(&types_dir, "vvp", &["simulation"], "run.log");
    assert!(
        fs::read_to_string(types_dir.join("run.log"))
            .unwrap()
            .contains("CHISEL FULL U64 MEMORY RESET OPERANDS PASS")
    );
    let top = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == "Timer")
        .unwrap();
    let connects = top
        .ports
        .iter()
        .map(|p| {
            let port = match p.name.as_str() {
                "clk" => "clock".into(),
                "rst" => "reset".into(),
                n => format!("io_{n}"),
            };
            format!(".{port}({})", p.name)
        })
        .collect::<Vec<_>>()
        .join(", ");
    for seed in [0x1282_1970_5511, 0xdead_beef_1282, 0x7359_2401_ffff] {
        let t = trace(seed);
        let tb = testbench(&hir, &t.frames);
        let dir = root.join(format!("firrtl-{seed:x}"));
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("design.fir"),
            &bitloom_firrtl::emit(&hir).files[0].contents,
        )
        .unwrap();
        fs::write(dir.join("tb.sv"), &tb).unwrap();
        run(
            &dir,
            &firtool,
            &[
                "design.fir",
                "--verilog",
                "--disable-all-randomization",
                "--lowering-options=disallowLocalVariables",
                "-o",
                "design.v",
            ],
            "lower.log",
        );
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
                .contains("FR197 TIMER PASS")
        );
        let dir = root.join("chisel");
        let filename = format!("tb-{seed:x}.sv");
        fs::write(
            dir.join(&filename),
            tb.replace("dut(.*)", &format!("dut({connects})")),
        )
        .unwrap();
        run(
            &dir,
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
                &filename,
            ],
            &format!("compile-{seed:x}.log"),
        );
        run(&dir, "vvp", &["simulation"], &format!("run-{seed:x}.log"));
        assert!(
            fs::read_to_string(dir.join(format!("run-{seed:x}.log")))
                .unwrap()
                .contains("FR197 TIMER PASS")
        );
        fs::rename(
            dir.join("trace.vcd"),
            dir.join(format!("trace-{seed:x}.vcd")),
        )
        .unwrap();
    }
    println!(
        "Timer FIRRTL and JVM/Chisel three-seed actual RTL PASS {}",
        root.display()
    );
}

// [P0] Validate the independent scoreboard itself against two real RTL faults.
// All three simulations use identical drivers and unchanged golden values.
#[test]
fn p0_timer_direct_scoreboard_rejects_mask_and_stalled_data_mutations() {
    let hir = Timer::elaborate().unwrap();
    let design = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let mut t = Trace::default();
    // Nonzero WSTRB selects only reserved CTRL bits: the imminent hit survives.
    t.imminent();
    let match_cycle = t.frames.len();
    t.write(0, u32::MAX, 2);
    assert_eq!(t.frames[match_cycle].before["match_event"], 1);
    // Read snapshot remains stable while the full-width counter keeps advancing.
    t.reset();
    t.write(4, 0x805a_00a5, 15);
    t.write(8, u32::MAX, 15);
    t.write(0, 1, 1);
    let read_cycle = t.frames.len();
    t.request(
        Input {
            addr: 4,
            ..Input::default()
        },
        3,
    );
    let snapshot = t.frames[read_cycle].after["rdata"];
    for frame in &t.frames[read_cycle + 1..read_cycle + 4] {
        assert_eq!(frame.after["rsp_valid"], 1);
        assert_eq!(frame.after["rdata"], snapshot);
    }
    assert_eq!(t.m.accepted, t.m.consumed);
    let tb = testbench(&hir, &t.frames);
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-timer")
        .join(format!("scoreboard-negative-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();

    // Mutate only emitted DUT text, never the oracle, expected values, or driver.
    let mask_old = "  assign config_write = (write_ab | compare_write_commit);";
    let mask_new = "  assign config_write = (write_ab | compare_write_commit) | (req_valid & req_ready & write & ((addr == 16'h0000) | (addr == 16'h0004) | (addr == 16'h0008)));";
    let data_old = "      _csr_data <= _csr_n130;";
    let data_new =
        "      _csr_data <= (_csr_pending && !rsp_ready) ? (_csr_data ^ 32'h00000001) : _csr_n130;";
    for old in [mask_old, data_old] {
        assert_eq!(design.matches(old).count(), 1, "unique DUT mutation anchor");
    }
    let wrong_mask = design.replacen(mask_old, mask_new, 1);
    let wrong_data = design.replacen(data_old, data_new, 1);
    assert_ne!(wrong_mask, design);
    assert_ne!(wrong_data, design);
    assert_eq!(wrong_mask.matches(mask_new).count(), 1);
    assert_eq!(wrong_data.matches(data_new).count(), 1);
    fs::write(
        root.join("mutations.log"),
        format!(
            "frames={}\nmask cycle={match_cycle} before field=match_event\nstalled-data cycle={} after field=rdata\nmask old={mask_old}\nmask new={mask_new}\ndata old={data_old}\ndata new={data_new}\n",
            t.frames.len(), read_cycle + 1
        ),
    )
    .unwrap();

    fn recorded_run(
        dir: &Path,
        tool: &str,
        args: &[&str],
        label: &str,
    ) -> std::process::ExitStatus {
        use std::io::Write as _;
        let mut commands = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(dir.join("commands.log"))
            .unwrap();
        writeln!(
            commands,
            "timeout --kill-after=5s 60s {tool} {}",
            args.join(" ")
        )
        .unwrap();
        let log = fs::File::create(dir.join(format!("{label}.log"))).unwrap();
        let status = Command::new("timeout")
            .args(["--kill-after=5s", "60s", tool])
            .args(args)
            .current_dir(dir)
            .stdout(Stdio::from(log.try_clone().unwrap()))
            .stderr(Stdio::from(log))
            .status()
            .expect("GNU timeout and real RTL tools required");
        fs::write(
            dir.join(format!("{label}.exit")),
            format!("{status}\ncode={:?}\n", status.code()),
        )
        .unwrap();
        status
    }
    for (name, rtl, expected_failure) in [
        ("original", design.as_str(), None),
        (
            "wrong-effective-mask",
            wrong_mask.as_str(),
            Some(format!(
                "cycle={match_cycle} before match_event expected=1 got=0"
            )),
        ),
        (
            "wrong-stalled-data",
            wrong_data.as_str(),
            Some(format!(
                "cycle={} after rdata expected={snapshot:x} got=",
                read_cycle + 1
            )),
        ),
    ] {
        let dir = root.join(name);
        fs::create_dir_all(&dir).unwrap();
        // Each run starts without stale simulation artifacts from any earlier run.
        for artifact in ["simulation", "trace.vcd", "commands.log"] {
            let path = dir.join(artifact);
            if path.exists() {
                fs::remove_file(path).unwrap();
            }
        }
        fs::write(dir.join("design.v"), rtl).unwrap();
        fs::write(dir.join("tb.sv"), &tb).unwrap();
        assert_eq!(fs::read_to_string(dir.join("tb.sv")).unwrap(), tb);
        let compile = recorded_run(
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
        assert!(
            compile.success(),
            "{name}: compilation failure is not mutation detection: {}\n{}",
            dir.display(),
            fs::read_to_string(dir.join("compile.log")).unwrap()
        );
        let simulation = recorded_run(&dir, "vvp", &["simulation"], "run");
        let log = fs::read_to_string(dir.join("run.log")).unwrap();
        assert!(fs::metadata(dir.join("trace.vcd")).unwrap().len() > 0);
        if let Some(expected) = expected_failure {
            // Icarus $fatal(1) returns 1; timeout, signal, missing tool, syntax,
            // or another assertion failure must never count as a detected fault.
            assert_eq!(simulation.code(), Some(1), "{name}: {log}");
            let fatals = log
                .lines()
                .filter(|line| line.starts_with("FATAL:"))
                .collect::<Vec<_>>();
            assert_eq!(fatals.len(), 1, "{name}: {log}");
            assert!(fatals[0].starts_with("FATAL: tb.sv:"), "{name}: {log}");
            assert!(
                fatals[0].contains(&expected),
                "{name}: expected {expected}; {log}"
            );
            assert!(log.contains("Scope: tb"), "{name}: {log}");
            assert!(!log.contains("FR197 TIMER PASS"), "{name}: {log}");
        } else {
            assert!(simulation.success(), "original control must pass: {log}");
            assert!(log.contains("FR197 TIMER PASS"), "{log}");
            assert!(!log.contains("FATAL:"), "{log}");
        }
        println!("scoreboard control={name} artifacts={}", dir.display());
    }
}
