//! ATDD: HLS CI / release smoke fixture exists and documents backend pin (Story 24.3).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn hls_smoke_script_documents_backend_and_cache() {
    let root = workspace_root();
    let script = root.join("scripts/hls-smoke.sh");
    let stub = root.join("scripts/fixtures/bambu-ci-stub.sh");
    let ci = root.join(".github/workflows/ci.yml");

    assert!(script.is_file(), "missing {}", script.display());
    assert!(stub.is_file(), "missing {}", stub.display());
    assert!(ci.is_file(), "missing {}", ci.display());

    let smoke = fs::read_to_string(&script).unwrap();
    assert!(smoke.contains("2024.10"), "smoke must pin Bambu 2024.10");
    assert!(
        smoke.contains("BITLOOM_HLS_CACHE") || smoke.contains("cache"),
        "smoke must document cache strategy"
    );
    assert!(
        smoke.contains("set -e") || smoke.contains("set -euo"),
        "smoke must fail on error (no ignore)"
    );

    let workflow = fs::read_to_string(&ci).unwrap();
    assert!(
        workflow.contains("hls-smoke"),
        "ci.yml must define hls-smoke job"
    );
    assert!(
        !workflow.contains("continue-on-error: true")
            || !workflow
                .split("hls-smoke:")
                .nth(1)
                .unwrap_or("")
                .contains("continue-on-error: true"),
        "hls-smoke job must not ignore failures"
    );
}

#[test]
fn hls_smoke_script_produces_rtl_with_ci_stub() {
    let root = workspace_root();
    let status = Command::new("bash")
        .arg(root.join("scripts/hls-smoke.sh"))
        .current_dir(&root)
        .env_remove("BITLOOM_HLS_USE_REAL")
        .status()
        .expect("spawn hls-smoke.sh");
    assert!(status.success(), "hls-smoke.sh must pass with CI stub");
    let out = root.join("target/hls-smoke");
    let has_v = fs::read_dir(&out)
        .expect("hls-smoke out dir")
        .flatten()
        .any(|e| {
            e.path()
                .extension()
                .and_then(|x| x.to_str())
                .is_some_and(|ext| matches!(ext, "v" | "sv"))
        });
    assert!(has_v, "expected .v/.sv under {}", out.display());
}
