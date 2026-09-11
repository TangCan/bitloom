//! ATDD: FR144 surface semver CI / just gate.
//!
//! ```text
//! cargo test -p bitloom --test fr144_semver_ci_gate
//! ```

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    let path = workspace_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn fr144_script_and_just_exist() {
    let script = read("scripts/semver-check.sh");
    assert!(
        script.contains("cargo-semver-checks"),
        "script must invoke cargo-semver-checks"
    );
    assert!(
        script.contains("BITLOOM_SEMVER_FORCE_MISSING"),
        "script must support forced missing-tool path"
    );
    assert!(
        script.contains("bitloom-prelude") && script.contains("bitloom-sim"),
        "script must check FR142 library surface crates"
    );
    assert!(
        script.contains("release-type") || script.contains("RELEASE_TYPE"),
        "script must select release-type (pre-1.0 major)"
    );
    assert!(
        script.contains("refusing silent success") || script.contains("非零"),
        "script must refuse silent success when tool missing"
    );

    let just = read("Justfile");
    assert!(
        just.contains("semver-check:") && just.contains("scripts/semver-check.sh"),
        "Justfile must expose semver-check recipe"
    );
}

#[test]
fn fr144_ci_required_job_no_continue_on_error() {
    let ci = read(".github/workflows/ci.yml");
    assert!(
        ci.contains("semver-check:") || ci.contains("\n  semver-check:"),
        "CI must define semver-check job"
    );
    assert!(
        ci.contains("cargo-semver-checks") || ci.contains("semver-check.sh"),
        "CI must install/run cargo-semver-checks / script"
    );
    // Job steps must not enable continue-on-error (ignore comment-only mentions elsewhere).
    let idx = ci.find("semver-check:").expect("semver-check job");
    let block: String = ci[idx..].lines().take(25).collect::<Vec<_>>().join("\n");
    assert!(
        !block.lines().any(|l| {
            let t = l.trim();
            t.starts_with("continue-on-error:") || t.contains("continue-on-error: true")
        }),
        "semver-check job steps must not set continue-on-error"
    );
    assert!(
        ci.contains("FR144") || ci.contains("semver-check.sh"),
        "CI must document FR144 / script path"
    );
}

#[test]
fn fr144_policy_documents_local_repro() {
    let policy = read("docs/semver-1-0-policy.md");
    assert!(
        policy.contains("just semver-check") && policy.contains("FR144"),
        "policy must document local just semver-check for FR144"
    );
}

#[test]
fn fr144_force_missing_tool_exits_nonzero() {
    let script = workspace_root().join("scripts/semver-check.sh");
    let out = Command::new("bash")
        .arg(&script)
        .env("BITLOOM_SEMVER_FORCE_MISSING", "1")
        .output()
        .expect("run semver-check.sh");
    assert!(
        !out.status.success(),
        "BITLOOM_SEMVER_FORCE_MISSING=1 must exit non-zero; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("cargo-semver-checks") || err.contains("unavailable"),
        "stderr must diagnose missing tool: {err}"
    );
}
