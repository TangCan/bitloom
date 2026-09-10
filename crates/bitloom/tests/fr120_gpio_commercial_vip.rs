//! ATDD (red→green): Story 61.2 / FR120 — commercial VIP GPIO (C1–C4).
//!
//! Beyond FR108 P1–P4: rising-edge IRQ, open-drain+OE, atomic set/clear;
//! FR98 four-class + FR108 Gpio regression; docs name GpioVip / FR120.
//!
//! ```text
//! cargo test -p bitloom --test fr120_gpio_commercial_vip
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom_hir::PortValues;
use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::{Gpio, GpioVip};
use bitloom_sim::Sim;
use bitloom_vlog::emit;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn vip_drive(
    sim: &mut Sim,
    rst: u64,
    dir: u64,
    wr_en: u64,
    wr_data: u64,
    wr_mask: u64,
    set_en: u64,
    set_data: u64,
    clr_en: u64,
    clr_data: u64,
    od: u64,
    pad_in: u64,
    irq_en: u64,
    irq_clear: u64,
) {
    let mut pv = PortValues::default();
    pv.set("rst", rst);
    pv.set("dir", dir);
    pv.set("wr_en", wr_en);
    pv.set("wr_data", wr_data);
    pv.set("wr_mask", wr_mask);
    pv.set("set_en", set_en);
    pv.set("set_data", set_data);
    pv.set("clr_en", clr_en);
    pv.set("clr_data", clr_data);
    pv.set("od", od);
    pv.set("pad_in", pad_in);
    pv.set("irq_en", irq_en);
    pv.set("irq_clear", irq_clear);
    sim.set_inputs(pv);
    sim.settle();
    sim.tick();
}

#[test]
fn fr120_gpio_vip_elaborate_emit_ports() {
    let hir = GpioVip::elaborate().expect("elaborate GpioVip");
    assert_eq!(hir.abi_name, "GpioVip");
    let v = &emit(&hir).files[0].contents;
    assert!(v.contains("module GpioVip"), "emit module GpioVip");
    for pin in [
        "dir",
        "wr_en",
        "wr_data",
        "wr_mask",
        "set_en",
        "set_data",
        "clr_en",
        "clr_data",
        "od",
        "pad_in",
        "irq_en",
        "irq_clear",
        "pad_out",
        "pad_oe",
        "rd_data",
        "irq_status",
        "irq_out",
    ] {
        assert!(v.contains(pin), "FR120 emit port {pin}");
    }
}

