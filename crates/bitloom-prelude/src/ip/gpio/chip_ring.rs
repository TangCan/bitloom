//! Multi-peripheral / full-chip pad ring (FR136 / Epic 75) beyond [`GpioSocPad`].
//!
//! **R1 Multi-peripheral:** GPIO SocPad face (`pad_out` / `pad_in` / `dir` / masked
//! write) + UART pad side (`uart_tx` / `uart_wr_*`, UartTx-shaped).
//! **R2 Full-chip shape:** 3 banks × 8 = 24-bit pad — bank0=`[7:0]`, bank1=`[15:8]`,
//! bank2=`[23:16]`; public [`CHIP_PAD_RING_BANK_COUNT`] / [`chip_pad_ring_bank_pin_index`].
//! **R3** Ring-level scoreboard — see `fr136_multi_peripheral_full_chip_pad_ring`.
//!
//! Non-goals (NFR59): SPI/I2C/AXI pad mux; drive strength / debounce / analog;
//! third-party VIP binary co-sim. FR128 / FR120 / FR108 / FR98 closes remain valid
//! (NFR56). FR139 `ip/gpio/{base,vip,socpad}` paths unchanged.

#![allow(unused_imports)]
use crate::{
    Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span,
    SynthesizableClosureViolation, diagnose_synthesizable_closure_violations,
};

/// Full-chip pad ring bank count (FR136 R2) — beyond FR128 dual-bank.
pub const CHIP_PAD_RING_BANK_COUNT: usize = 3;
/// Pins per bank (matches FR128 / FR120 bank width).
pub const CHIP_PAD_RING_PINS_PER_BANK: usize = 8;
/// Total GPIO pad width — 3×8 = 24 (≥24 full-chip).
pub const CHIP_PAD_RING_PAD_WIDTH: usize = 24;

/// Public bank/pin → absolute bit index on the ring (`None` if out of range).
pub fn chip_pad_ring_bank_pin_index(bank: usize, pin: usize) -> Option<usize> {
    if bank >= CHIP_PAD_RING_BANK_COUNT || pin >= CHIP_PAD_RING_PINS_PER_BANK {
        None
    } else {
        Some(bank * CHIP_PAD_RING_PINS_PER_BANK + pin)
    }
}

/// Multi-peripheral full-chip pad ring (FR136 / Epic 75).
///
/// Elaboratable product path combining a 24-bit GPIO SocPad face with a UART TX
/// pad side. Distinct from [`super::GpioSocPad`] (FR128 alone ≠ FR136).
pub struct ChipPadRing;

impl Elaboratable for ChipPadRing {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("ChipPadRing");
        s.begin_module("ChipPadRing", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());

