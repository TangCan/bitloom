//! ATDD / guardrail: Story 106.3 / FR173 — Epic 106 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr173_closeout_epic106
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
fn fr173_readme_marks_fr173_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR173")
            && (readme.contains("已关闭") || readme.contains("Epic 106"))
            && (readme.contains("1.158.0") || readme.contains("fr173")),
        "README must mark FR173 / Epic 106 closed"
    );
    assert!(
        readme.contains("NFR81")
            || (readme.contains("不得") && readme.contains("FR174"))
            || readme.contains("未关闭"),
        "README must keep remaining Phase 21 honesty"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr173_deferred_marks_epic106_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 106")
            && deferred.contains("FR173")
            && (deferred.contains("已关闭") || deferred.contains("Story 106.3")),
        "deferred must mark Epic 106 / FR173 closed"
    );
    assert!(
        deferred.contains("FR174") && deferred.contains("Epic 107"),
        "deferred must point remaining deepen to Epic 107+"
    );
}

#[test]
fn fr173_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 106")
            && agents.contains("FR173")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 106 / FR173 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 106")
            && (spine.contains("已关闭") || spine.contains("106.3"))
            && spine.contains("FR173"),
        "spine must note Epic 106 / FR173 closed"
    );
}

#[test]
fn fr173_nfr14_epic106_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic106-firtool-bump-ad9-fr173.md");
    for needle in [
        "- [x] **FR173 钉死子集实现 + 验收**",
        "- [x] **文档 / deferred / README / spine 收口**",
        "- [x] **NFR78/81：",
        "- [x] **品牌 / AD-6：",
        "- [x] **其余 FR174–177：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 106 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 106.3") || text.contains("可宣称"),
        "NFR14 must be closed after Story 106.3"
    );
}

#[test]
fn fr173_sprint_epic106_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-106: done") || sprint.contains("epic-106:done"),
        "epic-106 must be done"
    );
    assert!(
        sprint.contains("106-3-fr173-收口与文档指针: done")
            || sprint.contains("106-3-fr173-收口与文档指针:done"),
        "106-3 must be done"
    );
}

#[test]
fn fr173_epics_phase21_epic106_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase21Epic106Status: complete")
            || epics.contains("phase21Epic106Status:complete"),
        "epics.md must stamp phase21Epic106Status complete"
    );
}

#[test]
fn fr173_product_doc_closed() {
    let doc = read("docs/fr173-firtool-bump-ad9.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("106.3"),
        "fr173 product doc must note closed status"
    );
    assert!(
        doc.contains("NFR81")
            || doc.contains("FR174")
            || (doc.contains("Further") && doc.contains("beyond")),
        "product doc must note remaining deepen / NFR81"
    );
}
