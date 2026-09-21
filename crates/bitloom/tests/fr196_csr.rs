//! Story127.2: active hardware acceptance, independent handwritten oracle.
//! No browser test applies; missing product API is the intended initial compile RED.
use bitloom_hir::{PortDirection, PortValues};
use bitloom_prelude::{
    Diagnostics, ElaborateSession, FrozenHir, GroundType, Span,
    ip::{CsrAccess, CsrBlock, CsrField, CsrOwner, CsrRegister},
};
use bitloom_sim::{GeneratedFunctional, Sim, TickEngine};
use std::{
    collections::BTreeMap,
    fmt::Write as _,
    fs,
    io::{self, Write as _},
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    time::Duration,
};

fn bank() -> CsrBlock {
    CsrBlock {
        name: "Probe".into(),
        registers: [
            (
                "control",
                0,
                CsrAccess::Rw,
                CsrOwner::Leaf,
                0x80ff00ff,
                None,
                false,
                false,
            ),
            (
                "status",
                4,
                CsrAccess::Ro,
                CsrOwner::External,
                0xff,
                None,
                true,
                false,
            ),
            (
                "tx",
                8,
                CsrAccess::Wo,
                CsrOwner::None,
                0xff,
                None,
                false,
                true,
            ),
            (
                "events",
                12,
                CsrAccess::W1c,
                CsrOwner::Leaf,
                0x8000000f,
                Some("hw_events"),
                false,
                false,
            ),
            (
                "counter",
                16,
                CsrAccess::Rw,
                CsrOwner::External,
                0xffffffff,
                None,
                false,
                true,
            ),
        ]
        .into_iter()
        .map(
            |(name, offset, access, owner, mask, event, read_reject, write_reject)| CsrRegister {
                name: name.into(),
                offset,
                reset: 0,
                access,
                owner,
                event: event.map(str::to_string),
                read_reject,
                write_reject,
                fields: vec![CsrField {
                    name: "bits".into(),
                    mask,
                    reset: 0,
                    access,
                }],
            },
        )
        .collect(),
    }
}
#[derive(Clone, Copy, Debug, Default)]
struct Input {
    rst: bool,
    valid: bool,
    write: bool,
    addr: u16,
    data: u32,
    strb: u8,
    ready: bool,
    status: u32,
    counter: u32,
    event: u32,
    reject_read: bool,
    reject_tx: bool,
    reject_counter: bool,
}
#[derive(Clone, Copy, Debug)]
struct Response {
    data: u32,
    error: u32,
}
#[derive(Default)]
struct Oracle {
    control: u32,
    events: u32,
    pending: Option<Response>,
    counter: u32,
    accepted: usize,
    consumed: usize,
    cancelled: usize,
    reads: usize,
    writes: usize,
    errors: usize,
    zero_mask: usize,
    strobes: [usize; 16],
    peer_rejects: usize,
    peer_allowed_high: usize,
    peer_partial_writes: usize,
    max_stall: usize,
    stall: usize,
}
type Values = BTreeMap<String, u64>;
#[derive(Clone)]
struct Frame {
    input: Input,
    before: Values,
    after: Values,
}
fn byte_mask(strobe: u8) -> u32 {
    let mut result = 0;
    for byte in 0..4 {
        if strobe & (1 << byte) != 0 {
            result |= 0xff << (8 * byte);
        }
    }
    result
}
impl Oracle {
    fn outputs(&self, i: Input, peer: bool) -> Values {
        let mut o = Values::new();
        o.insert(
            "req_ready".into(),
            u64::from(!i.rst && self.pending.is_none()),
        );
        o.insert("rsp_valid".into(), u64::from(self.pending.is_some()));
        if let Some(r) = self.pending {
            o.insert("rdata".into(), r.data.into());
            o.insert("error".into(), r.error.into());
        }
        o.insert("control_value".into(), self.control.into());
        o.insert("events_value".into(), self.events.into());
        if peer {
            o.insert("counter_value".into(), self.counter.into());
            o.insert("counter_write_reject".into(), i.reject_counter as u64);
        }
        for name in ["control", "status", "tx", "events", "counter"] {
            o.insert(format!("{name}_read_commit"), 0);
            o.insert(format!("{name}_write_commit"), 0);
        }
        let bytes = byte_mask(i.strb);
        for (name, mask, current, rw) in [
            ("control", 0x80ff00ff, self.control, true),
            ("tx", 0xff, 0, false),
            ("events", 0x8000000f, 0, false),
            ("counter", u32::MAX, i.counter, true),
        ] {
            let mask = bytes & mask;
            let candidate = if rw {
                ((current & !mask) | (i.data & mask))
                    & if name == "control" {
                        0x80ff00ff
                    } else {
                        u32::MAX
                    }
            } else {
                i.data & mask
            };
            o.insert(format!("{name}_write_mask"), mask.into());
            o.insert(format!("{name}_candidate"), candidate.into());
        }
        if !i.rst && i.valid && self.pending.is_none() {
            let (_, name, allowed) = self.access(i);
            if let Some(name) = name {
                if allowed {
                    o.insert(
                        format!("{name}_{}_commit", if i.write { "write" } else { "read" }),
                        1,
                    );
                }
            }
        }
        o
    }
    // Addresses and masks are deliberately handwritten, independent of CsrBlock.
    fn access(&self, i: Input) -> (Response, Option<&'static str>, bool) {
        let (name, mask, readable, writable, value, reject) = match i.addr {
            0 => ("control", 0x80ff00ff, true, true, self.control, false),
            4 => (
                "status",
                0xff,
                true,
                false,
                i.status & 0xff,
                i.reject_read && !i.write,
            ),
            8 => ("tx", 0xff, false, true, 0, i.reject_tx && i.write),
            12 => ("events", 0x8000000f, true, true, self.events, false),
            16 => (
                "counter",
                u32::MAX,
                true,
                true,
                i.counter,
                i.reject_counter && i.write,
            ),
            _ => return (Response { data: 0, error: 2 }, None, false),
        };
        let effective = byte_mask(i.strb) & mask;
        let error = if (i.write && !writable)
            || (!i.write && !readable)
            || (reject && (!i.write || effective != 0))
        {
            2
        } else {
            0
        };
        (
            Response {
                data: if error != 0 || i.write { 0 } else { value },
                error,
            },
            Some(name),
            error == 0 && (!i.write || effective != 0),
        )
    }
    fn frame(&mut self, mut i: Input, peer: bool) -> Frame {
        if peer {
            i.counter = self.counter;
            i.reject_counter =
                (((self.counter & !byte_mask(i.strb)) | (i.data & byte_mask(i.strb))) & 0x80000000)
                    != 0
                    && self.counter & 1 != 0;
        }
        let before = self.outputs(i, peer);
        if i.rst {
            self.cancelled += usize::from(self.pending.is_some());
            self.pending = None;
            self.control = 0;
            self.events = 0;
            self.counter = 0;
            self.stall = 0;
        } else {
            let mut clear = 0;
            let mut peer_written = false;
            if self.pending.is_some() {
                if i.ready {
                    self.pending = None;
                    self.consumed += 1;
                    self.stall = 0;
                } else {
                    self.stall += 1;
                    self.max_stall = self.max_stall.max(self.stall);
                }
            } else if i.valid {
                let (rsp, name, pulse) = self.access(i);
                self.accepted += 1;
                self.errors += usize::from(rsp.error != 0);
                self.peer_rejects += usize::from(
                    peer && i.addr == 16 && i.write && i.reject_counter && byte_mask(i.strb) != 0,
                );
                if i.write {
                    self.strobes[i.strb as usize] += 1;
                }
                if rsp.error == 0 && i.write && !pulse {
                    self.zero_mask += 1;
                }
                self.pending = Some(rsp);
                if pulse {
                    if i.write {
                        self.writes += 1;
                        let merged = |name: &str| before[&format!("{name}_candidate")] as u32;
                        match name.unwrap() {
                            "control" => self.control = merged("control"),
                            "events" => clear = merged("events"),
                            "counter" if peer => {
                                self.peer_partial_writes += usize::from(i.strb != 15);
                                self.peer_allowed_high += usize::from(
                                    merged("counter") & 0x80000000 != 0 && i.counter & 1 == 0,
                                );
                                self.counter = merged("counter");
                                peer_written = true;
                            }
                            _ => {}
                        }
                    } else {
                        self.reads += 1;
                    }
                }
            }
            self.events = ((self.events & !clear) | i.event) & 0x8000000f;
            if peer && !peer_written {
                self.counter = self.counter.wrapping_add(1);
            }
        }
        assert_eq!(
            self.accepted,
            self.consumed + self.cancelled + usize::from(self.pending.is_some())
        );
        let mut after_input = i;
        if peer {
            after_input.counter = self.counter;
            after_input.reject_counter =
                (((self.counter & !byte_mask(i.strb)) | (i.data & byte_mask(i.strb))) & 0x80000000)
                    != 0
                    && self.counter & 1 != 0;
        }
        Frame {
            input: i,
            before,
            after: self.outputs(after_input, peer),
        }
    }
}
fn random(seed: &mut u64) -> u32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed as u32
}
fn transaction(
    frames: &mut Vec<Frame>,
    model: &mut Oracle,
    peer: bool,
    request: Input,
    stall: usize,
) {
    assert!(model.pending.is_none());
    frames.push(model.frame(
        Input {
            valid: true,
            ready: false,
            ..request
        },
        peer,
    ));
    for n in 0..stall {
        frames.push(model.frame(
            Input {
                valid: false,
                ready: false,
                status: 0xffff_ff00 | (n as u32),
                counter: 0xabc0_0000 + n as u32,
                event: 0x8000_0001,
                reject_read: !request.reject_read,
                reject_tx: !request.reject_tx,
                reject_counter: !request.reject_counter,
                ..request
            },
            peer,
        ));
    }
    frames.push(model.frame(
        Input {
            valid: false,
            ready: true,
            ..request
        },
        peer,
    ));
}
fn trace(initial: u64, peer: bool) -> Vec<Frame> {
    let mut seed = initial;
    let mut frames = Vec::new();
    let mut m = Oracle::default();
    frames.push(m.frame(
        Input {
            rst: true,
            valid: true,
            write: true,
            strb: 15,
            data: u32::MAX,
            ..Input::default()
        },
        peer,
    ));
    // All 16 strobes across all four writable access kinds, with zero/full/high data.
    for addr in [0, 8, 12, 16] {
        for strb in 0..16 {
            for data in [0, u32::MAX, 0x805a_00a5] {
                transaction(
                    &mut frames,
                    &mut m,
                    peer,
                    Input {
                        write: true,
                        addr,
                        data,
                        strb,
                        counter: 0x1234_5678,
                        event: 0x8000_0001,
                        ..Input::default()
                    },
                    0,
                );
                if addr != 8 {
                    transaction(
                        &mut frames,
                        &mut m,
                        peer,
                        Input {
                            addr,
                            counter: 0xaabb_ccdd,
                            ..Input::default()
                        },
                        0,
                    );
                }
            }
        }
    }
    // Permissions, address holes/alignment/high bits, even with zero strobes.
    for addr in [1, 2, 3, 5, 20, 0x100, 0x8000, 0xfffc, 0xffff] {
        for write in [false, true] {
            transaction(
                &mut frames,
                &mut m,
                peer,
                Input {
                    addr,
                    write,
                    data: u32::MAX,
                    strb: 15,
                    event: 0xf,
                    ..Input::default()
                },
                0,
            );
        }
    }
    for strb in 0..16 {
        transaction(
            &mut frames,
            &mut m,
            peer,
            Input {
                addr: 4,
                write: true,
                strb,
                ..Input::default()
            },
            0,
        );
        transaction(
            &mut frames,
            &mut m,
            peer,
            Input {
                addr: 8,
                strb,
                ..Input::default()
            },
            0,
        );
        for addr in [8, 16] {
            transaction(
                &mut frames,
                &mut m,
                peer,
                Input {
                    addr,
                    write: true,
                    strb,
                    data: 0x12345678,
                    counter: 0xffeeddcc,
                    reject_tx: true,
                    reject_counter: true,
                    ..Input::default()
                },
                0,
            );
        }
    }
    transaction(
        &mut frames,
        &mut m,
        peer,
        Input {
            addr: 4,
            status: 0xfedc_ba98,
            reject_read: true,
            ..Input::default()
        },
        0,
    );
    transaction(
        &mut frames,
        &mut m,
        peer,
        Input {
            addr: 4,
            status: 0xdead_bea5,
            ..Input::default()
        },
        31,
    );
    transaction(
        &mut frames,
        &mut m,
        peer,
        Input {
            addr: 12,
            event: 0xf,
            ..Input::default()
        },
        31,
    );
    // Cancel a live response and a concurrent apparent request; synchronous reset wins.
    frames.push(m.frame(
        Input {
            valid: true,
            addr: 4,
            status: u32::MAX,
            ..Input::default()
        },
        peer,
    ));
    frames.push(m.frame(
        Input {
            rst: true,
            valid: true,
            write: true,
            strb: 15,
            data: u32::MAX,
            event: u32::MAX,
            ..Input::default()
        },
        peer,
    ));
    transaction(
        &mut frames,
        &mut m,
        peer,
        Input {
            addr: 0,
            ..Input::default()
        },
        0,
    );
    // Sustained legal producer: request tuple held until accepted, environment may change.
    let mut held: Option<Input> = None;
    for cycle in 0..1400 {
        if held.is_none() && random(&mut seed) & 3 != 0 {
            let r = random(&mut seed);
            held = Some(Input {
                valid: true,
                write: r & 1 != 0,
                addr: [0, 4, 8, 12, 16, 1, 0x8000][((r >> 1) % 7) as usize],
                data: random(&mut seed),
                strb: ((r >> 5) & 15) as u8,
                ..Input::default()
            });
        }
        let mut i = held.unwrap_or_default();
        let r = random(&mut seed);
        i.ready = r & 3 != 0;
        i.status = random(&mut seed);
        i.counter = random(&mut seed);
        i.event = random(&mut seed);
        i.reject_read = r & 8 != 0;
        i.reject_tx = r & 16 != 0;
        i.reject_counter = r & 32 != 0;
        i.rst = cycle == 433 || cycle == 901;
        let accept = !i.rst && m.pending.is_none() && i.valid;
        frames.push(m.frame(i, peer));
        if accept || i.rst {
            held = None;
        }
    }
    while m.pending.is_some() || held.is_some() {
        let i = Input {
            ready: true,
            ..held.unwrap_or_default()
        };
        let accept = m.pending.is_none() && i.valid;
        frames.push(m.frame(i, peer));
        if accept {
            held = None;
        }
    }
    for pair in frames.windows(2) {
        let a = &pair[0];
        let b = &pair[1];
        if a.input.valid && a.before["req_ready"] == 0 && !a.input.rst && !b.input.rst {
            assert!(b.input.valid, "producer must hold valid");
            assert_eq!(
                (a.input.write, a.input.addr, a.input.data, a.input.strb),
                (b.input.write, b.input.addr, b.input.data, b.input.strb)
            );
        }
    }
    assert!(m.strobes.iter().all(|&n| n > 0));
    assert!(m.errors > 30 && m.zero_mask > 5);
    assert!(m.reads > 20 && m.writes > 100 && m.cancelled > 0 && m.max_stall >= 31);
    assert_eq!(m.accepted, m.consumed + m.cancelled);
    if peer {
        assert!(m.peer_rejects > 0 && m.peer_partial_writes > 0 && m.peer_allowed_high > 0);
        println!(
            "peer_rejects={} peer_partial_writes={} peer_allowed_high={}",
            m.peer_rejects, m.peer_partial_writes, m.peer_allowed_high
        );
    }
    println!(
        "FR196 seed={initial:x} peer={peer} frames={} accepted={} consumed={} cancelled={} reads={} writes={} errors={} zero_mask={} strobes={:?} max_stall={}",
        frames.len(),
        m.accepted,
        m.consumed,
        m.cancelled,
        m.reads,
        m.writes,
        m.errors,
        m.zero_mask,
        m.strobes,
        m.max_stall
    );
    frames
}
fn input_values(i: Input, peer: bool) -> Values {
    let mut p = Values::new();
    for (name, value) in [
        ("clk", 0),
        ("rst", i.rst as u64),
        ("req_valid", i.valid as u64),
        ("write", i.write as u64),
        ("addr", i.addr.into()),
        ("wdata", i.data.into()),
        ("wstrb", i.strb.into()),
        ("rsp_ready", i.ready as u64),
        ("status_value", i.status.into()),
        ("hw_events", i.event.into()),
        ("status_read_reject", i.reject_read as u64),
        ("tx_write_reject", i.reject_tx as u64),
        ("counter_write_reject", i.reject_counter as u64),
    ] {
        p.insert(name.into(), value);
    }
    if !peer {
        p.insert("counter_value".into(), i.counter.into());
    } else {
        p.remove("counter_write_reject");
    }
    p
}
fn native_check(sim: &Sim, expected: &Values, context: &str) {
    for (name, &value) in expected {
        assert_eq!(sim.ports().get(name), Some(value), "{context} {name}");
    }
}
// Independent producer contract monitor. It uses driven inputs and actual DUT
// ready, never the handwritten expected outputs or the reference state machine.
#[derive(Default)]
struct ProducerMonitor {
    held: BTreeMap<String, [u64; 4]>,
}
impl ProducerMonitor {
    fn sample(&mut self, input: &Values, prefix: &str, ready: u64) {
        if input.get("rst").copied().unwrap_or(0) != 0 {
            self.held.remove(prefix);
            return;
        }
        let get = |name: &str| input.get(&format!("{prefix}{name}")).copied().unwrap_or(0);
        let valid = get("req_valid");
        let payload = [get("write"), get("addr"), get("wdata"), get("wstrb")];
        if let Some(previous) = self.held.get(prefix) {
            assert_eq!(valid, 1, "producer {prefix}: withdrew a stalled request");
            assert_eq!(
                &payload, previous,
                "producer {prefix}: changed a stalled request"
            );
        }
        if valid != 0 && ready == 0 {
            self.held.insert(prefix.into(), payload);
        } else {
            self.held.remove(prefix);
        }
    }
}
fn producer_prefixes(hir: &FrozenHir, top: &str) -> Vec<String> {
    hir.circuit()
        .modules
        .iter()
        .find(|m| m.name == top)
        .unwrap()
        .ports
        .iter()
        .filter(|p| p.direction == PortDirection::Input)
        .filter_map(|p| p.name.strip_suffix("req_valid").map(str::to_owned))
        .collect()
}
fn rtl_producer_monitors(hir: &FrozenHir, top: &str) -> String {
    let mut text = String::new();
    for (index, prefix) in producer_prefixes(hir, top).iter().enumerate() {
        writeln!(
            text,
            r#"reg producer_hold_{index}=0;
reg [52:0] producer_payload_{index}=0;
always @(posedge clk) begin
  if (!rst && producer_hold_{index}) begin
    if ({prefix}req_valid !== 1'b1 ||
        {{{prefix}write,{prefix}addr,{prefix}wdata,{prefix}wstrb}} !== producer_payload_{index})
      $fatal(1,"producer {prefix}: unstable or withdrawn stalled request");
  end
  producer_hold_{index} <= !rst && {prefix}req_valid && !{prefix}req_ready;
  producer_payload_{index} <= {{{prefix}write,{prefix}addr,{prefix}wdata,{prefix}wstrb}};
end"#
        )
        .unwrap();
    }
    text
}

