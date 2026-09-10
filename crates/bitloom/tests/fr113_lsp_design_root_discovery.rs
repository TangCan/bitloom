//! ATDD: FR113 Cargo-graph + metadata design_roots discovery (Story 55.2).
//!
//! cargo test -p bitloom --test fr113_lsp_design_root_discovery

use std::fs;
use std::path::PathBuf;

use bitloom_lsp::{
    AnalysisMode, DesignFixture, MVP_INTERACTIVE_BUDGET, analyze, analyze_discovered_root,
    analyze_on_did_save_at, analyze_workspace_design_roots, discover_design_roots,
};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn fixture(name: &str) -> PathBuf {
    workspace_root()
        .join("crates/bitloom-lsp/fixtures")
        .join(name)
}

fn read(rel: &str) -> String {
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr113_docs_strategy_and_bans() {
    let text = read("docs/fr113-lsp-design-root-discovery.md");
    assert!(text.contains("FR113") && text.contains("Bitloom"));
    assert!(
        text.contains("design_roots") && text.contains("Cargo"),
        "must document Cargo metadata design_roots"
    );
    assert!(
        text.contains("DesignFixture") && (text.contains("alone") || text.contains("≠")),
        "must ban DesignFixture alone"
    );
    assert!(
        text.contains("deferred") && (text.contains("syn") || text.contains("#[bitloom::top]")),
        "full syn-scan must stay deferred"
    );
    assert!(
        text.contains("FR90") || text.contains("rust-analyzer"),
        "must isolate FR90"
    );
    assert!(text.contains("fr99") || text.contains("FR99"));
}

#[test]
fn fr113_discover_metadata_fixture_not_design_fixture_enum() {
    let roots = discover_design_roots(fixture("fr113_meta_ok")).expect("discover");
    assert_eq!(roots.len(), 1);
    assert_eq!(roots[0].package_name, "fr113_meta_ok");
    assert_eq!(roots[0].root_id, "Fr113OkCounter");
    // Path must not be the DesignFixture type name alone as the discovery source.
    let cargo = fs::read_to_string(fixture("fr113_meta_ok").join("Cargo.toml")).unwrap();
    assert!(cargo.contains("[package.metadata.bitloom]"));
    assert!(
        cargo.contains("design_roots") && cargo.contains("Fr113OkCounter"),
        "fixture must declare metadata design_roots, not DesignFixture-only API"
    );
}

#[test]
fn fr113_analyze_discovered_full_elaborate_pass() {
    let roots = discover_design_roots(fixture("fr113_meta_ok")).unwrap();
    let r = analyze_discovered_root(
        AnalysisMode::FullElaborate,
        &roots[0],
        MVP_INTERACTIVE_BUDGET,
    );
    assert!(r.called_finish, "FR113 must full-elaborate discovered root");
    assert!(r.diagnostics.is_empty(), "{:?}", r.diagnostics);
    assert!(r.symbols.iter().any(|s| s.name == "Fr113OkCounter"));
}

#[test]
fn fr113_shallow_discovered_does_not_finish() {
    let roots = discover_design_roots(fixture("fr113_meta_ok")).unwrap();
    let r = analyze_discovered_root(AnalysisMode::Shallow, &roots[0], MVP_INTERACTIVE_BUDGET);
    assert!(!r.called_finish, "shallow must not close FR113");
}

#[test]
fn fr113_no_metadata_readable_no_design_roots() {
    let r = analyze_workspace_design_roots(
        AnalysisMode::FullElaborate,
        fixture("fr113_meta_none"),
        MVP_INTERACTIVE_BUDGET,
    );
    assert!(!r.called_finish);
    assert!(
        r.diagnostics
            .iter()
            .any(|d| d.code == "bitloom-lsp.no-design-roots"),
        "expected no-design-roots: {:?}",
        r.diagnostics
    );
}

#[test]
fn fr113_fail_root_readable_elaborate_fail() {
    let r = analyze_workspace_design_roots(
        AnalysisMode::FullElaborate,
        fixture("fr113_meta_fail"),
        MVP_INTERACTIVE_BUDGET,
    );
    assert!(r.called_finish);
    assert!(
        !r.diagnostics.is_empty(),
        "fail fixture must surface elaborate diagnostics"
    );
    assert!(
        r.diagnostics
            .iter()
            .any(|d| d.code.contains("E0142") || d.message.contains("E0142")),
        "{:?}",
        r.diagnostics
    );
}

#[test]
fn fr113_unknown_root_id_readable() {
    use bitloom_lsp::DiscoveredDesignRoot;
    let root = DiscoveredDesignRoot {
        package_name: "x".into(),
        package_dir: fixture("fr113_meta_ok"),
        root_id: "NotRegistered".into(),
    };
    let r = analyze_discovered_root(AnalysisMode::FullElaborate, &root, MVP_INTERACTIVE_BUDGET);
    assert!(!r.called_finish);
    assert!(
        r.diagnostics
            .iter()
            .any(|d| d.code == "bitloom-lsp.unknown-design-root")
    );
}

#[test]
fn fr113_did_save_at_prefers_discovery() {
    let hint = fixture("fr113_meta_ok").join("src/lib.rs");
    let r = analyze_on_did_save_at(Some(&hint));
    assert!(r.called_finish);
    assert!(r.symbols.iter().any(|s| s.name == "Fr113OkCounter"));
}

#[test]
fn fr113_fr99_design_fixture_still_green() {
    let r = analyze(
        AnalysisMode::FullElaborate,
        DesignFixture::OkCounter,
        MVP_INTERACTIVE_BUDGET,
    );
    assert!(r.called_finish);
    assert!(r.symbols.iter().any(|s| s.name == "Fr99OkCounter"));
}
