//! ATDD (red→green): Story 43.4 / FR98 — I2C near-VIP (I1–I4).
//!
//! Beyond FR82 START+8data+STOP SCL-high toy: ACK/NACK-driven master write,
//! documented read path, START / 7-bit addr / data / STOP, half-period SCL,
//! elaborate→emit→tick, docs. Does **not** close FR98 four-class VIP (AXI/GPIO → 43.5).

use std::fs;
use std::path::PathBuf;

use bitloom_hir::PortValues;
use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::I2cMaster;
use bitloom_sim::Sim;
use bitloom_vlog::emit;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn i2c_drive(sim: &mut Sim, rst: u64, start: u64, addr: u64, rw: u64, tx_data: u64, sda_in: u64) {
    let mut pv = PortValues::default();
    pv.set("rst", rst);
    pv.set("start", start);
    pv.set("addr", addr);
    pv.set("rw", rw);
    pv.set("tx_data", tx_data);
    pv.set("sda_in", sda_in);
    sim.set_inputs(pv);
    sim.settle();
    sim.tick();
}

#[test]
fn fr98_i2c_elaborate_emit_ports() {
    let hir = I2cMaster::elaborate().expect("elaborate I2cMaster");
    assert_eq!(hir.abi_name, "I2cMaster");
    let v = &emit(&hir).files[0].contents;
    assert!(v.contains("module I2cMaster"));
    for pin in [
        "addr",
        "rw",
        "scl",
        "sda_out",
        "rx_data",
        "rx_valid",
        "ack_error",
    ] {
        assert!(
            v.contains(pin),
            "FR98 I2cMaster must emit near-VIP port `{pin}` (I1–I3)"
        );
    }
}

#[test]
fn fr98_i2c_idle_scl_high() {
    let mut sim = Sim::new(I2cMaster::elaborate().expect("i2c"));
    i2c_drive(&mut sim, 1, 0, 0, 0, 0, 1);
    i2c_drive(&mut sim, 0, 0, 0, 0, 0, 1);
    assert_eq!(sim.ports().get("busy"), Some(0));
    assert_eq!(
        sim.ports().get("scl"),
        Some(1),
        "I2: idle SCL must be high (released)"
    );
    assert_eq!(sim.ports().get("sda_out"), Some(1));
    assert_eq!(sim.ports().get("ack_error"), Some(0));
}

#[test]
fn fr98_i2c_write_ack() {
    let mut sim = Sim::new(I2cMaster::elaborate().expect("i2c"));
    i2c_drive(&mut sim, 1, 0, 0, 0, 0, 1);
    i2c_drive(&mut sim, 0, 0, 0, 0, 0, 1);

    let addr = 0x50u64;
    let data = 0xA5u64;
    let addr_byte = (addr << 1) | 0; // write

    i2c_drive(&mut sim, 0, 1, addr, 0, data, 0);
    assert_eq!(sim.ports().get("busy"), Some(1));
    assert_eq!(sim.ports().get("scl"), Some(1), "START: SCL high");
    assert_eq!(sim.ports().get("sda_out"), Some(0), "START: SDA low");
    assert_eq!(sim.ports().get("tx_byte"), Some(addr_byte));

    let mut saw_scl_low = false;
    let mut saw_scl_high_busy = false;
    // Addr 8 bits × 2 halves + ACK × 2 + data 8×2 + ACK × 2 + STOP room
    for _ in 0..64 {
        i2c_drive(&mut sim, 0, 0, addr, 0, data, 0); // ACK=0
        if sim.ports().get("busy") == Some(1) {
            match sim.ports().get("scl") {
                Some(0) => saw_scl_low = true,
                Some(1) => saw_scl_high_busy = true,
                _ => {}
            }
        }
        if sim.ports().get("busy") == Some(0) {
            break;
        }
    }

    assert!(
        saw_scl_low && saw_scl_high_busy,
        "I2: SCL must toggle (not stuck high toy); low={saw_scl_low} high={saw_scl_high_busy}"
    );
    assert_eq!(sim.ports().get("busy"), Some(0), "write txn must complete");
    assert_eq!(
        sim.ports().get("ack_error"),
        Some(0),
        "I1: ACK path must clear/keep ack_error=0"
    );
    assert_eq!(sim.ports().get("scl"), Some(1));
    assert_eq!(sim.ports().get("sda_out"), Some(1));
}

