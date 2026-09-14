//! ATDD / guardrail: Story 111.4 / FR178 — AD pointer + Epic 111 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr178_ad_pointer_epic111_close
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
fn fr178_spine_phase22_pointer_gate_closed() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Phase 22") && (spine.contains("FR178") || spine.contains("NFR81")),
        "ARCHITECTURE-SPINE must declare Phase 22 contract pointer"
    );
    assert!(
        spine.contains("NFR83")
            && (spine.contains("仍有效") || spine.contains("不得") || spine.contains("失败")),
        "spine must keep NFR83 Phase 12–21 isolation"
    );
    assert!(
        spine.contains("FR179") && spine.contains("FR184"),
        "spine Deferred must point FR179–184"
    );
    assert!(
        spine.contains("Epic 111")
            && (spine.contains("已关闭") || spine.contains("closed") || spine.contains("111.4")),
        "spine must note Epic 111 / FR178 gate closed"
    );
    assert!(
        spine.contains("NFR85")
            || spine.contains("AD-9")
            || spine.contains("AD-25")
            || spine.contains("AD-27")
            || spine.contains("FR142"),
        "spine must note AD/surface touch honesty (NFR85) for Phase 22 deepen"
    );
}

#[test]
fn fr178_agents_phase22_pointer() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Phase 22")
            && (agents.contains("FR178") || agents.contains("FR184") || agents.contains("NFR81")),
        "AGENTS.md must carry Phase 22 / FR178 gate pointer"
    );
    assert!(
        agents.contains("NFR83") || agents.contains("Epic 111"),
        "AGENTS must note gate close / NFR83 isolation"
    );
    assert!(
        agents.contains("Bitloom") || agents.contains("bitloom"),
        "AGENTS must keep Bitloom brand"
    );
}

#[test]
fn fr178_nfr14_epic111_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic111-phase22-nfr81-leftovers.md",
    );
    for needle in [
        "- [x] **FR178 / Correct Course + PRD：",
        "- [x] **README / deferred：",
        "- [x] **AD 指针：",
        "- [x] **NFR83–87：",
        "- [x] **禁止事项未触发：",
        "- [x] **品牌 / 依赖：",
        "- [x] **Epic 112–117：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 111 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 111.4") || text.contains("闸门已开"),
        "NFR14 risk record must be closed after Story 111.4"
    );
}

#[test]
fn fr178_sprint_epic111_done_112_ready() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-111: done") || sprint.contains("epic-111:done"),
        "epic-111 must be done after Story 111.4"
    );
    assert!(
        sprint.contains("111-4-ad-指针与-epic-111-收口-fr178: done")
            || sprint.contains("111-4-ad-指针与-epic-111-收口-fr178:done"),
        "111-4 must be done"
    );
    assert!(
        sprint.contains("112-1-epic-112-nfr14-风险记录: ready-for-dev")
            || sprint.contains("112-1-epic-112-nfr14-风险记录:ready-for-dev")
            || sprint.contains("112-1-epic-112-nfr14-风险记录: done")
            || sprint.contains("112-1-epic-112-nfr14-风险记录: in-progress"),
        "112-1 must be ready-for-dev (or further) after Epic 111 closes"
    );
}

#[test]
fn fr178_epics_phase22_epic111_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase22Epic111Status: complete")
            || epics.contains("phase22Epic111Status:complete")
            || (epics.contains("Epic 111") && epics.contains("correctCoursePhase22Approved")),
        "epics.md must reflect Epic 111 / Phase 22 gate complete"
    );
}
