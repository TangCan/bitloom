//! ATDD (red→green): Story 38.2 / FR89 — UartTx programmable baud deepen (branch A).
//!
//! Gate: NFR14 Epic 38 selected branch A (programmable baud), not B (minimal RX).
//! Proves elaborate → emit `.v` → tick with `baud_div > 0` bit timing, docs boundaries,
//! and no RX / VIP / full-protocol delivery claims.

use std::fs;
use std::path::PathBuf;

use bitloom_hir::PortValues;
use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::UartTx;
use bitloom_sim::Sim;
use bitloom_vlog::emit;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn uart_drive(sim: &mut Sim, rst: u64, wr_en: u64, wr_data: u64, baud_div: u64) {
    let mut pv = PortValues::default();
    pv.set("rst", rst);
    pv.set("wr_en", wr_en);
    pv.set("wr_data", wr_data);
    pv.set("baud_div", baud_div);
    sim.set_inputs(pv);
    sim.settle();
    sim.tick();
}

#[test]
fn fr89_uarttx_programmable_baud_elaborate_emit_tick() {
    let hir = UartTx::elaborate().expect("elaborate UartTx");
    assert_eq!(hir.abi_name, "UartTx");
    let art = emit(&hir);
    let v = &art.files[0].contents;
    assert!(v.contains("module UartTx"));
    assert!(
        v.contains("baud_div"),
        "FR89 deepen must emit baud_div (programmable baud subset)"
    );
    assert!(
        v.contains("tx_busy") && (v.contains(" tx") || v.contains("\ttx") || v.contains("(tx")),
        "serial tx must remain"
    );

    // baud_div = 1 → 2 clocks per bit
    let baud_div = 1u64;
    let mut sim = Sim::new(hir);
    uart_drive(&mut sim, 1, 0, 0, baud_div);
    assert_eq!(sim.ports().get("tx"), Some(1));
    assert_eq!(sim.ports().get("tx_busy"), Some(0));

    // 0x01 → after start: data LSB=1 for one bit period
    uart_drive(&mut sim, 0, 1, 0x01, baud_div);
    assert_eq!(sim.ports().get("tx_busy"), Some(1));
    assert_eq!(sim.ports().get("tx"), Some(0), "start bit");

    // Still start on second clock of the bit (baud_div=1 → 2 clk/bit)
    uart_drive(&mut sim, 0, 0, 0, baud_div);
    assert_eq!(
        sim.ports().get("tx"),
        Some(0),
        "start bit must hold for full baud period"
    );

    // Next baud tick → data LSB = 1
    uart_drive(&mut sim, 0, 0, 0, baud_div);
    assert_eq!(sim.ports().get("tx"), Some(1), "data LSB after baud period");

    // Hold LSB through second clock of data bit
    uart_drive(&mut sim, 0, 0, 0, baud_div);
    assert_eq!(sim.ports().get("tx"), Some(1), "data bit hold");
}

#[test]
fn fr89_uarttx_baud_div_zero_matches_fr82_one_clk_per_bit() {
    let mut sim = Sim::new(UartTx::elaborate().expect("elaborate"));
    uart_drive(&mut sim, 1, 0, 0, 0);
    uart_drive(&mut sim, 0, 1, 0xA5, 0);
    assert_eq!(sim.ports().get("tx"), Some(0)); // start
    let expected = [0u64, 1, 0, 1, 0, 0, 1, 0, 1, 1]; // start + 8 data + stop
    for (i, &bit) in expected.iter().enumerate().skip(1) {
        uart_drive(&mut sim, 0, 0, 0, 0);
        assert_eq!(
            sim.ports().get("tx"),
            Some(bit),
            "FR82-compat frame bit {i}"
        );
    }
    uart_drive(&mut sim, 0, 0, 0, 0);
    assert_eq!(sim.ports().get("tx_busy"), Some(0));
}

#[test]
fn fr89_docs_ip_readme_documents_baud_subset_and_non_goals() {
    let readme = fs::read_to_string(workspace_root().join("docs/ip/README.md")).expect("README");
    let uartish = readme.to_lowercase();
    assert!(
        uartish.contains("baud")
            && (readme.contains("可编程")
                || uartish.contains("programmable")
                || readme.contains("分频")
                || uartish.contains("baud_div")),
        "docs/ip must document programmable baud deepen (FR89)"
    );
    assert!(
        readme.contains("FR89") || readme.contains("Epic 38") || readme.contains("38"),
        "docs should tie deepen to FR89 / Epic 38"
    );
    // Explicit non-goals for Epic 38 UartTx deepen (historical): VIP / full protocol
    // family beyond baud subset. RX / full-duplex overturned later by FR98 / Epic 43.2.
    let has_vip = readme.contains("VIP") || readme.contains("全协议");
    assert!(
        has_vip
            && (readme.contains("非目标")
                || readme.contains("明确非")
                || uartish.contains("non-goal")),
        "docs must keep VIP|全协议 honesty / non-goals language"
    );
    assert!(
        readme.contains("FR89") || readme.contains("Epic 38") || readme.contains("38"),
        "docs should retain FR89 / Epic 38 baud deepen mention"
    );
    // Epic 38 must not claim "最小 RX 已交付" under FR89 wording
    assert!(
        !readme.contains("最小 RX 已交付"),
        "must not claim Epic 38 branch B (minimal RX) under FR89 wording"
    );
}

#[test]
fn fr89_uarttx_api_has_no_generator_closures_and_baud_div() {
    let src = fs::read_to_string(workspace_root().join("crates/bitloom-prelude/src/ip.rs"))
        .expect("ip.rs");
    let uart = src
        .split("impl Elaboratable for UartTx")
        .nth(1)
        .and_then(|s| s.split("impl Elaboratable for UartRx").next())
        .unwrap_or("");
    assert!(
        !uart.contains("Fn(") && !uart.contains("dyn Fn"),
        "UartTx elaborate must not accept generator closures"
    );
    assert!(
        uart.contains("baud_div") || uart.contains("baud_cnt"),
        "UartTx deepen must include baud divider / counter nets"
    );
    // FR98 may add UartRx under Epic 43; Epic 38 branch B remains historically undelivered
    // (see nfr14-risk-epic38 + fr89_epic38_boundary_closeout).
}
