//! First-class IP (FR37 / FR48 / FR82 / FR89 / FR98 UART+SPI+I2C): SyncFifo,
//! UartTx, UartRx, SpiMaster, I2cMaster, Axi4LiteSlave, black-box; plus FR77
//! overlay [`Crc8Lut`] (Epic 29.3).
//!
//! Epic 34 / FR82 deepens five classes to **non-stub** synthesizable baselines.
//! Epic 38 / FR89 deepens UartTx programmable baud. Epic 43 / FR98 Stories
//! 43.2–43.4 add UART/SPI/I2C near-VIP; AXI near-VIP remains Story 43.5.
//! Those IP APIs take **no** generator closures. Design crates reach IP via
//! `bitloom_prelude::ip` only.
//!
//! **FR77 / Cap-R-63:** [`Crc8Lut`] accepts elaborate-time table closures
//! (`elaborate_with_table_fn`) on top of the Epic 27 Mem-init path; default poly
//! needs no closure. Closures dissolve before freeze (NFR36); synthesizable leg
//! uses [`SynthesizableClosure`] checks (D1).

use crate::{
    Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span,
    SynthesizableClosureViolation, diagnose_synthesizable_closure_violations,
};

/// Depth-4 single-clock sync FIFO with `wr_en`/`rd_en` and `full`/`empty` (FR82).
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

/// UART TX 8N1 bit-bang with programmable baud divider — FR82 + FR89; FR98 pair with [`UartRx`].
///
/// Accepts a byte only when `!tx_busy`; drives serial `tx` (idle high) through
/// start + 8 data (LSB first) + stop. `tx_byte` holds the latched payload.
///
/// `baud_div` (8-bit): clocks-per-bit **minus one**. `0` ⇒ 1 clk/bit (FR82
/// baud=`clk` compatible; unset sim inputs also read as 0). `N>0` ⇒ each bit
/// held for `N+1` clocks before advancing the frame.
///
/// **FR98 full-duplex contract:** instantiate [`UartTx`] + [`UartRx`] together
/// (independent `tx`/`rx` lines). On-chip half-duplex mux is a non-goal.
///
/// Non-goals: parity, FIFO'd TX, fractional baud / baud tables, flow control,
/// IrDA, commercial VIP co-sim, generator closures (Epic 29).
pub struct UartTx;

