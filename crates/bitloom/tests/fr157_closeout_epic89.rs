//! ATDD / guardrail: Story 89.3 / FR157 — Epic 89 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr157_closeout_epic89
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
fn fr157_readme_marks_fr157_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR157")
            && (readme.contains("已关闭") || readme.contains("Epic 89"))
            && (readme.contains("bitloom::fsm")
                || readme.contains("FsmLabels")
                || readme.contains("fr157")),
        "README must mark FR157 / Epic 89 closed"
    );
    assert!(
        readme.contains("FR158")
            && (readme.contains("未关闭前不得宣称") || readme.contains("不得宣称")),
        "README must keep FR158+ unclaimed"
    );
    assert!(
        (readme.contains("全清") || readme.contains("NFR59"))
            && (readme.contains("不得") || readme.contains("禁止")),
        "README must forbid claiming NFR59 fully cleared via FR157 alone"
    );
    assert!(readme.contains("Bitloom") && readme.contains("bitloom"));
}

#[test]
fn fr157_deferred_marks_epic89_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 89")
            && deferred.contains("FR157")
            && (deferred.contains("已关闭") || deferred.contains("Story 89.3")),
        "deferred must mark Epic 89 / FR157 closed"
    );
    assert!(
        deferred.contains("FR158") && deferred.contains("Epic 90"),
        "deferred must point remaining deepen to Epic 90+"
    );
    assert!(
        (deferred.contains("全清") || deferred.contains("NFR59"))
            && (deferred.contains("不得") || deferred.contains("≠")),
        "deferred must state FR157 ≠ NFR59 fully cleared"
    );
}

#[test]
fn fr157_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 89")
            && agents.contains("FR157")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 89 / FR157 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 89")
            && (spine.contains("已关闭") || spine.contains("89.3"))
            && spine.contains("FR157"),
        "spine must note Epic 89 / FR157 closed"
    );
}

#[test]
fn fr157_nfr14_epic89_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic89-auto-fsm-labels-fr157.md");
    for needle in [
        "- [x] **FR157 钉死子集实现 + ATDD**",
        "- [x] **文档 / deferred / README 收口**",
        "- [x] **NFR68/71/72：**",
        "- [x] **品牌 / AD-6：**",
        "- [x] **其余 FR158–165：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 89 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 89.3"),
        "NFR14 risk record must be closed after Story 89.3"
    );
}

#[test]
fn fr157_sprint_epic89_done_90_ready() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-89: done") || sprint.contains("epic-89:done"),
        "epic-89 must be done"
    );
    assert!(
        sprint.contains("89-3-fr157-收口与文档指针: done")
            || sprint.contains("89-3-fr157-收口与文档指针:done"),
        "89-3 must be done"
    );
    assert!(
        sprint.contains("90-1-epic-90-nfr14-风险记录: ready-for-dev")
            || sprint.contains("90-1-epic-90-nfr14-风险记录:ready-for-dev")
            || sprint.contains("90-1-epic-90-nfr14-风险记录: done")
            || sprint.contains("90-1-epic-90-nfr14-风险记录: in-progress"),
        "90-1 must be ready-for-dev after Epic 89 closes"
    );
}

#[test]
fn fr157_epics_phase19_epic89_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase19Epic89Status: complete")
            || epics.contains("phase19Epic89Status:complete"),
        "epics.md must stamp phase19Epic89Status complete"
    );
}

#[test]
fn fr157_product_doc_closed_status() {
    let doc = read("docs/fr157-auto-fsm-labels.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("89.3"),
        "fr157 product doc must note closed status"
    );
}
