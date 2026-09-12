//! ATDD Story 95.2 / FR163 — unlisted-protocol handwritten FL (`UartRxFunctional`).
//!
//! ≠ FR135 UartTx alone；≠ FR126 Gpio alone；≠ GeneratedFunctional alone.

use bitloom_hir::PortValues;
use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::UartRx;
use bitloom_sim::{AbstractionView, IpDualModelMatrix, UartRxFunctional};
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
fn fr163_docs_contract() {
    let text = read("docs/fr163-unlisted-protocol-handwritten-fl.md");
    assert!(text.contains("FR163") && text.contains("Bitloom"));
    assert!(text.contains("UartRxFunctional") && text.contains("verify_uart_rx_handwritten"));
    assert!(
        text.contains("FR135")
            && (text.contains("alone") || text.contains("≠") || text.contains("不得"))
    );
    assert!(
        text.contains("GeneratedFunctional")
            && (text.contains("alone") || text.contains("≠") || text.contains("不得"))
    );
    assert!(
        (text.contains("Spi")
            || text.contains("SPI")
            || text.contains("I2c")
            || text.contains("Axi"))
            && (text.contains("NFR71") || text.contains("新合同") || text.contains("不做"))
    );
}

#[test]
fn fr163_uart_rx_handwritten_pass() {
    let st = IpDualModelMatrix::new().verify_uart_rx_handwritten(UartRx::elaborate().unwrap());
    assert!(st.is_pass(), "{st:?}");
}

#[test]
fn fr163_deliberate_mismatch_fails() {
    struct Bad;
    impl AbstractionView for Bad {
        fn cycle(&mut self, inputs: &PortValues) -> PortValues {
            let mut o = inputs.clone();
            o.set("rd_data", 0xdead);
            o.set("rd_valid", 1);
            o.set("rx_busy", 1);
            o
        }
    }
    let st = IpDualModelMatrix::new()
        .verify_uart_rx_handwritten_with(UartRx::elaborate().unwrap(), &mut Bad);
    assert!(!st.is_pass());
}

#[test]
fn fr163_exports_uart_rx_functional() {
    let _ = UartRxFunctional::new();
    let dual = read("crates/bitloom-sim/src/ip_dual.rs");
    assert!(
        dual.contains("UartRxFunctional")
            && dual.contains("verify_uart_rx_handwritten")
            && dual.contains("FR163")
    );
    assert!(
        dual.contains("uart_rx_dual_stimulus"),
        "must pin uart_rx_dual_stimulus fixture"
    );
}

#[test]
fn fr163_not_satisfied_by_uart_tx_or_generated_alone() {
    let dual = read("crates/bitloom-sim/src/ip_dual.rs");
    assert!(
        dual.contains("UartRxFunctional") && dual.contains("UartTxFunctional"),
        "FR163 UartRx handwritten must coexist with FR135 UartTx (alone ≠ FR163)"
    );
    assert!(
        dual.contains("verify_uart_rx_handwritten")
            && dual.contains("verify_uart_tx_handwritten")
            && dual.contains("verify_generated_rst_compare"),
        "handwritten UART RX path must be distinct from UartTx handwritten and GeneratedFunctional"
    );
    let nfr = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic95-unlisted-protocol-handwritten-fl-fr163.md",
    );
    assert!(nfr.contains("UartRx") && nfr.contains("FR135"));
}

#[test]
fn fr163_design_crate_prelude_only() {
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