fn run_native(hir: &FrozenHir, frames: &[Frame]) {
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        let mut sim = Sim::with_engine(hir.clone(), engine);
        let mut producer = ProducerMonitor::default();
        for (cycle, f) in frames.iter().enumerate() {
            let mut ports = PortValues::default();
            for (n, v) in input_values(f.input, false) {
                ports.set(n, v);
            }
            sim.set_inputs(ports);
            sim.settle();
            producer.sample(
                &input_values(f.input, false),
                "",
                sim.ports().get("req_ready").unwrap(),
            );
            if cycle > 0 {
                native_check(&sim, &f.before, &format!("{engine:?} cycle={cycle} before"));
            }
            sim.tick();
            native_check(&sim, &f.after, &format!("{engine:?} cycle={cycle} after"));
        }
    }
}
fn testbench(hir: &FrozenHir, top: &str, frames: &[Frame], peer: bool) -> String {
    let module = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == top)
        .unwrap();
    let mut tb = String::from("module tb;\n");
    for p in &module.ports {
        let width = match p.ty {
            GroundType::UInt { width } => width,
            GroundType::Clock | GroundType::Reset => 1,
            _ => panic!("unexpected type"),
        };
        let kind = if p.direction == PortDirection::Input {
            "reg"
        } else {
            "wire"
        };
        writeln!(
            tb,
            "{kind} [{}:0] {}{};",
            width - 1,
            p.name,
            if kind == "reg" { "=0" } else { "" }
        )
        .unwrap();
    }
    tb.push_str(&rtl_producer_monitors(hir, top));
    writeln!(tb, "{top} dut(.*);initial begin").unwrap();
    for (cycle, f) in frames.iter().enumerate() {
        for (name, value) in input_values(f.input, peer) {
            writeln!(tb, "{name}=64'h{value:016x};").unwrap();
        }
        tb.push_str("#1;\n");
        if cycle > 0 {
            rtl_check(&mut tb, &f.before, cycle, "before");
        }
        tb.push_str("clk=1;#1;\n");
        rtl_check(&mut tb, &f.after, cycle, "after");
        tb.push_str("clk=0;#1;\n");
    }
    tb.push_str("$display(\"FR196 PASS\");$finish;end endmodule\n");
    tb
}
fn rtl_check(tb: &mut String, values: &Values, cycle: usize, phase: &str) {
    for (name, value) in values {
        writeln!(tb,"if ({name} !== 64'h{value:016x}) $fatal(1,\"cycle={cycle} {phase} {name} expected={value:x} got=%h\",{name});").unwrap();
    }
}
#[test]
fn p0_leaf_directed_and_random_native_and_real_rtl() {
    let hir = bank().elaborate("ProbeLeaf").unwrap();
    for seed in [0x1272_1960_5511, 0xdead_beef_1272, 0x7359_2401_ffff] {
        let frames = trace(seed, false);
        run_native(&hir, &frames);
        run_rtl(
            &format!("leaf-{seed:x}"),
            &hir,
            &testbench(&hir, "ProbeLeaf", &frames, false),
        );
    }
}
fn peer_body(s: &mut ElaborateSession, _: &[(String, u32)]) -> Result<(), Diagnostics> {
    let sp = Span::default();
    let word = GroundType::UInt { width: 32 };
    s.add_input("clk", GroundType::Clock, sp);
    s.add_input("rst", GroundType::Reset, sp);
    s.add_input("commit", GroundType::UInt { width: 1 }, sp);
    s.add_input("candidate", word.clone(), sp);
    s.add_output("value", word.clone(), sp);
    s.add_output("reject", GroundType::UInt { width: 1 }, sp);
    s.declare_wire("candidate_high", GroundType::UInt { width: 1 }, sp);
    s.declare_wire("current_odd", GroundType::UInt { width: 1 }, sp);
    s.declare_reg("count", word.clone(), sp);
    s.declare_wire("one", word.clone(), sp);
    s.declare_wire("incremented", word, sp);
    s.begin_combinational(sp);
    s.assign_lit("one", 1, sp);
    s.assign_add("incremented", "count", "one", sp);
    s.assign_net("value", "count", sp);
    s.assign_slice("candidate_high", "candidate", 31, 1, sp);
    s.assign_slice("current_odd", "count", 0, 1, sp);
    s.assign_and("reject", "candidate_high", "current_odd", sp);
    s.end_process();
    s.begin_sequential(sp);
    s.assign_reg_d_mux("count", "commit", "candidate", "incremented", sp);
    s.end_process();
    Ok(())
}
fn hierarchy() -> FrozenHir {
    let description = bank();
    let leaf = description.elaborate("ProbeLeaf").unwrap();
    let mut s = ElaborateSession::new("ProbeSystem");
    description.define_module(&mut s, "ProbeLeaf").unwrap();
    s.define_module("CounterPeer", vec![], peer_body).unwrap();
    let sp = Span::default();
    s.begin_module("ProbeSystem", sp);
    let mut connections = Vec::new();
    for p in &leaf.circuit().modules[0].ports {
        if p.name == "counter_value"
            || p.name == "counter_write_reject"
            || p.direction == PortDirection::Output
        {
            s.add_output(p.name.clone(), p.ty.clone(), sp);
        } else {
            s.add_input(p.name.clone(), p.ty.clone(), sp);
        }
        connections.push((p.name.clone(), p.name.clone()));
    }
    s.add_instance("leaf", "ProbeLeaf", connections, vec![], sp);
    s.add_instance(
        "peer",
        "CounterPeer",
        vec![
            ("clk".into(), "clk".into()),
            ("rst".into(), "rst".into()),
            ("commit".into(), "counter_write_commit".into()),
            ("candidate".into(), "counter_candidate".into()),
            ("value".into(), "counter_value".into()),
            ("reject".into(), "counter_write_reject".into()),
        ],
        vec![],
        sp,
    );
    s.end_module();
    let hir = s.finish().unwrap();
    assert_eq!(hir.circuit().modules.len(), 3);
    hir
}
#[test]
fn p0_same_session_real_peer_owns_counter_and_partial_write_wins_increment() {
    let hir = hierarchy();
    let frames = trace(0x1272_0a11_ce55, true);
    run_rtl(
        "peer-system",
        &hir,
        &testbench(&hir, "ProbeSystem", &frames, true),
    );
}
#[test]
fn p1_native_and_generated_hierarchy_are_explicitly_unsupported() {
    let hir = hierarchy();
    let mut errors = Vec::new();
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
            .expect("generated hierarchy must reject"),
    );
    for e in errors {
        let message = e
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| e.downcast_ref::<&str>().copied())
            .unwrap_or("");
        assert!(
            message.contains("hierarchical simulation is unsupported"),
            "{message}"
        );
    }
}