#[test]
fn fr98_i2c_addr_nack() {
    let mut sim = Sim::new(I2cMaster::elaborate().expect("i2c"));
    i2c_drive(&mut sim, 1, 0, 0, 0, 0, 1);
    i2c_drive(&mut sim, 0, 0, 0, 0, 0, 1);

    i2c_drive(&mut sim, 0, 1, 0x12, 0, 0xFF, 1); // will NACK
    assert_eq!(sim.ports().get("busy"), Some(1));

    for _ in 0..40 {
        i2c_drive(&mut sim, 0, 0, 0x12, 0, 0xFF, 1); // sda_in=1 → NACK
        if sim.ports().get("busy") == Some(0) {
            break;
        }
    }

    assert_eq!(sim.ports().get("busy"), Some(0));
    assert_eq!(
        sim.ports().get("ack_error"),
        Some(1),
        "I1: NACK on addr ACK slot must set ack_error"
    );
}

#[test]
fn fr98_i2c_read_byte() {
    let mut sim = Sim::new(I2cMaster::elaborate().expect("i2c"));
    i2c_drive(&mut sim, 1, 0, 0, 0, 0, 1);
    i2c_drive(&mut sim, 0, 0, 0, 0, 0, 1);

    let addr = 0x2Au64;
    // Slave returns 0x3C = 0011_1100 MSB-first
    let rx_bits = [0u64, 0, 1, 1, 1, 1, 0, 0];

    i2c_drive(&mut sim, 0, 1, addr, 1, 0, 0); // START; rw=1
    assert_eq!(sim.ports().get("busy"), Some(1));
    assert_eq!(sim.ports().get("sda_out"), Some(0));

    // Leave START (1) + 16 addr halves + 2 addr-ACK (drive ACK=0)
    for _ in 0..19 {
        i2c_drive(&mut sim, 0, 0, addr, 1, 0, 0);
    }

    // 8 data bits × 2 halves — sample on half1; hold bit stable both halves
    for &bit in &rx_bits {
        i2c_drive(&mut sim, 0, 0, addr, 1, 0, bit); // half0
        i2c_drive(&mut sim, 0, 0, addr, 1, 0, bit); // half1 sample
    }

    // DACK half0
    i2c_drive(&mut sim, 0, 0, addr, 1, 0, 0);
    // DACK half1 → rx_valid pulse
    i2c_drive(&mut sim, 0, 0, addr, 1, 0, 0);
    assert_eq!(
        sim.ports().get("rx_valid"),
        Some(1),
        "I1: read path must pulse rx_valid"
    );
    assert_eq!(
        sim.ports().get("rx_data"),
        Some(0x3C),
        "I1: read path must assemble rx_data MSB-first"
    );

    // STOP → idle
    for _ in 0..4 {
        i2c_drive(&mut sim, 0, 0, addr, 1, 0, 0);
        if sim.ports().get("busy") == Some(0) {
            break;
        }
    }
    assert_eq!(sim.ports().get("busy"), Some(0));
    assert_eq!(sim.ports().get("ack_error"), Some(0));
}

#[test]
fn fr98_docs_ip_i2c_near_vip_boundaries() {
    let root = workspace_root();
    let readme = fs::read_to_string(root.join("docs/ip/README.md")).expect("docs/ip");
    let lower = readme.to_lowercase();
    assert!(
        readme.contains("I2cMaster")
            && (readme.contains("FR98 / Epic 43.4")
                || (readme.contains("I2cMaster")
                    && readme.contains("ACK")
                    && (readme.contains("近 VIP") || readme.contains("near-VIP")))),
        "I4: docs/ip must document FR98 I2C near-VIP"
    );
    assert!(
        readme.contains("ACK") || readme.contains("NACK") || lower.contains("ack"),
        "I4: docs must mention ACK/NACK delivery"
    );
    assert!(
        lower.contains("非目标")
            || lower.contains("non-goal")
            || lower.contains("stretch")
            || lower.contains("slave")
            || lower.contains("10-bit")
            || lower.contains("多主"),
        "I4: docs must state non-goals (stretch/multi-master/10-bit/slave/…)"
    );
    assert!(
        !(readme.contains("FR98 全绿") || lower.contains("fr98 complete")),
        "must not claim four-class FR98 green from I2C alone"
    );
    assert!(lower.contains("bitloom"), "public brand Bitloom required");

    let nfr =
        fs::read_to_string(root.join(
            "_agile-output/implementation-artifacts/nfr14-risk-epic43-vip-full-protocol-ip.md",
        ))
        .expect("nfr14");
    assert!(
        nfr.contains("- [ ] **AXI：**")
            || (nfr.contains("AXI") && nfr.contains("- [ ]") && nfr.contains("43.5")),
        "must not close full Epic 43 NFR14 checklist in Story 43.4"
    );
}
