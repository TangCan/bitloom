//! ATDD — Story 45.4 / FR103: first-class IP dual-model completeness.
//!
//! ```text
//! cargo test -p bitloom --test fr103_ip_dual_model
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom_hir::PortValues;
use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::{Axi4LiteSlave, I2cMaster, SpiMaster, SyncFifo, UartRx, UartTx};
use bitloom_sim::{AbstractionView, EquivStatus, IpDualModelMatrix};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    let path = workspace_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn assert_readable_fail(status: &EquivStatus) {
    match status {
        EquivStatus::Fail { cycle, mismatches } => {
            assert!(
                !mismatches.is_empty(),
                "Fail at cycle {cycle} must carry readable PortMismatch list"
            );
            let rendered = format!("{mismatches:?}");
            assert!(
                !rendered.is_empty(),
                "Fail diagnostics must be human-readable, got: {rendered}"
            );
        }
        EquivStatus::Pass { .. } => panic!("expected Fail with readable diagnostics"),
    }
}

#[test]
fn fr103_docs_completion_surface_and_five_ip_classes() {
    let text = read("docs/fr103-ip-dual-model.md");
    assert!(text.contains("FR103"), "must name FR103");
    assert!(text.contains("Bitloom"), "must brand Bitloom");
    assert!(
        text.contains("完成面") || text.to_lowercase().contains("completion"),
        "must declare completion surface"
    );
    for needle in ["SyncFifo", "UART", "SPI", "I2C", "AXI"] {
        assert!(
            text.contains(needle),
            "FR103 doc must cover IP class: {needle}"
        );
    }
    assert!(
        text.contains("GeneratedFunctional")
            || text.contains("生成")
            || text.contains("generation"),
        "must document generation / GeneratedFunctional path"
    );
    assert!(
        text.contains("SyncFifoFunctional") || text.contains("手写"),
        "must document SyncFifo handwritten FL (Mem)"
    );
}

#[test]
fn fr103_docs_adapters_supporting_not_sufficient() {
    let text = read("docs/fr103-ip-dual-model.md");
    assert!(
        (text.contains("FR78") || text.contains("FR92") || text.contains("adapter"))
            && (text.contains("alone ≠")
                || text.contains("≠ FR103")
                || text.contains("非充分")
                || text.contains("alone")),
        "FR103 doc must state adapter templates alone ≠ FR103"
    );
}

#[test]
fn fr103_docs_tlm_product_still_epic46() {
    let text = read("docs/fr103-ip-dual-model.md");
    assert!(
        text.contains("Epic 46") || text.contains("FR101"),
        "must keep SystemC TLM product on Epic 46 / FR101"
    );
}

#[test]
fn fr103_sync_fifo_dual_model_pass() {
    let matrix = IpDualModelMatrix::new();
    let status = matrix.verify_sync_fifo(SyncFifo::elaborate().expect("SyncFifo"));
    assert!(
        status.is_pass(),
        "SyncFifo handwritten FL must match settle+tick: {status:?}"
    );
}

#[test]
fn fr103_sync_fifo_deliberate_mismatch_fails_readable() {
    struct WrongFifo;
    impl AbstractionView for WrongFifo {
        fn cycle(&mut self, inputs: &PortValues) -> PortValues {
            let mut out = inputs.clone();
            out.set("full", 1);
            out.set("empty", 0);
            out.set("data_out", 0xDEAD);
            out
        }
    }
    let matrix = IpDualModelMatrix::new();
    let status =
        matrix.verify_sync_fifo_with(SyncFifo::elaborate().expect("SyncFifo"), &mut WrongFifo);
    assert_readable_fail(&status);
}

#[test]
fn fr103_uart_spi_i2c_axi_generated_fl_rst_compare() {
    let matrix = IpDualModelMatrix::new();
    for (name, hir) in [
        ("UartTx", UartTx::elaborate().expect("UartTx")),
        ("UartRx", UartRx::elaborate().expect("UartRx")),
        ("SpiMaster", SpiMaster::elaborate().expect("SpiMaster")),
        ("I2cMaster", I2cMaster::elaborate().expect("I2cMaster")),
        ("Axi4LiteSlave", Axi4LiteSlave::elaborate().expect("Axi")),
    ] {
        let status = matrix.verify_generated_rst_compare(hir);
        assert!(
            status.is_pass(),
            "{name} GeneratedFunctional ≡ tick (rst alphabet) must Pass: {status:?}"
        );
    }
}

#[test]
fn fr103_product_api_exported() {
    let sim = read("crates/bitloom-sim/src/lib.rs");
    assert!(
        sim.contains("IpDualModelMatrix") || sim.contains("ip_dual"),
        "bitloom-sim must export IpDualModelMatrix / ip_dual"
    );
    let dual = read("crates/bitloom-sim/src/ip_dual.rs");
    assert!(
        dual.contains("SyncFifoFunctional") && dual.contains("FR103"),
        "ip_dual must document FR103 + SyncFifoFunctional"
    );
}
