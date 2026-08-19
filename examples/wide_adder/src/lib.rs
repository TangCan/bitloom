//! Const-generic width example — depends only on `rhdl-prelude`.

use rhdl_prelude::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// Parameterized adder shell elaborated at W=8 and W=16 from the same source pattern.
pub struct WideAdder<const W: u32>;

impl<const W: u32> Elaboratable for WideAdder<W> {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new(format!("WideAdder{W}"));
        s.begin_module(format!("WideAdder{W}"), Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("a", GroundType::UInt { width: W }, Span::default());
        s.add_input("b", GroundType::UInt { width: W }, Span::default());
        s.add_output("y", GroundType::UInt { width: W }, Span::default());
        s.begin_combinational(Span::default());
        // Phase-1: drive y from a (same-width connect); full add expr lands with ALU nodes.
        s.assign_net("y", "a", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

pub fn rhdl_elaborate() -> Result<FrozenHir, Diagnostics> {
    WideAdder::<8>::elaborate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rhdl_prelude::GroundType;

    #[test]
    fn w8_and_w16_port_widths() {
        let h8 = WideAdder::<8>::elaborate().unwrap();
        let h16 = WideAdder::<16>::elaborate().unwrap();
        assert!(matches!(
            h8.circuit().modules[0].ports[2].ty,
            GroundType::UInt { width: 8 }
        ));
        assert!(matches!(
            h16.circuit().modules[0].ports[2].ty,
            GroundType::UInt { width: 16 }
        ));
    }
}
