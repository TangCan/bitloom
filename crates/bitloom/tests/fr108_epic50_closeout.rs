//! ATDD Story 50.3 — FR108 / Epic 50 closeout + FR98 cross-link honesty.
//!
//! ```text
//! cargo test -p bitloom --test fr108_epic50_closeout
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
fn fr108_nfr14_epic50_close_conditions_checked() {
    let text = read("_agile-output/implementation-artifacts/nfr14-risk-epic50-gpio-near-vip.md");
    for needle in [
        "- [x] **50.2 / FR108：",
        "- [x] **文档 / deferred / FR98 交叉链",
        "- [x] **禁止事项未触发",
        "- [x] **品牌 / 依赖：",
        "- [x] **FR98 MVP 关闭仍有效",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 50 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 50.3") || text.contains("50.3")),
        "NFR14 must be closed with Story 50.3 pointer"
    );
}

#[test]
fn fr108_docs_readme_deferred_closed() {
    let ip = read("docs/ip/README.md");
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        ip.contains("FR108")
            && (ip.contains("已关闭") || ip.contains("50.3") || ip.contains("Epic 50")),
        "docs/ip must declare FR108 / Epic 50 closed"
    );
    assert!(
        (readme.contains("FR108") || readme.contains("GPIO"))
            && (readme.contains("已关闭") || readme.contains("Epic 50")),
        "README must note FR108 / Epic 50 closed"
    );
    assert!(
        deferred.contains("FR108")
            && (deferred.contains("已关闭")
                || deferred.contains("closed")
                || deferred.contains("50.3")),
        "deferred must close GPIO / FR108 item"
    );
    assert!(
        ip.contains("NFR47")
            || ip.contains("非目标")
            || ip.contains("商业 VIP")
            || ip.contains("中断"),
        "docs/ip must keep NFR47 / non-goal honesty"
    );
}

#[test]
fn fr108_fr98_cross_link_and_mvp_still_valid() {
    let ip = read("docs/ip/README.md");
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        (ip.contains("FR98") || ip.contains("Epic 43"))
            && (ip.contains("可选") || ip.contains("G0") || ip.contains("G1")),
        "FR98 cross-link: GPIO was optional at FR98 close"
    );
    assert!(
        ip.contains("FR108") && (ip.contains("Phase 13") || ip.contains("Epic 50")),
        "FR108 is Phase 13 deepen face"
    );
    assert!(
        sprint.contains("epic-43: done") || sprint.contains("43-5-"),
        "FR98 / Epic 43 must remain closed (NFR44)"
    );
}

#[test]
fn fr108_sprint_epic50_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("50-3-fr108-收口与-fr98-交叉链: done")
            || sprint.contains("50-3-fr108-收口与-fr98-交叉链:done")
    );
    assert!(sprint.contains("epic-50: done") || sprint.contains("epic-50:done"));
    assert!(
        sprint.contains("50-2-gpio-近-vip-实现与-atdd-fr108: done")
            || sprint.contains("50-2-gpio-近-vip-实现与-atdd-fr108:done")
    );
}
