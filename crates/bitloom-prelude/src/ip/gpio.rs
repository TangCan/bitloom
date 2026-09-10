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
        s.begin_module("Gpio", Span::default());
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
        s.end_module();
        s.finish()
    }
}

/// 8-bit GPIO commercial VIP bank (FR120 / Epic 61): FR108 P1–P4 plus C1–C3.
///
/// **Baseline (FR108):** `dir`, masked `wr_*`, `pad_in` → `pad_out`/`rd_data`.
///
/// **C1 rising-edge IRQ:** `pad_prev` samples `pad_in`; rising = `pad_in & ~prev`;
/// `irq_status` sticky pending; `irq_clear` clears; `irq_out` = 1 iff
/// `(irq_status & irq_en) != 0` (bank OR).
///
/// **C2 open-drain + OE:** per-bit `od`; `pad_oe = dir & ~(od & out)`;
/// `pad_out = out & pad_oe` (OD drives low only; Hi-Z when `od & out`).
///
/// **C3 atomic set/clear:** after optional masked write, `set_en`/`set_data` then
/// `clr_en`/`clr_data` (same-cycle priority: wr → set → clr).
///
/// Non-goals (NFR51): full SoC pad ring, commercial co-sim scoreboard, debounce /
/// drive strength, level/falling/dual-edge IRQ modes, analog. FR98 / FR108 closes
/// remain valid (NFR48). Lives in `ip/gpio.rs` (FR131 protocol split).
pub struct GpioVip;

impl Elaboratable for GpioVip {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("GpioVip");
        s.begin_module("GpioVip", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("dir", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("wr_en", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("wr_data", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("wr_mask", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("set_en", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("set_data", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("clr_en", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("clr_data", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("od", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("pad_in", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("irq_en", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("irq_clear", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("pad_out", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("pad_oe", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("rd_data", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("irq_status", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("irq_out", GroundType::UInt { width: 1 }, Span::default());

        s.declare_reg("out_r", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("pad_prev_r", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("irq_pend_r", GroundType::UInt { width: 8 }, Span::default());

        for w in [
            "c_ff",
            "c0_8",
            "c0_1",
            "c1_1",
            "not_mask",
            "kept",
            "newt",
            "merged",
            "after_wr",
            "or_set",
            "after_set",
            "not_clr",
            "and_clr",
            "next_out",
            "not_dir",
            "from_out",
            "from_pad",
            "od_and_out",
            "not_od_drive",
            "not_prev",
            "rising",
            "pend_or",
            "not_clear",
            "next_pend",
            "armed",
        ] {
            let width = if w.ends_with("_1") { 1 } else { 8 };
            s.declare_wire(w, GroundType::UInt { width }, Span::default());
        }

        s.begin_combinational(Span::default());
        s.assign_lit("c_ff", 0xff, Span::default());
        s.assign_lit("c0_8", 0, Span::default());
        s.assign_lit("c0_1", 0, Span::default());
        s.assign_lit("c1_1", 1, Span::default());

        // P3 masked write → C3 set → C3 clear (documented same-cycle priority)
        s.assign_xor("not_mask", "wr_mask", "c_ff", Span::default());
        s.assign_and("kept", "out_r", "not_mask", Span::default());
        s.assign_and("newt", "wr_data", "wr_mask", Span::default());
        s.assign_or("merged", "kept", "newt", Span::default());
        s.assign_mux("after_wr", "wr_en", "merged", "out_r", Span::default());
        s.assign_or("or_set", "after_wr", "set_data", Span::default());
        s.assign_mux("after_set", "set_en", "or_set", "after_wr", Span::default());
        s.assign_xor("not_clr", "clr_data", "c_ff", Span::default());
        s.assign_and("and_clr", "after_set", "not_clr", Span::default());
        s.assign_mux(
            "next_out",
            "clr_en",
            "and_clr",
            "after_set",
            Span::default(),
        );

        // C2 open-drain OE + pad_out
        s.assign_and("od_and_out", "od", "out_r", Span::default());
        s.assign_xor("not_od_drive", "od_and_out", "c_ff", Span::default());
        s.assign_and("pad_oe", "dir", "not_od_drive", Span::default());
        s.assign_and("pad_out", "out_r", "pad_oe", Span::default());

        // P1/P2 rd_data
        s.assign_xor("not_dir", "dir", "c_ff", Span::default());
        s.assign_and("from_out", "out_r", "dir", Span::default());
        s.assign_and("from_pad", "pad_in", "not_dir", Span::default());
        s.assign_or("rd_data", "from_out", "from_pad", Span::default());

        // C1 rising-edge IRQ
        s.assign_xor("not_prev", "pad_prev_r", "c_ff", Span::default());
        s.assign_and("rising", "pad_in", "not_prev", Span::default());
        s.assign_or("pend_or", "irq_pend_r", "rising", Span::default());
        s.assign_xor("not_clear", "irq_clear", "c_ff", Span::default());
        s.assign_and("next_pend", "pend_or", "not_clear", Span::default());
        s.assign_net("irq_status", "irq_pend_r", Span::default());
        s.assign_and("armed", "irq_pend_r", "irq_en", Span::default());
        s.assign_mux("irq_out", "armed", "c1_1", "c0_1", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_from("out_r", "next_out", Span::default());
        s.assign_reg_d_from("pad_prev_r", "pad_in", Span::default());
        s.assign_reg_d_from("irq_pend_r", "next_pend", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}