impl Elaboratable for UartTx {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("UartTx");
        s.begin_module("UartTx", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("wr_en", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("wr_data", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("baud_div", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("tx", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("tx_byte", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("tx_busy", GroundType::UInt { width: 1 }, Span::default());

        s.declare_reg("hold", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("shift_reg", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("busy", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("bit_idx", GroundType::UInt { width: 4 }, Span::default());
        s.declare_reg("baud_cnt", GroundType::UInt { width: 8 }, Span::default());

        s.declare_wire("c0_1", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("c1_1", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("c0_4", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire("c1_4", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire("c9_4", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire("c0_8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("c1_8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("accept", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("is_start", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("is_stop", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("lsb8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("data_bit", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("tx_active", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire(
            "tx_data_or_stop",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.declare_wire("do_shift", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("do_shift2", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("not_start", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("not_stop", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("shift_shr", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("bit_idx_p1", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire("baud_eq", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("baud_tick", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire(
            "baud_cnt_p1",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire(
            "baud_cnt_busy",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire(
            "baud_cnt_busy_or_idle",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire(
            "next_baud_cnt",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire("next_busy", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire(
            "busy_after_tick",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.declare_wire(
            "busy_when_busy",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.declare_wire(
            "busy_when_busy_or_idle",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.declare_wire(
            "bit_after_tick",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.declare_wire(
            "bit_when_busy",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.declare_wire(
            "bit_when_busy_or_idle",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.declare_wire(
            "next_bit_idx",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.declare_wire(
            "do_shift_tick",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.declare_wire(
            "shift_after_tick",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire(
            "next_shift_busy",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire(
            "next_shift_final",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire("next_hold", GroundType::UInt { width: 8 }, Span::default());

        s.begin_combinational(Span::default());
        s.assign_lit("c0_1", 0, Span::default());
        s.assign_lit("c1_1", 1, Span::default());
        s.assign_lit("c0_4", 0, Span::default());
        s.assign_lit("c1_4", 1, Span::default());
        s.assign_lit("c9_4", 9, Span::default());
        s.assign_lit("c0_8", 0, Span::default());
        s.assign_lit("c1_8", 1, Span::default());
        // accept = wr_en && !busy
        s.assign_mux("accept", "busy", "c0_1", "wr_en", Span::default());
        s.assign_eq("is_start", "bit_idx", "c0_4", Span::default());
        s.assign_eq("is_stop", "bit_idx", "c9_4", Span::default());
        s.assign_and("lsb8", "shift_reg", "c1_8", Span::default());
        s.assign_eq("data_bit", "lsb8", "c1_8", Span::default());
        s.assign_mux(
            "tx_data_or_stop",
            "is_stop",
            "c1_1",
            "data_bit",
            Span::default(),
        );
        s.assign_mux(
            "tx_active",
            "is_start",
            "c0_1",
            "tx_data_or_stop",
            Span::default(),
        );
        // idle line high when !busy
        s.assign_mux("tx", "busy", "tx_active", "c1_1", Span::default());
        s.assign_net("tx_byte", "hold", Span::default());
        s.assign_net("tx_busy", "busy", Span::default());

        // baud_div = clocks/bit − 1; baud_tick advances one bit period
        s.assign_eq("baud_eq", "baud_cnt", "baud_div", Span::default());
        s.assign_and("baud_tick", "busy", "baud_eq", Span::default());
        s.assign_add("baud_cnt_p1", "baud_cnt", "c1_8", Span::default());
        s.assign_mux(
            "baud_cnt_busy",
            "baud_eq",
            "c0_8",
            "baud_cnt_p1",
            Span::default(),
        );
        s.assign_mux(
            "baud_cnt_busy_or_idle",
            "busy",
            "baud_cnt_busy",
            "c0_8",
            Span::default(),
        );
        s.assign_mux(
            "next_baud_cnt",
            "accept",
            "c0_8",
            "baud_cnt_busy_or_idle",
            Span::default(),
        );

        s.assign_xor("not_start", "is_start", "c1_1", Span::default());
        s.assign_xor("not_stop", "is_stop", "c1_1", Span::default());
        s.assign_and("do_shift", "busy", "not_start", Span::default());
        s.assign_and("do_shift2", "do_shift", "not_stop", Span::default());
        s.assign_and("do_shift_tick", "do_shift2", "baud_eq", Span::default());
        s.assign_shr("shift_shr", "shift_reg", "c1_8", Span::default());
        s.assign_add("bit_idx_p1", "bit_idx", "c1_4", Span::default());

        // Frame advance only on baud_tick (baud_div=0 ⇒ every busy cycle).
        s.assign_mux(
            "busy_after_tick",
            "is_stop",
            "c0_1",
            "c1_1",
            Span::default(),
        );
        s.assign_mux(
            "busy_when_busy",
            "baud_tick",
            "busy_after_tick",
            "c1_1",
            Span::default(),
        );
        s.assign_mux(
            "busy_when_busy_or_idle",
            "busy",
            "busy_when_busy",
            "c0_1",
            Span::default(),
        );
        s.assign_mux(
            "next_busy",
            "accept",
            "c1_1",
            "busy_when_busy_or_idle",
            Span::default(),
        );

        s.assign_mux(
            "bit_after_tick",
            "is_stop",
            "c0_4",
            "bit_idx_p1",
            Span::default(),
        );
        s.assign_mux(
            "bit_when_busy",
            "baud_tick",
            "bit_after_tick",
            "bit_idx",
            Span::default(),
        );
        s.assign_mux(
            "bit_when_busy_or_idle",
            "busy",
            "bit_when_busy",
            "c0_4",
            Span::default(),
        );
        s.assign_mux(
            "next_bit_idx",
            "accept",
            "c0_4",
            "bit_when_busy_or_idle",
            Span::default(),
        );

        s.assign_mux(
            "shift_after_tick",
            "do_shift_tick",
            "shift_shr",
            "shift_reg",
            Span::default(),
        );
        s.assign_mux(
            "next_shift_busy",
            "busy",
            "shift_after_tick",
            "shift_reg",
            Span::default(),
        );
        s.assign_mux(
            "next_shift_final",
            "accept",
            "wr_data",
            "next_shift_busy",
            Span::default(),
        );
        s.assign_mux("next_hold", "accept", "wr_data", "hold", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_from("busy", "next_busy", Span::default());
        s.assign_reg_d_from("bit_idx", "next_bit_idx", Span::default());
        s.assign_reg_d_from("shift_reg", "next_shift_final", Span::default());
        s.assign_reg_d_from("hold", "next_hold", Span::default());
        s.assign_reg_d_from("baud_cnt", "next_baud_cnt", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

/// UART RX 8N1 bit-bang with programmable baud divider — FR98 near-VIP (Story 43.2).
///
/// Idle line high. Falling edge on `rx` while `!rx_busy` starts a frame. Skips a
/// dedicated start-bit wait (edge already seen) and samples **8 data bits
/// LSB-first** then one stop-bit period using the same `baud_div` semantics as
/// [`UartTx`] (clk/bit − 1; sample at end of each baud period).
///
/// Outputs: `rd_data` (latched byte), `rd_valid` (**one-cycle pulse** when the
/// stop period completes), `rx_busy` while a frame is in progress.
///
/// **FR98 full-duplex contract:** pair with [`UartTx`] on independent wires.
/// Non-goals: framing-error ports, parity, FIFO'd RX, flow control, IrDA,
/// half-duplex direction control, generator closures (Epic 29).
pub struct UartRx;

impl Elaboratable for UartRx {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("UartRx");
        s.begin_module("UartRx", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("rx", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("baud_div", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("rd_data", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("rd_valid", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("rx_busy", GroundType::UInt { width: 1 }, Span::default());

        s.declare_reg("rx_prev", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("busy", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("bit_idx", GroundType::UInt { width: 4 }, Span::default());
        s.declare_reg("baud_cnt", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("shift_reg", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("hold", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("valid", GroundType::UInt { width: 1 }, Span::default());

        s.declare_wire("c0_1", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("c1_1", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("c0_4", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire("c1_4", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire("c9_4", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire("c0_8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("c1_8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("c80_8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire(
            "rx_prev_high",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.declare_wire("rx_low", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("fall", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("start", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("is_stop", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("not_stop", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("baud_eq", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("baud_tick", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire(
            "baud_cnt_p1",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire(
            "baud_cnt_busy",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire(
            "baud_cnt_busy_or_idle",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire(
            "next_baud_cnt",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire("do_sample", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("do_finish", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("shift_shr", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("rx8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("shift_in", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("bit_idx_p1", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire(
            "busy_after_tick",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.declare_wire(
            "busy_when_busy",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.declare_wire(
            "busy_when_busy_or_idle",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.declare_wire("next_busy", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire(
            "bit_after_tick",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.declare_wire(
            "bit_when_busy",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.declare_wire(
            "bit_when_busy_or_idle",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.declare_wire(
            "next_bit_idx",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.declare_wire(
            "shift_after_sample",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire(
            "next_shift_busy",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire("next_shift", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("next_hold", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("next_valid", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire(
            "next_rx_prev",
            GroundType::UInt { width: 1 },
            Span::default(),
        );

        s.begin_combinational(Span::default());
        s.assign_lit("c0_1", 0, Span::default());
        s.assign_lit("c1_1", 1, Span::default());
        s.assign_lit("c0_4", 0, Span::default());
        s.assign_lit("c1_4", 1, Span::default());
        s.assign_lit("c9_4", 9, Span::default());
        s.assign_lit("c0_8", 0, Span::default());
        s.assign_lit("c1_8", 1, Span::default());
        s.assign_lit("c80_8", 0x80, Span::default());

        s.assign_eq("rx_prev_high", "rx_prev", "c1_1", Span::default());
        s.assign_eq("rx_low", "rx", "c0_1", Span::default());
        s.assign_and("fall", "rx_prev_high", "rx_low", Span::default());
        // start = !busy && falling edge
        s.assign_mux("start", "busy", "c0_1", "fall", Span::default());

        s.assign_eq("is_stop", "bit_idx", "c9_4", Span::default());

        s.assign_eq("baud_eq", "baud_cnt", "baud_div", Span::default());
        s.assign_and("baud_tick", "busy", "baud_eq", Span::default());
        s.assign_add("baud_cnt_p1", "baud_cnt", "c1_8", Span::default());
        s.assign_mux(
            "baud_cnt_busy",
            "baud_eq",
            "c0_8",
            "baud_cnt_p1",
            Span::default(),
        );
        s.assign_mux(
            "baud_cnt_busy_or_idle",
            "busy",
            "baud_cnt_busy",
            "c0_8",
            Span::default(),
        );
        s.assign_mux(
            "next_baud_cnt",
            "start",
            "c0_8",
            "baud_cnt_busy_or_idle",
            Span::default(),
        );

        // Sample data bits (bit_idx 1..8) at end of baud period; finish on stop.
        s.assign_xor("not_stop", "is_stop", "c1_1", Span::default());
        s.assign_and("do_sample", "baud_tick", "not_stop", Span::default());
        s.assign_and("do_finish", "baud_tick", "is_stop", Span::default());

        s.assign_shr("shift_shr", "shift_reg", "c1_8", Span::default());
        s.assign_mux("rx8", "rx", "c80_8", "c0_8", Span::default());
        s.assign_or("shift_in", "shift_shr", "rx8", Span::default());
        s.assign_add("bit_idx_p1", "bit_idx", "c1_4", Span::default());

        // On stop baud_tick → idle; else stay busy through data/stop.
        s.assign_mux(
            "busy_after_tick",
            "is_stop",
            "c0_1",
            "c1_1",
            Span::default(),
        );
        s.assign_mux(
            "busy_when_busy",
            "baud_tick",
            "busy_after_tick",
            "c1_1",
            Span::default(),
        );
        s.assign_mux(
            "busy_when_busy_or_idle",
            "busy",
            "busy_when_busy",
            "c0_1",
            Span::default(),
        );
        s.assign_mux(
            "next_busy",
            "start",
            "c1_1",
            "busy_when_busy_or_idle",
            Span::default(),
        );

        // start → bit_idx=1 (edge consumed start); stop tick → 0; else +1 on tick
        s.assign_mux(
            "bit_after_tick",
            "is_stop",
            "c0_4",
            "bit_idx_p1",
            Span::default(),
        );
        s.assign_mux(
            "bit_when_busy",
            "baud_tick",
            "bit_after_tick",
            "bit_idx",
            Span::default(),
        );
        s.assign_mux(
            "bit_when_busy_or_idle",
            "busy",
            "bit_when_busy",
            "c0_4",
            Span::default(),
        );
        s.assign_mux(
            "next_bit_idx",
            "start",
            "c1_4",
            "bit_when_busy_or_idle",
            Span::default(),
        );

        s.assign_mux(
            "shift_after_sample",
            "do_sample",
            "shift_in",
            "shift_reg",
            Span::default(),
        );
        s.assign_mux(
            "next_shift_busy",
            "busy",
            "shift_after_sample",
            "shift_reg",
            Span::default(),
        );
        s.assign_mux(
            "next_shift",
            "start",
            "c0_8",
            "next_shift_busy",
            Span::default(),
        );

        // Capture assembled byte at stop completion (after 8 samples).
        s.assign_mux(
            "next_hold",
            "do_finish",
            "shift_reg",
            "hold",
            Span::default(),
        );
        s.assign_mux("next_valid", "do_finish", "c1_1", "c0_1", Span::default());
        s.assign_net("next_rx_prev", "rx", Span::default());

        s.assign_net("rd_data", "hold", Span::default());
        s.assign_net("rd_valid", "valid", Span::default());
        s.assign_net("rx_busy", "busy", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_from("rx_prev", "next_rx_prev", Span::default());
        s.assign_reg_d_from("busy", "next_busy", Span::default());
        s.assign_reg_d_from("bit_idx", "next_bit_idx", Span::default());
        s.assign_reg_d_from("baud_cnt", "next_baud_cnt", Span::default());
        s.assign_reg_d_from("shift_reg", "next_shift", Span::default());
        s.assign_reg_d_from("hold", "next_hold", Span::default());
        s.assign_reg_d_from("valid", "next_valid", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

/// SPI **master** near-VIP (FR98 / Epic 43.3) — configurable CPOL/CPHA + multi-byte frames.
///
/// Accepts `start && !busy`; latches `tx_data` / effective `byte_count` (`0`≡1);
/// drives `cs_n` low for the whole frame; half-period `sclk` (`idle = cpol`);
/// MSB-first; samples `miso` into `rx_data` with per-byte `rx_valid`.
///
/// **FR98 S1–S4:** four SPI modes via `cpol`/`cpha`; Master multi-byte; docs in `docs/ip/`.
/// **Non-goals:** DMA, multi-CS array, slave, LSB-first, non-8×N word sizes, generator closures.
pub struct SpiMaster;

impl Elaboratable for SpiMaster {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("SpiMaster");
        s.begin_module("SpiMaster", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("start", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("tx_data", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("miso", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("cpol", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("cpha", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("byte_count", GroundType::UInt { width: 3 }, Span::default());
        s.add_output("mosi_byte", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("rx_data", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("rx_valid", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("busy", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("cs_n", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("sclk", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("mosi", GroundType::UInt { width: 1 }, Span::default());

        s.declare_reg("hold", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("shift", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("rx_shift", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("rx_hold", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("busy_r", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("bit_idx", GroundType::UInt { width: 4 }, Span::default());
        s.declare_reg("half", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("bytes_left", GroundType::UInt { width: 3 }, Span::default());
        s.declare_reg("valid_r", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("mosi_r", GroundType::UInt { width: 1 }, Span::default());

        for (n, w) in [
            ("c0_1", 1u32),
            ("c1_1", 1),
            ("c0_3", 3),
            ("c1_3", 3),
            ("c0_4", 4),
            ("c1_4", 4),
            ("c7_4", 4),
            ("c0_8", 8),
            ("c1_8", 8),
            ("cff_8", 8),
            ("c80_8", 8),
            ("accept", 1),
            ("bc_is0", 1),
            ("eff_count", 3),
            ("cpha0", 1),
            ("cpha1", 1),
            ("half0", 1),
            ("is_last_bit", 1),
            ("is_last_byte", 1),
            ("sclk_busy", 1),
            ("tx_msb_m", 8),
            ("tx_msb", 1),
            ("sh_msb_m", 8),
            ("sh_msb", 1),
            ("sh8_msb_m", 8),
            ("sh8_msb", 1),
            ("accept_mosi", 1),
            ("shift_shl", 8),
            ("shift_shl8", 8),
            ("rx_shl", 8),
            ("rx_shl8", 8),
            ("miso8", 8),
            ("rx_sampled", 8),
            ("bit_p1", 4),
            ("bytes_m1", 3),
            ("do_lead", 1),
            ("do_trail", 1),
            ("sample_lead", 1),
            ("launch_lead", 1),
            ("shift_trail", 1),
            ("sample_trail", 1),
            ("finish_byte", 1),
            ("cont_frame", 1),
            ("end_frame", 1),
            ("lead_shift", 8),
            ("lead_mosi", 1),
            ("lead_rxsh", 8),
            ("tr_shift0", 8),
            ("tr_mosi0", 1),
            ("tr_rxsh0", 8),
            ("tr_shift", 8),
            ("tr_mosi", 1),
            ("tr_hold", 8),
            ("tr_bytes", 3),
            ("tr_bit", 4),
            ("tr_busy", 1),
            ("tr_valid", 1),
            ("tr_rxh", 8),
            ("tr_rxsh", 8),
            ("b_half", 1),
            ("b_bit", 4),
            ("b_bytes", 3),
            ("b_hold", 8),
            ("b_shift", 8),
            ("b_rxsh", 8),
            ("b_rxh", 8),
            ("b_valid", 1),
            ("b_mosi", 1),
            ("b_busy", 1),
            ("nb_busy", 1),
            ("nb_half", 1),
            ("nb_bit", 4),
            ("nb_bytes", 3),
            ("nb_shift", 8),
            ("nb_rxsh", 8),
            ("nb_hold", 8),
            ("nb_rxh", 8),
            ("nb_valid", 1),
            ("nb_mosi", 1),
            ("n_busy", 1),
            ("n_half", 1),
            ("n_bit", 4),
            ("n_bytes", 3),
            ("n_shift", 8),
            ("n_rxsh", 8),
            ("n_hold", 8),
            ("n_rxh", 8),
            ("n_valid", 1),
            ("n_mosi", 1),
        ] {
            s.declare_wire(n, GroundType::UInt { width: w }, Span::default());
        }

        s.begin_combinational(Span::default());
        s.assign_lit("c0_1", 0, Span::default());
        s.assign_lit("c1_1", 1, Span::default());
        s.assign_lit("c0_3", 0, Span::default());
        s.assign_lit("c1_3", 1, Span::default());
        s.assign_lit("c0_4", 0, Span::default());
        s.assign_lit("c1_4", 1, Span::default());
        s.assign_lit("c7_4", 7, Span::default());
        s.assign_lit("c0_8", 0, Span::default());
        s.assign_lit("c1_8", 1, Span::default());
        s.assign_lit("cff_8", 0xFF, Span::default());
        s.assign_lit("c80_8", 0x80, Span::default());

        s.assign_mux("accept", "busy_r", "c0_1", "start", Span::default());
        s.assign_eq("bc_is0", "byte_count", "c0_3", Span::default());
        s.assign_mux("eff_count", "bc_is0", "c1_3", "byte_count", Span::default());
        s.assign_eq("cpha0", "cpha", "c0_1", Span::default());
        s.assign_xor("cpha1", "cpha0", "c1_1", Span::default());
        s.assign_eq("half0", "half", "c0_1", Span::default());
        s.assign_eq("is_last_bit", "bit_idx", "c7_4", Span::default());
        s.assign_eq("is_last_byte", "bytes_left", "c1_3", Span::default());

        s.assign_xor("sclk_busy", "cpol", "half", Span::default());
        s.assign_mux("sclk", "busy_r", "sclk_busy", "cpol", Span::default());
        s.assign_mux("cs_n", "busy_r", "c0_1", "c1_1", Span::default());
        s.assign_mux("mosi", "busy_r", "mosi_r", "c0_1", Span::default());
        s.assign_net("mosi_byte", "hold", Span::default());
        s.assign_net("rx_data", "rx_hold", Span::default());
        s.assign_net("rx_valid", "valid_r", Span::default());
        s.assign_net("busy", "busy_r", Span::default());

        s.assign_and("tx_msb_m", "tx_data", "c80_8", Span::default());
        s.assign_eq("tx_msb", "tx_msb_m", "c80_8", Span::default());
        s.assign_and("sh_msb_m", "shift", "c80_8", Span::default());
        s.assign_eq("sh_msb", "sh_msb_m", "c80_8", Span::default());
        s.assign_mux("accept_mosi", "cpha0", "tx_msb", "c0_1", Span::default());

        s.assign_shl("shift_shl", "shift", "c1_8", Span::default());
        s.assign_and("shift_shl8", "shift_shl", "cff_8", Span::default());
        s.assign_and("sh8_msb_m", "shift_shl8", "c80_8", Span::default());
        s.assign_eq("sh8_msb", "sh8_msb_m", "c80_8", Span::default());
        s.assign_shl("rx_shl", "rx_shift", "c1_8", Span::default());
        s.assign_and("rx_shl8", "rx_shl", "cff_8", Span::default());
        s.assign_mux("miso8", "miso", "c1_8", "c0_8", Span::default());
        s.assign_or("rx_sampled", "rx_shl8", "miso8", Span::default());
        s.assign_add("bit_p1", "bit_idx", "c1_4", Span::default());
        s.assign_sub("bytes_m1", "bytes_left", "c1_3", Span::default());

        s.assign_and("do_lead", "busy_r", "half0", Span::default());
        s.assign_xor("do_trail", "do_lead", "busy_r", Span::default()); // busy && !half0
        s.assign_and("sample_lead", "do_lead", "cpha0", Span::default());
        s.assign_and("launch_lead", "do_lead", "cpha1", Span::default());
        s.assign_and("shift_trail", "do_trail", "cpha0", Span::default());
        s.assign_and("sample_trail", "do_trail", "cpha1", Span::default());
        s.assign_and("finish_byte", "do_trail", "is_last_bit", Span::default());
        s.assign_mux(
            "cont_frame",
            "is_last_byte",
            "c0_1",
            "finish_byte",
            Span::default(),
        );
        s.assign_and("end_frame", "finish_byte", "is_last_byte", Span::default());

        // Leading
        s.assign_mux(
            "lead_shift",
            "launch_lead",
            "shift_shl8",
            "shift",
            Span::default(),
        );
        s.assign_mux(
            "lead_mosi",
            "launch_lead",
            "sh_msb",
            "mosi_r",
            Span::default(),
        );
        s.assign_mux(
            "lead_rxsh",
            "sample_lead",
            "rx_sampled",
            "rx_shift",
            Span::default(),
        );

        // Trailing pre-finish
        s.assign_mux(
            "tr_shift0",
            "shift_trail",
            "shift_shl8",
            "shift",
            Span::default(),
        );
        s.assign_mux(
            "tr_mosi0",
            "shift_trail",
            "sh8_msb",
            "mosi_r",
            Span::default(),
        );
        s.assign_mux(
            "tr_rxsh0",
            "sample_trail",
            "rx_sampled",
            "rx_shift",
            Span::default(),
        );

        // Trailing finish / continue / advance
        s.assign_mux("tr_valid", "finish_byte", "c1_1", "c0_1", Span::default());
        s.assign_mux("tr_busy", "end_frame", "c0_1", "c1_1", Span::default());
        s.assign_mux("tr_bit", "finish_byte", "c0_4", "bit_p1", Span::default());
        s.assign_mux(
            "tr_bytes",
            "cont_frame",
            "bytes_m1",
            "bytes_left",
            Span::default(),
        );
        s.assign_mux("tr_hold", "cont_frame", "tx_data", "hold", Span::default());
        s.assign_mux(
            "tr_shift",
            "cont_frame",
            "tx_data",
            "tr_shift0",
            Span::default(),
        );
        s.assign_mux(
            "tr_mosi",
            "cont_frame",
            "accept_mosi",
            "tr_mosi0",
            Span::default(),
        );
        s.assign_mux(
            "tr_rxh",
            "finish_byte",
            "tr_rxsh0",
            "rx_hold",
            Span::default(),
        );
        s.assign_mux(
            "tr_rxsh",
            "finish_byte",
            "c0_8",
            "tr_rxsh0",
            Span::default(),
        );

        // Busy merge: lead vs trail (busy always one of them)
        s.assign_mux("b_half", "do_lead", "c1_1", "c0_1", Span::default());
        s.assign_mux("b_bit", "do_trail", "tr_bit", "bit_idx", Span::default());
        s.assign_mux(
            "b_bytes",
            "do_trail",
            "tr_bytes",
            "bytes_left",
            Span::default(),
        );
        s.assign_mux("b_hold", "do_trail", "tr_hold", "hold", Span::default());
        s.assign_mux(
            "b_shift",
            "do_trail",
            "tr_shift",
            "lead_shift",
            Span::default(),
        );
        s.assign_mux(
            "b_rxsh",
            "do_trail",
            "tr_rxsh",
            "lead_rxsh",
            Span::default(),
        );
        s.assign_mux("b_rxh", "do_trail", "tr_rxh", "rx_hold", Span::default());
        s.assign_mux("b_valid", "do_trail", "tr_valid", "c0_1", Span::default());
        s.assign_mux(
            "b_mosi",
            "do_trail",
            "tr_mosi",
            "lead_mosi",
            Span::default(),
        );
        s.assign_mux("b_busy", "do_trail", "tr_busy", "c1_1", Span::default());

        // !busy hold (except valid clears)
        s.assign_mux("nb_busy", "busy_r", "b_busy", "c0_1", Span::default());
        s.assign_mux("nb_half", "busy_r", "b_half", "c0_1", Span::default());
        s.assign_mux("nb_bit", "busy_r", "b_bit", "c0_4", Span::default());
        s.assign_mux("nb_bytes", "busy_r", "b_bytes", "c0_3", Span::default());
        s.assign_mux("nb_shift", "busy_r", "b_shift", "shift", Span::default());
        s.assign_mux("nb_rxsh", "busy_r", "b_rxsh", "rx_shift", Span::default());
        s.assign_mux("nb_hold", "busy_r", "b_hold", "hold", Span::default());
        s.assign_mux("nb_rxh", "busy_r", "b_rxh", "rx_hold", Span::default());
        s.assign_mux("nb_valid", "busy_r", "b_valid", "c0_1", Span::default());
        s.assign_mux("nb_mosi", "busy_r", "b_mosi", "c0_1", Span::default());

        // Accept overrides
        s.assign_mux("n_busy", "accept", "c1_1", "nb_busy", Span::default());
        s.assign_mux("n_half", "accept", "c0_1", "nb_half", Span::default());
        s.assign_mux("n_bit", "accept", "c0_4", "nb_bit", Span::default());
        s.assign_mux(
            "n_bytes",
            "accept",
            "eff_count",
            "nb_bytes",
            Span::default(),
        );
        s.assign_mux("n_shift", "accept", "tx_data", "nb_shift", Span::default());
        s.assign_mux("n_rxsh", "accept", "c0_8", "nb_rxsh", Span::default());
        s.assign_mux("n_hold", "accept", "tx_data", "nb_hold", Span::default());
        s.assign_mux("n_rxh", "accept", "rx_hold", "nb_rxh", Span::default());
        s.assign_mux("n_valid", "accept", "c0_1", "nb_valid", Span::default());
        s.assign_mux(
            "n_mosi",
            "accept",
            "accept_mosi",
            "nb_mosi",
            Span::default(),
        );
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_from("busy_r", "n_busy", Span::default());
        s.assign_reg_d_from("half", "n_half", Span::default());
        s.assign_reg_d_from("bit_idx", "n_bit", Span::default());
        s.assign_reg_d_from("bytes_left", "n_bytes", Span::default());
        s.assign_reg_d_from("shift", "n_shift", Span::default());
        s.assign_reg_d_from("rx_shift", "n_rxsh", Span::default());
        s.assign_reg_d_from("hold", "n_hold", Span::default());
        s.assign_reg_d_from("rx_hold", "n_rxh", Span::default());
        s.assign_reg_d_from("valid_r", "n_valid", Span::default());
        s.assign_reg_d_from("mosi_r", "n_mosi", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

/// I2C **master** near-VIP (FR98 / Epic 43.4) — ACK/NACK-driven write + read.
///
/// Accepts `start && !busy`; latches `addr[6:0]`/`rw` into address byte
/// (`{addr,rw}`) and `tx_data` for writes. Transaction:
/// START → 7-bit addr + R/W → ACK slot → write data + ACK **or** read byte +
/// master NACK → STOP. SCL uses half-period edges (idle high; bit setup `scl=0`,
/// sample `scl=1`). Exports `rx_data`/`rx_valid` and sticky `ack_error` on NACK.
///
/// `addr` is 8-bit wire; **only low 7 bits** are the I2C address (I2).
/// Non-goals: clock stretch, multi-master, 10-bit, slave, SMBus PEC, generator
/// closures (Epic 29). Single-byte payload per `start` (multi-byte not required by I1–I4).
pub struct I2cMaster;

impl Elaboratable for I2cMaster {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("I2cMaster");
        s.begin_module("I2cMaster", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("start", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("addr", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("rw", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("tx_data", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("sda_in", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("tx_byte", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("busy", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("scl", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("sda_out", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("rx_data", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("rx_valid", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("ack_error", GroundType::UInt { width: 1 }, Span::default());

        for (n, w) in [
            ("hold", 8),
            ("shift", 8),
            ("rx_shift", 8),
            ("rx_hold", 8),
            ("busy_r", 1),
            ("half", 1),
            ("bit_idx", 4),
            ("stage", 3),
            ("rw_r", 1),
            ("valid_r", 1),
            ("ack_err_r", 1),
        ] {
            s.declare_reg(n, GroundType::UInt { width: w }, Span::default());
        }
        for (n, w) in [
            ("c0_1", 1),
            ("c1_1", 1),
            ("c0_3", 3),
            ("c1_3", 3),
            ("c2_3", 3),
            ("c3_3", 3),
            ("c4_3", 3),
            ("c5_3", 3),
            ("c0_4", 4),
            ("c1_4", 4),
            ("c7_4", 4),
            ("c0_8", 8),
            ("c1_8", 8),
            ("cff_8", 8),
            ("c80_8", 8),
            ("accept", 1),
            ("addr_shl", 8),
            ("rw8", 8),
            ("addr_byte", 8),
            ("is_start", 1),
            ("is_addr", 1),
            ("is_aack", 1),
            ("is_data", 1),
            ("is_dack", 1),
            ("is_stop", 1),
            ("half0", 1),
            ("half1", 1),
            ("is_last_bit", 1),
            ("rw_read", 1),
            ("msb_m", 8),
            ("msb", 1),
            ("sda_data", 1),
            ("sda_mid", 1),
            ("is_ackph", 1),
            ("sda_or_ack", 1),
            ("is_lowdrv", 1),
            ("sda_active", 1),
            ("scl_busy", 1),
            ("shift_shl", 8),
            ("shift_shl8", 8),
            ("rx_shl", 8),
            ("rx_shl8", 8),
            ("sda_in8", 8),
            ("rx_sampled", 8),
            ("bit_p1", 4),
            ("nack", 1),
            ("do_start", 1),
            ("do_addr0", 1),
            ("do_addr1", 1),
            ("do_aack0", 1),
            ("do_aack1", 1),
            ("do_data0", 1),
            ("do_data1", 1),
            ("do_dack0", 1),
            ("do_dack1", 1),
            ("do_stop0", 1),
            ("do_stop1", 1),
            ("a1_done", 1),
            ("d1_done", 1),
            ("a1_bit", 4),
            ("a1_stage", 3),
            ("d1_bit", 4),
            ("d1_stage", 3),
            ("aa1_stage", 3),
            ("aa1_shift", 8),
            ("aa1_hold", 8),
            ("aa1_rxsh", 8),
            ("aa1_ackerr", 1),
            ("d1_rxsh_w", 8),
            ("d1_rxsh", 8),
            ("wr_ackerr", 1),
            ("dk_ackerr", 1),
            ("dk_valid", 1),
            ("dk_rxh", 8),
            ("m0_busy", 1),
            ("m0_half", 1),
            ("m0_bit", 4),
            ("m0_stage", 3),
            ("m0_shift", 8),
            ("m0_hold", 8),
            ("m0_rxsh", 8),
            ("m0_rxh", 8),
            ("m0_valid", 1),
            ("m0_ackerr", 1),
            ("m0_rw", 1),
            ("m1_busy", 1),
            ("m1_half", 1),
            ("m1_bit", 4),
            ("m1_stage", 3),
            ("m1_shift", 8),
            ("m1_hold", 8),
            ("m1_rxsh", 8),
            ("m1_rxh", 8),
            ("m1_valid", 1),
            ("m1_ackerr", 1),
            ("m1_rw", 1),
            ("m2_busy", 1),
            ("m2_half", 1),
            ("m2_bit", 4),
            ("m2_stage", 3),
            ("m2_shift", 8),
            ("m2_hold", 8),
            ("m2_rxsh", 8),
            ("m2_rxh", 8),
            ("m2_valid", 1),
            ("m2_ackerr", 1),
            ("m2_rw", 1),
            ("m3_busy", 1),
            ("m3_half", 1),
            ("m3_bit", 4),
            ("m3_stage", 3),
            ("m3_shift", 8),
            ("m3_hold", 8),
            ("m3_rxsh", 8),
            ("m3_rxh", 8),
            ("m3_valid", 1),
            ("m3_ackerr", 1),
            ("m3_rw", 1),
            ("m4_busy", 1),
            ("m4_half", 1),
            ("m4_bit", 4),
            ("m4_stage", 3),
            ("m4_shift", 8),
            ("m4_hold", 8),
            ("m4_rxsh", 8),
            ("m4_rxh", 8),
            ("m4_valid", 1),
            ("m4_ackerr", 1),
            ("m4_rw", 1),
            ("m5_busy", 1),
            ("m5_half", 1),
            ("m5_bit", 4),
            ("m5_stage", 3),
            ("m5_shift", 8),
            ("m5_hold", 8),
            ("m5_rxsh", 8),
            ("m5_rxh", 8),
            ("m5_valid", 1),
            ("m5_ackerr", 1),
            ("m5_rw", 1),
            ("m6_busy", 1),
            ("m6_half", 1),
            ("m6_bit", 4),
            ("m6_stage", 3),
            ("m6_shift", 8),
            ("m6_hold", 8),
            ("m6_rxsh", 8),
            ("m6_rxh", 8),
            ("m6_valid", 1),
            ("m6_ackerr", 1),
            ("m6_rw", 1),
            ("m7_busy", 1),
            ("m7_half", 1),
            ("m7_bit", 4),
            ("m7_stage", 3),
            ("m7_shift", 8),
            ("m7_hold", 8),
            ("m7_rxsh", 8),
            ("m7_rxh", 8),
            ("m7_valid", 1),
            ("m7_ackerr", 1),
            ("m7_rw", 1),
            ("m8_busy", 1),
            ("m8_half", 1),
            ("m8_bit", 4),
            ("m8_stage", 3),
            ("m8_shift", 8),
            ("m8_hold", 8),
            ("m8_rxsh", 8),
            ("m8_rxh", 8),
            ("m8_valid", 1),
            ("m8_ackerr", 1),
            ("m8_rw", 1),
            ("m9_busy", 1),
            ("m9_half", 1),
            ("m9_bit", 4),
            ("m9_stage", 3),
            ("m9_shift", 8),
            ("m9_hold", 8),
            ("m9_rxsh", 8),
            ("m9_rxh", 8),
            ("m9_valid", 1),
            ("m9_ackerr", 1),
            ("m9_rw", 1),
            ("m10_busy", 1),
            ("m10_half", 1),
            ("m10_bit", 4),
            ("m10_stage", 3),
            ("m10_shift", 8),
            ("m10_hold", 8),
            ("m10_rxsh", 8),
            ("m10_rxh", 8),
            ("m10_valid", 1),
            ("m10_ackerr", 1),
            ("m10_rw", 1),
            ("m11_busy", 1),
            ("m11_half", 1),
            ("m11_bit", 4),
            ("m11_stage", 3),
            ("m11_shift", 8),
            ("m11_hold", 8),
            ("m11_rxsh", 8),
            ("m11_rxh", 8),
            ("m11_valid", 1),
            ("m11_ackerr", 1),
            ("m11_rw", 1),
            ("nb_busy", 1),
            ("n_busy", 1),
            ("nb_half", 1),
            ("n_half", 1),
            ("nb_bit", 4),
            ("n_bit", 4),
            ("nb_stage", 3),
            ("n_stage", 3),
            ("nb_shift", 8),
            ("n_shift", 8),
            ("nb_hold", 8),
            ("n_hold", 8),
            ("nb_rxsh", 8),
            ("n_rxsh", 8),
            ("nb_rxh", 8),
            ("n_rxh", 8),
            ("nb_valid", 1),
            ("n_valid", 1),
            ("nb_ackerr", 1),
            ("n_ackerr", 1),
            ("nb_rw", 1),
            ("n_rw", 1),
        ] {
            s.declare_wire(n, GroundType::UInt { width: w }, Span::default());
        }

        s.begin_combinational(Span::default());
        s.assign_lit("c0_1", 0, Span::default());
        s.assign_lit("c1_1", 1, Span::default());
        s.assign_lit("c0_3", 0, Span::default());
        s.assign_lit("c1_3", 1, Span::default());
        s.assign_lit("c2_3", 2, Span::default());
        s.assign_lit("c3_3", 3, Span::default());
        s.assign_lit("c4_3", 4, Span::default());
        s.assign_lit("c5_3", 5, Span::default());
        s.assign_lit("c0_4", 0, Span::default());
        s.assign_lit("c1_4", 1, Span::default());
        s.assign_lit("c7_4", 7, Span::default());
        s.assign_lit("c0_8", 0, Span::default());
        s.assign_lit("c1_8", 1, Span::default());
        s.assign_lit("cff_8", 255, Span::default());
        s.assign_lit("c80_8", 128, Span::default());
        s.assign_mux("accept", "busy_r", "c0_1", "start", Span::default());
        s.assign_shl("addr_shl", "addr", "c1_8", Span::default());
        s.assign_mux("rw8", "rw", "c1_8", "c0_8", Span::default());
        s.assign_or("addr_byte", "addr_shl", "rw8", Span::default());
        s.assign_eq("is_start", "stage", "c0_3", Span::default());
        s.assign_eq("is_addr", "stage", "c1_3", Span::default());
        s.assign_eq("is_aack", "stage", "c2_3", Span::default());
        s.assign_eq("is_data", "stage", "c3_3", Span::default());
        s.assign_eq("is_dack", "stage", "c4_3", Span::default());
        s.assign_eq("is_stop", "stage", "c5_3", Span::default());
        s.assign_eq("half0", "half", "c0_1", Span::default());
        s.assign_xor("half1", "half0", "c1_1", Span::default());
        s.assign_eq("is_last_bit", "bit_idx", "c7_4", Span::default());
        s.assign_net("rw_read", "rw_r", Span::default());
        s.assign_and("msb_m", "shift", "c80_8", Span::default());
        s.assign_eq("msb", "msb_m", "c80_8", Span::default());
        s.assign_mux("sda_data", "rw_read", "c1_1", "msb", Span::default());
        s.assign_mux("sda_mid", "is_addr", "msb", "sda_data", Span::default());
        s.assign_or("is_ackph", "is_aack", "is_dack", Span::default());
        s.assign_mux("sda_or_ack", "is_ackph", "c1_1", "sda_mid", Span::default());
        s.assign_or("is_lowdrv", "is_start", "is_stop", Span::default());
        s.assign_mux(
            "sda_active",
            "is_lowdrv",
            "c0_1",
            "sda_or_ack",
            Span::default(),
        );
        s.assign_mux("sda_out", "busy_r", "sda_active", "c1_1", Span::default());
        s.assign_mux("scl_busy", "is_start", "c1_1", "half", Span::default());
        s.assign_mux("scl", "busy_r", "scl_busy", "c1_1", Span::default());
        s.assign_net("tx_byte", "hold", Span::default());
        s.assign_net("busy", "busy_r", Span::default());
        s.assign_net("rx_data", "rx_hold", Span::default());
        s.assign_net("rx_valid", "valid_r", Span::default());
        s.assign_net("ack_error", "ack_err_r", Span::default());
        s.assign_shl("shift_shl", "shift", "c1_8", Span::default());
        s.assign_and("shift_shl8", "shift_shl", "cff_8", Span::default());
        s.assign_shl("rx_shl", "rx_shift", "c1_8", Span::default());
        s.assign_and("rx_shl8", "rx_shl", "cff_8", Span::default());
        s.assign_mux("sda_in8", "sda_in", "c1_8", "c0_8", Span::default());
        s.assign_or("rx_sampled", "rx_shl8", "sda_in8", Span::default());
        s.assign_add("bit_p1", "bit_idx", "c1_4", Span::default());
        s.assign_net("nack", "sda_in", Span::default());
        s.assign_and("do_start", "busy_r", "is_start", Span::default());
        s.assign_and("do_addr0", "is_addr", "half0", Span::default());
        s.assign_and("do_addr1", "is_addr", "half1", Span::default());
        s.assign_and("do_aack0", "is_aack", "half0", Span::default());
        s.assign_and("do_aack1", "is_aack", "half1", Span::default());
        s.assign_and("do_data0", "is_data", "half0", Span::default());
        s.assign_and("do_data1", "is_data", "half1", Span::default());
        s.assign_and("do_dack0", "is_dack", "half0", Span::default());
        s.assign_and("do_dack1", "is_dack", "half1", Span::default());
        s.assign_and("do_stop0", "is_stop", "half0", Span::default());
        s.assign_and("do_stop1", "is_stop", "half1", Span::default());
        s.assign_and("a1_done", "do_addr1", "is_last_bit", Span::default());
        s.assign_and("d1_done", "do_data1", "is_last_bit", Span::default());
        s.assign_mux("a1_bit", "a1_done", "c0_4", "bit_p1", Span::default());
        s.assign_mux("a1_stage", "a1_done", "c2_3", "c1_3", Span::default());
        s.assign_mux("d1_bit", "d1_done", "c0_4", "bit_p1", Span::default());
        s.assign_mux("d1_stage", "d1_done", "c4_3", "c3_3", Span::default());
        s.assign_mux("aa1_stage", "nack", "c5_3", "c3_3", Span::default());
        s.assign_mux("aa1_ackerr", "nack", "c1_1", "ack_err_r", Span::default());
        s.assign_mux("aa1_shift", "rw_read", "shift", "tx_data", Span::default());
        s.assign_mux("aa1_hold", "rw_read", "hold", "tx_data", Span::default());
        s.assign_mux("aa1_rxsh", "rw_read", "c0_8", "rx_shift", Span::default());
        s.assign_mux(
            "d1_rxsh_w",
            "do_data1",
            "rx_sampled",
            "rx_shift",
            Span::default(),
        );
        s.assign_mux(
            "d1_rxsh",
            "rw_read",
            "d1_rxsh_w",
            "rx_shift",
            Span::default(),
        );
        s.assign_mux("wr_ackerr", "nack", "c1_1", "ack_err_r", Span::default());
        s.assign_mux(
            "dk_ackerr",
            "rw_read",
            "ack_err_r",
            "wr_ackerr",
            Span::default(),
        );
        s.assign_mux("dk_valid", "rw_read", "c1_1", "c0_1", Span::default());
        s.assign_mux("dk_rxh", "rw_read", "rx_shift", "rx_hold", Span::default());
        s.assign_net("m0_busy", "busy_r", Span::default());
        s.assign_net("m0_half", "half", Span::default());
        s.assign_net("m0_bit", "bit_idx", Span::default());
        s.assign_net("m0_stage", "stage", Span::default());
        s.assign_net("m0_shift", "shift", Span::default());
        s.assign_net("m0_hold", "hold", Span::default());
        s.assign_net("m0_rxsh", "rx_shift", Span::default());
        s.assign_net("m0_rxh", "rx_hold", Span::default());
        s.assign_net("m0_valid", "c0_1", Span::default());
        s.assign_net("m0_ackerr", "ack_err_r", Span::default());
        s.assign_net("m0_rw", "rw_r", Span::default());
        s.assign_mux("m1_busy", "do_start", "c1_1", "m0_busy", Span::default());
        s.assign_mux("m1_half", "do_start", "c0_1", "m0_half", Span::default());
        s.assign_mux("m1_bit", "do_start", "c0_4", "m0_bit", Span::default());
        s.assign_mux("m1_stage", "do_start", "c1_3", "m0_stage", Span::default());
        s.assign_mux(
            "m1_shift",
            "do_start",
            "m0_shift",
            "m0_shift",
            Span::default(),
        );
        s.assign_mux("m1_hold", "do_start", "m0_hold", "m0_hold", Span::default());
        s.assign_mux("m1_rxsh", "do_start", "m0_rxsh", "m0_rxsh", Span::default());
        s.assign_mux("m1_rxh", "do_start", "m0_rxh", "m0_rxh", Span::default());
        s.assign_mux("m1_valid", "do_start", "c0_1", "m0_valid", Span::default());
        s.assign_mux(
            "m1_ackerr",
            "do_start",
            "m0_ackerr",
            "m0_ackerr",
            Span::default(),
        );
        s.assign_mux("m1_rw", "do_start", "m0_rw", "m0_rw", Span::default());
        s.assign_mux("m2_busy", "do_addr0", "c1_1", "m1_busy", Span::default());
        s.assign_mux("m2_half", "do_addr0", "c1_1", "m1_half", Span::default());
        s.assign_mux("m2_bit", "do_addr0", "m1_bit", "m1_bit", Span::default());
        s.assign_mux(
            "m2_stage",
            "do_addr0",
            "m1_stage",
            "m1_stage",
            Span::default(),
        );
        s.assign_mux(
            "m2_shift",
            "do_addr0",
            "m1_shift",
            "m1_shift",
            Span::default(),
        );
        s.assign_mux("m2_hold", "do_addr0", "m1_hold", "m1_hold", Span::default());
        s.assign_mux("m2_rxsh", "do_addr0", "m1_rxsh", "m1_rxsh", Span::default());
        s.assign_mux("m2_rxh", "do_addr0", "m1_rxh", "m1_rxh", Span::default());
        s.assign_mux("m2_valid", "do_addr0", "c0_1", "m1_valid", Span::default());
        s.assign_mux(
            "m2_ackerr",
            "do_addr0",
            "m1_ackerr",
            "m1_ackerr",
            Span::default(),
        );
        s.assign_mux("m2_rw", "do_addr0", "m1_rw", "m1_rw", Span::default());
        s.assign_mux("m3_busy", "do_addr1", "c1_1", "m2_busy", Span::default());
        s.assign_mux("m3_half", "do_addr1", "c0_1", "m2_half", Span::default());
        s.assign_mux("m3_bit", "do_addr1", "a1_bit", "m2_bit", Span::default());
        s.assign_mux(
            "m3_stage",
            "do_addr1",
            "a1_stage",
            "m2_stage",
            Span::default(),
        );
        s.assign_mux(
            "m3_shift",
            "do_addr1",
            "shift_shl8",
            "m2_shift",
            Span::default(),
        );
        s.assign_mux("m3_hold", "do_addr1", "m2_hold", "m2_hold", Span::default());
        s.assign_mux("m3_rxsh", "do_addr1", "m2_rxsh", "m2_rxsh", Span::default());
        s.assign_mux("m3_rxh", "do_addr1", "m2_rxh", "m2_rxh", Span::default());
        s.assign_mux("m3_valid", "do_addr1", "c0_1", "m2_valid", Span::default());
        s.assign_mux(
            "m3_ackerr",
            "do_addr1",
            "m2_ackerr",
            "m2_ackerr",
            Span::default(),
        );
        s.assign_mux("m3_rw", "do_addr1", "m2_rw", "m2_rw", Span::default());
        s.assign_mux("m4_busy", "do_aack0", "c1_1", "m3_busy", Span::default());
        s.assign_mux("m4_half", "do_aack0", "c1_1", "m3_half", Span::default());
        s.assign_mux("m4_bit", "do_aack0", "m3_bit", "m3_bit", Span::default());
        s.assign_mux(
            "m4_stage",
            "do_aack0",
            "m3_stage",
            "m3_stage",
            Span::default(),
        );
        s.assign_mux(
            "m4_shift",
            "do_aack0",
            "m3_shift",
            "m3_shift",
            Span::default(),
        );
        s.assign_mux("m4_hold", "do_aack0", "m3_hold", "m3_hold", Span::default());
        s.assign_mux("m4_rxsh", "do_aack0", "m3_rxsh", "m3_rxsh", Span::default());
        s.assign_mux("m4_rxh", "do_aack0", "m3_rxh", "m3_rxh", Span::default());
        s.assign_mux("m4_valid", "do_aack0", "c0_1", "m3_valid", Span::default());
        s.assign_mux(
            "m4_ackerr",
            "do_aack0",
            "m3_ackerr",
            "m3_ackerr",
            Span::default(),
        );
        s.assign_mux("m4_rw", "do_aack0", "m3_rw", "m3_rw", Span::default());
        s.assign_mux("m5_busy", "do_aack1", "c1_1", "m4_busy", Span::default());
        s.assign_mux("m5_half", "do_aack1", "c0_1", "m4_half", Span::default());
        s.assign_mux("m5_bit", "do_aack1", "c0_4", "m4_bit", Span::default());
        s.assign_mux(
            "m5_stage",
            "do_aack1",
            "aa1_stage",
            "m4_stage",
            Span::default(),
        );
        s.assign_mux(
            "m5_shift",
            "do_aack1",
            "aa1_shift",
            "m4_shift",
            Span::default(),
        );
        s.assign_mux(
            "m5_hold",
            "do_aack1",
            "aa1_hold",
            "m4_hold",
            Span::default(),
        );
        s.assign_mux(
            "m5_rxsh",
            "do_aack1",
            "aa1_rxsh",
            "m4_rxsh",
            Span::default(),
        );
        s.assign_mux("m5_rxh", "do_aack1", "m4_rxh", "m4_rxh", Span::default());
        s.assign_mux("m5_valid", "do_aack1", "c0_1", "m4_valid", Span::default());
        s.assign_mux(
            "m5_ackerr",
            "do_aack1",
            "aa1_ackerr",
            "m4_ackerr",
            Span::default(),
        );
        s.assign_mux("m5_rw", "do_aack1", "m4_rw", "m4_rw", Span::default());
        s.assign_mux("m6_busy", "do_data0", "c1_1", "m5_busy", Span::default());
        s.assign_mux("m6_half", "do_data0", "c1_1", "m5_half", Span::default());
        s.assign_mux("m6_bit", "do_data0", "m5_bit", "m5_bit", Span::default());
        s.assign_mux(
            "m6_stage",
            "do_data0",
            "m5_stage",
            "m5_stage",
            Span::default(),
        );
        s.assign_mux(
            "m6_shift",
            "do_data0",
            "m5_shift",
            "m5_shift",
            Span::default(),
        );
        s.assign_mux("m6_hold", "do_data0", "m5_hold", "m5_hold", Span::default());
        s.assign_mux("m6_rxsh", "do_data0", "m5_rxsh", "m5_rxsh", Span::default());
        s.assign_mux("m6_rxh", "do_data0", "m5_rxh", "m5_rxh", Span::default());
        s.assign_mux("m6_valid", "do_data0", "c0_1", "m5_valid", Span::default());
        s.assign_mux(
            "m6_ackerr",
            "do_data0",
            "m5_ackerr",
            "m5_ackerr",
            Span::default(),
        );
        s.assign_mux("m6_rw", "do_data0", "m5_rw", "m5_rw", Span::default());
        s.assign_mux("m7_busy", "do_data1", "c1_1", "m6_busy", Span::default());
        s.assign_mux("m7_half", "do_data1", "c0_1", "m6_half", Span::default());
        s.assign_mux("m7_bit", "do_data1", "d1_bit", "m6_bit", Span::default());
        s.assign_mux(
            "m7_stage",
            "do_data1",
            "d1_stage",
            "m6_stage",
            Span::default(),
        );
        s.assign_mux(
            "m7_shift",
            "do_data1",
            "shift_shl8",
            "m6_shift",
            Span::default(),
        );
        s.assign_mux("m7_hold", "do_data1", "m6_hold", "m6_hold", Span::default());
        s.assign_mux("m7_rxsh", "do_data1", "d1_rxsh", "m6_rxsh", Span::default());
        s.assign_mux("m7_rxh", "do_data1", "m6_rxh", "m6_rxh", Span::default());
        s.assign_mux("m7_valid", "do_data1", "c0_1", "m6_valid", Span::default());
        s.assign_mux(
            "m7_ackerr",
            "do_data1",
            "m6_ackerr",
            "m6_ackerr",
            Span::default(),
        );
        s.assign_mux("m7_rw", "do_data1", "m6_rw", "m6_rw", Span::default());
        s.assign_mux("m8_busy", "do_dack0", "c1_1", "m7_busy", Span::default());
        s.assign_mux("m8_half", "do_dack0", "c1_1", "m7_half", Span::default());
        s.assign_mux("m8_bit", "do_dack0", "m7_bit", "m7_bit", Span::default());
        s.assign_mux(
            "m8_stage",
            "do_dack0",
            "m7_stage",
            "m7_stage",
            Span::default(),
        );
        s.assign_mux(
            "m8_shift",
            "do_dack0",
            "m7_shift",
            "m7_shift",
            Span::default(),
        );
        s.assign_mux("m8_hold", "do_dack0", "m7_hold", "m7_hold", Span::default());
        s.assign_mux("m8_rxsh", "do_dack0", "m7_rxsh", "m7_rxsh", Span::default());
        s.assign_mux("m8_rxh", "do_dack0", "m7_rxh", "m7_rxh", Span::default());
        s.assign_mux("m8_valid", "do_dack0", "c0_1", "m7_valid", Span::default());
        s.assign_mux(
            "m8_ackerr",
            "do_dack0",
            "m7_ackerr",
            "m7_ackerr",
            Span::default(),
        );
        s.assign_mux("m8_rw", "do_dack0", "m7_rw", "m7_rw", Span::default());
        s.assign_mux("m9_busy", "do_dack1", "c1_1", "m8_busy", Span::default());
        s.assign_mux("m9_half", "do_dack1", "c0_1", "m8_half", Span::default());
        s.assign_mux("m9_bit", "do_dack1", "m8_bit", "m8_bit", Span::default());
        s.assign_mux("m9_stage", "do_dack1", "c5_3", "m8_stage", Span::default());
        s.assign_mux(
            "m9_shift",
            "do_dack1",
            "m8_shift",
            "m8_shift",
            Span::default(),
        );
        s.assign_mux("m9_hold", "do_dack1", "m8_hold", "m8_hold", Span::default());
        s.assign_mux("m9_rxsh", "do_dack1", "m8_rxsh", "m8_rxsh", Span::default());
        s.assign_mux("m9_rxh", "do_dack1", "dk_rxh", "m8_rxh", Span::default());
        s.assign_mux(
            "m9_valid",
            "do_dack1",
            "dk_valid",
            "m8_valid",
            Span::default(),
        );
        s.assign_mux(
            "m9_ackerr",
            "do_dack1",
            "dk_ackerr",
            "m8_ackerr",
            Span::default(),
        );
        s.assign_mux("m9_rw", "do_dack1", "m8_rw", "m8_rw", Span::default());
        s.assign_mux("m10_busy", "do_stop0", "c1_1", "m9_busy", Span::default());
        s.assign_mux("m10_half", "do_stop0", "c1_1", "m9_half", Span::default());
        s.assign_mux("m10_bit", "do_stop0", "m9_bit", "m9_bit", Span::default());
        s.assign_mux(
            "m10_stage",
            "do_stop0",
            "m9_stage",
            "m9_stage",
            Span::default(),
        );
        s.assign_mux(
            "m10_shift",
            "do_stop0",
            "m9_shift",
            "m9_shift",
            Span::default(),
        );
        s.assign_mux(
            "m10_hold",
            "do_stop0",
            "m9_hold",
            "m9_hold",
            Span::default(),
        );
        s.assign_mux(
            "m10_rxsh",
            "do_stop0",
            "m9_rxsh",
            "m9_rxsh",
            Span::default(),
        );
        s.assign_mux("m10_rxh", "do_stop0", "m9_rxh", "m9_rxh", Span::default());
        s.assign_mux("m10_valid", "do_stop0", "c0_1", "m9_valid", Span::default());
        s.assign_mux(
            "m10_ackerr",
            "do_stop0",
            "m9_ackerr",
            "m9_ackerr",
            Span::default(),
        );
        s.assign_mux("m10_rw", "do_stop0", "m9_rw", "m9_rw", Span::default());
        s.assign_mux("m11_busy", "do_stop1", "c0_1", "m10_busy", Span::default());
        s.assign_mux("m11_half", "do_stop1", "c0_1", "m10_half", Span::default());
        s.assign_mux("m11_bit", "do_stop1", "c0_4", "m10_bit", Span::default());
        s.assign_mux(
            "m11_stage",
            "do_stop1",
            "c0_3",
            "m10_stage",
            Span::default(),
        );
        s.assign_mux(
            "m11_shift",
            "do_stop1",
            "m10_shift",
            "m10_shift",
            Span::default(),
        );
        s.assign_mux(
            "m11_hold",
            "do_stop1",
            "m10_hold",
            "m10_hold",
            Span::default(),
        );
        s.assign_mux(
            "m11_rxsh",
            "do_stop1",
            "m10_rxsh",
            "m10_rxsh",
            Span::default(),
        );
        s.assign_mux("m11_rxh", "do_stop1", "m10_rxh", "m10_rxh", Span::default());
        s.assign_mux(
            "m11_valid",
            "do_stop1",
            "c0_1",
            "m10_valid",
            Span::default(),
        );
        s.assign_mux(
            "m11_ackerr",
            "do_stop1",
            "m10_ackerr",
            "m10_ackerr",
            Span::default(),
        );
        s.assign_mux("m11_rw", "do_stop1", "m10_rw", "m10_rw", Span::default());
        s.assign_mux("nb_busy", "busy_r", "m11_busy", "c0_1", Span::default());
        s.assign_mux("nb_half", "busy_r", "m11_half", "c0_1", Span::default());
        s.assign_mux("nb_bit", "busy_r", "m11_bit", "c0_4", Span::default());
        s.assign_mux("nb_stage", "busy_r", "m11_stage", "c0_3", Span::default());
        s.assign_mux("nb_shift", "busy_r", "m11_shift", "shift", Span::default());
        s.assign_mux("nb_hold", "busy_r", "m11_hold", "hold", Span::default());
        s.assign_mux("nb_rxsh", "busy_r", "m11_rxsh", "rx_shift", Span::default());
        s.assign_mux("nb_rxh", "busy_r", "m11_rxh", "rx_hold", Span::default());
        s.assign_mux("nb_valid", "busy_r", "m11_valid", "c0_1", Span::default());
        s.assign_mux(
            "nb_ackerr",
            "busy_r",
            "m11_ackerr",
            "ack_err_r",
            Span::default(),
        );
        s.assign_mux("nb_rw", "busy_r", "m11_rw", "rw_r", Span::default());
        s.assign_mux("n_busy", "accept", "c1_1", "nb_busy", Span::default());
        s.assign_mux("n_half", "accept", "c0_1", "nb_half", Span::default());
        s.assign_mux("n_bit", "accept", "c0_4", "nb_bit", Span::default());
        s.assign_mux("n_stage", "accept", "c0_3", "nb_stage", Span::default());
        s.assign_mux(
            "n_shift",
            "accept",
            "addr_byte",
            "nb_shift",
            Span::default(),
        );
        s.assign_mux("n_hold", "accept", "addr_byte", "nb_hold", Span::default());
        s.assign_mux("n_rxsh", "accept", "c0_8", "nb_rxsh", Span::default());
        s.assign_mux("n_rxh", "accept", "nb_rxh", "nb_rxh", Span::default());
        s.assign_mux("n_valid", "accept", "c0_1", "nb_valid", Span::default());
        s.assign_mux("n_ackerr", "accept", "c0_1", "nb_ackerr", Span::default());
        s.assign_mux("n_rw", "accept", "rw", "nb_rw", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_from("busy_r", "n_busy", Span::default());
        s.assign_reg_d_from("half", "n_half", Span::default());
        s.assign_reg_d_from("bit_idx", "n_bit", Span::default());
        s.assign_reg_d_from("stage", "n_stage", Span::default());
        s.assign_reg_d_from("shift", "n_shift", Span::default());
        s.assign_reg_d_from("hold", "n_hold", Span::default());
        s.assign_reg_d_from("rx_shift", "n_rxsh", Span::default());
        s.assign_reg_d_from("rx_hold", "n_rxh", Span::default());
        s.assign_reg_d_from("valid_r", "n_valid", Span::default());
        s.assign_reg_d_from("ack_err_r", "n_ackerr", Span::default());
        s.assign_reg_d_from("rw_r", "n_rw", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

/// AXI4-Lite **minimal slave** single-register handshake toy (FR82 / Open Q7).
///
/// Documented widths: **ADDR=8**, **DATA=32**. One backing `data_r` register:
/// write when `awvalid && wvalid && !bvalid` (captures `wdata`; ignores addr/wstrb);
/// holds `bvalid` until `bready`; read when `arvalid && !rvalid && !bvalid`
/// (asserts `rvalid` with `rdata=data_r` until `rready`). `*ready` are combinatorial
/// when the corresponding response channel is idle.
///
/// Non-goals: Full AXI (burst/ID/QoS), interconnect, multi-slave decode, VIP,
/// generator closures (Epic 29).
pub struct Axi4LiteSlave;

impl Elaboratable for Axi4LiteSlave {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("Axi4LiteSlave");
        s.begin_module("Axi4LiteSlave", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input(
            "s_axi_awaddr",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.add_input(
            "s_axi_awvalid",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.add_output(
            "s_axi_awready",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.add_input(
            "s_axi_wdata",
            GroundType::UInt { width: 32 },
            Span::default(),
        );
        s.add_input(
            "s_axi_wstrb",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.add_input(
            "s_axi_wvalid",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.add_output(
            "s_axi_wready",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.add_output(
            "s_axi_bresp",
            GroundType::UInt { width: 2 },
            Span::default(),
        );
        s.add_output(
            "s_axi_bvalid",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.add_input(
            "s_axi_bready",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.add_input(
            "s_axi_araddr",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.add_input(
            "s_axi_arvalid",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.add_output(
            "s_axi_arready",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.add_output(
            "s_axi_rdata",
            GroundType::UInt { width: 32 },
            Span::default(),
        );
        s.add_output(
            "s_axi_rresp",
            GroundType::UInt { width: 2 },
            Span::default(),
        );
        s.add_output(
            "s_axi_rvalid",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.add_input(
            "s_axi_rready",
            GroundType::UInt { width: 1 },
            Span::default(),
        );

        s.declare_reg("data_r", GroundType::UInt { width: 32 }, Span::default());
        s.declare_reg("bvalid_r", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("rvalid_r", GroundType::UInt { width: 1 }, Span::default());

        s.declare_wire("c0_1", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("c1_1", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("aw_ready", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("ar_ready", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("aw_fire", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("w_fire", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("do_write", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("ar_fire", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("do_read", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("b_fire", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("r_fire", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("next_data", GroundType::UInt { width: 32 }, Span::default());
        s.declare_wire("b_clear", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("b_hold", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire(
            "next_bvalid",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.declare_wire("r_clear", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("r_hold", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire(
            "next_rvalid",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.declare_wire("not_bvalid", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("not_rvalid", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("can_ar", GroundType::UInt { width: 1 }, Span::default());

        s.begin_combinational(Span::default());
        s.assign_lit("c0_1", 0, Span::default());
        s.assign_lit("c1_1", 1, Span::default());
        // awready/wready when !bvalid; arready when !rvalid && !bvalid
        s.assign_xor("not_bvalid", "bvalid_r", "c1_1", Span::default());
        s.assign_xor("not_rvalid", "rvalid_r", "c1_1", Span::default());
        s.assign_net("aw_ready", "not_bvalid", Span::default());
        s.assign_and("can_ar", "not_rvalid", "not_bvalid", Span::default());
        s.assign_net("ar_ready", "can_ar", Span::default());
        s.assign_net("s_axi_awready", "aw_ready", Span::default());
        s.assign_net("s_axi_wready", "aw_ready", Span::default());
        s.assign_net("s_axi_arready", "ar_ready", Span::default());
        s.assign_net("s_axi_bvalid", "bvalid_r", Span::default());
        s.assign_net("s_axi_rvalid", "rvalid_r", Span::default());
        s.assign_net("s_axi_rdata", "data_r", Span::default());
        s.assign_lit("s_axi_bresp", 0, Span::default());
        s.assign_lit("s_axi_rresp", 0, Span::default());

        s.assign_and("aw_fire", "s_axi_awvalid", "aw_ready", Span::default());
        s.assign_and("w_fire", "s_axi_wvalid", "aw_ready", Span::default());
        s.assign_and("do_write", "aw_fire", "w_fire", Span::default());
        s.assign_and("ar_fire", "s_axi_arvalid", "ar_ready", Span::default());
        // Prefer write over read in the same cycle
        s.assign_mux("do_read", "do_write", "c0_1", "ar_fire", Span::default());
        s.assign_and("b_fire", "bvalid_r", "s_axi_bready", Span::default());
        s.assign_and("r_fire", "rvalid_r", "s_axi_rready", Span::default());

        s.assign_mux(
            "next_data",
            "do_write",
            "s_axi_wdata",
            "data_r",
            Span::default(),
        );
        s.assign_mux("b_clear", "b_fire", "c0_1", "bvalid_r", Span::default());
        s.assign_mux("b_hold", "do_write", "c1_1", "b_clear", Span::default());
        s.assign_net("next_bvalid", "b_hold", Span::default());
        s.assign_mux("r_clear", "r_fire", "c0_1", "rvalid_r", Span::default());
        s.assign_mux("r_hold", "do_read", "c1_1", "r_clear", Span::default());
        s.assign_net("next_rvalid", "r_hold", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_from("data_r", "next_data", Span::default());
        s.assign_reg_d_from("bvalid_r", "next_bvalid", Span::default());
        s.assign_reg_d_from("rvalid_r", "next_rvalid", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

/// Opaque vendor IP wrapper: ports only; no child FrozenHir body (FR37 black-box).
///
/// **Boundary (FR82 / Epic 34):** retained as an opaque shell — Bitloom does **not**
/// inline vendor netlists into HIR. Pair with [`vendor_blackbox_v`] (or an external
/// `.v`) at link/synth time. Not a synthesizable behavior model.
pub struct ExtBlackBox;

impl Elaboratable for ExtBlackBox {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("ExtBlackBox");
        s.begin_module("ExtBlackBox", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
        // Black-box: declare ports only; vendor netlist supplied separately.
        s.end_module();
        s.finish()
    }
}

/// Vendor Verilog stub paired with [`ExtBlackBox`] (not inlined into HIR).
pub fn vendor_blackbox_v() -> &'static str {
    "module vendor_ext_ip(input clk, input rst, input [7:0] data_in, output [7:0] data_out);\nendmodule\n"
}

/// CRC-8 byte → residue for one table entry (MSB-first, poly XOR).
pub fn crc8_table_byte(byte: u8, poly: u8) -> u8 {
    let mut b = byte;
    for _ in 0..8 {
        if b & 0x80 != 0 {
            b = (b << 1) ^ poly;
        } else {
            b <<= 1;
        }
    }
    b
}

/// CRC-8 LUT ROM IP — FR77 / Cap-R-63 generator-closure customization overlay.
///
/// SyncReadMem depth-256 × 8: `addr` → registered `rdata`. Built on Epic 27
/// `declare_sync_read_mem_with_init_fn` / `generate_mem_init`.
///
/// - **Without user closure:** [`Crc8Lut::elaborate`] / [`Elaboratable::elaborate`]
///   use documented default poly [`Crc8Lut::DEFAULT_POLY`] (`0x07`, CRC-8/SMBUS-style).
/// - **With closure:** [`Crc8Lut::elaborate_with_table_fn`] runs `Fn(usize) -> u64`
///   at elaborate time; empty `violations` = legal [`crate::SynthesizableClosure`];
///   non-empty → clear diagnostics (no silent default).
///
/// Closures do **not** enter FrozenHir / emit (NFR36). Not comb/seq synthesizable
/// closure inline (Epic 28) — table generation only.
pub struct Crc8Lut;

impl Crc8Lut {
    pub const DEPTH: u32 = 256;
    pub const WIDTH: u32 = 8;
    /// Documented default polynomial when no customization closure is supplied.
    pub const DEFAULT_POLY: u8 = 0x07;

    /// Elaborate with documented default CRC-8/SMBUS-style poly `0x07` (no user Fn).
    pub fn elaborate_default() -> Result<FrozenHir, Diagnostics> {
        Self::elaborate_with_poly(Self::DEFAULT_POLY)
    }

    /// Elaborate with a fixed polynomial (table built inside the session; no user Fn).
    pub fn elaborate_with_poly(poly: u8) -> Result<FrozenHir, Diagnostics> {
        Self::elaborate_with_table_fn(&[], move |i| crc8_table_byte(i as u8, poly) as u64)
    }

    /// Customize the LUT via an elaborate-time generator closure (FR77 / Cap-R-63).
    ///
    /// `f(addr)` is evaluated for each address inside [`ElaborateSession`] and stored
    /// as plain `Vec<u64>` on the mem node — the Rust `Fn` does not survive freeze
    /// (NFR36). Pass empty `violations` for a legal synthesizable generator; any
    /// documented [`SynthesizableClosureViolation`] fails with clear diagnostics.
    pub fn elaborate_with_table_fn<F>(
        violations: &[SynthesizableClosureViolation],
        f: F,
    ) -> Result<FrozenHir, Diagnostics>
    where
        F: Fn(usize) -> u64,
    {
        if !violations.is_empty() {
            return Err(diagnose_synthesizable_closure_violations(
                violations,
                Span::default(),
            ));
        }

        let mut s = ElaborateSession::new("Crc8Lut");
        s.begin_module("Crc8Lut", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("addr", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("rdata", GroundType::UInt { width: 8 }, Span::default());

        s.declare_reg("q", GroundType::UInt { width: 8 }, Span::default());
        // Epic 27 path: Fn dissolves to MemDecl.init before freeze (NFR36).
        s.declare_sync_read_mem_with_init_fn("lut", Self::DEPTH, Self::WIDTH, f, Span::default());

        s.begin_combinational(Span::default());
        s.assign_net("rdata", "q", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_mem_read("q", "lut", "addr", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

impl Elaboratable for Crc8Lut {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        Crc8Lut::elaborate_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SynthesizableClosureViolation;
    use bitloom_hir::PortValues;
    use bitloom_sim::Sim;
    use bitloom_vlog::emit;

    fn smoke_elaborate_emit_tick<T: Elaboratable>(abi: &str) {
        let hir = T::elaborate().expect("elaborate");
        assert_eq!(hir.abi_name, abi);
        let art = emit(&hir);
        assert!(
            art.files
                .iter()
                .any(|f| f.contents.contains(&format!("module {abi}"))),
            "emit must contain module {abi}"
        );
        let mut sim = Sim::new(hir);
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        sim.set_inputs(pv);
        sim.tick();
    }

    fn fifo_drive(sim: &mut Sim, rst: u64, wr_en: u64, rd_en: u64, data_in: u64) {
        let mut pv = PortValues::default();
        pv.set("rst", rst);
        pv.set("wr_en", wr_en);
        pv.set("rd_en", rd_en);
        pv.set("data_in", data_in);
        sim.set_inputs(pv);
        // Same-cycle enables (`wr_en && !full`) need comb settle before the edge.
        sim.settle();
        sim.tick();
    }

    #[test]
    fn sync_fifo_elaborate_emit_tick() {
        smoke_elaborate_emit_tick::<SyncFifo>("SyncFifo");
        let hir = SyncFifo::elaborate().unwrap();
        let art = emit(&hir);
        let v = &art.files[0].contents;
        assert!(v.contains("module SyncFifo"), "emit .v");
        assert!(
            v.contains("full") && v.contains("empty") && v.contains("wr_en"),
            "non-stub ports in emit"
        );

        let mut sim = Sim::new(SyncFifo::elaborate().unwrap());
        // reset
        fifo_drive(&mut sim, 1, 0, 0, 0);
        assert_eq!(sim.ports().get("empty"), Some(1));
        assert_eq!(sim.ports().get("full"), Some(0));

        // push 0x11, 0x22
        fifo_drive(&mut sim, 0, 1, 0, 0x11);
        assert_eq!(sim.ports().get("empty"), Some(0));
        fifo_drive(&mut sim, 0, 1, 0, 0x22);

        // pop — dout tracks head via async mem read of rd_ptr
        fifo_drive(&mut sim, 0, 0, 1, 0);
        assert_eq!(sim.ports().get("data_out"), Some(0x11));
        fifo_drive(&mut sim, 0, 0, 1, 0);
        assert_eq!(sim.ports().get("data_out"), Some(0x22));
        assert_eq!(sim.ports().get("empty"), Some(1));
    }

    #[test]
    fn sync_fifo_fills_then_blocks_write() {
        let mut sim = Sim::new(SyncFifo::elaborate().unwrap());
        fifo_drive(&mut sim, 1, 0, 0, 0);
        for b in [0xA0u64, 0xA1, 0xA2, 0xA3] {
            fifo_drive(&mut sim, 0, 1, 0, b);
        }
        assert_eq!(sim.ports().get("full"), Some(1));
        // write while full must not change occupancy / lose head
        fifo_drive(&mut sim, 0, 1, 0, 0xFF);
        assert_eq!(sim.ports().get("full"), Some(1));
        fifo_drive(&mut sim, 0, 0, 1, 0);
        assert_eq!(sim.ports().get("data_out"), Some(0xA0));
    }

    fn uart_drive(sim: &mut Sim, rst: u64, wr_en: u64, wr_data: u64) {
        uart_drive_baud(sim, rst, wr_en, wr_data, 0);
    }

    fn uart_drive_baud(sim: &mut Sim, rst: u64, wr_en: u64, wr_data: u64, baud_div: u64) {
        let mut pv = PortValues::default();
        pv.set("rst", rst);
        pv.set("wr_en", wr_en);
        pv.set("wr_data", wr_data);
        pv.set("baud_div", baud_div);
        sim.set_inputs(pv);
        sim.settle();
        sim.tick();
    }

    #[test]
    fn uart_tx_elaborate_emit_tick() {
        smoke_elaborate_emit_tick::<UartTx>("UartTx");
        let hir = UartTx::elaborate().unwrap();
        let art = emit(&hir);
        let v = &art.files[0].contents;
        assert!(v.contains("module UartTx"));
        assert!(
            v.contains("tx_busy") && v.contains("tx"),
            "serial tx in emit"
        );

        let mut sim = Sim::new(UartTx::elaborate().unwrap());
        uart_drive(&mut sim, 1, 0, 0);
        assert_eq!(sim.ports().get("tx"), Some(1)); // idle high
        assert_eq!(sim.ports().get("tx_busy"), Some(0));

        // accept byte 0xA5 = 0b1010_0101 → LSB first: 1,0,1,0,0,1,0,1
        uart_drive(&mut sim, 0, 1, 0xA5);
        assert_eq!(sim.ports().get("tx_busy"), Some(1));
        assert_eq!(sim.ports().get("tx_byte"), Some(0xA5));
        assert_eq!(sim.ports().get("tx"), Some(0)); // start bit

        let expected = [0u64, 1, 0, 1, 0, 0, 1, 0, 1, 1]; // start + 8 data + stop
        // Current cycle already showed start (bit_idx=0). Next 9 cycles: data…stop.
        for (i, &bit) in expected.iter().enumerate().skip(1) {
            uart_drive(&mut sim, 0, 0, 0);
            assert_eq!(sim.ports().get("tx"), Some(bit), "frame bit {i} mismatch");
        }
        // Stop bit cycle leaves bit_idx==9 / busy==1; one idle tick clears busy.
        assert_eq!(sim.ports().get("tx_busy"), Some(1));
        uart_drive(&mut sim, 0, 0, 0);
        assert_eq!(sim.ports().get("tx_busy"), Some(0));
        assert_eq!(sim.ports().get("tx"), Some(1)); // idle

        // ignore wr_en while busy: start new frame, then wr_en with other data must not replace
        uart_drive(&mut sim, 0, 1, 0x3C);
        assert_eq!(sim.ports().get("tx_busy"), Some(1));
        uart_drive(&mut sim, 0, 1, 0xFF);
        assert_eq!(sim.ports().get("tx_byte"), Some(0x3C));
    }

    #[test]
    fn uart_tx_programmable_baud_holds_bits() {
        let mut sim = Sim::new(UartTx::elaborate().unwrap());
        let baud_div = 1u64; // 2 clk/bit
        uart_drive_baud(&mut sim, 1, 0, 0, baud_div);
        uart_drive_baud(&mut sim, 0, 1, 0x01, baud_div);
        assert_eq!(sim.ports().get("tx"), Some(0)); // start
        uart_drive_baud(&mut sim, 0, 0, 0, baud_div);
        assert_eq!(sim.ports().get("tx"), Some(0), "start held");
        uart_drive_baud(&mut sim, 0, 0, 0, baud_div);
        assert_eq!(sim.ports().get("tx"), Some(1), "LSB after baud period");
    }

    fn uart_rx_drive(sim: &mut Sim, rst: u64, rx: u64, baud_div: u64) {
        let mut pv = PortValues::default();
        pv.set("rst", rst);
        pv.set("rx", rx);
        pv.set("baud_div", baud_div);
        sim.set_inputs(pv);
        sim.settle();
        sim.tick();
    }

    #[test]
    fn uart_rx_elaborate_emit_tick() {
        smoke_elaborate_emit_tick::<UartRx>("UartRx");
        let hir = UartRx::elaborate().unwrap();
        let art = emit(&hir);
        let v = &art.files[0].contents;
        assert!(v.contains("module UartRx"));
        assert!(
            v.contains("rd_data") && v.contains("rd_valid") && v.contains("baud_div"),
            "RX ports in emit"
        );

        // Idle high then falling edge + 8N1 for 0xA5 (LSB first)
        let mut sim = Sim::new(UartRx::elaborate().unwrap());
        uart_rx_drive(&mut sim, 1, 1, 0);
        uart_rx_drive(&mut sim, 0, 1, 0);
        // start + data bits of 0xA5 = 0b1010_0101 → LSB first: 1,0,1,0,0,1,0,1 + stop
        let bits = [0u64, 1, 0, 1, 0, 0, 1, 0, 1, 1];
        uart_rx_drive(&mut sim, 0, bits[0], 0); // start edge
        for &b in &bits[1..] {
            uart_rx_drive(&mut sim, 0, b, 0);
        }
        assert_eq!(sim.ports().get("rd_valid"), Some(1));
        assert_eq!(sim.ports().get("rd_data"), Some(0xA5));
    }

    fn spi_drive(
        sim: &mut Sim,
        rst: u64,
        start: u64,
        tx_data: u64,
        miso: u64,
        cpol: u64,
        cpha: u64,
        byte_count: u64,
    ) {
        let mut pv = PortValues::default();
        pv.set("rst", rst);
        pv.set("start", start);
        pv.set("tx_data", tx_data);
        pv.set("miso", miso);
        pv.set("cpol", cpol);
        pv.set("cpha", cpha);
        pv.set("byte_count", byte_count);
        sim.set_inputs(pv);
        sim.settle();
        sim.tick();
    }

    #[test]
    fn spi_master_elaborate_emit_tick() {
        smoke_elaborate_emit_tick::<SpiMaster>("SpiMaster");
        let hir = SpiMaster::elaborate().unwrap();
        let art = emit(&hir);
        let v = &art.files[0].contents;
        assert!(v.contains("module SpiMaster"));
        assert!(
            v.contains("cs_n")
                && v.contains("sclk")
                && v.contains("mosi")
                && v.contains("cpol")
                && v.contains("rx_data"),
            "FR98 SPI must emit serial pins + CPOL/RX"
        );

        let mut sim = Sim::new(SpiMaster::elaborate().unwrap());
        spi_drive(&mut sim, 1, 0, 0, 0, 0, 0, 1);
        assert_eq!(sim.ports().get("busy"), Some(0));
        assert_eq!(sim.ports().get("cs_n"), Some(1));
        assert_eq!(sim.ports().get("sclk"), Some(0));

        // Mode-0 half-period: 0xA5 = 1010_0101
        let expected = [1u64, 0, 1, 0, 0, 1, 0, 1];
        spi_drive(&mut sim, 0, 1, 0xA5, 0, 0, 0, 1);
        assert_eq!(sim.ports().get("busy"), Some(1));
        assert_eq!(sim.ports().get("mosi_byte"), Some(0xA5));
        assert_eq!(sim.ports().get("cs_n"), Some(0));
        assert_eq!(sim.ports().get("sclk"), Some(0), "Mode-0 setup half");
        assert_eq!(sim.ports().get("mosi"), Some(1));

        for (i, &bit) in expected.iter().enumerate() {
            // leading
            spi_drive(&mut sim, 0, 0, 0, 0, 0, 0, 1);
            assert_eq!(sim.ports().get("sclk"), Some(1), "leading bit {i}");
            assert_eq!(sim.ports().get("mosi"), Some(bit), "MOSI bit {i}");
            // trailing (or finish on last)
            spi_drive(&mut sim, 0, 0, 0, 0, 0, 0, 1);
            if i + 1 < expected.len() {
                assert_eq!(sim.ports().get("busy"), Some(1));
                assert_eq!(sim.ports().get("sclk"), Some(0));
                assert_eq!(sim.ports().get("mosi"), Some(expected[i + 1]));
            }
        }
        assert_eq!(sim.ports().get("busy"), Some(0));
        assert_eq!(sim.ports().get("cs_n"), Some(1));

        // busy-gated: start while busy must not replace hold
        spi_drive(&mut sim, 0, 1, 0x3C, 0, 0, 0, 1);
        assert_eq!(sim.ports().get("busy"), Some(1));
        spi_drive(&mut sim, 0, 1, 0xFF, 0, 0, 0, 1);
        assert_eq!(sim.ports().get("mosi_byte"), Some(0x3C));
    }

    fn i2c_drive(
        sim: &mut Sim,
        rst: u64,
        start: u64,
        addr: u64,
        rw: u64,
        tx_data: u64,
        sda_in: u64,
    ) {
        let mut pv = PortValues::default();
        pv.set("rst", rst);
        pv.set("start", start);
        pv.set("addr", addr);
        pv.set("rw", rw);
        pv.set("tx_data", tx_data);
        pv.set("sda_in", sda_in);
        sim.set_inputs(pv);
        sim.settle();
        sim.tick();
    }

    #[test]
    fn i2c_master_elaborate_emit_tick() {
        smoke_elaborate_emit_tick::<I2cMaster>("I2cMaster");
        let hir = I2cMaster::elaborate().unwrap();
        let art = emit(&hir);
        let v = &art.files[0].contents;
        assert!(v.contains("module I2cMaster"));
        assert!(
            v.contains("scl")
                && v.contains("sda_out")
                && v.contains("addr")
                && v.contains("ack_error"),
            "FR98 I2C must emit near-VIP ports"
        );

        let mut sim = Sim::new(I2cMaster::elaborate().unwrap());
        i2c_drive(&mut sim, 1, 0, 0, 0, 0, 1);
        assert_eq!(sim.ports().get("busy"), Some(0));
        assert_eq!(sim.ports().get("scl"), Some(1));
        assert_eq!(sim.ports().get("sda_out"), Some(1));

        let addr = 0x50u64;
        let data = 0xA5u64;
        let addr_byte = addr << 1;
        i2c_drive(&mut sim, 0, 1, addr, 0, data, 0);
        assert_eq!(sim.ports().get("busy"), Some(1));
        assert_eq!(sim.ports().get("tx_byte"), Some(addr_byte));
        assert_eq!(sim.ports().get("sda_out"), Some(0)); // START
        assert_eq!(sim.ports().get("scl"), Some(1));

        let mut saw_scl_low = false;
        for _ in 0..64 {
            i2c_drive(&mut sim, 0, 0, addr, 0, data, 0);
            if sim.ports().get("scl") == Some(0) {
                saw_scl_low = true;
            }
            if sim.ports().get("busy") == Some(0) {
                break;
            }
        }
        assert!(saw_scl_low, "SCL must leave idle-high during transfer");
        assert_eq!(sim.ports().get("busy"), Some(0));
        assert_eq!(sim.ports().get("ack_error"), Some(0));

        // busy-gated: second start while busy must not replace hold
        i2c_drive(&mut sim, 0, 1, 0x11, 0, 0x22, 0);
        assert_eq!(sim.ports().get("busy"), Some(1));
        let held = sim.ports().get("tx_byte");
        i2c_drive(&mut sim, 0, 1, 0x7F, 0, 0xFF, 0);
        assert_eq!(sim.ports().get("tx_byte"), held);
    }

    fn axi_drive(
        sim: &mut Sim,
        rst: u64,
        awvalid: u64,
        wvalid: u64,
        wdata: u64,
        bready: u64,
        arvalid: u64,
        rready: u64,
    ) {
        let mut pv = PortValues::default();
        pv.set("rst", rst);
        pv.set("s_axi_awaddr", 0);
        pv.set("s_axi_awvalid", awvalid);
        pv.set("s_axi_wdata", wdata);
        pv.set("s_axi_wstrb", 0xF);
        pv.set("s_axi_wvalid", wvalid);
        pv.set("s_axi_bready", bready);
        pv.set("s_axi_araddr", 0);
        pv.set("s_axi_arvalid", arvalid);
        pv.set("s_axi_rready", rready);
        sim.set_inputs(pv);
        sim.settle();
        sim.tick();
    }

    #[test]
    fn axi4_lite_slave_elaborate_emit_tick() {
        smoke_elaborate_emit_tick::<Axi4LiteSlave>("Axi4LiteSlave");
        let hir = Axi4LiteSlave::elaborate().unwrap();
        let art = emit(&hir);
        let v = &art.files[0].contents;
        assert!(v.contains("module Axi4LiteSlave"));
        assert!(
            v.contains("s_axi_awready") && v.contains("s_axi_bvalid") && v.contains("s_axi_rvalid"),
            "FR82 AXI-Lite must emit handshake ports"
        );

        let mut sim = Sim::new(Axi4LiteSlave::elaborate().unwrap());
        axi_drive(&mut sim, 1, 0, 0, 0, 0, 0, 0);
        assert_eq!(sim.ports().get("s_axi_awready"), Some(1));
        assert_eq!(sim.ports().get("s_axi_bvalid"), Some(0));

        // write 0xDEAD_BEEF; hold bready low so bvalid sticks
        axi_drive(&mut sim, 0, 1, 1, 0xDEAD_BEEF, 0, 0, 0);
        assert_eq!(sim.ports().get("s_axi_bvalid"), Some(1));
        assert_eq!(sim.ports().get("s_axi_bresp"), Some(0));
        assert_eq!(sim.ports().get("s_axi_awready"), Some(0)); // blocked while bvalid

        axi_drive(&mut sim, 0, 0, 0, 0, 1, 0, 0);
        assert_eq!(sim.ports().get("s_axi_bvalid"), Some(0));
        assert_eq!(sim.ports().get("s_axi_awready"), Some(1));

        // read back
        axi_drive(&mut sim, 0, 0, 0, 0, 0, 1, 0);
        assert_eq!(sim.ports().get("s_axi_rvalid"), Some(1));
        assert_eq!(sim.ports().get("s_axi_rdata"), Some(0xDEAD_BEEF));
        axi_drive(&mut sim, 0, 0, 0, 0, 0, 0, 1);
        assert_eq!(sim.ports().get("s_axi_rvalid"), Some(0));
    }

    #[test]
    fn blackbox_elaborate_emit_tick_opaque() {
        smoke_elaborate_emit_tick::<ExtBlackBox>("ExtBlackBox");
        let hir = ExtBlackBox::elaborate().unwrap();
        // Opaque: no regs / processes in body — ports only.
        let m = &hir.circuit().modules[0];
        assert!(
            m.body.is_empty(),
            "black-box must not inline vendor HIR body"
        );
        assert!(vendor_blackbox_v().contains("vendor_ext_ip"));
    }

    #[test]
    fn fifo_uart_api_has_no_generator_closure_params() {
        // Compile-time / surface check: Elaboratable::elaborate takes no Fn.
        let _ = SyncFifo::elaborate();
        let _ = UartTx::elaborate();
        let _ = SpiMaster::elaborate();
        let _ = I2cMaster::elaborate();
        let _ = Axi4LiteSlave::elaborate();
    }

    #[test]
    fn crc8_lut_default_elaborate_emit_tick() {
        let hir = Crc8Lut::elaborate().expect("default");
        assert_eq!(hir.abi_name, "Crc8Lut");
        let art = emit(&hir);
        let v = &art.files[0].contents;
        assert!(v.contains("module Crc8Lut"));
        assert!(v.contains("lut[0] = 0;"));
        assert!(v.contains(&format!(
            "lut[1] = {};",
            crc8_table_byte(1, Crc8Lut::DEFAULT_POLY)
        )));
        assert!(
            !v.to_lowercase().contains("closure") && !v.contains("||"),
            "NFR36: emit must not retain closure IR"
        );

        let mut sim = Sim::new(hir);
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        sim.set_inputs(pv.clone());
        sim.tick();
        pv.set("rst", 0);
        pv.set("addr", 1);
        sim.set_inputs(pv);
        sim.tick(); // schedule SyncReadMem
        sim.tick(); // deliver into q / rdata
        assert_eq!(
            sim.ports().get("rdata"),
            Some(crc8_table_byte(1, Crc8Lut::DEFAULT_POLY) as u64)
        );
    }

    #[test]
    fn crc8_lut_custom_poly_via_table_fn() {
        let poly = 0x1du8;
        let hir = Crc8Lut::elaborate_with_table_fn(&[], |i| crc8_table_byte(i as u8, poly) as u64)
            .expect("custom");
        let mut sim = Sim::new(hir);
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        sim.set_inputs(pv.clone());
        sim.tick();
        pv.set("rst", 0);
        pv.set("addr", 0xA5);
        sim.set_inputs(pv);
        sim.tick(); // schedule
        sim.tick(); // deliver
        assert_eq!(
            sim.ports().get("rdata"),
            Some(crc8_table_byte(0xA5, poly) as u64)
        );
        // Custom poly must differ from default at this address (proves customization).
        assert_ne!(
            crc8_table_byte(0xA5, poly),
            crc8_table_byte(0xA5, Crc8Lut::DEFAULT_POLY)
        );
    }

    #[test]
    fn crc8_lut_synthesizable_closure_violation_is_clear_error() {
        let err = Crc8Lut::elaborate_with_table_fn(
            &[SynthesizableClosureViolation::heap("Vec in table Fn")],
            |_| 0,
        )
        .expect_err("must reject");
        assert!(
            err.0.iter().any(|d| d.code == "rhdl::E0143"),
            "expected E0143, got {err:?}"
        );
    }
}
