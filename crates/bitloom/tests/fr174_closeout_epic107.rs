//! ATDD / guardrail: Story 107.3 / FR174 — Epic 107 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr174_closeout_epic107
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
fn fr174_readme_marks_fr174_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR174")
            && (readme.contains("已关闭") || readme.contains("Epic 107"))
            && (readme.contains("1.156.0") || readme.contains("fr174")),
        "README must mark FR174 / Epic 107 closed"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr174_deferred_marks_epic107_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 107")
            && deferred.contains("FR174")
            && (deferred.contains("已关闭") || deferred.contains("Story 107.3")),
        "deferred must mark Epic 107 / FR174 closed"
    );
    assert!(
        deferred.contains("FR175") && deferred.contains("Epic 108"),
        "deferred must point remaining deepen to Epic 108+"
    );
}

#[test]
fn fr174_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 107")
            && agents.contains("FR174")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 107 / FR174 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 107")
            && (spine.contains("已关闭") || spine.contains("107.3"))
            && spine.contains("FR174"),
        "spine must note Epic 107 / FR174 closed"
    );
}

#[test]
fn fr174_nfr14_epic107_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic107-unpaired-head-fr174.md");
    for needle in [
        "- [x] **FR174 钉死子集实现 + 验收**",
        "- [x] **文档 / deferred / README / spine 收口**",
        "- [x] **NFR78/81：",
        "- [x] **品牌 / AD-6：",
        "- [x] **其余 FR175–177：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 107 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 107.3") || text.contains("可宣称"),
        "NFR14 must be closed after Story 107.3"
    );
}

#[test]
fn fr174_sprint_epic107_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-107: done") || sprint.contains("epic-107:done"),
        "epic-107 must be done"
    );
    assert!(
        sprint.contains("107-3-fr174-收口与文档指针: done")
            || sprint.contains("107-3-fr174-收口与文档指针:done"),
        "107-3 must be done"
    );
}

#[test]
fn fr174_epics_phase21_epic107_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase21Epic107Status: complete")
            || epics.contains("phase21Epic107Status:complete"),
        "epics.md must stamp phase21Epic107Status complete"
    );
}

#[test]
fn fr174_product_doc_closed() {
    let doc = read("docs/fr174-unpaired-head.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("107.3"),
        "fr174 product doc must note closed status"
    );
}
