//! ATDD / guardrail: Story 121.3 / FR188 — Epic 121 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr188_closeout_epic121
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
fn fr188_readme_marks_fr188_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR188")
            && (readme.contains("已关闭") || readme.contains("Epic 121"))
            && (readme.contains("community")
                || readme.contains("scalafmt")
                || readme.contains("fr188")
                || readme.contains("Style")),
        "README must mark FR188 / Epic 121 closed"
    );
    assert!(readme.contains("Bitloom"));
    assert!(
        readme.contains("FR181")
            && (readme.contains("alone") || readme.contains("≠") || readme.contains("超 FR181")),
        "README must keep ≠ FR181 alone honesty"
    );
}

#[test]
fn fr188_deferred_marks_epic121_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 121")
            && deferred.contains("FR188")
            && (deferred.contains("已关闭") || deferred.contains("Story 121.3")),
        "deferred must mark Epic 121 / FR188 closed"
    );
    assert!(
        deferred.contains("NFR91") || deferred.contains("FR189") || deferred.contains("Epic 122"),
        "deferred must point remaining deepen / NFR91"
    );
}

#[test]
fn fr188_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 121")
            && agents.contains("FR188")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 121 / FR188 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 121")
            && (spine.contains("已关闭") || spine.contains("121.3"))
            && spine.contains("FR188"),
        "spine must note Epic 121 / FR188 closed"
    );
}

#[test]
fn fr188_nfr14_epic121_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic121-community-style-guide-pack-fr188.md",
    );
    for needle in [
        "- [x] **Style Guide 全家桶通道 + 验收谓词：**",
        "- [x] **AD-27 修订（NFR90）：**",
        "- [x] **docs/fr188-* + README/deferred：**",
        "- [x] **NFR88：**",
        "- [x] **禁止事项未触发：**",
        "- [x] **品牌 / 依赖：**",
        "- [x] **超子集：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 121 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 121.3") || text.contains("可宣称"),
        "NFR14 must be closed after Story 121.3"
    );
}

#[test]
fn fr188_sprint_epic121_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-121: done") || sprint.contains("epic-121:done"),
        "epic-121 must be done"
    );
    assert!(
        sprint.contains("121-3-fr188-收口与文档指针: done")
            || sprint.contains("121-3-fr188-收口与文档指针:done"),
        "121-3 must be done"
    );
    assert!(
        sprint.contains("122-1-epic-122-nfr14-风险记录: ready-for-dev")
            || sprint.contains("122-1-epic-122-nfr14-风险记录:ready-for-dev")
            || sprint.contains("122-1-epic-122-nfr14-风险记录: done")
            || sprint.contains("122-1-epic-122-nfr14-风险记录: in-progress"),
        "122-1 must be ready-for-dev after Epic 121 closes"
    );
}

#[test]
fn fr188_epics_phase23_epic121_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase23Epic121Status: complete")
            || epics.contains("phase23Epic121Status:complete"),
        "epics.md must stamp phase23Epic121Status complete"
    );
}

#[test]
fn fr188_product_doc_closed() {
    let doc = read("docs/fr188-community-style-guide-pack.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("121.3"),
        "fr188 product doc must note closed status"
    );
    assert!(
        doc.contains("NFR91") || doc.contains("超子集") || doc.contains("IDE"),
        "must leave fuller suite as NFR91"
    );
}
