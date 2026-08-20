//! Story 17.1 — pipeline language-surface feasibility spike (FR68).
//!
//! Proves ≥2 pipeline `Reg`s + a forwarding `assign_mux` (and stall hold)
//! elaborate and tick under Bitloom. Design deps: `bitloom-prelude` only.

use bitloom_prelude::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// Two-stage pipe + EX/MEM→EX-style forward mux + stall hold on stage 0.
pub struct PipelineFeasibilitySpike;

impl Elaboratable for PipelineFeasibilitySpike {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("PipelineFeasibilitySpike");
        s.begin_module("PipelineFeasibilitySpike", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("din", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("bypass", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("fwd_sel", GroundType::Bool, Span::default());
        s.add_input("stall", GroundType::Bool, Span::default());
        s.add_output("s0_out", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("s1_out", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("fwd_out", GroundType::UInt { width: 8 }, Span::default());

        // Two pipeline registers: s0 then s1.
        s.declare_reg("s0", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("s1", GroundType::UInt { width: 8 }, Span::default());

        s.declare_wire("s0_next", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("fwd", GroundType::UInt { width: 8 }, Span::default());

        s.begin_combinational(Span::default());
        // Stall hold: freeze s0 when stall=1 (mux hold — not module-level `en`).
        s.assign_mux("s0_next", "stall", "s0", "din", Span::default());
        // Forward mux: EX/MEM→EX style — prefer bypass when fwd_sel=1.
        s.assign_mux("fwd", "fwd_sel", "bypass", "s1", Span::default());
        s.assign_net("s0_out", "s0", Span::default());
        s.assign_net("s1_out", "s1", Span::default());
        s.assign_net("fwd_out", "fwd", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        // Order matters for interpreter (in-place reg update): capture old s0 into s1 first.
        s.assign_reg_d_from("s1", "s0", Span::default());
        s.assign_reg_d_from("s0", "s0_next", Span::default());
        s.end_process();

        s.end_module();
        s.finish()
    }
}

pub fn rhdl_elaborate() -> Result<FrozenHir, Diagnostics> {
    PipelineFeasibilitySpike::elaborate()
}

#[cfg(test)]
mod tests {
    use bitloom_hir::PortValues;
    use bitloom_prelude::Elaboratable;
    use bitloom_sim::Sim;
    use bitloom_vlog::emit;

    use super::*;

    #[test]
    fn elaborate_ok() {
        let f = PipelineFeasibilitySpike::elaborate().unwrap();
        assert_eq!(f.abi_name, "PipelineFeasibilitySpike");
    }

    #[test]
    fn tick_two_stage_pipe_and_forward_golden() {
        let mut sim = Sim::new(PipelineFeasibilitySpike::elaborate().unwrap());
        let mut pv = PortValues::default();
        // Drive din under reset so comb computes s0_next before the first non-reset edge.
        pv.set("rst", 1);
        pv.set("din", 0x11);
        pv.set("bypass", 0xEE);
        pv.set("fwd_sel", 0);
        pv.set("stall", 0);
        sim.set_inputs(pv.clone());
        sim.tick();
        assert_eq!(sim.ports().get("s0_out"), Some(0));
        assert_eq!(sim.ports().get("s1_out"), Some(0));

        pv.set("rst", 0);
        sim.set_inputs(pv.clone());
        sim.tick(); // s0<=0x11, s1<=0
        assert_eq!(sim.ports().get("s0_out"), Some(0x11));
        assert_eq!(sim.ports().get("s1_out"), Some(0));
        assert_eq!(sim.ports().get("fwd_out"), Some(0)); // sel=0 → s1

        pv.set("din", 0x22);
        sim.set_inputs(pv.clone());
        sim.tick(); // comb still had din=0x11 → s0 stays 0x11, s1<=0x11
        sim.tick(); // now s0<=0x22, s1<=0x11
        assert_eq!(sim.ports().get("s0_out"), Some(0x22));
        assert_eq!(sim.ports().get("s1_out"), Some(0x11));
        assert_eq!(sim.ports().get("fwd_out"), Some(0x11));

        pv.set("fwd_sel", 1);
        pv.set("bypass", 0xAB);
        sim.set_inputs(pv.clone());
        sim.tick();
        assert_eq!(sim.ports().get("fwd_out"), Some(0xAB));
    }

    #[test]
    fn tick_stall_hold_mux_golden() {
        let mut sim = Sim::new(PipelineFeasibilitySpike::elaborate().unwrap());
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("din", 0x55);
        pv.set("bypass", 0);
        pv.set("fwd_sel", 0);
        pv.set("stall", 0);
        sim.set_inputs(pv.clone());
        sim.tick();

        pv.set("rst", 0);
        sim.set_inputs(pv.clone());
        sim.tick();
        assert_eq!(sim.ports().get("s0_out"), Some(0x55));

        // Arm stall while still presenting old din so comb sees stall=1 with s0=0x55.
        pv.set("stall", 1);
        pv.set("din", 0x99);
        sim.set_inputs(pv.clone());
        // One tick: seq may still use prior s0_next (=0x55 from stall=0); then comb sets hold.
        sim.tick();
        // Second tick under stall must keep s0 at 0x55 despite din=0x99.
        sim.tick();
        assert_eq!(sim.ports().get("s0_out"), Some(0x55));
        assert_eq!(sim.ports().get("s1_out"), Some(0x55));
    }

    #[test]
    fn emit_verilog_smoke() {
        let f = PipelineFeasibilitySpike::elaborate().unwrap();
        let art = emit(&f);
        assert_eq!(art.filelist, vec!["PipelineFeasibilitySpike.v"]);
        let v = &art.files[0].contents;
        assert!(v.contains("module PipelineFeasibilitySpike"));
        assert!(v.contains("always @(posedge clk)"));
    }
}
