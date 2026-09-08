//! ATDD: Story 34.3 / FR82 — SpiMaster + I2cMaster + Axi4LiteSlave non-stub
//! elaborate→emit→tick; documented minimal subsets; no generator-closure API.

use std::fs;
use std::path::PathBuf;

use bitloom_hir::PortValues;
use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::{Axi4LiteSlave, I2cMaster, SpiMaster};
use bitloom_sim::Sim;
use bitloom_vlog::emit;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
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
fn fr82_spi_master_elaborate_emit_tick_non_stub() {
    let hir = SpiMaster::elaborate().expect("elaborate");
    assert_eq!(hir.abi_name, "SpiMaster");
    let art = emit(&hir);
    let v = &art.files[0].contents;
    assert!(v.contains("module SpiMaster"));
    assert!(
        v.contains("cs_n") && v.contains("sclk") && v.contains("mosi") && v.contains("busy"),
        "FR82 SpiMaster must emit serial pins (not hold-only stub)"
    );

    let mut sim = Sim::new(hir);
    spi_drive(&mut sim, 1, 0, 0, 0);
    spi_drive(&mut sim, 0, 1, 0x81, 0); // MSB=1, LSB=1
    assert_eq!(sim.ports().get("busy"), Some(1));
    assert_eq!(sim.ports().get("cs_n"), Some(0));
    assert_eq!(sim.ports().get("sclk"), Some(1));
    assert_eq!(sim.ports().get("mosi"), Some(1)); // MSB
    spi_drive(&mut sim, 0, 0, 0, 0);
    assert_eq!(sim.ports().get("mosi"), Some(0));
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
fn fr82_i2c_master_elaborate_emit_tick_non_stub() {
    let hir = I2cMaster::elaborate().expect("elaborate");
    assert_eq!(hir.abi_name, "I2cMaster");
    let art = emit(&hir);
    let v = &art.files[0].contents;
    assert!(v.contains("module I2cMaster"));
    assert!(
        v.contains("scl") && v.contains("sda_out") && v.contains("busy"),
        "FR82 I2cMaster must emit scl/sda_out (not hold-only stub)"
    );

    let mut sim = Sim::new(hir);
    i2c_drive(&mut sim, 1, 0, 0, 1);
    assert_eq!(sim.ports().get("sda_out"), Some(1));
    i2c_drive(&mut sim, 0, 1, 0x80, 1); // MSB=1
    assert_eq!(sim.ports().get("busy"), Some(1));
    assert_eq!(sim.ports().get("sda_out"), Some(0)); // START
    i2c_drive(&mut sim, 0, 0, 0, 1);
    assert_eq!(sim.ports().get("sda_out"), Some(1)); // first data MSB
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
fn fr82_axi4_lite_slave_elaborate_emit_tick_non_stub() {
    let hir = Axi4LiteSlave::elaborate().expect("elaborate");
    assert_eq!(hir.abi_name, "Axi4LiteSlave");
    let art = emit(&hir);
    let v = &art.files[0].contents;
    assert!(v.contains("module Axi4LiteSlave"));
    assert!(
        v.contains("s_axi_awready")
            && v.contains("s_axi_bvalid")
            && v.contains("s_axi_arready")
            && v.contains("s_axi_rvalid"),
        "FR82 AXI-Lite must emit real handshake (not mirror-valid stub)"
    );

    let mut sim = Sim::new(hir);
    axi_drive(&mut sim, 1, 0, 0, 0, 0, 0, 0);
    axi_drive(&mut sim, 0, 1, 1, 0xCAFE_BABE, 0, 0, 0);
    assert_eq!(sim.ports().get("s_axi_bvalid"), Some(1));
    axi_drive(&mut sim, 0, 0, 0, 0, 1, 0, 0);
    assert_eq!(sim.ports().get("s_axi_bvalid"), Some(0));
    axi_drive(&mut sim, 0, 0, 0, 0, 0, 1, 0);
    assert_eq!(sim.ports().get("s_axi_rvalid"), Some(1));
    assert_eq!(sim.ports().get("s_axi_rdata"), Some(0xCAFE_BABE));
}

#[test]
fn fr82_spi_i2c_axi_api_has_no_generator_closures() {
    let _ = SpiMaster::elaborate();
    let _ = I2cMaster::elaborate();
    let _ = Axi4LiteSlave::elaborate();
    let src = fs::read_to_string(workspace_root().join("crates/bitloom-prelude/src/ip.rs"))
        .expect("ip.rs");
    let spi = src
        .split("impl Elaboratable for SpiMaster")
        .nth(1)
        .and_then(|s| s.split("impl Elaboratable for I2cMaster").next())
        .unwrap_or("");
    let i2c = src
        .split("impl Elaboratable for I2cMaster")
        .nth(1)
        .and_then(|s| s.split("impl Elaboratable for Axi4LiteSlave").next())
        .unwrap_or("");
    let axi = src
        .split("impl Elaboratable for Axi4LiteSlave")
        .nth(1)
        .and_then(|s| s.split("impl Elaboratable for ExtBlackBox").next())
        .unwrap_or("");
    assert!(
        !spi.contains("Fn(")
            && !spi.contains("dyn Fn")
            && !i2c.contains("Fn(")
            && !i2c.contains("dyn Fn")
            && !axi.contains("Fn(")
            && !axi.contains("dyn Fn"),
        "Spi/I2c/Axi elaborate must not accept generator closures"
    );
}

#[test]
fn fr82_spi_i2c_axi_subset_documented() {
    let ip = fs::read_to_string(workspace_root().join("docs/ip/README.md")).expect("ip readme");
    assert!(
        ip.contains("SpiMaster")
            && ip.contains("I2cMaster")
            && ip.contains("Axi4LiteSlave")
            && (ip.contains("非 stub") || ip.contains("FR82")),
        "docs/ip must document FR82 SPI/I2C/AXI subsets"
    );
    assert!(
        ip.contains("Mode-0") || ip.contains("MSB") || ip.contains("byte shifter"),
        "SPI documented minimal subset"
    );
    assert!(
        ip.contains("START") && ip.contains("STOP"),
        "I2C documented minimal subset"
    );
    assert!(
        ip.contains("握手") || ip.contains("handshake") || ip.contains("single-register"),
        "AXI documented minimal subset"
    );
    let surface = fs::read_to_string(
        workspace_root().join("_agile-output/specs/spec-rhdl/language-surface.md"),
    )
    .expect("language-surface");
    assert!(
        surface.contains("SpiMaster") && surface.contains("FR82"),
        "language-surface must mention FR82 SPI/I2C/AXI"
    );
}
