//! ATDD Story 74.2 / FR135 — more IP handwritten FL beyond Gpio (`UartTxFunctional`).
//!
//! ≠ GeneratedFunctional alone；≠ Gpio / FR126 alone；≠ SyncFifo / FR103 alone.

use bitloom_hir::PortValues;
use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::UartTx;
use bitloom_sim::{AbstractionView, IpDualModelMatrix, UartTxFunctional};
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
fn fr135_docs_contract() {
    let text = read("docs/fr135-more-ip-handwritten-fl.md");
    assert!(text.contains("FR135") && text.contains("Bitloom"));
    assert!(text.contains("UartTxFunctional") && text.contains("verify_uart_tx_handwritten"));
    assert!(
        text.contains("FR126")
            && (text.contains("alone") || text.contains("≠") || text.contains("不得"))
    );
    assert!(
        text.contains("GeneratedFunctional")
            && (text.contains("alone") || text.contains("≠") || text.contains("不得"))
    );
    assert!(text.contains("FR103") || text.contains("SyncFifo"));
}

#[test]
fn fr135_uart_tx_handwritten_pass() {
    let st = IpDualModelMatrix::new().verify_uart_tx_handwritten(UartTx::elaborate().unwrap());
    assert!(st.is_pass(), "{st:?}");
}

#[test]
fn fr135_deliberate_mismatch_fails() {
    struct Bad;
    impl AbstractionView for Bad {
        fn cycle(&mut self, inputs: &PortValues) -> PortValues {
            let mut o = inputs.clone();
            o.set("tx", 0);
            o.set("tx_byte", 0xdead);
            o.set("tx_busy", 1);
            o
        }
    }
    let st = IpDualModelMatrix::new()
        .verify_uart_tx_handwritten_with(UartTx::elaborate().unwrap(), &mut Bad);
    assert!(!st.is_pass());
}

#[test]
fn fr135_exports_uart_tx_functional() {
    let _ = UartTxFunctional::new();
    let dual = read("crates/bitloom-sim/src/ip_dual.rs");
    assert!(
        dual.contains("UartTxFunctional")
            && dual.contains("verify_uart_tx_handwritten")
            && dual.contains("FR135")
    );
    assert!(
        dual.contains("uart_tx_dual_stimulus"),
        "must pin uart_tx_dual_stimulus fixture"
    );
}

#[test]
fn fr135_not_satisfied_by_gpio_or_generated_alone() {
    let dual = read("crates/bitloom-sim/src/ip_dual.rs");
    assert!(
        dual.contains("UartTxFunctional") && dual.contains("GpioFunctional"),
        "FR135 UartTx handwritten must coexist with FR126 Gpio (alone ≠ FR135)"
    );
    assert!(
        dual.contains("verify_uart_tx_handwritten")
            && dual.contains("verify_gpio_handwritten")
            && dual.contains("verify_generated_rst_compare"),
        "handwritten UART path must be distinct from Gpio handwritten and GeneratedFunctional"
    );
    let nfr =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic74-more-ip-handwritten-fl.md");
    assert!(nfr.contains("UartTx") && nfr.contains("GeneratedFunctional"));
}

#[test]
fn fr135_design_crate_prelude_only() {
    let prelude = read("crates/bitloom-prelude/Cargo.toml");
    let deps = prelude
        .split("[dependencies]")
        .nth(1)
        .and_then(|s| s.split('[').next())
        .expect("[dependencies]");
    assert!(
        !deps.contains("bitloom-sim"),
        "design crate [dependencies] must not include bitloom-sim (handwritten FL stays in toolchain; sim may be [dev-dependencies] only)"
    );
    assert!(
        prelude.contains("Bitloom") || prelude.contains("bitloom"),
        "public brand Bitloom"
    );
}
