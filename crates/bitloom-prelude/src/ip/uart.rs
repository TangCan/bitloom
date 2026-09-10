#![allow(unused_imports)]
use crate::{
    Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span,
    SynthesizableClosureViolation, diagnose_synthesizable_closure_violations,
};

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
