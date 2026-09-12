//! ATDD Story 101.2 / FR168 — SPI + I2C + AXI handwritten FL（三者皆须）.
//!
//! ≠ FR163 UartRx alone；≠ GeneratedFunctional alone；≠「至少一项」.

use bitloom_hir::PortValues;
use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::{Axi4LiteSlave, I2cMaster, SpiMaster};
use bitloom_sim::{
    AbstractionView, Axi4LiteSlaveFunctional, I2cMasterFunctional, IpDualModelMatrix,
    SpiMasterFunctional,
};
use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr168_docs_contract() {
    let text = read("docs/fr168-spi-i2c-axi-handwritten-fl.md");
    assert!(text.contains("FR168") && text.contains("Bitloom"));
    assert!(
        text.contains("SpiMasterFunctional")
            && text.contains("I2cMasterFunctional")
            && text.contains("Axi4LiteSlaveFunctional")
    );
    assert!(
        text.contains("verify_spi_master_handwritten")
            && text.contains("verify_i2c_master_handwritten")
            && text.contains("verify_axi4_lite_slave_handwritten")
    );
    assert!(
        text.contains("FR163")
            && (text.contains("alone") || text.contains("≠") || text.contains("不得"))
    );
    assert!(
        text.contains("GeneratedFunctional")
            && (text.contains("alone") || text.contains("≠") || text.contains("不得"))
    );
}

#[test]
fn fr168_spi_master_handwritten_pass() {
    let st =
        IpDualModelMatrix::new().verify_spi_master_handwritten(SpiMaster::elaborate().unwrap());
    assert!(st.is_pass(), "{st:?}");
}

#[test]
fn fr168_i2c_master_handwritten_pass() {
    let st =
        IpDualModelMatrix::new().verify_i2c_master_handwritten(I2cMaster::elaborate().unwrap());
    assert!(st.is_pass(), "{st:?}");
}

#[test]
fn fr168_axi4_lite_slave_handwritten_pass() {
    let st = IpDualModelMatrix::new()
        .verify_axi4_lite_slave_handwritten(Axi4LiteSlave::elaborate().unwrap());
    assert!(st.is_pass(), "{st:?}");
}

#[test]
fn fr168_deliberate_spi_mismatch_fails() {
    struct Bad;
    impl AbstractionView for Bad {
        fn cycle(&mut self, inputs: &PortValues) -> PortValues {
            let mut o = inputs.clone();
            o.set("mosi_byte", 0xdead);
            o.set("rx_data", 0xbeef);
            o.set("rx_valid", 1);
            o.set("busy", 1);
            o.set("cs_n", 0);
            o.set("sclk", 1);
            o.set("mosi", 1);
            o
        }
    }
    let st = IpDualModelMatrix::new()
        .verify_spi_master_handwritten_with(SpiMaster::elaborate().unwrap(), &mut Bad);
    assert!(!st.is_pass());
}

#[test]
fn fr168_deliberate_i2c_mismatch_fails() {
    struct Bad;
    impl AbstractionView for Bad {
        fn cycle(&mut self, inputs: &PortValues) -> PortValues {
            let mut o = inputs.clone();
            o.set("tx_byte", 0xff);
            o.set("busy", 1);
            o.set("scl", 0);
            o.set("sda_out", 0);
            o.set("rx_data", 0xaa);
            o.set("rx_valid", 1);
            o.set("ack_error", 1);
            o
        }
    }
    let st = IpDualModelMatrix::new()
        .verify_i2c_master_handwritten_with(I2cMaster::elaborate().unwrap(), &mut Bad);
    assert!(!st.is_pass());
}

#[test]
fn fr168_deliberate_axi_mismatch_fails() {
    struct Bad;
    impl AbstractionView for Bad {
        fn cycle(&mut self, inputs: &PortValues) -> PortValues {
            let mut o = inputs.clone();
            o.set("s_axi_awready", 0);
            o.set("s_axi_wready", 0);
            o.set("s_axi_bresp", 3);
            o.set("s_axi_bvalid", 1);
            o.set("s_axi_arready", 0);
            o.set("s_axi_rdata", 0xbad);
            o.set("s_axi_rresp", 3);
            o.set("s_axi_rvalid", 1);
            o
        }
    }
    let st = IpDualModelMatrix::new()
        .verify_axi4_lite_slave_handwritten_with(Axi4LiteSlave::elaborate().unwrap(), &mut Bad);
    assert!(!st.is_pass());
}

#[test]
fn fr168_exports_all_three_functionals() {
    let _ = SpiMasterFunctional::new();
    let _ = I2cMasterFunctional::new();
    let _ = Axi4LiteSlaveFunctional::new();
    let dual = read("crates/bitloom-sim/src/ip_dual.rs");
    assert!(
        dual.contains("SpiMasterFunctional")
            && dual.contains("I2cMasterFunctional")
            && dual.contains("Axi4LiteSlaveFunctional")
            && dual.contains("verify_spi_master_handwritten")
            && dual.contains("verify_i2c_master_handwritten")
            && dual.contains("verify_axi4_lite_slave_handwritten")
            && dual.contains("FR168")
    );
    assert!(
        dual.contains("spi_master_dual_stimulus")
            && dual.contains("i2c_master_dual_stimulus")
            && dual.contains("axi4_lite_slave_dual_stimulus"),
        "must pin dual_stimulus fixtures for all three"
    );
    assert!(
        !dual.contains("GeneratedFunctional::")
            || !dual
                .lines()
                .any(|l| l.contains("SpiMasterFunctional") && l.contains("GeneratedFunctional")),
        "handwritten FL must not delegate to GeneratedFunctional"
    );
}

#[test]
fn fr168_not_satisfied_by_uart_rx_or_generated_alone() {
    let dual = read("crates/bitloom-sim/src/ip_dual.rs");
    assert!(
        dual.contains("SpiMasterFunctional")
            && dual.contains("I2cMasterFunctional")
            && dual.contains("Axi4LiteSlaveFunctional")
            && dual.contains("UartRxFunctional"),
        "FR168 three handwritten must coexist with FR163 UartRx (alone ≠ FR168)"
    );
    assert!(
        dual.contains("verify_spi_master_handwritten")
            && dual.contains("verify_uart_rx_handwritten")
            && dual.contains("verify_generated_rst_compare"),
        "handwritten SPI/I2C/AXI path must be distinct from UartRx and GeneratedFunctional"
    );
    let nfr = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic101-spi-i2c-axi-handwritten-fl-fr168.md",
    );
    assert!(
        nfr.contains("SpiMaster") && nfr.contains("I2cMaster") && nfr.contains("Axi4LiteSlave")
    );
    assert!(nfr.contains("FR163") && (nfr.contains("alone") || nfr.contains("≠")));
}

#[test]
fn fr168_design_crate_prelude_only() {
    let prelude = read("crates/bitloom-prelude/Cargo.toml");
    let deps = prelude
        .split("[dependencies]")
        .nth(1)
        .and_then(|s| s.split('[').next())
        .expect("[dependencies]");
    assert!(
        !deps.contains("bitloom-sim"),
        "design crate [dependencies] must not include bitloom-sim"
    );
    assert!(
        prelude.contains("Bitloom") || prelude.contains("bitloom"),
        "public brand Bitloom"
    );
}
