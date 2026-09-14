//! ATDD / guardrail: Story 117.3 / FR184 — Epic 117 + Phase 22 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr184_closeout_epic117
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
fn fr184_readme_marks_fr184_and_phase22_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR184")
            && (readme.contains("已关闭") || readme.contains("complete"))
            && readme.contains("fr184-phase22-claim-honesty"),
        "README must mark FR184 closed with doc pointer"
    );
    assert!(
        readme.contains("Phase 22")
            && (readme.contains("已关闭")
                || readme.contains("complete")
                || readme.contains("已齐")
                || readme.contains("Epic 111–117")),
        "README must mark Phase 22 closed / stories complete"
    );
    assert!(
        readme.contains("NFR86")
            && (readme.contains("新合同") || readme.contains("仍须") || readme.contains("超出")),
        "README must keep NFR86 leftovers honesty"
    );
    assert!(
        readme.contains("NFR81")
            && (readme.contains("账本已空") || readme.contains("已空"))
            && (readme.contains("不得") || readme.contains("禁止")),
        "README must forbid NFR81 ledger-empty claim"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr184_deferred_marks_epic117_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 117")
            && deferred.contains("FR184")
            && (deferred.contains("已关闭") || deferred.contains("Story 117.3")),
        "deferred must mark Epic 117 / FR184 closed"
    );
    assert!(
        deferred.contains("Epic 111–117")
            || deferred.contains("Epic 111-117")
            || deferred.contains("规划+实现故事已齐"),
        "deferred must declare Phase 22 stories complete"
    );
}

#[test]
fn fr184_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 117")
            && agents.contains("FR184")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 117 / FR184 closed"
    );
    assert!(
        agents.contains("Epic 111–117")
            || agents.contains("planning+implementation stories complete")
            || agents.contains("Phase 22 planning+implementation"),
        "AGENTS must declare Phase 22 stories complete"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 117")
            && (spine.contains("已关闭") || spine.contains("117.3"))
            && spine.contains("FR184"),
        "spine must note Epic 117 / FR184 closed"
    );
    assert!(
        spine.contains("Epic 111–117")
            || spine.contains("全部已关闭")
            || spine.contains("规划+实现故事已齐")
            || spine.contains("规划故事已齐"),
        "spine must declare Phase 22 stories complete"
    );
}

#[test]
fn fr184_nfr14_epic117_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic117-phase22-claim-honesty-fr184.md",
    );
    for needle in [
        "- [x] **FR184 诚实面落地**",
        "- [x] **文档 / deferred / README / spine 收口**",
        "- [x] **NFR83/86/87：",
        "- [x] **品牌 / AD-6：",
        "- [x] **Phase 22 规划故事齐：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 117 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 117.3") || text.contains("可宣称"),
        "NFR14 must be closed after Story 117.3"
    );
}

#[test]
fn fr184_sprint_epic117_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-117: done") || sprint.contains("epic-117:done"),
        "epic-117 must be done"
    );
    assert!(
        sprint.contains("117-3-fr184-收口与-phase-22-故事清单指针: done")
            || sprint.contains("117-3-fr184-收口与-phase-22-故事清单指针:done"),
        "117-3 must be done"
    );
}

#[test]
fn fr184_epics_phase22_epic117_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase22Epic117Status: complete")
            || epics.contains("phase22Epic117Status:complete"),
        "epics.md must stamp phase22Epic117Status complete"
    );
    assert!(
        epics.contains("phase22Status: complete")
            || epics.contains("Phase 22 planning+implementation stories complete"),
        "epics.md must declare Phase 22 complete"
    );
}

#[test]
fn fr184_product_doc_closed() {
    let doc = read("docs/fr184-phase22-claim-honesty.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("117.3"),
        "fr184 product doc must note closed status"
    );
    assert!(
        doc.contains("NFR86") || doc.contains("NFR81"),
        "must leave beyond-NFR14 / NFR81 honesty"
    );
}
