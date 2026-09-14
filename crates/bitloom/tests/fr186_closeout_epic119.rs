//! ATDD / guardrail: Story 119.3 / FR186 — Epic 119 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr186_closeout_epic119
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
fn fr186_readme_marks_fr186_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR186")
            && (readme.contains("已关闭") || readme.contains("Epic 119"))
            && (readme.contains("live tip")
                || readme.contains("circt-live-tip")
                || readme.contains("fr186")),
        "README must mark FR186 / Epic 119 closed"
    );
    assert!(readme.contains("Bitloom"));
    assert!(
        readme.contains("FR179")
            && (readme.contains("alone") || readme.contains("≠") || readme.contains("超 FR179")),
        "README must keep ≠ FR179 alone honesty"
    );
}

#[test]
fn fr186_deferred_marks_epic119_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 119")
            && deferred.contains("FR186")
            && (deferred.contains("已关闭") || deferred.contains("Story 119.3")),
        "deferred must mark Epic 119 / FR186 closed"
    );
    assert!(
        deferred.contains("NFR91") || deferred.contains("FR187") || deferred.contains("Epic 120"),
        "deferred must point remaining deepen / NFR91"
    );
}

#[test]
fn fr186_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 119")
            && agents.contains("FR186")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 119 / FR186 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 119")
            && (spine.contains("已关闭") || spine.contains("119.3"))
            && spine.contains("FR186"),
        "spine must note Epic 119 / FR186 closed"
    );
}

#[test]
fn fr186_nfr14_epic119_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic119-unbounded-circt-tip-fr186.md",
    );
    for needle in [
        "- [x] **无界 tip 通道 + 验收谓词：**",
        "- [x] **AD-9 修订（NFR90）：**",
        "- [x] **docs/README/deferred：**",
        "- [x] **NFR88 / NFR91：**",
        "- [x] **禁止事项未触发：**",
        "- [x] **品牌 / 依赖：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 119 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 119.3") || text.contains("可宣称"),
        "NFR14 must be closed after Story 119.3"
    );
}

#[test]
fn fr186_sprint_epic119_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-119: done") || sprint.contains("epic-119:done"),
        "epic-119 must be done"
    );
    assert!(
        sprint.contains("119-3-fr186-收口与文档指针: done")
            || sprint.contains("119-3-fr186-收口与文档指针:done"),
        "119-3 must be done"
    );
    assert!(
        sprint.contains("120-1-epic-120-nfr14-风险记录: ready-for-dev")
            || sprint.contains("120-1-epic-120-nfr14-风险记录:ready-for-dev")
            || sprint.contains("120-1-epic-120-nfr14-风险记录: done")
            || sprint.contains("120-1-epic-120-nfr14-风险记录: in-progress"),
        "120-1 must be ready-for-dev after Epic 119 closes"
    );
}

#[test]
fn fr186_epics_phase23_epic119_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase23Epic119Status: complete")
            || epics.contains("phase23Epic119Status:complete"),
        "epics.md must stamp phase23Epic119Status complete"
    );
}

#[test]
fn fr186_product_doc_closed() {
    let doc = read("docs/fr186-unbounded-circt-tip.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("119.3"),
        "fr186 product doc must note closed status"
    );
    assert!(
        doc.contains("NFR91") || doc.contains("超子集") || doc.contains("newer contract"),
        "must leave beyond-subset as NFR91"
    );
}
