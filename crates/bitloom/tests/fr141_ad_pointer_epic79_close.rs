//! ATDD / guardrail: Story 79.4 / FR141 — AD pointer + Epic 79 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr141_ad_pointer_epic79_close
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
fn fr141_spine_phase17_pointer_gate_closed() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Phase 17")
            && (spine.contains("FR141") || spine.contains("稳定门") || spine.contains("1.0")),
        "ARCHITECTURE-SPINE must declare Phase 17 API stability contract pointer"
    );
    assert!(
        spine.contains("NFR60")
            && (spine.contains("仍有效") || spine.contains("不得") || spine.contains("失败")),
        "spine must keep NFR60 Phase 12–16 isolation"
    );
    assert!(
        spine.contains("NFR62")
            && (spine.contains("SemVer")
                || spine.contains("semver")
                || spine.contains("CI")
                || spine.contains("publish")),
        "spine must require NFR62 policy/CI/publish honesty for Phase 17"
    );
    assert!(
        spine.contains("FR142") && spine.contains("FR146") && spine.contains("FR147"),
        "spine Deferred must point FR142–146 / FR147"
    );
    assert!(
        spine.contains("Epic 79")
            && (spine.contains("已关闭") || spine.contains("closed") || spine.contains("79.4")),
        "spine must note Epic 79 / FR141 gate closed"
    );
}

#[test]
fn fr141_agents_phase17_pointer() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Phase 17")
            && (agents.contains("FR141") || agents.contains("FR147") || agents.contains("1.0")),
        "AGENTS.md must carry Phase 17 / FR141 gate pointer"
    );
    assert!(
        agents.contains("NFR60") || agents.contains("Epic 79"),
        "AGENTS must note gate close / NFR60 isolation"
    );
    assert!(
        agents.contains("Bitloom") || agents.contains("bitloom"),
        "AGENTS must keep Bitloom brand"
    );
}

#[test]
fn fr141_nfr14_epic79_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic79-phase17-api-stability-1-0.md",
    );
    for needle in [
        "- [x] **FR141 / Correct Course + PRD：",
        "- [x] **README / deferred：",
        "- [x] **AD 指针：",
        "- [x] **NFR60–63：",
        "- [x] **禁止事项未触发：",
        "- [x] **品牌 / 依赖：",
        "- [x] **Epic 80–83：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 79 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 79.4") || text.contains("79.4")),
        "NFR14 Epic 79 record must be closed with Story 79.4 pointer"
    );
    assert!(
        text.contains("闸门已开") || text.contains("Phase 17 闸门"),
        "must declare Phase 17 gate opened"
    );
}

#[test]
fn fr141_sprint_epic79_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("79-4-ad-指针与-epic-79-收口-fr141-fr147-nfr62: done")
            || sprint.contains("79-4-ad-指针与-epic-79-收口-fr141-fr147-nfr62:done"),
        "79-4 must be done"
    );
    assert!(
        sprint.contains("epic-79: done") || sprint.contains("epic-79:done"),
        "epic-79 must be done"
    );
    for epic in ["epic-80:", "epic-81:", "epic-82:", "epic-83:"] {
        assert!(
            sprint.contains(epic),
            "expected seeded Phase 17 epic key containing {epic}"
        );
    }
    // Gate: Epic 80–83 must not be ready-for-dev *before* epic-79 is done.
    // After the gate closes they may leave backlog (NFR14 → ready → done).
    let epic79_done = sprint.contains("epic-79: done") || sprint.contains("epic-79:done");
    if !epic79_done {
        for line in sprint.lines() {
            let t = line.trim();
            if (t.starts_with("80-")
                || t.starts_with("81-")
                || t.starts_with("82-")
                || t.starts_with("83-"))
                && t.contains("ready-for-dev")
            {
                panic!("Epic 80–83 must not be ready-for-dev before epic-79 done: {t}");
            }
        }
    }
}

#[test]
fn fr141_epics_phase17_epic79_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase17Epic79Status: complete")
            || epics.contains("phase17Epic79Status:complete"),
        "epics.md must mark phase17Epic79Status complete"
    );
}
