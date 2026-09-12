//! ATDD Story 102.2 / FR169 — external CIRCT multi-lower / MLIR allocation gate.
//!
//! ```text
//! cargo test -p bitloom --test fr169_circt_mlir_allocation
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
fn fr169_docs_contract_forbid_fr164_alone() {
    let docs = read("docs/fr169-circt-mlir-allocation.md");
    assert!(docs.contains("FR169") && docs.contains("Bitloom"));
    assert!(
        docs.contains("multi-lower")
            || docs.contains("allocation")
            || docs.contains("hw.module")
            || docs.contains("--ir-hw"),
        "must name multi-lower / allocation gate"
    );
    assert!(
        docs.contains("FR164")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("仍")),
        "must keep FR164 distinct"
    );
    assert!(
        docs.contains("FR137")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("仍")),
        "must keep FR137 distinct"
    );
    assert!(docs.contains("1.155.0") || docs.contains("firtool-1.155.0"));
    assert!(
        docs.contains("circt-external-alloc-check") || docs.contains("circt-external-alloc"),
        "must name alloc-check path / CI job"
    );
    assert!(
        (docs.contains("升钉")
            || docs.contains("bump")
            || docs.contains("option B")
            || docs.contains("(B)"))
            && (docs.contains("Deferred")
                || docs.contains("不做")
                || docs.contains("not")
                || docs.contains("未")),
        "must defer firtool bump option B"
    );
}

#[test]
fn fr169_script_and_just_path() {
    let script = read("scripts/circt-external-alloc-check.sh");
    assert!(script.contains("1.155.0"));
    assert!(script.contains("FR169"));
    assert!(
        script.contains("RHDL_FIRTOOL_PATH") || script.contains("firtool ensure"),
        "must resolve via AD-9"
    );
    assert!(
        script.contains("--ir-hw") && script.contains("--ir-fir"),
        "must run multi-lower IR emits beyond compile/sim"
    );
    assert!(
        script.contains("BITLOOM_CIRCT_ALLOC_FORCE_MISSING"),
        "must honor FORCE_MISSING"
    );
    let just = read("Justfile");
    assert!(just.contains("circt-external-alloc-check"));
    assert!(just.contains("scripts/circt-external-alloc-check.sh"));
}

#[test]
fn fr169_ci_required_job_no_continue_on_error() {
    let ci = read(".github/workflows/ci.yml");
    assert!(
        ci.contains("circt-external-alloc:"),
        "must define required circt-external-alloc job"
    );
    let idx = ci
        .find("circt-external-alloc:")
        .expect("circt-external-alloc");
    let block: String = ci[idx..].lines().take(40).collect::<Vec<_>>().join("\n");
    assert!(
        !block.contains("continue-on-error"),
        "FR169 required job must not continue-on-error"
    );
    assert!(
        block.contains("circt-external-alloc-check"),
        "CI must run circt-external-alloc-check"
    );
}

#[test]
fn fr169_force_missing_nonzero_readable() {
    let script = root().join("scripts/circt-external-alloc-check.sh");
    assert!(script.is_file());
    let out = Command::new("bash")
        .arg(&script)
        .env("BITLOOM_CIRCT_ALLOC_FORCE_MISSING", "1")
        .current_dir(root())
        .output()
        .expect("spawn");
    assert!(!out.status.success(), "FORCE_MISSING must fail non-zero");
    let err = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        err.contains("FORCE_MISSING")
            || err.contains("unavailable")
            || err.contains("missing")
            || err.contains("refusing silent"),
        "failure must be readable: {err}"
    );
}

#[test]
fn fr169_not_satisfied_by_fr164_or_fr137_alone() {
    let fr137 = read("scripts/circt-external-check.sh");
    let fr164 = read("scripts/circt-external-sim-check.sh");
    let fr169 = read("scripts/circt-external-alloc-check.sh");
    assert!(fr137.contains("FR137") || fr137.contains("compile"));
    assert!(fr164.contains("FR164"));
    assert!(
        fr169.contains("FR169")
            && fr169.contains("--ir-hw")
            && !fr169.contains("fr164_circt_sim_gate"),
        "FR169 must be a distinct multi-lower path, not FR164 sim re-run"
    );
}

#[test]
fn fr169_live_alloc_check_when_firtool_available() {
    let script = root().join("scripts/circt-external-alloc-check.sh");
    let out = Command::new("bash")
        .arg(&script)
        .env_remove("BITLOOM_CIRCT_ALLOC_FORCE_MISSING")
        .current_dir(root())
        .output()
        .expect("spawn live alloc-check");
    assert!(
        out.status.success(),
        "live alloc-check must pass with AD-9 firtool; stdout={} stderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let hw = root().join("target/circt-external-alloc-check/fr169_alloc.ir-hw.mlir");
    assert!(hw.is_file(), "must write HW dialect IR");
    let hw_text = fs::read_to_string(&hw).unwrap();
    assert!(
        hw_text.contains("hw.module") && hw_text.contains("Fr169AllocGate"),
        "HW IR must show dialect allocation markers"
    );
}
