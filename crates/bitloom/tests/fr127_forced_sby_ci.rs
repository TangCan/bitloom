//! ATDD Story 67.2 / FR127 — required CI formal-sby job.

use std::fs;
use std::path::PathBuf;

fn read(rel: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    fs::read_to_string(root.join(rel)).unwrap()
}

#[test]
fn fr127_ci_job_required_no_continue_on_error() {
    let ci = read(".github/workflows/ci.yml");
    assert!(ci.contains("formal-sby:"), "must define formal-sby job");
    assert!(
        ci.contains("ci-install-sby.sh") && ci.contains("formal-sby-check"),
        "must install sby and run formal-sby-check"
    );
    // Job block must not enable continue-on-error
    let idx = ci.find("formal-sby:").expect("formal-sby");
    let rest = &ci[idx..];
    let end = rest.find("\n  [a-z]").unwrap_or(rest.len());
    // crude: take until next top-level-ish job or EOF — scan 40 lines
    let block: String = rest.lines().take(40).collect::<Vec<_>>().join("\n");
    assert!(
        !block.contains("continue-on-error"),
        "FR127 required job must not continue-on-error"
    );
    assert!(block.contains("timeout-minutes"));
    let _ = end;
}

#[test]
fn fr127_install_script_exists() {
    let script = read("scripts/ci-install-sby.sh");
    assert!(script.contains("sby") && script.contains("yosys") && script.contains("z3"));
    assert!(script.contains("FR127") || script.contains("Bitloom"));
}

#[test]
fn fr127_docs_boundary_vs_fr119() {
    let docs = read("docs/fr127-forced-sby-ci.md");
    assert!(docs.contains("FR127") && docs.contains("Bitloom"));
    assert!(docs.contains("formal-sby") && docs.contains("FR119"));
    assert!(docs.contains("alone") || docs.contains("≠") || docs.contains("不得"));
}

#[test]
fn fr127_force_missing_script_contract() {
    // Documented local path still fails readably when forced missing (FR119/FR127 shared script).
    let script = read("scripts/formal-sby-check.sh");
    assert!(script.contains("BITLOOM_SBY_FORCE_MISSING"));
    assert!(script.contains("refusing silent success") || script.contains("exit 1"));
}
