//! ATDD Story 106.2 / FR173 — firtool bump beyond AD-9 with Chisel pairing.
//!
//! ```text
//! cargo test -p bitloom --test fr173_firtool_bump_ad9
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
fn fr173_docs_and_ad9_pin_chisel_715_firtool_1158() {
    let docs = read("docs/fr173-firtool-bump-ad9.md");
    assert!(docs.contains("FR173") && docs.contains("Bitloom"));
    assert!(
        docs.contains("7.15.0") && (docs.contains("1.158.0") || docs.contains("firtool-1.158")),
        "docs must pin Chisel 7.15.0 ↔ firtool-1.158.0"
    );
    assert!(
        docs.contains("FR169")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("仍")),
        "must keep FR169 distinct"
    );
    assert!(
        (docs.contains("PATH") || docs.contains("HEAD"))
            && (docs.contains("Forbidden") || docs.contains("禁止") || docs.contains("≠")),
        "must forbid PATH-random / unpaired HEAD"
    );

    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("firtool-1.158.0") && spine.contains("7.15.0"),
        "AD-9 / Stack must revise to firtool-1.158.0 ↔ Chisel 7.15.0"
    );
    assert!(
        spine.contains("FR173") && (spine.contains("Revised") || spine.contains("修订")),
        "spine must record FR173 AD-9 revise"
    );
    assert!(
        spine.contains("NFR78")
            || (spine.contains("1.155.0") && (spine.contains("仍") || spine.contains("历史"))),
        "spine must keep NFR78 historical pin honesty"
    );
}

#[test]
fn fr173_cli_and_chisel_constants() {
    let firtool = read("crates/bitloom/src/firtool.rs");
    assert!(
        firtool.contains("1.158.0") && firtool.contains("firtool-1.158.0"),
        "CLI firtool module must download firtool-1.158.0"
    );
    let chisel = read("crates/rhdl-firrtl/src/chisel.rs");
    assert!(
        chisel.contains("\"7.15.0\"") && chisel.contains("\"1.158.0\""),
        "CHISEL_TARGET / FIRTOOL_TARGET must be 7.15.0 / 1.158.0"
    );
}

#[test]
fn fr173_scripts_pin_new_version() {
    for script in [
        "scripts/circt-external-check.sh",
        "scripts/circt-external-sim-check.sh",
        "scripts/circt-external-alloc-check.sh",
        "scripts/parser-restore-check.sh",
        "scripts/chisel-fr28-compile.sh",
    ] {
        let text = read(script);
        assert!(
            text.contains("1.158.0"),
            "{script} must expect firtool 1.158.0"
        );
    }
    let chisel = read("scripts/chisel-fr28-compile.sh");
    assert!(
        chisel.contains("7.15.0"),
        "FR28 compile script must pin Chisel 7.15.0"
    );
}

#[test]
fn fr173_nfr78_prior_closes_still_documented() {
    let fr169 = read("docs/fr169-circt-mlir-allocation.md");
    assert!(
        fr169.contains("closed") || fr169.contains("已关闭") || fr169.contains("102.3"),
        "FR169 close evidence must remain"
    );
    assert!(
        fr169.contains("1.155.0"),
        "FR169 historical close pin 1.155.0 must remain documented"
    );
}

#[test]
fn fr173_live_firtool_info_reports_1158() {
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
        stdout.contains("1.158.0"),
        "firtool info must report 1.158.0; got {stdout}"
    );
}

#[test]
fn fr173_live_ensure_downloads_when_needed() {
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
    let combined = format!("{stdout}{stderr}");
    assert!(
        combined.contains("firtool") || PathBuf::from(stdout.trim()).exists() || !stdout.is_empty(),
        "ensure should print firtool path; stdout={stdout} stderr={stderr}"
    );
}
