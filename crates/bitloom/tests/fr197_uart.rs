//! Story 128.5: independent absolute-time UART oracle; no DUT internals or descriptors.
use bitloom_prelude::{Elaboratable, FrozenHir, ip::UartCsr};
use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Write as _,
    fs,
    path::Path,
    process::{Command, Stdio},
};

#[derive(Clone, Copy, Default, Debug)]
struct Input {
    rst: bool,
    valid: bool,
    write: bool,
    addr: u16,
    data: u32,
    strb: u8,
    ready: bool,
    rx: bool,
    phase: u8,
}
#[derive(Clone, Copy, Debug)]
struct Response {
    data: u32,
    error: u32,
}
#[derive(Clone, Copy, Debug)]
struct Tx {
    start: u64,
    period: u64,
    byte: u8,
}
#[derive(Clone, Copy, Debug)]
struct Rx {
    edge: u64,
    period: u64,
    byte: u8,
}
#[derive(Default)]
struct Oracle {
    cycle: u64,
    enable: bool,
    div: u32,
    events: u32,
    txq: VecDeque<u8>,
    rxq: VecDeque<u8>,
    tx: Option<Tx>,
    rx: Option<Rx>,
    s1: bool,
    s2: bool,
    history: bool,
    response: Option<Response>,
    accepted: usize,
    consumed: usize,
    cancelled: usize,
    event_counts: [usize; 4],
    tx_push: usize,
    tx_start: usize,
    tx_cancel: usize,
    tx_complete: usize,
    tx_active_cancel: usize,
    rx_active_cancel: usize,
    rx_push: usize,
    rx_pop: usize,
    rx_cancel: usize,
    samples: Vec<(u64, u64, i32)>,
}
type Values = BTreeMap<&'static str, u64>;
struct Frame {
    i: Input,
    before: Values,
    after: Values,
    tx_launch: Option<(u64, u8)>,
}
fn byte_mask(strb: u8) -> u32 {
    (0..4)
        .filter(|b| strb & (1 << b) != 0)
        .fold(0, |m, b| m | (255 << (b * 8)))
}
impl Oracle {
    fn commit(&self, i: Input) -> bool {
        !i.rst && i.valid && self.response.is_none()
    }
    fn status(&self) -> u32 {
        (!self.rxq.is_empty()) as u32
            | ((self.txq.len() == 4) as u32) << 1
            | (self.tx.is_some() as u32) << 2
            | (self.rx.is_some() as u32) << 3
    }
    // Returns response and successful effective software operation, based solely on prestate.
    fn request(&self, i: Input) -> (Response, bool) {
        let mask = byte_mask(i.strb);
        let busy = self.tx.is_some() || self.rx.is_some();
        let merge_div = (self.div & !mask) | (i.data & mask);
        let new_enable = i.data & 1 != 0;
        let read = match i.addr {
            0 => Some(self.enable as u32),
            4 => Some(self.div),
            8 => Some(self.status()),
            16 => self.rxq.front().map(|x| *x as u32),
            20 => Some(self.events),
            _ => None,
        };
        let writable = matches!(i.addr, 0 | 4 | 12 | 20);
        let effective =
            mask & match i.addr {
                0 => 1,
                4 => u32::MAX,
                12 => 255,
                20 => 15,
                _ => 0,
            } != 0;
        let reject = effective
            && match i.addr {
                0 => (busy && new_enable != self.enable) || (new_enable && self.div < 3),
                4 => busy || (self.enable && merge_div < 3),
                12 => self.txq.len() == 4,
                _ => false,
            };
        let legal = if i.write {
            writable && !reject
        } else {
            read.is_some()
        };
        (
            Response {
                data: if legal && !i.write { read.unwrap() } else { 0 },
                error: if legal { 0 } else { 2 },
            },
            legal && i.write && effective,
        )
    }
    fn effective_config(&self, i: Input) -> (bool, u32) {
        let mut pair = (self.enable, self.div);
        if self.commit(i) && self.request(i).1 {
            if i.addr == 0 {
                pair.0 = i.data & 1 != 0;
            }
            if i.addr == 4 {
                let m = byte_mask(i.strb);
                pair.1 = (self.div & !m) | (i.data & m);
            }
        }
        pair
    }
    fn raw(&self, i: Input) -> u32 {
        if i.rst {
            return 0;
        }
        let mut bits = 0;
        if self.tx.is_none() && self.effective_config(i).0 && !self.txq.is_empty() {
            bits |= 2;
        }
        if let Some(r) = self.rx {
            if self.cycle == r.edge + r.period / 2 + 9 * r.period {
                bits |= if !self.s2 {
                    8
                } else if self.rxq.len() == 4 {
                    4
                } else {
                    1
                };
            }
        }
        bits
    }
    fn tx_level(&self) -> bool {
        match self.tx {
            None => true,
            Some(t) => {
                let bit = (self.cycle - 1 - t.start) / t.period;
                match bit {
                    0 => false,
                    1..=8 => t.byte & (1 << (bit - 1)) != 0,
                    _ => true,
                }
            }
        }
    }
    fn outputs(&self, i: Input) -> Values {
        let mut v = Values::from([
            ("req_ready", (!i.rst && self.response.is_none()) as u64),
            ("rsp_valid", self.response.is_some() as u64),
            ("tx", self.tx_level() as u64),
            ("raw_events", self.raw(i) as u64),
        ]);
        if let Some(r) = self.response {
            v.insert("rdata", r.data as u64);
            v.insert("error", r.error as u64);
        }
        v
    }
    fn frame(&mut self, i: Input) -> Frame {
        let before = self.outputs(i);
        if i.rst {
            self.cancelled += self.response.is_some() as usize;
            self.tx_cancel += self.txq.len();
            self.tx_active_cancel += self.tx.is_some() as usize;
            self.rx_active_cancel += self.rx.is_some() as usize;
            self.rx_cancel += self.rxq.len();
            self.enable = false;
            self.div = 0;
            self.events = 0;
            self.txq.clear();
            self.rxq.clear();
            self.tx = None;
            self.rx = None;
            self.s1 = false;
            self.s2 = false;
            self.history = false;
            self.response = None;
        } else {
            let raw = self.raw(i);
            let config = self.effective_config(i);
            let commit = self.commit(i);
            let (response, write) = self.request(i);
            let old_tx_empty = self.tx.is_none();
            let old_rx_empty = self.rx.is_none();
            if commit {
                self.response = Some(response);
                self.accepted += 1;
                if !i.write && i.addr == 16 && response.error == 0 {
                    self.rxq.pop_front();
                    self.rx_pop += 1;
                }
                if write && i.addr == 20 {
                    self.events &= !(i.data & byte_mask(i.strb) & 15);
                }
            } else if self.response.is_some() && i.ready {
                self.response = None;
                self.consumed += 1;
            }
            // Engine pops first, but software push legality was already decided from prestate.
            if raw & 2 != 0 {
                let byte = self.txq.pop_front().unwrap();
                self.tx_start += 1;
                self.tx = Some(Tx {
                    start: self.cycle,
                    period: u64::from(config.1) + 1,
                    byte,
                });
            }
            if let Some(t) = self.tx {
                if !old_tx_empty && self.cycle == t.start + 10 * t.period {
                    self.tx = None;
                    self.tx_complete += 1;
                }
            }
            if commit && write && i.addr == 12 {
                self.txq.push_back(i.data as u8);
                self.tx_push += 1;
            }
            if let Some(mut r) = self.rx {
                let half = r.period / 2;
                if self.cycle == r.edge + half {
                    self.samples.push((self.cycle, r.edge, -1));
                    if self.s2 {
                        self.rx = None;
                    }
                }
                for k in 0..8 {
                    if self.cycle == r.edge + half + (k + 1) * r.period {
                        self.samples.push((self.cycle, r.edge, k as i32));
                        if self.s2 {
                            r.byte |= 1 << k;
                        }
                        self.rx = Some(r);
                    }
                }
                if self.cycle == r.edge + half + 9 * r.period {
                    self.samples.push((self.cycle, r.edge, 8));
                    if raw & 1 != 0 {
                        self.rxq.push_back(r.byte);
                        self.rx_push += 1;
                    }
                    self.rx = None;
                }
            }
            if old_rx_empty && config.0 && !self.s2 && self.history {
                self.rx = Some(Rx {
                    edge: self.cycle,
                    period: u64::from(config.1) + 1,
                    byte: 0,
                });
            }
            self.enable = config.0;
            self.div = config.1;
            self.events |= raw;
            for bit in 0..4 {
                self.event_counts[bit] += (raw & (1 << bit) != 0) as usize;
            }
            self.history = self.s2;
            self.s2 = self.s1;
            self.s1 = i.rx;
        }
        self.cycle += 1;
        assert!(self.txq.len() <= 4 && self.rxq.len() <= 4);
        assert_eq!(
            self.accepted,
            self.consumed + self.cancelled + self.response.is_some() as usize
        );
        assert_eq!(
            self.tx_push,
            self.tx_start + self.tx_cancel + self.txq.len()
        );
        assert_eq!(self.rx_push, self.rx_pop + self.rx_cancel + self.rxq.len());
        assert_eq!(
            self.tx_start,
            self.tx_complete + self.tx_active_cancel + self.tx.is_some() as usize
        );
        let tx_launch = self
            .tx
            .filter(|t| t.start + 1 == self.cycle)
            .map(|t| (t.period, t.byte));
        Frame {
            tx_launch,
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
    pin: bool,
    phase: u8,
    loopback: bool,
}
impl Trace {
    fn step(&mut self, mut i: Input) {
        i.rx = if self.loopback {
            self.m.tx_level()
        } else {
            self.pin
        };
        i.phase = self.phase;
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
    fn reset(&mut self) {
        self.pin = true;
        self.phase = 1;
        self.step(Input {
            rst: true,
            ..Input::default()
        });
        self.idle(5);
    }
    fn request(&mut self, i: Input, stall: usize) {
        assert!(self.m.response.is_none());
        self.step(Input { valid: true, ..i });
        self.idle(stall);
        self.step(Input {
            ready: true,
            ..Input::default()
        });
    }
    fn wr(&mut self, addr: u16, data: u32, strb: u8) {
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
    fn rd(&mut self, addr: u16) {
        self.request(
            Input {
                addr,
                ..Input::default()
            },
            0,
        );
    }
    fn config(&mut self, div: u32) {
        self.wr(4, div, 15);
        self.wr(0, 1, 1);
    }
    // Independent raw pin sender: never consults receiver ready, busy, timers or samples.
    fn serial(&mut self, byte: u8, div: u32, good_stop: bool) {
        let p = (u64::from(div) + 1) as usize;
        self.pin = false;
        self.idle(p);
        for bit in 0..8 {
            self.pin = byte & (1 << bit) != 0;
            self.idle(p);
        }
        self.pin = good_stop;
        self.idle(p);
    }
    fn inspect(&mut self) {
        for a in [0, 4, 8, 20] {
            self.rd(a);
        }
    }
    fn finish(&mut self) {
        if self.m.response.is_some() {
            self.step(Input {
                ready: true,
                ..Input::default()
            });
        }
        self.inspect();
    }
}
fn trace(seed0: u64) -> Trace {
    let mut t = Trace::default();
    // Full register width and all byte strobes: disabled values need no astronomical simulation.
    for div in [
        0,
        1,
        2,
        3,
        4,
        5,
        0x100,
        0x12345678,
        0x80000000,
        0xfffffffe,
        u32::MAX,
    ] {
        for strb in 0..16 {
            t.reset();
            t.wr(4, div, strb);
            t.rd(4);
            t.mark("wide_strobes");
        }
    }
    for addr in [0, 12, 20] {
        for strb in 0..16 {
            t.reset();
            t.wr(addr, u32::MAX, strb);
            t.inspect();
            t.mark("other_strobes");
        }
    }
    for addr in [0, 4, 8, 12, 16, 20, 1, 2, 24, 0x100, 0xfffc] {
        for write in [false, true] {
            for strb in [0, 15] {
                t.reset();
                t.request(
                    Input {
                        addr,
                        write,
                        strb,
                        data: 0x55,
                        ..Input::default()
                    },
                    3,
                );
                t.inspect();
                t.mark("permissions");
            }
        }
    }
    for div in [3, 4, 5] {
        for phase in [1, 5, 9] {
            for byte in [0, 255, 0x55, 0xa6] {
                t.reset();
                t.phase = phase;
                t.config(div);
                let count = t.m.rx_push;
                t.serial(byte, div, true);
                t.pin = true;
                t.idle(5);
                assert_eq!(t.m.rx_push, count + 1);
                assert_eq!(t.m.rxq.front(), Some(&byte));
                t.rd(16);
                t.rd(20);
                t.mark("serial_phase_byte");
            }
        }
    }
    // No extra idle between frames; stop period belongs only to the preceding frame.
    for div in [3, 4] {
        for phase in [1, 5, 9] {
            t.reset();
            t.phase = phase;
            t.config(div);
            t.serial(0x55, div, true);
            t.serial(0xa6, div, true);
            t.pin = true;
            t.idle(6);
            assert_eq!(
                t.m.rxq.iter().copied().collect::<Vec<_>>(),
                vec![0x55, 0xa6]
            );
            t.rd(16);
            t.rd(16);
            t.mark("back_to_back");
        }
    }
    for div in [3, 4, 5] {
        for phase in [1, 5, 9] {
            t.reset();
            t.phase = phase;
            t.config(div);
            let counts = t.m.event_counts;
            t.pin = false;
            t.idle(1);
            t.pin = true;
            t.idle(80);
            assert_eq!(t.m.event_counts, counts);
            t.rd(20);
            t.mark("short_start");
            t.serial(0x55, div, false);
            t.pin = true;
            t.idle(8);
            assert!(t.m.rxq.is_empty());
            assert_eq!(t.m.events, 8);
            t.rd(20);
            t.mark("framing");
        }
    }
    // Pin release high and release low must not invent an edge.
    for pin in [false, true] {
        t.reset();
        t.pin = pin;
        t.step(Input {
            rst: true,
            ..Input::default()
        });
        t.config(3);
        t.idle(100);
        assert!(t.m.rxq.is_empty());
        assert_eq!(t.m.events, 0);
        t.mark("reset_release");
    }
    // TX queue prefill, byte-select no-op even at full, enable and start on same edge.
    for div in [3, 4, 5, 0x100] {
        t.reset();
        t.wr(4, div, 15);
        for b in [0x00, 0xff, 0x55, 0xa6] {
            t.wr(12, b, 1);
        }
        assert_eq!(t.m.txq.len(), 4);
        t.wr(12, 0xcc, 1);
        t.wr(12, 0xcc, 14);
        t.wr(12, 0xcc, 0);
        t.wr(0, 1, 1);
        t.wr(4, div, 15);
        t.wr(0, 0, 1);
        t.wr(0, 1, 1);
        t.wr(4, 0, 0);
        t.idle((42 * (div + 1)) as usize);
        t.inspect();
        assert!(t.m.txq.is_empty() && t.m.tx.is_none());
        t.mark("tx_decode_busy_configuration");
    }
    // Empty/full/nonfull receive+read collisions at independently computed stop edge.
    for occupancy in [0, 1, 4] {
        for clear in [false, true] {
            t.reset();
            t.config(3);
            for b in 0..occupancy {
                t.serial(0x30 + b, 3, true);
                t.pin = true;
                t.idle(6);
            }
            let old = t.m.rxq.clone();
            let received = t.m.rx_push;
            let overflow = t.m.event_counts[2];
            // 9 physical bits then start stop. FSM e is three edges after raw transition.
            t.pin = false;
            t.idle(4);
            for bit in 0..8 {
                t.pin = 0xa6 & (1 << bit) != 0;
                t.idle(4);
            }
            t.pin = true;
            let r = t.m.rx.expect("independent receiver has pending stop");
            let stop = r.edge + r.period / 2 + 9 * r.period;
            while t.m.cycle < stop {
                t.idle(1);
            }
            assert!(t.m.response.is_none());
            t.step(Input {
                valid: true,
                addr: if clear { 20 } else { 16 },
                write: clear,
                data: 15,
                strb: 15,
                ..Input::default()
            });
            t.idle(12);
            t.step(Input {
                ready: true,
                ..Input::default()
            });
            if occupancy == 4 {
                assert_eq!(t.m.rx_push, received);
                assert_eq!(t.m.event_counts[2], overflow + 1);
            } else {
                assert_eq!(t.m.rx_push, received + 1);
            }
            if !clear && occupancy > 0 {
                assert_eq!(t.frames[t.frames.len() - 14].after["rdata"], old[0] as u64);
            }
            while !t.m.rxq.is_empty() {
                t.rd(16);
            }
            t.inspect();
            t.mark(if clear {
                "event_set_clear_collision"
            } else {
                "rx_prestate_collision"
            });
        }
    }
    // A full TX queue rejects a push even when the idle engine takes its head.
    t.reset();
    t.config(5);
    t.wr(12, 0x11, 1);
    t.idle(1);
    for byte in [0x22, 0x33, 0x44, 0x55] {
        t.wr(12, byte, 1);
    }
    assert_eq!(t.m.txq.len(), 4);
    while t.m.tx.is_some() {
        t.idle(1);
    }
    t.step(Input {
        valid: true,
        write: true,
        addr: 12,
        data: 0xee,
        strb: 1,
        ..Input::default()
    });
    assert_eq!(t.m.response.unwrap().error, 2);
    assert_eq!(t.m.txq.len(), 3);
    t.step(Input {
        ready: true,
        ..Input::default()
    });
    t.mark("tx_full_pop_collision");
    // RX falling-edge detection competes with an idle configuration write.
    for (addr, data, strb, initial_enable) in [
        (0, 1, 1, false),
        (0, 0, 1, true),
        (4, 5, 15, true),
        (4, 0, 0, true),
        (4, 1, 15, true),
    ] {
        t.reset();
        t.wr(4, 3, 15);
        if initial_enable {
            t.wr(0, 1, 1);
        }
        t.pin = false;
        t.idle(2);
        assert!(!t.m.s2 && t.m.history);
        t.step(Input {
            valid: true,
            write: true,
            addr,
            data,
            strb,
            ..Input::default()
        });
        assert_eq!(t.m.rx.is_some(), !(addr == 0 && data == 0));
        if addr == 4 && data == 5 {
            assert_eq!(t.m.rx.unwrap().period, 6);
        }
        t.step(Input {
            ready: true,
            ..Input::default()
        });
        t.pin = true;
        t.idle(90);
        t.inspect();
        t.mark("rx_config_start_collision");
    }
    // Full-width launch observations are deliberately bounded, not 2^32-cycle claims.
    for div in [0x80000000, 0xfffffffe, u32::MAX] {
        t.reset();
        t.wr(4, div, 15);
        t.wr(12, 0xa6, 1);
        t.wr(0, 1, 1);
        t.idle(40);
        assert!(t.m.tx.is_some());
        assert!(!t.m.tx_level());
        assert_eq!(t.m.tx.unwrap().period, u64::from(div) + 1);
        t.rd(4);
        t.mark("full_width_bounded_launch");
    }
    // Reset cancels queued data and response, including interrupted start/data/stop.
    for delay in [0, 1, 3, 8, 20, 36, 39] {
        t.reset();
        t.config(3);
        t.wr(12, 0xa6, 1);
        t.pin = false;
        t.idle(delay);
        t.request(
            Input {
                addr: 8,
                ..Input::default()
            },
            0,
        );
        t.step(Input {
            valid: true,
            write: true,
            addr: 12,
            data: 0x55,
            strb: 1,
            ..Input::default()
        });
        t.step(Input {
            rst: true,
            valid: true,
            write: true,
            addr: 12,
            data: 0xff,
            strb: 1,
            ..Input::default()
        });
        assert!(t.m.response.is_none() && t.m.txq.is_empty() && t.m.rxq.is_empty());
        t.pin = true;
        t.idle(6);
        t.inspect();
        t.mark("reset_cancel");
    }
    // Repeatable legal master traffic while raw serial line evolves independently.
    t.reset();
    t.config(3);
    let mut seed = seed0;
    let mut held = None;
    for n in 0..2400 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        if held.is_none() && seed & 3 != 0 {
            held = Some(Input {
                valid: true,
                addr: [0, 4, 8, 12, 16, 20, 1, 0x100][(seed as usize >> 3) & 7],
                write: seed & 4 != 0,
                data: (seed >> 16) as u32,
                strb: ((seed >> 8) & 15) as u8,
                ..Input::default()
            });
        }
        t.pin = seed & 0x1000 != 0;
        let i = Input {
            ready: seed & 0x2000 != 0,
            rst: n == 731 || n == 1731,
            ..held.unwrap_or_default()
        };
        let take = t.m.commit(i);
        t.step(i);
        if take || i.rst {
            held = None;
        }
    }
    while held.is_some() || t.m.response.is_some() {
        let i = Input {
            ready: true,
            ..held.unwrap_or_default()
        };
        let take = t.m.commit(i);
        t.step(i);
        if take {
            held = None;
        }
    }
    t.finish();
    for (tag, count) in [
        ("tx_full_pop_collision", 1),
        ("rx_config_start_collision", 5),
        ("full_width_bounded_launch", 3),
        ("wide_strobes", 176),
        ("other_strobes", 48),
        ("permissions", 44),
        ("serial_phase_byte", 36),
        ("back_to_back", 6),
        ("short_start", 9),
        ("framing", 9),
        ("reset_release", 2),
        ("tx_decode_busy_configuration", 4),
        ("rx_prestate_collision", 3),
        ("event_set_clear_collision", 3),
        ("reset_cancel", 7),
    ] {
        assert_eq!(t.tags[tag], count, "{tag}");
    }
    assert!(t.m.event_counts.iter().all(|n| *n > 0));
    assert_eq!(t.m.accepted, t.m.consumed + t.m.cancelled);
    t
}
fn check(tb: &mut String, v: &Values, cycle: usize, phase: &str) {
    for (name, value) in v {
        writeln!(tb,"if ({name} !== 64'h{value:x}) $fatal(1,\"UART cycle={cycle} {phase} {name} expected={value:x} got=%h\",{name});").unwrap();
    }
}
fn testbench(hir: &FrozenHir, frames: &[Frame]) -> String {
    let top = &hir.circuit().name;
    let mut tb = String::from(
        "`timescale 1ns/1ps\nmodule tb;reg clk=1,rst=0,req_valid=0,write=0,rsp_ready=0,rx=1;reg [15:0] addr=0;reg [31:0] wdata=0;reg [3:0] wstrb=0;wire tx,req_ready,rsp_valid;wire [3:0] raw_events;wire [31:0] rdata;wire [1:0] error;reg [7:0] decoded=0;\n",
    );
    writeln!(
        tb,
        "{top} dut(.*);initial begin $dumpfile(\"trace.vcd\");$dumpvars(0,tb);"
    )
    .unwrap();
    let mut decoder: Option<(usize, u64, u8)> = None;
    for (n, f) in frames.iter().enumerate() {
        let i = f.i;
        writeln!(
            tb,
            "rst={};req_valid={};write={};addr=16'h{:x};wdata=32'h{:x};wstrb={};rsp_ready={};",
            i.rst as u8, i.valid as u8, i.write as u8, i.addr, i.data, i.strb, i.ready as u8
        )
        .unwrap();
        // Actual sub-cycle phase relative to preceding rising edge. Last check consumed 0.01ns.
        let phase = if i.phase == 0 { 1 } else { i.phase };
        if phase <= 5 {
            writeln!(
                tb,
                "#{};rx={};#{};clk=0;#4.9;",
                phase as f64 - 0.01,
                i.rx as u8,
                5.0 - phase as f64
            )
            .unwrap();
        } else {
            writeln!(
                tb,
                "#4.99;clk=0;#{};rx={};#{};",
                phase as f64 - 5.0,
                i.rx as u8,
                9.9 - phase as f64
            )
            .unwrap();
        }
        if n != 0 {
            check(&mut tb, &f.before, n, "before");
        }
        tb.push_str("#0.1;clk=1;#0.01;\n");
        check(&mut tb, &f.after, n, "after");
        // A second observer reconstructs a byte from actual TX samples at bit centres.
        // Timing comes from the external launch transaction, never internal DUT state.
        if i.rst {
            decoder = None;
            tb.push_str("decoded=0;\n");
        }
        if let Some((period, byte)) = f.tx_launch {
            decoder = Some((n, period, byte));
            tb.push_str("decoded=0;\n");
        }
        if let Some((start, period, byte)) = decoder {
            let elapsed = (n - start) as u64;
            for bit in 0..8 {
                if elapsed == (bit + 1) * period + period / 2 {
                    writeln!(tb, "decoded[{bit}]=tx;").unwrap();
                }
            }
            if elapsed == 9 * period + period / 2 {
                writeln!(tb,"if(tx !== 1'b1 || decoded !== 8'h{byte:02x}) $fatal(1,\"UART independent TX decoder expected={byte:02x} got=%h\",decoded);").unwrap();
                decoder = None;
            }
        }
    }
    tb.push_str("$display(\"FR197 UART PASS\");$finish;end endmodule\n");
    tb
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
        backend_run(dir, "vvp", &["firrtl-sim"], "firrtl-simulate").contains("FR197 UART PASS")
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
    assert!(backend_run(&chisel, "vvp", &["simulation"], "simulate").contains("FR197 UART PASS"));
    println!(
        "UART vectors FIRRTL and JVM actual RTL PASS {}",
        dir.display()
    );
}

fn execute(backends: bool) {
    let hir = UartCsr::elaborate().unwrap();
    let mut frames = vec![];
    let mut accounting = String::new();
    for seed in [0x1285_1970_5511, 0xdead_beef_1285, 0x8359_2501_ffff] {
        let mut t = trace(seed);
        writeln!(accounting,"seed={seed:x} frames={} accepted={} consumed={} cancelled={} tx_push={} tx_start={} tx_cancel={} rx_push={} rx_pop={} rx_cancel={} events={:?} samples={} directed={:?}",t.frames.len(),t.m.accepted,t.m.consumed,t.m.cancelled,t.m.tx_push,t.m.tx_start,t.m.tx_cancel,t.m.rx_push,t.m.rx_pop,t.m.rx_cancel,t.m.event_counts,t.m.samples.len(),t.tags).unwrap();
        writeln!(accounting,"active-ledger seed={seed:x} tx_start={} tx_complete={} tx_active_cancel={} tx_active={} rx_active_cancel={}",t.m.tx_start,t.m.tx_complete,t.m.tx_active_cancel,t.m.tx.is_some() as usize,t.m.rx_active_cancel).unwrap();
        t.frames[0].before.clear();
        frames.extend(t.frames);
    }
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-uart")
        .join(format!(
            "{}-{}-{stamp}",
            if backends { "backends" } else { "direct" },
            std::process::id()
        ));
    fs::create_dir_all(&dir).unwrap();
    let tb = testbench(&hir, &frames);
    fs::write(dir.join("tb.sv"), &tb).unwrap();
    fs::write(dir.join("accounting.log"), &accounting).unwrap();
    if backends {
        vector_backends(&hir, "UartCsr", &tb, &dir);
    } else {
        let rtl = bitloom_vlog::emit(&hir)
            .files
            .into_iter()
            .map(|f| f.contents)
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(dir.join("design.v"), rtl).unwrap();
        backend_run(
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
        assert!(backend_run(&dir, "vvp", &["simulation"], "simulate").contains("FR197 UART PASS"));
    }
    println!("{accounting}artifacts={}", dir.display());
}
#[test]
fn p0_uart_direct_rtl_independent_serial_csr_fifo_oracle() {
    execute(false);
}
#[test]
#[ignore = "dedicated actual FIRRTL and Chisel UART simulation"]
fn p1_uart_firrtl_chisel_same_independent_serial_vectors() {
    execute(true);
}

fn replace_identifiers(text: &str, names: &BTreeMap<String, String>) -> String {
    let mut result = String::new();
    let mut chars = text.char_indices().peekable();
    while let Some((start, c)) = chars.next() {
        if c.is_ascii_alphabetic() || c == '_' {
            let mut end = start + 1;
            while let Some(&(i, c)) = chars.peek() {
                if c.is_ascii_alphanumeric() || c == '_' {
                    end = i + 1;
                    chars.next();
                } else {
                    break;
                }
            }
            let token = &text[start..end];
            result.push_str(names.get(token).map(String::as_str).unwrap_or(token));
        } else {
            result.push(c);
        }
    }
    result
}
fn pair_hir(renamed: bool) -> FrozenHir {
    use bitloom_hir::PortDirection;
    use bitloom_prelude::{ElaborateSession, GroundType, Span, ip::Irq};
    let reference = UartCsr::elaborate().unwrap();
    let ports = &reference
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == "UartCsr")
        .unwrap()
        .ports;
    let irq = Irq::elaborate().unwrap();
    let irq_ports = &irq
        .circuit()
        .modules
        .iter()
        .find(|m| m.name == "Irq")
        .unwrap()
        .ports;
    let mut s = ElaborateSession::new("UartPair");
    UartCsr::define_module(&mut s, "SharedUart").unwrap();
    UartCsr::define_module(&mut s, if renamed { "OtherUart" } else { "SharedUart" }).unwrap();
    Irq::define_module(&mut s, "EventIrq").unwrap();
    let sp = Span::default();
    s.begin_module("UartPair", sp);
    s.add_input("clk", GroundType::Clock, sp);
    s.add_input("rst", GroundType::Reset, sp);
    for lane in 0..2 {
        let mut connections = vec![];
        for p in ports {
            let name = if p.name == "clk" || p.name == "rst" {
                p.name.clone()
            } else {
                format!("u{lane}_{}", p.name)
            };
            if p.name != "clk" && p.name != "rst" {
                if p.direction == PortDirection::Input {
                    s.add_input(&name, p.ty.clone(), sp);
                } else {
                    s.add_output(&name, p.ty.clone(), sp);
                }
            }
            connections.push((p.name.clone(), name));
        }
        s.add_instance(
            format!("uart{lane}"),
            if lane == 1 && renamed {
                "OtherUart"
            } else {
                "SharedUart"
            },
            connections,
            vec![],
            sp,
        );
    }
    let mut connections = vec![];
    for p in irq_ports {
        let name = if p.name == "clk" || p.name == "rst" {
            p.name.clone()
        } else {
            format!("i_{}", p.name)
        };
        if p.name == "raw_events" {
            s.declare_wire(&name, p.ty.clone(), sp);
        } else if p.name != "clk" && p.name != "rst" {
            if p.direction == PortDirection::Input {
                s.add_input(&name, p.ty.clone(), sp);
            } else {
                s.add_output(&name, p.ty.clone(), sp);
            }
        }
        connections.push((p.name.clone(), name));
    }
    s.add_instance("interrupts", "EventIrq", connections, vec![], sp);
    for (n, width) in [
        ("zero", 1),
        ("rx_event", 1),
        ("tx_event", 1),
        ("overflow", 1),
        ("framing", 1),
        ("error_event", 1),
        ("low", 2),
        ("middle", 3),
        ("four", 4),
    ] {
        s.declare_wire(n, GroundType::UInt { width }, sp);
    }
    s.begin_combinational(sp);
    s.assign_lit("zero", 0, sp);
    for (n, lo) in [
        ("rx_event", 0),
        ("tx_event", 1),
        ("overflow", 2),
        ("framing", 3),
    ] {
        s.assign_slice(n, "u0_raw_events", lo, 1, sp);
    }
    s.assign_or("error_event", "overflow", "framing", sp);
    s.assign_concat("low", "rx_event", "zero", sp);
    s.assign_concat("middle", "tx_event", "low", sp);
    s.assign_concat("four", "error_event", "middle", sp);
    s.assign_concat("i_raw_events", "zero", "four", sp);
    s.end_process();
    s.end_module();
    s.finish().unwrap()
}
fn pair_tb(hir: &FrozenHir, frames: &[Frame], other: &[Frame]) -> String {
    let standalone = UartCsr::elaborate().unwrap();
    let mut declarations = String::new();
    let mut processes = String::new();
    for (lane, frames) in [(0, frames), (1, other)] {
        let original = testbench(&standalone, frames);
        let body = original.split_once("module tb;").unwrap().1;
        let (decl, body) = body.split_once("UartCsr dut(.*);").unwrap();
        let mut map = BTreeMap::new();
        for n in [
            "req_valid",
            "write",
            "rsp_ready",
            "rx",
            "addr",
            "wdata",
            "wstrb",
            "tx",
            "req_ready",
            "rsp_valid",
            "raw_events",
            "rdata",
            "error",
            "decoded",
        ] {
            map.insert(n.into(), format!("u{lane}_{n}"));
        }
        let mut decl = replace_identifiers(decl, &map);
        if lane == 1 {
            decl = decl.replace("reg clk=1,rst=0,", "reg ");
        }
        declarations.push_str(&decl);
        let mut body = replace_identifiers(body, &map)
            .replace("end endmodule", "end")
            .replace("$finish;", &format!("done{lane}=1;"));
        if lane == 1 {
            body = body
                .replace("$dumpfile(\"trace.vcd\");$dumpvars(0,tb);", "")
                .replace("clk=0;", "")
                .replace("clk=1;", "");
            for value in [0, 1] {
                body = body.replace(&format!("rst={value};"), "");
            }
        }
        processes.push_str(&body);
    }
    let mut tb = format!(
        "`timescale 1ns/1ps\nmodule tb;{declarations}\nreg done0=0,done1=0,done_irq=0;\nreg i_req_valid=0,i_write=0,i_rsp_ready=0;reg [15:0] i_addr=0;reg [31:0] i_wdata=0;reg [3:0] i_wstrb=0;wire i_req_ready,i_rsp_valid,i_irq;wire[31:0] i_rdata;wire[1:0] i_error;\n{} dut(.*);{processes}\ninitial begin\n",
        hir.circuit().name
    );
    let mut pending = 0u32;
    let mut enable = 0u32;
    let mut response = None;
    let mut clears_with_event = 0;
    let mut quiet_clears = 0;
    let mut dual_clear = 0;
    let mut sticky_quiet_clear = 0;
    let mut subsequent_event = 0;
    let mut local_sticky = 0u32;
    let mut awaiting_next = false;
    let collision = |f: &Frame| {
        !f.i.rst
            && f.i.valid
            && f.i.write
            && f.i.addr == 20
            && f.i.strb & 1 != 0
            && f.before.get("req_ready") == Some(&1)
            && f.before.get("raw_events").copied().unwrap_or(0) & u64::from(f.i.data) & 15 != 0
    };
    for (cycle, f) in frames.iter().enumerate() {
        let critical = collision(f);
        let reserve = frames.get(cycle + 1).is_some_and(collision);
        let valid = critical || (!reserve && cycle % 3 != 2);
        let write = critical || cycle % 12 < 6;
        let addr = if critical {
            0
        } else if cycle % 12 < 3 {
            4
        } else if cycle % 12 < 6 {
            0
        } else if cycle % 12 < 9 {
            12
        } else {
            0
        };
        let raw = f.before.get("raw_events").copied().unwrap_or(0) as u32;
        let raw = ((raw & 3) << 1) | if raw & 12 != 0 { 8 } else { 0 };
        let ready = true;
        let req_ready = !f.i.rst && response.is_none();
        writeln!(
            tb,
            "i_req_valid={};i_write={};i_addr={addr};i_wdata=31;i_wstrb=15;i_rsp_ready={};#9.89;",
            valid as u8, write as u8, ready as u8
        )
        .unwrap();
        if cycle > 0 {
            writeln!(tb,"if(i_irq!==1'b{} || i_req_ready!==1'b{} || i_rsp_valid!==1'b{}) $fatal(1,\"UART IRQ state cycle={cycle}\");",(pending&enable!=0)as u8,req_ready as u8,response.is_some()as u8).unwrap();
            if let Some(data) = response {
                writeln!(tb,"if(i_error!==0 || i_rdata!==32'h{data:x}) $fatal(1,\"UART IRQ snapshot cycle={cycle}\");").unwrap();
            }
        }
        if f.i.rst {
            pending = 0;
            enable = 0;
            response = None;
            local_sticky = 0;
            awaiting_next = false;
        } else {
            if awaiting_next && raw != 0 {
                subsequent_event += 1;
                awaiting_next = false;
            }
            if critical {
                assert!(req_ready, "reserve idle slot for dual clear");
                dual_clear += 1;
            }
            if valid && req_ready {
                response = Some(if write {
                    0
                } else if addr == 12 {
                    raw
                } else {
                    pending
                });
                if write && addr == 4 {
                    enable = 31;
                }
                if write && addr == 0 {
                    pending = 0;
                    if raw != 0 {
                        clears_with_event += 1
                    } else {
                        quiet_clears += 1;
                        if local_sticky != 0 {
                            sticky_quiet_clear += 1;
                            awaiting_next = true;
                        }
                    }
                }
            } else if response.is_some() && ready {
                response = None;
            }
            pending |= raw;
            if f.i.valid && f.i.write && f.i.addr == 20 && f.before.get("req_ready") == Some(&1) {
                local_sticky &= !(f.i.data & byte_mask(f.i.strb) & 15);
            }
            local_sticky |= f.before.get("raw_events").copied().unwrap_or(0) as u32;
            assert_eq!(pending & 17, 0, "IRQ0/4 unconnected");
        }
        writeln!(
            tb,
            "#0.11;if(i_irq!==1'b{}) $fatal(1,\"UART IRQ set-priority cycle={cycle}\");",
            (pending & enable != 0) as u8
        )
        .unwrap();
    }
    assert!(
        clears_with_event > 0
            && quiet_clears > 0
            && dual_clear > 0
            && sticky_quiet_clear > 0
            && subsequent_event > 0
    );
    println!(
        "UART IRQ reach dual_clear={dual_clear} sticky_quiet_clear={sticky_quiet_clear} subsequent_event={subsequent_event} clears_with_event={clears_with_event}"
    );
    writeln!(tb,"done_irq=1;end initial begin wait(done0&&done1&&done_irq);$display(\"FR197 UART PASS\");$finish;end endmodule").unwrap();
    tb
}
fn composition(backends: bool) {
    for renamed in [false, true] {
        let hir = pair_hir(renamed);
        let primary = trace(0x1285_1970_5511);
        let mut oracle = Oracle::default();
        let mut second = Vec::new();
        for (n, f) in primary.frames.iter().enumerate() {
            let i = Input {
                rst: f.i.rst,
                valid: n % 4 != 3,
                write: n % 31 != 0,
                addr: match n % 31 {
                    0 => 16,
                    1 => 0,
                    2 => 4,
                    _ => 12,
                },
                data: match n % 31 {
                    1 => 1,
                    2 => 5,
                    _ => (n * 73) as u32,
                },
                strb: 15,
                ready: n % 7 != 0,
                rx: n % 91 > 33,
                phase: f.i.phase,
            };
            second.push(oracle.frame(i));
        }
        let tb = pair_tb(&hir, &primary.frames, &second);
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/fr197-uart")
            .join(format!(
                "pair-{}-{}-{}",
                if backends { "backends" } else { "direct" },
                renamed,
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("tb.sv"), &tb).unwrap();
        if backends {
            vector_backends(&hir, "UartPair", &tb, &dir);
        } else {
            fs::write(
                dir.join("design.v"),
                bitloom_vlog::emit(&hir)
                    .files
                    .into_iter()
                    .map(|f| f.contents)
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
            .unwrap();
            backend_run(
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
                backend_run(&dir, "vvp", &["simulation"], "simulate").contains("FR197 UART PASS")
            );
        }
        println!(
            "UART pair renamed={renamed} actual UART/IRQ PASS {}",
            dir.display()
        );
    }
}
#[test]
fn p0_uart_shared_renamed_pair_and_actual_irq() {
    composition(false);
}
#[test]
#[ignore = "dedicated actual composed FIRRTL and Chisel UART/IRQ simulation"]
fn p1_uart_pair_irq_firrtl_chisel() {
    composition(true);
}

fn loopback(backends: bool) {
    let hir = UartCsr::elaborate().unwrap();
    let mut t = Trace {
        loopback: true,
        ..Trace::default()
    };
    for div in [3, 4, 5, 8] {
        t.reset();
        t.wr(4, div, 15);
        for byte in [0, 255, 85, 166] {
            t.wr(12, byte, 1);
        }
        t.wr(0, 1, 1);
        for _ in 0..((div + 1) * 45) {
            let level = t.m.tx_level();
            t.pin = level;
            t.step(Input::default());
        }
        assert_eq!(
            t.m.rxq.iter().copied().collect::<Vec<_>>(),
            [0, 255, 85, 166]
        );
        for _ in 0..4 {
            t.rd(16);
        }
    }
    // Use the actual DUT output wire, not the generated pin sequence.
    let mut tb = testbench(&hir, &t.frames).replace(",rx=1;", ";wire rx=tx;");
    for value in [0, 1] {
        tb = tb.replace(&format!("rx={value};"), "");
    }
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-uart")
        .join(format!(
            "loopback-{}-{}",
            backends,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("tb.sv"), &tb).unwrap();
    if backends {
        vector_backends(&hir, "UartCsr", &tb, &dir);
    } else {
        fs::write(
            dir.join("design.v"),
            bitloom_vlog::emit(&hir)
                .files
                .into_iter()
                .map(|f| f.contents)
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .unwrap();
        backend_run(
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
        assert!(backend_run(&dir, "vvp", &["simulation"], "simulate").contains("FR197 UART PASS"));
    }
    println!(
        "UART RTL wire loopback four dividers/sixteen frames PASS {}",
        dir.display()
    );
}
#[test]
fn p0_uart_actual_loopback() {
    loopback(false);
}
#[test]
#[ignore = "dedicated actual FIRRTL/Chisel wire loopback"]
fn p1_uart_actual_loopback_backends() {
    loopback(true);
}

#[test]
fn p0_uart_full_width_near_endpoint_state_injection() {
    let hir = UartCsr::elaborate().unwrap();
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-uart")
        .join(format!(
            "wide-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&dir).unwrap();
    fs::write(
        dir.join("design.v"),
        bitloom_vlog::emit(&hir)
            .files
            .into_iter()
            .map(|f| f.contents)
            .collect::<Vec<_>>()
            .join("\n"),
    )
    .unwrap();
    let mut tb = String::from(
        r#"`timescale 1ns/1ps
module tb;
reg clk=0,rst=1,req_valid=0,write=0,rsp_ready=0,rx=1;
reg[15:0] addr=0;reg[31:0] wdata=0;reg[3:0] wstrb=15;
wire req_ready,rsp_valid,tx;wire[31:0]rdata;wire[1:0]error;wire[3:0]raw_events;
UartCsr dut(.*);
task tick;begin #5;clk=1;#0.01;clk=0;#4.99;end endtask
task wr(input[15:0]a,input[31:0]v);begin req_valid=1;write=1;addr=a;wdata=v;tick();if(!rsp_valid||error!=0)$fatal(1,"wide CSR");req_valid=0;rsp_ready=1;tick();rsp_ready=0;end endtask
initial begin $dumpfile("trace.vcd");$dumpvars(0,tb);
"#,
    );
    for div in [0x80000000u32, 0xfffffffe, 0xffffffff] {
        let half = (u64::from(div) + 1) / 2;
        writeln!(tb,r#"rst=1;tick();rst=0;rx=1;wr(4,32'h{div:x});wr(12,1);wr(0,1);
if(dut.tx_div!==32'h{div:x}||dut.tx_timer!==32'h{:x}||tx!==0)$fatal(1,"wide launch/latch");
dut.tx_timer=1;tick();if(dut.tx_timer!==0||tx!==0)$fatal(1,"wide decrement");
tick();if(dut.tx_timer!==32'h{div:x}||tx!==1)$fatal(1,"wide reload/no overflow");
rst=1;tick();rst=0;rx=1;wr(4,32'h{div:x});wr(0,1);tick();rx=0;tick();tick();tick();
if(dut.rx_state!==1||dut.rx_div!==32'h{div:x}||dut.rx_timer!==32'h{:x})$fatal(1,"wide half initial");
dut.rx_timer=1;tick();if(dut.rx_timer!==0||dut.rx_state!==1)$fatal(1,"wide RX decrement");
tick();if(dut.rx_timer!==32'h{div:x}||dut.rx_state!==2)$fatal(1,"wide RX reload");"#,div-1,half-1).unwrap();
    }
    tb.push_str("$display(\"FR197 UART PASS\");$finish;end endmodule\n");
    fs::write(dir.join("tb.sv"), tb).unwrap();
    backend_run(
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
    assert!(backend_run(&dir, "vvp", &["simulation"], "simulate").contains("FR197 UART PASS"));
    println!(
        "UART full-width near-endpoint injected-state (not long-run simulation) PASS {}",
        dir.display()
    );
}

#[test]
fn p0_uart_behavior_mutations_serial_fifo_event() {
    let hir = UartCsr::elaborate().unwrap();
    let t = trace(0x1285_1970_5511);
    let tb = testbench(&hir, &t.frames);
    let source = bitloom_vlog::emit(&hir)
        .files
        .into_iter()
        .map(|f| f.contents)
        .collect::<Vec<_>>()
        .join("\n");
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-uart")
        .join(format!(
            "behavior-mutations-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&dir).unwrap();
    let timer_line = source
        .lines()
        .find(|l| l.trim().starts_with("tx_timer <= _uart_"))
        .unwrap();
    let serial = source.replace(timer_line, &timer_line.replace(';', " + 32'd1;"));
    let fifo = source.replace(
        "assign txq_input_valid = tx_data_write_commit;",
        "assign txq_input_valid = 1'b0;",
    );
    let event_line = source
        .lines()
        .find(|l| l.trim().starts_with("assign raw_events ="))
        .unwrap();
    let event = source.replace(event_line, "  assign raw_events = 4'b0;");
    for (name, rtl, expected) in [
        ("control", source.clone(), "FR197 UART PASS"),
        ("serial-countdown", serial, "tx expected="),
        ("fifo-push", fifo, "after error expected=2 got=0"),
        ("raw-event", event, "raw_events expected="),
    ] {
        let out = dir.join(name);
        fs::create_dir_all(&out).unwrap();
        fs::write(out.join("design.v"), &rtl).unwrap();
        fs::write(out.join("tb.sv"), &tb).unwrap();
        if name != "control" {
            assert_ne!(rtl, source);
        }
        backend_run(
            &out,
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
        let start = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let output = Command::new("timeout")
            .args(["--kill-after=5s", "60s", "vvp", "simulation"])
            .current_dir(&out)
            .output()
            .unwrap();
        let log =
            String::from_utf8(output.stdout).unwrap() + &String::from_utf8(output.stderr).unwrap();
        fs::write(out.join("simulate.log"), &log).unwrap();
        fs::write(out.join("simulate.json"),serde_json::json!({"tool":"vvp","args":["simulation"],"exit_code":output.status.code(),"expected_assertion":expected,"start_utc_unix_ms":start,"end_utc_unix_ms":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()}).to_string()).unwrap();
        assert_eq!(
            output.status.code(),
            Some(if name == "control" { 0 } else { 1 }),
            "{name}: {log}"
        );
        assert!(
            log.contains(expected),
            "specified behavioral assertion for {name}: {log}"
        );
        let vcd = fs::read_to_string(out.join("trace.vcd")).unwrap();
        assert!(vcd.contains("$enddefinitions") && vcd.contains("#0"));
    }
    println!(
        "UART serial/FIFO/event actual DUT mutations specified failures PASS {}",
        dir.display()
    );
}

fn contract_edges_trace() -> Trace {
    let mut t = Trace::default();
    // Preserve unselected nonzero bytes, including the high divider bit.
    for strb in 0..16 {
        t.reset();
        t.wr(4, 0x12345678, 15);
        t.wr(4, 0xaabbccdd, strb);
        t.rd(4);
        assert_eq!(
            t.m.div,
            (0x12345678 & !byte_mask(strb)) | (0xaabbccdd & byte_mask(strb))
        );
        t.mark("nonzero_partial_div_merge");
    }
    t.reset();
    t.config(3);
    for (value, strb, expected, error) in [
        (0x80000000, 8, 0x80000003, 0),
        (0, 1, 0x80000000, 0),
        (0, 8, 0x80000000, 2),
        (3, 1, 0x80000003, 0),
        (0, 8, 3, 0),
        (2, 1, 3, 2),
    ] {
        t.step(Input {
            valid: true,
            write: true,
            addr: 4,
            data: value,
            strb,
            ..Input::default()
        });
        assert_eq!(t.m.response.unwrap().error, error);
        assert_eq!(t.m.div, expected);
        t.step(Input {
            ready: true,
            ..Input::default()
        });
        t.rd(4);
        t.mark("enabled_merged_div_legality");
    }
    // The second byte is resident when the old frame finishes. Configuration,
    // not a preceding bus access, coincides with that next idle launch edge.
    for (addr, data, strb, period) in [
        (0, 0, 1, 0),
        (4, 5, 15, 6),
        (4, 1, 15, 4),
        (4, 0, 0, 4),
        (0, 1, 1, 4),
    ] {
        t.reset();
        t.config(3);
        t.wr(12, 0xa6, 1);
        t.wr(12, 0x55, 1);
        while t.m.tx.is_some() {
            t.idle(1);
        }
        assert_eq!(t.m.txq.len(), 1);
        assert!(t.m.response.is_none());
        t.step(Input {
            valid: true,
            write: true,
            addr,
            data,
            strb,
            ..Input::default()
        });
        if period == 0 {
            assert!(t.m.tx.is_none());
            assert_eq!(t.m.txq.len(), 1);
        } else {
            assert_eq!(t.m.tx.unwrap().period, period);
        }
        t.step(Input {
            ready: true,
            ..Input::default()
        });
        if period == 0 {
            t.wr(0, 1, 1);
        }
        t.idle(100);
        assert!(t.m.tx.is_none() && t.m.txq.is_empty());
        t.inspect();
        t.mark("tx_idle_configuration_collision");
    }
    t.reset();
    t.config(3);
    t.wr(12, 0xa6, 1);
    t.wr(12, 0x55, 1);
    while t.m.tx.is_some() {
        t.idle(1);
    }
    let starts = t.m.tx_start;
    let pushes = t.m.tx_push;
    t.step(Input {
        valid: true,
        write: true,
        addr: 12,
        data: 0xff,
        strb: 1,
        ..Input::default()
    });
    assert_eq!(t.m.tx_start, starts + 1);
    assert_eq!(t.m.tx_push, pushes + 1);
    assert_eq!(t.m.tx.unwrap().byte, 0x55);
    assert_eq!(t.m.txq.front(), Some(&0xff));
    t.step(Input {
        ready: true,
        ..Input::default()
    });
    t.idle(100);
    t.mark("tx_nonfull_simultaneous_push_pop");
    // RX-only and simultaneous TX/RX activity independently reject changes.
    for dual in [false, true] {
        t.reset();
        t.config(3);
        if dual {
            t.wr(12, 0xa6, 1);
        }
        t.pin = false;
        t.idle(3);
        assert!(t.m.rx.is_some());
        assert_eq!(t.m.tx.is_some(), dual);
        for (addr, data, strb, error) in [
            (4, 3, 15, 2),
            (0, 0, 1, 2),
            (0, 1, 1, 0),
            (4, 0, 0, 0),
            (0, 0, 8, 0),
        ] {
            t.step(Input {
                valid: true,
                write: true,
                addr,
                data,
                strb,
                ..Input::default()
            });
            assert_eq!(t.m.response.unwrap().error, error);
            assert!(t.m.enable);
            assert_eq!(t.m.div, 3);
            t.step(Input {
                ready: true,
                ..Input::default()
            });
        }
        t.pin = true;
        t.idle(70);
        t.inspect();
        t.mark("rx_only_or_dual_busy_configuration");
    }
    // The final busy edge still rejects configuration, despite retiring a frame.
    t.reset();
    t.config(3);
    t.wr(12, 0xa6, 1);
    let tx = t.m.tx.unwrap();
    while t.m.cycle < tx.start + 10 * tx.period {
        t.idle(1);
    }
    assert!(t.m.tx.is_some());
    t.step(Input {
        valid: true,
        write: true,
        addr: 4,
        data: 5,
        strb: 15,
        ..Input::default()
    });
    assert_eq!(t.m.response.unwrap().error, 2);
    assert!(t.m.tx.is_none());
    assert_eq!(t.m.div, 3);
    t.step(Input {
        ready: true,
        ..Input::default()
    });
    t.rd(4);
    t.mark("final_busy_edge_reject");
    t.reset();
    t.config(3);
    t.serial(0x55, 3, true);
    let rx = t.m.rx.unwrap();
    assert_eq!(t.m.cycle, rx.edge + rx.period / 2 + 9 * rx.period);
    t.step(Input {
        valid: true,
        write: true,
        addr: 4,
        data: 5,
        strb: 15,
        ..Input::default()
    });
    assert_eq!(t.m.response.unwrap().error, 2);
    assert!(t.m.rx.is_none());
    assert_eq!(t.m.div, 3);
    t.step(Input {
        ready: true,
        ..Input::default()
    });
    t.pin = true;
    t.rd(4);
    t.mark("final_busy_edge_reject");
    t.wr(0, 0, 1);
    assert!(!t.m.enable);
    assert_eq!(t.m.rxq.front(), Some(&0x55));
    t.rd(16);
    assert!(t.m.rxq.is_empty());
    t.wr(12, 0xa6, 1);
    t.idle(20);
    assert!(t.m.tx.is_none());
    assert_eq!(t.m.txq.front(), Some(&0xa6));
    t.mark("disabled_queue_access");
    // EVENT same- and different-bit clears coincide with every raw event class.
    for (occupancy, good, event) in [(0, true, 1), (4, true, 4), (4, false, 8)] {
        for clear in [event, 2] {
            t.reset();
            t.config(3);
            t.wr(12, 0x55, 1);
            t.idle(50);
            for byte in 0..occupancy {
                t.serial(0x30 + byte, 3, true);
                t.pin = true;
                t.idle(6);
            }
            let pushes = t.m.rx_push;
            let old = t.m.events;
            t.serial(0xa6, 3, good);
            let r = t.m.rx.unwrap();
            let stop = r.edge + r.period / 2 + 9 * r.period;
            while t.m.cycle < stop {
                t.idle(1);
            }
            assert_eq!(t.m.cycle, stop);
            let i = Input {
                valid: true,
                write: true,
                addr: 20,
                data: clear,
                strb: 1,
                ..Input::default()
            };
            assert_eq!(t.m.raw(i), event);
            t.step(i);
            assert_eq!(t.m.events, (old & !clear) | event);
            assert_eq!(t.m.rx_push, pushes + usize::from(event == 1));
            t.step(Input {
                ready: true,
                ..Input::default()
            });
            t.pin = true;
            t.idle(5);
            t.rd(20);
            t.mark("each_event_same_other_clear");
        }
    }
    for clear in [2, 1] {
        t.reset();
        t.config(3);
        t.serial(0x33, 3, true);
        t.pin = true;
        t.idle(6);
        t.wr(12, 0xa6, 1);
        t.wr(12, 0x55, 1);
        while t.m.tx.is_some() {
            t.idle(1);
        }
        let old = t.m.events;
        let i = Input {
            valid: true,
            write: true,
            addr: 20,
            data: clear,
            strb: 1,
            ..Input::default()
        };
        assert_eq!(t.m.raw(i), 2);
        t.step(i);
        assert_eq!(t.m.events, (old & !clear) | 2);
        t.step(Input {
            ready: true,
            ..Input::default()
        });
        t.idle(50);
        t.rd(20);
        t.mark("each_event_same_other_clear");
    }
    // A pending write response stalls through actual bidirectional serial work;
    // changing inactive bus payloads cannot cause another software operation.
    t.reset();
    t.config(3);
    let pushes = t.m.tx_push;
    let starts = t.m.tx_start;
    t.step(Input {
        valid: true,
        write: true,
        addr: 12,
        data: 0xa6,
        strb: 1,
        ..Input::default()
    });
    t.serial(0x55, 3, true);
    t.pin = true;
    t.idle(8);
    for n in 0..32 {
        t.step(Input {
            write: true,
            addr: [0, 4, 12, 20][n % 4],
            data: (n * 197) as u32,
            strb: (n % 16) as u8,
            ..Input::default()
        });
    }
    assert_eq!(t.m.tx_push, pushes + 1);
    assert_eq!(t.m.tx_start, starts + 1);
    assert_eq!(t.m.rxq.front(), Some(&0x55));
    assert_eq!(t.m.response.unwrap().data, 0);
    t.step(Input {
        ready: true,
        ..Input::default()
    });
    t.rd(16);
    t.mark("stalled_response_serial_and_inactive_noise");
    // Reset before FSM entry: only sync1, then sync1+sync2, have propagated low.
    for stages in [1, 2] {
        t.reset();
        t.config(3);
        t.pin = false;
        t.idle(stages);
        assert!(!t.m.s1);
        assert_eq!(!t.m.s2, stages == 2);
        assert!(t.m.rx.is_none());
        t.step(Input {
            rst: true,
            ..Input::default()
        });
        t.idle(5);
        t.config(3);
        t.idle(8);
        assert!(t.m.rx.is_none());
        t.pin = true;
        t.idle(5);
        t.serial(0xa6, 3, true);
        t.pin = true;
        t.idle(5);
        assert_eq!(t.m.rxq.front(), Some(&0xa6));
        t.rd(16);
        t.mark("reset_partial_synchronizer");
    }
    t.reset();
    t.config(3);
    for byte in [0, 255, 85, 166] {
        t.serial(byte, 3, true);
        t.pin = true;
        t.idle(6);
    }
    t.wr(0, 0, 1);
    for byte in [1, 2, 3, 4] {
        t.wr(12, byte, 1);
    }
    assert_eq!(t.m.txq.len(), 4);
    assert_eq!(t.m.rxq.len(), 4);
    t.step(Input {
        valid: true,
        addr: 8,
        ..Input::default()
    });
    assert!(t.m.response.is_some());
    t.step(Input {
        rst: true,
        valid: true,
        write: true,
        addr: 0,
        data: 1,
        strb: 1,
        ..Input::default()
    });
    assert!(t.m.txq.is_empty() && t.m.rxq.is_empty() && t.m.response.is_none());
    t.idle(5);
    t.inspect();
    t.mark("reset_both_full_and_response");
    t.reset();
    t.config(3);
    let received = t.m.rx_push;
    t.serial(0x55, 3, true);
    let r = t.m.rx.unwrap();
    assert_eq!(t.m.cycle, r.edge + r.period / 2 + 9 * r.period);
    t.step(Input {
        rst: true,
        valid: true,
        write: true,
        addr: 12,
        data: 0xff,
        strb: 1,
        ..Input::default()
    });
    assert_eq!(t.m.rx_push, received);
    assert!(t.m.rxq.is_empty() && t.m.txq.is_empty());
    t.pin = true;
    t.idle(5);
    t.inspect();
    t.mark("reset_beats_stop_and_software");
    t.finish();
    assert_eq!(
        t.tags,
        BTreeMap::from([
            ("disabled_queue_access", 1),
            ("final_busy_edge_reject", 2),
            ("nonzero_partial_div_merge", 16),
            ("enabled_merged_div_legality", 6),
            ("tx_idle_configuration_collision", 5),
            ("tx_nonfull_simultaneous_push_pop", 1),
            ("rx_only_or_dual_busy_configuration", 2),
            ("each_event_same_other_clear", 8),
            ("stalled_response_serial_and_inactive_noise", 1),
            ("reset_partial_synchronizer", 2),
            ("reset_both_full_and_response", 1),
            ("reset_beats_stop_and_software", 1)
        ])
    );
    t
}
fn contract_edges(backends: bool) {
    let hir = UartCsr::elaborate().unwrap();
    let t = contract_edges_trace();
    let tb = testbench(&hir, &t.frames);
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/fr197-uart")
        .join(format!(
            "edges-{}-{}",
            backends,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("tb.sv"), &tb).unwrap();
    fs::write(
        dir.join("coverage.json"),
        serde_json::to_string_pretty(&t.tags).unwrap(),
    )
    .unwrap();
    if backends {
        vector_backends(&hir, "UartCsr", &tb, &dir);
    } else {
        fs::write(
            dir.join("design.v"),
            bitloom_vlog::emit(&hir)
                .files
                .into_iter()
                .map(|f| f.contents)
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .unwrap();
        backend_run(
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
        assert!(backend_run(&dir, "vvp", &["simulation"], "simulate").contains("FR197 UART PASS"));
    }
    println!(
        "UART explicit contract edge matrix PASS frames={} coverage={:?} {}",
        t.frames.len(),
        t.tags,
        dir.display()
    );
}
#[test]
fn p0_uart_explicit_contract_edges() {
    contract_edges(false);
}
#[test]
#[ignore = "dedicated actual contract-edge FIRRTL and Chisel UART simulation"]
fn p1_uart_contract_edges_backends() {
    contract_edges(true);
}
