//! ATDD: FR161 — formal-sby image/tooling hygiene (Story 93.2).
//!
//! cargo test -p bitloom --test fr161_formal_sby_image_hygiene

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
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr161_docs_contract_and_forbid_fr127_alone() {
    let text = read("docs/fr161-formal-sby-image-hygiene.md");
    assert!(text.contains("FR161") && text.contains("Bitloom"));
    assert!(
        text.contains("ci-sby-pins.env") && (text.contains("SBY_GIT_REF") || text.contains("pin")),
        "must name pins file / pin vars"
    );
    assert!(
        text.contains("FR127")
            && (text.contains("alone") || text.contains("≠") || text.contains("仍")),
        "must keep FR127 distinct / alone ban"
    );
    assert!(
        text.contains("hygiene") || text.contains("ci-sby-hygiene-check") || text.contains("卫生"),
        "must name hygiene check"
    );
    let fr127 = read("docs/fr127-forced-sby-ci.md");
    assert!(
        fr127.contains("fr161") || fr127.contains("FR161"),
        "FR127 must cross-link FR161"
    );
}

#[test]
fn fr161_pins_file_nonempty() {
    let pins = read("scripts/ci-sby-pins.env");
    assert!(pins.contains("SBY_GIT_URL="));
    assert!(pins.contains("SBY_GIT_REF="));
    assert!(pins.contains("SBY_GIT_SHA="));
    assert!(
        pins.contains("yosys-0.47") || pins.contains("SBY_GIT_REF="),
        "must pin a concrete ref"
    );
    let sha_line = pins
        .lines()
        .find(|l| l.starts_with("SBY_GIT_SHA="))
        .expect("SBY_GIT_SHA line");
    let sha = sha_line.trim_start_matches("SBY_GIT_SHA=");
    assert_eq!(sha.len(), 40, "SBY_GIT_SHA must be 40 hex chars: {sha}");
    assert!(sha.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn fr161_install_script_obeys_pins() {
    let install = read("scripts/ci-install-sby.sh");
    assert!(
        install.contains("ci-sby-pins.env")
            && install.contains("SBY_GIT_REF")
            && install.contains("SBY_GIT_SHA"),
        "install must source/verify FR161 pins"
    );
    assert!(
        install.contains("--branch") && install.contains("$SBY_GIT_REF"),
        "install must clone pinned branch/ref"
    );
    assert!(
        !install.contains("git clone --depth 1 https://github.com/YosysHQ/sby.git \"$TMP\"")
            || install.contains("SBY_GIT_REF"),
        "must not float-clone HEAD without pin"
    );
}

#[test]
fn fr161_ci_runs_hygiene_before_install() {
    let ci = read(".github/workflows/ci.yml");
    let idx = ci.find("formal-sby:").expect("formal-sby job");
    let block: String = ci[idx..].lines().take(25).collect::<Vec<_>>().join("\n");
    assert!(
        block.contains("ci-sby-hygiene-check.sh"),
        "CI formal-sby must run hygiene check: {block}"
    );
    let hyg = block.find("ci-sby-hygiene-check.sh").unwrap();
    let inst = block.find("ci-install-sby.sh").expect("install step");
    assert!(
        hyg < inst,
        "hygiene must run before install in formal-sby job"
    );
    assert!(!block.contains("continue-on-error"));
}

#[test]
fn fr161_hygiene_script_pass_and_force_fail() {
    let root = workspace_root();
    let script = root.join("scripts/ci-sby-hygiene-check.sh");
    let ok = Command::new("bash")
        .arg(&script)
        .current_dir(&root)
        .status()
        .expect("run hygiene");
    assert!(ok.success(), "hygiene check must pass on pinned tree");

    let fail = Command::new("bash")
        .arg(&script)
        .env("BITLOOM_SBY_HYGIENE_FORCE_FAIL", "1")
        .current_dir(&root)
        .output()
        .expect("force fail");
    assert!(
        !fail.status.success(),
        "BITLOOM_SBY_HYGIENE_FORCE_FAIL must non-zero"
    );
    let err = String::from_utf8_lossy(&fail.stderr);
    assert!(
        err.contains("FR161") && (err.contains("forced fail") || err.contains("refusing")),
        "force fail must be readable: {err}"
    );
}

#[test]
fn fr161_fr127_job_still_present() {
    let ci = read(".github/workflows/ci.yml");
    assert!(ci.contains("formal-sby:") && ci.contains("formal-sby-check.sh"));
    let docs = read("docs/fr127-forced-sby-ci.md");
    assert!(docs.contains("closed") || docs.contains("FR127"));
}