fn bounded(command: &mut Command, log_path: &Path, timeout: Duration) -> io::Result<ExitStatus> {
    let log = fs::File::create(log_path)?;
    // GNU timeout creates and signals a process group, including tool children.
    let mut runner = Command::new("timeout");
    runner
        .arg("--kill-after=5s")
        .arg(format!("{}s", timeout.as_secs_f64()))
        .arg(command.get_program())
        .args(command.get_args());
    if let Some(dir) = command.get_current_dir() {
        runner.current_dir(dir);
    }
    for (key, value) in command.get_envs() {
        if let Some(value) = value {
            runner.env(key, value);
        } else {
            runner.env_remove(key);
        }
    }
    let status = runner
        .stdout(Stdio::from(log.try_clone()?))
        .stderr(Stdio::from(log))
        .status()?;
    writeln!(
        fs::OpenOptions::new().append(true).open(log_path)?,
        "status={status}"
    )?;
    if matches!(status.code(), Some(124 | 137)) {
        return Err(io::Error::new(
            io::ErrorKind::TimedOut,
            format!("tool timed out; log {}", log_path.display()),
        ));
    }
    Ok(status)
}

fn run_rtl(label: &str, hir: &FrozenHir, tb: &str) {
    let design = bitloom_vlog::emit(hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let (status, log) = run_rtl_source(label, &design, tb);
    assert!(status.success(), "FR196 {label}: vvp failed\n{log}");
    assert!(
        log.contains("FR196 PASS"),
        "simulation did not reach success marker"
    );
}

// Compilation and instrument failures always fail the Rust test. Only a completed
// simulation is returned for callers to assert an intentional RTL fatal.
fn run_rtl_source(label: &str, design: &str, tb: &str) -> (ExitStatus, String) {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr196-csr")
        .join(format!("{label}-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("design.v"), design).unwrap();
    fs::write(dir.join("tb.sv"), tb).unwrap();
    let timeout = Duration::from_secs(60);
    let mut commands = String::new();
    for tool in ["iverilog", "vvp"] {
        let path = dir.join(format!("{tool}-version.log"));
        let status = bounded(Command::new(tool).arg("-V"), &path, timeout)
            .expect("FR196 RTL tool required and version probe must complete within 60s");
        assert!(
            status.success(),
            "{tool} version probe failed; {}",
            path.display()
        );
        writeln!(commands, "{tool} -V\n{}", fs::read_to_string(path).unwrap()).unwrap();
    }
    commands.push_str("iverilog -g2012 -s tb -o simulation design.v tb.sv\nvvp simulation\n");
    fs::write(dir.join("commands.log"), commands).unwrap();
    for (tool, args, filename) in [
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
            "compile.log",
        ),
        ("vvp", vec!["simulation"], "run.log"),
    ] {
        let path = dir.join(filename);
        let status = bounded(
            Command::new(tool).args(args).current_dir(&dir),
            &path,
            timeout,
        )
        .expect("FR196 RTL tool must exist and finish within 60s; artifacts preserved");
        let log = fs::read_to_string(&path).unwrap();
        if tool == "vvp" {
            println!("{label}: {log}artifacts={}", dir.display());
            return (status, log);
        }
        assert!(
            status.success(),
            "FR196 {label}: compile failed; artifacts {}\n{log}",
            dir.display()
        );
    }
    unreachable!("simulation stage is mandatory")
}

// Directed fixtures use handwritten values and only the public ports. The same
// before/after expectations run in both native engines and emitted RTL.
struct DirectedFrame {
    inputs: Values,
    before: Values,
    after: Values,
}
fn values(entries: &[(&str, u64)]) -> Values {
    entries.iter().map(|(n, v)| (n.to_string(), *v)).collect()
}
fn directed(
    inputs: &[(&str, u64)],
    before: &[(&str, u64)],
    after: &[(&str, u64)],
) -> DirectedFrame {
    DirectedFrame {
        inputs: values(inputs),
        before: values(before),
        after: values(after),
    }
}
fn run_directed(label: &str, hir: &FrozenHir, top: &str, frames: &[DirectedFrame], native: bool) {
    let module = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == top)
        .unwrap();
    let drive = |frame: &DirectedFrame| {
        let mut inputs: Values = module
            .ports
            .iter()
            .filter(|p| p.direction == PortDirection::Input)
            .map(|p| (p.name.clone(), 0))
            .collect();
        for (name, value) in &frame.inputs {
            assert!(inputs.contains_key(name), "unknown test input {name}");
            inputs.insert(name.clone(), *value);
        }
        inputs
    };
    if native {
        for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
            let mut sim = Sim::with_engine(hir.clone(), engine);
            let mut producer = ProducerMonitor::default();
            for (cycle, frame) in frames.iter().enumerate() {
                let mut inputs = PortValues::default();
                for (name, value) in drive(frame) {
                    inputs.set(name, value);
                }
                sim.set_inputs(inputs);
                sim.settle();
                for prefix in producer_prefixes(hir, top) {
                    producer.sample(
                        &drive(frame),
                        &prefix,
                        sim.ports().get(&format!("{prefix}req_ready")).unwrap(),
                    );
                }
                native_check(
                    &sim,
                    &frame.before,
                    &format!("{label} {engine:?} {cycle} before"),
                );
                sim.tick();
                native_check(
                    &sim,
                    &frame.after,
                    &format!("{label} {engine:?} {cycle} after"),
                );
            }
        }
    }
    let mut tb = String::from("module tb;\n");
    for port in &module.ports {
        let width = match port.ty {
            GroundType::UInt { width } => width,
            GroundType::Clock | GroundType::Reset => 1,
            _ => panic!("unexpected port type"),
        };
        let kind = if port.direction == PortDirection::Input {
            "reg"
        } else {
            "wire"
        };
        writeln!(
            tb,
            "{kind} [{}:0] {}{};",
            width - 1,
            port.name,
            if kind == "reg" { "=0" } else { "" }
        )
        .unwrap();
    }
    tb.push_str(&rtl_producer_monitors(hir, top));
    writeln!(tb, "{top} dut(.*); initial begin").unwrap();
    for (cycle, frame) in frames.iter().enumerate() {
        for (name, value) in drive(frame) {
            writeln!(tb, "{name}=64'h{value:016x};").unwrap();
        }
        tb.push_str("#1;\n");
        rtl_check(&mut tb, &frame.before, cycle, "before");
        tb.push_str("clk=1;#1;\n");
        rtl_check(&mut tb, &frame.after, cycle, "after");
        tb.push_str("clk=0;#1;\n");
    }
    tb.push_str("$display(\"FR196 PASS\");$finish;end endmodule\n");
    run_rtl(label, hir, &tb);
}

