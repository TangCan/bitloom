//! ATDD Story 60.3 — FR119 / Epic 60 closeout + FR100/FR112 cross-link honesty.
//!
//! ```text
//! cargo test -p bitloom --test fr119_epic60_closeout
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
fn fr119_nfr14_epic60_close_conditions_checked() {
    let text = read("_agile-output/implementation-artifacts/nfr14-risk-epic60-symbiyosys-smt.md");
    for needle in [
        "- [x] **60.2 / FR119：",
        "- [x] **文档 / deferred / README / FR100·FR112 交叉链",
        "- [x] **禁止事项未触发",
        "- [x] **品牌 / 依赖：",
        "- [x] **FR100 / FR112 分支 B 关闭仍有效",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 60 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 60.3") || text.contains("60.3")),
        "NFR14 must be closed with Story 60.3 pointer"
    );
}

#[test]
fn fr119_docs_readme_deferred_closed() {
    let fr119 = read("docs/fr119-symbiyosys-smt.md");
    let fr100 = read("docs/fr100-formal-equiv.md");
    let fr112 = read("docs/fr112-generated-functional-memread-equiv.md");
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        (fr119.contains("closed") || fr119.contains("已关闭"))
            && (fr119.contains("Epic 60") || fr119.contains("60.3")),
        "fr119 doc must declare Epic 60 / FR119 closed"
    );
    assert!(
        readme.contains("FR119")
            && (readme.contains("Epic 60 已关闭") || readme.contains("FR119 / Epic 60 已关闭")),
        "README must note FR119 / Epic 60 closed"
    );
    assert!(
        deferred.contains("FR119")
            && (deferred.contains("已关闭") || deferred.contains("closed"))
            && (deferred.contains("60.3") || deferred.contains("Epic 60")),
        "deferred must close FR119 item"
    );
    assert!(
        epics.contains("phase14Epic60Status: complete"),
        "epics.md must stamp Epic 60 complete"
    );
    assert!(
        fr100.contains("fr119") || fr100.contains("FR119"),
        "FR100 must cross-link FR119"
    );
    assert!(
        fr112.contains("fr119") || fr112.contains("FR119"),
        "FR112 must cross-link FR119"
    );
    assert!(
        (fr119.contains("分支 C")
            || fr119.contains("handwritten FL")
            || fr119.contains("Branch C"))
            && fr119.contains("deferred"),
        "branch C more IP FL must remain deferred"
    );
    assert!(
        deferred.contains("FR121")
            && (deferred.contains("仍 deferred")
                || deferred.contains("FR121–122")
                || deferred.contains("FR121–FR122")),
        "FR121–122 must remain deferred delivery"
    );
}

#[test]
fn fr119_fr100_fr112_mvp_still_valid() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-45: done") || sprint.contains("epic-45:done"),
        "FR100 / Epic 45 must remain closed (NFR48)"
    );
    assert!(
        sprint.contains("epic-54: done") || sprint.contains("epic-54:done"),
        "FR112 / Epic 54 must remain closed (NFR48)"
    );
    let fr100 = read("docs/fr100-formal-equiv.md");
    let fr112 = read("docs/fr112-generated-functional-memread-equiv.md");
    assert!(
        (fr100.contains("F1-(i)") || fr100.contains("FormalEquivProduct"))
            && fr100.contains("FR100"),
        "FR100 doc must remain the F1-(i) surface"
    );
    assert!(
        (fr112.contains("closed") || fr112.contains("已关闭") || fr112.contains("Epic 54"))
            && (fr112.contains("MemRead") || fr112.contains("GeneratedFunctional")),
        "FR112-B doc must remain closed"
    );
}

#[test]
fn fr119_sprint_epic60_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("60-3-fr119-收口与文档指针: done")
            || sprint.contains("60-3-fr119-收口与文档指针:done")
    );
    assert!(sprint.contains("epic-60: done") || sprint.contains("epic-60:done"));
    assert!(
        sprint.contains("60-2-sby-smt-路径实现与验收-fr119: done")
            || sprint.contains("60-2-sby-smt-路径实现与验收-fr119:done")
    );
    assert!(
        sprint.contains("60-1-epic-60-nfr14-风险记录: done")
            || sprint.contains("60-1-epic-60-nfr14-风险记录:done")
    );
    assert!(
        sprint.contains("epic-62: backlog")
            || sprint.contains("epic-62:backlog")
            || sprint.contains("epic-62: in-progress")
            || sprint.contains("epic-62:in-progress")
            || sprint.contains("epic-62: done")
            || sprint.contains("epic-62:done"),
        "epic-62 must be backlog, in-progress, or done"
    );
    if !sprint.contains("epic-62: backlog") && !sprint.contains("epic-62:backlog") {
        assert!(
            sprint.contains("62-1-epic-62-nfr14-风险记录: done")
                || sprint.contains("62-1-epic-62-nfr14-风险记录:done"),
            "leaving epic-62 backlog requires Story 62.1 NFR14 done (gate)"
        );
    }
    assert!(
        !sprint.contains("epic-63: done") && !sprint.contains("epic-63:done"),
        "epic-63 must not be done yet"
    );
    assert!(
        sprint.contains("epic-63: backlog") || sprint.contains("epic-63:backlog"),
        "epic-63 must remain backlog"
    );
    assert!(
        sprint.contains("epic-61: backlog")
            || sprint.contains("epic-61:backlog")
            || sprint.contains("epic-61: in-progress")
            || sprint.contains("epic-61:in-progress")
            || sprint.contains("epic-61: done")
            || sprint.contains("epic-61:done"),
        "epic-61 must be backlog, in-progress, or done"
    );
}
