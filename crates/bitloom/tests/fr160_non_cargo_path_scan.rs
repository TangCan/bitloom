//! ATDD / docs guardrail: FR160 product page (Story 92.2).
//! Functional discover tests live in `bitloom-lsp` (bitloom must not depend on bitloom-lsp).
//!
//! cargo test -p bitloom --test fr160_non_cargo_path_scan

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
fn fr160_docs_and_fixture_exist() {
    let text = read("docs/fr160-non-cargo-path-scan.md");
    assert!(text.contains("FR160") && text.contains("Bitloom"));
    assert!(text.contains("discover_design_roots_under"));
    assert!(
        text.contains("FR118") && (text.contains("alone") || text.contains("≠")),
        "must ban FR118 alone"
    );
    let fixture = workspace_root().join("crates/bitloom-lsp/fixtures/fr160_non_cargo");
    assert!(!fixture.join("Cargo.toml").is_file());
    assert!(fixture.join("design/top.rs").is_file());
    let src = read("crates/bitloom-lsp/fixtures/fr160_non_cargo/design/top.rs");
    assert!(src.contains("#[bitloom::top]") && src.contains("Fr160BareTop"));
    let fr118 = read("docs/fr118-syn-scan-design-root-discovery.md");
    assert!(fr118.contains("FR160") || fr118.contains("fr160"));
}

#[test]
fn fr160_bitloom_lsp_integration_tests_pass() {
    let status = Command::new("cargo")
        .arg("+1.97.1")
        .args([
            "test",
            "-p",
            "bitloom-lsp",
            "--test",
            "fr160_non_cargo_path_scan",
            "--quiet",
        ])
        .current_dir(workspace_root())
        .status()
        .expect("spawn bitloom-lsp FR160 tests");
    assert!(
        status.success(),
        "bitloom-lsp FR160 ATDD must pass (bitloom must not path-dep bitloom-lsp)"
    );
}