#[test]
fn p0_external_rw_masks_reserved_peer_bits_before_read_and_merge() {
    let mut config = bank();
    config.registers[0].owner = CsrOwner::External;
    let hir = config.elaborate("ExternalMasked").unwrap();
    let mut frames = vec![directed(&[("rst", 1)], &[], &[("rsp_valid", 0)])];
    for strb in 0..16u8 {
        // Handwritten sparse mask and raw peer data; never read the descriptor
        // back to derive expected values. Reserved peer bits are deliberately 1.
        let mask = byte_mask(strb) & 0x80ff00ff;
        let candidate = ((0xffff_ffff & !mask) | (0x1234_5678 & mask)) & 0x80ff00ff;
        frames.push(directed(
            &[
                ("req_valid", 1),
                ("write", 1),
                ("wdata", 0x12345678),
                ("wstrb", strb.into()),
                ("control_value", 0xffffffff),
            ],
            &[
                ("req_ready", 1),
                ("control_candidate", candidate.into()),
                ("control_write_mask", mask.into()),
                ("control_write_commit", u64::from(mask != 0)),
            ],
            &[
                ("rsp_valid", 1),
                ("error", 0),
                ("rdata", 0),
                ("control_write_commit", 0),
            ],
        ));
        frames.push(directed(&[("rsp_ready", 1)], &[], &[("rsp_valid", 0)]));
        frames.push(directed(
            &[("req_valid", 1), ("control_value", 0xffffffff)],
            &[("control_read_commit", 1)],
            &[
                ("rsp_valid", 1),
                ("rdata", 0x80ff00ff),
                ("error", 0),
                ("control_read_commit", 0),
            ],
        ));
        frames.push(directed(&[("rsp_ready", 1)], &[], &[("rsp_valid", 0)]));
    }
    run_directed("sparse-external", &hir, "ExternalMasked", &frames, true);
}

