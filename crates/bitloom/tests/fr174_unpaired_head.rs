//! ATDD Story 107.2 / FR174 — unpaired CIRCT/firtool document-pinned mainline.
//!
//! ```text
//! cargo test -p bitloom --test fr174_unpaired_head
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
fn fr174_docs_contract_forbid_fr170_alone() {
    let docs = read("docs/fr174-unpaired-head.md");
    assert!(docs.contains("FR174") && docs.contains("Bitloom"));
    assert!(
        docs.contains("1.156.0") && docs.contains("1.158.0"),
        "must pin unpaired 1.156.0 and contrast AD-9 1.158.0"
    );
    assert!(
        docs.contains("FR170")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("仍")),
        "must keep FR170 distinct"
    );
    assert!(
        docs.contains("AD-9")
            && (docs.contains("AD-27") || docs.contains("revise") || docs.contains("修订")),
        "must note AD revise"
    );
    assert!(
        (docs.contains("PATH") || docs.contains("Forbidden") || docs.contains("禁止"))
            && (docs.contains("FORCE_MISSING")
                || docs.contains("BITLOOM_FIRTOOL_HEAD_FORCE_MISSING")),
        "must forbid PATH-random / silent-Ok"
    );
}

#[test]
fn fr174_script_and_just_and_ci() {
    let script = read("scripts/circt-unpaired-head-check.sh");
    assert!(script.contains("1.156.0") && script.contains("1.158.0"));
    assert!(script.contains("BITLOOM_FIRTOOL_HEAD_FORCE_MISSING"));
    assert!(script.contains("BITLOOM_FIRTOOL_HEAD_PATH") || script.contains("FIRTOOL_HEAD"));
    assert!(script.contains("FR174"));
    let just = read("Justfile");
    assert!(just.contains("circt-unpaired-head-check"));
    assert!(just.contains("scripts/circt-unpaired-head-check.sh"));
    let ci = read(".github/workflows/ci.yml");
    assert!(
        ci.contains("circt-unpaired-head") && ci.contains("circt-unpaired-head-check.sh"),
        "CI must run FR174 gate"
    );
    assert!(
        !ci.contains("continue-on-error: true")
            || !ci
                .lines()
                .any(|l| l.contains("circt-unpaired-head") && l.contains("continue-on-error")),
        "FR174 CI must not continue-on-error"
    );
}

#[test]
fn fr174_spine_ad9_ad27_revised() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("FR174") && spine.contains("1.156.0"),
        "AD-9 must document FR174 unpaired channel"
    );
    assert!(
        spine.contains("circt-unpaired-head") || spine.contains("BITLOOM_FIRTOOL_HEAD_PATH"),
        "spine must name unpaired check / env"
    );
    assert!(
        spine.contains("AD-27") && spine.contains("FR174"),
        "AD-27 must note FR174 revise"
    );
}

#[test]
fn fr174_force_missing_nonzero() {
    let out = Command::new("bash")
        .arg("scripts/circt-unpaired-head-check.sh")
        .current_dir(root())
        .env("BITLOOM_FIRTOOL_HEAD_FORCE_MISSING", "1")
        .output()
        .expect("run force-missing");
    assert!(
        !out.status.success(),
        "FORCE_MISSING must be non-zero; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("FORCE_MISSING") || err.contains("refusing") || err.contains("unavailable"),
        "stderr must be readable; got {err}"
    );
}

#[test]
fn fr174_live_unpaired_check() {
    let out = Command::new("bash")
        .arg("scripts/circt-unpaired-head-check.sh")
        .current_dir(root())
        .env_remove("BITLOOM_FIRTOOL_HEAD_FORCE_MISSING")
        .output()
        .expect("run live unpaired check");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "live unpaired check must pass; stdout={stdout} stderr={stderr}"
    );
    assert!(
        stdout.contains("1.156.0") || stdout.contains("OK"),
        "stdout should mention unpaired pin; got {stdout}"
    );
}