        // —— GPIO SocPad face (24-bit / 3-bank) ——
        s.add_input("dir", GroundType::UInt { width: 24 }, Span::default());
        s.add_input("wr_en", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("wr_data", GroundType::UInt { width: 24 }, Span::default());
        s.add_input("wr_mask", GroundType::UInt { width: 24 }, Span::default());
        s.add_input("pad_in", GroundType::UInt { width: 24 }, Span::default());
        s.add_output("pad_out", GroundType::UInt { width: 24 }, Span::default());

        // —— UART pad side (UartTx-shaped ports; prefixed to avoid GPIO clash) ——
        s.add_input("uart_wr_en", GroundType::UInt { width: 1 }, Span::default());
        s.add_input(
            "uart_wr_data",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.add_input(
            "uart_baud_div",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.add_output("uart_tx", GroundType::UInt { width: 1 }, Span::default());
        s.add_output(
            "uart_tx_byte",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.add_output(
            "uart_tx_busy",
            GroundType::UInt { width: 1 },
            Span::default(),
        );

        s.declare_reg("out_r", GroundType::UInt { width: 24 }, Span::default());
        s.declare_reg("u_hold", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg(
            "u_shift_reg",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_reg("u_busy", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("u_bit_idx", GroundType::UInt { width: 4 }, Span::default());
        s.declare_reg("u_baud_cnt", GroundType::UInt { width: 8 }, Span::default());

        for (w, width) in [
            ("c_ffffff", 24u32),
            ("c0_24", 24),
            ("c0_1", 1),
            ("c1_1", 1),
            ("g_not_mask", 24),
            ("g_kept", 24),
            ("g_newt", 24),
            ("g_merged", 24),
            ("g_next_out", 24),
            ("u_c0_4", 4),
            ("u_c1_4", 4),
            ("u_c9_4", 4),
            ("u_c0_8", 8),
            ("u_c1_8", 8),
            ("u_accept", 1),
            ("u_is_start", 1),
            ("u_is_stop", 1),
            ("u_lsb8", 8),
            ("u_data_bit", 1),
            ("u_tx_active", 1),
            ("u_tx_data_or_stop", 1),
            ("u_do_shift", 1),
            ("u_do_shift2", 1),
            ("u_not_start", 1),
            ("u_not_stop", 1),
            ("u_shift_shr", 8),
            ("u_bit_idx_p1", 4),
            ("u_baud_eq", 1),
            ("u_baud_tick", 1),
            ("u_baud_cnt_p1", 8),
            ("u_baud_cnt_busy", 8),
            ("u_baud_cnt_busy_or_idle", 8),
            ("u_next_baud_cnt", 8),
            ("u_next_busy", 1),
            ("u_busy_after_tick", 1),
            ("u_busy_when_busy", 1),
            ("u_busy_when_busy_or_idle", 1),
            ("u_bit_after_tick", 4),
            ("u_bit_when_busy", 4),
            ("u_bit_when_busy_or_idle", 4),
            ("u_next_bit_idx", 4),
            ("u_do_shift_tick", 1),
            ("u_shift_after_tick", 8),
            ("u_next_shift_busy", 8),
            ("u_next_shift_final", 8),
            ("u_next_hold", 8),
        ] {
            s.declare_wire(w, GroundType::UInt { width }, Span::default());
        }

        s.begin_combinational(Span::default());
        s.assign_lit("c_ffffff", 0xff_ffff, Span::default());
        s.assign_lit("c0_24", 0, Span::default());
        s.assign_lit("c0_1", 0, Span::default());
        s.assign_lit("c1_1", 1, Span::default());
        s.assign_lit("u_c0_4", 0, Span::default());
        s.assign_lit("u_c1_4", 1, Span::default());
        s.assign_lit("u_c9_4", 9, Span::default());
        s.assign_lit("u_c0_8", 0, Span::default());
        s.assign_lit("u_c1_8", 1, Span::default());

        // GPIO masked write → pad_out = out_r & dir
        s.assign_xor("g_not_mask", "wr_mask", "c_ffffff", Span::default());
        s.assign_and("g_kept", "out_r", "g_not_mask", Span::default());
        s.assign_and("g_newt", "wr_data", "wr_mask", Span::default());
        s.assign_or("g_merged", "g_kept", "g_newt", Span::default());
        s.assign_mux("g_next_out", "wr_en", "g_merged", "out_r", Span::default());
        s.assign_and("pad_out", "out_r", "dir", Span::default());

        // UART 8N1 (baud_div = clocks/bit − 1); mirror UartTx
        s.assign_mux("u_accept", "u_busy", "c0_1", "uart_wr_en", Span::default());
        s.assign_eq("u_is_start", "u_bit_idx", "u_c0_4", Span::default());
        s.assign_eq("u_is_stop", "u_bit_idx", "u_c9_4", Span::default());
        s.assign_and("u_lsb8", "u_shift_reg", "u_c1_8", Span::default());
        s.assign_eq("u_data_bit", "u_lsb8", "u_c1_8", Span::default());
        s.assign_mux(
            "u_tx_data_or_stop",
            "u_is_stop",
            "c1_1",
            "u_data_bit",
            Span::default(),
        );
        s.assign_mux(
            "u_tx_active",
            "u_is_start",
            "c0_1",
            "u_tx_data_or_stop",
            Span::default(),
        );
        s.assign_mux("uart_tx", "u_busy", "u_tx_active", "c1_1", Span::default());
        s.assign_net("uart_tx_byte", "u_hold", Span::default());
        s.assign_net("uart_tx_busy", "u_busy", Span::default());

        s.assign_eq("u_baud_eq", "u_baud_cnt", "uart_baud_div", Span::default());
        s.assign_and("u_baud_tick", "u_busy", "u_baud_eq", Span::default());
        s.assign_add("u_baud_cnt_p1", "u_baud_cnt", "u_c1_8", Span::default());
        s.assign_mux(
            "u_baud_cnt_busy",
            "u_baud_eq",
            "u_c0_8",
            "u_baud_cnt_p1",
            Span::default(),
        );
        s.assign_mux(
            "u_baud_cnt_busy_or_idle",
            "u_busy",
            "u_baud_cnt_busy",
            "u_c0_8",
            Span::default(),
        );
        s.assign_mux(
            "u_next_baud_cnt",
            "u_accept",
            "u_c0_8",
            "u_baud_cnt_busy_or_idle",
            Span::default(),
        );

        s.assign_xor("u_not_start", "u_is_start", "c1_1", Span::default());
        s.assign_xor("u_not_stop", "u_is_stop", "c1_1", Span::default());
        s.assign_and("u_do_shift", "u_busy", "u_not_start", Span::default());
        s.assign_and("u_do_shift2", "u_do_shift", "u_not_stop", Span::default());
        s.assign_and(
            "u_do_shift_tick",
            "u_do_shift2",
            "u_baud_eq",
            Span::default(),
        );
        s.assign_shr("u_shift_shr", "u_shift_reg", "u_c1_8", Span::default());
        s.assign_add("u_bit_idx_p1", "u_bit_idx", "u_c1_4", Span::default());

        s.assign_mux(
            "u_busy_after_tick",
            "u_is_stop",
            "c0_1",
            "c1_1",
            Span::default(),
        );
        s.assign_mux(
            "u_busy_when_busy",
            "u_baud_tick",
            "u_busy_after_tick",
            "c1_1",
            Span::default(),
        );
        s.assign_mux(
            "u_busy_when_busy_or_idle",
            "u_busy",
            "u_busy_when_busy",
            "c0_1",
            Span::default(),
        );
        s.assign_mux(
            "u_next_busy",
            "u_accept",
            "c1_1",
            "u_busy_when_busy_or_idle",
            Span::default(),
        );

        s.assign_mux(
            "u_bit_after_tick",
            "u_is_stop",
            "u_c0_4",
            "u_bit_idx_p1",
            Span::default(),
        );
        s.assign_mux(
            "u_bit_when_busy",
            "u_baud_tick",
            "u_bit_after_tick",
            "u_bit_idx",
            Span::default(),
        );
        s.assign_mux(
            "u_bit_when_busy_or_idle",
            "u_busy",
            "u_bit_when_busy",
            "u_c0_4",
            Span::default(),
        );
        s.assign_mux(
            "u_next_bit_idx",
            "u_accept",
            "u_c0_4",
            "u_bit_when_busy_or_idle",
            Span::default(),
        );

        s.assign_mux(
            "u_shift_after_tick",
            "u_do_shift_tick",
            "u_shift_shr",
            "u_shift_reg",
            Span::default(),
        );
        s.assign_mux(
            "u_next_shift_busy",
            "u_busy",
            "u_shift_after_tick",
            "u_shift_reg",
            Span::default(),
        );
        s.assign_mux(
            "u_next_shift_final",
            "u_accept",
            "uart_wr_data",
            "u_next_shift_busy",
            Span::default(),
        );
        s.assign_mux(
            "u_next_hold",
            "u_accept",
            "uart_wr_data",
            "u_hold",
            Span::default(),
        );
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_from("out_r", "g_next_out", Span::default());
        s.assign_reg_d_from("u_busy", "u_next_busy", Span::default());
        s.assign_reg_d_from("u_bit_idx", "u_next_bit_idx", Span::default());
        s.assign_reg_d_from("u_shift_reg", "u_next_shift_final", Span::default());
        s.assign_reg_d_from("u_hold", "u_next_hold", Span::default());
        s.assign_reg_d_from("u_baud_cnt", "u_next_baud_cnt", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}
