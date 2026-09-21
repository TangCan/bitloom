//! Story127.4: API and standalone native behavior.
use bitloom_hir::{PortDirection, PortValues};
use bitloom_prelude::{Elaboratable, ElaborateSession, GroundType, Span, ip::CsrDecoder};
use bitloom_sim::{Sim, TickEngine};
use std::collections::BTreeMap;
const LEAVES: [&str; 4] = ["uart", "gpio", "timer", "irq"];
type Values = BTreeMap<String, u64>;
fn ports() -> BTreeMap<String, (PortDirection, GroundType)> {
    use PortDirection::{Input, Output};
    let mut p = BTreeMap::from([
        ("clk".into(), (Input, GroundType::Clock)),
        ("rst".into(), (Input, GroundType::Reset)),
    ]);
    for (n, w, d) in [
        ("req_valid", 1, Input),
        ("write", 1, Input),
        ("addr", 16, Input),
        ("wdata", 32, Input),
        ("wstrb", 4, Input),
        ("rsp_ready", 1, Input),
        ("req_ready", 1, Output),
        ("rsp_valid", 1, Output),
        ("rdata", 32, Output),
        ("error", 2, Output),
    ] {
        p.insert(n.into(), (d, GroundType::UInt { width: w }));
    }
    for leaf in LEAVES {
        for (n, w, d) in [
            ("req_ready", 1, Input),
            ("rsp_valid", 1, Input),
            ("rdata", 32, Input),
            ("error", 2, Input),
            ("req_valid", 1, Output),
            ("write", 1, Output),
            ("addr", 16, Output),
            ("wdata", 32, Output),
            ("wstrb", 4, Output),
            ("rsp_ready", 1, Output),
        ] {
            p.insert(format!("{leaf}_{n}"), (d, GroundType::UInt { width: w }));
        }
    }
    p
}
fn inputs() -> Values {
    ports()
        .into_iter()
        .filter(|(_, (d, _))| *d == PortDirection::Input)
        .map(|(n, _)| (n, 0))
        .collect()
}
fn set(i: &mut Values, name: &str, v: u64) {
    i.insert(name.into(), v);
}
struct Dut(Sim);
impl Dut {
    fn new(engine: TickEngine) -> Self {
        let mut d = Self(Sim::with_engine(CsrDecoder::elaborate().unwrap(), engine));
        let mut i = inputs();
        set(&mut i, "rst", 1);
        d.step(&i);
        d
    }
    fn view(&mut self, i: &Values) -> Values {
        let mut v = PortValues::default();
        for (n, x) in i {
            v.set(n, *x);
        }
        self.0.set_inputs(v);
        self.0.settle();
        ports()
            .into_iter()
            .filter(|(_, (d, _))| *d == PortDirection::Output)
            .map(|(n, _)| {
                let v = self.0.ports().get(&n).unwrap();
                (n, v)
            })
            .collect()
    }
    fn step(&mut self, i: &Values) -> Values {
        let o = self.view(i);
        self.0.tick();
        o
    }
}
// Deliberately independent literal ranges; no production map/helper imported.
fn golden(addr: u64) -> Option<(usize, u64)> {
    if addr <= 0x00ff {
        Some((0, addr))
    } else if (0x0100..=0x01ff).contains(&addr) {
        Some((1, addr - 0x0100))
    } else if (0x0200..=0x02ff).contains(&addr) {
        Some((2, addr - 0x0200))
    } else if (0x0300..=0x03ff).contains(&addr) {
        Some((3, addr - 0x0300))
    } else {
        None
    }
}
fn no_requests(o: &Values) {
    for l in LEAVES {
        assert_eq!(o[&format!("{l}_req_valid")], 0);
    }
}
#[test]
fn p0_exact_52_ports_shared_body_and_definition_reuse() {
    assert_eq!(std::mem::size_of::<CsrDecoder>(), 0);
    let hir = CsrDecoder::elaborate().unwrap();
    let m = &hir.circuit().modules[0];
    assert_eq!(m.ports.len(), 52);
    let actual: BTreeMap<_, _> = m
        .ports
        .iter()
        .map(|p| (p.name.clone(), (p.direction, p.ty.clone())))
        .collect();
    assert_eq!(actual, ports());
    let mut s = ElaborateSession::new("CsrDecoder");
    assert_eq!(
        CsrDecoder::define_module(&mut s, "CsrDecoder").unwrap(),
        "CsrDecoder"
    );
    CsrDecoder::define_module(&mut s, "CsrDecoder").unwrap();
    let shared = s.finish().unwrap();
    assert_eq!(shared.circuit().modules.len(), 1);
    assert_eq!(
        hir, shared,
        "independent and shared entry must use the same body"
    );
    assert_eq!(
        bitloom_vlog::emit(&hir).files[0].contents,
        bitloom_vlog::emit(&shared).files[0].contents
    );
}
#[test]
fn p0_reused_definition_rejects_wrong_connection_width_and_direction() {
    for bad_width in [true, false] {
        let mut s = ElaborateSession::new("Top");
        CsrDecoder::define_module(&mut s, "Decoder").unwrap();
        let span = Span::default();
        s.begin_module("Top", span);
        let mut connects = vec![];
        for (n, (d, ty)) in ports() {
            let ty = if bad_width && n == "addr" {
                GroundType::UInt { width: 8 }
            } else {
                ty
            };
            if d == PortDirection::Input || (!bad_width && n == "req_ready") {
                s.add_input(n.clone(), ty, span);
            } else {
                s.add_output(n.clone(), ty, span);
            }
            connects.push((n.clone(), n));
        }
        s.add_instance("decoder", "Decoder", connects, vec![], span);
        s.end_module();
        assert!(
            !s.finish()
                .expect_err("invalid decoder wiring accepted")
                .0
                .is_empty()
        );
    }
}
#[test]
fn p0_all_65536_addresses_route_actual_dut_without_high_alias_or_alignment() {
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        let mut d = Dut::new(engine);
        let mut i = inputs();
        set(&mut i, "req_valid", 1);
        set(&mut i, "write", 1);
        set(&mut i, "wdata", 0xa55ac33c);
        set(&mut i, "wstrb", 0);
        // No clock edge: exhaustively inspect actual idle combinational routing.
        for addr in 0..=65535 {
            set(&mut i, "addr", addr);
            let o = d.view(&i);
            let g = golden(addr);
            for (j, l) in LEAVES.iter().enumerate() {
                assert_eq!(
                    o[&format!("{l}_req_valid")],
                    u64::from(g.is_some_and(|(k, _)| k == j)),
                    "{engine:?} addr={addr:04x}"
                );
                if let Some((k, local)) = g {
                    if k == j {
                        assert_eq!(o[&format!("{l}_addr")], local);
                        assert_eq!(o[&format!("{l}_write")], 1);
                        assert_eq!(o[&format!("{l}_wdata")], 0xa55ac33c);
                        assert_eq!(o[&format!("{l}_wstrb")], 0);
                    }
                }
            }
            assert_eq!(o["req_ready"], u64::from(g.is_none()));
            assert_eq!(o["rsp_valid"], 0, "DECERR must not be combinational");
        }
    }
}
#[test]
fn p0_owner_survives_long_request_response_stalls_and_input_changes() {
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        for (owner, leaf) in LEAVES.iter().enumerate() {
            let mut d = Dut::new(engine);
            let mut i = inputs();
            for (n, v) in [
                ("req_valid", 1),
                ("addr", owner as u64 * 256 + 255),
                ("write", 1),
                ("wdata", 0xfedc1234),
                ("wstrb", 5),
            ] {
                set(&mut i, n, v);
            }
            for _ in 0..31 {
                let o = d.step(&i);
                assert_eq!(o["req_ready"], 0);
                assert_eq!(o[&format!("{leaf}_req_valid")], 1);
                assert_eq!(o[&format!("{leaf}_addr")], 255);
                assert_eq!(o[&format!("{leaf}_wdata")], 0xfedc1234);
            }
            set(&mut i, &format!("{leaf}_req_ready"), 1);
            let o = d.step(&i);
            assert_eq!(o["req_ready"], 1);
            assert_eq!(o[&format!("{leaf}_req_valid")], 1, "same-edge leaf commit");
            i = inputs();
            // A causal responder can wait without a finite latency guarantee.
            for _ in 0..31 {
                let o = d.step(&i);
                no_requests(&o);
                assert_eq!(o["req_ready"], 0);
                assert_eq!(o["rsp_valid"], 0);
            }
            for l in LEAVES {
                set(&mut i, &format!("{l}_req_ready"), 1);
                set(&mut i, &format!("{l}_rsp_valid"), 1);
                set(&mut i, &format!("{l}_rdata"), 0xdeadbeef);
                set(&mut i, &format!("{l}_error"), 3);
            }
            // Nonowner noise intentionally tests isolation; not claimed as legal peer traffic.
            set(&mut i, &format!("{leaf}_rdata"), 0x1234abcd);
            set(&mut i, &format!("{leaf}_error"), 2);
            for n in 0..31 {
                set(&mut i, "addr", 0x8000 + n);
                set(&mut i, "write", n % 2);
                let o = d.step(&i);
                no_requests(&o);
                assert_eq!((o["rsp_valid"], o["rdata"], o["error"]), (1, 0x1234abcd, 2));
                for l in LEAVES {
                    assert_eq!(o[&format!("{l}_rsp_ready")], 0);
                }
            }
            set(&mut i, "rsp_ready", 1);
            let o = d.step(&i);
            assert_eq!(o["req_ready"], 0);
            for l in LEAVES {
                assert_eq!(o[&format!("{l}_rsp_ready")], u64::from(l == *leaf));
            }
            for _ in 0..8 {
                let o = d.step(&inputs());
                assert_eq!(o["rsp_valid"], 0);
                no_requests(&o);
            }
        }
    }
}
#[test]
fn p0_decerr_is_registered_stable_and_reset_cancels_before_consumption() {
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        for addr in [0x0400, 0x0401, 0x7fff, 0x8000, 0x8104, 0xfffc, 0xffff] {
            let mut d = Dut::new(engine);
            let mut i = inputs();
            set(&mut i, "req_valid", 1);
            set(&mut i, "addr", addr);
            let o = d.step(&i);
            no_requests(&o);
            assert_eq!((o["req_ready"], o["rsp_valid"]), (1, 0));
            i = inputs();
            for n in 0..31 {
                set(&mut i, "addr", n);
                let o = d.step(&i);
                no_requests(&o);
                assert_eq!(
                    (o["req_ready"], o["rsp_valid"], o["error"], o["rdata"]),
                    (0, 1, 3, 0)
                );
            }
            set(&mut i, "rst", 1);
            set(&mut i, "rsp_ready", 1);
            set(&mut i, "req_valid", 1);
            let o = d.step(&i);
            assert_eq!(
                o["rsp_valid"], 1,
                "synchronous reset must not gate response combinationally"
            );
            assert_eq!(o["req_ready"], 0);
            no_requests(&o);
            for l in LEAVES {
                assert_eq!(o[&format!("{l}_rsp_ready")], 0);
            }
            for _ in 0..8 {
                let o = d.step(&inputs());
                assert_eq!(o["rsp_valid"], 0);
            }
            let mut i = inputs();
            set(&mut i, "req_valid", 1);
            set(&mut i, "addr", 0x0204);
            set(&mut i, "timer_req_ready", 1);
            let o = d.step(&i);
            assert_eq!(
                (o["req_ready"], o["timer_req_valid"], o["timer_addr"]),
                (1, 1, 4)
            );
        }
    }
}

