//! ATDD: Story 83.2 / FR146 Bitloom 1.0.0 release artifacts.

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
fn fr146_workspace_version_is_1_0_0() {
    let cargo = read("Cargo.toml");
    assert!(
        cargo.contains("version = \"1.0.0\""),
        "workspace.package version must be 1.0.0"
    );
}

#[test]
fn fr146_changelog_has_1_0_0_section() {
    let log = read("CHANGELOG.md");
    assert!(
        log.contains("[1.0.0]") && log.contains("2026-09-11"),
        "CHANGELOG must have [1.0.0] dated section"
    );
    assert!(
        log.contains("public-api-1-0-surface") || log.contains("FR142") || log.contains("Phase 17"),
        "1.0.0 notes should mention stability gate / surface"
    );
}

#[test]
fn fr146_release_checklist_exists() {
    let text = read("docs/fr146-bitloom-1-0-0-release.md");
    assert!(text.contains("FR146"));
    assert!(text.contains("dry-run") || text.contains("Dry-run"));
    assert!(text.contains("v1.0.0"));
    assert!(text.contains("NFR59"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
}

#[test]
fn fr146_release_checklist_requires_tag() {
    let text = read("docs/fr146-bitloom-1-0-0-release.md");
    assert!(
        text.contains("v1.0.0") && (text.contains("git tag") || text.contains("annotated")),
        "checklist must require annotated tag v1.0.0"
    );
}
