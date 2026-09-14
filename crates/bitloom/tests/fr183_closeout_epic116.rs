//! ATDD / guardrail: Story 116.3 / FR183 — Epic 116 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr183_closeout_epic116
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
fn fr183_readme_marks_fr183_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR183")
            && (readme.contains("已关闭") || readme.contains("Epic 116"))
            && (readme.contains("bitloom-firrtl") || readme.contains("fr183")),
        "README must mark FR183 / Epic 116 closed"
    );
    assert!(
        !readme.contains("FR183 / 116 | SemVer/docs 诚实（≠ 静默扩大）— **未关闭**"),
        "README must not keep the open FR183 row"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr183_deferred_marks_epic116_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 116")
            && deferred.contains("FR183")
            && (deferred.contains("已关闭") || deferred.contains("Story 116.3")),
        "deferred must mark Epic 116 / FR183 closed"
    );
    assert!(
        deferred.contains("FR184") && deferred.contains("Epic 117"),
        "deferred must point remaining deepen to Epic 117"
    );
}

#[test]
fn fr183_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 116")
            && agents.contains("FR183")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 116 / FR183 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 116")
            && (spine.contains("已关闭") || spine.contains("116.3"))
            && spine.contains("FR183"),
        "spine must note Epic 116 / FR183 closed"
    );
}

#[test]
fn fr183_nfr14_epic116_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic116-explicit-fr142-api-expand-fr183.md",
    );
    for needle in [
        "- [x] **显式表面扩展 + 验收谓词：**",
        "- [x] **表面文档修订（NFR85）：**",
        "- [x] **docs/fr183-* + README/deferred：**",
        "- [x] **NFR83：**",
        "- [x] **禁止事项未触发：**",
        "- [x] **品牌 / 依赖：**",
        "- [x] **超子集：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 116 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 116.3") || text.contains("可宣称"),
        "NFR14 must be closed after Story 116.3"
    );
}

#[test]
fn fr183_sprint_epic116_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-116: done") || sprint.contains("epic-116:done"),
        "epic-116 must be done"
    );
    assert!(
        sprint.contains("116-3-fr183-收口与文档指针: done")
            || sprint.contains("116-3-fr183-收口与文档指针:done"),
        "116-3 must be done"
    );
}

#[test]
fn fr183_epics_phase22_epic116_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase22Epic116Status: complete")
            || epics.contains("phase22Epic116Status:complete"),
        "epics.md must stamp phase22Epic116Status complete"
    );
}

#[test]
fn fr183_product_doc_closed() {
    let doc = read("docs/fr183-explicit-fr142-api-expand.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("116.3"),
        "fr183 product doc must note closed status"
    );
    assert!(
        doc.contains("FR190")
            || doc.contains("NFR86")
            || doc.contains("NFR91")
            || doc.contains("Further"),
        "must leave further API expands as FR190/NFR91 (or historical NFR86)"
    );
}
