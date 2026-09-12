//! ATDD / guardrail: Story 105.4 / FR172 — AD pointer + Epic 105 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr172_ad_pointer_epic105_close
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
fn fr172_spine_phase21_pointer_gate_closed() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Phase 21") && (spine.contains("FR172") || spine.contains("NFR76")),
        "ARCHITECTURE-SPINE must declare Phase 21 contract pointer"
    );
    assert!(
        spine.contains("NFR78")
            && (spine.contains("仍有效") || spine.contains("不得") || spine.contains("失败")),
        "spine must keep NFR78 Phase 12–20 isolation"
    );
    assert!(
        spine.contains("FR173") && spine.contains("FR177"),
        "spine Deferred must point FR173–177"
    );
    assert!(
        spine.contains("Epic 105")
            && (spine.contains("已关闭") || spine.contains("closed") || spine.contains("105.4")),
        "spine must note Epic 105 / FR172 gate closed"
    );
    assert!(
        spine.contains("NFR80") || spine.contains("AD-9") || spine.contains("AD-27"),
        "spine must note AD touch honesty (NFR80) for Phase 21 deepen"
    );
}

#[test]
fn fr172_agents_phase21_pointer() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Phase 21")
            && (agents.contains("FR172") || agents.contains("FR177") || agents.contains("NFR76")),
        "AGENTS.md must carry Phase 21 / FR172 gate pointer"
    );
    assert!(
        agents.contains("NFR78") || agents.contains("Epic 105"),
        "AGENTS must note gate close / NFR78 isolation"
    );
    assert!(
        agents.contains("Bitloom") || agents.contains("bitloom"),
        "AGENTS must keep Bitloom brand"
    );
}

#[test]
fn fr172_nfr14_epic105_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic105-phase21-nfr76-leftovers.md",
    );
    for needle in [
        "- [x] **FR172 / Correct Course + PRD：",
        "- [x] **README / deferred：",
        "- [x] **AD 指针：",
        "- [x] **NFR78–82：",
        "- [x] **禁止事项未触发：",
        "- [x] **品牌 / 依赖：",
        "- [x] **Epic 106–110：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 105 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 105.4") || text.contains("闸门已开"),
        "NFR14 risk record must be closed after Story 105.4"
    );
}

#[test]
fn fr172_sprint_epic105_done_106_ready() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-105: done") || sprint.contains("epic-105:done"),
        "epic-105 must be done after Story 105.4"
    );
    assert!(
        sprint.contains("105-4-ad-指针与-epic-105-收口-fr172: done")
            || sprint.contains("105-4-ad-指针与-epic-105-收口-fr172:done"),
        "105-4 must be done"
    );
    assert!(
        sprint.contains("106-1-epic-106-nfr14-风险记录: ready-for-dev")
            || sprint.contains("106-1-epic-106-nfr14-风险记录:ready-for-dev")
            || sprint.contains("106-1-epic-106-nfr14-风险记录: done")
            || sprint.contains("106-1-epic-106-nfr14-风险记录: in-progress"),
        "106-1 must be ready-for-dev (or further) after Epic 105 closes"
    );
}

#[test]
fn fr172_epics_phase21_epic105_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase21Epic105Status: complete")
            || epics.contains("phase21Epic105Status:complete")
            || (epics.contains("Epic 105") && epics.contains("correctCoursePhase21Approved")),
        "epics.md must reflect Epic 105 / Phase 21 gate complete"
    );
}
