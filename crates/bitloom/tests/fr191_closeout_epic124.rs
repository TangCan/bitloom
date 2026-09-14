//! ATDD / guardrail: Story 124.3 / FR191 — Epic 124 + Phase 23 honesty closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr191_closeout_epic124
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
fn fr191_readme_marks_fr191_and_phase23_planning_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR191")
            && (readme.contains("已关闭") || readme.contains("complete"))
            && readme.contains("fr191-phase23-claim-honesty"),
        "README must mark FR191 closed with doc pointer"
    );
    assert!(
        readme.contains("Phase 23")
            && (readme.contains("规划故事已齐")
                || readme.contains("Epic 118–124")
                || readme.contains("Epic 118-124")),
        "README must mark Phase 23 planning stories complete"
    );
    assert!(
        readme.contains("FR189")
            && (readme.contains("blocked-upstream") || readme.contains("blocked")),
        "README must keep FR189 blocked honesty"
    );
    assert!(
        readme.contains("NFR91")
            && (readme.contains("新合同") || readme.contains("仍须") || readme.contains("超出")),
        "README must keep NFR91 leftovers honesty"
    );
    assert!(
        readme.contains("NFR86")
            && (readme.contains("账本已空") || readme.contains("已空"))
            && (readme.contains("不得") || readme.contains("禁止")),
        "README must forbid NFR86 ledger-empty claim"
    );
    assert!(
        readme.contains("Phase 22") && (readme.contains("alone") || readme.contains("冒充")),
        "README must keep Phase 22 alone honesty"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr191_deferred_marks_epic124_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 124")
            && deferred.contains("FR191")
            && (deferred.contains("已关闭") || deferred.contains("Story 124.3")),
        "deferred must mark Epic 124 / FR191 closed"
    );
    assert!(
        deferred.contains("Epic 118–124")
            || deferred.contains("Epic 118-124")
            || deferred.contains("规划故事已齐"),
        "deferred must declare Phase 23 planning stories complete"
    );
    assert!(
        deferred.contains("FR189")
            && (deferred.contains("blocked") || deferred.contains("blocked-upstream")),
        "deferred must keep FR189 blocked"
    );
}

#[test]
fn fr191_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 124")
            && agents.contains("FR191")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 124 / FR191 closed"
    );
    assert!(
        agents.contains("Epic 118–124")
            || agents.contains("planning stories complete")
            || agents.contains("Phase 23 planning"),
        "AGENTS must declare Phase 23 planning stories complete"
    );
    assert!(
        agents.contains("FR189")
            && (agents.contains("blocked") || agents.contains("blocked-upstream")),
        "AGENTS must keep FR189 blocked"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 124")
            && (spine.contains("已关闭") || spine.contains("124.3"))
            && spine.contains("FR191"),
        "spine must note Epic 124 / FR191 closed"
    );
    assert!(
        spine.contains("Epic 118–124")
            || spine.contains("规划故事已齐")
            || spine.contains("规划+诚实"),
        "spine must declare Phase 23 planning stories complete"
    );
    assert!(
        spine.contains("FR189")
            && (spine.contains("blocked") || spine.contains("blocked-upstream")),
        "spine must keep FR189 blocked"
    );
}

#[test]
fn fr191_nfr14_epic124_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic124-phase23-claim-honesty-fr191.md",
    );
    for needle in [
        "- [x] **FR191 诚实面落地**",
        "- [x] **文档 / deferred / README / spine 收口**",
        "- [x] **Phase 23 规划故事齐声明**",
        "- [x] **NFR88：**",
        "- [x] **禁止事项未触发：**",
        "- [x] **品牌 / 依赖：**",
        "- [x] **超子集：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 124 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 124.3") || text.contains("可宣称"),
        "NFR14 must be closed after Story 124.3"
    );
    assert!(
        text.contains("FR189") && (text.contains("blocked") || text.contains("未关")),
        "NFR14 close must keep FR189 honesty"
    );
}

#[test]
fn fr191_sprint_epic124_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-124: done") || sprint.contains("epic-124:done"),
        "epic-124 must be done"
    );
    assert!(
        sprint.contains("124-3-fr191-收口与-phase-23-故事清单指针: done")
            || sprint.contains("124-3-fr191-收口与-phase-23-故事清单指针:done"),
        "124-3 must be done"
    );
}

#[test]
fn fr191_epics_phase23_epic124_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase23Epic124Status: complete")
            || epics.contains("phase23Epic124Status:complete"),
        "epics.md must stamp phase23Epic124Status complete"
    );
    assert!(
        epics.contains("phase23Status: planning-complete")
            || epics.contains("Phase 23 planning stories complete")
            || epics.contains("规划故事已齐"),
        "epics.md must keep Phase 23 planning-complete (not false full-impl complete)"
    );
    assert!(
        !epics.contains("phase23Status: complete\n")
            || epics.contains("FR189") && epics.contains("blocked"),
        "must not silently mark full Phase 23 implementation complete without FR189 honesty"
    );
}

#[test]
fn fr191_product_doc_closed() {
    let doc = read("docs/fr191-phase23-claim-honesty.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("124.3"),
        "fr191 product doc must note closed status"
    );
    assert!(
        doc.contains("FR189") && (doc.contains("blocked") || doc.contains("blocked-upstream")),
        "must keep FR189 blocked honesty"
    );
    assert!(
        doc.contains("NFR91") || doc.contains("NFR86"),
        "must leave beyond-NFR14 / NFR86 honesty"
    );
}
