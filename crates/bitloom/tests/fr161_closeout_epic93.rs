//! ATDD / guardrail: Story 93.3 / FR161 — Epic 93 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr161_closeout_epic93
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
fn fr161_readme_marks_fr161_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR161")
            && (readme.contains("已关闭") || readme.contains("Epic 93"))
            && (readme.contains("ci-sby-pins")
                || readme.contains("hygiene")
                || readme.contains("formal-sby")
                || readme.contains("fr161")),
        "README must mark FR161 / Epic 93 closed"
    );
    assert!(
        readme.contains("FR162")
            && (readme.contains("未关闭前不得宣称") || readme.contains("不得宣称")),
        "README must keep FR162+ unclaimed"
    );
    assert!(
        (readme.contains("全清") || readme.contains("NFR59"))
            && (readme.contains("不得") || readme.contains("禁止")),
        "README must forbid NFR59 fully-cleared claim"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr161_deferred_marks_epic93_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 93")
            && deferred.contains("FR161")
            && (deferred.contains("已关闭") || deferred.contains("Story 93.3")),
        "deferred must mark Epic 93 / FR161 closed"
    );
    assert!(
        deferred.contains("FR162") && deferred.contains("Epic 94"),
        "deferred must point remaining deepen to Epic 94+"
    );
}

#[test]
fn fr161_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 93")
            && agents.contains("FR161")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 93 / FR161 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 93")
            && (spine.contains("已关闭") || spine.contains("93.3"))
            && spine.contains("FR161"),
        "spine must note Epic 93 / FR161 closed"
    );
}

#[test]
fn fr161_nfr14_epic93_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic93-formal-sby-image-hygiene-fr161.md",
    );
    for needle in [
        "- [x] **FR161 钉死子集实现 + 验收**",
        "- [x] **文档 / deferred / README 收口**",
        "- [x] **NFR68/70/71/72：**",
        "- [x] **品牌 / AD-6：**",
        "- [x] **其余 FR162–165：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 93 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 93.3"),
        "NFR14 risk record must be closed after Story 93.3"
    );
}

#[test]
fn fr161_sprint_epic93_done_94_ready() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-93: done") || sprint.contains("epic-93:done"),
        "epic-93 must be done"
    );
    assert!(
        sprint.contains("93-3-fr161-收口与文档指针: done")
            || sprint.contains("93-3-fr161-收口与文档指针:done"),
        "93-3 must be done"
    );
    assert!(
        sprint.contains("94-1-epic-94-nfr14-风险记录: ready-for-dev")
            || sprint.contains("94-1-epic-94-nfr14-风险记录:ready-for-dev")
            || sprint.contains("94-1-epic-94-nfr14-风险记录: done")
            || sprint.contains("94-1-epic-94-nfr14-风险记录: in-progress"),
        "94-1 must be ready-for-dev after Epic 93 closes"
    );
}

#[test]
fn fr161_epics_phase19_epic93_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase19Epic93Status: complete")
            || epics.contains("phase19Epic93Status:complete"),
        "epics.md must stamp phase19Epic93Status complete"
    );
}

#[test]
fn fr161_product_doc_closed_status() {
    let doc = read("docs/fr161-formal-sby-image-hygiene.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("93.3"),
        "fr161 product doc must note closed status"
    );
}
