//! First-class IP stubs (FR37 / FR48): SyncFifo, UartTx, SpiMaster, I2cMaster,
//! Axi4LiteSlave, black-box.
//!
//! Minimal synthesizable Bitloom modules — not full protocol stacks.
//! Design crates reach these via `bitloom_prelude::ip` (only dependency: this prelude).

use crate::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// Depth-1 registered skid FIFO (single-clock).
pub struct SyncFifo;

impl Elaboratable for SyncFifo {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("SyncFifo");
        s.begin_module("SyncFifo", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("q", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("data_out", "q", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        s.assign_reg_d_from("q", "data_in", Span::default());
        s.end_process();
        s.end_module();
        s.finish()
    }
}

/// Minimal UART TX holding register (not a baud-rate shifter).
///
/// Ports: write a byte when `wr_en` is observed by capturing into `tx_byte`;
/// `tx_busy` mirrors the held-valid register for smoke.
pub struct UartTx;

impl Elaboratable for UartTx {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("UartTx");
        s.begin_module("UartTx", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("wr_en", GroundType::UInt { width: 1 }, Span::default());
        s.add_input("wr_data", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("tx_byte", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("tx_busy", GroundType::UInt { width: 1 }, Span::default());
        s.declare_reg("hold", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("busy", GroundType::UInt { width: 1 }, Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("tx_byte", "hold", Span::default());
        s.assign_net("tx_busy", "busy", Span::default());
        s.end_process();
        s.begin_sequential(Span::default());
        // Stub: always sample wr_data / wr_en (full UART would gate on !busy).
        s.assign_reg_d_from("hold", "wr_data", Span::default());
        s.assign_reg_d_from("busy", "wr_en", Span::default());
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

    #[test]
    fn sync_fifo_elaborate_emit_tick() {
        smoke_elaborate_emit_tick::<SyncFifo>("SyncFifo");
        let mut sim = Sim::new(SyncFifo::elaborate().unwrap());
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("data_in", 0);
        sim.set_inputs(pv.clone());
        sim.tick();
        pv.set("rst", 0);
        pv.set("data_in", 0x5A);
        sim.set_inputs(pv);
        sim.tick();
        assert_eq!(sim.ports().get("data_out"), Some(0x5A));
    }

    #[test]
    fn uart_tx_elaborate_emit_tick() {
        smoke_elaborate_emit_tick::<UartTx>("UartTx");
        let mut sim = Sim::new(UartTx::elaborate().unwrap());
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("wr_en", 0);
        pv.set("wr_data", 0);
        sim.set_inputs(pv.clone());
        sim.tick();
        pv.set("rst", 0);
        pv.set("wr_en", 1);
        pv.set("wr_data", 0xA5);
        sim.set_inputs(pv);
        sim.tick();
        assert_eq!(sim.ports().get("tx_byte"), Some(0xA5));
        assert_eq!(sim.ports().get("tx_busy"), Some(1));
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
}
