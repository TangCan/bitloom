//! ATDD Story 78.2 / FR139 — VIP/SocPad further split (C1) inside bitloom-prelude.
//! C2 cross-crate not selected; public `bitloom_prelude::ip::*` paths stable (C3).

use std::fs;
use std::path::PathBuf;

use bitloom_prelude::Elaboratable;
use bitloom_prelude::ip::{
    Axi4LiteSlave, Gpio, GpioSocPad, GpioVip, I2cMaster, SpiMaster, SyncFifo, UartTx,
};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn fr139_gpio_submodules_exist() {
    let gpio = root().join("crates/bitloom-prelude/src/ip/gpio");
    assert!(
        !root()
            .join("crates/bitloom-prelude/src/ip/gpio.rs")
            .exists(),
        "FR139 C1: monolith ip/gpio.rs must be replaced by ip/gpio/"
    );
    for name in ["mod.rs", "base.rs", "vip.rs", "socpad.rs"] {
        assert!(gpio.join(name).is_file(), "missing ip/gpio/{name}");
    }
    let mod_rs = fs::read_to_string(gpio.join("mod.rs")).unwrap();
    assert!(
        mod_rs.contains("FR139") || mod_rs.contains("pub use"),
        "gpio/mod.rs must re-export and cite FR139"
    );
    assert!(
        mod_rs.contains("mod base") && mod_rs.contains("mod vip") && mod_rs.contains("mod socpad"),
        "gpio/mod.rs must declare base/vip/socpad"
    );
    assert!(
        mod_rs.contains("pub use base")
            && mod_rs.contains("pub use vip")
            && mod_rs.contains("pub use socpad"),
        "gpio/mod.rs must pub use submodules"
    );
}

#[test]
fn fr139_public_api_stable_elaborate() {
    assert!(Gpio::elaborate().is_ok());
    assert!(GpioVip::elaborate().is_ok());
    assert!(GpioSocPad::elaborate().is_ok());
    // C4 regression smoke: FR98 four classes + SyncFifo still elaborate
    assert!(SyncFifo::elaborate().is_ok());
    assert!(UartTx::elaborate().is_ok());
    assert!(SpiMaster::elaborate().is_ok());
    assert!(I2cMaster::elaborate().is_ok());
    assert!(Axi4LiteSlave::elaborate().is_ok());
}

#[test]
fn fr139_nfr14_and_no_cross_crate() {
    let nfr = fs::read_to_string(root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic78-vip-socpad-cross-crate.md",
    ))
    .unwrap();
    assert!(nfr.contains("C1") && nfr.contains("C2") && nfr.contains("FR139"));
    assert!(
        nfr.contains("未勾选 C2") || nfr.contains("默认保留 prelude"),
        "risk record must default to prelude-internal split when C2 not selected"
    );
    // No silent new IP crate: Cargo.toml workspace must not invent bitloom-ip-gpio without C2
    let cargo = fs::read_to_string(root().join("Cargo.toml")).unwrap();
    assert!(
        !cargo.contains("bitloom-ip-gpio") && !cargo.contains("bitloom_gpio"),
        "C2 not selected: must not add separate GPIO IP crate"
    );
    let ip_mod = fs::read_to_string(root().join("crates/bitloom-prelude/src/ip/mod.rs")).unwrap();
    assert!(
        ip_mod.contains("FR139") || ip_mod.contains("gpio"),
        "ip/mod.rs must remain the FR131 protocol surface including gpio"
    );
}
