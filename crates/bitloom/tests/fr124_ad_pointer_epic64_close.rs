//! ATDD / guardrail: Story 64.4 / FR124 — AD pointer + Epic 64 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr124_ad_pointer_epic64_close
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
fn fr124_spine_phase15_pointer_gate_closed() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Phase 15")
            && (spine.contains("FR124") || spine.contains("NFR51") || spine.contains("剩余")),
        "ARCHITECTURE-SPINE must declare Phase 15 deepen contract pointer"
    );
    assert!(
        spine.contains("NFR52")
            && (spine.contains("仍有效") || spine.contains("不得") || spine.contains("失败")),
        "spine must keep NFR52 Phase 12–14 isolation"
    );
    assert!(
        spine.contains("NFR54")
            && spine.contains("AD-25")
            && spine.contains("AD-27")
            && (spine.contains("sby") || spine.contains("SBY") || spine.contains("FR127")),
        "spine must require NFR54 AD/CI sync for Phase 15 deepen epics"
    );
    assert!(
        spine.contains("FR129") && spine.contains("FR130") && spine.contains("FR127"),
        "spine Deferred must point FR129/130/127 deepen to AD / CI revisions"
    );
    assert!(
        spine.contains("Epic 64")
            && (spine.contains("已关闭") || spine.contains("closed") || spine.contains("64.4")),
        "spine must note Epic 64 / FR124 gate closed"
    );
}

#[test]
fn fr124_agents_phase15_pointer() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Phase 15")
            && (agents.contains("FR124") || agents.contains("FR132") || agents.contains("NFR51")),
        "AGENTS.md must carry Phase 15 / FR124 gate pointer"
    );
    assert!(
        agents.contains("NFR52") || agents.contains("Epic 64"),
        "AGENTS must note gate close / NFR52 isolation"
    );
    assert!(
        agents.contains("Bitloom") || agents.contains("bitloom"),
        "AGENTS must keep Bitloom brand"
    );
}

#[test]
fn fr124_nfr14_epic64_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic64-phase15-nfr51-leftover-deepen.md",
    );
    for needle in [
        "- [x] **FR124 / Correct Course + PRD：",
        "- [x] **README / deferred：",
        "- [x] **AD 指针：",
        "- [x] **NFR52–55：",
        "- [x] **禁止事项未触发：",
        "- [x] **品牌 / 依赖：",
        "- [x] **Epic 65–71：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 64 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 64.4") || text.contains("64.4")),
        "NFR14 Epic 64 record must be closed with Story 64.4 pointer"
    );
    assert!(
        text.contains("闸门已开") || text.contains("Phase 15 闸门"),
        "must declare Phase 15 gate opened"
    );
}

#[test]
fn fr124_sprint_epic64_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("64-4-ad-指针与-epic-64-收口-fr124-nfr54: done")
            || sprint.contains("64-4-ad-指针与-epic-64-收口-fr124-nfr54:done"),
        "64-4 must be done"
    );
    assert!(
        sprint.contains("epic-64: done") || sprint.contains("epic-64:done"),
        "epic-64 must be done"
    );
    for epic in [
        "epic-65:", "epic-66:", "epic-67:", "epic-68:", "epic-69:", "epic-70:", "epic-71:",
    ] {
        assert!(
            sprint.contains(epic),
            "expected seeded Phase 15 epic key containing {epic}"
        );
    }
    // After gate close, deepen epics remain backlog until each NFR14 (not ready yet)
    for line in sprint.lines() {
        let t = line.trim();
        if (t.starts_with("65-")
            || t.starts_with("66-")
            || t.starts_with("67-")
            || t.starts_with("68-")
            || t.starts_with("69-")
            || t.starts_with("70-")
            || t.starts_with("71-"))
            && t.contains("ready-for-dev")
        {
            // Allowed only after each epic's own create-story; at Epic 64 close they stay backlog.
            panic!("at Epic 64 close, deepen stories should still be backlog: {t}");
        }
    }
}

#[test]
fn fr124_epics_phase15_epic64_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase15Epic64Status: complete")
            || epics.contains("phase15Epic64Status:complete"),
        "epics.md must mark phase15Epic64Status complete"
    );
}
