//! ATDD Story 53.3 — FR111 / Epic 53 closeout + AD-27 cross-link honesty.
//!
//! ```text
//! cargo test -p bitloom --test fr111_epic53_closeout
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
fn fr111_nfr14_epic53_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic53-idiomatic-chisel-depth.md");
    for needle in [
        "- [x] **53.2 / FR111：",
        "- [x] **文档 / deferred / AD-27 交叉链",
        "- [x] **禁止事项未触发",
        "- [x] **品牌 / 依赖：",
        "- [x] **FR97 MVP 关闭仍有效",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 53 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 53.3") || text.contains("53.3")),
        "NFR14 must be closed with Story 53.3 pointer"
    );
}

#[test]
fn fr111_docs_readme_deferred_ad27_closed() {
    let fr111 = read("docs/fr111-idiomatic-chisel-depth.md");
    let fr97 = read("docs/fr97-idiomatic-chisel.md");
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        fr111.contains("closed")
            || fr111.contains("已关闭")
            || fr111.contains("53.3")
            || fr111.contains("Epic 53"),
        "fr111 doc must declare Epic 53 closed"
    );
    assert!(
        (readme.contains("FR111") || readme.contains("可维护深度"))
            && (readme.contains("已关闭") || readme.contains("Epic 53")),
        "README must note FR111 / Epic 53 closed"
    );
    assert!(
        deferred.contains("FR111")
            && (deferred.contains("已关闭")
                || deferred.contains("closed")
                || deferred.contains("53.3")),
        "deferred must close FR111 item"
    );
    assert!(
        spine.contains("FR111") && spine.contains("AD-27"),
        "AD-27 cross-link must mention FR111"
    );
    assert!(
        fr97.contains("FR111") || fr111.contains("FR97"),
        "MVP vs depth distinction retained"
    );
    assert!(
        fr111.contains("NFR47")
            || fr111.contains("Non-goals")
            || fr111.contains("Parser")
            || fr111.contains("全家桶"),
        "NFR47 honesty retained"
    );
}

#[test]
fn fr111_fr97_mvp_still_valid() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-42: done") || sprint.contains("42-3-"),
        "FR97 / Epic 42 must remain closed (NFR44)"
    );
}

#[test]
fn fr111_sprint_epic53_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("53-3-fr111-收口与-ad-27-交叉链: done")
            || sprint.contains("53-3-fr111-收口与-ad-27-交叉链:done")
    );
    assert!(sprint.contains("epic-53: done") || sprint.contains("epic-53:done"));
    assert!(
        sprint.contains("53-2-idiomatic-加深发射-检查与-atdd-fr111: done")
            || sprint.contains("53-2-idiomatic-加深发射-检查与-atdd-fr111:done")
            || sprint.contains("53-2-idiomatic-chisel-可维护深度-fr111: done")
            || sprint.contains("53-2-idiomatic-chisel-可维护深度-fr111:done")
    );
}
