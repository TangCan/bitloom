//! FR29 mixed-`both` fixture: RTL `tick` vs handwritten abstraction, compared on `PortValues`.

use bitloom_prelude::{
    Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, HostView, Span, ViewKind,
    rhdl,
};

/// Cycle-accurate RTL generator (FrozenHir). Not a host view.
pub struct CounterRtl;

impl Elaboratable for CounterRtl {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("MixedCounter");
        s.begin_module("MixedCounter", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "count", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_inc("count", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

/// Untimed host model of the same port contract.
#[rhdl::abstraction]
pub struct CounterAbs {
    pub count: u64,
}

impl CounterAbs {
    pub fn cycle(&mut self, inputs: &bitloom_prelude::PortValues) -> bitloom_prelude::PortValues {
        if inputs.get("rst").unwrap_or(0) != 0 {
            self.count = 0;
        } else {
            self.count = self.count.wrapping_add(1);
        }
        let mut out = inputs.clone();
        out.set("data_out", self.count);
        out
    }
}

/// Handwritten pin adapter (not generated from HIR).
#[rhdl::bridge]
pub struct CounterBridge;

impl CounterBridge {
    pub fn to_pins(inputs: &bitloom_prelude::PortValues) -> bitloom_prelude::PortValues {
        inputs.clone()
    }
}

/// Mixed fixture: RTL + abstraction + bridge in one host type.
#[rhdl::both]
pub struct MixedCounter {
    pub abs: CounterAbs,
}

impl MixedCounter {
    pub fn new() -> Self {
        Self {
            abs: CounterAbs { count: 0 },
        }
    }
}

impl Default for MixedCounter {
    fn default() -> Self {
        Self::new()
    }
}

const _: () = {
    assert!(<CounterAbs as HostView>::KIND as u8 == ViewKind::Abstraction as u8);
    assert!(<CounterBridge as HostView>::KIND as u8 == ViewKind::Bridge as u8);
    assert!(<MixedCounter as HostView>::KIND as u8 == ViewKind::Both as u8);
};

#[cfg(test)]
mod tests {
    use super::*;
    use bitloom_prelude::Elaboratable;
    use bitloom_prelude::PortValues;
    use bitloom_sim::{AbstractionView, Sim, check_mixed_both};

    impl AbstractionView for CounterAbs {
        fn cycle(&mut self, inputs: &PortValues) -> PortValues {
            CounterAbs::cycle(self, inputs)
        }
    }

    #[test]
    fn both_fixture_matches_tick() {
        let hir = CounterRtl::elaborate().unwrap();
        let mut sim = Sim::new(hir);
        let mut mixed = MixedCounter::new();
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        check_mixed_both(&mut sim, &mut mixed.abs, pv.clone()).unwrap();
        pv.set("rst", 0);
        for _ in 0..3 {
            let bridged = CounterBridge::to_pins(&pv);
            check_mixed_both(&mut sim, &mut mixed.abs, bridged).unwrap();
        }
        assert_eq!(sim.ports().get("data_out"), Some(3));
    }

    #[test]
    fn mismatch_fails() {
        struct Wrong;
        impl AbstractionView for Wrong {
            fn cycle(&mut self, inputs: &PortValues) -> PortValues {
                let mut o = inputs.clone();
                o.set("data_out", 7);
                o
            }
        }
        let hir = CounterRtl::elaborate().unwrap();
        let mut sim = Sim::new(hir);
        let mut w = Wrong;
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        assert!(check_mixed_both(&mut sim, &mut w, pv).is_err());
    }
}
