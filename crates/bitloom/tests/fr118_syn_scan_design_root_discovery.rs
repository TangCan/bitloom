//! ATDD: FR118 workspace #[bitloom::top] syn-scan discovery (Story 59.2).
//!
//! cargo test -p bitloom --test fr118_syn_scan_design_root_discovery

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
fn fr118_docs_strategy_and_bans() {
    let text = read("docs/fr118-syn-scan-design-root-discovery.md");
    assert!(text.contains("FR118") && text.contains("Bitloom"));
    assert!(
        text.contains("#[bitloom::top]") && (text.contains("syn-scan") || text.contains("syn")),
        "must document syn-scan of #[bitloom::top]"
    );
    assert!(
        text.contains("DesignFixture") && (text.contains("alone") || text.contains("≠")),
        "must ban DesignFixture alone"
    );
    assert!(
        text.contains("design_roots")
            && (text.contains("alone") || text.contains("≠") || text.contains("FR113")),
        "must ban metadata alone / isolate FR113"
    );
    assert!(
        (text.contains("shallow") || text.contains("Shallow"))
            && (text.contains("finish") || text.contains("not")),
        "must ban shallow finish"
    );
    assert!(text.contains("FR99") || text.contains("fr99"));
    assert!(text.contains("FR113") || text.contains("fr113"));
}

#[test]
fn fr118_discover_no_metadata_syn_scan_positive() {
    let cargo = fs::read_to_string(fixture("fr118_syn_ok").join("Cargo.toml")).unwrap();
    assert!(
        !cargo.contains("design_roots"),
        "positive fixture must have no metadata design_roots"
    );
    let roots = discover_design_roots(fixture("fr118_syn_ok")).expect("discover");
    assert_eq!(roots.len(), 1);
    assert_eq!(roots[0].package_name, "fr118_syn_ok");
    assert_eq!(roots[0].root_id, "Fr118OkCounter");
    let src = fs::read_to_string(fixture("fr118_syn_ok").join("src/lib.rs")).unwrap();
    assert!(
        src.contains("#[bitloom::top]") && src.contains("Fr118OkCounter"),
        "fixture must expose #[bitloom::top] for syn-scan"
    );
}

#[test]
fn fr118_analyze_syn_scan_full_elaborate_pass() {
    let roots = discover_design_roots(fixture("fr118_syn_ok")).unwrap();
    let r = analyze_discovered_root(
        AnalysisMode::FullElaborate,
        &roots[0],
        MVP_INTERACTIVE_BUDGET,
    );
    assert!(
        r.called_finish,
        "FR118 must full-elaborate syn-scanned root"
    );
    assert!(r.diagnostics.is_empty(), "{:?}", r.diagnostics);
    assert!(r.symbols.iter().any(|s| s.name == "Fr118OkCounter"));
}

#[test]
fn fr118_shallow_syn_scan_does_not_finish() {
    let roots = discover_design_roots(fixture("fr118_syn_ok")).unwrap();
    let r = analyze_discovered_root(AnalysisMode::Shallow, &roots[0], MVP_INTERACTIVE_BUDGET);
    assert!(!r.called_finish, "shallow must not close FR118");
}

#[test]
fn fr118_fail_root_readable_elaborate_fail() {
    let r = analyze_workspace_design_roots(
        AnalysisMode::FullElaborate,
        fixture("fr118_syn_fail"),
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
fn fr118_fr113_metadata_still_works() {
    let roots = discover_design_roots(fixture("fr113_meta_ok")).expect("discover");
    assert_eq!(roots[0].root_id, "Fr113OkCounter");
    let r = analyze_discovered_root(
        AnalysisMode::FullElaborate,
        &roots[0],
        MVP_INTERACTIVE_BUDGET,
    );
    assert!(r.called_finish);
    assert!(r.diagnostics.is_empty());
}

#[test]
fn fr118_fr99_design_fixture_still_green() {
    let r = analyze(
        AnalysisMode::FullElaborate,
        DesignFixture::OkCounter,
        MVP_INTERACTIVE_BUDGET,
    );
    assert!(r.called_finish);
    assert!(r.symbols.iter().any(|s| s.name == "Fr99OkCounter"));
}

#[test]
fn fr118_did_save_at_prefers_syn_scan() {
    let hint = fixture("fr118_syn_ok").join("src/lib.rs");
    let r = analyze_on_did_save_at(Some(&hint));
    assert!(r.called_finish);
    assert!(r.symbols.iter().any(|s| s.name == "Fr118OkCounter"));
}

#[test]
fn fr118_nfr14_gate_selects_syn_scan() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic59-syn-scan-design-root-discovery.md",
    );
    assert!(
        text.contains("#[bitloom::top]") && (text.contains("syn-scan") || text.contains("syn")),
        "NFR14 must keep syn-scan selected"
    );
    assert!(
        text.contains("DesignFixture")
            && (text.contains("不得") || text.contains("alone") || text.contains("≠"))
    );
}
