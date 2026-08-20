//! Single-clock FIFO-shaped fixture for FR22 (depth-1 skid buffer style).

use bitloom_prelude::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// Depth-1 registered skid: holds last `data_in` when not reset.
pub struct SkidFifo;

impl Elaboratable for SkidFifo {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("SkidFifo");
        s.begin_module("SkidFifo", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("q", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "q", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_from("q", "data_in", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

pub fn rhdl_elaborate() -> Result<FrozenHir, Diagnostics> {
    SkidFifo::elaborate()
}

#[cfg(test)]
mod tests {
    use bitloom_hir::PortValues;
    use bitloom_prelude::Elaboratable;
    use bitloom_vlog::emit;
    use rhdl_sim::Sim;

    use super::*;

    #[test]
    fn elaborate_ok() {
        let f = SkidFifo::elaborate().unwrap();
        assert_eq!(f.abi_name, "SkidFifo");
    }

    #[test]
    fn tick_skips_reset_then_captures_data_in() {
        let mut sim = Sim::new(SkidFifo::elaborate().unwrap());
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("data_in", 0x11);
        sim.set_inputs(pv.clone());
        sim.tick();
        assert_eq!(sim.ports().get("data_out"), Some(0));
        pv.set("rst", 0);
        pv.set("data_in", 0xA5);
        sim.set_inputs(pv);
        sim.tick();
        assert_eq!(sim.ports().get("data_out"), Some(0xA5));
    }

    #[test]
    fn build_smoke_emits_yosys_friendly_v() {
        let hir = SkidFifo::elaborate().unwrap();
        let art = emit(&hir);
        assert_eq!(art.filelist, vec!["SkidFifo.v"]);
        let v = &art.files[0].contents;
        assert!(v.contains("module SkidFifo"));
        assert!(v.contains("always @(posedge clk)"));
        assert!(!v.contains("always_ff"));
        assert!(!v.contains("logic "));
    }
}
