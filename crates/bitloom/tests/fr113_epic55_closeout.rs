//! ATDD Story 55.3 — FR113 / Epic 55 closeout + FR99 cross-link honesty.
//!
//! ```text
//! cargo test -p bitloom --test fr113_epic55_closeout
//! ```

use std::fs;
use std::path::PathBuf;

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
fn fr113_nfr14_epic55_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic55-lsp-design-root-discovery.md",
    );
    for needle in [
        "- [x] **55.2 / FR113：",
        "- [x] **文档 / deferred / FR99 交叉链",
        "- [x] **禁止事项未触发",
        "- [x] **品牌 / 依赖：",
        "- [x] **FR99 MVP 关闭仍有效",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 55 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 55.3") || text.contains("55.3")),
        "NFR14 must be closed with Story 55.3 pointer"
    );
}

#[test]
fn fr113_docs_readme_deferred_closed() {
    let fr113 = read("docs/fr113-lsp-design-root-discovery.md");
    let fr99 = read("docs/fr99-bitloom-lsp.md");
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        fr113.contains("closed")
            || fr113.contains("已关闭")
            || fr113.contains("55.3")
            || fr113.contains("Epic 55"),
        "fr113 doc must declare Epic 55 closed"
    );
    assert!(
        (readme.contains("FR113") || readme.contains("设计根"))
            && (readme.contains("已关闭") || readme.contains("Epic 55")),
        "README must note FR113 / Epic 55 closed"
    );
    assert!(
        deferred.contains("FR113")
            && (deferred.contains("已关闭")
                || deferred.contains("closed")
                || deferred.contains("55.3")),
        "deferred must close FR113 item"
    );
    assert!(
        fr99.contains("fr113") || fr99.contains("FR113"),
        "FR99 must cross-link FR113"
    );
    assert!(
        fr113.contains("deferred") && (fr113.contains("syn") || fr113.contains("#[bitloom::top]")),
        "NFR47: syn-scan remains deferred"
    );
}

#[test]
fn fr113_fr99_mvp_still_valid() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-44: done") || sprint.contains("44-4-"),
        "FR99 / Epic 44 must remain closed (NFR44)"
    );
}

#[test]
fn fr113_sprint_epic55_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("55-3-fr113-收口与-fr99-交叉链: done")
            || sprint.contains("55-3-fr113-收口与-fr99-交叉链:done")
    );
    assert!(sprint.contains("epic-55: done") || sprint.contains("epic-55:done"));
    assert!(
        sprint.contains("55-2-设计根发现实现与-atdd-fr113: done")
            || sprint.contains("55-2-设计根发现实现与-atdd-fr113:done")
            || sprint.contains("55-2-lsp-设计根发现加深-fr113: done")
            || sprint.contains("55-2-lsp-设计根发现加深-fr113:done")
    );
}
