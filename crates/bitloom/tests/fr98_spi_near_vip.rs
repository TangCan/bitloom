//! ATDD (red→green): Story 43.3 / FR98 — SPI near-VIP (S1–S4).
//!
//! Beyond FR82 Mode-0-ish single-byte toy: configurable CPOL/CPHA (4 modes),
//! master multi-byte with cs_n frame boundary, elaborate→emit→tick, docs.
//! Does **not** close FR98 four-class VIP (I2C/AXI remain Epic 43 later stories).

use std::fs;
use std::path::PathBuf;

use bitloom_hir::PortValues;
use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::SpiMaster;
use bitloom_sim::Sim;
use bitloom_vlog::emit;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn spi_drive(
    sim: &mut Sim,
    rst: u64,
    start: u64,
    tx_data: u64,
    miso: u64,
    cpol: u64,
    cpha: u64,
    byte_count: u64,
) {
    let mut pv = PortValues::default();
    pv.set("rst", rst);
    pv.set("start", start);
    pv.set("tx_data", tx_data);
    pv.set("miso", miso);
    pv.set("cpol", cpol);
    pv.set("cpha", cpha);
    pv.set("byte_count", byte_count);
    sim.set_inputs(pv);
    sim.settle();
    sim.tick();
}

#[test]
fn fr98_spi_elaborate_emit_ports() {
    let hir = SpiMaster::elaborate().expect("elaborate SpiMaster");
    assert_eq!(hir.abi_name, "SpiMaster");
    let v = &emit(&hir).files[0].contents;
    assert!(v.contains("module SpiMaster"));
    for pin in [
        "cpol",
        "cpha",
        "byte_count",
        "cs_n",
        "sclk",
        "mosi",
        "rx_data",
        "rx_valid",
    ] {
        assert!(
            v.contains(pin),
            "FR98 SpiMaster must emit near-VIP port `{pin}` (S1/S3)"
        );
    }
}

#[test]
fn fr98_spi_cpol_cpha_idle_and_emit() {
    for (cpol, cpha) in [(0u64, 0u64), (0, 1), (1, 0), (1, 1)] {
        let mut sim = Sim::new(SpiMaster::elaborate().expect("spi"));
        spi_drive(&mut sim, 1, 0, 0, 0, cpol, cpha, 1);
        spi_drive(&mut sim, 0, 0, 0, 0, cpol, cpha, 1);
        assert_eq!(
            sim.ports().get("busy"),
            Some(0),
            "idle cpol={cpol} cpha={cpha}"
        );
        assert_eq!(sim.ports().get("cs_n"), Some(1));
        assert_eq!(
            sim.ports().get("sclk"),
            Some(cpol),
            "S1 idle sclk must equal cpol (mode {cpol}/{cpha})"
        );
    }
}

#[test]
fn fr98_spi_mode0_byte_transfer_rx() {
    let mut sim = Sim::new(SpiMaster::elaborate().expect("spi"));
    spi_drive(&mut sim, 1, 0, 0, 0, 0, 0, 1);
    spi_drive(&mut sim, 0, 0, 0, 0, 0, 0, 1);

    let tx = 0xA5u64;
    let rx_bits = [0u64, 0, 1, 1, 1, 1, 0, 0]; // → 0x3C

    spi_drive(&mut sim, 0, 1, tx, rx_bits[0], 0, 0, 1);
    assert_eq!(sim.ports().get("busy"), Some(1));
    assert_eq!(sim.ports().get("cs_n"), Some(0));
    assert_eq!(
        sim.ports().get("sclk"),
        Some(0),
        "Mode-0 setup half: sclk idle-low"
    );
    assert_eq!(sim.ports().get("mosi"), Some(1), "MSB of 0xA5");
    assert_eq!(sim.ports().get("mosi_byte"), Some(tx));

    for (i, &bit) in rx_bits.iter().enumerate() {
        // leading — sample this MISO bit
        spi_drive(&mut sim, 0, 0, 0, bit, 0, 0, 1);
        assert_eq!(sim.ports().get("sclk"), Some(1), "leading bit {i}");
        // trailing — shift / finish
        spi_drive(&mut sim, 0, 0, 0, bit, 0, 0, 1);
    }

    assert_eq!(
        sim.ports().get("rx_valid"),
        Some(1),
        "S3: RX path must pulse rx_valid"
    );
    assert_eq!(
        sim.ports().get("rx_data"),
        Some(0x3C),
        "S1 Mode-0 must assemble MISO MSB-first into rx_data"
    );
    assert_eq!(sim.ports().get("busy"), Some(0));
    assert_eq!(
        sim.ports().get("cs_n"),
        Some(1),
        "S2: CS released after frame"
    );
}

