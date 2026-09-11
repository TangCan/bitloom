//! ATDD / guardrail: Story 84.4 / FR148 — AD pointer + Epic 84 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr148_ad_pointer_epic84_close
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
fn fr148_spine_phase18_pointer_gate_closed() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Phase 18")
            && (spine.contains("FR148") || spine.contains("CLI") || spine.contains("crates.io")),
        "ARCHITECTURE-SPINE must declare Phase 18 CLI publishability contract pointer"
    );
    assert!(
        spine.contains("NFR64")
            && (spine.contains("仍有效") || spine.contains("不得") || spine.contains("失败")),
        "spine must keep NFR64 Phase 12–17 isolation"
    );
    assert!(
        spine.contains("AD-2")
            && (spine.contains("bitloom-firrtl")
                || spine.contains("bitloom-viz")
                || spine.contains("bitloom-*")),
        "spine must cite AD-2 bitloom-* publish names for Phase 18"
    );
    assert!(
        spine.contains("NFR66") || spine.contains("NFR67"),
        "spine must require NFR66/NFR67 honesty for Phase 18"
    );
    assert!(
        spine.contains("FR149") && spine.contains("FR151") && spine.contains("FR153"),
        "spine Deferred must point FR149–153"
    );
    assert!(
        spine.contains("Epic 84")
            && (spine.contains("已关闭") || spine.contains("closed") || spine.contains("84.4")),
        "spine must note Epic 84 / FR148 gate closed"
    );
}

#[test]
fn fr148_agents_phase18_pointer() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Phase 18")
            && (agents.contains("FR148") || agents.contains("FR153") || agents.contains("CLI")),
        "AGENTS.md must carry Phase 18 / FR148 gate pointer"
    );
    assert!(
        agents.contains("NFR64") || agents.contains("Epic 84"),
        "AGENTS must note gate close / NFR64 isolation"
    );
    assert!(
        agents.contains("Bitloom") || agents.contains("bitloom"),
        "AGENTS must keep Bitloom brand"
    );
    assert!(
        agents.contains("AD-2") || agents.contains("bitloom-*") || agents.contains("NFR66"),
        "AGENTS must keep AD-2 / bitloom-* publish identity"
    );
}

#[test]
fn fr148_nfr14_epic84_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic84-phase18-cli-crates-io-publish.md",
    );
    for needle in [
        "- [x] **FR148 / Correct Course + PRD：",
        "- [x] **README / deferred：",
        "- [x] **AD 指针：",
        "- [x] **NFR64–67：",
        "- [x] **禁止事项未触发：",
        "- [x] **品牌 / 依赖：",
        "- [x] **Epic 85–86：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 84 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 84.4") || text.contains("84.4")),
        "NFR14 Epic 84 record must be closed with Story 84.4 pointer"
    );
    assert!(
        text.contains("闸门已开") || text.contains("Phase 18 闸门"),
        "must declare Phase 18 gate opened"
    );
}

#[test]
fn fr148_sprint_epic84_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("84-4-ad-指针与-epic-84-收口-fr148-nfr66: done")
            || sprint.contains("84-4-ad-指针与-epic-84-收口-fr148-nfr66:done"),
        "84-4 must be done"
    );
    assert!(
        sprint.contains("epic-84: done") || sprint.contains("epic-84:done"),
        "epic-84 must be done"
    );
    for epic in ["epic-85:", "epic-86:"] {
        assert!(
            sprint.contains(epic),
            "expected seeded Phase 18 epic key containing {epic}"
        );
    }
    // After gate closes, 85.1 may be ready-for-dev; 85.2+ must stay backlog until 85.1 NFR14.
    for line in sprint.lines() {
        let t = line.trim();
        if (t.starts_with("85-2-")
            || t.starts_with("85-3-")
            || t.starts_with("85-4-")
            || t.starts_with("85-5-")
            || t.starts_with("85-6-")
            || t.starts_with("86-"))
            && t.contains("ready-for-dev")
        {
            panic!("85.2+ / Epic 86 must stay backlog until per-epic NFR14: {t}");
        }
    }
}

#[test]
fn fr148_epics_phase18_epic84_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase18Epic84Status: complete")
            || epics.contains("phase18Epic84Status:complete"),
        "epics.md must mark phase18Epic84Status complete"
    );
}
