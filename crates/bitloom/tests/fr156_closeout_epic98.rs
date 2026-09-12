//! ATDD / guardrail: Story 98.3 / FR156 — Epic 98 + Phase 19 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr156_closeout_epic98
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
fn fr156_readme_marks_fr156_and_phase19_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR156")
            && (readme.contains("已关闭") || readme.contains("complete"))
            && readme.contains("fr156-phase19-claim-honesty"),
        "README must mark FR156 closed with doc pointer"
    );
    assert!(
        readme.contains("Phase 19")
            && (readme.contains("已关闭")
                || readme.contains("complete")
                || readme.contains("已齐")),
        "README must mark Phase 19 closed / stories complete"
    );
    assert!(
        readme.contains("NFR71")
            && (readme.contains("新合同") || readme.contains("仍须") || readme.contains("超出")),
        "README must keep NFR71 leftovers honesty"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr156_deferred_marks_epic98_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 98")
            && deferred.contains("FR156")
            && (deferred.contains("已关闭") || deferred.contains("Story 98.3")),
        "deferred must mark Epic 98 / FR156 closed"
    );
    assert!(
        deferred.contains("Epic 87–98") || deferred.contains("Epic 87-98"),
        "deferred must declare Phase 19 stories complete"
    );
}

#[test]
fn fr156_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 98")
            && agents.contains("FR156")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 98 / FR156 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 98")
            && (spine.contains("已关闭") || spine.contains("98.3"))
            && spine.contains("FR156"),
        "spine must note Epic 98 / FR156 closed"
    );
}

#[test]
fn fr156_nfr14_epic98_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic98-phase19-claim-honesty-fr156.md",
    );
    for needle in [
        "- [x] **FR156 诚实面落地**",
        "- [x] **文档 / deferred / README / spine 收口**",
        "- [x] **NFR68/71/72：**",
        "- [x] **品牌 / AD-6：**",
        "- [x] **Phase 19 规划故事齐：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 98 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 98.3"),
        "NFR14 risk record must be closed after Story 98.3"
    );
}

#[test]
fn fr156_sprint_epic98_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-98: done") || sprint.contains("epic-98:done"),
        "epic-98 must be done"
    );
    assert!(
        sprint.contains("98-3-fr156-收口与-phase-19-故事清单指针: done")
            || sprint.contains("98-3-fr156-收口与-phase-19-故事清单指针:done"),
        "98-3 must be done"
    );
}

#[test]
fn fr156_epics_phase19_epic98_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase19Epic98Status: complete")
            || epics.contains("phase19Epic98Status:complete"),
        "epics.md must stamp phase19Epic98Status complete"
    );
    assert!(
        epics.contains("phase19Status: complete") || epics.contains("phase19Status:complete"),
        "epics.md must stamp phase19Status complete"
    );
}

#[test]
fn fr156_product_doc_closed_status() {
    let doc = read("docs/fr156-phase19-claim-honesty.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("98.3"),
        "fr156 product doc must note closed status"
    );
}
