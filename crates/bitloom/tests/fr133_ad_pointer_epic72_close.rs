//! ATDD / guardrail: Story 72.4 / FR133 — AD pointer + Epic 72 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr133_ad_pointer_epic72_close
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
fn fr133_spine_phase16_pointer_gate_closed() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Phase 16")
            && (spine.contains("FR133") || spine.contains("NFR55") || spine.contains("终局")),
        "ARCHITECTURE-SPINE must declare Phase 16 final-closeout contract pointer"
    );
    assert!(
        spine.contains("NFR56")
            && (spine.contains("仍有效") || spine.contains("不得") || spine.contains("失败")),
        "spine must keep NFR56 Phase 12–15 isolation"
    );
    assert!(
        spine.contains("NFR58")
            && spine.contains("AD-27")
            && (spine.contains("CIRCT") || spine.contains("FR137"))
            && (spine.contains("crate") || spine.contains("FR139")),
        "spine must require NFR58 AD/toolchain sync for Phase 16 deepen epics"
    );
    assert!(
        spine.contains("FR138") && spine.contains("FR137") && spine.contains("FR139"),
        "spine Deferred must point FR138/137/139 deepen to AD / CIRCT / crate revisions"
    );
    assert!(
        spine.contains("Epic 72")
            && (spine.contains("已关闭") || spine.contains("closed") || spine.contains("72.4")),
        "spine must note Epic 72 / FR133 gate closed"
    );
}

#[test]
fn fr133_agents_phase16_pointer() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Phase 16")
            && (agents.contains("FR133") || agents.contains("FR140") || agents.contains("NFR55")),
        "AGENTS.md must carry Phase 16 / FR133 gate pointer"
    );
    assert!(
        agents.contains("NFR56") || agents.contains("Epic 72"),
        "AGENTS must note gate close / NFR56 isolation"
    );
    assert!(
        agents.contains("Bitloom") || agents.contains("bitloom"),
        "AGENTS must keep Bitloom brand"
    );
}

#[test]
fn fr133_nfr14_epic72_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic72-phase16-nfr55-final-closeout.md",
    );
    for needle in [
        "- [x] **FR133 / Correct Course + PRD：",
        "- [x] **README / deferred：",
        "- [x] **AD 指针：",
        "- [x] **NFR56–59：",
        "- [x] **禁止事项未触发：",
        "- [x] **品牌 / 依赖：",
        "- [x] **Epic 73–78：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 72 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 72.4") || text.contains("72.4")),
        "NFR14 Epic 72 record must be closed with Story 72.4 pointer"
    );
    assert!(
        text.contains("闸门已开") || text.contains("Phase 16 闸门"),
        "must declare Phase 16 gate opened"
    );
}

#[test]
fn fr133_sprint_epic72_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("72-4-ad-指针与-epic-72-收口-fr133-fr140-nfr58: done")
            || sprint.contains("72-4-ad-指针与-epic-72-收口-fr133-fr140-nfr58:done"),
        "72-4 must be done"
    );
    assert!(
        sprint.contains("epic-72: done") || sprint.contains("epic-72:done"),
        "epic-72 must be done"
    );
    for epic in [
        "epic-73:", "epic-74:", "epic-75:", "epic-76:", "epic-77:", "epic-78:",
    ] {
        assert!(
            sprint.contains(epic),
            "expected seeded Phase 16 epic key containing {epic}"
        );
    }
    // After gate close, deepen epics remain backlog until each NFR14 (not ready yet)
    for line in sprint.lines() {
        let t = line.trim();
        if (t.starts_with("73-")
            || t.starts_with("74-")
            || t.starts_with("75-")
            || t.starts_with("76-")
            || t.starts_with("77-")
            || t.starts_with("78-"))
            && t.contains("ready-for-dev")
        {
            panic!("at Epic 72 close, deepen stories should still be backlog: {t}");
        }
    }
}

#[test]
fn fr133_epics_phase16_epic72_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase16Epic72Status: complete")
            || epics.contains("phase16Epic72Status:complete"),
        "epics.md must mark phase16Epic72Status complete"
    );
}
