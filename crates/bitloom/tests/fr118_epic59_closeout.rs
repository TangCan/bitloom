//! ATDD Story 59.3 — FR118 / Epic 59 closeout + FR99/FR113 cross-link honesty.
//!
//! ```text
//! cargo test -p bitloom --test fr118_epic59_closeout
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
fn fr118_nfr14_epic59_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic59-syn-scan-design-root-discovery.md",
    );
    for needle in [
        "- [x] **59.2 / FR118：",
        "- [x] **文档 / deferred / README / FR99·FR113 交叉链",
        "- [x] **禁止事项未触发",
        "- [x] **品牌 / 依赖：",
        "- [x] **FR99 MVP 与 FR113 关闭仍有效",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 59 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 59.3") || text.contains("59.3")),
        "NFR14 must be closed with Story 59.3 pointer"
    );
}

#[test]
fn fr118_docs_readme_deferred_closed() {
    let fr118 = read("docs/fr118-syn-scan-design-root-discovery.md");
    let fr113 = read("docs/fr113-lsp-design-root-discovery.md");
    let fr99 = read("docs/fr99-bitloom-lsp.md");
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        (fr118.contains("closed") || fr118.contains("已关闭"))
            && (fr118.contains("Epic 59") || fr118.contains("59.3")),
        "fr118 doc must declare Epic 59 / FR118 closed"
    );
    assert!(
        readme.contains("FR118")
            && (readme.contains("Epic 59 已关闭") || readme.contains("FR118 / Epic 59 已关闭")),
        "README must note FR118 / Epic 59 closed"
    );
    assert!(
        deferred.contains("FR118")
            && (deferred.contains("已关闭") || deferred.contains("closed"))
            && (deferred.contains("59.3") || deferred.contains("Epic 59")),
        "deferred must close FR118 item"
    );
    assert!(
        epics.contains("phase14Epic59Status: complete"),
        "epics.md must stamp Epic 59 complete"
    );
    assert!(
        fr113.contains("fr118") || fr113.contains("FR118"),
        "FR113 must cross-link FR118"
    );
    assert!(
        fr99.contains("fr118") || fr99.contains("FR118"),
        "FR99 must cross-link FR118"
    );
    assert!(
        deferred.contains("FR120")
            && (deferred.contains("仍 deferred")
                || deferred.contains("FR120–122")
                || deferred.contains("FR120–FR122")),
        "FR120–122 must remain deferred delivery"
    );
}

#[test]
fn fr118_fr99_fr113_mvp_still_valid() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-44: done") || sprint.contains("epic-44:done"),
        "FR99 / Epic 44 must remain closed (NFR48)"
    );
    assert!(
        sprint.contains("epic-55: done") || sprint.contains("epic-55:done"),
        "FR113 / Epic 55 must remain closed (NFR48)"
    );
    let fr99 = read("docs/fr99-bitloom-lsp.md");
    let fr113 = read("docs/fr113-lsp-design-root-discovery.md");
    assert!(
        (fr99.contains("closed") || fr99.contains("已关闭") || fr99.contains("Epic 44"))
            && fr99.contains("FR99"),
        "FR99 doc must remain closed"
    );
    assert!(
        (fr113.contains("closed") || fr113.contains("已关闭") || fr113.contains("Epic 55"))
            && fr113.contains("FR113"),
        "FR113 doc must remain closed"
    );
}

#[test]
fn fr118_sprint_epic59_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("59-3-fr118-收口与文档指针: done")
            || sprint.contains("59-3-fr118-收口与文档指针:done")
    );
    assert!(sprint.contains("epic-59: done") || sprint.contains("epic-59:done"));
    assert!(
        sprint.contains("59-2-syn-scan-发现路径实现与验收-fr118: done")
            || sprint.contains("59-2-syn-scan-发现路径实现与验收-fr118:done")
    );
    // Epic 61–63 must not be prematurely closed; Epic 60 may be done after 60.3.
    for epic in 61..=63 {
        assert!(
            !sprint.contains(&format!("epic-{epic}: done"))
                && !sprint.contains(&format!("epic-{epic}:done")),
            "epic-{epic} must not be done yet"
        );
        assert!(
            sprint.contains(&format!("epic-{epic}: backlog"))
                || sprint.contains(&format!("epic-{epic}:backlog")),
            "epic-{epic} must remain backlog"
        );
    }
    assert!(
        sprint.contains("epic-60: backlog")
            || sprint.contains("epic-60:backlog")
            || sprint.contains("epic-60: in-progress")
            || sprint.contains("epic-60:in-progress")
            || sprint.contains("epic-60: done")
            || sprint.contains("epic-60:done"),
        "epic-60 must be backlog, in-progress, or done"
    );
}
