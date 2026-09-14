//! ATDD / guardrail: Story 123.3 / FR190 — Epic 123 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr190_closeout_epic123
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
fn fr190_readme_marks_fr190_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR190")
            && (readme.contains("已关闭") || readme.contains("Epic 123"))
            && (readme.contains("emit")
                || readme.contains("import")
                || readme.contains("check_")
                || readme.contains("fr190")
                || readme.contains("FR142")),
        "README must mark FR190 / Epic 123 closed"
    );
    assert!(readme.contains("Bitloom"));
    assert!(
        readme.contains("FR183")
            && (readme.contains("alone") || readme.contains("≠") || readme.contains("超 FR183")),
        "README must keep ≠ FR183 alone honesty"
    );
}

#[test]
fn fr190_deferred_marks_epic123_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 123")
            && deferred.contains("FR190")
            && (deferred.contains("已关闭") || deferred.contains("Story 123.3")),
        "deferred must mark Epic 123 / FR190 closed"
    );
    assert!(
        deferred.contains("FR189")
            && (deferred.contains("blocked")
                || deferred.contains("未关")
                || deferred.contains("NFR91")
                || deferred.contains("Epic 124")),
        "deferred must keep FR189 honesty / point remaining"
    );
}

#[test]
fn fr190_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 123")
            && agents.contains("FR190")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 123 / FR190 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 123")
            && (spine.contains("已关闭") || spine.contains("123.3"))
            && spine.contains("FR190"),
        "spine must note Epic 123 / FR190 closed"
    );
}

#[test]
fn fr190_nfr14_epic123_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic123-further-fr142-api-expand-fr190.md",
    );
    for needle in [
        "- [x] **表面扩展 S1–S3 + 验收谓词：**",
        "- [x] **表面文档修订（NFR90）：**",
        "- [x] **docs/fr190-* + README/deferred：**",
        "- [x] **NFR88：**",
        "- [x] **禁止事项未触发：**",
        "- [x] **品牌 / 依赖：**",
        "- [x] **超子集：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 123 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 123.3") || text.contains("可宣称"),
        "NFR14 must be closed after Story 123.3"
    );
}

#[test]
fn fr190_sprint_epic123_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-123: done") || sprint.contains("epic-123:done"),
        "epic-123 must be done"
    );
    assert!(
        sprint.contains("123-3-fr190-收口与文档指针: done")
            || sprint.contains("123-3-fr190-收口与文档指针:done"),
        "123-3 must be done"
    );
    assert!(
        sprint.contains("124-1-epic-124-nfr14-风险记录: ready-for-dev")
            || sprint.contains("124-1-epic-124-nfr14-风险记录:ready-for-dev")
            || sprint.contains("124-1-epic-124-nfr14-风险记录: done")
            || sprint.contains("124-1-epic-124-nfr14-风险记录: in-progress"),
        "124-1 must be ready-for-dev after Epic 123 closes"
    );
}

#[test]
fn fr190_epics_phase23_epic123_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase23Epic123Status: complete")
            || epics.contains("phase23Epic123Status:complete"),
        "epics.md must stamp phase23Epic123Status complete"
    );
}

#[test]
fn fr190_product_doc_closed() {
    let doc = read("docs/fr190-further-fr142-api-expand.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("123.3"),
        "fr190 product doc must note closed status"
    );
    assert!(
        doc.contains("NFR91") || doc.contains("超子集") || doc.contains("Further"),
        "must leave fuller expands as NFR91"
    );
}
