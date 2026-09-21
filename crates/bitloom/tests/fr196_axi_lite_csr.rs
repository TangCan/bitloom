//! Story127.3 bridge acceptance: native engines and emitted RTL.
//! The reference tracks public handshake events, not implementation state/latency.
use bitloom_hir::{PortDirection, PortValues};
use bitloom_prelude::{Elaboratable, FrozenHir, GroundType, ip::AxiLiteCsrBridge};
use bitloom_sim::{Sim, TickEngine};
use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Write as _,
    fs,
    io::{self, Write as _},
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    time::{Duration, Instant},
};
#[path = "fr196_axi_lite_csr/tools.rs"]
mod tools;
type Values = BTreeMap<String, u64>;
const INPUTS: &[&str] = &[
    "clk",
    "rst",
    "s_axi_awaddr",
    "s_axi_awprot",
    "s_axi_awvalid",
    "s_axi_wdata",
    "s_axi_wstrb",
    "s_axi_wvalid",
    "s_axi_bready",
    "s_axi_araddr",
    "s_axi_arprot",
    "s_axi_arvalid",
    "s_axi_rready",
    "csr_req_ready",
    "csr_rsp_valid",
    "csr_rdata",
    "csr_error",
];
const OUTPUTS: &[&str] = &[
    "s_axi_awready",
    "s_axi_wready",
    "s_axi_bvalid",
    "s_axi_bresp",
    "s_axi_arready",
    "s_axi_rvalid",
    "s_axi_rdata",
    "s_axi_rresp",
    "csr_req_valid",
    "csr_write",
    "csr_addr",
    "csr_wdata",
    "csr_wstrb",
    "csr_rsp_ready",
];
fn inputs() -> Values {
    INPUTS.iter().map(|n| (n.to_string(), 0)).collect()
}
fn set(v: &mut Values, n: &str, x: u64) {
    v.insert(n.into(), x);
}
fn bit(v: &Values, n: &str) -> bool {
    v[n] != 0
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Req {
    write: bool,
    addr: u64,
    data: u64,
    strb: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rsp {
    error: u64,
    data: u64,
}
fn reply(r: Req) -> Rsp {
    let error = match r.addr % 3 {
        0 => 0,
        1 => 2,
        _ => 3,
    };
    Rsp {
        error,
        data: if r.write || error != 0 {
            0
        } else {
            (r.addr ^ 0xcafe_5231) & 0xffff_ffff
        },
    }
}
#[derive(Default, Debug)]
struct Ledger {
    aw: u64,
    w: u64,
    ar: u64,
    complete_write: u64,
    submit: u64,
    consume: u64,
    b: u64,
    r: u64,
    cancel_aw: u64,
    cancel_w: u64,
    cancel_ar: u64,
    cancel_owner: u64,
    cancel_b: u64,
    cancel_r: u64,
    epochs: u64,
}
#[derive(Default)]
struct Monitor {
    held: BTreeMap<String, Vec<u64>>,
}
impl Monitor {
    fn channel(
        &mut self,
        name: &str,
        reset: bool,
        valid: bool,
        ready: bool,
        payload: Vec<u64>,
    ) -> Result<(), String> {
        if reset {
            self.held.remove(name);
            return Ok(());
        }
        if let Some(old) = self.held.get(name) {
            if !valid || *old != payload {
                return Err(format!("{name} withdrew valid or changed stalled payload"));
            }
        }
        if valid && !ready {
            self.held.insert(name.into(), payload);
        } else {
            self.held.remove(name);
        }
        Ok(())
    }
}
#[derive(Default)]
struct Score {
    aw: VecDeque<u64>,
    w: VecDeque<(u64, u64)>,
    ar: VecDeque<u64>,
    owner: Option<Req>,
    b: VecDeque<Rsp>,
    r: VecDeque<Rsp>,
    monitor: Monitor,
    l: Ledger,
    order: Vec<Req>,
}
impl Score {
    fn edge(&mut self, i: &Values, o: &Values) {
        let rst = bit(i, "rst");
        for (name, valid, ready, payload) in [
            (
                "AW",
                bit(i, "s_axi_awvalid"),
                bit(o, "s_axi_awready"),
                vec![i["s_axi_awaddr"], i["s_axi_awprot"]],
            ),
            (
                "W",
                bit(i, "s_axi_wvalid"),
                bit(o, "s_axi_wready"),
                vec![i["s_axi_wdata"], i["s_axi_wstrb"]],
            ),
            (
                "AR",
                bit(i, "s_axi_arvalid"),
                bit(o, "s_axi_arready"),
                vec![i["s_axi_araddr"], i["s_axi_arprot"]],
            ),
            (
                "peer",
                bit(i, "csr_rsp_valid"),
                bit(o, "csr_rsp_ready"),
                vec![i["csr_rdata"], i["csr_error"]],
            ),
            (
                "offer",
                bit(o, "csr_req_valid"),
                bit(i, "csr_req_ready"),
                vec![
                    o["csr_write"],
                    o["csr_addr"],
                    o["csr_wdata"],
                    o["csr_wstrb"],
                ],
            ),
            (
                "B",
                bit(o, "s_axi_bvalid"),
                bit(i, "s_axi_bready"),
                vec![o["s_axi_bresp"]],
            ),
            (
                "R",
                bit(o, "s_axi_rvalid"),
                bit(i, "s_axi_rready"),
                vec![o["s_axi_rresp"], o["s_axi_rdata"]],
            ),
        ] {
            self.monitor
                .channel(name, rst, valid, ready, payload)
                .unwrap();
        }
        if rst {
            self.l.cancel_aw += self.aw.len() as u64;
            self.l.cancel_w += self.w.len() as u64;
            self.l.cancel_ar += self.ar.len() as u64;
            self.l.cancel_owner += u64::from(self.owner.take().is_some());
            self.l.cancel_b += self.b.len() as u64;
            self.l.cancel_r += self.r.len() as u64;
            self.aw.clear();
            self.w.clear();
            self.ar.clear();
            self.b.clear();
            self.r.clear();
            self.l.epochs += 1;
            return;
        }
        // AXI visible responses must already have a causally consumed CSR response.
        if bit(o, "s_axi_bvalid") {
            assert_eq!(
                o["s_axi_bresp"],
                self.b.front().expect("unsolicited B").error
            );
            if bit(i, "s_axi_bready") {
                self.b.pop_front();
                self.l.b += 1;
            }
        }
        if bit(o, "s_axi_rvalid") {
            let r = self.r.front().expect("unsolicited R");
            assert_eq!((o["s_axi_rresp"], o["s_axi_rdata"]), (r.error, r.data));
            if bit(i, "s_axi_rready") {
                self.r.pop_front();
                self.l.r += 1;
            }
        }
        assert!(
            !(bit(i, "csr_rsp_valid")
                && bit(o, "csr_rsp_ready")
                && bit(o, "csr_req_valid")
                && bit(i, "csr_req_ready")),
            "CSR response and new submission on the same edge"
        );
        if bit(i, "csr_rsp_valid") && bit(o, "csr_rsp_ready") {
            let req = self.owner.take().expect("CSR response without owner");
            let r = reply(req);
            assert_eq!((i["csr_error"], i["csr_rdata"]), (r.error, r.data));
            let q = if req.write { &mut self.b } else { &mut self.r };
            assert!(q.is_empty(), "overwrote response slot");
            q.push_back(r);
            self.l.consume += 1;
        }
        if bit(o, "csr_req_valid") && bit(i, "csr_req_ready") {
            assert!(self.owner.is_none(), "more than one CSR owner");
            let wr = bit(o, "csr_write");
            let expected = if wr {
                let addr = self.aw.pop_front().expect("write without AW");
                let (data, strb) = self.w.pop_front().expect("write without W");
                assert!(self.b.is_empty(), "B capacity not reserved");
                Req {
                    write: true,
                    addr,
                    data,
                    strb,
                }
            } else {
                assert!(self.r.is_empty(), "R capacity not reserved");
                Req {
                    write: false,
                    addr: self.ar.pop_front().expect("read without AR"),
                    data: 0,
                    strb: 0,
                }
            };
            assert_eq!(o["csr_addr"], expected.addr, "raw address changed");
            if wr {
                assert_eq!(
                    (o["csr_wdata"], o["csr_wstrb"]),
                    (expected.data, expected.strb)
                );
            }
            self.owner = Some(expected);
            self.order.push(expected);
            self.l.submit += 1;
        }
        let pairs_before = self.aw.len().min(self.w.len());
        if bit(i, "s_axi_awvalid") && bit(o, "s_axi_awready") {
            self.aw.push_back(i["s_axi_awaddr"]);
            self.l.aw += 1;
        }
        if bit(i, "s_axi_wvalid") && bit(o, "s_axi_wready") {
            self.w.push_back((i["s_axi_wdata"], i["s_axi_wstrb"]));
            self.l.w += 1;
        }
        if bit(i, "s_axi_arvalid") && bit(o, "s_axi_arready") {
            self.ar.push_back(i["s_axi_araddr"]);
            self.l.ar += 1;
        }
        self.l.complete_write += (self.aw.len().min(self.w.len()) - pairs_before) as u64;
        assert!(
            self.aw.len() <= 1 && self.w.len() <= 1 && self.ar.len() <= 1,
            "capture overflow"
        );
    }
    fn empty(&self) -> bool {
        self.aw.is_empty()
            && self.w.is_empty()
            && self.ar.is_empty()
            && self.owner.is_none()
            && self.b.is_empty()
            && self.r.is_empty()
    }
    fn account(&self) {
        let writes = self.order.iter().filter(|r| r.write).count() as u64;
        let reads = self.order.len() as u64 - writes;
        assert_eq!(self.l.aw, writes + self.l.cancel_aw + self.aw.len() as u64);
        assert_eq!(self.l.w, writes + self.l.cancel_w + self.w.len() as u64);
        assert_eq!(self.l.ar, reads + self.l.cancel_ar + self.ar.len() as u64);
        assert_eq!(
            self.l.submit,
            self.l.consume + self.l.cancel_owner + u64::from(self.owner.is_some())
        );
        assert_eq!(
            self.l.consume,
            self.l.b
                + self.l.r
                + self.l.cancel_b
                + self.l.cancel_r
                + self.b.len() as u64
                + self.r.len() as u64
        );
    }
}
struct Harness {
    sim: Sim,
    hir: FrozenHir,
    score: Score,
    frames: Vec<(Values, Values)>,
}
impl Harness {
    fn new(engine: TickEngine) -> Self {
        let hir = AxiLiteCsrBridge::elaborate().unwrap();
        let mut h = Self {
            sim: Sim::with_engine(hir.clone(), engine),
            hir,
            score: Score::default(),
            frames: vec![],
        };
        let mut i = inputs();
        set(&mut i, "rst", 1);
        h.step(i);
        h
    }
    fn step(&mut self, i: Values) -> Values {
        let mut p = PortValues::default();
        for (n, v) in &i {
            p.set(n, *v);
        }
        self.sim.set_inputs(p);
        self.sim.settle();
        let o: Values = OUTPUTS
            .iter()
            .map(|n| (n.to_string(), self.sim.ports().get(n).unwrap()))
            .collect();
        self.score.edge(&i, &o);
        self.sim.tick();
        self.frames.push((i, o.clone()));
        o
    }
    fn peer(&self, mut i: Values, ready: bool) -> Values {
        set(&mut i, "csr_req_ready", u64::from(ready));
        if let Some(req) = self.score.owner {
            let r = reply(req);
            set(&mut i, "csr_rsp_valid", 1);
            set(&mut i, "csr_rdata", r.data);
            set(&mut i, "csr_error", r.error);
        }
        i
    }
    fn idle(&mut self, n: usize, b: bool, r: bool) {
        for _ in 0..n {
            let mut i = inputs();
            set(&mut i, "s_axi_bready", b as u64);
            set(&mut i, "s_axi_rready", r as u64);
            let i = self.peer(i, true);
            self.step(i);
        }
    }
    fn drain(&mut self) {
        for _ in 0..128 {
            if self.score.empty() {
                // Keep sampling after the final response: a delayed duplicate or
                // replay must still reach the event oracle rather than escaping.
                self.idle(8, true, true);
                assert!(self.score.empty(), "traffic reappeared after drain");
                self.score.account();
                return;
            }
            self.idle(1, true, true);
        }
        panic!("bounded drain failed {:?}", self.score.l)
    }
    fn issue(
        &mut self,
        write: bool,
        addr: u64,
        data: u64,
        strb: u64,
        prot: u64,
        gap: usize,
        w_first: bool,
    ) {
        let (mut aw, mut w, mut ar) = (false, false, false);
        for t in 0..256 {
            let mut i = inputs();
            set(&mut i, "s_axi_bready", 1);
            set(&mut i, "s_axi_rready", 1);
            set(&mut i, "s_axi_awaddr", addr);
            set(&mut i, "s_axi_awprot", prot);
            set(&mut i, "s_axi_wdata", data);
            set(&mut i, "s_axi_wstrb", strb);
            set(&mut i, "s_axi_araddr", addr);
            set(&mut i, "s_axi_arprot", prot);
            set(
                &mut i,
                "s_axi_awvalid",
                (write && !aw && (!w_first || t >= gap)) as u64,
            );
            set(
                &mut i,
                "s_axi_wvalid",
                (write && !w && (w_first || t >= gap)) as u64,
            );
            set(&mut i, "s_axi_arvalid", (!write && !ar) as u64);
            let i = self.peer(i, t % 7 != 0);
            let o = self.step(i.clone());
            aw |= bit(&i, "s_axi_awvalid") && bit(&o, "s_axi_awready");
            w |= bit(&i, "s_axi_wvalid") && bit(&o, "s_axi_wready");
            ar |= bit(&i, "s_axi_arvalid") && bit(&o, "s_axi_arready");
            if (write && aw && w) || (!write && ar) {
                return;
            }
        }
        panic!("channel capture timeout")
    }
    fn rtl(&self, label: &str) {
        let module = self
            .hir
            .circuit()
            .modules
            .iter()
            .find(|m| m.name == "AxiLiteCsrBridge")
            .unwrap();
        let mut tb = String::from("module tb;\n");
        for p in &module.ports {
            let w = match p.ty {
                GroundType::UInt { width } => width,
                GroundType::Clock | GroundType::Reset => 1,
                _ => panic!(),
            };
            writeln!(
                tb,
                "{} [{}:0] {}{};",
                if p.direction == PortDirection::Input {
                    "reg"
                } else {
                    "wire"
                },
                w - 1,
                p.name,
                if p.direction == PortDirection::Input {
                    "=0"
                } else {
                    ""
                }
            )
            .unwrap();
        }
        tb.push_str("AxiLiteCsrBridge dut(.*);initial begin\n");
        for (cycle, (i, o)) in self.frames.iter().enumerate() {
            for (n, v) in i {
                writeln!(tb, "{n}=64'h{v:x};").unwrap();
            }
            tb.push_str("#1;\n");
            if cycle > 0 {
                for (n, v) in o {
                    writeln!(
                        tb,
                        "if ({n} !== 64'h{v:x}) $fatal(1,\"cycle {cycle} {n}\");"
                    )
                    .unwrap();
                }
            }
            tb.push_str("clk=1;#1;clk=0;#1;\n");
        }
        tb.push_str("$display(\"FR196 PASS\");$finish;end endmodule\n");
        tools::run_rtl(label, &self.hir, &tb);
    }
}
#[test]
fn p0_raw_order_gaps_strobes_prot_errors_native_and_rtl() {
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        let mut h = Harness::new(engine);
        for gap in [0, 1, 7, 31] {
            for first in [false, true] {
                for prot in 0..8 {
                    for strb in 0..16 {
                        let addr = [0, 1, 2, 3, 4, 8, 12, 20, 0x8000, 0xfffc, 0xffff]
                            [(strb as usize + prot as usize) % 11];
                        h.issue(true, addr, 0xa550_00ff ^ strb, strb, prot, gap, first);
                        h.issue(false, addr, 0, 0, prot, 0, false);
                        h.drain();
                    }
                }
            }
        }
        assert_eq!(h.score.l.submit, 2048);
        assert_eq!(h.score.l.b, 1024);
        assert_eq!(h.score.l.r, 1024);
        h.rtl(&format!("matrix-{engine:?}"));
    }
}
#[test]
fn p0_offer_lock_round_robin_and_independent_response_slots() {
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        let mut h = Harness::new(engine);
        // Make all three captures simultaneously eligible before permitting CSR.
        let mut i = inputs();
        for (n, v) in [
            ("s_axi_awvalid", 1),
            ("s_axi_wvalid", 1),
            ("s_axi_arvalid", 1),
            ("s_axi_awaddr", 9),
            ("s_axi_araddr", 6),
            ("s_axi_wdata", 0x12345678),
            ("s_axi_wstrb", 15),
        ] {
            set(&mut i, n, v);
        }
        let o = h.step(i);
        assert!(bit(&o, "s_axi_awready") && bit(&o, "s_axi_wready") && bit(&o, "s_axi_arready"));
        for _ in 0..40 {
            h.step(inputs());
        }
        assert!(h.score.order.is_empty());
        h.idle(64, false, false);
        assert_eq!(
            h.score.order.iter().map(|r| r.write).collect::<Vec<_>>(),
            vec![false, true]
        );
        assert_eq!((h.score.blen(), h.score.rlen()), (1, 1));
        // No free response slots: captured traffic cannot execute until its own slot drains.
        h.issue_blocked_pair();
        let count = h.score.l.submit;
        h.idle(32, false, false);
        assert_eq!(h.score.l.submit, count);
        let initial_reads = h.score.l.r;
        for _ in 0..64 {
            h.idle(1, false, true);
            if h.score.l.r > initial_reads {
                break;
            }
        }
        assert_eq!(h.score.l.r, initial_reads + 1);
        h.idle(64, false, false);
        let held = &h.frames.last().unwrap().1;
        assert_eq!((held["s_axi_bvalid"], held["s_axi_rvalid"]), (1, 1));
        assert_eq!(h.score.l.submit, count + 1);
        assert!(!h.score.order.last().unwrap().write);
        h.idle(64, true, false);
        assert_eq!(h.frames.last().unwrap().1["s_axi_rvalid"], 1);
        assert_eq!(h.score.l.submit, count + 2);
        assert!(h.score.order.last().unwrap().write);
        h.drain();
        h.rtl(&format!("arbitration-{engine:?}"));
    }
}
impl Score {
    fn blen(&self) -> usize {
        self.b.len()
    }
    fn rlen(&self) -> usize {
        self.r.len()
    }
}
impl Harness {
    fn issue_blocked_pair(&mut self) {
        let (mut aw, mut w, mut ar) = (false, false, false);
        for _ in 0..64 {
            let mut i = inputs();
            for (n, v) in [
                ("s_axi_awvalid", (!aw) as u64),
                ("s_axi_wvalid", (!w) as u64),
                ("s_axi_arvalid", (!ar) as u64),
                ("s_axi_awaddr", 12),
                ("s_axi_araddr", 15),
                ("s_axi_wdata", 0x55667788),
                ("s_axi_wstrb", 3),
            ] {
                set(&mut i, n, v);
            }
            let i = self.peer(i, true);
            let o = self.step(i);
            aw |= bit(&o, "s_axi_awready");
            w |= bit(&o, "s_axi_wready");
            ar |= bit(&o, "s_axi_arready");
            if aw && w && ar {
                return;
            }
        }
        panic!("capture blocked pair timeout")
    }
}
#[test]
fn p0_reset_cancels_partial_offer_execution_responses_and_recovers() {
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        let mut h = Harness::new(engine);
        for stage in 0..8 {
            let mut i = inputs();
            if stage != 1 {
                set(&mut i, "s_axi_awvalid", 1);
                set(&mut i, "s_axi_awaddr", 6);
            }
            if stage != 0 {
                set(&mut i, "s_axi_wvalid", 1);
                set(&mut i, "s_axi_wstrb", 15);
            }
            if stage >= 2 {
                set(&mut i, "s_axi_arvalid", 1);
                set(&mut i, "s_axi_araddr", 9);
            }
            h.step(i);
            if stage >= 3 {
                for _ in 0..8 {
                    h.step(inputs());
                }
            }
            if stage >= 4 {
                for _ in 0..16 {
                    let mut i = inputs();
                    set(&mut i, "csr_req_ready", 1);
                    h.step(i);
                    if h.score.owner.is_some() {
                        break;
                    }
                }
                assert!(h.score.owner.is_some());
            }
            if stage >= 5 {
                h.idle(32, false, false);
                assert!(!h.score.b.is_empty() && !h.score.r.is_empty());
            }
            if stage >= 6 {
                h.issue_blocked_pair();
            }
            if stage == 6 {
                assert_eq!(
                    (
                        h.score.aw.len(),
                        h.score.w.len(),
                        h.score.ar.len(),
                        h.score.b.len(),
                        h.score.r.len()
                    ),
                    (1, 1, 1, 1, 1)
                );
                assert!(h.score.owner.is_none());
                println!("reset stage6 {engine:?}: AW/W/AR/B/R full, execution empty");
            }
            if stage == 7 {
                h.idle(1, true, false); // Free B only, retain the blocked R.
                for _ in 0..16 {
                    let mut i = inputs();
                    set(&mut i, "csr_req_ready", 1);
                    h.step(i);
                    if h.score.owner.is_some() {
                        break;
                    }
                }
                assert!(h.score.owner.unwrap().write);
                let mut i = inputs();
                set(&mut i, "s_axi_awvalid", 1);
                set(&mut i, "s_axi_awaddr", 21);
                assert!(bit(&h.step(i), "s_axi_awready"));
                assert_eq!(
                    (
                        h.score.aw.len(),
                        h.score.w.len(),
                        h.score.ar.len(),
                        h.score.b.len(),
                        h.score.r.len()
                    ),
                    (1, 0, 1, 0, 1)
                );
                println!(
                    "reset stage7 {engine:?}: partial AW + AR + blocked R + executing write, W/B empty"
                );
            }
            let before = (
                h.score.l.aw,
                h.score.l.w,
                h.score.l.ar,
                h.score.l.submit,
                h.score.l.consume,
                h.score.l.b,
                h.score.l.r,
            );
            // All producer valids and consumer readies compete with reset.
            // A response is causal only when an old execution exists.
            let mut i = h.peer(inputs(), true);
            for n in [
                "rst",
                "s_axi_awvalid",
                "s_axi_wvalid",
                "s_axi_arvalid",
                "s_axi_bready",
                "s_axi_rready",
            ] {
                set(&mut i, n, 1);
            }
            h.step(i);
            assert_eq!(
                (
                    h.score.l.aw,
                    h.score.l.w,
                    h.score.l.ar,
                    h.score.l.submit,
                    h.score.l.consume,
                    h.score.l.b,
                    h.score.l.r
                ),
                before,
                "reset priority stage {stage}"
            );
            assert!(h.score.empty());
            h.idle(8, true, true);
            h.issue(true, 3, 0x1234, 15, 7, 1, false);
            h.issue(false, 6, 0, 0, 7, 0, false);
            h.drain();
        }
        assert!(
            h.score.l.cancel_aw > 0
                && h.score.l.cancel_w > 0
                && h.score.l.cancel_ar > 0
                && h.score.l.cancel_owner > 0
                && h.score.l.cancel_b > 0
                && h.score.l.cancel_r > 0
        );
        h.score.account();
        h.rtl(&format!("reset-{engine:?}"));
    }
}
fn random(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}
#[test]
fn p1_sixteen_reproducible_seeds_each_complete_one_thousand_transactions() {
    let start = Instant::now();
    for seed in 1..=16 {
        let seed_start = Instant::now();
        let mut rng = seed;
        let mut h = Harness::new(TickEngine::Compiled);
        let mut gaps = [0u64; 4];
        let mut strobes = [0u64; 16];
        let mut errors = [0u64; 4];
        let (mut aw_first, mut w_first, mut b_stalls, mut r_stalls) = (0, 0, 0, 0);
        for n in 0..1000 {
            let r = random(&mut rng);
            let write = r & 1 != 0;
            let gi = ((r >> 1) & 3) as usize;
            let strb = (r >> 3) & 15;
            let addr = [0, 1, 2, 3, 8, 0x8000, 0xfffc, 0xffff][((r >> 7) & 7) as usize];
            if write {
                gaps[gi] += 1;
                strobes[strb as usize] += 1;
            }
            let first_frame = h.frames.len();
            h.issue(
                write,
                addr,
                random(&mut rng) & 0xffff_ffff,
                strb,
                (r >> 10) & 7,
                [0, 1, 7, 31][gi],
                r & 0x2000 != 0,
            );
            h.idle((r % 33) as usize, false, false);
            h.drain();
            let frames = &h.frames[first_frame..];
            if write {
                let accepted = |valid: &str, ready: &str| {
                    frames
                        .iter()
                        .position(|(i, o)| bit(i, valid) && bit(o, ready))
                        .unwrap()
                };
                let a = accepted("s_axi_awvalid", "s_axi_awready");
                let w = accepted("s_axi_wvalid", "s_axi_wready");
                aw_first += usize::from(a < w);
                w_first += usize::from(w < a);
            }
            b_stalls += frames
                .iter()
                .filter(|(i, o)| bit(o, "s_axi_bvalid") && !bit(i, "s_axi_bready"))
                .count();
            r_stalls += frames
                .iter()
                .filter(|(i, o)| bit(o, "s_axi_rvalid") && !bit(i, "s_axi_rready"))
                .count();
            errors[reply(Req {
                write,
                addr,
                data: 0,
                strb,
            })
            .error as usize] += 1;
            assert_eq!(h.score.l.b + h.score.l.r, n + 1);
        }
        assert_eq!(h.score.l.submit, 1000);
        assert_eq!(gaps.iter().sum::<u64>(), h.score.l.complete_write);
        assert_eq!(strobes.iter().sum::<u64>(), h.score.l.complete_write);
        assert!(aw_first > 0 && w_first > 0 && b_stalls > 0 && r_stalls > 0);
        println!(
            "seed={seed} observed_aw_first={aw_first} observed_w_first={w_first} B_stall_cycles={b_stalls} R_stall_cycles={r_stalls}"
        );
        assert!(gaps.iter().all(|n| *n > 0) && strobes.iter().all(|n| *n > 0));
        assert!(errors[0] > 0 && errors[2] > 0 && errors[3] > 0);
        h.rtl(&format!("seed-{seed}"));
        println!(
            "seed={seed} completed=1000 ledger={:?} gaps={gaps:?} strobes={strobes:?} errors={errors:?} seed_elapsed={:?} cumulative_elapsed={:?}",
            h.score.l,
            seed_start.elapsed(),
            start.elapsed()
        );
    }
}
#[test]
fn p0_protocol_monitor_rejects_withdrawal_mutation_and_accepts_reset() {
    for name in ["AW", "W", "AR", "peer", "offer", "B", "R"] {
        for (change, ready) in [(false, false), (true, false), (false, true), (true, true)] {
            let mut m = Monitor::default();
            m.channel(name, false, true, false, vec![1, 2, 3]).unwrap();
            assert!(
                m.channel(
                    name,
                    false,
                    change,
                    ready,
                    if change { vec![1, 2, 4] } else { vec![1, 2, 3] }
                )
                .is_err()
            );
        }
        let mut accepted = Monitor::default();
        accepted
            .channel(name, false, true, false, vec![1, 2, 3])
            .unwrap();
        accepted
            .channel(name, false, true, true, vec![1, 2, 3])
            .unwrap();
        accepted.channel(name, false, false, false, vec![]).unwrap();
        let mut m = Monitor::default();
        m.channel(name, false, true, false, vec![1]).unwrap();
        assert!(m.channel(name, true, false, false, vec![2]).is_ok());
    }
}
#[path = "fr196_axi_lite_csr/integration.rs"]
mod integration;
#[test]
fn p0_late_read_cannot_steal_locked_write_and_partial_write_never_blocks_read() {
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        let mut h = Harness::new(engine);
        let mut i = inputs();
        for (n, v) in [
            ("s_axi_awvalid", 1),
            ("s_axi_wvalid", 1),
            ("s_axi_awaddr", 6),
            ("s_axi_wdata", 0xabcde),
            ("s_axi_wstrb", 15),
        ] {
            set(&mut i, n, v);
        }
        let o = h.step(i);
        assert!(bit(&o, "s_axi_awready") && bit(&o, "s_axi_wready"));
        let mut offered = false;
        for _ in 0..64 {
            let o = h.step(inputs());
            if bit(&o, "csr_req_valid") {
                assert_eq!(o["csr_write"], 1);
                offered = true;
                break;
            }
        }
        assert!(offered);
        let mut i = inputs();
        set(&mut i, "s_axi_arvalid", 1);
        set(&mut i, "s_axi_araddr", 9);
        let o = h.step(i);
        assert!(bit(&o, "s_axi_arready"));
        for _ in 0..31 {
            let o = h.step(inputs());
            assert_eq!(
                (o["csr_req_valid"], o["csr_write"], o["csr_addr"]),
                (1, 1, 6)
            );
        }
        h.drain();
        assert_eq!(
            h.score.order.iter().map(|r| r.write).collect::<Vec<_>>(),
            vec![true, false]
        );
        for only_aw in [true, false] {
            let mut i = inputs();
            set(&mut i, "rst", 1);
            h.step(i);
            let mut i = inputs();
            set(
                &mut i,
                if only_aw {
                    "s_axi_awvalid"
                } else {
                    "s_axi_wvalid"
                },
                1,
            );
            set(&mut i, "s_axi_awaddr", 12);
            set(&mut i, "s_axi_wdata", 0x55);
            set(&mut i, "s_axi_wstrb", 15);
            h.step(i);
            let before = h.score.l.r;
            h.issue(false, 9, 0, 0, 0, 0, false);
            h.idle(64, true, true);
            assert_eq!(h.score.l.r, before + 1, "partial write blocked read");
            assert!(h.score.owner.is_none());
            let mut i = inputs();
            set(
                &mut i,
                if only_aw {
                    "s_axi_wvalid"
                } else {
                    "s_axi_awvalid"
                },
                1,
            );
            set(&mut i, "s_axi_awaddr", 12);
            set(&mut i, "s_axi_wdata", 0x55);
            set(&mut i, "s_axi_wstrb", 15);
            h.step(i);
            h.drain();
        }
        h.score.account();
        h.rtl(&format!("late-read-partials-{engine:?}"));
    }
}

