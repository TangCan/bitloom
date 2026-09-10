#![allow(unused_imports)]
use crate::{
    Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span,
    SynthesizableClosureViolation, diagnose_synthesizable_closure_violations,
};

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
