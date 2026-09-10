//! ATDD Story 66.2 / FR126 — more IP handwritten FL (`GpioFunctional`).

use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::Gpio;
use bitloom_sim::{
    AbstractionView, GpioFunctional, IpDualModelMatrix, PortValues, gpio_dual_stimulus,
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
fn fr126_docs_contract() {
    let text = read("docs/fr126-more-ip-handwritten-fl.md");
    assert!(text.contains("FR126") && text.contains("Bitloom"));
    assert!(text.contains("GpioFunctional") && text.contains("verify_gpio_handwritten"));
    assert!(
        text.contains("FR103")
            && (text.contains("alone") || text.contains("≠") || text.contains("不得"))
    );
    assert!(text.contains("FR112") || text.contains("FR119"));
}

#[test]
fn fr126_gpio_handwritten_pass() {
    let st = IpDualModelMatrix::new().verify_gpio_handwritten(Gpio::elaborate().unwrap());
    assert!(st.is_pass(), "{st:?}");
}

#[test]
fn fr126_deliberate_mismatch_fails() {
    struct Bad;
    impl AbstractionView for Bad {
        fn cycle(&mut self, inputs: &PortValues) -> PortValues {
            let mut o = inputs.clone();
            o.set("pad_out", 0xdead);
            o.set("rd_data", 0xbeef);
            o
        }
    }
    let st = IpDualModelMatrix::new().verify_handwritten(
        Gpio::elaborate().unwrap(),
        &mut Bad,
        gpio_dual_stimulus(),
    );
    assert!(!st.is_pass());
}

#[test]
fn fr126_exports_gpio_functional() {
    let _ = GpioFunctional::new();
    let dual = read("crates/bitloom-sim/src/ip_dual.rs");
    assert!(dual.contains("GpioFunctional") && dual.contains("FR126"));
}