#[test]
fn p0_concurrent_independent_producers_with_bounded_backpressure() {
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        let mut h = Harness::new(engine);
        let (mut aw, mut w, mut ar) = (0u64, 0u64, 0u64);
        let (mut av, mut wv, mut rv) = (false, false, false);
        let (mut overlapping, mut stalled) = (0, [0; 6]);
        let mut rng = 0x1273_c0ffee;
        for cycle in 0..20000 {
            let noise = random(&mut rng);
            av |= aw < 256 && noise & 3 != 0;
            wv |= w < 256 && noise & 12 != 0;
            rv |= ar < 256 && noise & 48 != 0;
            let mut i = inputs();
            for (n, v) in [
                ("s_axi_awvalid", av as u64),
                ("s_axi_awaddr", aw * 7),
                ("s_axi_wvalid", wv as u64),
                ("s_axi_wdata", ((w * 0x1234567) ^ 0xa55a1234) & 0xffff_ffff),
                ("s_axi_wstrb", w % 16),
                ("s_axi_arvalid", rv as u64),
                ("s_axi_araddr", ar * 11),
                ("s_axi_bready", (cycle % 37 >= 23) as u64),
                ("s_axi_rready", (cycle % 41 >= 27) as u64),
            ] {
                set(&mut i, n, v);
            }
            let i = h.peer(i, cycle % 13 >= 5);
            let o = h.step(i.clone());
            for (lane, (valid, ready, input_valid)) in [
                ("s_axi_awvalid", "s_axi_awready", true),
                ("s_axi_wvalid", "s_axi_wready", true),
                ("s_axi_arvalid", "s_axi_arready", true),
                ("s_axi_bvalid", "s_axi_bready", false),
                ("s_axi_rvalid", "s_axi_rready", false),
                ("csr_req_valid", "csr_req_ready", false),
            ]
            .into_iter()
            .enumerate()
            {
                let (v, r) = if input_valid {
                    (bit(&i, valid), bit(&o, ready))
                } else {
                    (bit(&o, valid), bit(&i, ready))
                };
                if v && !r {
                    stalled[lane] += 1;
                }
            }
            if av && bit(&o, "s_axi_awready") {
                aw += 1;
                av = false;
            }
            if wv && bit(&o, "s_axi_wready") {
                w += 1;
                wv = false;
            }
            if rv && bit(&o, "s_axi_arready") {
                ar += 1;
                rv = false;
            }
            if (!h.score.aw.is_empty() || !h.score.w.is_empty()) && !h.score.ar.is_empty() {
                overlapping += 1;
            }
            if aw == 256 && w == 256 && ar == 256 && h.score.empty() {
                break;
            }
        }
        assert_eq!((aw, w, ar), (256, 256, 256));
        assert!(h.score.empty());
        h.score.account();
        assert_eq!(
            (
                h.score.l.submit,
                h.score.l.consume,
                h.score.l.b,
                h.score.l.r
            ),
            (512, 512, 256, 256)
        );
        assert!(overlapping > 0 && stalled.iter().all(|n| *n > 0));
        println!(
            "concurrent {engine:?}: overlapping={overlapping} stalls_AW_W_AR_B_R_CSR={stalled:?} ledger={:?}",
            h.score.l
        );
        h.rtl(&format!("concurrent-{engine:?}"));
    }
}

