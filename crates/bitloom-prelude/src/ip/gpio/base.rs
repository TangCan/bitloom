#![allow(unused_imports)]
use crate::{
    Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span,
    SynthesizableClosureViolation, diagnose_synthesizable_closure_violations,
};

/// 8-bit GPIO bank near-VIP (FR108 / Epic 50): direction, masked write, pad R/W.
///
/// Ports: `dir` (1=out), `wr_en`/`wr_data`/`wr_mask`, `pad_in` → `pad_out`/`rd_data`.
/// `rd_data = (out & dir) | (pad_in & ~dir)`; `pad_out = out & dir`.
///
/// Non-goals (NFR47): commercial VIP co-sim, IRQ controller, full SoC pad ring,
/// undeclared open-drain/analog. FR98 UART/SPI/I2C/AXI MVP closes remain valid.
pub struct Gpio;

impl Elaboratable for Gpio {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("Gpio");
        Self::define_module(&mut s, "Gpio")?;
        s.finish()
    }
}

impl Gpio {
    /// Define the legacy eight-bit GPIO bank in a shared session (FR194).
    pub fn define_module(
        session: &mut ElaborateSession,
        name: impl Into<String>,
    ) -> Result<String, Diagnostics> {
        session.define_module(name, vec![], Self::define_body)
    }

    fn define_body(s: &mut ElaborateSession, _: &[(String, u32)]) -> Result<(), Diagnostics> {
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("dir", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("wr_en", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("wr_data", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("wr_mask", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("pad_in", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("pad_out", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("rd_data", GroundType::UInt { width: 8 }, Span::default());

        s.declare_reg("out_r", GroundType::UInt { width: 8 }, Span::default());
        for w in [
            "c_ff", "not_mask", "kept", "newt", "merged", "not_dir", "from_out", "from_pad",
        ] {
            s.declare_wire(w, GroundType::UInt { width: 8 }, Span::default());
        }

        s.begin_combinational(Span::default());
        s.assign_lit("c_ff", 0xff, Span::default());
        s.assign_and("pad_out", "out_r", "dir", Span::default());
        s.assign_xor("not_dir", "dir", "c_ff", Span::default());
        s.assign_and("from_out", "out_r", "dir", Span::default());
        s.assign_and("from_pad", "pad_in", "not_dir", Span::default());
        s.assign_or("rd_data", "from_out", "from_pad", Span::default());
        s.assign_xor("not_mask", "wr_mask", "c_ff", Span::default());
        s.assign_and("kept", "out_r", "not_mask", Span::default());
        s.assign_and("newt", "wr_data", "wr_mask", Span::default());
        s.assign_or("merged", "kept", "newt", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_mux("out_r", "wr_en", "merged", "out_r", Span::default());
        s.end_process();
        Ok(())
    }
}
