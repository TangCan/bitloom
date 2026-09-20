//! ATDD: Bitloom 1.1.0 minor release artifacts.

use std::fs;
use std::path::PathBuf;

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
fn current_workspace_version_retains_1_1_release_history() {
    let output = std::process::Command::new("cargo")
        .args([
            "metadata",
            "--no-deps",
            "--offline",
            "--format-version",
            "1",
        ])
        .current_dir(workspace_root())
        .output()
        .expect("cargo metadata");
    assert!(output.status.success());
    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let cli = metadata["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["name"] == "bitloom")
        .unwrap();
    assert_eq!(cli["version"], env!("CARGO_PKG_VERSION"));
    let version: Vec<u64> = cli["version"]
        .as_str()
        .unwrap()
        .split('.')
        .take(2)
        .map(|v| v.parse().unwrap())
        .collect();
    assert!(version.as_slice() >= [1, 1].as_slice());
    assert!(read("CHANGELOG.md").contains("[1.1.0]"));
}

#[test]
fn bitloom_1_1_0_changelog_section() {
    let log = read("CHANGELOG.md");
    assert!(
        log.contains("[1.1.0]") && log.contains("2026-09-14"),
        "CHANGELOG must have [1.1.0] dated section"
    );
    assert!(
        log.contains("FR183") || log.contains("FR190") || log.contains("minor"),
        "1.1.0 notes should mention additive / FR183/FR190 / minor"
    );
    assert!(
        log.contains("FR189") && (log.contains("undelivered") || log.contains("deferred")),
        "1.1.0 must honestly note FR189 undelivered"
    );
}

#[test]
fn bitloom_1_1_0_release_checklist() {
    let text = read("docs/bitloom-1-1-0-release.md");
    assert!(text.contains("1.1.0"));
    assert!(text.contains("v1.1.0"));
    assert!(text.contains("dry-run") || text.contains("Dry-run") || text.contains("publish"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(text.contains("FR189"));
}