#[test]
fn p0_delayed_csr_completion_retains_owner_while_new_channels_arrive() {
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        for first_write in [false, true] {
            let mut h = Harness::new(engine);
            h.issue(first_write, 0, 0x12345678, 15, 0, 0, false);
            for _ in 0..16 {
                let mut i = inputs();
                set(&mut i, "csr_req_ready", 1);
                h.step(i);
                if h.score.owner.is_some() {
                    break;
                }
            }
            let original = h.score.owner.expect("first request committed");
            assert_eq!(original.write, first_write);
            let mut i = inputs();
            for (n, v) in [
                ("s_axi_awvalid", 1),
                ("s_axi_awaddr", 6),
                ("s_axi_wvalid", 1),
                ("s_axi_wdata", 0xaabbccdd),
                ("s_axi_wstrb", 5),
                ("s_axi_arvalid", 1),
                ("s_axi_araddr", 9),
                ("csr_req_ready", 1),
            ] {
                set(&mut i, n, v);
            }
            let o = h.step(i);
            assert!(
                bit(&o, "s_axi_awready") && bit(&o, "s_axi_wready") && bit(&o, "s_axi_arready")
            );
            for _ in 0..31 {
                let mut i = inputs();
                set(&mut i, "csr_req_ready", 1);
                let o = h.step(i);
                assert!(!bit(&o, "csr_req_valid"));
                assert_eq!(h.score.owner, Some(original));
                assert_eq!((h.score.l.submit, h.score.l.consume), (1, 0));
            }
            assert_eq!(
                (h.score.aw.len(), h.score.w.len(), h.score.ar.len()),
                (1, 1, 1)
            );
            h.drain();
            assert_eq!(
                (
                    h.score.l.submit,
                    h.score.l.consume,
                    h.score.l.b + h.score.l.r
                ),
                (3, 3, 3)
            );
            assert_eq!(h.score.order[0], original);
            println!(
                "delayed completion {engine:?} first_write={first_write} held31cycles newAW/W/ARcaptured ledger={:?}",
                h.score.l
            );
            h.rtl(&format!("delayed-{engine:?}-{first_write}"));
        }
    }
}