#[test]
fn p0_leaf_owned_rejection_events_and_candidates_native_and_real_rtl() {
    let mut config = bank();
    config
        .registers
        .retain(|r| r.name == "control" || r.name == "events");
    for r in &mut config.registers {
        r.write_reject = true;
    }
    let hir = config.elaborate("RejectLeaf").unwrap();
    let frames = vec![
        directed(
            &[("rst", 1), ("hw_events", 15)],
            &[],
            &[("control_value", 0), ("events_value", 0), ("rsp_valid", 0)],
        ),
        // Rejected RW write: candidate remains visible, natural event accumulates.
        directed(
            &[
                ("req_valid", 1),
                ("write", 1),
                ("wdata", 0xffffffff),
                ("wstrb", 15),
                ("control_write_reject", 1),
                ("hw_events", 1),
            ],
            &[
                ("control_candidate", 0x80ff00ff),
                ("control_write_mask", 0x80ff00ff),
                ("control_write_commit", 0),
                ("req_ready", 1),
            ],
            &[
                ("control_value", 0),
                ("events_value", 1),
                ("rsp_valid", 1),
                ("error", 2),
                ("rdata", 0),
            ],
        ),
        // The original request was accepted; no new request is offered while
        // its response stalls. Toggle reject, retaining candidate/mask and
        // allowing natural events. Legal sustained-valid traces run above.
        directed(
            &[
                ("req_valid", 0),
                ("write", 1),
                ("wdata", 0xffffffff),
                ("wstrb", 15),
                ("hw_events", 2),
            ],
            &[
                ("control_candidate", 0x80ff00ff),
                ("control_write_mask", 0x80ff00ff),
                ("control_write_commit", 0),
                ("req_ready", 0),
            ],
            &[
                ("control_value", 0),
                ("events_value", 3),
                ("rsp_valid", 1),
                ("error", 2),
                ("rdata", 0),
            ],
        ),
        directed(
            &[("rsp_ready", 1), ("hw_events", 4)],
            &[],
            &[("events_value", 7), ("rsp_valid", 0)],
        ),
        directed(
            &[
                ("req_valid", 1),
                ("write", 1),
                ("wdata", 0x805a005a),
                ("wstrb", 15),
            ],
            &[
                ("control_write_commit", 1),
                ("control_candidate", 0x805a005a),
                ("control_write_mask", 0x80ff00ff),
            ],
            &[
                ("control_value", 0x805a005a),
                ("events_value", 7),
                ("rsp_valid", 1),
                ("error", 0),
                ("rdata", 0),
                ("control_write_commit", 0),
            ],
        ),
        directed(&[("rsp_ready", 1)], &[], &[("rsp_valid", 0)]),
        // Byte1 is entirely reserved: reject is ignored and no write commits.
        directed(
            &[
                ("req_valid", 1),
                ("write", 1),
                ("wdata", 0xffffffff),
                ("wstrb", 2),
                ("control_write_reject", 1),
            ],
            &[
                ("control_write_commit", 0),
                ("control_write_mask", 0),
                ("control_candidate", 0x805a005a),
            ],
            &[
                ("control_value", 0x805a005a),
                ("rsp_valid", 1),
                ("error", 0),
                ("rdata", 0),
            ],
        ),
        directed(&[("rsp_ready", 1)], &[], &[("rsp_valid", 0)]),
        // Rejected W1C clear cannot erase old bits; simultaneous set is retained.
        directed(
            &[
                ("req_valid", 1),
                ("write", 1),
                ("addr", 12),
                ("wdata", 15),
                ("wstrb", 15),
                ("events_write_reject", 1),
                ("hw_events", 8),
            ],
            &[
                ("events_write_commit", 0),
                ("events_candidate", 15),
                ("events_write_mask", 0x8000000f),
            ],
            &[
                ("events_value", 15),
                ("rsp_valid", 1),
                ("error", 2),
                ("rdata", 0),
            ],
        ),
        directed(
            &[
                ("rsp_ready", 1),
                ("wdata", 15),
                ("wstrb", 15),
                ("hw_events", 0x80000000),
            ],
            &[
                ("events_candidate", 15),
                ("events_write_mask", 0x8000000f),
                ("events_write_commit", 0),
            ],
            &[("events_value", 0x8000000f), ("rsp_valid", 0)],
        ),
        // Accepted clear and set collide: hardware set wins bit1.
        directed(
            &[
                ("req_valid", 1),
                ("write", 1),
                ("addr", 12),
                ("wdata", 0x8000000f),
                ("wstrb", 15),
                ("hw_events", 2),
            ],
            &[
                ("events_write_commit", 1),
                ("events_candidate", 0x8000000f),
                ("events_write_mask", 0x8000000f),
            ],
            &[
                ("events_value", 2),
                ("rsp_valid", 1),
                ("error", 0),
                ("rdata", 0),
                ("events_write_commit", 0),
            ],
        ),
        directed(&[("rsp_ready", 1)], &[], &[("rsp_valid", 0)]),
        directed(
            &[
                ("req_valid", 1),
                ("write", 1),
                ("addr", 12),
                ("wdata", 0xffffffff),
                ("wstrb", 2),
                ("events_write_reject", 1),
                ("hw_events", 4),
            ],
            &[
                ("events_write_commit", 0),
                ("events_candidate", 0),
                ("events_write_mask", 0),
            ],
            &[
                ("events_value", 6),
                ("rsp_valid", 1),
                ("error", 0),
                ("rdata", 0),
            ],
        ),
        directed(&[("rsp_ready", 1)], &[], &[("rsp_valid", 0)]),
        directed(
            &[("req_valid", 1), ("addr", 12), ("hw_events", 8)],
            &[("events_read_commit", 1)],
            &[
                ("rdata", 6),
                ("error", 0),
                ("rsp_valid", 1),
                ("events_value", 14),
                ("events_read_commit", 0),
            ],
        ),
        directed(
            &[],
            &[("rdata", 6)],
            &[
                ("rdata", 6),
                ("events_value", 14),
                ("control_value", 0x805a005a),
            ],
        ),
    ];
    run_directed("leaf-reject", &hir, "RejectLeaf", &frames, true);
}

