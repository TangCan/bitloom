//! ATDD / guardrail: Story 87.4 / FR154 — AD pointer + Epic 87 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr154_ad_pointer_epic87_close
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
fn fr154_spine_phase19_pointer_gate_closed() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Phase 19")
            && (spine.contains("FR154") || spine.contains("NFR59") || spine.contains("FR152")),
        "ARCHITECTURE-SPINE must declare Phase 19 contract pointer"
    );
    assert!(
        spine.contains("NFR68")
            && (spine.contains("仍有效") || spine.contains("不得") || spine.contains("失败")),
        "spine must keep NFR68 Phase 12–18 isolation"
    );
    assert!(
        spine.contains("FR155") && spine.contains("FR156") && spine.contains("FR157"),
        "spine Deferred must point FR155–157+"
    );
    assert!(
        spine.contains("Epic 87")
            && (spine.contains("已关闭") || spine.contains("closed") || spine.contains("87.4")),
        "spine must note Epic 87 / FR154 gate closed"
    );
    assert!(
        spine.contains("NFR70") || spine.contains("AD-25") || spine.contains("AD-27"),
        "spine must note AD touch honesty for Phase 19 deepen"
    );
}

#[test]
fn fr154_agents_phase19_pointer() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Phase 19")
            && (agents.contains("FR154") || agents.contains("FR165") || agents.contains("NFR59")),
        "AGENTS.md must carry Phase 19 / FR154 gate pointer"
    );
    assert!(
        agents.contains("NFR68") || agents.contains("Epic 87"),
        "AGENTS must note gate close / NFR68 isolation"
    );
    assert!(
        agents.contains("Bitloom") || agents.contains("bitloom"),
        "AGENTS must keep Bitloom brand"
    );
}

#[test]
fn fr154_nfr14_epic87_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic87-phase19-nfr59-fr152a.md");
    for needle in [
        "- [x] **FR154 / Correct Course + PRD：",
        "- [x] **README / deferred：",
        "- [x] **AD 指针：",
        "- [x] **NFR68–72：",
        "- [x] **禁止事项未触发：",
        "- [x] **品牌 / 依赖：",
        "- [x] **Epic 88–98：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 87 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 87.4") || text.contains("闸门已开"),
        "NFR14 risk record must be closed after Story 87.4"
    );
}

#[test]
fn fr154_sprint_epic87_done_88_ready() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-87: done") || sprint.contains("epic-87:done"),
        "epic-87 must be done after Story 87.4"
    );
    assert!(
        sprint.contains("87-4-ad-指针与-epic-87-收口-fr154: done")
            || sprint.contains("87-4-ad-指针与-epic-87-收口-fr154:done"),
        "87-4 must be done"
    );
    assert!(
        sprint.contains("88-1-epic-88-nfr14-风险记录: ready-for-dev")
            || sprint.contains("88-1-epic-88-nfr14-风险记录:ready-for-dev")
            || sprint.contains("88-1-epic-88-nfr14-风险记录: done")
            || sprint.contains("88-1-epic-88-nfr14-风险记录: in-progress"),
        "88-1 must be ready-for-dev (or further) after Epic 87 closes"
    );
}

#[test]
fn fr154_epics_phase19_epic87_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase19Epic87Status: complete")
            || epics.contains("phase19Epic87Status:complete")
            || (epics.contains("Epic 87") && epics.contains("correctCoursePhase19Approved")),
        "epics.md must reflect Epic 87 / Phase 19 gate complete"
    );
}