#[test]
fn p0_invalid_names_and_conflicting_definitions_poison_session() {
    for name in ["", "not a module", "module"] {
        let mut s = ElaborateSession::new("Top");
        assert!(CsrDecoder::define_module(&mut s, name).is_err(), "{name:?}");
        assert!(s.finish().is_err());
    }
    let mut s = ElaborateSession::new("Decoder");
    CsrDecoder::define_module(&mut s, "Decoder").unwrap();
    assert!(bitloom_prelude::ip::AxiLiteCsrBridge::define_module(&mut s, "Decoder").is_err());
    assert!(s.finish().is_err());
}

#[test]
fn p0_owner_reset_cancels_waiting_and_visible_response_without_replay() {
    for engine in [TickEngine::Interpreter, TickEngine::Compiled] {
        for leaf in LEAVES {
            for visible in [false, true] {
                let mut d = Dut::new(engine);
                let mut i = inputs();
                let index = LEAVES.iter().position(|l| *l == leaf).unwrap();
                set(&mut i, "req_valid", 1);
                set(&mut i, "addr", index as u64 * 256);
                set(&mut i, &format!("{leaf}_req_ready"), 1);
                assert_eq!(d.step(&i)["req_ready"], 1);
                i = inputs();
                set(&mut i, &format!("{leaf}_rsp_valid"), u64::from(visible));
                set(&mut i, "rst", 1);
                set(&mut i, "rsp_ready", 1);
                set(&mut i, "req_valid", 1);
                let o = d.step(&i);
                assert_eq!(o["rsp_valid"], u64::from(visible));
                no_requests(&o);
                for l in LEAVES {
                    assert_eq!(o[&format!("{l}_rsp_ready")], 0);
                }
                for _ in 0..31 {
                    assert_eq!(d.step(&inputs())["rsp_valid"], 0);
                }
                i = inputs();
                set(&mut i, "req_valid", 1);
                set(&mut i, "addr", 0xffff);
                assert_eq!(d.step(&i)["req_ready"], 1);
                i = inputs();
                set(&mut i, "rsp_ready", 1);
                let o = d.step(&i);
                assert_eq!((o["rsp_valid"], o["error"]), (1, 3));
                assert_eq!(d.step(&inputs())["rsp_valid"], 0);
            }
        }
    }
}

#[path = "fr196_csr_decoder/standalone.rs"]
mod standalone;
