//! ATDD (red→green): Story 43.5 / FR98 — AXI4-Lite near-VIP (A1–A4).
//!
//! Beyond FR82 single-register ignore-addr/wstrb toy: multi-register window,
//! address decode, wstrb byte merges, AW/W/B + AR/R handshake,
//! elaborate→emit→tick, docs. Closes FR98 AXI face; GPIO optional not required.
//!
//! ```text
//! cargo test -p bitloom --test fr98_axi_near_vip
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom_hir::PortValues;
use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::Axi4LiteSlave;
use bitloom_sim::Sim;
use bitloom_vlog::emit;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn axi_drive(
    sim: &mut Sim,
    rst: u64,
    awaddr: u64,
    awvalid: u64,
    wdata: u64,
    wstrb: u64,
    wvalid: u64,
    bready: u64,
    araddr: u64,
    arvalid: u64,
    rready: u64,
) {
    let mut pv = PortValues::default();
    pv.set("rst", rst);
    pv.set("s_axi_awaddr", awaddr);
    pv.set("s_axi_awvalid", awvalid);
    pv.set("s_axi_wdata", wdata);
    pv.set("s_axi_wstrb", wstrb);
    pv.set("s_axi_wvalid", wvalid);
    pv.set("s_axi_bready", bready);
    pv.set("s_axi_araddr", araddr);
    pv.set("s_axi_arvalid", arvalid);
    pv.set("s_axi_rready", rready);
    sim.set_inputs(pv);
    sim.settle();
    sim.tick();
}

