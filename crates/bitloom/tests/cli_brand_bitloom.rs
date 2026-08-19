//! ATDD: publish package + cargo subcommand binary are Bitloom (Story 11.2 / FR42).

use std::fs;
use std::path::PathBuf;

fn crate_toml() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml")
}

#[test]
fn package_and_bin_are_bitloom() {
    let text = fs::read_to_string(crate_toml()).expect("Cargo.toml");
    assert!(
        text.contains("name = \"bitloom\""),
        "package name must be bitloom:\n{text}"
    );
    assert!(
        text.contains("name = \"cargo-bitloom\""),
        "binary must be cargo-bitloom for `cargo bitloom`:\n{text}"
    );
    assert!(
        !text.contains("name = \"rhdl-rs\"") && !text.contains("name = \"cargo-rhdl\""),
        "must not keep rhdl-rs / cargo-rhdl as package/bin names"
    );
}

#[test]
fn help_brand_is_bitloom() {
    let bin = env!("CARGO_BIN_EXE_cargo-bitloom");
    let out = std::process::Command::new(bin)
        .arg("--help")
        .output()
        .expect("help");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "help failed: {stdout}");
    assert!(
        stdout.to_lowercase().contains("bitloom"),
        "help must mention Bitloom: {stdout}"
    );
    assert!(
        !stdout.contains("crates.io: rhdl-rs"),
        "help must not advertise rhdl-rs: {stdout}"
    );
}
