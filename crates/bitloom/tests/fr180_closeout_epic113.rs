//! ATDD / guardrail: Story 113.3 / FR180 — Epic 113 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr180_closeout_epic113
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
fn fr180_readme_marks_fr180_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR180")
            && (readme.contains("已关闭") || readme.contains("Epic 113"))
            && (readme.contains("fork") || readme.contains("fr180")),
        "README must mark FR180 / Epic 113 closed"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr180_deferred_marks_epic113_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 113")
            && deferred.contains("FR180")
            && (deferred.contains("已关闭") || deferred.contains("Story 113.3")),
        "deferred must mark Epic 113 / FR180 closed"
    );
    assert!(
        deferred.contains("FR181") && deferred.contains("Epic 114"),
        "deferred must point remaining deepen to Epic 114+"
    );
}

#[test]
fn fr180_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 113")
            && agents.contains("FR180")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 113 / FR180 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 113")
            && (spine.contains("已关闭") || spine.contains("113.3"))
            && spine.contains("FR180"),
        "spine must note Epic 113 / FR180 closed"
    );
}

#[test]
fn fr180_nfr14_epic113_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic113-handshake-dialect-deepen-fr180.md",
    );
    for needle in [
        "- [x] **Handshake deepen 通道 + 验收谓词：**",
        "- [x] **AD-25 修订（NFR85）：**",
        "- [x] **docs/fr180-* + README/deferred：**",
        "- [x] **NFR83：**",
        "- [x] **禁止事项未触发：**",
        "- [x] **品牌 / 依赖：**",
        "- [x] **超子集：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 113 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 113.3") || text.contains("可宣称"),
        "NFR14 must be closed after Story 113.3"
    );
}

#[test]
fn fr180_sprint_epic113_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-113: done") || sprint.contains("epic-113:done"),
        "epic-113 must be done"
    );
    assert!(
        sprint.contains("113-3-fr180-收口与文档指针: done")
            || sprint.contains("113-3-fr180-收口与文档指针:done"),
        "113-3 must be done"
    );
}

#[test]
fn fr180_epics_phase22_epic113_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase22Epic113Status: complete")
            || epics.contains("phase22Epic113Status:complete"),
        "epics.md must stamp phase22Epic113Status complete"
    );
}

#[test]
fn fr180_product_doc_closed() {
    let doc = read("docs/fr180-handshake-dialect-deepen.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("113.3"),
        "fr180 product doc must note closed status"
    );
    assert!(
        doc.contains("NFR86") || doc.contains("full") || doc.contains("全家桶"),
        "must leave fuller dialect as NFR86"
    );
}