fn full_word_register(name: &str, offset: u32) -> CsrRegister {
    CsrRegister {
        name: name.into(),
        offset,
        reset: 0,
        access: CsrAccess::Rw,
        owner: CsrOwner::Leaf,
        event: None,
        read_reject: false,
        write_reject: false,
        fields: vec![CsrField {
            name: "bits".into(),
            mask: 0xffffffff,
            reset: 0,
            access: CsrAccess::Rw,
        }],
    }
}
#[test]
fn p0_highest_and_high_bit_separated_addresses_do_not_alias_in_real_rtl() {
    let block = CsrBlock {
        name: "AddressProbe".into(),
        registers: vec![
            full_word_register("low", 0),
            full_word_register("high", 0x8000),
            full_word_register("last", 0xfffc),
        ],
    };
    let hir = block.elaborate("AddressLeaf").unwrap();
    let mut frames = vec![directed(
        &[("rst", 1)],
        &[],
        &[("low_value", 0), ("high_value", 0), ("last_value", 0)],
    )];
    for (addr, data, name) in [
        (0, 0x11223344, "low"),
        (0x8000, 0xaabbccdd, "high"),
        (0xfffc, 0x805a0099, "last"),
    ] {
        frames.push(directed(
            &[
                ("req_valid", 1),
                ("write", 1),
                ("addr", addr),
                ("wdata", data),
                ("wstrb", 15),
            ],
            &[(&format!("{name}_write_commit"), 1)],
            &[
                ("rsp_valid", 1),
                ("error", 0),
                ("rdata", 0),
                (&format!("{name}_value"), data),
            ],
        ));
        frames.push(directed(&[("rsp_ready", 1)], &[], &[("rsp_valid", 0)]));
    }
    for (addr, data, name) in [
        (0xfffc, 0x805a0099, "last"),
        (0, 0x11223344, "low"),
        (0x8000, 0xaabbccdd, "high"),
    ] {
        frames.push(directed(
            &[("req_valid", 1), ("addr", addr)],
            &[(&format!("{name}_read_commit"), 1)],
            &[
                ("rsp_valid", 1),
                ("error", 0),
                ("rdata", data),
                ("low_value", 0x11223344),
                ("high_value", 0xaabbccdd),
                ("last_value", 0x805a0099),
            ],
        ));
        frames.push(directed(&[("rsp_ready", 1)], &[], &[("rsp_valid", 0)]));
    }
    run_directed("high-addresses", &hir, "AddressLeaf", &frames, true);
}

#[test]
fn p0_two_instances_reuse_definition_but_keep_private_state_real_rtl() {
    let block = CsrBlock {
        name: "PrivateState".into(),
        registers: vec![full_word_register("word", 0)],
    };
    let leaf = block.elaborate("SharedCsr").unwrap();
    let mut s = ElaborateSession::new("TwoCsr");
    block.define_module(&mut s, "SharedCsr").unwrap();
    block.define_module(&mut s, "SharedCsr").unwrap(); // actually exercise definition reuse
    let sp = Span::default();
    s.begin_module("TwoCsr", sp);
    s.add_input("clk", GroundType::Clock, sp);
    s.add_input("rst", GroundType::Reset, sp);
    for instance in ["a", "b"] {
        let mut connections = vec![];
        for port in &leaf.circuit().modules[0].ports {
            let wire = if matches!(port.name.as_str(), "clk" | "rst") {
                port.name.clone()
            } else {
                let wire = format!("{instance}_{}", port.name);
                if port.direction == PortDirection::Input {
                    s.add_input(&wire, port.ty.clone(), sp);
                } else {
                    s.add_output(&wire, port.ty.clone(), sp);
                }
                wire
            };
            connections.push((port.name.clone(), wire));
        }
        s.add_instance(instance, "SharedCsr", connections, vec![], sp);
    }
    s.end_module();
    let hir = s.finish().unwrap();
    assert_eq!(hir.circuit().modules.len(), 2);
    let frames = vec![
        directed(
            &[("rst", 1)],
            &[],
            &[("a_word_value", 0), ("b_word_value", 0)],
        ),
        directed(
            &[
                ("a_req_valid", 1),
                ("a_write", 1),
                ("a_wdata", 0x11223344),
                ("a_wstrb", 15),
            ],
            &[("a_word_write_commit", 1), ("b_word_write_commit", 0)],
            &[
                ("a_word_value", 0x11223344),
                ("b_word_value", 0),
                ("a_rsp_valid", 1),
                ("b_rsp_valid", 0),
                ("a_error", 0),
            ],
        ),
        // B commits while A remains stalled; only B's storage changes.
        directed(
            &[
                ("b_req_valid", 1),
                ("b_write", 1),
                ("b_wdata", 0xaabbccdd),
                ("b_wstrb", 15),
            ],
            &[
                ("a_req_ready", 0),
                ("a_word_write_commit", 0),
                ("b_word_write_commit", 1),
            ],
            &[
                ("a_word_value", 0x11223344),
                ("b_word_value", 0xaabbccdd),
                ("a_rsp_valid", 1),
                ("b_rsp_valid", 1),
                ("b_error", 0),
            ],
        ),
        directed(
            &[("a_rsp_ready", 1), ("b_rsp_ready", 1)],
            &[],
            &[("a_rsp_valid", 0), ("b_rsp_valid", 0)],
        ),
        directed(
            &[("a_req_valid", 1), ("b_req_valid", 1)],
            &[("a_word_read_commit", 1), ("b_word_read_commit", 1)],
            &[
                ("a_rdata", 0x11223344),
                ("b_rdata", 0xaabbccdd),
                ("a_rsp_valid", 1),
                ("b_rsp_valid", 1),
                ("a_error", 0),
                ("b_error", 0),
            ],
        ),
        directed(
            &[("a_rsp_ready", 1), ("b_rsp_ready", 1)],
            &[],
            &[("a_rsp_valid", 0), ("b_rsp_valid", 0)],
        ),
        // Partial A-only write cannot leak into the reused B definition.
        directed(
            &[
                ("a_req_valid", 1),
                ("a_write", 1),
                ("a_wdata", 0xee),
                ("a_wstrb", 1),
            ],
            &[("a_word_write_commit", 1), ("b_word_write_commit", 0)],
            &[
                ("a_word_value", 0x112233ee),
                ("b_word_value", 0xaabbccdd),
                ("a_error", 0),
            ],
        ),
        directed(&[("a_rsp_ready", 1)], &[], &[("a_rsp_valid", 0)]),
        directed(
            &[("a_req_valid", 1), ("b_req_valid", 1)],
            &[("a_word_read_commit", 1), ("b_word_read_commit", 1)],
            &[
                ("a_rdata", 0x112233ee),
                ("b_rdata", 0xaabbccdd),
                ("a_rsp_valid", 1),
                ("b_rsp_valid", 1),
                ("a_error", 0),
                ("b_error", 0),
            ],
        ),
    ];
    run_directed("two-instances", &hir, "TwoCsr", &frames, false);
}

