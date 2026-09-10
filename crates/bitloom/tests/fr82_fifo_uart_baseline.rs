//! ATDD: Story 34.2 / FR82 — SyncFifo + UartTx non-stub elaborate→emit→tick;
//! no generator-closure API; black-box boundary documented.

use std::fs;
use std::path::PathBuf;

use bitloom_hir::PortValues;
use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::{ExtBlackBox, SyncFifo, UartTx, vendor_blackbox_v};
use bitloom_sim::Sim;
use bitloom_vlog::emit;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn fr82_sync_fifo_elaborate_emit_tick_non_stub() {
    let hir = SyncFifo::elaborate().expect("elaborate");
    assert_eq!(hir.abi_name, "SyncFifo");
    let art = emit(&hir);
    let v = &art.files[0].contents;
    assert!(v.contains("module SyncFifo"));
    assert!(
        v.contains("full") && v.contains("empty") && v.contains("wr_en") && v.contains("rd_en"),
        "FR82 SyncFifo must emit handshake ports (not depth-1 skid stub)"
    );

    let mut sim = Sim::new(hir);
    let mut pv = PortValues::default();
    pv.set("rst", 1);
    pv.set("wr_en", 0);
    pv.set("rd_en", 0);
    pv.set("data_in", 0);
    sim.set_inputs(pv.clone());
    sim.settle();
    sim.tick();
    pv.set("rst", 0);
    pv.set("wr_en", 1);
    pv.set("data_in", 0x42);
    sim.set_inputs(pv.clone());
    sim.settle();
    sim.tick();
    assert_eq!(sim.ports().get("empty"), Some(0));
    pv.set("wr_en", 0);
    pv.set("rd_en", 1);
    sim.set_inputs(pv);
    sim.settle();
    sim.tick();
    assert_eq!(sim.ports().get("data_out"), Some(0x42));
}

#[test]
fn fr82_uart_tx_elaborate_emit_tick_non_stub() {
    let hir = UartTx::elaborate().expect("elaborate");
    assert_eq!(hir.abi_name, "UartTx");
    let art = emit(&hir);
    let v = &art.files[0].contents;
    assert!(v.contains("module UartTx"));
    assert!(
        v.contains("tx_busy") && (v.contains(" tx") || v.contains("\ttx") || v.contains("(tx")),
        "FR82 UartTx must emit serial tx (not hold-only stub)"
    );

    let mut sim = Sim::new(hir);
    let mut pv = PortValues::default();
    pv.set("rst", 1);
    pv.set("wr_en", 0);
    pv.set("wr_data", 0);
    sim.set_inputs(pv.clone());
    sim.settle();
    sim.tick();
    pv.set("rst", 0);
    pv.set("wr_en", 1);
    pv.set("wr_data", 0x01); // LSB=1 after start
    sim.set_inputs(pv);
    sim.settle();
    sim.tick();
    assert_eq!(sim.ports().get("tx_busy"), Some(1));
    assert_eq!(sim.ports().get("tx"), Some(0)); // start
    // next bit should be data LSB = 1
    let mut pv2 = PortValues::default();
    pv2.set("rst", 0);
    pv2.set("wr_en", 0);
    pv2.set("wr_data", 0);
    sim.set_inputs(pv2);
    sim.settle();
    sim.tick();
    assert_eq!(sim.ports().get("tx"), Some(1));
}

#[test]
fn fr82_ip_api_has_no_generator_closures() {
    // Elaboratable::elaborate() takes no Fn — Epic 29 owns closure customization.
    let _ = SyncFifo::elaborate();
    let _ = UartTx::elaborate();
    let root = workspace_root().join("crates/bitloom-prelude/src/ip");
    let fifo = fs::read_to_string(root.join("sync_fifo.rs")).expect("ip/sync_fifo.rs");
    let uart = fs::read_to_string(root.join("uart.rs")).expect("ip/uart.rs");
    assert!(
        !fifo.contains("Fn(")
            && !fifo.contains("dyn Fn")
            && !uart.contains("Fn(")
            && !uart.contains("dyn Fn"),
        "SyncFifo/UartTx elaborate must not accept generator closures"
    );
}

#[test]
fn fr82_blackbox_boundary_documented() {
    let hir = ExtBlackBox::elaborate().unwrap();
    assert!(hir.circuit().modules[0].body.is_empty());
    assert!(vendor_blackbox_v().contains("vendor_ext_ip"));

    let ip = fs::read_to_string(workspace_root().join("docs/ip/README.md")).expect("ip readme");
    assert!(
        ip.contains("黑盒")
            && (ip.contains("不内联") || ip.contains("opaque") || ip.contains("空 body")),
        "docs/ip must document black-box boundary"
    );
    let surface = fs::read_to_string(
        workspace_root().join("_agile-output/specs/spec-rhdl/language-surface.md"),
    )
    .expect("language-surface");
    assert!(
        surface.contains("ExtBlackBox") && surface.contains("FR82"),
        "language-surface must mention FR82 IP / ExtBlackBox"
    );
}
