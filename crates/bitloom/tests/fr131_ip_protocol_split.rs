//! ATDD Story 71.2 / FR131 — ip.rs protocol split.

use std::fs;
use std::path::PathBuf;

use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::{Axi4LiteSlave, Gpio, GpioVip, I2cMaster, SpiMaster, SyncFifo, UartTx};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn fr131_no_monolith_ip_rs() {
    let monolith = root().join("crates/bitloom-prelude/src/ip.rs");
    assert!(
        !monolith.exists(),
        "FR131 requires split: monolith ip.rs must not remain"
    );
}

#[test]
fn fr131_protocol_modules_exist() {
    let ip = root().join("crates/bitloom-prelude/src/ip");
    for name in [
        "mod.rs",
        "sync_fifo.rs",
        "uart.rs",
        "spi.rs",
        "i2c.rs",
        "axi.rs",
        "blackbox.rs",
        "crc.rs",
    ] {
        assert!(ip.join(name).is_file(), "missing ip/{name}");
    }
    // FR131 protocol module `gpio` may be `gpio.rs` or FR139 directory `gpio/`.
    assert!(
        ip.join("gpio.rs").is_file() || ip.join("gpio/mod.rs").is_file(),
        "missing ip/gpio.rs or ip/gpio/mod.rs"
    );
    let mod_rs = fs::read_to_string(ip.join("mod.rs")).unwrap();
    assert!(mod_rs.contains("FR131") || mod_rs.contains("pub use"));
    assert!(mod_rs.contains("mod uart") && mod_rs.contains("mod gpio"));
}

#[test]
fn fr131_public_api_stable_elaborate() {
    assert!(SyncFifo::elaborate().is_ok());
    assert!(UartTx::elaborate().is_ok());
    assert!(SpiMaster::elaborate().is_ok());
    assert!(I2cMaster::elaborate().is_ok());
    assert!(Axi4LiteSlave::elaborate().is_ok());
    assert!(Gpio::elaborate().is_ok());
    assert!(GpioVip::elaborate().is_ok());
}

#[test]
fn fr131_docs_and_nfr14() {
    let docs = fs::read_to_string(root().join("docs/fr131-ip-protocol-split.md")).unwrap();
    assert!(docs.contains("FR131") && docs.contains("Bitloom"));
    assert!(docs.contains("ip/") && (docs.contains("协议") || docs.contains("protocol")));
    assert!(docs.contains("FR98") && docs.contains("FR108") && docs.contains("FR120"));
    let readme = fs::read_to_string(root().join("docs/ip/README.md")).unwrap();
    assert!(readme.contains("ip/") && readme.contains("FR131"));
    let nfr =
        fs::read_to_string(root().join(
            "_agile-output/implementation-artifacts/nfr14-risk-epic71-ip-rs-protocol-split.md",
        ))
        .unwrap();
    assert!(nfr.contains("P1") && nfr.contains("P4"));
}