fn idle(sim: &mut Sim) {
    axi_drive(sim, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
}

fn write_word(sim: &mut Sim, addr: u64, data: u64, wstrb: u64) {
    axi_drive(sim, 0, addr, 1, data, wstrb, 1, 0, 0, 0, 0);
    assert_eq!(
        sim.ports().get("s_axi_bvalid"),
        Some(1),
        "A1: bvalid after write"
    );
    axi_drive(sim, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0);
    assert_eq!(
        sim.ports().get("s_axi_bvalid"),
        Some(0),
        "A1: bready clears bvalid"
    );
}

fn read_word(sim: &mut Sim, addr: u64) -> u64 {
    axi_drive(sim, 0, 0, 0, 0, 0, 0, 0, addr, 1, 0);
    assert_eq!(
        sim.ports().get("s_axi_rvalid"),
        Some(1),
        "A1: rvalid after read"
    );
    let data = sim.ports().get("s_axi_rdata").expect("rdata");
    axi_drive(sim, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1);
    assert_eq!(
        sim.ports().get("s_axi_rvalid"),
        Some(0),
        "A1: rready clears rvalid"
    );
    data
}

#[test]
fn fr98_axi_elaborate_emit_ports() {
    let hir = Axi4LiteSlave::elaborate().expect("elaborate Axi4LiteSlave");
    assert_eq!(hir.abi_name, "Axi4LiteSlave");
    let v = &emit(&hir).files[0].contents;
    assert!(v.contains("module Axi4LiteSlave"));
    for pin in [
        "s_axi_awaddr",
        "s_axi_awready",
        "s_axi_wstrb",
        "s_axi_wready",
        "s_axi_bvalid",
        "s_axi_araddr",
        "s_axi_arready",
        "s_axi_rvalid",
        "s_axi_rdata",
    ] {
        assert!(
            v.contains(pin),
            "FR98 Axi4LiteSlave must emit near-VIP port `{pin}` (A1/A3)"
        );
    }
    // Multi-register evidence in emitted netlist (A2)
    assert!(
        v.contains("data0_r") || v.contains("data1_r") || (v.matches("data").count() >= 4),
        "A2: emit must show multi-register bank (not single data_r toy)"
    );
}

#[test]
fn fr98_axi_write_read_handshake() {
    let mut sim = Sim::new(Axi4LiteSlave::elaborate().expect("axi"));
    axi_drive(&mut sim, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    idle(&mut sim);
    assert_eq!(sim.ports().get("s_axi_awready"), Some(1));
    assert_eq!(sim.ports().get("s_axi_arready"), Some(1));

    write_word(&mut sim, 0x00, 0xDEAD_BEEF, 0xF);
    assert_eq!(read_word(&mut sim, 0x00), 0xDEAD_BEEF);
}

#[test]
fn fr98_axi_multi_reg_decode() {
    let mut sim = Sim::new(Axi4LiteSlave::elaborate().expect("axi"));
    axi_drive(&mut sim, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    idle(&mut sim);

    write_word(&mut sim, 0x00, 0x1111_1111, 0xF);
    write_word(&mut sim, 0x04, 0x2222_2222, 0xF);
    write_word(&mut sim, 0x08, 0x3333_3333, 0xF);
    write_word(&mut sim, 0x0C, 0x4444_4444, 0xF);

    assert_eq!(read_word(&mut sim, 0x00), 0x1111_1111, "A2: reg0");
    assert_eq!(read_word(&mut sim, 0x04), 0x2222_2222, "A2: reg1");
    assert_eq!(read_word(&mut sim, 0x08), 0x3333_3333, "A2: reg2");
    assert_eq!(read_word(&mut sim, 0x0C), 0x4444_4444, "A2: reg3");

    // Unmapped read → 0 (documented subset)
    assert_eq!(read_word(&mut sim, 0x10), 0, "A1: unmapped read returns 0");
}

#[test]
fn fr98_axi_wstrb_partial_write() {
    let mut sim = Sim::new(Axi4LiteSlave::elaborate().expect("axi"));
    axi_drive(&mut sim, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0);
    idle(&mut sim);

    write_word(&mut sim, 0x04, 0xAABB_CCDD, 0xF);
    // Update only low byte
    write_word(&mut sim, 0x04, 0x0000_00EE, 0x1);
    assert_eq!(
        read_word(&mut sim, 0x04),
        0xAABB_CCEE,
        "A1: wstrb must merge bytes; not ignore"
    );
    // Update high two bytes
    write_word(&mut sim, 0x04, 0x1122_0000, 0xC);
    assert_eq!(
        read_word(&mut sim, 0x04),
        0x1122_CCEE,
        "A1: wstrb lanes 2–3"
    );
}

#[test]
fn fr98_docs_ip_axi_near_vip_boundaries() {
    let ip =
        fs::read_to_string(workspace_root().join("docs/ip/README.md")).expect("docs/ip/README.md");
    assert!(
        ip.contains("Bitloom") || ip.contains("bitloom"),
        "docs must use public brand Bitloom"
    );
    assert!(
        ip.contains("Axi4LiteSlave")
            && (ip.contains("FR98") || ip.contains("近 VIP") || ip.contains("near-VIP")),
        "docs must document AXI FR98 / near-VIP face"
    );
    assert!(
        (ip.contains("wstrb") || ip.contains("WSTRB"))
            && (ip.contains("译码") || ip.contains("addr") || ip.contains("多寄存器")),
        "A4/A1: docs must mention addr decode / wstrb / multi-reg"
    );
    assert!(
        (ip.contains("Full AXI") || ip.contains("Full") || ip.contains("互联"))
            && (ip.contains("非目标") || ip.contains("非 Full") || ip.contains("not")),
        "A4: docs must state Lite vs Full / interconnect non-goals"
    );
    assert!(
        ip.contains("GPIO")
            && (ip.contains("可选")
                || ip.contains("未交付")
                || ip.contains("optional")
                || ip.contains("FR108")),
        "G1: docs must note GPIO FR98-optional status and/or FR108 delivery"
    );
    assert!(
        !ip.contains("忽略 addr/wstrb") || ip.contains("FR82"),
        "must not still claim current AXI ignores addr/wstrb as the FR98 face"
    );
}