#[test]
fn fr98_spi_multibyte_cs_frame() {
    let mut sim = Sim::new(SpiMaster::elaborate().expect("spi"));
    spi_drive(&mut sim, 1, 0, 0, 0, 0, 0, 2);
    spi_drive(&mut sim, 0, 0, 0, 0, 0, 0, 2);

    let b0 = 0x12u64;
    let b1 = 0x34u64;
    spi_drive(&mut sim, 0, 1, b0, 0, 0, 0, 2);
    assert_eq!(sim.ports().get("cs_n"), Some(0));
    assert_eq!(sim.ports().get("mosi_byte"), Some(b0));

    let mut bytes_done = 0u32;
    let mut saw_busy_cs_low = false;
    for _ in 0..80 {
        // Offer b1 on tx_data so the byte-boundary reload can latch it.
        spi_drive(&mut sim, 0, 0, b1, 0, 0, 0, 2);
        if sim.ports().get("busy") == Some(1) {
            assert_eq!(
                sim.ports().get("cs_n"),
                Some(0),
                "S2: cs_n must stay low for entire multi-byte frame"
            );
            saw_busy_cs_low = true;
        }
        if sim.ports().get("rx_valid") == Some(1) {
            bytes_done += 1;
            if bytes_done == 1 {
                assert_eq!(sim.ports().get("busy"), Some(1));
                assert_eq!(sim.ports().get("cs_n"), Some(0));
            }
        }
        if sim.ports().get("busy") == Some(0) && bytes_done >= 2 {
            break;
        }
    }

    assert!(saw_busy_cs_low);
    assert!(
        bytes_done >= 2,
        "S2: expected rx_valid for each of 2 bytes, got {bytes_done}"
    );
    assert_eq!(sim.ports().get("cs_n"), Some(1));
    assert_eq!(sim.ports().get("mosi_byte"), Some(b1));
}

#[test]
fn fr98_docs_ip_spi_near_vip_boundaries() {
    let root = workspace_root();
    let readme = fs::read_to_string(root.join("docs/ip/README.md")).expect("docs/ip");
    let lower = readme.to_lowercase();
    assert!(
        readme.contains("SpiMaster")
            && (readme.contains("FR98")
                || readme.contains("近 VIP")
                || readme.contains("near-VIP")),
        "S4: docs/ip must document FR98 SPI near-VIP"
    );
    assert!(
        readme.contains("CPOL")
            || readme.contains("CPHA")
            || lower.contains("cpol")
            || lower.contains("cpha"),
        "S4: docs must mention CPOL/CPHA delivery"
    );
    assert!(
        lower.contains("非目标")
            || lower.contains("non-goal")
            || lower.contains("dma")
            || lower.contains("slave"),
        "S4: docs must state non-goals (DMA/multi-CS/slave/…)"
    );
    assert!(
        !(readme.contains("FR98 全绿") || lower.contains("fr98 complete")),
        "must not claim four-class FR98 green from SPI alone"
    );
    assert!(lower.contains("bitloom"), "public brand Bitloom required");

    let nfr =
        fs::read_to_string(root.join(
            "_agile-output/implementation-artifacts/nfr14-risk-epic43-vip-full-protocol-ip.md",
        ))
        .expect("nfr14");
    assert!(
        nfr.contains("- [ ] **I2C：**")
            || nfr.contains("- [ ] **AXI：**")
            || (nfr.contains("I2C") && nfr.contains("- [ ]") && nfr.contains("43.4")),
        "must not close full Epic 43 NFR14 checklist in Story 43.3"
    );
}