#[test]
fn fr120_c3_atomic_set_clear_and_p3_mask() {
    let mut sim = Sim::new(GpioVip::elaborate().expect("vip"));
    vip_drive(&mut sim, 1, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    // Masked write baseline (P3)
    vip_drive(&mut sim, 0, 0xff, 1, 0xa0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0);
    assert_eq!(sim.ports().get("pad_out"), Some(0xa0));
    // C3 set low nibble
    vip_drive(&mut sim, 0, 0xff, 0, 0, 0, 1, 0x0f, 0, 0, 0, 0, 0, 0);
    assert_eq!(sim.ports().get("pad_out"), Some(0xaf), "C3: set");
    // C3 clear bit 7
    vip_drive(&mut sim, 0, 0xff, 0, 0, 0, 0, 0, 1, 0x80, 0, 0, 0, 0);
    assert_eq!(sim.ports().get("pad_out"), Some(0x2f), "C3: clear");
    // Negative: set_en=0 does not apply set_data
    vip_drive(&mut sim, 0, 0xff, 0, 0, 0, 0, 0xf0, 0, 0, 0, 0, 0, 0);
    assert_eq!(
        sim.ports().get("pad_out"),
        Some(0x2f),
        "C3 negative: set_en=0 keeps out"
    );
}

#[test]
fn fr120_c2_open_drain_oe() {
    let mut sim = Sim::new(GpioVip::elaborate().expect("vip"));
    vip_drive(&mut sim, 1, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    // Push-pull write 0xff
    vip_drive(&mut sim, 0, 0xff, 1, 0xff, 0xff, 0, 0, 0, 0, 0x00, 0, 0, 0);
    assert_eq!(sim.ports().get("pad_oe"), Some(0xff));
    assert_eq!(sim.ports().get("pad_out"), Some(0xff));
    // OD on low nibble while out=0xff → those bits Hi-Z (oe cleared)
    vip_drive(&mut sim, 0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0x0f, 0, 0, 0);
    assert_eq!(
        sim.ports().get("pad_oe"),
        Some(0xf0),
        "C2: od&out clears OE"
    );
    assert_eq!(sim.ports().get("pad_out"), Some(0xf0));
    // Negative: od=0 restores full OE for dir
    vip_drive(&mut sim, 0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0x00, 0, 0, 0);
    assert_eq!(
        sim.ports().get("pad_oe"),
        Some(0xff),
        "C2 negative: od=0 push-pull OE"
    );
}

#[test]
fn fr120_c1_rising_irq_and_negatives() {
    let mut sim = Sim::new(GpioVip::elaborate().expect("vip"));
    vip_drive(&mut sim, 1, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    // Establish pad_prev=0
    vip_drive(&mut sim, 0, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x01, 0);
    // Rising edge bit0 with irq_en
    vip_drive(&mut sim, 0, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0x01, 0);
    assert_eq!(sim.ports().get("irq_status"), Some(0x01), "C1: pending");
    assert_eq!(sim.ports().get("irq_out"), Some(1), "C1: irq_out");
    // Negative: irq_en=0 suppresses irq_out but keeps status
    vip_drive(&mut sim, 0, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0x00, 0);
    assert_eq!(sim.ports().get("irq_status"), Some(0x01));
    assert_eq!(
        sim.ports().get("irq_out"),
        Some(0),
        "C1 negative: irq_en=0 suppresses irq_out"
    );
    // Clear without new edge
    vip_drive(&mut sim, 0, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0x01, 0x01);
    assert_eq!(sim.ports().get("irq_status"), Some(0x00), "C1: clear");
    assert_eq!(sim.ports().get("irq_out"), Some(0));
}

#[test]
fn fr120_docs_name_gpiovip_fr120_honest_non_goals() {
    let root = workspace_root();
    let ip_readme = fs::read_to_string(root.join("docs/ip/README.md")).expect("ip README");
    assert!(
        ip_readme.contains("GpioVip") || ip_readme.contains("GPIO VIP"),
        "docs/ip must name GpioVip / commercial GPIO VIP face"
    );
    assert!(
        ip_readme.contains("FR120") || ip_readme.contains("Epic 61"),
        "docs must mark FR120 / Epic 61"
    );
    assert!(
        ip_readme.contains("Bitloom"),
        "public brand Bitloom in IP docs"
    );
    let lower = ip_readme.to_lowercase();
    assert!(
        lower.contains("nfr51")
            || lower.contains("非目标")
            || lower.contains("deferred")
            || lower.contains("全 soc")
            || lower.contains("debounce"),
        "docs must disclose uncovered / deferred protocols"
    );
    // Must not claim FR108 alone is FR120
    assert!(
        ip_readme.contains("FR108")
            && (ip_readme.contains("超出")
                || ip_readme.contains("beyond")
                || ip_readme.contains("C1")),
        "docs must distinguish FR120 beyond FR108"
    );
}

#[test]
fn fr120_nfr48_fr98_fr108_no_regression() {
    use bitloom_prelude::ip::{Axi4LiteSlave, I2cMaster, SpiMaster, UartRx, UartTx};
    for (name, hir) in [
        ("UartTx", UartTx::elaborate()),
        ("UartRx", UartRx::elaborate()),
        ("SpiMaster", SpiMaster::elaborate()),
        ("I2cMaster", I2cMaster::elaborate()),
        ("Axi4LiteSlave", Axi4LiteSlave::elaborate()),
        ("Gpio", Gpio::elaborate()),
    ] {
        let h = hir.expect(name);
        assert_eq!(h.abi_name, name);
    }
    // FR108 near-VIP still ticks
    let mut sim = Sim::new(Gpio::elaborate().expect("gpio"));
    let mut pv = PortValues::default();
    pv.set("rst", 1);
    pv.set("dir", 0xff);
    pv.set("wr_en", 0);
    pv.set("wr_data", 0);
    pv.set("wr_mask", 0);
    pv.set("pad_in", 0);
    sim.set_inputs(pv.clone());
    sim.settle();
    sim.tick();
    pv.set("rst", 0);
    pv.set("wr_en", 1);
    pv.set("wr_data", 0x55);
    pv.set("wr_mask", 0xff);
    sim.set_inputs(pv);
    sim.settle();
    sim.tick();
    assert_eq!(
        sim.ports().get("pad_out"),
        Some(0x55),
        "FR108 Gpio regression"
    );
}
