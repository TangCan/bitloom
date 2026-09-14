//! ATDD Story 108.2 / FR175 — broader CIRCT SV / ir-verilog beyond FR169.
//!
//! ```text
//! cargo test -p bitloom --test fr175_broader_circt_mlir_sim
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
fn fr175_docs_contract_forbid_fr169_alone() {
    let docs = read("docs/fr175-broader-circt-mlir-sim.md");
    assert!(docs.contains("FR175") && docs.contains("Bitloom"));
    assert!(docs.contains("--ir-sv") && docs.contains("--ir-verilog"));
    assert!(
        docs.contains("FR169")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("仍")),
        "must keep FR169 distinct"
    );
    assert!(docs.contains("1.159.0") || docs.contains("firtool-1.158"));
    assert!(
        docs.contains("BITLOOM_CIRCT_SV_FORCE_MISSING") || docs.contains("FORCE_MISSING"),
        "must document FORCE_MISSING"
    );
}

#[test]
fn fr175_script_just_ci() {
    let script = read("scripts/circt-external-sv-check.sh");
    assert!(script.contains("--ir-sv") && script.contains("--ir-verilog"));
    assert!(script.contains("1.159.0"));
    assert!(script.contains("BITLOOM_CIRCT_SV_FORCE_MISSING"));
    assert!(script.contains("FR175"));
    let just = read("Justfile");
    assert!(just.contains("circt-external-sv-check"));
    let ci = read(".github/workflows/ci.yml");
    assert!(ci.contains("circt-external-sv") && ci.contains("circt-external-sv-check.sh"));
}

#[test]
fn fr175_force_missing_nonzero() {
    let out = Command::new("bash")
        .arg("scripts/circt-external-sv-check.sh")
        .current_dir(root())
        .env("BITLOOM_CIRCT_SV_FORCE_MISSING", "1")
        .output()
        .expect("force-missing");
    assert!(!out.status.success());
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("FORCE_MISSING") || err.contains("refusing") || err.contains("unavailable"),
        "readable stderr; got {err}"
    );
}

#[test]
fn fr175_live_sv_check() {
    let out = Command::new("bash")
        .arg("scripts/circt-external-sv-check.sh")
        .current_dir(root())
        .env_remove("BITLOOM_CIRCT_SV_FORCE_MISSING")
        .output()
        .expect("live sv check");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "live check must pass; stdout={stdout} stderr={stderr}"
    );
}
