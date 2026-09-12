//! ATDD / guardrail: Story 97.3 / FR165 — Epic 97 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr165_closeout_epic97
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
    let path = workspace_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn fr165_readme_marks_fr165_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR165")
            && (readme.contains("已关闭") || readme.contains("Epic 97"))
            && (readme.contains("fr165")
                || readme.contains("Style")
                || readme.contains("linter")
                || readme.contains("Chisel")),
        "README must mark FR165 / Epic 97 closed"
    );
    assert!(
        readme.contains("FR156")
            && (readme.contains("未关闭前不得宣称")
                || readme.contains("不得宣称")
                || readme.contains("Epic 98")),
        "README must keep FR156 claim honesty open"
    );
    assert!(
        (readme.contains("全清") || readme.contains("NFR59"))
            && (readme.contains("不得") || readme.contains("禁止")),
        "README must forbid NFR59 fully-cleared claim"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr165_deferred_marks_epic97_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 97")
            && deferred.contains("FR165")
            && (deferred.contains("已关闭") || deferred.contains("Story 97.3")),
        "deferred must mark Epic 97 / FR165 closed"
    );
    assert!(
        deferred.contains("FR156") && deferred.contains("Epic 98"),
        "deferred must point remaining claim honesty to Epic 98"
    );
}

#[test]
fn fr165_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 97")
            && agents.contains("FR165")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 97 / FR165 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 97")
            && (spine.contains("已关闭") || spine.contains("97.3"))
            && spine.contains("FR165"),
        "spine must note Epic 97 / FR165 closed"
    );
}

#[test]
fn fr165_nfr14_epic97_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic97-deeper-chisel-parser-ecosystem-fr165.md",
    );
    for needle in [
        "- [x] **FR165 钉死子集实现 + 验收**",
        "- [x] **文档 / deferred / README / spine 收口**",
        "- [x] **NFR68/70/71/72：**",
        "- [x] **品牌 / AD-6：**",
        "- [x] **FR156 / Epic 98：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 97 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 97.3"),
        "NFR14 risk record must be closed after Story 97.3"
    );
}

#[test]
fn fr165_sprint_epic97_done_98_ready() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-97: done") || sprint.contains("epic-97:done"),
        "epic-97 must be done"
    );
    assert!(
        sprint.contains("97-3-fr165-收口与文档指针: done")
            || sprint.contains("97-3-fr165-收口与文档指针:done"),
        "97-3 must be done"
    );
    assert!(
        sprint.contains("98-1-epic-98-nfr14-风险记录: ready-for-dev")
            || sprint.contains("98-1-epic-98-nfr14-风险记录:ready-for-dev")
            || sprint.contains("98-1-epic-98-nfr14-风险记录: done")
            || sprint.contains("98-1-epic-98-nfr14-风险记录: in-progress"),
        "98-1 must be ready-for-dev after Epic 97 closes"
    );
}

#[test]
fn fr165_epics_phase19_epic97_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase19Epic97Status: complete")
            || epics.contains("phase19Epic97Status:complete"),
        "epics.md must stamp phase19Epic97Status complete"
    );
}

#[test]
fn fr165_product_doc_closed_status() {
    let doc = read("docs/fr165-deeper-chisel-parser-ecosystem.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("97.3"),
        "fr165 product doc must note closed status"
    );
}
