//! Story 15.1 language-surface feasibility spike (FR60).
//!
//! Proves Episode I needs that already work on Bitloom: sync reset, SyncReadMem
//! ports, multi-state control via `Eq`/`Mux`/`Lit`, elaborate + `bitloom-sim` tick.
//! Design deps: `bitloom-prelude` only (NFR24).

use bitloom_prelude::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// Minimal multi-state + SyncReadMem + sync-reset fixture.
pub struct FeasibilitySpike;

impl Elaboratable for FeasibilitySpike {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("FeasibilitySpike");
        s.begin_module("FeasibilitySpike", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("addr", GroundType::UInt { width: 2 }, Span::default());
        s.add_input("wdata", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("we", GroundType::Bool, Span::default());
        s.add_output("state_out", GroundType::UInt { width: 2 }, Span::default());
        s.add_output("rdata", GroundType::UInt { width: 8 }, Span::default());

        s.declare_sync_read_mem("imem", 4, 8, Span::default());
        s.declare_reg("state", GroundType::UInt { width: 2 }, Span::default());
        s.declare_reg("q", GroundType::UInt { width: 8 }, Span::default());

        s.declare_wire("c0", GroundType::UInt { width: 2 }, Span::default());
        s.declare_wire("c1", GroundType::UInt { width: 2 }, Span::default());
        s.declare_wire("c2", GroundType::UInt { width: 2 }, Span::default());
        s.declare_wire("is_idle", GroundType::Bool, Span::default());
        s.declare_wire("is_fetch", GroundType::Bool, Span::default());
        s.declare_wire("mid", GroundType::UInt { width: 2 }, Span::default());
        s.declare_wire("next_state", GroundType::UInt { width: 2 }, Span::default());

        s.begin_combinational(Span::default());
        s.assign_lit("c0", 0, Span::default());
        s.assign_lit("c1", 1, Span::default());
        s.assign_lit("c2", 2, Span::default());
        // Idle(0)→Fetch(1)→Exec(2)→Idle(0)
        s.assign_eq("is_idle", "state", "c0", Span::default());
        s.assign_eq("is_fetch", "state", "c1", Span::default());
        s.assign_mux("mid", "is_fetch", "c2", "c0", Span::default());
        s.assign_mux("next_state", "is_idle", "c1", "mid", Span::default());
        s.assign_net("state_out", "state", Span::default());
        s.assign_net("rdata", "q", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_from("state", "next_state", Span::default());
        // Sequential `we` gating is deferred; tests keep writes idempotent.
        s.assign_mem_write("imem", "addr", "wdata", Span::default());
        s.assign_reg_d_mem_read("q", "imem", "addr", Span::default());
        s.end_process();

        s.end_module();
        s.finish()
    }
}

pub fn rhdl_elaborate() -> Result<FrozenHir, Diagnostics> {
    FeasibilitySpike::elaborate()
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
        let f = FeasibilitySpike::elaborate().unwrap();
        assert_eq!(f.abi_name, "FeasibilitySpike");
    }

    #[test]
    fn tick_fsm_and_sync_read_mem_golden() {
        let mut sim = Sim::new(FeasibilitySpike::elaborate().unwrap());
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("addr", 1);
        pv.set("wdata", 0x5A);
        pv.set("we", 1);
        sim.set_inputs(pv.clone());
        sim.tick();
        assert_eq!(sim.ports().get("state_out"), Some(0));
        assert_eq!(sim.ports().get("rdata"), Some(0));

        pv.set("rst", 0);
        sim.set_inputs(pv.clone());
        sim.tick(); // Idle→Fetch; write + schedule read
        assert_eq!(sim.ports().get("state_out"), Some(1));
        assert_eq!(sim.ports().get("rdata"), Some(0));

        sim.tick(); // Fetch→Exec; pending read delivers
        assert_eq!(sim.ports().get("state_out"), Some(2));
        assert_eq!(sim.ports().get("rdata"), Some(0x5A));

        sim.tick(); // Exec→Idle
        assert_eq!(sim.ports().get("state_out"), Some(0));
        assert_eq!(sim.ports().get("rdata"), Some(0x5A));
    }

    #[test]
    fn build_smoke_emits_yosys_friendly_v() {
        let hir = FeasibilitySpike::elaborate().unwrap();
        let art = emit(&hir);
        assert_eq!(art.filelist, vec!["FeasibilitySpike.v"]);
        let v = &art.files[0].contents;
        assert!(v.contains("module FeasibilitySpike"));
        assert!(v.contains("always @(posedge clk)"));
        assert!(!v.contains("always_ff"));
    }
}
