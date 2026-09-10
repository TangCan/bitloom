//! ATDD / guardrail: Story 57.4 / FR116 — AD pointer + Epic 57 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr116_ad_pointer_epic57_close
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
fn fr116_spine_phase14_pointer() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Phase 14")
            && (spine.contains("FR116") || spine.contains("NFR47") || spine.contains("未选加深")),
        "ARCHITECTURE-SPINE must declare Phase 14 deepen contract pointer"
    );
    assert!(
        spine.contains("NFR48")
            && (spine.contains("仍有效") || spine.contains("不得") || spine.contains("失败")),
        "spine must keep NFR48 Phase 12/13 isolation"
    );
    assert!(
        spine.contains("NFR50")
            && spine.contains("AD-25")
            && spine.contains("AD-27")
            && (spine.contains("formal") || spine.contains("SBY") || spine.contains("FR119")),
        "spine must require NFR50 AD sync for Phase 14 deepen epics"
    );
    assert!(
        spine.contains("FR121") && spine.contains("FR122") && spine.contains("FR119"),
        "spine Deferred must point FR121/122/119 deepen to AD / formal revisions"
    );
}

#[test]
fn fr116_nfr14_epic57_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic57-phase14-nfr47-deferred-deepen.md",
    );
    for needle in [
        "- [x] **FR116 / Correct Course + PRD：",
        "- [x] **README / deferred：",
        "- [x] **AD 指针：",
        "- [x] **NFR48–51：",
        "- [x] **禁止事项未触发：",
        "- [x] **品牌 / 依赖：",
        "- [x] **Epic 58–63：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 57 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 57.4") || text.contains("57.4")),
        "NFR14 Epic 57 record must be closed with Story 57.4 pointer"
    );
}

#[test]
fn fr116_sprint_epic57_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("57-4-ad-指针与-epic-57-收口-fr116-nfr50: done")
            || sprint.contains("57-4-ad-指针与-epic-57-收口-fr116-nfr50:done"),
        "57-4 must be done"
    );
    assert!(
        sprint.contains("epic-57: done") || sprint.contains("epic-57:done"),
        "epic-57 must be done"
    );
    // After Epic 57 close, Epic 58–63 may leave permanent freeze under their own NFR14.
    // Keep them seeded; do not over-freeze forever as backlog-only.
    assert!(
        sprint.contains("epic-58:") || sprint.contains("58-1-"),
        "Phase 14 deepen epics should remain seeded after Epic 57 close"
    );
    for epic in [
        "epic-58:", "epic-59:", "epic-60:", "epic-61:", "epic-62:", "epic-63:",
    ] {
        assert!(
            sprint.contains(epic),
            "expected seeded Phase 14 epic key containing {epic}"
        );
    }
}
