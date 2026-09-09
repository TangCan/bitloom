//! ATDD (red→green): Story 44.3 / FR99 — keystroke (didSave) full-design elaborate
//! diagnostics + symbols on Bitloom LSP, distinguishable from shallow/non-elaborate.
//!
//! ```text
//! cargo test -p bitloom --test fr99_bitloom_lsp_full_elaborate
//! ```

use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use bitloom_lsp::{AnalysisMode, DesignFixture, MVP_INTERACTIVE_BUDGET, MVP_MAX_MODULES, analyze};

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
fn fr99_full_vs_shallow_elaborate_contract() {
    let full = analyze(
        AnalysisMode::FullElaborate,
        DesignFixture::FailHwCapture,
        MVP_INTERACTIVE_BUDGET,
    );
    let shallow = analyze(
        AnalysisMode::Shallow,
        DesignFixture::FailHwCapture,
        MVP_INTERACTIVE_BUDGET,
    );

    assert!(
        full.called_finish,
        "full path must call ElaborateSession::finish()"
    );
    assert!(
        !shallow.called_finish,
        "shallow path must not call finish()"
    );
    assert!(
        full.diagnostics
            .iter()
            .any(|d| d.code == "rhdl::E0142" || d.message.contains("E0142")),
        "full elaborate must surface hardware-semantic E0142; got {:?}",
        full.diagnostics
    );
    assert!(
        shallow.diagnostics.is_empty(),
        "shallow must not invent elaborate diagnostics; got {:?}",
        shallow.diagnostics
    );
}

#[test]
fn fr99_full_elaborate_diagnostics_and_symbols() {
    let ok = analyze(
        AnalysisMode::FullElaborate,
        DesignFixture::OkCounter,
        MVP_INTERACTIVE_BUDGET,
    );
    assert!(ok.called_finish);
    assert!(
        ok.diagnostics.is_empty(),
        "ok fixture should elaborate clean"
    );
    assert!(
        ok.symbols.iter().any(|s| s.kind == "module"),
        "must expose module symbols: {:?}",
        ok.symbols
    );
    assert!(
        ok.symbols.iter().any(|s| s.kind == "port"),
        "must expose port symbols: {:?}",
        ok.symbols
    );

    let fail = analyze(
        AnalysisMode::FullElaborate,
        DesignFixture::FailHwCapture,
        MVP_INTERACTIVE_BUDGET,
    );
    assert!(fail.called_finish);
    assert!(
        fail.diagnostics
            .iter()
            .all(|d| !d.message.trim().is_empty()),
        "failure diagnostics must be readable"
    );
    assert!(
        fail.diagnostics
            .iter()
            .any(|d| d.message.contains("rhdl::E0142") || d.code == "rhdl::E0142"),
        "readable diagnostic should include code"
    );
}

#[test]
fn fr99_full_elaborate_p3_p4_docs_and_behavior() {
    let doc = read("docs/fr99-bitloom-lsp.md");
    assert!(
        doc.contains("P3") || doc.contains("2s") || doc.contains("interactive"),
        "docs must document P3 interactive budget / timeout"
    );
    assert!(
        doc.contains("P4")
            || doc.contains("MVP_MAX_MODULES")
            || doc.contains(&MVP_MAX_MODULES.to_string())
            || doc.contains("oversized")
            || doc.contains("规模"),
        "docs must document P4 oversized / scale ceiling"
    );
    assert!(
        doc.contains("timeout") || doc.contains("超时") || doc.contains("bitloom-lsp.timeout"),
        "docs must name timeout behavior"
    );

    let over = analyze(
        AnalysisMode::FullElaborate,
        DesignFixture::Oversized,
        MVP_INTERACTIVE_BUDGET,
    );
    assert!(over.oversized, "oversized fixture must set oversized flag");
    assert!(
        !over.called_finish,
        "must not pretend finish succeeded on oversized"
    );
    assert!(
        over.diagnostics
            .iter()
            .any(|d| d.code.contains("oversized") || d.message.contains("oversized")),
        "oversized must yield readable diagnostic: {:?}",
        over.diagnostics
    );

    let timed = analyze(
        AnalysisMode::FullElaborate,
        DesignFixture::SlowForTimeout,
        Duration::from_millis(30),
    );
    assert!(timed.timed_out, "slow fixture must time out");
    assert!(
        timed
            .diagnostics
            .iter()
            .any(|d| d.code.contains("timeout") || d.message.contains("timeout")),
        "timeout must yield readable diagnostic: {:?}",
        timed.diagnostics
    );
}

#[test]
fn fr99_full_elaborate_trigger_docs() {
    let doc = read("docs/fr99-bitloom-lsp.md");
    assert!(
        doc.contains("didSave")
            || doc.contains("textDocument/didSave")
            || doc.contains("didChange"),
        "P1: docs must name edit trigger (didSave or didChange)"
    );
    assert!(
        doc.contains("full") && (doc.contains("elaborate") || doc.contains("Elaborate")),
        "docs must describe full-design elaborate"
    );
    assert!(
        doc.contains("shallow")
            || doc.contains("浅层")
            || doc.contains("non-elaborate")
            || doc.contains("AnalysisMode"),
        "docs must contrast shallow / non-elaborate path"
    );
    assert!(
        doc.contains("documentSymbol")
            || doc.contains("document symbol")
            || doc.contains("goto")
            || doc.contains("符号"),
        "docs must mention symbols / goto capability"
    );
}

#[test]
fn fr99_full_elaborate_scope_guards() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("44-3-按键全-elaborate-诊断-符号-fr99: done"),
        "sprint must mark 44-3 done"
    );
    assert!(
        sprint.contains("44-4-fr99-收口与撤销-lsp-非目标: backlog"),
        "44-4 must remain backlog"
    );
    assert!(
        sprint.contains("epic-44: in-progress"),
        "epic-44 must stay in-progress (no FR99 closeout)"
    );

    let nfr14 =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic44-full-elaborate-lsp.md");
    // 44.4 owns full epic close — do not require all boxes ticked.
    assert!(
        nfr14.contains("- [ ] **文档")
            || nfr14.contains("- [ ] **禁止")
            || nfr14.contains("Story 44.4"),
        "NFR14 epic close / Path B revocation must remain for 44.4"
    );
}
