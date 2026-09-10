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
/// - **Write (A1):** when `awvalid && wvalid && !bvalid`, decode `awaddr`, merge
///   `wdata` through `wstrb` byte lanes into the hit register (unmapped write
///   ignored); hold `bvalid` until `bready`.
/// - **Read (A1):** when `arvalid && !rvalid && !bvalid`, latch `rdata` from the
///   decoded register (unmapped → 0); hold `rvalid` until `rready`.
/// - **Ready:** combinatorial when the corresponding response channel is idle;
///   write preferred over read in the same cycle.
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
            "can_ar",
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
        s.assign_net("aw_ready", "not_bvalid", Span::default());
        s.assign_and("can_ar", "not_rvalid", "not_bvalid", Span::default());
        s.assign_net("ar_ready", "can_ar", Span::default());
        s.assign_net("s_axi_awready", "aw_ready", Span::default());
        s.assign_net("s_axi_wready", "aw_ready", Span::default());
        s.assign_net("s_axi_arready", "ar_ready", Span::default());
        s.assign_net("s_axi_bvalid", "bvalid_r", Span::default());
        s.assign_net("s_axi_rvalid", "rvalid_r", Span::default());
        s.assign_net("s_axi_rdata", "rdata_r", Span::default());
        s.assign_lit("s_axi_bresp", 0, Span::default());
        s.assign_lit("s_axi_rresp", 0, Span::default());

        s.assign_and("aw_fire", "s_axi_awvalid", "aw_ready", Span::default());
        s.assign_and("w_fire", "s_axi_wvalid", "aw_ready", Span::default());
        s.assign_and("do_write", "aw_fire", "w_fire", Span::default());
        s.assign_and("ar_fire", "s_axi_arvalid", "ar_ready", Span::default());
        s.assign_mux("do_read", "do_write", "c0_1", "ar_fire", Span::default());
        s.assign_and("b_fire", "bvalid_r", "s_axi_bready", Span::default());
        s.assign_and("r_fire", "rvalid_r", "s_axi_rready", Span::default());

        s.assign_eq("aw_eq0", "s_axi_awaddr", "c0_8", Span::default());
        s.assign_eq("aw_eq1", "s_axi_awaddr", "c4_8", Span::default());
        s.assign_eq("aw_eq2", "s_axi_awaddr", "c8_8", Span::default());
        s.assign_eq("aw_eq3", "s_axi_awaddr", "c12_8", Span::default());
        s.assign_eq("ar_eq0", "s_axi_araddr", "c0_8", Span::default());
        s.assign_eq("ar_eq1", "s_axi_araddr", "c4_8", Span::default());
        s.assign_eq("ar_eq2", "s_axi_araddr", "c8_8", Span::default());
        s.assign_eq("ar_eq3", "s_axi_araddr", "c12_8", Span::default());
        s.assign_and("wr0", "do_write", "aw_eq0", Span::default());
        s.assign_and("wr1", "do_write", "aw_eq1", Span::default());
        s.assign_and("wr2", "do_write", "aw_eq2", Span::default());
        s.assign_and("wr3", "do_write", "aw_eq3", Span::default());

        s.assign_and("strb0", "s_axi_wstrb", "c1_4", Span::default());
        s.assign_and("strb1", "s_axi_wstrb", "c2_4", Span::default());
        s.assign_and("strb2", "s_axi_wstrb", "c4_4", Span::default());
        s.assign_and("strb3", "s_axi_wstrb", "c8_4", Span::default());
        s.assign_mux("lane0", "strb0", "c_ff", "c0_32", Span::default());
        s.assign_mux("lane1", "strb1", "c_ff00", "c0_32", Span::default());
        s.assign_mux("lane2", "strb2", "c_ff0000", "c0_32", Span::default());
        s.assign_mux("lane3", "strb3", "c_ff000000", "c0_32", Span::default());
        s.assign_or("lane01", "lane0", "lane1", Span::default());
        s.assign_or("lane23", "lane2", "lane3", Span::default());
        s.assign_or("byte_mask", "lane01", "lane23", Span::default());
        s.assign_xor("byte_mask_n", "byte_mask", "c_ffff_ffff", Span::default());
        s.assign_and("wdata_m", "s_axi_wdata", "byte_mask", Span::default());

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