#[test]
fn p0_multifield_union_native_and_real_rtl() {
    let mut register = full_word_register("control", 0);
    register.fields = vec![
        CsrField {
            name: "enable".into(),
            mask: 1,
            reset: 0,
            access: CsrAccess::Rw,
        },
        CsrField {
            name: "mode".into(),
            mask: 0xf0,
            reset: 0,
            access: CsrAccess::Rw,
        },
        CsrField {
            name: "high".into(),
            mask: 0x80000000,
            reset: 0,
            access: CsrAccess::Rw,
        },
    ];
    let hir = CsrBlock {
        name: "Fields".into(),
        registers: vec![register],
    }
    .elaborate("FieldsLeaf")
    .unwrap();
    let mut frames = vec![directed(&[("rst", 1)], &[], &[("control_value", 0)])];
    // All masks and next values are explicit golden constants, not obtained
    // from field iteration or any DUT helper. A first-field-only mutant fails.
    for (data, strb, mask, next, commit) in [
        (1, 1, 0xf1, 1, 1),
        (0xffffffff, 8, 0x80000000, 0x80000001, 1),
        (0xa0, 1, 0xf1, 0x800000a0, 1),
        (0xffffffff, 4, 0, 0x800000a0, 0),
        (0xffffffff, 15, 0x800000f1, 0x800000f1, 1),
    ] {
        frames.push(directed(
            &[
                ("req_valid", 1),
                ("write", 1),
                ("wdata", data),
                ("wstrb", strb),
            ],
            &[
                ("control_candidate", next),
                ("control_write_mask", mask),
                ("control_write_commit", commit),
            ],
            &[
                ("control_value", next),
                ("rsp_valid", 1),
                ("rdata", 0),
                ("error", 0),
                ("control_write_commit", 0),
            ],
        ));
        frames.push(directed(&[("rsp_ready", 1)], &[], &[("rsp_valid", 0)]));
        frames.push(directed(
            &[("req_valid", 1)],
            &[("control_read_commit", 1)],
            &[
                ("rsp_valid", 1),
                ("error", 0),
                ("rdata", next),
                ("control_value", next),
            ],
        ));
        frames.push(directed(&[("rsp_ready", 1)], &[], &[("rsp_valid", 0)]));
    }
    run_directed("multifield", &hir, "FieldsLeaf", &frames, true);
}

#[test]
fn p0_read_rejection_for_leaf_external_and_w1c_native_and_real_rtl() {
    let mut config = bank();
    config
        .registers
        .retain(|r| matches!(r.name.as_str(), "control" | "events" | "counter"));
    for r in &mut config.registers {
        r.read_reject = true;
    }
    let hir = config.elaborate("ReadRejectLeaf").unwrap();
    let mut frames = vec![];
    for (name, addr, snapshot) in [
        ("control", 0, 0x805a005a),
        ("events", 12, 0xf),
        ("counter", 16, 0x12345678),
    ] {
        let reject = format!("{name}_read_reject");
        let commit = format!("{name}_read_commit");
        frames.push(directed(
            &[("rst", 1), ("hw_events", 0xffffffff)],
            &[],
            &[("rsp_valid", 0), ("control_value", 0), ("events_value", 0)],
        ));
        frames.push(directed(
            &[
                ("req_valid", 1),
                ("write", 1),
                ("wdata", 0x805a005a),
                ("wstrb", 15),
                ("hw_events", 1),
            ],
            &[("control_write_commit", 1)],
            &[
                ("control_value", 0x805a005a),
                ("events_value", 1),
                ("error", 0),
            ],
        ));
        frames.push(directed(&[("rsp_ready", 1)], &[], &[("rsp_valid", 0)]));
        frames.push(directed(
            &[
                ("req_valid", 1),
                ("addr", addr),
                (&reject, 1),
                ("counter_value", 0xaabbccdd),
                ("hw_events", 2),
            ],
            &[(&commit, 0), ("req_ready", 1)],
            &[
                ("rsp_valid", 1),
                ("error", 2),
                ("rdata", 0),
                ("events_value", 3),
                ("control_value", 0x805a005a),
            ],
        ));
        frames.push(directed(
            &[("counter_value", 0x11111111), ("hw_events", 4)],
            &[(&commit, 0), ("req_ready", 0)],
            &[
                ("rsp_valid", 1),
                ("error", 2),
                ("rdata", 0),
                ("events_value", 7),
            ],
        ));
        frames.push(directed(
            &[("rsp_ready", 1), ("hw_events", 8)],
            &[],
            &[("rsp_valid", 0), ("events_value", 15)],
        ));
        frames.push(directed(
            &[
                ("req_valid", 1),
                ("addr", addr),
                ("counter_value", 0x12345678),
                ("hw_events", 0x80000000),
            ],
            &[(&commit, 1)],
            &[
                ("rsp_valid", 1),
                ("error", 0),
                ("rdata", snapshot),
                ("events_value", 0x8000000f),
                (&commit, 0),
            ],
        ));
        frames.push(directed(
            &[(&reject, 1), ("counter_value", 0xffffffff)],
            &[(&commit, 0)],
            &[
                ("rsp_valid", 1),
                ("error", 0),
                ("rdata", snapshot),
                ("events_value", 0x8000000f),
            ],
        ));
        frames.push(directed(&[("rsp_ready", 1)], &[], &[("rsp_valid", 0)]));
    }
    run_directed("read-reject", &hir, "ReadRejectLeaf", &frames, true);
}

#[test]
fn p0_producer_monitor_rejects_withdrawal_and_payload_change_for_each_prefix() {
    for prefix in ["", "a_", "b_"] {
        for withdrawal in [false, true] {
            let mut monitor = ProducerMonitor::default();
            let mut input = Values::from([
                (format!("{prefix}req_valid"), 1),
                (format!("{prefix}addr"), 4),
            ]);
            monitor.sample(&input, prefix, 0);
            if withdrawal {
                input.insert(format!("{prefix}req_valid"), 0);
            } else {
                input.insert(format!("{prefix}addr"), 8);
            }
            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(
                    || monitor.sample(&input, prefix, 1)
                ))
                .is_err()
            );
            // Reset is the only permitted cancellation before acceptance.
            input.insert("rst".into(), 1);
            monitor.sample(&input, prefix, 0);
        }
    }
}

