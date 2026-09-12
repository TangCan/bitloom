//! ATDD / guardrail: Story 94.3 / FR162 — Epic 94 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr162_closeout_epic94
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
fn fr162_readme_marks_fr162_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR162")
            && (readme.contains("已关闭") || readme.contains("Epic 94"))
            && (readme.contains("fr162")
                || readme.contains("tywaves.gui")
                || readme.contains("GUI")
                || readme.contains("主表面")),
        "README must mark FR162 / Epic 94 closed"
    );
    assert!(
        readme.contains("FR163")
            && (readme.contains("未关闭前不得宣称") || readme.contains("不得宣称")),
        "README must keep FR163+ unclaimed"
    );
    assert!(
        (readme.contains("全清") || readme.contains("NFR59"))
            && (readme.contains("不得") || readme.contains("禁止")),
        "README must forbid NFR59 fully-cleared claim"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr162_deferred_marks_epic94_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 94")
            && deferred.contains("FR162")
            && (deferred.contains("已关闭") || deferred.contains("Story 94.3")),
        "deferred must mark Epic 94 / FR162 closed"
    );
    assert!(
        deferred.contains("FR163") && deferred.contains("Epic 95"),
        "deferred must point remaining deepen to Epic 95+"
    );
}

#[test]
fn fr162_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 94")
            && agents.contains("FR162")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 94 / FR162 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 94")
            && (spine.contains("已关闭") || spine.contains("94.3"))
            && spine.contains("FR162"),
        "spine must note Epic 94 / FR162 closed"
    );
}

#[test]
fn fr162_nfr14_epic94_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic94-deeper-gui-ide-fr162.md");
    for needle in [
        "- [x] **FR162 钉死子集实现 + 验收**",
        "- [x] **文档 / deferred / README 收口**",
        "- [x] **NFR68/70/71/72：**",
        "- [x] **品牌 / AD-6：**",
        "- [x] **其余 FR163–165：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 94 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 94.3"),
        "NFR14 risk record must be closed after Story 94.3"
    );
}

#[test]
fn fr162_sprint_epic94_done_95_ready() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-94: done") || sprint.contains("epic-94:done"),
        "epic-94 must be done"
    );
    assert!(
        sprint.contains("94-3-fr162-收口与文档指针: done")
            || sprint.contains("94-3-fr162-收口与文档指针:done"),
        "94-3 must be done"
    );
    assert!(
        sprint.contains("95-1-epic-95-nfr14-风险记录: ready-for-dev")
            || sprint.contains("95-1-epic-95-nfr14-风险记录:ready-for-dev")
            || sprint.contains("95-1-epic-95-nfr14-风险记录: done")
            || sprint.contains("95-1-epic-95-nfr14-风险记录: in-progress"),
        "95-1 must be ready-for-dev after Epic 94 closes"
    );
}

#[test]
fn fr162_epics_phase19_epic94_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase19Epic94Status: complete")
            || epics.contains("phase19Epic94Status:complete"),
        "epics.md must stamp phase19Epic94Status complete"
    );
}

#[test]
fn fr162_product_doc_closed_status() {
    let doc = read("docs/fr162-deeper-gui-ide-default-wave.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("94.3"),
        "fr162 product doc must note closed status"
    );
}
