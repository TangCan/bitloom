#![allow(unused_imports)]
use crate::{
    Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span,
    SynthesizableClosureViolation, diagnose_synthesizable_closure_violations,
};

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