// Hardware testbench shell, shared by positive controls and fault injection.
fn automation_tb(hir: &FrozenHir, top: &str) -> String {
    let module = hir
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == top)
        .unwrap();
    let mut tb = String::from("module tb;\n");
    for port in &module.ports {
        let width = match port.ty {
            GroundType::UInt { width } => width,
            GroundType::Clock | GroundType::Reset => 1,
            _ => panic!("unexpected CSR port type"),
        };
        let (kind, init) = if port.direction == PortDirection::Input {
            ("reg", "=0")
        } else {
            ("wire", "")
        };
        writeln!(tb, "{kind} [{}:0] {}{init};", width - 1, port.name).unwrap();
    }
    tb.push_str(&rtl_producer_monitors(hir, top));
    writeln!(
        tb,
        "{top} dut(.*);\ntask tick; begin #1;clk=1;#1;clk=0;#1;end endtask\ninitial begin"
    )
    .unwrap();
    tb
}

#[test]
fn p0_real_rtl_producer_monitor_faults_and_legal_cancellation_for_each_prefix() {
    let block = CsrBlock {
        name: "Monitor".into(),
        registers: vec![full_word_register("word", 0)],
    };
    let leaf = block.elaborate("MonitorLeaf").unwrap();
    let mut session = ElaborateSession::new("MonitorTop");
    block.define_module(&mut session, "MonitorLeaf").unwrap();
    let sp = Span::default();
    session.begin_module("MonitorTop", sp);
    session.add_input("clk", GroundType::Clock, sp);
    session.add_input("rst", GroundType::Reset, sp);
    for (index, prefix) in ["", "a_", "b_"].into_iter().enumerate() {
        let mut connections = vec![];
        for port in &leaf.circuit().modules[0].ports {
            let name = if matches!(port.name.as_str(), "clk" | "rst") {
                port.name.clone()
            } else {
                let name = format!("{prefix}{}", port.name);
                if port.direction == PortDirection::Input {
                    session.add_input(&name, port.ty.clone(), sp);
                } else {
                    session.add_output(&name, port.ty.clone(), sp);
                }
                name
            };
            connections.push((port.name.clone(), name));
        }
        session.add_instance(
            format!("leaf{index}"),
            "MonitorLeaf",
            connections,
            vec![],
            sp,
        );
    }
    session.end_module();
    let hir = session.finish().unwrap();
    let design = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    for (index, prefix) in ["", "a_", "b_"].into_iter().enumerate() {
        for action in [
            "withdraw", "write", "addr", "wdata", "wstrb", "reset", "accept",
        ] {
            let mut tb = automation_tb(&hir, "MonitorTop");
            // First accept one read to occupy the response slot, then offer
            // another request while ready is low, arming the real RTL monitor.
            writeln!(tb, "rst=1;tick;rst=0;{prefix}req_valid=1;tick;\nif ({prefix}req_ready !== 0) $fatal(1,\"setup did not stall\");\ntick;").unwrap();
            match action {
                "withdraw" => writeln!(tb, "{prefix}req_valid=0;tick;").unwrap(),
                "write" | "addr" | "wdata" | "wstrb" => writeln!(tb, "{prefix}{action}=1;tick;").unwrap(),
                "reset" => writeln!(tb, "rst=1;{prefix}req_valid=0;{prefix}wdata=99;tick;rst=0;tick;\nif ({prefix}req_ready !== 1 || {prefix}rsp_valid !== 0) $fatal(1,\"reset did not cancel\");\n{prefix}req_valid=1;tick;\nif ({prefix}rsp_valid !== 1) $fatal(1,\"reset recovery did not accept\");").unwrap(),
                "accept" => writeln!(tb, "tick;{prefix}rsp_ready=1;tick;\nif ({prefix}req_ready !== 1) $fatal(1,\"response not drained\");\ntick;\nif ({prefix}rsp_valid !== 1 || {prefix}error !== 0) $fatal(1,\"held request not accepted\");\n{prefix}req_valid=0;tick;").unwrap(),
                _ => unreachable!(),
            }
            tb.push_str("$display(\"FR196 PASS\");$finish;end endmodule\n");
            let (status, log) = run_rtl_source(&format!("monitor-{index}-{action}"), &design, &tb);
            if matches!(action, "reset" | "accept") {
                assert!(
                    status.success(),
                    "positive control {prefix}/{action}: {log}"
                );
                assert!(log.contains("FR196 PASS"), "{log}");
                assert!(!log.contains("FATAL:"), "{log}");
            } else {
                assert_eq!(status.code(), Some(1), "negative {prefix}/{action}: {log}");
                assert!(
                    log.contains(&format!(
                        "producer {prefix}: unstable or withdrawn stalled request"
                    )),
                    "{log}"
                );
                assert!(log.contains("FATAL:"), "{log}");
                assert!(
                    !log.contains("FR196 PASS"),
                    "negative reached success: {log}"
                );
            }
        }
    }
}

#[test]
fn p0_real_rtl_multifield_missing_high_bit_mutant_is_caught_by_independent_oracle() {
    let mut register = full_word_register("control", 0);
    register.fields = [("enable", 1), ("mode", 0xf0), ("high", 0x80000000)]
        .into_iter()
        .map(|(name, mask)| CsrField {
            name: name.into(),
            mask,
            reset: 0,
            access: CsrAccess::Rw,
        })
        .collect();
    let hir = CsrBlock {
        name: "Mutation".into(),
        registers: vec![register],
    }
    .elaborate("MutationLeaf")
    .unwrap();
    let design = bitloom_vlog::emit(&hir)
        .files
        .iter()
        .map(|f| f.contents.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let mut tb = automation_tb(&hir, "MutationLeaf");
    tb.push_str("rst=1;tick;rst=0;req_valid=1;write=1;wdata=32'hffffffff;wstrb=15;tick;\n");
    // Literal golden mask is independent of descriptor iteration and emitter.
    rtl_check(
        &mut tb,
        &values(&[
            ("control_value", 0x800000f1),
            ("rsp_valid", 1),
            ("error", 0),
        ]),
        1,
        "multifield-golden",
    );
    tb.push_str("req_valid=0;rsp_ready=1;tick;req_valid=1;write=0;tick;\n");
    rtl_check(
        &mut tb,
        &values(&[("rdata", 0x800000f1)]),
        3,
        "readback-golden",
    );
    tb.push_str("$display(\"FR196 PASS\");$finish;end endmodule\n");
    let (status, log) = run_rtl_source("multifield-mutation-control", &design, &tb);
    assert!(status.success(), "positive golden control: {log}");
    assert!(log.contains("FR196 PASS"), "{log}");
    // Change only the generated output assignment; preserve the descriptor,
    // stimulus, monitor and golden expectations. Require exactly one mutation.
    let assignment = design
        .lines()
        .filter(|line| line.trim_start().starts_with("assign control_value ="))
        .collect::<Vec<_>>();
    assert_eq!(
        assignment.len(),
        1,
        "expected one generated control_value driver"
    );
    let rhs = assignment[0]
        .split_once('=')
        .unwrap()
        .1
        .trim()
        .strip_suffix(';')
        .unwrap();
    let mutant = design.replacen(
        assignment[0],
        &format!("  assign control_value = ({rhs}) & 32'h7fffffff;"),
        1,
    );
    assert_ne!(mutant, design);
    let (status, log) = run_rtl_source("multifield-missing-high-bit-mutant", &mutant, &tb);
    assert_eq!(status.code(), Some(1), "mutant must fail in vvp: {log}");
    assert!(
        log.contains("FATAL:") && log.contains("multifield-golden control_value expected=800000f1"),
        "wrong failure: {log}"
    );
    assert!(!log.contains("FR196 PASS"), "mutant reached success: {log}");
}
