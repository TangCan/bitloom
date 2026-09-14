//! ATDD / guardrail: Story 120.3 / FR187 — Epic 120 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr187_closeout_epic120
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
fn fr187_readme_marks_fr187_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR187")
            && (readme.contains("已关闭") || readme.contains("Epic 120"))
            && (readme.contains("branch")
                || readme.contains("fr187")
                || readme.contains("Handshake")),
        "README must mark FR187 / Epic 120 closed"
    );
    assert!(readme.contains("Bitloom"));
    assert!(
        readme.contains("FR180")
            && (readme.contains("alone") || readme.contains("≠") || readme.contains("超 FR180")),
        "README must keep ≠ FR180 alone honesty"
    );
}

#[test]
fn fr187_deferred_marks_epic120_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 120")
            && deferred.contains("FR187")
            && (deferred.contains("已关闭") || deferred.contains("Story 120.3")),
        "deferred must mark Epic 120 / FR187 closed"
    );
    assert!(
        deferred.contains("NFR91") || deferred.contains("FR188") || deferred.contains("Epic 121"),
        "deferred must point remaining deepen / NFR91"
    );
}

#[test]
fn fr187_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 120")
            && agents.contains("FR187")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 120 / FR187 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 120")
            && (spine.contains("已关闭") || spine.contains("120.3"))
            && spine.contains("FR187"),
        "spine must note Epic 120 / FR187 closed"
    );
}

#[test]
fn fr187_nfr14_epic120_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic120-handshake-lower-deepen-fr187.md",
    );
    for needle in [
        "- [x] **Handshake lower deepen 通道 + 验收谓词：**",
        "- [x] **AD-25 修订（NFR90）：**",
        "- [x] **docs/fr187-* + README/deferred：**",
        "- [x] **NFR88：**",
        "- [x] **禁止事项未触发：**",
        "- [x] **品牌 / 依赖：**",
        "- [x] **超子集：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 120 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 120.3") || text.contains("可宣称"),
        "NFR14 must be closed after Story 120.3"
    );
}

#[test]
fn fr187_sprint_epic120_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-120: done") || sprint.contains("epic-120:done"),
        "epic-120 must be done"
    );
    assert!(
        sprint.contains("120-3-fr187-收口与文档指针: done")
            || sprint.contains("120-3-fr187-收口与文档指针:done"),
        "120-3 must be done"
    );
    assert!(
        sprint.contains("121-1-epic-121-nfr14-风险记录: ready-for-dev")
            || sprint.contains("121-1-epic-121-nfr14-风险记录:ready-for-dev")
            || sprint.contains("121-1-epic-121-nfr14-风险记录: done")
            || sprint.contains("121-1-epic-121-nfr14-风险记录: in-progress"),
        "121-1 must be ready-for-dev after Epic 120 closes"
    );
}

#[test]
fn fr187_epics_phase23_epic120_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase23Epic120Status: complete")
            || epics.contains("phase23Epic120Status:complete"),
        "epics.md must stamp phase23Epic120Status complete"
    );
}

#[test]
fn fr187_product_doc_closed() {
    let doc = read("docs/fr187-handshake-lower-deepen.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("120.3"),
        "fr187 product doc must note closed status"
    );
    assert!(
        doc.contains("NFR91") || doc.contains("超子集") || doc.contains("full"),
        "must leave fuller lower as NFR91"
    );
}
