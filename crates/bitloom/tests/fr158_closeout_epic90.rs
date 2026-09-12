//! ATDD / guardrail: Story 90.3 / FR158 — Epic 90 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr158_closeout_epic90
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
fn fr158_readme_marks_fr158_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR158")
            && (readme.contains("已关闭") || readme.contains("Epic 90"))
            && (readme.contains("genhtml")
                || readme.contains("--genhtml")
                || readme.contains("fr158")),
        "README must mark FR158 / Epic 90 closed"
    );
    assert!(
        readme.contains("FR159")
            && (readme.contains("未关闭前不得宣称") || readme.contains("不得宣称")),
        "README must keep FR159+ unclaimed"
    );
    assert!(
        (readme.contains("全清") || readme.contains("NFR59"))
            && (readme.contains("不得") || readme.contains("禁止")),
        "README must forbid NFR59 fully-cleared claim"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr158_deferred_marks_epic90_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 90")
            && deferred.contains("FR158")
            && (deferred.contains("已关闭") || deferred.contains("Story 90.3")),
        "deferred must mark Epic 90 / FR158 closed"
    );
    assert!(
        deferred.contains("FR159") && deferred.contains("Epic 91"),
        "deferred must point remaining deepen to Epic 91+"
    );
}

#[test]
fn fr158_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 90")
            && agents.contains("FR158")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 90 / FR158 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 90")
            && (spine.contains("已关闭") || spine.contains("90.3"))
            && spine.contains("FR158"),
        "spine must note Epic 90 / FR158 closed"
    );
}

#[test]
fn fr158_nfr14_epic90_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic90-third-party-lcov-gui-fr158.md",
    );
    for needle in [
        "- [x] **FR158 钉死子集实现 + 验收**",
        "- [x] **文档 / deferred / README 收口**",
        "- [x] **NFR68/71/72：**",
        "- [x] **品牌 / AD-6：**",
        "- [x] **其余 FR159–165：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 90 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 90.3"),
        "NFR14 risk record must be closed after Story 90.3"
    );
}

#[test]
fn fr158_sprint_epic90_done_91_ready() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-90: done") || sprint.contains("epic-90:done"),
        "epic-90 must be done"
    );
    assert!(
        sprint.contains("90-3-fr158-收口与文档指针: done")
            || sprint.contains("90-3-fr158-收口与文档指针:done"),
        "90-3 must be done"
    );
    assert!(
        sprint.contains("91-1-epic-91-nfr14-风险记录: ready-for-dev")
            || sprint.contains("91-1-epic-91-nfr14-风险记录:ready-for-dev")
            || sprint.contains("91-1-epic-91-nfr14-风险记录: done")
            || sprint.contains("91-1-epic-91-nfr14-风险记录: in-progress"),
        "91-1 must be ready-for-dev after Epic 90 closes"
    );
}

#[test]
fn fr158_epics_phase19_epic90_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase19Epic90Status: complete")
            || epics.contains("phase19Epic90Status:complete"),
        "epics.md must stamp phase19Epic90Status complete"
    );
}

#[test]
fn fr158_product_doc_closed_status() {
    let doc = read("docs/fr158-third-party-lcov-gui.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("90.3"),
        "fr158 product doc must note closed status"
    );
}
