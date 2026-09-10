//! ATDD (red→green): Story 50.2 / FR108 — GPIO near-VIP (P1–P4).
//!
//! Direction, masked write, pad R/W via elaborate→emit→tick; docs boundaries;
//! NFR47 non-goals; NFR44 FR98 UART/SPI/I2C/AXI remain green.
//!
//! ```text
//! cargo test -p bitloom --test fr108_gpio_near_vip
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom_hir::PortValues;
use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::Gpio;
use bitloom_sim::Sim;
use bitloom_vlog::emit;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn gpio_drive(
    sim: &mut Sim,
    rst: u64,
    dir: u64,
    wr_en: u64,
    wr_data: u64,
    wr_mask: u64,
    pad_in: u64,
) {
    let mut pv = PortValues::default();
    pv.set("rst", rst);
    pv.set("dir", dir);
    pv.set("wr_en", wr_en);
    pv.set("wr_data", wr_data);
    pv.set("wr_mask", wr_mask);
    pv.set("pad_in", pad_in);
    sim.set_inputs(pv);
    sim.settle();
    sim.tick();
}

#[test]
fn fr108_gpio_elaborate_emit_ports() {
    let hir = Gpio::elaborate().expect("elaborate Gpio");
    assert_eq!(hir.abi_name, "Gpio");
    let v = &emit(&hir).files[0].contents;
    assert!(v.contains("module Gpio"), "emit module Gpio");
    for pin in [
        "dir", "wr_en", "wr_data", "wr_mask", "pad_in", "pad_out", "rd_data",
    ] {
        assert!(v.contains(pin), "P1/P2 emit port {pin}");
    }
}

#[test]
fn fr108_gpio_direction_write_mask_tick() {
    let mut sim = Sim::new(Gpio::elaborate().expect("gpio"));
    gpio_drive(&mut sim, 1, 0xff, 0, 0, 0, 0);
    gpio_drive(&mut sim, 0, 0xff, 1, 0xa5, 0xff, 0);
    assert_eq!(
        sim.ports().get("pad_out"),
        Some(0xa5),
        "P2: write + pad_out"
    );
    assert_eq!(
        sim.ports().get("rd_data"),
        Some(0xa5),
        "P2: rd_data mirrors out"
    );

    // P3: masked write low nibble
    gpio_drive(&mut sim, 0, 0xff, 1, 0x0b, 0x0f, 0);
    assert_eq!(sim.ports().get("pad_out"), Some(0xab), "P3: wr_mask");

    // P1: mixed direction — low out, high in from pad
    gpio_drive(&mut sim, 0, 0x0f, 0, 0, 0, 0x50);
    assert_eq!(
        sim.ports().get("pad_out"),
        Some(0x0b),
        "P1: pad_out masked by dir"
    );
    assert_eq!(
        sim.ports().get("rd_data"),
        Some(0x5b),
        "P1/P2: rd_data = (out&dir)|(pad_in&~dir)"
    );
}

#[test]
fn fr108_gpio_negative_mask_zero_keeps_out() {
    // P4: boundary — wr_en with mask 0 must not change out_r
    let mut sim = Sim::new(Gpio::elaborate().expect("gpio"));
    gpio_drive(&mut sim, 1, 0xff, 0, 0, 0, 0);
    gpio_drive(&mut sim, 0, 0xff, 1, 0x55, 0xff, 0);
    assert_eq!(sim.ports().get("pad_out"), Some(0x55));
    gpio_drive(&mut sim, 0, 0xff, 1, 0xaa, 0x00, 0);
    assert_eq!(
        sim.ports().get("pad_out"),
        Some(0x55),
        "P4: mask=0 keeps previous out"
    );
}

#[test]
fn fr108_docs_boundaries_honest() {
    let root = workspace_root();
    let ip_readme = fs::read_to_string(root.join("docs/ip/README.md")).expect("ip README");
    assert!(
        ip_readme.contains("Gpio") || ip_readme.contains("GPIO"),
        "docs/ip must name Gpio / GPIO (FR108)"
    );
    assert!(
        ip_readme.contains("Bitloom"),
        "public brand Bitloom in IP docs"
    );
    let lower = ip_readme.to_lowercase();
    assert!(
        !lower.contains("commercial gpio vip")
            && !lower.contains("gpio vip complete")
            && !lower.contains("full soc pad"),
        "NFR47: ban commercial VIP GPIO / full SoC pad slogans"
    );
    // Delivered protocol face vs non-goals
    assert!(
        ip_readme.contains("FR108")
            || ip_readme.contains("近 VIP")
            || ip_readme.contains("near-VIP")
            || ip_readme.contains("near VIP"),
        "docs must mark GPIO near-VIP / FR108 delivery"
    );
    assert!(
        lower.contains("非目标")
            || lower.contains("nfr47")
            || lower.contains("commercial vip")
            || lower.contains("中断"),
        "docs must state non-goals / NFR47 boundary"
    );
}

#[test]
fn fr108_nfr44_fr98_ip_still_reachable() {
    // Smoke: FR98 four-class types still elaborate (regression gate)
    use bitloom_prelude::ip::{Axi4LiteSlave, I2cMaster, SpiMaster, UartRx, UartTx};
    for (name, hir) in [
        ("UartTx", UartTx::elaborate()),
        ("UartRx", UartRx::elaborate()),
        ("SpiMaster", SpiMaster::elaborate()),
        ("I2cMaster", I2cMaster::elaborate()),
        ("Axi4LiteSlave", Axi4LiteSlave::elaborate()),
    ] {
        let h = hir.expect(name);
        assert_eq!(h.abi_name, name);
    }
}
