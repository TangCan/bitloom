use std::collections::BTreeMap;
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
    peer_allowed_high:usize,
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
                                self.peer_allowed_high += usize::from(merged("counter") & 0x80000000 != 0 && i.counter & 1 == 0);
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
        assert!(m.peer_rejects > 0 && m.peer_partial_writes > 0 && m.peer_allowed_high>0);
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

fn main(){for seed in [0x1272_1960_5511,0xdead_beef_1272,0x7359_2401_ffff]{trace(seed,false);}trace(0x1272_0a11_ce55,true);}
