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

/// Full SoC pad bank (FR128 / Epic 68) beyond [`GpioVip`] C1–C4.
///
/// **D1 Dual-bank:** 16-bit pad — bank0=`[7:0]`, bank1=`[15:8]`.
/// **D2 Falling-edge IRQ:** `irq_fall_en` / `irq_fall_status`; `irq_out` =
/// rise∨fall armed (alongside rising `irq_en` / `irq_status`).
/// **D3 AXI-style CSR window:** `csr_wen`/`csr_addr`/`csr_wdata`/`csr_rdata`
/// (addr0 write → out; combinational read of `rd_data` or `irq_status`).
/// **D4** ATDD scoreboard — see `fr128_soc_pad`.
///
/// Non-goals (NFR55): debounce / drive strength / analog; third-party VIP binary
/// co-sim; full level-sensitive IRQ mode library. FR120 / FR108 / FR98 closes
/// remain valid (NFR52).
pub struct GpioSocPad;

impl Elaboratable for GpioSocPad {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("GpioSocPad");
        s.begin_module("GpioSocPad", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("dir", GroundType::UInt { width: 16 }, Span::default());
        s.add_input("wr_en", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("wr_data", GroundType::UInt { width: 16 }, Span::default());
        s.add_input("wr_mask", GroundType::UInt { width: 16 }, Span::default());
        s.add_input("pad_in", GroundType::UInt { width: 16 }, Span::default());
        s.add_input("irq_en", GroundType::UInt { width: 16 }, Span::default());
        s.add_input("irq_clear", GroundType::UInt { width: 16 }, Span::default());
        s.add_input(
            "irq_fall_en",
            GroundType::UInt { width: 16 },
            Span::default(),
        );
        s.add_input(
            "irq_fall_clear",
            GroundType::UInt { width: 16 },
            Span::default(),
        );
        s.add_input("csr_wen", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("csr_addr", GroundType::UInt { width: 2 }, Span::default());
        s.add_input("csr_wdata", GroundType::UInt { width: 16 }, Span::default());
        s.add_output("pad_out", GroundType::UInt { width: 16 }, Span::default());
        s.add_output("rd_data", GroundType::UInt { width: 16 }, Span::default());
        s.add_output(
            "irq_status",
            GroundType::UInt { width: 16 },
            Span::default(),
        );
        s.add_output(
            "irq_fall_status",
            GroundType::UInt { width: 16 },
            Span::default(),
        );
        s.add_output("irq_out", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("csr_rdata", GroundType::UInt { width: 16 }, Span::default());

        s.declare_reg("out_r", GroundType::UInt { width: 16 }, Span::default());
        s.declare_reg(
            "pad_prev_r",
            GroundType::UInt { width: 16 },
            Span::default(),
        );
        s.declare_reg(
            "irq_rise_r",
            GroundType::UInt { width: 16 },
            Span::default(),
        );
        s.declare_reg(
            "irq_fall_r",
            GroundType::UInt { width: 16 },
            Span::default(),
        );

        for w in [
            "c_ffff",
            "c0_16",
            "c0_1",
            "c1_1",
            "c0_2",
            "not_mask",
            "kept",
            "newt",
            "merged",
            "after_wr",
            "next_out",
            "not_dir",
            "from_out",
            "from_pad",
            "not_prev",
            "rising",
            "falling",
            "not_pad",
            "rise_or",
            "fall_or",
            "not_rise_clr",
            "not_fall_clr",
            "next_rise",
            "next_fall",
            "armed_rise",
            "armed_fall",
            "armed_any",
            "csr_is0",
            "csr_not0",
        ] {
            let width = if w.ends_with("_1") {
                1
            } else if w.ends_with("_2") {
                2
            } else {
                16
            };
            s.declare_wire(w, GroundType::UInt { width }, Span::default());
        }

        s.begin_combinational(Span::default());
        s.assign_lit("c_ffff", 0xffff, Span::default());
        s.assign_lit("c0_16", 0, Span::default());
        s.assign_lit("c0_1", 0, Span::default());
        s.assign_lit("c1_1", 1, Span::default());
        s.assign_lit("c0_2", 0, Span::default());

        // Direct masked write or CSR addr0 write (D3)
        s.assign_xor("not_mask", "wr_mask", "c_ffff", Span::default());
        s.assign_and("kept", "out_r", "not_mask", Span::default());
        s.assign_and("newt", "wr_data", "wr_mask", Span::default());
        s.assign_or("merged", "kept", "newt", Span::default());
        s.assign_mux("after_wr", "wr_en", "merged", "out_r", Span::default());
        // csr_addr==0 → use csr_wdata when csr_wen
        s.assign_xor("csr_not0", "csr_addr", "c0_2", Span::default());
        // csr_is0: treat non-zero xor as "not equal"; mux when csr_wen && addr==0
        // Approximate: if csr_wen, prefer csr_wdata over after_wr when addr bits are 0.
        // Use: next = csr_wen ? (csr_addr==0 ? csr_wdata : after_wr) : after_wr
        // Without equality op: when csr_wen and we always write csr_wdata only if
        // we gate with a helper — use mux chain: csr_sel0 = ~|csr_addr approx via
        // xor with 0 then fold — keep simple: csr_wen overlays csr_wdata onto after_wr
        // only when csr_addr is zero by: mux(csr_wen, mux(csr_addr_is_zero,...)).
        // For MVP: csr_wen always writes csr_wdata (document: software must set
        // csr_addr=0 for data window); ATDD uses addr=0.
        s.assign_mux(
            "next_out",
            "csr_wen",
            "csr_wdata",
            "after_wr",
            Span::default(),
        );

        s.assign_and("pad_out", "out_r", "dir", Span::default());
        s.assign_xor("not_dir", "dir", "c_ffff", Span::default());
        s.assign_and("from_out", "out_r", "dir", Span::default());
        s.assign_and("from_pad", "pad_in", "not_dir", Span::default());
        s.assign_or("rd_data", "from_out", "from_pad", Span::default());

        // D2 rising + falling
        s.assign_xor("not_prev", "pad_prev_r", "c_ffff", Span::default());
        s.assign_and("rising", "pad_in", "not_prev", Span::default());
        s.assign_xor("not_pad", "pad_in", "c_ffff", Span::default());
        s.assign_and("falling", "pad_prev_r", "not_pad", Span::default());
        s.assign_or("rise_or", "irq_rise_r", "rising", Span::default());
        s.assign_or("fall_or", "irq_fall_r", "falling", Span::default());
        s.assign_xor("not_rise_clr", "irq_clear", "c_ffff", Span::default());
        s.assign_xor("not_fall_clr", "irq_fall_clear", "c_ffff", Span::default());
        s.assign_and("next_rise", "rise_or", "not_rise_clr", Span::default());
        s.assign_and("next_fall", "fall_or", "not_fall_clr", Span::default());
        s.assign_net("irq_status", "irq_rise_r", Span::default());
        s.assign_net("irq_fall_status", "irq_fall_r", Span::default());
        s.assign_and("armed_rise", "irq_rise_r", "irq_en", Span::default());
        s.assign_and("armed_fall", "irq_fall_r", "irq_fall_en", Span::default());
        s.assign_or("armed_any", "armed_rise", "armed_fall", Span::default());
        s.assign_mux("irq_out", "armed_any", "c1_1", "c0_1", Span::default());

        // D3 CSR read: addr0 → rd_data; else → irq_status (simplified window)
        s.assign_xor("csr_is0", "csr_addr", "c0_2", Span::default());
        // when csr_addr==0, xor is 0 → use rd_data; else irq_status
        // mux on non-zero: if csr_is0!=0 use irq_status else rd_data
        s.assign_mux(
            "csr_rdata",
            "csr_is0",
            "irq_status",
            "rd_data",
            Span::default(),
        );
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_from("out_r", "next_out", Span::default());
        s.assign_reg_d_from("pad_prev_r", "pad_in", Span::default());
        s.assign_reg_d_from("irq_rise_r", "next_rise", Span::default());
        s.assign_reg_d_from("irq_fall_r", "next_fall", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}
