//! ATDD Story 52.3 — FR110 / Epic 52 closeout + AD-25 cross-link honesty.
//!
//! ```text
//! cargo test -p bitloom --test fr110_epic52_closeout
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
fn fr110_nfr14_epic52_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic52-hls-commercial-depth.md");
    for needle in [
        "- [x] **52.2 / FR110：",
        "- [x] **文档 / deferred / AD-25 交叉链",
        "- [x] **禁止事项未触发",
        "- [x] **品牌 / 依赖：",
        "- [x] **FR95/96 MVP 关闭仍有效",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 52 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 52.3") || text.contains("52.3")),
        "NFR14 must be closed with Story 52.3 pointer"
    );
}

#[test]
fn fr110_docs_readme_deferred_ad25_closed() {
    let fr110 = read("docs/fr110-hls-commercial-depth.md");
    let fr35 = read("docs/fr35-hls.md");
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        fr110.contains("closed")
            || fr110.contains("已关闭")
            || fr110.contains("52.3")
            || fr110.contains("Epic 52"),
        "fr110 doc must declare Epic 52 closed"
    );
    assert!(
        (readme.contains("FR110") || readme.contains("商业深度"))
            && (readme.contains("已关闭") || readme.contains("Epic 52")),
        "README must note FR110 / Epic 52 closed"
    );
    assert!(
        deferred.contains("FR110")
            && (deferred.contains("已关闭")
                || deferred.contains("closed")
                || deferred.contains("52.3")),
        "deferred must close FR110 item"
    );
    assert!(
        spine.contains("FR110") && spine.contains("AD-25"),
        "AD-25 cross-link must mention FR110"
    );
    assert!(
        fr35.contains("FR110") || fr110.contains("FR95"),
        "MVP vs depth distinction retained"
    );
    assert!(
        fr110.contains("NFR47")
            || fr110.contains("Non-goals")
            || fr110.contains("全家桶")
            || fr110.contains("Handshake"),
        "NFR47 honesty retained"
    );
}

#[test]
fn fr110_fr95_mvp_still_valid() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-41: done") || sprint.contains("41-4-"),
        "FR95/96 / Epic 41 must remain closed (NFR44)"
    );
}

#[test]
fn fr110_sprint_epic52_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("52-3-fr110-收口与-ad-25-交叉链: done")
            || sprint.contains("52-3-fr110-收口与-ad-25-交叉链:done")
    );
    assert!(sprint.contains("epic-52: done") || sprint.contains("epic-52:done"));
    assert!(
        sprint.contains("52-2-树内-hls-商业深度-fr110: done")
            || sprint.contains("52-2-树内-hls-商业深度-fr110:done")
    );
}