#[test]
fn p0_scoreboard_rejects_response_and_new_request_on_same_edge() {
    let setup = || {
        let mut s = Score::default();
        s.owner = Some(Req {
            write: false,
            addr: 0,
            data: 0,
            strb: 0,
        });
        s.aw.push_back(6);
        s.w.push_back((0x55, 15));
        s
    };
    let mut i = inputs();
    let mut o: Values = OUTPUTS.iter().map(|n| (n.to_string(), 0)).collect();
    for (n, v) in [
        ("csr_rsp_valid", 1),
        ("csr_req_ready", 1),
        ("csr_rdata", reply(setup().owner.unwrap()).data),
    ] {
        set(&mut i, n, v);
    }
    for (n, v) in [
        ("csr_rsp_ready", 1),
        ("csr_req_valid", 1),
        ("csr_write", 1),
        ("csr_addr", 6),
        ("csr_wdata", 0x55),
        ("csr_wstrb", 15),
    ] {
        set(&mut o, n, v);
    }
    let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| setup().edge(&i, &o)))
        .unwrap_err();
    let message = failure
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| failure.downcast_ref::<&str>().copied())
        .unwrap_or("");
    assert!(
        message.contains("CSR response and new submission on the same edge"),
        "{message}"
    );
    // The same two events on separate edges are legal to the oracle.
    let mut good = setup();
    set(&mut o, "csr_req_valid", 0);
    good.edge(&i, &o);
    set(&mut i, "csr_rsp_valid", 0);
    set(&mut o, "csr_rsp_ready", 0);
    set(&mut o, "csr_req_valid", 1);
    good.edge(&i, &o);
    assert!(good.owner.unwrap().write);
}
