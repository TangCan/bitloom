#![allow(unused_imports)]
use crate::{
    Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span,
    SynthesizableClosureViolation, diagnose_synthesizable_closure_violations,
};

/// AXI4-Lite **near-VIP slave** multi-register window (FR82 baseline → FR98 / Epic 43.5).
///
/// Documented widths: **ADDR=8**, **DATA=32**, `wstrb` 4-bit. Word window
/// `0x00` / `0x04` / `0x08` / `0x0C` → `data0_r`..`data3_r`.
///
/// - **Write:** independently capture one AW and one W; commit exactly once when
///   both are present. Same-edge AW/W still produces BVALID after one tick.
/// - **Read:** independently snapshot the decoded register on AR acceptance,
///   including during B stalls. Concurrent same-address reads see the old value.
/// - **Ready:** depends only on registered slot/response occupancy. Responses
///   hold until ready; synchronous reset clears the bank and all pending state.
/// - Unmapped/unaligned reads return zero and writes are ignored, always OKAY.
///
/// Non-goals (A4): Full AXI (burst/ID/QoS), interconnect, multi-slave arrays,
/// commercial VIP co-sim, GPIO VIP, generator closures (Epic 29).
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

        for name in ["data0_r", "data1_r", "data2_r", "data3_r", "rdata_r"] {
            s.declare_reg(name, GroundType::UInt { width: 32 }, Span::default());
        }
        s.declare_reg("bvalid_r", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("rvalid_r", GroundType::UInt { width: 1 }, Span::default());

        for (name, width) in [
            ("aw_pending_r", 1),
            ("w_pending_r", 1),
            ("awaddr_r", 8),
            ("wdata_r", 32),
            ("wstrb_r", 4),
        ] {
            s.declare_reg(name, GroundType::UInt { width }, Span::default());
            s.declare_wire(
                &format!("next_{name}"),
                GroundType::UInt { width },
                Span::default(),
            );
        }
        for (name, width) in [("write_addr", 8), ("write_data", 32), ("write_strb", 4)] {
            s.declare_wire(name, GroundType::UInt { width }, Span::default());
        }

        for (n, w) in [
            ("c0_1", 1u32),
            ("c1_1", 1),
            ("c1_4", 4),
            ("c2_4", 4),
            ("c4_4", 4),
            ("c8_4", 4),
            ("c0_8", 8),
            ("c4_8", 8),
            ("c8_8", 8),
            ("c12_8", 8),
            ("c0_32", 32),
            ("c_ff", 32),
            ("c_ff00", 32),
            ("c_ff0000", 32),
            ("c_ff000000", 32),
            ("c_ffff_ffff", 32),
        ] {
            s.declare_wire(n, GroundType::UInt { width: w }, Span::default());
        }

        for n in [
            "aw_ready",
            "ar_ready",
            "aw_fire",
            "w_fire",
            "do_write",
            "ar_fire",
            "do_read",
            "b_fire",
            "r_fire",
            "b_clear",
            "b_hold",
            "next_bvalid",
            "r_clear",
            "r_hold",
            "next_rvalid",
            "not_bvalid",
            "not_rvalid",
            "w_ready",
            "aw_empty",
            "w_empty",
            "aw_have",
            "w_have",
            "aw_eq0",
            "aw_eq1",
            "aw_eq2",
            "aw_eq3",
            "ar_eq0",
            "ar_eq1",
            "ar_eq2",
            "ar_eq3",
            "ar_lo",
            "wr0",
            "wr1",
            "wr2",
            "wr3",
        ] {
            s.declare_wire(n, GroundType::UInt { width: 1 }, Span::default());
        }

        for n in ["strb0", "strb1", "strb2", "strb3"] {
            s.declare_wire(n, GroundType::UInt { width: 4 }, Span::default());
        }

        for n in [
            "lane0",
            "lane1",
            "lane2",
            "lane3",
            "lane01",
            "lane23",
            "byte_mask",
            "byte_mask_n",
            "wdata_m",
            "old0_m",
            "old1_m",
            "old2_m",
            "old3_m",
            "merged0",
            "merged1",
            "merged2",
            "merged3",
            "next_data0",
            "next_data1",
            "next_data2",
            "next_data3",
            "rdata_lo1",
            "rdata_lo",
            "rdata_hi1",
            "rdata_hi",
            "rdata_mux",
            "next_rdata",
        ] {
            s.declare_wire(n, GroundType::UInt { width: 32 }, Span::default());
        }

        s.begin_combinational(Span::default());
        s.assign_lit("c0_1", 0, Span::default());
        s.assign_lit("c1_1", 1, Span::default());
        s.assign_lit("c1_4", 1, Span::default());
        s.assign_lit("c2_4", 2, Span::default());
        s.assign_lit("c4_4", 4, Span::default());
        s.assign_lit("c8_4", 8, Span::default());
        s.assign_lit("c0_8", 0, Span::default());
        s.assign_lit("c4_8", 4, Span::default());
        s.assign_lit("c8_8", 8, Span::default());
        s.assign_lit("c12_8", 12, Span::default());
        s.assign_lit("c0_32", 0, Span::default());
        s.assign_lit("c_ff", 0x0000_00FF, Span::default());
        s.assign_lit("c_ff00", 0x0000_FF00, Span::default());
        s.assign_lit("c_ff0000", 0x00FF_0000, Span::default());
        s.assign_lit("c_ff000000", 0xFF00_0000, Span::default());
        s.assign_lit("c_ffff_ffff", 0xFFFF_FFFF, Span::default());

        s.assign_xor("not_bvalid", "bvalid_r", "c1_1", Span::default());
        s.assign_xor("not_rvalid", "rvalid_r", "c1_1", Span::default());
        s.assign_xor("aw_empty", "aw_pending_r", "c1_1", Span::default());
        s.assign_xor("w_empty", "w_pending_r", "c1_1", Span::default());
        s.assign_and("aw_ready", "not_bvalid", "aw_empty", Span::default());
        s.assign_and("w_ready", "not_bvalid", "w_empty", Span::default());
        s.assign_net("ar_ready", "not_rvalid", Span::default());
        s.assign_net("s_axi_awready", "aw_ready", Span::default());
        s.assign_net("s_axi_wready", "w_ready", Span::default());
        s.assign_net("s_axi_arready", "ar_ready", Span::default());
        s.assign_net("s_axi_bvalid", "bvalid_r", Span::default());
        s.assign_net("s_axi_rvalid", "rvalid_r", Span::default());
        s.assign_net("s_axi_rdata", "rdata_r", Span::default());
        s.assign_lit("s_axi_bresp", 0, Span::default());
        s.assign_lit("s_axi_rresp", 0, Span::default());

        s.assign_and("aw_fire", "s_axi_awvalid", "aw_ready", Span::default());
        s.assign_and("w_fire", "s_axi_wvalid", "w_ready", Span::default());
        s.assign_or("aw_have", "aw_pending_r", "aw_fire", Span::default());
        s.assign_or("w_have", "w_pending_r", "w_fire", Span::default());
        s.assign_and("do_write", "aw_have", "w_have", Span::default());
        s.assign_mux(
            "next_aw_pending_r",
            "do_write",
            "c0_1",
            "aw_have",
            Span::default(),
        );
        s.assign_mux(
            "next_w_pending_r",
            "do_write",
            "c0_1",
            "w_have",
            Span::default(),
        );
        for (next, fire, input, old) in [
            ("next_awaddr_r", "aw_fire", "s_axi_awaddr", "awaddr_r"),
            ("next_wdata_r", "w_fire", "s_axi_wdata", "wdata_r"),
            ("next_wstrb_r", "w_fire", "s_axi_wstrb", "wstrb_r"),
        ] {
            s.assign_mux(next, fire, input, old, Span::default());
        }
        for (selected, pending, saved, live) in [
            ("write_addr", "aw_pending_r", "awaddr_r", "s_axi_awaddr"),
            ("write_data", "w_pending_r", "wdata_r", "s_axi_wdata"),
            ("write_strb", "w_pending_r", "wstrb_r", "s_axi_wstrb"),
        ] {
            s.assign_mux(selected, pending, saved, live, Span::default());
        }
        s.assign_and("ar_fire", "s_axi_arvalid", "ar_ready", Span::default());
        s.assign_net("do_read", "ar_fire", Span::default());
        s.assign_and("b_fire", "bvalid_r", "s_axi_bready", Span::default());
        s.assign_and("r_fire", "rvalid_r", "s_axi_rready", Span::default());

        s.assign_eq("aw_eq0", "write_addr", "c0_8", Span::default());
        s.assign_eq("aw_eq1", "write_addr", "c4_8", Span::default());
        s.assign_eq("aw_eq2", "write_addr", "c8_8", Span::default());
        s.assign_eq("aw_eq3", "write_addr", "c12_8", Span::default());
        s.assign_eq("ar_eq0", "s_axi_araddr", "c0_8", Span::default());
        s.assign_eq("ar_eq1", "s_axi_araddr", "c4_8", Span::default());
        s.assign_eq("ar_eq2", "s_axi_araddr", "c8_8", Span::default());
        s.assign_eq("ar_eq3", "s_axi_araddr", "c12_8", Span::default());
        s.assign_and("wr0", "do_write", "aw_eq0", Span::default());
        s.assign_and("wr1", "do_write", "aw_eq1", Span::default());
        s.assign_and("wr2", "do_write", "aw_eq2", Span::default());
        s.assign_and("wr3", "do_write", "aw_eq3", Span::default());

        s.assign_and("strb0", "write_strb", "c1_4", Span::default());
        s.assign_and("strb1", "write_strb", "c2_4", Span::default());
        s.assign_and("strb2", "write_strb", "c4_4", Span::default());
        s.assign_and("strb3", "write_strb", "c8_4", Span::default());
        s.assign_mux("lane0", "strb0", "c_ff", "c0_32", Span::default());
        s.assign_mux("lane1", "strb1", "c_ff00", "c0_32", Span::default());
        s.assign_mux("lane2", "strb2", "c_ff0000", "c0_32", Span::default());
        s.assign_mux("lane3", "strb3", "c_ff000000", "c0_32", Span::default());
        s.assign_or("lane01", "lane0", "lane1", Span::default());
        s.assign_or("lane23", "lane2", "lane3", Span::default());
        s.assign_or("byte_mask", "lane01", "lane23", Span::default());
        s.assign_xor("byte_mask_n", "byte_mask", "c_ffff_ffff", Span::default());
        s.assign_and("wdata_m", "write_data", "byte_mask", Span::default());

        s.assign_and("old0_m", "data0_r", "byte_mask_n", Span::default());
        s.assign_or("merged0", "wdata_m", "old0_m", Span::default());
        s.assign_mux("next_data0", "wr0", "merged0", "data0_r", Span::default());

        s.assign_and("old1_m", "data1_r", "byte_mask_n", Span::default());
        s.assign_or("merged1", "wdata_m", "old1_m", Span::default());
        s.assign_mux("next_data1", "wr1", "merged1", "data1_r", Span::default());

        s.assign_and("old2_m", "data2_r", "byte_mask_n", Span::default());
        s.assign_or("merged2", "wdata_m", "old2_m", Span::default());
        s.assign_mux("next_data2", "wr2", "merged2", "data2_r", Span::default());

        s.assign_and("old3_m", "data3_r", "byte_mask_n", Span::default());
        s.assign_or("merged3", "wdata_m", "old3_m", Span::default());
        s.assign_mux("next_data3", "wr3", "merged3", "data3_r", Span::default());

        // rdata = ar_eq0 ? data0 : ar_eq1 ? data1 : ar_eq2 ? data2 : ar_eq3 ? data3 : 0
        s.assign_mux("rdata_lo1", "ar_eq1", "data1_r", "c0_32", Span::default());
        s.assign_mux(
            "rdata_lo",
            "ar_eq0",
            "data0_r",
            "rdata_lo1",
            Span::default(),
        );
        s.assign_mux("rdata_hi1", "ar_eq3", "data3_r", "c0_32", Span::default());
        s.assign_mux(
            "rdata_hi",
            "ar_eq2",
            "data2_r",
            "rdata_hi1",
            Span::default(),
        );
        s.assign_or("ar_lo", "ar_eq0", "ar_eq1", Span::default());
        s.assign_mux(
            "rdata_mux",
            "ar_lo",
            "rdata_lo",
            "rdata_hi",
            Span::default(),
        );
        s.assign_mux(
            "next_rdata",
            "do_read",
            "rdata_mux",
            "rdata_r",
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
        for name in [
            "aw_pending_r",
            "w_pending_r",
            "awaddr_r",
            "wdata_r",
            "wstrb_r",
        ] {
            s.assign_reg_d_from(name, &format!("next_{name}"), Span::default());
        }
        s.assign_reg_d_from("data0_r", "next_data0", Span::default());
        s.assign_reg_d_from("data1_r", "next_data1", Span::default());
        s.assign_reg_d_from("data2_r", "next_data2", Span::default());
        s.assign_reg_d_from("data3_r", "next_data3", Span::default());
        s.assign_reg_d_from("rdata_r", "next_rdata", Span::default());
        s.assign_reg_d_from("bvalid_r", "next_bvalid", Span::default());
        s.assign_reg_d_from("rvalid_r", "next_rvalid", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}
