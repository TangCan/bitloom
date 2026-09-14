//! ATDD / guardrail: Story 114.3 / FR181 — Epic 114 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr181_closeout_epic114
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
fn fr181_readme_marks_fr181_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR181")
            && (readme.contains("已关闭") || readme.contains("Epic 114"))
            && (readme.contains("wartremover") || readme.contains("fr181")),
        "README must mark FR181 / Epic 114 closed"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr181_deferred_marks_epic114_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 114")
            && deferred.contains("FR181")
            && (deferred.contains("已关闭") || deferred.contains("Story 114.3")),
        "deferred must mark Epic 114 / FR181 closed"
    );
    assert!(
        deferred.contains("FR182") && deferred.contains("Epic 115"),
        "deferred must point remaining deepen to Epic 115+"
    );
}

#[test]
fn fr181_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 114")
            && agents.contains("FR181")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 114 / FR181 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 114")
            && (spine.contains("已关闭") || spine.contains("114.3"))
            && spine.contains("FR181"),
        "spine must note Epic 114 / FR181 closed"
    );
}

#[test]
fn fr181_nfr14_epic114_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic114-deeper-style-guide-linter-fr181.md",
    );
    for needle in [
        "- [x] **Style/linter deepen 通道 + 验收谓词：**",
        "- [x] **AD-27 修订（NFR85）：**",
        "- [x] **docs/fr181-* + README/deferred：**",
        "- [x] **NFR83：**",
        "- [x] **禁止事项未触发：**",
        "- [x] **品牌 / 依赖：**",
        "- [x] **超子集：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 114 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 114.3") || text.contains("可宣称"),
        "NFR14 must be closed after Story 114.3"
    );
}

#[test]
fn fr181_sprint_epic114_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-114: done") || sprint.contains("epic-114:done"),
        "epic-114 must be done"
    );
    assert!(
        sprint.contains("114-3-fr181-收口与文档指针: done")
            || sprint.contains("114-3-fr181-收口与文档指针:done"),
        "114-3 must be done"
    );
}

#[test]
fn fr181_epics_phase22_epic114_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase22Epic114Status: complete")
            || epics.contains("phase22Epic114Status:complete"),
        "epics.md must stamp phase22Epic114Status complete"
    );
}

#[test]
fn fr181_product_doc_closed() {
    let doc = read("docs/fr181-deeper-style-guide-linter.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("114.3"),
        "fr181 product doc must note closed status"
    );
    assert!(
        doc.contains("NFR86") || doc.contains("全家桶"),
        "must leave fuller suite as NFR86"
    );
}
