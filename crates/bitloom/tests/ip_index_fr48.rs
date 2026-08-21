//! ATDD: FR48 IP index docs list all five categories + black-box via bitloom-prelude
//! (Story 22.6). Red if index missing or incomplete.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn ip_index_documents_five_categories_via_prelude() {
    let path = workspace_root().join("docs/ip/README.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(
        text.contains("Bitloom") && text.contains("samitbasu/rhdl"),
        "index must brand Bitloom and disclaim samitbasu/rhdl"
    );
    assert!(
        text.contains("bitloom-prelude") || text.contains("bitloom_prelude"),
        "index must route instantiation via bitloom-prelude"
    );

    for needle in [
        "SyncFifo",
        "UartTx",
        "SpiMaster",
        "I2cMaster",
        "Axi4LiteSlave",
        "ExtBlackBox",
    ] {
        assert!(text.contains(needle), "index must name IP type {needle}");
    }

    for (cat, smoke) in [
        ("FIFO", "sync_fifo"),
        ("UART", "uart_tx"),
        ("SPI", "spi_master"),
        ("I2C", "i2c_master"),
        ("AXI", "axi4_lite"),
    ] {
        assert!(
            text.contains(cat) && text.contains(smoke),
            "index must list category {cat} with smoke `{smoke}`"
        );
    }

    assert!(
        text.contains("AXI4-Lite") && (text.contains("最小从") || text.contains("min")),
        "index must state AXI = AXI4-Lite min slave"
    );
    assert!(
        text.contains("just test") || text.contains("cargo test --workspace"),
        "index must note CI / just test reachability"
    );
    assert!(
        text.contains("已知限制") || text.contains("Non-goals") || text.contains("限制"),
        "index must document known limits per class"
    );
}
