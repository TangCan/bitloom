//! ATDD Story 112.2 / FR179 — floating CIRCT HEAD track beyond FR174.
//!
//! ```text
//! cargo test -p bitloom --test fr179_floating_circt_git_head
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
fn fr179_docs_contract_forbid_fr174_alone() {
    let docs = read("docs/fr179-floating-circt-git-head.md");
    assert!(docs.contains("FR179") && docs.contains("Bitloom"));
    assert!(
        docs.contains("1.159.0") && docs.contains("1.156.0"),
        "must pin floating-track 1.159.0 and contrast FR174 1.156.0"
    );
    assert!(
        docs.contains("1.158.0") || docs.contains("FR173") || docs.contains("FR182"),
        "must retain FR173/FR182 honesty for product pin history"
    );
    assert!(
        docs.contains("floating-head")
            || docs.contains("channel")
            || docs.contains("BITLOOM_FIRTOOL_FLOATING_HEAD_PATH"),
        "must distinguish floating-track channel by path when versions coincide with AD-9"
    );
    assert!(
        docs.contains("FR174")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("仍")),
        "must keep FR174 distinct"
    );
    assert!(
        docs.contains("AD-9")
            && (docs.contains("NFR85") || docs.contains("revise") || docs.contains("修订")),
        "must note AD-9 revise / NFR85"
    );
    assert!(
        (docs.contains("PATH") || docs.contains("Forbidden") || docs.contains("禁止"))
            && (docs.contains("FORCE_MISSING")
                || docs.contains("BITLOOM_FIRTOOL_FLOATING_HEAD_FORCE_MISSING")),
        "must forbid PATH-random / silent-Ok"
    );
    assert!(
        docs.contains("NFR86") || docs.contains("unbounded") || docs.contains("live tip"),
        "must leave unbounded tip as NFR86 honesty"
    );
}

#[test]
fn fr179_script_and_just_and_ci() {
    let script = read("scripts/circt-floating-git-head-check.sh");
    assert!(script.contains("1.159.0") && script.contains("1.156.0"));
    assert!(script.contains("AD9_PRODUCT_VERSION=\"1.159.0\""));
    assert!(
        script.contains("floating-head") || script.contains("channel"),
        "script must distinguish channel by path"
    );
    assert!(
        !script.contains("firtool reports AD-9 product pin")
            || script.contains("coincide")
            || script.contains("path/cache"),
        "must not reject AD-9 version equality unconditionally after FR182"
    );
    assert!(script.contains("BITLOOM_FIRTOOL_FLOATING_HEAD_FORCE_MISSING"));
    assert!(script.contains("BITLOOM_FIRTOOL_FLOATING_HEAD_PATH"));
    assert!(script.contains("FR179"));
    let just = read("Justfile");
    assert!(just.contains("circt-floating-git-head-check"));
    assert!(just.contains("scripts/circt-floating-git-head-check.sh"));
    let ci = read(".github/workflows/ci.yml");
    assert!(
        ci.contains("circt-floating-git-head") && ci.contains("circt-floating-git-head-check.sh"),
        "CI must run FR179 gate"
    );
}

#[test]
fn fr179_spine_ad9_revised() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("FR179") && spine.contains("1.159.0"),
        "AD-9 must document FR179 floating-track channel"
    );
    assert!(
        spine.contains("circt-floating-git-head")
            || spine.contains("BITLOOM_FIRTOOL_FLOATING_HEAD_PATH"),
        "spine must name floating-head check / env"
    );
}

#[test]
fn fr179_force_missing_nonzero() {
    let out = Command::new("bash")
        .arg("scripts/circt-floating-git-head-check.sh")
        .current_dir(root())
        .env("BITLOOM_FIRTOOL_FLOATING_HEAD_FORCE_MISSING", "1")
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
fn fr179_live_floating_check() {
    let out = Command::new("bash")
        .arg("scripts/circt-floating-git-head-check.sh")
        .current_dir(root())
        .env_remove("BITLOOM_FIRTOOL_FLOATING_HEAD_FORCE_MISSING")
        .output()
        .expect("run live floating check");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "live floating check must pass; stdout={stdout} stderr={stderr}"
    );
    assert!(
        stdout.contains("1.159.0") || stdout.contains("OK"),
        "stdout should mention floating-track pin; got {stdout}"
    );
}
