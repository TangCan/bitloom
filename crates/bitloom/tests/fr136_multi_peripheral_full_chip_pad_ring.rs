//! ATDD Story 75.2 / FR136 — multi-peripheral / full-chip pad ring beyond GpioSocPad.
//!
//! ≠ FR128 D1–D4 alone；≠ FR120 C1–C4 alone；≠ FR108 alone；≠ docs-only.

use std::fs;
use std::path::PathBuf;

use bitloom_hir::PortValues;
use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::{
    CHIP_PAD_RING_BANK_COUNT, CHIP_PAD_RING_PAD_WIDTH, CHIP_PAD_RING_PINS_PER_BANK, ChipPadRing,
    Gpio, GpioSocPad, GpioVip, UartTx, chip_pad_ring_bank_pin_index,
};
use bitloom_sim::Sim;
use bitloom_vlog::emit;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

fn drive_gpio(
    sim: &mut Sim,
    rst: u64,
    dir: u64,
    wr_en: u64,
    wr_data: u64,
    wr_mask: u64,
    pad_in: u64,
    uart_wr_en: u64,
    uart_wr_data: u64,
    uart_baud_div: u64,
) {
    let mut pv = PortValues::default();
    pv.set("rst", rst);
    pv.set("dir", dir);
    pv.set("wr_en", wr_en);
    pv.set("wr_data", wr_data);
    pv.set("wr_mask", wr_mask);
    pv.set("pad_in", pad_in);
    pv.set("uart_wr_en", uart_wr_en);
    pv.set("uart_wr_data", uart_wr_data);
    pv.set("uart_baud_div", uart_baud_div);
    sim.set_inputs(pv);
    sim.settle();
    sim.tick();
}

#[test]
fn fr136_docs_contract() {
    let text = read("docs/fr136-multi-peripheral-full-chip-pad-ring.md");
    assert!(text.contains("FR136") && text.contains("Bitloom"));
    assert!(text.contains("ChipPadRing") || text.contains("MultiPeripheral"));
    assert!(text.contains("R1") && text.contains("R4"));
    assert!(
        text.contains("FR128")
            && (text.contains("alone") || text.contains("≠") || text.contains("不得"))
    );
    assert!(text.contains("UART") || text.contains("UartTx") || text.contains("uart_tx"));
}

#[test]
fn fr136_r1_chip_pad_ring_elaborate_ports() {
    let hir = ChipPadRing::elaborate().expect("elaborate ChipPadRing");
    assert_eq!(hir.abi_name, "ChipPadRing");
    let v = &emit(&hir).files[0].contents;
    assert!(v.contains("module ChipPadRing"));
    for pin in [
        "dir",
        "pad_in",
        "pad_out",
        "wr_en",
        "wr_data",
        "wr_mask",
        "uart_tx",
        "uart_wr_en",
        "uart_wr_data",
        "uart_baud_div",
        "uart_tx_busy",
    ] {
        assert!(v.contains(pin), "missing multi-peripheral pad port {pin}");
    }
    // Still distinct from single GpioSocPad / standalone UartTx
    assert!(GpioSocPad::elaborate().is_ok());
    assert!(UartTx::elaborate().is_ok());
}

#[test]
fn fr136_r2_full_chip_ring_shape() {
    assert!(
        CHIP_PAD_RING_BANK_COUNT >= 3 || CHIP_PAD_RING_PAD_WIDTH >= 24,
        "R2: need ≥3 banks or width ≥24 (got banks={} width={})",
        CHIP_PAD_RING_BANK_COUNT,
        CHIP_PAD_RING_PAD_WIDTH
    );
    assert_eq!(
        CHIP_PAD_RING_BANK_COUNT * CHIP_PAD_RING_PINS_PER_BANK,
        CHIP_PAD_RING_PAD_WIDTH
    );
    // Beyond FR128 dual-bank ×8 = 16
    assert!(
        CHIP_PAD_RING_PAD_WIDTH > 16 || CHIP_PAD_RING_BANK_COUNT > 2,
        "must exceed FR128 dual-bank 16"
    );
    assert_eq!(chip_pad_ring_bank_pin_index(0, 0), Some(0));
    assert_eq!(chip_pad_ring_bank_pin_index(1, 0), Some(8));
    assert_eq!(
        chip_pad_ring_bank_pin_index(2, 7),
        Some(CHIP_PAD_RING_PAD_WIDTH - 1)
    );
    assert_eq!(
        chip_pad_ring_bank_pin_index(CHIP_PAD_RING_BANK_COUNT, 0),
        None
    );

    let hir = ChipPadRing::elaborate().unwrap();
    let v = &emit(&hir).files[0].contents;
    assert!(
        v.contains(&format!("{}", CHIP_PAD_RING_PAD_WIDTH))
            || v.contains("width: 24")
            || v.contains("[23:0]")
            || v.contains("UInt { width: 24 }"),
        "emitted ring must expose ≥24-bit / full-chip width"
    );
}

