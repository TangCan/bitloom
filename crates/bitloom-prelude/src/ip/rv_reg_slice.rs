use crate::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};
use bitloom_hir::Diagnostic;

/// Two-slot, single-clock ready/valid register slice (FR195).
///
/// WIDTH is 1..=64. Outputs depend only on registered state: no empty bypass,
/// and a full slice cannot accept until the cycle after a dequeue. Synchronous
/// active-high reset has priority over flush; either cancels all pending data.
/// Before relying on ready/valid, apply an active synchronous reset clock edge.
pub struct RvRegSlice<const WIDTH: u32 = 32>;

impl<const WIDTH: u32> Elaboratable for RvRegSlice<WIDTH> {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("RvRegSlice");
        Self::define_module(&mut s, "RvRegSlice")?;
        s.finish()
    }
}

impl<const WIDTH: u32> RvRegSlice<WIDTH> {
    /// Define this parameterized slice in the caller's shared session.
    pub fn define_module(
        session: &mut ElaborateSession,
        name: impl Into<String>,
    ) -> Result<String, Diagnostics> {
        if !(1..=64).contains(&WIDTH) {
            return Err(Diagnostics(vec![Diagnostic {
                span: Span::default(),
                code: "rhdl::E0200".into(),
                en: format!("RvRegSlice WIDTH must be 1..=64; got {WIDTH}"),
                zh: format!("RvRegSlice WIDTH 必须为 1..=64；实际为 {WIDTH}"),
            }]));
        }
        session.define_module(name, vec![("WIDTH".into(), WIDTH)], Self::define_body)
    }

    fn define_body(s: &mut ElaborateSession, _: &[(String, u32)]) -> Result<(), Diagnostics> {
        let sp = Span::default();
        s.add_input("clk", GroundType::Clock, sp);
        s.add_input("rst", GroundType::Reset, sp);
        for name in ["flush", "input_valid", "output_ready"] {
            s.add_input(name, GroundType::UInt { width: 1 }, sp);
        }
        s.add_input("input_data", GroundType::UInt { width: WIDTH }, sp);
        for name in ["input_ready", "output_valid"] {
            s.add_output(name, GroundType::UInt { width: 1 }, sp);
        }
        s.add_output("output_data", GroundType::UInt { width: WIDTH }, sp);
        s.declare_reg("count", GroundType::UInt { width: 2 }, sp);
        for name in ["front", "back"] {
            s.declare_reg(name, GroundType::UInt { width: WIDTH }, sp);
        }
        for name in [
            "zero",
            "one",
            "two",
            "inc",
            "dec",
            "after_push",
            "normal_count",
            "next_count",
        ] {
            s.declare_wire(name, GroundType::UInt { width: 2 }, sp);
        }
        for name in [
            "true_bit",
            "empty",
            "single",
            "full",
            "push",
            "pop",
            "replace_single",
            "load_front",
            "push_front",
        ] {
            s.declare_wire(name, GroundType::UInt { width: 1 }, sp);
        }
        for name in ["after_pop", "next_front"] {
            s.declare_wire(name, GroundType::UInt { width: WIDTH }, sp);
        }
        s.begin_combinational(sp);
        s.assign_lit("zero", 0, sp);
        s.assign_lit("one", 1, sp);
        s.assign_lit("two", 2, sp);
        s.assign_lit("true_bit", 1, sp);
        s.assign_eq("empty", "count", "zero", sp);
        s.assign_eq("single", "count", "one", sp);
        s.assign_eq("full", "count", "two", sp);
        s.assign_xor("input_ready", "full", "true_bit", sp);
        s.assign_xor("output_valid", "empty", "true_bit", sp);
        s.assign_net("output_data", "front", sp);
        s.assign_and("push", "input_valid", "input_ready", sp);
        s.assign_and("pop", "output_valid", "output_ready", sp);
        s.assign_add("inc", "count", "one", sp);
        s.assign_mux("after_push", "push", "inc", "count", sp);
        s.assign_sub("dec", "after_push", "one", sp);
        s.assign_mux("normal_count", "pop", "dec", "after_push", sp);
        s.assign_mux("next_count", "flush", "zero", "normal_count", sp);
        // Front replacement on empty enqueue or a simultaneous single-slot transfer.
        s.assign_and("replace_single", "pop", "single", sp);
        s.assign_or("load_front", "empty", "replace_single", sp);
        s.assign_and("push_front", "push", "load_front", sp);
        s.assign_mux("after_pop", "pop", "back", "front", sp);
        s.assign_mux("next_front", "push_front", "input_data", "after_pop", sp);
        s.end_process();
        // Builder registers provide synchronous reset priority over these D inputs.
        s.begin_sequential(sp);
        s.assign_reg_d_from("count", "next_count", sp);
        s.assign_reg_d_from("front", "next_front", sp);
        s.assign_reg_d_mux("back", "push", "input_data", "back", sp);
        s.end_process();
        Ok(())
    }
}
