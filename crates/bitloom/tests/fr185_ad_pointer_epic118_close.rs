//! ATDD / guardrail: Story 118.4 / FR185 — AD pointer + Epic 118 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr185_ad_pointer_epic118_close
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
fn fr185_spine_phase23_pointer_gate_closed() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Phase 23") && (spine.contains("FR185") || spine.contains("NFR86")),
        "ARCHITECTURE-SPINE must declare Phase 23 contract pointer"
    );
    assert!(
        spine.contains("NFR88")
            && (spine.contains("仍有效") || spine.contains("不得") || spine.contains("失败")),
        "spine must keep NFR88 Phase 12–22 / closeout isolation"
    );
    assert!(
        spine.contains("FR186") && spine.contains("FR191"),
        "spine Deferred must point FR186–191"
    );
    assert!(
        spine.contains("Epic 118")
            && (spine.contains("已关闭") || spine.contains("closed") || spine.contains("118.4")),
        "spine must note Epic 118 / FR185 gate closed"
    );
    assert!(
        spine.contains("NFR90")
            || spine.contains("AD-9")
            || spine.contains("AD-25")
            || spine.contains("AD-27")
            || spine.contains("FR142"),
        "spine must note AD/surface touch honesty (NFR90) for Phase 23 deepen"
    );
}

#[test]
fn fr185_agents_phase23_pointer() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Phase 23")
            && (agents.contains("FR185") || agents.contains("FR191") || agents.contains("NFR86")),
        "AGENTS.md must carry Phase 23 / FR185 gate pointer"
    );
    assert!(
        agents.contains("NFR88") || agents.contains("Epic 118"),
        "AGENTS must note gate close / NFR88 isolation"
    );
    assert!(
        agents.contains("Bitloom") || agents.contains("bitloom"),
        "AGENTS must keep Bitloom brand"
    );
}

#[test]
fn fr185_nfr14_epic118_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic118-phase23-nfr86-leftovers.md",
    );
    for needle in [
        "- [x] **FR185 / Correct Course + PRD：",
        "- [x] **README / deferred：",
        "- [x] **AD 指针：",
        "- [x] **NFR88–92：",
        "- [x] **禁止事项未触发：",
        "- [x] **品牌 / 依赖：",
        "- [x] **Epic 119–124：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 118 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 118.4") || text.contains("闸门已开"),
        "NFR14 risk record must be closed after Story 118.4"
    );
}

#[test]
fn fr185_sprint_epic118_done_119_ready() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-118: done") || sprint.contains("epic-118:done"),
        "epic-118 must be done after Story 118.4"
    );
    assert!(
        sprint.contains("118-4-ad-指针与-epic-118-收口-fr185: done")
            || sprint.contains("118-4-ad-指针与-epic-118-收口-fr185:done"),
        "118-4 must be done"
    );
    assert!(
        sprint.contains("119-1-epic-119-nfr14-风险记录: ready-for-dev")
            || sprint.contains("119-1-epic-119-nfr14-风险记录:ready-for-dev")
            || sprint.contains("119-1-epic-119-nfr14-风险记录: done")
            || sprint.contains("119-1-epic-119-nfr14-风险记录: in-progress"),
        "119-1 must be ready-for-dev (or further) after Epic 118 closes"
    );
}

#[test]
fn fr185_epics_phase23_epic118_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase23Epic118Status: complete")
            || epics.contains("phase23Epic118Status:complete")
            || (epics.contains("Epic 118") && epics.contains("correctCoursePhase23Approved")),
        "epics.md must reflect Epic 118 / Phase 23 gate complete"
    );
}
