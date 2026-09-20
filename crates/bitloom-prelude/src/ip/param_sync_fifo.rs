use crate::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};
use bitloom_hir::Diagnostic;

/// Small single-clock register FIFO with no empty bypass (FR195).
///
/// WIDTH is 1..=64 and DEPTH is 1..=16. Apply a synchronous active-high
/// reset edge before use. Reset has priority over flush; both cancel resident
/// transactions. On a reset or flush edge, apparent `valid && ready`
/// handshakes do not count as successful input or output transfers.
/// A full FIFO rejects input even when dequeuing on that edge.
/// Invalid output payload is unspecified. This does not replace [`super::SyncFifo`].
pub struct ParamSyncFifo<const WIDTH: u32 = 32, const DEPTH: u32 = 4>;

impl<const WIDTH: u32, const DEPTH: u32> Elaboratable for ParamSyncFifo<WIDTH, DEPTH> {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("ParamSyncFifo");
        Self::define_module(&mut s, "ParamSyncFifo")?;
        s.finish()
    }
}

impl<const WIDTH: u32, const DEPTH: u32> ParamSyncFifo<WIDTH, DEPTH> {
    /// Define or reuse this specialization in the caller's session without freezing it.
    pub fn define_module(
        session: &mut ElaborateSession,
        name: impl Into<String>,
    ) -> Result<String, Diagnostics> {
        // Validate before allocating, computing widths, or changing the session.
        let mut diagnostics = Vec::new();
        for (parameter, value, maximum) in [("WIDTH", WIDTH, 64), ("DEPTH", DEPTH, 16)] {
            if !(1..=maximum).contains(&value) {
                diagnostics.push(Diagnostic {
                    span: Span::default(),
                    code: "rhdl::E0200".into(),
                    en: format!("ParamSyncFifo {parameter} must be 1..={maximum}; got {value}"),
                    zh: format!("ParamSyncFifo {parameter} 必须为 1..={maximum}；实际为 {value}"),
                });
            }
        }
        if !diagnostics.is_empty() {
            return Err(Diagnostics(diagnostics));
        }
        session.define_module(
            name,
            vec![("WIDTH".into(), WIDTH), ("DEPTH".into(), DEPTH)],
            Self::define_body,
        )
    }

    fn define_body(s: &mut ElaborateSession, _: &[(String, u32)]) -> Result<(), Diagnostics> {
        let sp = Span::default();
        let count_type = GroundType::UInt {
            width: u32::BITS - DEPTH.leading_zeros(),
        };
        let data_type = GroundType::UInt { width: WIDTH };
        let bit_type = GroundType::UInt { width: 1 };
        s.add_input("clk", GroundType::Clock, sp);
        s.add_input("rst", GroundType::Reset, sp);
        for name in ["flush", "input_valid", "output_ready"] {
            s.add_input(name, bit_type.clone(), sp);
        }
        s.add_input("input_data", data_type.clone(), sp);
        for name in ["input_ready", "output_valid"] {
            s.add_output(name, bit_type.clone(), sp);
        }
        s.add_output("output_data", data_type.clone(), sp);
        s.declare_reg("count", count_type.clone(), sp);
        for name in [
            "zero",
            "one",
            "capacity",
            "decremented",
            "after_pop",
            "incremented",
            "normal_count",
            "next_count",
        ] {
            s.declare_wire(name, count_type.clone(), sp);
        }
        for name in ["true_bit", "empty", "full", "push", "pop"] {
            s.declare_wire(name, bit_type.clone(), sp);
        }
        for i in 0..DEPTH {
            s.declare_reg(&format!("slot_{i}"), data_type.clone(), sp);
            s.declare_wire(&format!("index_{i}"), count_type.clone(), sp);
            s.declare_wire(&format!("at_{i}"), bit_type.clone(), sp);
            s.declare_wire(&format!("write_{i}"), bit_type.clone(), sp);
            s.declare_wire(&format!("shift_{i}"), data_type.clone(), sp);
            s.declare_wire(&format!("next_{i}"), data_type.clone(), sp);
        }
        s.begin_combinational(sp);
        s.assign_lit("zero", 0, sp);
        s.assign_lit("one", 1, sp);
        s.assign_lit("capacity", DEPTH as u64, sp);
        s.assign_lit("true_bit", 1, sp);
        s.assign_eq("empty", "count", "zero", sp);
        s.assign_eq("full", "count", "capacity", sp);
        s.assign_xor("input_ready", "full", "true_bit", sp);
        s.assign_xor("output_valid", "empty", "true_bit", sp);
        s.assign_net("output_data", "slot_0", sp);
        s.assign_and("push", "input_valid", "input_ready", sp);
        s.assign_and("pop", "output_valid", "output_ready", sp);
        s.assign_sub("decremented", "count", "one", sp);
        s.assign_mux("after_pop", "pop", "decremented", "count", sp);
        s.assign_add("incremented", "after_pop", "one", sp);
        s.assign_mux("normal_count", "push", "incremented", "after_pop", sp);
        s.assign_mux("next_count", "flush", "zero", "normal_count", sp);
        // The head is always slot zero. Pop shifts remaining words, then push
        // writes at the post-pop tail. This also handles every non-power-of-two depth.
        for i in 0..DEPTH {
            let slot = format!("slot_{i}");
            let shifted = format!("slot_{}", (i + 1).min(DEPTH - 1));
            s.assign_lit(&format!("index_{i}"), i as u64, sp);
            s.assign_eq(&format!("at_{i}"), "after_pop", &format!("index_{i}"), sp);
            s.assign_and(&format!("write_{i}"), "push", &format!("at_{i}"), sp);
            s.assign_mux(&format!("shift_{i}"), "pop", &shifted, &slot, sp);
            s.assign_mux(
                &format!("next_{i}"),
                &format!("write_{i}"),
                "input_data",
                &format!("shift_{i}"),
                sp,
            );
        }
        s.end_process();
        // Builder registers give synchronous reset priority over the D inputs.
        s.begin_sequential(sp);
        s.assign_reg_d_from("count", "next_count", sp);
        for i in 0..DEPTH {
            s.assign_reg_d_from(&format!("slot_{i}"), &format!("next_{i}"), sp);
        }
        s.end_process();
        Ok(())
    }
}
