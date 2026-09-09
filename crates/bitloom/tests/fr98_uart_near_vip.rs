//! ATDD (red→green): Story 43.2 / FR98 — UART near-VIP (U1–U5).
//!
//! Beyond FR89 UartTx-only programmable baud: TX + RX reachable, shared baud_div
//! semantics, 8N1, elaborate→emit→tick on both paths, docs boundaries.
//! Does **not** close FR98 four-class VIP (SPI/I2C/AXI remain Epic 43 later stories).

use std::fs;
use std::path::PathBuf;

use bitloom_hir::PortValues;
use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::{UartRx, UartTx};
use bitloom_sim::Sim;
use bitloom_vlog::emit;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn tx_drive(sim: &mut Sim, rst: u64, wr_en: u64, wr_data: u64, baud_div: u64) {
    let mut pv = PortValues::default();
    pv.set("rst", rst);
    pv.set("wr_en", wr_en);
    pv.set("wr_data", wr_data);
    pv.set("baud_div", baud_div);
    sim.set_inputs(pv);
    sim.settle();
    sim.tick();
}

fn rx_drive(sim: &mut Sim, rst: u64, rx: u64, baud_div: u64) {
    let mut pv = PortValues::default();
    pv.set("rst", rst);
    pv.set("rx", rx);
    pv.set("baud_div", baud_div);
    sim.set_inputs(pv);
    sim.settle();
    sim.tick();
}

#[test]
fn fr98_uart_tx_rx_elaborate_emit() {
    let tx = UartTx::elaborate().expect("elaborate UartTx");
    let rx = UartRx::elaborate().expect("elaborate UartRx");
    assert_eq!(tx.abi_name, "UartTx");
    assert_eq!(rx.abi_name, "UartRx");

    let tv = &emit(&tx).files[0].contents;
    let rv = &emit(&rx).files[0].contents;
    assert!(tv.contains("module UartTx") && tv.contains("baud_div"));
    assert!(
        rv.contains("module UartRx") && rv.contains("baud_div") && rv.contains("rd_data"),
        "UartRx must emit baud_div and rd_data (U1/U2)"
    );
    assert!(
        rv.contains("rd_valid")
            && (rv.contains(" rx") || rv.contains("\trx") || rv.contains("(rx")),
        "UartRx must expose serial rx + rd_valid"
    );
}

#[test]
fn fr98_uart_loopback_8n1() {
    let baud_div = 0u64;
    let payload = 0xA5u64;
    let mut tx = Sim::new(UartTx::elaborate().expect("tx"));
    let mut rx = Sim::new(UartRx::elaborate().expect("rx"));

    tx_drive(&mut tx, 1, 0, 0, baud_div);
    rx_drive(&mut rx, 1, 1, baud_div);
    tx_drive(&mut tx, 0, 0, 0, baud_div);
    rx_drive(&mut rx, 0, 1, baud_div); // idle high → rx_prev=1

    tx_drive(&mut tx, 0, 1, payload, baud_div);
    let line = tx.ports().get("tx").expect("tx");
    rx_drive(&mut rx, 0, line, baud_div);

    for _ in 0..16 {
        if rx.ports().get("rd_valid") == Some(1) {
            break;
        }
        tx_drive(&mut tx, 0, 0, 0, baud_div);
        let line = tx.ports().get("tx").expect("tx");
        rx_drive(&mut rx, 0, line, baud_div);
    }

    assert_eq!(
        rx.ports().get("rd_valid"),
        Some(1),
        "U4: RX path must complete a frame (rd_valid)"
    );
    assert_eq!(
        rx.ports().get("rd_data"),
        Some(payload),
        "U3: 8N1 loopback must recover payload"
    );
}

#[test]
fn fr98_uart_loopback_baud_div() {
    let baud_div = 1u64; // 2 clk/bit
    let payload = 0x3Cu64;
    let mut tx = Sim::new(UartTx::elaborate().expect("tx"));
    let mut rx = Sim::new(UartRx::elaborate().expect("rx"));

    tx_drive(&mut tx, 1, 0, 0, baud_div);
    rx_drive(&mut rx, 1, 1, baud_div);
    tx_drive(&mut tx, 0, 0, 0, baud_div);
    rx_drive(&mut rx, 0, 1, baud_div);

    tx_drive(&mut tx, 0, 1, payload, baud_div);
    let line = tx.ports().get("tx").expect("tx");
    rx_drive(&mut rx, 0, line, baud_div);

    for _ in 0..40 {
        if rx.ports().get("rd_valid") == Some(1) {
            break;
        }
        tx_drive(&mut tx, 0, 0, 0, baud_div);
        let line = tx.ports().get("tx").expect("tx");
        rx_drive(&mut rx, 0, line, baud_div);
    }

    assert_eq!(rx.ports().get("rd_valid"), Some(1), "U2/U4 baud_div>0 RX");
    assert_eq!(
        rx.ports().get("rd_data"),
        Some(payload),
        "U2 baud-timed loopback"
    );
}

#[test]
fn fr98_docs_ip_uart_near_vip_boundaries() {
    let readme = fs::read_to_string(workspace_root().join("docs/ip/README.md")).expect("README");
    let lower = readme.to_lowercase();

    assert!(
        readme.contains("Bitloom") || readme.contains("bitloom"),
        "public brand Bitloom"
    );
    assert!(
        readme.contains("UartRx")
            && (readme.contains("FR98") || readme.contains("Epic 43") || readme.contains("43")),
        "docs must name UartRx and FR98 / Epic 43 UART near-VIP"
    );
    assert!(
        readme.contains("8N1") || lower.contains("8n1"),
        "docs must state 8N1 frame surface (U3/U5)"
    );
    assert!(
        lower.contains("baud") || readme.contains("baud_div") || readme.contains("分频"),
        "docs must mention programmable baud (U2)"
    );
    // Full-duplex contract or TX+RX
    assert!(
        readme.contains("全双工")
            || lower.contains("full-duplex")
            || lower.contains("full duplex")
            || (readme.contains("TX") && readme.contains("RX")),
        "docs must describe TX+RX / full-duplex contract (U1)"
    );
    // Explicit non-goals remain honest
    assert!(
        readme.contains("非目标")
            || lower.contains("non-goal")
            || lower.contains("non-goals")
            || readme.contains("明确非"),
        "docs must keep explicit non-goals (U5)"
    );
    let nongoalish = readme.contains("流控")
        || lower.contains("flow control")
        || readme.contains("IrDA")
        || lower.contains("irda")
        || readme.contains("小数")
        || lower.contains("fractional")
        || readme.contains("parity")
        || readme.contains("校验");
    assert!(
        nongoalish,
        "docs should list concrete UART non-goals (flow/IrDA/fractional/parity…)"
    );
    // Must not claim FR98 four-class complete from UART alone
    assert!(
        !(readme.contains("FR98 全绿") || lower.contains("fr98 complete")),
        "must not claim FR98 full green from UART-only story"
    );
}
