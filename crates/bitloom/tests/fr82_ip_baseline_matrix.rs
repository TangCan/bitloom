//! ATDD matrix: Story 34.4 / FR82 — five-class IP baseline consolidation +
//! Epic 29 handoff docs (no-closure vs closure overlay; NFR37 vs stub history).
//!
//! Recipe (also covered by `just test` / `cargo test --workspace`):
//! ```text
//! cargo test -p bitloom --test fr82_ip_baseline_matrix
//! cargo test -p bitloom --test fr82_fifo_uart_baseline
//! cargo test -p bitloom --test fr82_spi_i2c_axi_baseline
//! ```
//!
//! Deep fixtures (rows consolidated here; see those files for fuller goldens):
//! - `fr82_fifo_uart_baseline` — SyncFifo + UartTx elaborate→emit→tick + black-box
//! - `fr82_spi_i2c_axi_baseline` — SpiMaster + I2cMaster + Axi4LiteSlave

use std::fs;
use std::path::PathBuf;

use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::{Axi4LiteSlave, I2cMaster, SpiMaster, SyncFifo, UartTx};
use bitloom_vlog::emit;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn matrix_sibling_fixtures_present() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
    for name in ["fr82_fifo_uart_baseline.rs", "fr82_spi_i2c_axi_baseline.rs"] {
        assert!(
            root.join(name).is_file(),
            "matrix consolidates sibling fixture {name}"
        );
    }
}

fn assert_elaborate_emit(name: &str, hir: bitloom_hir::FrozenHir) {
    assert_eq!(hir.abi_name, name);
    let art = emit(&hir);
    let v = &art.files[0].contents;
    assert!(
        v.contains(&format!("module {name}")),
        "FR82 {name} must emit a real module (not stub-only)"
    );
}

#[test]
fn matrix_five_ip_classes_elaborate_emit() {
    assert_elaborate_emit("SyncFifo", SyncFifo::elaborate().expect("SyncFifo"));
    assert_elaborate_emit("UartTx", UartTx::elaborate().expect("UartTx"));
    assert_elaborate_emit("SpiMaster", SpiMaster::elaborate().expect("SpiMaster"));
    assert_elaborate_emit("I2cMaster", I2cMaster::elaborate().expect("I2cMaster"));
    assert_elaborate_emit(
        "Axi4LiteSlave",
        Axi4LiteSlave::elaborate().expect("Axi4LiteSlave"),
    );
}

#[test]
fn matrix_all_five_ip_apis_have_no_generator_closures() {
    let _ = SyncFifo::elaborate();
    let _ = UartTx::elaborate();
    let _ = SpiMaster::elaborate();
    let _ = I2cMaster::elaborate();
    let _ = Axi4LiteSlave::elaborate();

    let root = workspace_root().join("crates/bitloom-prelude/src/ip");
    let files = [
        ("SyncFifo", "sync_fifo.rs"),
        ("UartTx", "uart.rs"),
        ("UartRx", "uart.rs"),
        ("SpiMaster", "spi.rs"),
        ("I2cMaster", "i2c.rs"),
        ("Axi4LiteSlave", "axi.rs"),
    ];
    for (name, file) in files {
        let src = fs::read_to_string(root.join(file)).unwrap_or_else(|e| panic!("{file}: {e}"));
        let needle = format!("impl Elaboratable for {name}");
        let body = src
            .split(&needle)
            .nth(1)
            .map(|s| s.split("impl Elaboratable for").next().unwrap_or(s))
            .unwrap_or("");
        assert!(
            !body.is_empty() && !body.contains("Fn(") && !body.contains("dyn Fn"),
            "{name} elaborate must not accept generator closures (Epic 34; overlay = Epic 29)"
        );
    }
}

#[test]
fn matrix_docs_epic29_handoff_and_nfr37_vs_stub() {
    let ip = fs::read_to_string(workspace_root().join("docs/ip/README.md")).expect("ip readme");
    assert!(
        ip.contains("Epic 29")
            && (ip.contains("闭包定制") || ip.contains("closure"))
            && (ip.contains("叠加") || ip.contains("overlay") || ip.contains("handoff")),
        "docs/ip must state Epic 29 is the closure customization overlay / handoff"
    );
    assert!(
        (ip.contains("无闭包") || ip.contains("无生成器闭包"))
            && (ip.contains("Epic 34") || ip.contains("FR82")),
        "docs/ip must state Epic 34 / FR82 has no generator closures"
    );
    assert!(
        (ip.contains("stub") || ip.contains("Stub") || ip.contains("端口语义"))
            && (ip.contains("NFR37") || ip.contains("Epic 22")),
        "docs/ip must distinguish FR82 from Epic 22 stub history (NFR37)"
    );
    assert!(
        ip.contains("SyncFifo")
            && ip.contains("UartTx")
            && ip.contains("SpiMaster")
            && ip.contains("I2cMaster")
            && ip.contains("Axi4LiteSlave"),
        "docs/ip must index all five FR82 classes"
    );

    let nfr = fs::read_to_string(
        workspace_root()
            .join("_agile-output/implementation-artifacts/nfr14-risk-epic34-ip-baseline.md"),
    )
    .expect("nfr14 epic34");
    assert!(
        nfr.contains("Epic 34 关闭条件")
            && nfr.contains("[x]")
            && nfr.contains("FR82")
            && nfr.contains("Epic 29"),
        "NFR14 Epic 34 record must check off close conditions (FR82 + Epic 29 handoff)"
    );
}
