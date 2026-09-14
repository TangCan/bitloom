//! ATDD Story 119.2 / FR186 — unbounded / live CIRCT tip beyond FR179.
//!
//! ```text
//! cargo test -p bitloom --test fr186_unbounded_circt_tip
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
fn fr186_docs_contract_forbid_fr179_alone() {
    let docs = read("docs/fr186-unbounded-circt-tip.md");
    assert!(docs.contains("FR186") && docs.contains("Bitloom"));
    assert!(
        docs.contains("FR179") && docs.contains("1.159.0"),
        "must contrast FR179 floating-track 1.159.0"
    );
    assert!(
        docs.contains("live tip") || docs.contains("live-tip") || docs.contains("无界"),
        "must describe live tip"
    );
    assert!(
        docs.contains("live-tip")
            || docs.contains("BITLOOM_FIRTOOL_LIVE_TIP_PATH")
            || docs.contains("channel"),
        "must distinguish live-tip channel by path"
    );
    assert!(
        docs.contains("非产品默认") || docs.contains("not the product default"),
        "must honesty-state not product default pin"
    );
    assert!(
        docs.contains("FR179")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("仍")),
        "must keep FR179 distinct"
    );
    assert!(
        docs.contains("AD-9")
            && (docs.contains("NFR90") || docs.contains("revise") || docs.contains("修订")),
        "must note AD-9 revise / NFR90"
    );
    assert!(
        (docs.contains("PATH") || docs.contains("Forbidden") || docs.contains("禁止"))
            && (docs.contains("FORCE_MISSING")
                || docs.contains("BITLOOM_FIRTOOL_LIVE_TIP_FORCE_MISSING")),
        "must forbid PATH-random / silent-Ok"
    );
}

#[test]
fn fr186_script_and_just_and_ci() {
    let script = read("scripts/circt-live-tip-check.sh");
    assert!(script.contains("FR186") && script.contains("live-tip"));
    assert!(script.contains("BITLOOM_FIRTOOL_LIVE_TIP_FORCE_MISSING"));
    assert!(script.contains("BITLOOM_FIRTOOL_LIVE_TIP_PATH"));
    assert!(
        script.contains("floating-head") || script.contains("FLOATING"),
        "script must reject FR179 floating-head cache path"
    );
    assert!(
        script.contains("latest") || script.contains("resolve_latest") || script.contains("API"),
        "script must resolve live tip identity"
    );
    let just = read("Justfile");
    assert!(just.contains("circt-live-tip-check"));
    assert!(just.contains("scripts/circt-live-tip-check.sh"));
    let ci = read(".github/workflows/ci.yml");
    assert!(
        ci.contains("circt-live-tip") && ci.contains("circt-live-tip-check.sh"),
        "CI must run FR186 gate"
    );
}

#[test]
fn fr186_spine_ad9_revised() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("FR186")
            && (spine.contains("live tip") || spine.contains("live-tip") || spine.contains("无界")),
        "AD-9 must document FR186 live tip channel"
    );
    assert!(
        spine.contains("circt-live-tip") || spine.contains("BITLOOM_FIRTOOL_LIVE_TIP_PATH"),
        "spine must name live-tip check / env"
    );
    assert!(
        spine.contains("NFR90") || spine.contains("correctCoursePhase23Approved"),
        "spine must cite NFR90 / Phase 23 revise"
    );
    assert!(
        spine.contains("非产品默认") || spine.contains("非产品默认钉") || spine.contains("诚实"),
        "spine must keep non-default-pin honesty"
    );
}

#[test]
fn fr186_force_missing_nonzero() {
    let out = Command::new("bash")
        .arg("scripts/circt-live-tip-check.sh")
        .current_dir(root())
        .env("BITLOOM_FIRTOOL_LIVE_TIP_FORCE_MISSING", "1")
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
fn fr186_live_tip_check() {
    let out = Command::new("bash")
        .arg("scripts/circt-live-tip-check.sh")
        .current_dir(root())
        .env_remove("BITLOOM_FIRTOOL_LIVE_TIP_FORCE_MISSING")
        // Pin tip tag for deterministic CI/cache while still using live-tip channel.
        .env("BITLOOM_FIRTOOL_LIVE_TIP_VERSION", "1.159.0")
        .output()
        .expect("run live tip check");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "live tip check must pass; stdout={stdout} stderr={stderr}"
    );
    assert!(
        stdout.contains("live-tip")
            || stdout.contains("tip")
            || stdout.contains("OK")
            || stdout.contains("FR186"),
        "stdout should mention live tip; got {stdout}"
    );
}
