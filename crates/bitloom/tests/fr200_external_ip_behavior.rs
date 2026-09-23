//! Story 130.3 RTL behavior and offline replay acceptance checks.

use std::{path::PathBuf, process::Command};

fn inputs() -> (PathBuf, PathBuf, PathBuf) {
    let cache = std::env::var_os("BITLOOM_EXTERNAL_IP_CACHE")
        .map(PathBuf::from)
        .expect("dedicated gate requires BITLOOM_EXTERNAL_IP_CACHE");
    (
        PathBuf::from(std::env::var_os("BITLOOM_EXTERNAL_IP_MANIFEST").expect("explicit manifest")),
        PathBuf::from(std::env::var_os("BITLOOM_EXTERNAL_IP_LOCK").expect("explicit lock")),
        cache,
    )
}

fn command(
    args: &[&str],
    manifest: &std::path::Path,
    lock: &std::path::Path,
    cache: &std::path::Path,
) -> std::process::Output {
    Command::new("timeout")
        .arg("180")
        .arg(env!("CARGO_BIN_EXE_cargo-bitloom"))
        .args(args)
        .arg("--manifest")
        .arg(manifest)
        .arg("--lock")
        .arg(lock)
        .arg("--cache")
        .arg(cache)
        .output()
        .expect("run external ip command")
}

#[test]
#[ignore = "explicit provisioned real-tool gate: just fr200-external-ip-pilot-check"]
fn locked_fifo_matches_independent_model_for_reset_and_boundaries() {
    let (manifest, lock, cache) = inputs();
    let result = command(&["external-ip", "behavior"], &manifest, &lock, &cache);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(String::from_utf8_lossy(&result.stdout).contains("FIFO_MODEL_PASS"));
}

#[test]
#[ignore = "requires bubblewrap network namespace and isolated copied-cache runner"]
fn offline_pilot_replay_consumes_only_copied_lock_and_cache() {
    let (manifest, lock, cache) = inputs();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let evidence = cache.parent().unwrap().join("rust-offline-replay.json");
    let result = Command::new("timeout")
        .args(["660", "python3"])
        .arg(root.join("scripts/phase24-external-ip-replay.py"))
        .args(["replay", "--compile", "--pilot", "--manifest"])
        .arg(manifest)
        .arg("--lock")
        .arg(lock)
        .arg("--cache")
        .arg(cache)
        .arg("--evidence")
        .arg(&evidence)
        .env("BITLOOM_BIN", env!("CARGO_BIN_EXE_cargo-bitloom"))
        .output()
        .expect("run isolated replay script");
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    for (suffix, marker) in [
        ("", "hdl_compile=passed"),
        ("-binding", "binding="),
        ("-behavior", "FIFO_MODEL_PASS"),
        ("-isolation", "ISOLATION_PASS"),
    ] {
        let path = evidence.with_file_name(format!("rust-offline-replay{suffix}.json"));
        let record: serde_json::Value =
            serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert_eq!(record["exitCode"], 0);
        assert!(record["stdout"].as_str().unwrap().contains(marker));
    }
}

#[test]
fn behavior_rejects_invalid_manifest_without_external_tools_or_cache() {
    let dir = std::env::temp_dir().join(format!("fr200-invalid-behavior-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let manifest = dir.join("manifest.json");
    std::fs::write(&manifest, b"{}").unwrap();
    let result = command(
        &["external-ip", "behavior"],
        &manifest,
        &dir.join("absent-lock"),
        &dir.join("absent-cache"),
    );
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("bitloom.external-ip.schema"));
    std::fs::remove_dir_all(dir).unwrap();
}
