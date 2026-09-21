use bitloom_prelude::{Elaboratable, ElaborateSession, ip::Irq};

fn main() {
    assert_eq!(timer_irq().circuit().name, "TimerIrq");
    let mut session = ElaborateSession::new("Irq5");
    Irq::define_module(&mut session, "Irq5").unwrap();
    Irq::define_module(&mut session, "Irq5").unwrap(); // Reuse, not duplicate.
    let composed = session.finish().unwrap(); // Exactly one freeze.
    assert_eq!(composed.circuit().name, "Irq5");
    let _standalone = Irq::elaborate().unwrap(); // Its own independent session.
    let registers = Irq::registers();
    let output = std::env::args().nth(1).expect("software output directory");
    std::fs::create_dir_all(&output).unwrap();
    std::fs::write(
        format!("{output}/irq-registers.h"),
        registers.emit_c_header().unwrap(),
    )
    .unwrap();
    std::fs::write(
        format!("{output}/irq-registers.md"),
        registers.emit_markdown().unwrap(),
    )
    .unwrap();
}

// Actual Timer event wiring: both definitions belong to the same session.
// Equivalent event wiring is exercised by fr197_irq_api on all RTL backends;
// this exact example graph is compiled and elaborated by the example gate.
fn timer_irq() -> bitloom_prelude::FrozenHir {
    use bitloom_prelude::{GroundType, Span, ip::Timer};
    let mut s = ElaborateSession::new("TimerIrq");
    Timer::define_module(&mut s, "ActualTimer").unwrap();
    Irq::define_module(&mut s, "ActualIrq").unwrap();
    let sp = Span::default();
    s.begin_module("TimerIrq", sp);
    s.add_input("clk", GroundType::Clock, sp);
    s.add_input("rst", GroundType::Reset, sp);
    for lane in 0..2 {
        let mut connections = vec![("clk".into(), "clk".into()), ("rst".into(), "rst".into())];
        for (name, width) in [
            ("req_valid", 1),
            ("write", 1),
            ("addr", 16),
            ("wdata", 32),
            ("wstrb", 4),
            ("rsp_ready", 1),
        ] {
            let net = format!("t{lane}_{name}");
            s.add_input(&net, GroundType::UInt { width }, sp);
            connections.push((name.into(), net));
        }
        for (name, width) in [
            ("req_ready", 1),
            ("rsp_valid", 1),
            ("rdata", 32),
            ("error", 2),
        ] {
            let net = format!("t{lane}_{name}");
            s.add_output(&net, GroundType::UInt { width }, sp);
            connections.push((name.into(), net));
        }
        if lane == 0 {
            s.add_output("match_event", GroundType::UInt { width: 1 }, sp);
            connections.push(("match_event".into(), "match_event".into()));
        } else {
            s.declare_wire("raw_events", GroundType::UInt { width: 5 }, sp);
            s.add_output("irq", GroundType::UInt { width: 1 }, sp);
            connections.push(("raw_events".into(), "raw_events".into()));
            connections.push(("irq".into(), "irq".into()));
        }
        s.add_instance(
            format!("peripheral{lane}"),
            if lane == 0 {
                "ActualTimer"
            } else {
                "ActualIrq"
            },
            connections,
            vec![],
            sp,
        );
    }
    s.declare_wire("zero4", GroundType::UInt { width: 4 }, sp);
    s.begin_combinational(sp);
    s.assign_lit("zero4", 0, sp);
    s.assign_concat("raw_events", "zero4", "match_event", sp);
    s.end_process();
    s.end_module();
    s.finish().unwrap()
}
