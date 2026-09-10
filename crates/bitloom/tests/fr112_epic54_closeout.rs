//! ATDD Story 54.3 — FR112 / Epic 54 closeout + FR100/FR103 cross-link honesty.
//!
//! ```text
//! cargo test -p bitloom --test fr112_epic54_closeout
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
fn fr112_nfr14_epic54_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic54-formal-dual-model-depth.md");
    for needle in [
        "- [x] **54.2 / FR112：",
        "- [x] **文档 / deferred / FR100·FR103 交叉链",
        "- [x] **禁止事项未触发",
        "- [x] **品牌 / 依赖：",
        "- [x] **FR100/103 MVP 关闭仍有效",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 54 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 54.3") || text.contains("54.3")),
        "NFR14 must be closed with Story 54.3 pointer"
    );
}

#[test]
fn fr112_docs_readme_deferred_closed() {
    let fr112 = read("docs/fr112-generated-functional-memread-equiv.md");
    let fr100 = read("docs/fr100-formal-equiv.md");
    let fr103 = read("docs/fr103-ip-dual-model.md");
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        fr112.contains("closed")
            || fr112.contains("已关闭")
            || fr112.contains("54.3")
            || fr112.contains("Epic 54"),
        "fr112 doc must declare Epic 54 closed"
    );
    assert!(
        (readme.contains("FR112") || readme.contains("双模型深度"))
            && (readme.contains("已关闭") || readme.contains("Epic 54")),
        "README must note FR112 / Epic 54 closed"
    );
    assert!(
        deferred.contains("FR112")
            && (deferred.contains("已关闭")
                || deferred.contains("closed")
                || deferred.contains("54.3")),
        "deferred must close FR112 item"
    );
    assert!(
        fr100.contains("fr112") || fr100.contains("FR112"),
        "FR100 must cross-link FR112"
    );
    assert!(
        fr103.contains("fr112") || fr103.contains("FR112"),
        "FR103 must cross-link FR112"
    );
    assert!(
        fr112.contains("deferred")
            && (fr112.contains("SymbiYosys") || fr112.contains("F1-(ii)"))
            && (fr112.contains("handwritten") || fr112.contains("(C)")),
        "NFR47: A/C remain deferred"
    );
}

#[test]
fn fr112_fr100_fr103_mvp_still_valid() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-45: done") || sprint.contains("45-4-"),
        "FR100/103 / Epic 45 must remain closed (NFR44)"
    );
}

#[test]
fn fr112_sprint_epic54_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("54-3-fr112-收口与交叉链: done")
            || sprint.contains("54-3-fr112-收口与交叉链:done")
    );
    assert!(sprint.contains("epic-54: done") || sprint.contains("epic-54:done"));
    assert!(
        sprint.contains("54-2-形式等价-双模型深度-fr112: done")
            || sprint.contains("54-2-形式等价-双模型深度-fr112:done")
    );
}