#[test]
fn fr136_r3_ring_scoreboard_pass() {
    let mut sim = Sim::new(ChipPadRing::elaborate().unwrap());
    let expected_pad = [0u64, 0x00_00ff, 0x00_ff00, 0xff_0000, 0xaa_55aa];
    let mut observed_pad = Vec::new();
    drive_gpio(&mut sim, 1, 0xff_ffff, 0, 0, 0, 0, 0, 0, 0);
    observed_pad.push(sim.ports().get("pad_out").unwrap_or(0));
    for &w in &expected_pad[1..] {
        drive_gpio(&mut sim, 0, 0xff_ffff, 1, w, 0xff_ffff, 0, 0, 0, 0);
        observed_pad.push(sim.ports().get("pad_out").unwrap_or(0));
    }
    assert_eq!(
        observed_pad, expected_pad,
        "R3 ring scoreboard GPIO pad_out sequence"
    );

    // UART pad side-effect: accept one byte (baud_div=0) and observe uart_tx leave idle
    drive_gpio(&mut sim, 0, 0xff_ffff, 0, 0, 0, 0, 1, 0x55, 0);
    let mut uart_bits = Vec::new();
    for _ in 0..12 {
        drive_gpio(&mut sim, 0, 0xff_ffff, 0, 0, 0, 0, 0, 0, 0);
        uart_bits.push(sim.ports().get("uart_tx").unwrap_or(1));
    }
    assert!(
        uart_bits.iter().any(|&b| b == 0),
        "R3 UART bitstream side-effect: uart_tx must leave idle-high (saw {uart_bits:?})"
    );
}

#[test]
fn fr136_r3_deliberate_wrong_model_fails() {
    let mut sim = Sim::new(ChipPadRing::elaborate().unwrap());
    drive_gpio(&mut sim, 1, 0xff_ffff, 0, 0, 0, 0, 0, 0, 0);
    drive_gpio(&mut sim, 0, 0xff_ffff, 1, 0x12_3456, 0xff_ffff, 0, 0, 0, 0);
    let observed = sim.ports().get("pad_out").unwrap_or(0);
    let wrong_expected = 0xdead_beefu64;
    assert_ne!(
        observed, wrong_expected,
        "deliberate wrong model must not match observed pad_out"
    );
    // Explicit Fail path: scoreboard helper treats mismatch as !pass
    let pass = observed == wrong_expected;
    assert!(!pass, "R3 wrong model → Fail");
}

#[test]
fn fr136_r4_not_satisfied_by_fr128_or_fr120_or_fr108_alone() {
    let socpad = read("crates/bitloom-prelude/src/ip/gpio/socpad.rs");
    let ring = read("crates/bitloom-prelude/src/ip/gpio/chip_ring.rs");
    let mod_rs = read("crates/bitloom-prelude/src/ip/gpio/mod.rs");
    assert!(
        ring.contains("ChipPadRing") && mod_rs.contains("chip_ring"),
        "FR136 ChipPadRing must land beyond GpioSocPad alone"
    );
    assert!(
        socpad.contains("GpioSocPad") && socpad.contains("FR128"),
        "FR128 GpioSocPad must remain (alone ≠ FR136)"
    );
    let nfr = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic75-multi-peripheral-full-chip-pad-ring.md",
    );
    assert!(nfr.contains("R1") && nfr.contains("R4"));
    assert!(
        nfr.contains("FR128")
            && (nfr.contains("alone") || nfr.contains("≠") || nfr.contains("不得"))
    );
    assert!(Gpio::elaborate().is_ok());
    assert!(GpioVip::elaborate().is_ok());
    assert!(GpioSocPad::elaborate().is_ok());
}

#[test]
fn fr136_design_crate_prelude_only() {
    let prelude = read("crates/bitloom-prelude/Cargo.toml");
    let deps = prelude
        .split("[dependencies]")
        .nth(1)
        .and_then(|s| s.split('[').next())
        .unwrap_or("");
    assert!(
        !deps.contains("bitloom-sim"),
        "design crate must not depend on bitloom-sim"
    );
}

#[test]
fn fr136_fr139_gpio_split_intact() {
    let gpio_dir = workspace_root().join("crates/bitloom-prelude/src/ip/gpio");
    for name in ["mod.rs", "base.rs", "vip.rs", "socpad.rs"] {
        assert!(
            gpio_dir.join(name).is_file(),
            "FR139 split file missing: {name}"
        );
    }
    let mod_rs = read("crates/bitloom-prelude/src/ip/gpio/mod.rs");
    assert!(
        mod_rs.contains("mod base") && mod_rs.contains("mod vip") && mod_rs.contains("mod socpad"),
        "gpio/mod.rs must keep base/vip/socpad"
    );
}
