//! ATDD Story 65.3 — FR125 / Epic 65 closeout.

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
fn fr125_nfr14_close_conditions_checked() {
    let text = read("_agile-output/implementation-artifacts/nfr14-risk-epic65-upstream-tywaves.md");
    for needle in [
        "- [x] **65.2 / FR125：",
        "- [x] **文档 / deferred / README",
        "- [x] **禁止事项未触发",
        "- [x] **品牌 / 依赖：",
        "- [x] **FR117 关闭仍有效",
    ] {
        assert!(text.contains(needle), "missing: {needle}");
    }
    assert!(text.contains("closed") && text.contains("65.3"));
}

#[test]
fn fr125_docs_readme_deferred_closed() {
    let fr125 = read("docs/fr125-upstream-tywaves.md");
    let fr117 = read("docs/fr117-typed-ide-wave.md");
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        (fr125.contains("closed") || fr125.contains("已关闭"))
            && (fr125.contains("Epic 65") || fr125.contains("65.3"))
    );
    assert!(fr117.contains("FR125") || fr117.contains("fr125"));
    assert!(
        readme.contains("FR125") && (readme.contains("Epic 65 已关闭") || readme.contains("65.3"))
    );
    assert!(
        deferred.contains("FR125") && (deferred.contains("已关闭") || deferred.contains("Epic 65"))
    );
}

#[test]
fn fr125_sprint_epic65_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("65-3-fr125-收口与文档指针: done")
            || sprint.contains("65-3-fr125-收口与文档指针:done")
    );
    assert!(sprint.contains("epic-65: done") || sprint.contains("epic-65:done"));
}

#[test]
fn fr125_fr117_still_valid() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(sprint.contains("epic-58: done") || sprint.contains("epic-58:done"));
}
