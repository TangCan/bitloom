//! ATDD / guardrail: Story 109.3 / FR176 — Epic 109 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr176_closeout_epic109
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
fn fr176_readme_marks_fr176_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR176")
            && (readme.contains("已关闭") || readme.contains("Epic 109"))
            && (readme.contains("ecosystem")
                || readme.contains("生态")
                || readme.contains("fr176")),
        "README must mark FR176 / Epic 109 closed"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr176_deferred_marks_epic109_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 109")
            && deferred.contains("FR176")
            && (deferred.contains("已关闭") || deferred.contains("Story 109.3")),
        "deferred must mark Epic 109 / FR176 closed"
    );
    assert!(
        deferred.contains("FR177") && deferred.contains("Epic 110"),
        "deferred must point remaining honesty to Epic 110"
    );
}

#[test]
fn fr176_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 109")
            && agents.contains("FR176")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 109 / FR176 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 109")
            && (spine.contains("已关闭") || spine.contains("109.3"))
            && spine.contains("FR176"),
        "spine must note Epic 109 / FR176 closed"
    );
}

#[test]
fn fr176_nfr14_epic109_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic109-deeper-parser-chisel-ecosystem-fr176.md",
    );
    for needle in [
        "- [x] **FR176 钉死子集实现 + 验收**",
        "- [x] **文档 / deferred / README / spine 收口**",
        "- [x] **NFR78/81：",
        "- [x] **品牌 / AD-6：",
        "- [x] **其余 FR177：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 109 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 109.3") || text.contains("可宣称"),
        "NFR14 must be closed after Story 109.3"
    );
}

#[test]
fn fr176_sprint_epic109_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-109: done") || sprint.contains("epic-109:done"),
        "epic-109 must be done"
    );
    assert!(
        sprint.contains("109-3-fr176-收口与文档指针: done")
            || sprint.contains("109-3-fr176-收口与文档指针:done"),
        "109-3 must be done"
    );
}

#[test]
fn fr176_epics_phase21_epic109_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase21Epic109Status: complete")
            || epics.contains("phase21Epic109Status:complete"),
        "epics.md must stamp phase21Epic109Status complete"
    );
}

#[test]
fn fr176_product_doc_closed() {
    let doc = read("docs/fr176-deeper-parser-chisel-ecosystem.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("109.3"),
        "fr176 product doc must note closed status"
    );
}
