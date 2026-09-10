#![allow(unused_imports)]
use crate::{
    Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span,
    SynthesizableClosureViolation, diagnose_synthesizable_closure_violations,
};

///
/// Non-goals: async/CDC FIFO ([`crate::SyncFIFO`] language-level FR79), FWFT bypass,
/// generator-closure depth/width params (Epic 29).
pub struct SyncFifo;

impl Elaboratable for SyncFifo {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("SyncFifo");
        s.begin_module("SyncFifo", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("wr_en", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("rd_en", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("full", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("empty", GroundType::UInt { width: 1 }, Span::default());

        s.declare_mem("ram", 4, 8, Span::default());
        s.declare_reg("wr_ptr", GroundType::UInt { width: 2 }, Span::default());
        s.declare_reg("rd_ptr", GroundType::UInt { width: 2 }, Span::default());
        s.declare_reg("count", GroundType::UInt { width: 3 }, Span::default());
        s.declare_reg("dout", GroundType::UInt { width: 8 }, Span::default());

        s.declare_wire("c0_1", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("c1_2", GroundType::UInt { width: 2 }, Span::default());
        s.declare_wire("c0_3", GroundType::UInt { width: 3 }, Span::default());
        s.declare_wire("c1_3", GroundType::UInt { width: 3 }, Span::default());
        s.declare_wire("c4_3", GroundType::UInt { width: 3 }, Span::default());
        s.declare_wire("wr_ptr_p1", GroundType::UInt { width: 2 }, Span::default());
        s.declare_wire("rd_ptr_p1", GroundType::UInt { width: 2 }, Span::default());
        s.declare_wire("count_p1", GroundType::UInt { width: 3 }, Span::default());
        s.declare_wire("count_m1", GroundType::UInt { width: 3 }, Span::default());
        s.declare_wire("can_wr", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("can_rd", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("next_wr", GroundType::UInt { width: 2 }, Span::default());
        s.declare_wire("next_rd", GroundType::UInt { width: 2 }, Span::default());
        s.declare_wire(
            "count_after_wr",
            GroundType::UInt { width: 3 },
            Span::default(),
        );
        s.declare_wire(
            "count_if_rd",
            GroundType::UInt { width: 3 },
            Span::default(),
        );
        s.declare_wire("next_count", GroundType::UInt { width: 3 }, Span::default());

        s.begin_combinational(Span::default());
        s.assign_lit("c0_1", 0, Span::default());
        s.assign_lit("c1_2", 1, Span::default());
        s.assign_lit("c0_3", 0, Span::default());
        s.assign_lit("c1_3", 1, Span::default());
        s.assign_lit("c4_3", 4, Span::default());
        s.assign_eq("full", "count", "c4_3", Span::default());
        s.assign_eq("empty", "count", "c0_3", Span::default());
        // can_wr = wr_en && !full; can_rd = rd_en && !empty
        s.assign_mux("can_wr", "full", "c0_1", "wr_en", Span::default());
        s.assign_mux("can_rd", "empty", "c0_1", "rd_en", Span::default());
        s.assign_add("wr_ptr_p1", "wr_ptr", "c1_2", Span::default());
        s.assign_add("rd_ptr_p1", "rd_ptr", "c1_2", Span::default());
        s.assign_add("count_p1", "count", "c1_3", Span::default());
        s.assign_sub("count_m1", "count", "c1_3", Span::default());
        s.assign_mux("next_wr", "can_wr", "wr_ptr_p1", "wr_ptr", Span::default());
        s.assign_mux("next_rd", "can_rd", "rd_ptr_p1", "rd_ptr", Span::default());
        s.assign_mux(
            "count_after_wr",
            "can_wr",
            "count_p1",
            "count",
            Span::default(),
        );
        // both: count unchanged; only-rd: count-1; only-wr: count+1
        s.assign_mux(
            "count_if_rd",
            "can_wr",
            "count",
            "count_m1",
            Span::default(),
        );
        s.assign_mux(
            "next_count",
            "can_rd",
            "count_if_rd",
            "count_after_wr",
            Span::default(),
        );
        s.assign_net("data_out", "dout", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        // Order: mem ops use current pointers, then advance pointers/count.
        s.assign_mem_write_en("ram", "wr_ptr", "data_in", "can_wr", Span::default());
        s.assign_reg_d_mem_read("dout", "ram", "rd_ptr", Span::default());
        s.assign_reg_d_from("wr_ptr", "next_wr", Span::default());
        s.assign_reg_d_from("rd_ptr", "next_rd", Span::default());
        s.assign_reg_d_from("count", "next_count", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}
