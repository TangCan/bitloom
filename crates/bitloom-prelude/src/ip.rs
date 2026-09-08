//! First-class IP (FR37 / FR48 / FR82): SyncFifo, UartTx, SpiMaster, I2cMaster,
//! Axi4LiteSlave, black-box.
//!
//! Epic 34 / FR82 deepens FIFO + UART to **non-stub** synthesizable paths (still
//! not full protocol stacks). SPI / I2C / AXI remain Epic 22-style stubs until 34.3.
//! Design crates reach these via `bitloom_prelude::ip` only (no generator closures —
//! Epic 29).

use crate::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// Depth-4 single-clock sync FIFO with `wr_en`/`rd_en` and `full`/`empty` (FR82).
///
/// Non-goals: async/CDC FIFO ([`crate::SyncFIFO`] bridge marker), FWFT bypass,
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

/// UART TX 8N1 bit-bang at one bit per `clk` (baud = clk) — FR82 non-stub baseline.
///
/// Accepts a byte only when `!tx_busy`; drives serial `tx` (idle high) through
/// start + 8 data (LSB first) + stop. `tx_byte` holds the latched payload.
///
/// Non-goals: programmable baud divider, RX, parity, FIFO'd TX, generator closures
/// (Epic 29). Deeper protocol work needs an explicit contract change.
pub struct UartTx;

