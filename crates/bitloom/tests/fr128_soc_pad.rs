//! ATDD Story 68.2 / FR128 — full SoC pad (`GpioSocPad` D1–D4).

use std::fs;
use std::path::PathBuf;

use bitloom_hir::PortValues;
use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::{Gpio, GpioSocPad, GpioVip};
use bitloom_sim::Sim;
use bitloom_vlog::emit;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn drive(
    sim: &mut Sim,
    rst: u64,
    dir: u64,
    wr_en: u64,
    wr_data: u64,
    wr_mask: u64,
    pad_in: u64,
    irq_en: u64,
    irq_clear: u64,
    irq_fall_en: u64,
    irq_fall_clear: u64,
    csr_wen: u64,
    csr_addr: u64,
    csr_wdata: u64,
) {
    let mut pv = PortValues::default();
    pv.set("rst", rst);
    pv.set("dir", dir);
    pv.set("wr_en", wr_en);
    pv.set("wr_data", wr_data);
    pv.set("wr_mask", wr_mask);
    pv.set("pad_in", pad_in);
    pv.set("irq_en", irq_en);
    pv.set("irq_clear", irq_clear);
    pv.set("irq_fall_en", irq_fall_en);
    pv.set("irq_fall_clear", irq_fall_clear);
    pv.set("csr_wen", csr_wen);
    pv.set("csr_addr", csr_addr);
    pv.set("csr_wdata", csr_wdata);
    sim.set_inputs(pv);
    sim.settle();
    sim.tick();
}

#[test]
fn fr128_docs_contract() {
    let docs = fs::read_to_string(root().join("docs/fr128-soc-pad.md")).unwrap();
    assert!(docs.contains("FR128") && docs.contains("Bitloom"));
    assert!(docs.contains("D1") && docs.contains("D4") && docs.contains("GpioSocPad"));
    assert!(docs.contains("FR120") && (docs.contains("beyond") || docs.contains("Beyond")));
    let nfr = fs::read_to_string(
        root().join("_agile-output/implementation-artifacts/nfr14-risk-epic68-soc-pad.md"),
    )
    .unwrap();
    assert!(nfr.contains("D1") && nfr.contains("GpioSocPad"));
}

#[test]
fn fr128_elaborate_emit_ports() {
    let hir = GpioSocPad::elaborate().expect("elaborate");
    assert_eq!(hir.abi_name, "GpioSocPad");
    let v = &emit(&hir).files[0].contents;
    assert!(v.contains("module GpioSocPad"));
    for pin in [
        "dir",
        "pad_in",
        "pad_out",
        "rd_data",
        "irq_en",
        "irq_fall_en",
        "irq_fall_status",
        "irq_out",
        "csr_wen",
        "csr_addr",
        "csr_wdata",
        "csr_rdata",
    ] {
        assert!(v.contains(pin), "missing port {pin}");
    }
}

#[test]
fn fr128_d1_dual_bank_and_d3_csr() {
    let mut sim = Sim::new(GpioSocPad::elaborate().unwrap());
    drive(&mut sim, 1, 0xffff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    // Bank0 + bank1 write via wr_*
    drive(
        &mut sim, 0, 0xffff, 1, 0x12ab, 0xffff, 0, 0, 0, 0, 0, 0, 0, 0,
    );
    assert_eq!(
        sim.ports().get("pad_out"),
        Some(0x12ab),
        "D1 dual-bank pad_out"
    );
    // D3 CSR write overlays out
    drive(&mut sim, 0, 0xffff, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0x34cd);
    assert_eq!(sim.ports().get("pad_out"), Some(0x34cd), "D3 csr write");
    assert_eq!(
        sim.ports().get("csr_rdata"),
        Some(0x34cd),
        "D3 csr read data"
    );
}

#[test]
fn fr128_d2_falling_edge_irq() {
    let mut sim = Sim::new(GpioSocPad::elaborate().unwrap());
    drive(&mut sim, 1, 0, 0, 0, 0, 0xffff, 0, 0, 0xffff, 0, 0, 0, 0);
    // Sample high
    drive(&mut sim, 0, 0, 0, 0, 0, 0xffff, 0, 0, 0xffff, 0, 0, 0, 0);
    // Falling edge on bit0
    drive(&mut sim, 0, 0, 0, 0, 0, 0xfffe, 0, 0, 0xffff, 0, 0, 0, 0);
    assert_eq!(
        sim.ports().get("irq_fall_status").unwrap() & 1,
        1,
        "D2 fall pending"
    );
    assert_eq!(sim.ports().get("irq_out"), Some(1), "D2 irq_out from fall");
}

#[test]
fn fr128_d4_scoreboard_expected_vs_observed() {
    let mut sim = Sim::new(GpioSocPad::elaborate().unwrap());
    let expected = [0u64, 0x00ff, 0xff00, 0xa5a5];
    let mut observed = Vec::new();
    drive(&mut sim, 1, 0xffff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    observed.push(sim.ports().get("pad_out").unwrap_or(0));
    for &w in &expected[1..] {
        drive(&mut sim, 0, 0xffff, 1, w, 0xffff, 0, 0, 0, 0, 0, 0, 0, 0);
        observed.push(sim.ports().get("pad_out").unwrap_or(0));
    }
    assert_eq!(observed, expected, "D4 scoreboard pad_out sequence");
}

#[test]
fn fr128_regression_gpio_and_vip_still_elaborate() {
    assert!(Gpio::elaborate().is_ok());
    assert!(GpioVip::elaborate().is_ok());
}
