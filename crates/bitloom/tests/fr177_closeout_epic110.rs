//! ATDD / guardrail: Story 110.3 / FR177 — Epic 110 + Phase 21 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr177_closeout_epic110
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
fn fr177_readme_marks_fr177_and_phase21_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR177")
            && (readme.contains("已关闭") || readme.contains("complete"))
            && readme.contains("fr177-phase21-claim-honesty"),
        "README must mark FR177 closed with doc pointer"
    );
    assert!(
        readme.contains("Phase 21")
            && (readme.contains("已关闭")
                || readme.contains("complete")
                || readme.contains("已齐")
                || readme.contains("Epic 105–110")),
        "README must mark Phase 21 closed / stories complete"
    );
    assert!(
        readme.contains("NFR81")
            && (readme.contains("新合同") || readme.contains("仍须") || readme.contains("超出")),
        "README must keep NFR81 leftovers honesty"
    );
    assert!(
        readme.contains("NFR76")
            && (readme.contains("账本已空") || readme.contains("已空"))
            && (readme.contains("不得") || readme.contains("禁止")),
        "README must forbid NFR76 ledger-empty claim"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr177_deferred_marks_epic110_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 110")
            && deferred.contains("FR177")
            && (deferred.contains("已关闭") || deferred.contains("Story 110.3")),
        "deferred must mark Epic 110 / FR177 closed"
    );
    assert!(
        deferred.contains("Epic 105–110") || deferred.contains("Epic 105-110"),
        "deferred must declare Phase 21 stories complete"
    );
}

#[test]
fn fr177_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 110")
            && agents.contains("FR177")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 110 / FR177 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 110")
            && (spine.contains("已关闭") || spine.contains("110.3"))
            && spine.contains("FR177"),
        "spine must note Epic 110 / FR177 closed"
    );
    assert!(
        spine.contains("Epic 105–110")
            || spine.contains("全部已关闭")
            || spine.contains("规划+实现故事已齐")
            || spine.contains("规划故事已齐"),
        "spine must declare Phase 21 stories complete"
    );
}

#[test]
fn fr177_nfr14_epic110_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic110-phase21-claim-honesty-fr177.md",
    );
    for needle in [
        "- [x] **FR177 诚实面落地**",
        "- [x] **文档 / deferred / README / spine 收口**",
        "- [x] **NFR78/81/82：",
        "- [x] **品牌 / AD-6：",
        "- [x] **Phase 21 规划故事齐：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 110 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 110.3") || text.contains("可宣称"),
        "NFR14 must be closed after Story 110.3"
    );
}

#[test]
fn fr177_sprint_epic110_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-110: done") || sprint.contains("epic-110:done"),
        "epic-110 must be done"
    );
    assert!(
        sprint.contains("110-3-fr177-收口与-phase-21-故事清单指针: done")
            || sprint.contains("110-3-fr177-收口与-phase-21-故事清单指针:done"),
        "110-3 must be done"
    );
}

#[test]
fn fr177_epics_phase21_epic110_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase21Epic110Status: complete")
            || epics.contains("phase21Epic110Status:complete"),
        "epics.md must stamp phase21Epic110Status complete"
    );
}

#[test]
fn fr177_product_doc_closed() {
    let doc = read("docs/fr177-phase21-claim-honesty.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("110.3"),
        "fr177 product doc must note closed status"
    );
}