impl Elaboratable for UartTx {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("UartTx");
        s.begin_module("UartTx", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("wr_en", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("wr_data", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("tx", GroundType::UInt { width: 1 }, Span::default());
        s.add_output("tx_byte", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("tx_busy", GroundType::UInt { width: 1 }, Span::default());

        s.declare_reg("hold", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("shift_reg", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("busy", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("bit_idx", GroundType::UInt { width: 4 }, Span::default());

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
        s.declare_wire("next_busy", GroundType::UInt { width: 1 }, Span::default());
        s.declare_wire(
            "next_bit_idx",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.declare_wire(
            "next_bit_idx2",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.declare_wire(
            "next_bit_final",
            GroundType::UInt { width: 4 },
            Span::default(),
        );
        s.declare_wire(
            "next_shift2",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire(
            "next_shift_final",
            GroundType::UInt { width: 8 },
            Span::default(),
        );
        s.declare_wire("next_hold", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire(
            "busy_next_idle",
            GroundType::UInt { width: 1 },
            Span::default(),
        );
        s.declare_wire(
            "bit_when_busy",
            GroundType::UInt { width: 4 },
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

        s.assign_xor("not_start", "is_start", "c1_1", Span::default());
        s.assign_xor("not_stop", "is_stop", "c1_1", Span::default());
        s.assign_and("do_shift", "busy", "not_start", Span::default());
        s.assign_and("do_shift2", "do_shift", "not_stop", Span::default());
        s.assign_shr("shift_shr", "shift_reg", "c1_8", Span::default());
        s.assign_add("bit_idx_p1", "bit_idx", "c1_4", Span::default());
        s.assign_mux("busy_next_idle", "is_stop", "c0_1", "busy", Span::default());
        s.assign_mux(
            "next_busy",
            "accept",
            "c1_1",
            "busy_next_idle",
            Span::default(),
        );
        s.assign_mux(
            "bit_when_busy",
            "is_stop",
            "c0_4",
            "bit_idx_p1",
            Span::default(),
        );
        s.assign_mux(
            "next_bit_idx",
            "accept",
            "c0_4",
            "bit_when_busy",
            Span::default(),
        );
        s.assign_mux(
            "next_bit_idx2",
            "busy",
            "next_bit_idx",
            "c0_4",
            Span::default(),
        );
        s.assign_mux(
            "next_bit_final",
            "accept",
            "c0_4",
            "next_bit_idx2",
            Span::default(),
        );
        s.assign_mux(
            "next_shift2",
            "do_shift2",
            "shift_shr",
            "shift_reg",
            Span::default(),
        );
        s.assign_mux(
            "next_shift_final",
            "accept",
            "wr_data",
            "next_shift2",
            Span::default(),
        );
        s.assign_mux("next_hold", "accept", "wr_data", "hold", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_from("busy", "next_busy", Span::default());
        s.assign_reg_d_from("bit_idx", "next_bit_final", Span::default());
        s.assign_reg_d_from("shift_reg", "next_shift_final", Span::default());
        s.assign_reg_d_from("hold", "next_hold", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

/// Minimal SPI **master** byte buffer (not a full multi-mode / multi-slave stack).
///
/// Role: master. Stream/register surface: `start` + `tx_data[7:0]` → held `mosi_byte`,
/// `busy` mirrors start; `cs_n`/`sclk`/`mosi` are registered stubs for port semantics.
///
/// Non-goals: CPOL/CPHA modes, multi-CS, continuous DMA, slave mode.
/// Epic 34.3 may deepen; until then this remains the Epic 22 stub path.
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
        s.declare_reg("busy_r", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("cs_r", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("sclk_r", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("mosi_r", GroundType::UInt { width: 1 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("mosi_byte", "hold", Span::default());
        s.assign_net("busy", "busy_r", Span::default());
        s.assign_net("cs_n", "cs_r", Span::default());
        s.assign_net("sclk", "sclk_r", Span::default());
        s.assign_net("mosi", "mosi_r", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_from("hold", "tx_data", Span::default());
        s.assign_reg_d_from("busy_r", "start", Span::default());
        // Stub: cs_n/sclk track start; mosi samples miso for port liveness.
        s.assign_reg_d_from("cs_r", "start", Span::default());
        s.assign_reg_d_from("sclk_r", "start", Span::default());
        s.assign_reg_d_from("mosi_r", "miso", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

/// Minimal I2C **master** byte buffer (not a full multi-master / SMBUS stack).
///
/// Role: master. Register surface: `start` + `tx_data[7:0]` → held `tx_byte`;
/// `busy` mirrors start; `scl`/`sda_out` are registered stubs. `sda_in` is sampled.
///
/// Non-goals: multi-master arbitration, clock stretching FSM, 10-bit addressing, slave mode.
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
        s.declare_reg("busy_r", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("scl_r", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("sda_r", GroundType::UInt { width: 1 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("tx_byte", "hold", Span::default());
        s.assign_net("busy", "busy_r", Span::default());
        s.assign_net("scl", "scl_r", Span::default());
        s.assign_net("sda_out", "sda_r", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_from("hold", "tx_data", Span::default());
        s.assign_reg_d_from("busy_r", "start", Span::default());
        s.assign_reg_d_from("scl_r", "start", Span::default());
        s.assign_reg_d_from("sda_r", "sda_in", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

/// AXI4-Lite **minimal slave** (FR48 / Open Q7).
///
/// Documented widths: **ADDR=8**, **DATA=32**. Handshake stubs register channel valids;
/// a single `data_r` holds the last `s_axi_wdata` for `s_axi_rdata` smoke.
///
/// Non-goals: Full AXI (burst/ID/QoS), interconnect, multi-slave decode, VIP compliance.
pub struct Axi4LiteSlave;

impl Elaboratable for Axi4LiteSlave {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("Axi4LiteSlave");
        s.begin_module("Axi4LiteSlave", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        // Write address
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
        // Write data
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
        // Write response
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
        // Read address
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
        // Read data
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
        s.declare_reg("awready_r", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("wready_r", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("bvalid_r", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("arready_r", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("rvalid_r", GroundType::UInt { width: 1 }, Span::default());

        s.begin_combinational(Span::default());
        s.assign_net("s_axi_awready", "awready_r", Span::default());
        s.assign_net("s_axi_wready", "wready_r", Span::default());
        s.assign_net("s_axi_bvalid", "bvalid_r", Span::default());
        s.assign_lit("s_axi_bresp", 0, Span::default()); // OKAY
        s.assign_net("s_axi_arready", "arready_r", Span::default());
        s.assign_net("s_axi_rdata", "data_r", Span::default());
        s.assign_net("s_axi_rvalid", "rvalid_r", Span::default());
        s.assign_lit("s_axi_rresp", 0, Span::default()); // OKAY
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_from("data_r", "s_axi_wdata", Span::default());
        s.assign_reg_d_from("awready_r", "s_axi_awvalid", Span::default());
        s.assign_reg_d_from("wready_r", "s_axi_wvalid", Span::default());
        s.assign_reg_d_from("bvalid_r", "s_axi_bready", Span::default());
        s.assign_reg_d_from("arready_r", "s_axi_arvalid", Span::default());
        s.assign_reg_d_from("rvalid_r", "s_axi_rready", Span::default());
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

#[cfg(test)]
mod tests {
    use super::*;
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
        let mut pv = PortValues::default();
        pv.set("rst", rst);
        pv.set("wr_en", wr_en);
        pv.set("wr_data", wr_data);
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
    fn spi_master_elaborate_emit_tick() {
        smoke_elaborate_emit_tick::<SpiMaster>("SpiMaster");
        let mut sim = Sim::new(SpiMaster::elaborate().unwrap());
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("start", 0);
        pv.set("tx_data", 0);
        pv.set("miso", 0);
        sim.set_inputs(pv.clone());
        sim.tick();
        pv.set("rst", 0);
        pv.set("start", 1);
        pv.set("tx_data", 0x3C);
        pv.set("miso", 1);
        sim.set_inputs(pv);
        sim.tick();
        assert_eq!(sim.ports().get("mosi_byte"), Some(0x3C));
        assert_eq!(sim.ports().get("busy"), Some(1));
    }

    #[test]
    fn i2c_master_elaborate_emit_tick() {
        smoke_elaborate_emit_tick::<I2cMaster>("I2cMaster");
        let mut sim = Sim::new(I2cMaster::elaborate().unwrap());
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("start", 0);
        pv.set("tx_data", 0);
        pv.set("sda_in", 0);
        sim.set_inputs(pv.clone());
        sim.tick();
        pv.set("rst", 0);
        pv.set("start", 1);
        pv.set("tx_data", 0x42);
        pv.set("sda_in", 1);
        sim.set_inputs(pv);
        sim.tick();
        assert_eq!(sim.ports().get("tx_byte"), Some(0x42));
        assert_eq!(sim.ports().get("busy"), Some(1));
    }

    #[test]
    fn axi4_lite_slave_elaborate_emit_tick() {
        smoke_elaborate_emit_tick::<Axi4LiteSlave>("Axi4LiteSlave");
        let mut sim = Sim::new(Axi4LiteSlave::elaborate().unwrap());
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("s_axi_awaddr", 0);
        pv.set("s_axi_awvalid", 0);
        pv.set("s_axi_wdata", 0);
        pv.set("s_axi_wstrb", 0);
        pv.set("s_axi_wvalid", 0);
        pv.set("s_axi_bready", 0);
        pv.set("s_axi_araddr", 0);
        pv.set("s_axi_arvalid", 0);
        pv.set("s_axi_rready", 0);
        sim.set_inputs(pv.clone());
        sim.tick();
        pv.set("rst", 0);
        pv.set("s_axi_awvalid", 1);
        pv.set("s_axi_wvalid", 1);
        pv.set("s_axi_wdata", 0xDEAD_BEEFu64);
        pv.set("s_axi_wstrb", 0xF);
        pv.set("s_axi_bready", 1);
        pv.set("s_axi_arvalid", 1);
        pv.set("s_axi_rready", 1);
        sim.set_inputs(pv);
        sim.tick();
        assert_eq!(sim.ports().get("s_axi_rdata"), Some(0xDEAD_BEEF));
        assert_eq!(sim.ports().get("s_axi_awready"), Some(1));
        assert_eq!(sim.ports().get("s_axi_bresp"), Some(0));
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
    }
}
