//! ATDD / guardrail: Story 48.4 / FR106 — AD pointer + Epic 48 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr106_ad_pointer_epic48_close
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
fn fr106_spine_phase13_pointer() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Phase 13") && (spine.contains("FR106") || spine.contains("商业加深")),
        "ARCHITECTURE-SPINE must declare Phase 13 deepen contract pointer"
    );
    assert!(
        spine.contains("NFR44")
            && (spine.contains("仍有效") || spine.contains("不得") || spine.contains("失败")),
        "spine must keep NFR44 Phase 12 MVP isolation"
    );
    assert!(
        spine.contains("NFR46")
            && spine.contains("AD-5")
            && spine.contains("AD-25")
            && spine.contains("AD-27"),
        "spine must require NFR46 AD sync for deepen epics"
    );
    assert!(
        spine.contains("FR107") && spine.contains("FR110") && spine.contains("FR111"),
        "spine Deferred must point FR107/110/111 deepen to AD revisions"
    );
}

#[test]
fn fr106_nfr14_epic48_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic48-mvp-commercial-deepen.md");
    for needle in [
        "- [x] **FR106 / Correct Course + PRD：",
        "- [x] **README / deferred：",
        "- [x] **AD 指针：",
        "- [x] **NFR44–47：",
        "- [x] **禁止事项未触发：",
        "- [x] **品牌 / 依赖：",
        "- [x] **Epic 49–56：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 48 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 48.4") || text.contains("48.4")),
        "NFR14 Epic 48 record must be closed with Story 48.4 pointer"
    );
}

#[test]
fn fr106_sprint_epic48_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("48-4-ad-指针与-epic-48-收口-fr106: done")
            || sprint.contains("48-4-ad-指针与-epic-48-收口-fr106:done"),
        "48-4 must be done"
    );
    assert!(
        sprint.contains("epic-48: done") || sprint.contains("epic-48:done"),
        "epic-48 must be done"
    );
    // After Epic 48 close, Epic 49–56 may leave backlog under their own NFR14.
    assert!(
        sprint.contains("epic-49:") || sprint.contains("49-1-"),
        "Phase 13 deepen epics should remain seeded after Epic 48 close"
    );
}
