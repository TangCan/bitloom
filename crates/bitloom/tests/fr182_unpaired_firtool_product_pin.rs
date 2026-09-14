//! ATDD Story 115.2 / FR182 — unpaired firtool product-pin bump to 1.159.0.
//!
//! ```text
//! cargo test -p bitloom --test fr182_unpaired_firtool_product_pin
//! ```

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr182_docs_contract_forbid_prior_alone() {
    let docs = read("docs/fr182-unpaired-firtool-product-pin.md");
    assert!(docs.contains("FR182") && docs.contains("Bitloom"));
    assert!(
        docs.contains("1.159.0") && docs.contains("7.15.0"),
        "must pin unpaired product firtool-1.159.0 with Chisel 7.15.0"
    );
    assert!(
        docs.contains("unpaired") || docs.contains("无上游") || docs.contains("no upstream"),
        "must state unpaired / no upstream official pairing"
    );
    for prior in ["FR173", "FR174", "FR179"] {
        assert!(
            docs.contains(prior)
                && (docs.contains("alone") || docs.contains("≠") || docs.contains("仍")),
            "must keep {prior} distinct"
        );
    }
    assert!(
        docs.contains("AD-9")
            && (docs.contains("NFR85") || docs.contains("exception") || docs.contains("例外")),
        "must note AD-9 unpaired product-pin exception / NFR85"
    );
    assert!(
        (docs.contains("PATH") || docs.contains("Forbidden") || docs.contains("禁止"))
            && (docs.contains("silent") || docs.contains("FORCE") || docs.contains("≠")),
        "must forbid PATH-random / silent-Ok"
    );
}

#[test]
fn fr182_spine_ad9_unpaired_exception() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("FR182") && spine.contains("1.159.0"),
        "AD-9 must document FR182 product pin 1.159.0"
    );
    assert!(
        spine.contains("unpaired") || spine.contains("例外") || spine.contains("exception"),
        "AD-9 must record unpaired product-pin exception"
    );
    assert!(
        spine.contains("7.15.0"),
        "Chisel must remain 7.15.0 under FR182"
    );
    assert!(
        spine.contains("firtool-1.158.0")
            && (spine.contains("FR173") || spine.contains("NFR78") || spine.contains("NFR83")),
        "spine must retain FR173 1.158.0 close honesty"
    );
}

#[test]
fn fr182_cli_and_chisel_constants() {
    let firtool = read("crates/bitloom/src/firtool.rs");
    assert!(
        firtool.contains("1.159.0") && firtool.contains("firtool-1.159.0"),
        "CLI firtool module must download firtool-1.159.0"
    );
    assert!(
        firtool.contains("unpaired") || firtool.contains("FR182"),
        "CLI module must honesty-note FR182 unpaired pin"
    );
    let chisel = read("crates/rhdl-firrtl/src/chisel.rs");
    assert!(
        chisel.contains("\"7.15.0\"") && chisel.contains("\"1.159.0\""),
        "CHISEL_TARGET / FIRTOOL_TARGET must be 7.15.0 / 1.159.0"
    );
    assert!(
        chisel.contains("unpaired") || chisel.contains("FR182"),
        "chisel.rs must honesty-note unpaired FR182 pin"
    );
}

#[test]
fn fr182_scripts_pin_new_version() {
    for script in [
        "scripts/circt-external-check.sh",
        "scripts/circt-external-sim-check.sh",
        "scripts/circt-external-alloc-check.sh",
        "scripts/parser-restore-check.sh",
        "scripts/firtool-smoke.sh",
        "scripts/chisel-fr28-compile.sh",
    ] {
        let text = read(script);
        assert!(
            text.contains("1.159.0"),
            "{script} must expect firtool 1.159.0"
        );
    }
    let chisel = read("scripts/chisel-fr28-compile.sh");
    assert!(
        chisel.contains("7.15.0"),
        "FR28 compile script must keep Chisel 7.15.0"
    );
    let floating = read("scripts/circt-floating-git-head-check.sh");
    assert!(
        floating.contains("AD9_PRODUCT_VERSION=\"1.159.0\""),
        "FR179 script must contrast AD-9 product 1.159.0"
    );
    assert!(
        floating.contains("floating-head") || floating.contains("channel"),
        "FR179 must distinguish channel by path when versions coincide"
    );
    assert!(
        !floating.contains("firtool reports AD-9 product pin")
            || floating.contains("coincide")
            || floating.contains("path/cache"),
        "must not unconditionally reject version equality with AD-9 after FR182"
    );
}

#[test]
fn fr182_nfr83_prior_closes_still_documented() {
    let fr173 = read("docs/fr173-firtool-bump-ad9.md");
    assert!(
        fr173.contains("1.158.0")
            && (fr173.contains("closed") || fr173.contains("已关闭") || fr173.contains("106.3")),
        "FR173 close evidence @ 1.158.0 must remain"
    );
    let fr174 = read("docs/fr174-unpaired-head.md");
    assert!(
        fr174.contains("1.156.0")
            && (fr174.contains("closed") || fr174.contains("已关闭") || fr174.contains("107.3")),
        "FR174 close evidence must remain"
    );
    let fr179 = read("docs/fr179-floating-circt-git-head.md");
    assert!(
        fr179.contains("1.159.0")
            && (fr179.contains("closed") || fr179.contains("已关闭") || fr179.contains("112.3")),
        "FR179 close evidence must remain"
    );
}

#[test]
fn fr182_live_firtool_info_reports_1159() {
    let out = Command::new("cargo")
        .args(["run", "-q", "-p", "bitloom", "--", "firtool", "info"])
        .current_dir(root())
        .output()
        .expect("cargo run firtool info");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "firtool info must succeed: stdout={stdout} stderr={stderr}"
    );
    assert!(
        stdout.contains("1.159.0"),
        "firtool info must report 1.159.0; got {stdout}"
    );
}

#[test]
fn fr182_live_ensure_downloads_when_needed() {
    let out = Command::new("cargo")
        .args(["run", "-q", "-p", "bitloom", "--", "firtool", "ensure"])
        .current_dir(root())
        .output()
        .expect("cargo run firtool ensure");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "firtool ensure must succeed: stdout={stdout} stderr={stderr}"
    );
}
