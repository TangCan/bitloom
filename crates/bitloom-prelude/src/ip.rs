//! First-class IP (FR37 / FR48 / FR82 / FR89 / FR98 UART): SyncFifo, UartTx,
//! UartRx, SpiMaster, I2cMaster, Axi4LiteSlave, black-box; plus FR77 overlay
//! [`Crc8Lut`] (Epic 29.3).
//!
//! Epic 34 / FR82 deepens five classes to **non-stub** synthesizable baselines.
//! Epic 38 / FR89 deepens UartTx programmable baud. Epic 43 / FR98 Story 43.2
//! adds UartRx (near-VIP UART TX+RX); SPI/I2C/AXI near-VIP remain later stories.
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

/// SPI **master** Mode-0-ish byte shifter (FR82) — not a full multi-mode stack.
///
/// Accepts `start && !busy`; latches `tx_data` into `mosi_byte`; for 8 cycles drives
/// `cs_n` low, `sclk` high, `mosi` = MSB-first of the shift register, and samples
/// `miso` into the LSB of the shift (RX not exported — ABI unchanged).
///
/// Documented subset: CPOL=0 / CPHA=0 *toy* (1 bit/clk; idle `sclk=0`, `cs_n=1`).
/// Non-goals: other CPOL/CPHA, multi-CS, DMA, slave mode, generator closures (Epic 29).
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
        s.add_output("mosi_byte", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("busy", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("cs_n", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("sclk", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("mosi", GroundType::UInt { width: 1 }, Span::default());

        s.declare_reg("hold", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("shift", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("busy_r", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("bit_idx", GroundType::UInt { width: 4 }, Span::default());

        s.declare_wire("c0_1", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("c1_1", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("c0_4", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire("c1_4", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire("c7_4", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire("c0_8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("c1_8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("cff_8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("c80_8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("accept", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("is_last", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("msb_masked", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("mosi_bit", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("shift_shl", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("shift_shl8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("miso8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("shift_in", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("bit_idx_p1", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire("busy_cont", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("next_busy", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire(
            "next_bit_busy",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.declare_wire(
            "next_bit_gated",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.declare_wire(
            "next_bit_idx",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.declare_wire(
            "next_shift_busy",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire("next_shift", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("next_hold", GroundType::UInt { width: 8 }, Span::default());

        s.begin_combinational(Span::default());
        s.assign_lit("c0_1", 0, Span::default());
        s.assign_lit("c1_1", 1, Span::default());
        s.assign_lit("c0_4", 0, Span::default());
        s.assign_lit("c1_4", 1, Span::default());
        s.assign_lit("c7_4", 7, Span::default());
        s.assign_lit("c0_8", 0, Span::default());
        s.assign_lit("c1_8", 1, Span::default());
        s.assign_lit("cff_8", 0xFF, Span::default());
        s.assign_lit("c80_8", 0x80, Span::default());
        s.assign_mux("accept", "busy_r", "c0_1", "start", Span::default());
        s.assign_eq("is_last", "bit_idx", "c7_4", Span::default());
        s.assign_and("msb_masked", "shift", "c80_8", Span::default());
        s.assign_eq("mosi_bit", "msb_masked", "c80_8", Span::default());
        s.assign_mux("mosi", "busy_r", "mosi_bit", "c0_1", Span::default());
        // cs_n active-low while busy; idle high
        s.assign_mux("cs_n", "busy_r", "c0_1", "c1_1", Span::default());
        s.assign_mux("sclk", "busy_r", "c1_1", "c0_1", Span::default());
        s.assign_net("mosi_byte", "hold", Span::default());
        s.assign_net("busy", "busy_r", Span::default());

        s.assign_shl("shift_shl", "shift", "c1_8", Span::default());
        s.assign_and("shift_shl8", "shift_shl", "cff_8", Span::default());
        s.assign_mux("miso8", "miso", "c1_8", "c0_8", Span::default());
        s.assign_or("shift_in", "shift_shl8", "miso8", Span::default());
        s.assign_add("bit_idx_p1", "bit_idx", "c1_4", Span::default());
        s.assign_mux("busy_cont", "is_last", "c0_1", "busy_r", Span::default());
        s.assign_mux("next_busy", "accept", "c1_1", "busy_cont", Span::default());
        s.assign_mux(
            "next_bit_busy",
            "is_last",
            "c0_4",
            "bit_idx_p1",
            Span::default(),
        );
        s.assign_mux(
            "next_bit_gated",
            "busy_r",
            "next_bit_busy",
            "c0_4",
            Span::default(),
        );
        s.assign_mux(
            "next_bit_idx",
            "accept",
            "c0_4",
            "next_bit_gated",
            Span::default(),
        );
        s.assign_mux(
            "next_shift_busy",
            "busy_r",
            "shift_in",
            "shift",
            Span::default(),
        );
        s.assign_mux(
            "next_shift",
            "accept",
            "tx_data",
            "next_shift_busy",
            Span::default(),
        );
        s.assign_mux("next_hold", "accept", "tx_data", "hold", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_from("busy_r", "next_busy", Span::default());
        s.assign_reg_d_from("bit_idx", "next_bit_idx", Span::default());
        s.assign_reg_d_from("shift", "next_shift", Span::default());
        s.assign_reg_d_from("hold", "next_hold", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

/// I2C **master** START + 8-data + STOP bit-bang (FR82) — not multi-master / SMBus.
///
/// Accepts `start && !busy`; latches `tx_data` into `tx_byte`; 10 busy cycles:
/// START (`scl=1`,`sda_out=0`) → 8 MSB-first data bits (`scl=1`) → STOP (`scl=1`,`sda_out=1`).
/// Idle: both lines high. `sda_in` sampled into sticky `ack_sample` (not exported).
///
/// Documented subset: 1 bit/clk teaching toy (no half-period SCL, no ACK/NACK drive).
/// Non-goals: clock stretch, arbitration, 10-bit addr, slave, generator closures (Epic 29).
pub struct I2cMaster;

impl Elaboratable for I2cMaster {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("I2cMaster");
        s.begin_module("I2cMaster", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("start", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("tx_data", GroundType::UInt { width: 8 }, Span::default());
        s.add_input("sda_in", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("tx_byte", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("busy", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("scl", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("sda_out", GroundType::UInt { width: 1 }, Span::default());

        s.declare_reg("hold", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("shift", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("busy_r", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("phase", GroundType::UInt { width: 4 }, Span::default());
        s.declare_reg("ack_sample", GroundType::UInt { width: 1 }, Span::default());

        s.declare_wire("c0_1", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("c1_1", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("c0_4", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire("c1_4", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire("c9_4", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire("c0_8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("c1_8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("cff_8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("c80_8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("accept", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire(
            "is_start_ph",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.declare_wire("is_stop_ph", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("is_data", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("is_data2", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("is_data3", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("msb_masked", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("data_bit", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire(
            "sda_data_or_stop",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.declare_wire("sda_active", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("shift_shl", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("shift_shl8", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("phase_p1", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire("busy_cont", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire("next_busy", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire(
            "next_phase_busy",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.declare_wire(
            "next_phase_gated",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.declare_wire("next_phase", GroundType::UInt { width: 4 }, Span::default());
        s.declare_wire(
            "next_shift_data",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire("next_shift", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("next_hold", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("next_ack", GroundType::UInt { width: 1 }, Span::default());

        s.begin_combinational(Span::default());
        s.assign_lit("c0_1", 0, Span::default());
        s.assign_lit("c1_1", 1, Span::default());
        s.assign_lit("c0_4", 0, Span::default());
        s.assign_lit("c1_4", 1, Span::default());
        s.assign_lit("c9_4", 9, Span::default());
        s.assign_lit("c0_8", 0, Span::default());
        s.assign_lit("c1_8", 1, Span::default());
        s.assign_lit("cff_8", 0xFF, Span::default());
        s.assign_lit("c80_8", 0x80, Span::default());
        s.assign_mux("accept", "busy_r", "c0_1", "start", Span::default());
        s.assign_eq("is_start_ph", "phase", "c0_4", Span::default());
        s.assign_eq("is_stop_ph", "phase", "c9_4", Span::default());
        // is_data3 = busy && !start_ph && !stop_ph
        s.assign_mux("is_data", "is_start_ph", "c0_1", "c1_1", Span::default());
        s.assign_mux("is_data2", "is_stop_ph", "c0_1", "is_data", Span::default());
        s.assign_and("is_data3", "is_data2", "busy_r", Span::default());

        s.assign_and("msb_masked", "shift", "c80_8", Span::default());
        s.assign_eq("data_bit", "msb_masked", "c80_8", Span::default());
        s.assign_mux(
            "sda_data_or_stop",
            "is_stop_ph",
            "c1_1",
            "data_bit",
            Span::default(),
        );
        // START forces sda=0; else data/stop; idle high when !busy
        s.assign_mux(
            "sda_active",
            "is_start_ph",
            "c0_1",
            "sda_data_or_stop",
            Span::default(),
        );
        s.assign_mux("sda_out", "busy_r", "sda_active", "c1_1", Span::default());
        // scl idle/START/STOP/data all high in this 1-bit/clk toy
        s.assign_lit("scl", 1, Span::default());
        s.assign_net("tx_byte", "hold", Span::default());
        s.assign_net("busy", "busy_r", Span::default());

        s.assign_shl("shift_shl", "shift", "c1_8", Span::default());
        s.assign_and("shift_shl8", "shift_shl", "cff_8", Span::default());
        s.assign_add("phase_p1", "phase", "c1_4", Span::default());
        s.assign_mux("busy_cont", "is_stop_ph", "c0_1", "busy_r", Span::default());
        s.assign_mux("next_busy", "accept", "c1_1", "busy_cont", Span::default());
        s.assign_mux(
            "next_phase_busy",
            "is_stop_ph",
            "c0_4",
            "phase_p1",
            Span::default(),
        );
        s.assign_mux(
            "next_phase_gated",
            "busy_r",
            "next_phase_busy",
            "c0_4",
            Span::default(),
        );
        s.assign_mux(
            "next_phase",
            "accept",
            "c0_4",
            "next_phase_gated",
            Span::default(),
        );
        s.assign_mux(
            "next_shift_data",
            "is_data3",
            "shift_shl8",
            "shift",
            Span::default(),
        );
        s.assign_mux(
            "next_shift",
            "accept",
            "tx_data",
            "next_shift_data",
            Span::default(),
        );
        s.assign_mux("next_hold", "accept", "tx_data", "hold", Span::default());
        s.assign_mux(
            "next_ack",
            "is_data3",
            "sda_in",
            "ack_sample",
            Span::default(),
        );
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_from("busy_r", "next_busy", Span::default());
        s.assign_reg_d_from("phase", "next_phase", Span::default());
        s.assign_reg_d_from("shift", "next_shift", Span::default());
        s.assign_reg_d_from("hold", "next_hold", Span::default());
        s.assign_reg_d_from("ack_sample", "next_ack", Span::default());
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

    fn spi_drive(sim: &mut Sim, rst: u64, start: u64, tx_data: u64, miso: u64) {
        let mut pv = PortValues::default();
        pv.set("rst", rst);
        pv.set("start", start);
        pv.set("tx_data", tx_data);
        pv.set("miso", miso);
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
            v.contains("cs_n") && v.contains("sclk") && v.contains("mosi"),
            "FR82 SPI must emit serial pins"
        );

        let mut sim = Sim::new(SpiMaster::elaborate().unwrap());
        spi_drive(&mut sim, 1, 0, 0, 0);
        assert_eq!(sim.ports().get("busy"), Some(0));
        assert_eq!(sim.ports().get("cs_n"), Some(1));
        assert_eq!(sim.ports().get("sclk"), Some(0));

        // 0xA5 = 0b1010_0101 → MSB first: 1,0,1,0,0,1,0,1
        spi_drive(&mut sim, 0, 1, 0xA5, 0);
        assert_eq!(sim.ports().get("busy"), Some(1));
        assert_eq!(sim.ports().get("mosi_byte"), Some(0xA5));
        assert_eq!(sim.ports().get("cs_n"), Some(0));
        assert_eq!(sim.ports().get("sclk"), Some(1));
        assert_eq!(sim.ports().get("mosi"), Some(1)); // MSB

        let expected = [1u64, 0, 1, 0, 0, 1, 0, 1];
        for (i, &bit) in expected.iter().enumerate().skip(1) {
            spi_drive(&mut sim, 0, 0, 0, 0);
            assert_eq!(sim.ports().get("mosi"), Some(bit), "SPI bit {i}");
            assert_eq!(sim.ports().get("busy"), Some(1));
        }
        // last bit cycle already shown; one more tick clears busy
        spi_drive(&mut sim, 0, 0, 0, 0);
        assert_eq!(sim.ports().get("busy"), Some(0));
        assert_eq!(sim.ports().get("cs_n"), Some(1));

        // busy-gated: start while busy must not replace hold
        spi_drive(&mut sim, 0, 1, 0x3C, 0);
        assert_eq!(sim.ports().get("busy"), Some(1));
        spi_drive(&mut sim, 0, 1, 0xFF, 0);
        assert_eq!(sim.ports().get("mosi_byte"), Some(0x3C));
    }

    fn i2c_drive(sim: &mut Sim, rst: u64, start: u64, tx_data: u64, sda_in: u64) {
        let mut pv = PortValues::default();
        pv.set("rst", rst);
        pv.set("start", start);
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
            v.contains("scl") && v.contains("sda_out"),
            "FR82 I2C must emit scl/sda_out"
        );

        let mut sim = Sim::new(I2cMaster::elaborate().unwrap());
        i2c_drive(&mut sim, 1, 0, 0, 1);
        assert_eq!(sim.ports().get("busy"), Some(0));
        assert_eq!(sim.ports().get("scl"), Some(1));
        assert_eq!(sim.ports().get("sda_out"), Some(1)); // idle high

        // 0xA5 MSB-first after START
        i2c_drive(&mut sim, 0, 1, 0xA5, 1);
        assert_eq!(sim.ports().get("busy"), Some(1));
        assert_eq!(sim.ports().get("tx_byte"), Some(0xA5));
        assert_eq!(sim.ports().get("sda_out"), Some(0)); // START

        let data_bits = [1u64, 0, 1, 0, 0, 1, 0, 1];
        for (i, &bit) in data_bits.iter().enumerate() {
            i2c_drive(&mut sim, 0, 0, 0, 1);
            assert_eq!(sim.ports().get("sda_out"), Some(bit), "I2C data bit {i}");
            assert_eq!(sim.ports().get("busy"), Some(1));
        }
        // STOP
        i2c_drive(&mut sim, 0, 0, 0, 1);
        assert_eq!(sim.ports().get("sda_out"), Some(1));
        assert_eq!(sim.ports().get("busy"), Some(1));
        i2c_drive(&mut sim, 0, 0, 0, 1);
        assert_eq!(sim.ports().get("busy"), Some(0));
        assert_eq!(sim.ports().get("sda_out"), Some(1));

        i2c_drive(&mut sim, 0, 1, 0x42, 1);
        i2c_drive(&mut sim, 0, 1, 0xFF, 1);
        assert_eq!(sim.ports().get("tx_byte"), Some(0x42));
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
