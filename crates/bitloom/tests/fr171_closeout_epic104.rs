//! ATDD / guardrail: Story 104.3 / FR171 — Epic 104 + Phase 20 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr171_closeout_epic104
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
fn fr171_readme_marks_fr171_and_phase20_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR171")
            && (readme.contains("已关闭") || readme.contains("complete"))
            && readme.contains("fr171-phase20-claim-honesty"),
        "README must mark FR171 closed with doc pointer"
    );
    assert!(
        readme.contains("Phase 20")
            && (readme.contains("已关闭")
                || readme.contains("complete")
                || readme.contains("已齐")
                || readme.contains("Epic 99–104")),
        "README must mark Phase 20 closed / stories complete"
    );
    assert!(
        readme.contains("NFR76")
            && (readme.contains("新合同") || readme.contains("仍须") || readme.contains("超出")),
        "README must keep NFR76 leftovers honesty"
    );
    assert!(
        readme.contains("NFR71")
            && (readme.contains("账本已空") || readme.contains("已空"))
            && (readme.contains("不得") || readme.contains("禁止")),
        "README must forbid NFR71 ledger-empty claim"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr171_deferred_marks_epic104_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 104")
            && deferred.contains("FR171")
            && (deferred.contains("已关闭") || deferred.contains("Story 104.3")),
        "deferred must mark Epic 104 / FR171 closed"
    );
    assert!(
        deferred.contains("Epic 99–104") || deferred.contains("Epic 99-104"),
        "deferred must declare Phase 20 stories complete"
    );
}

#[test]
fn fr171_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 104")
            && agents.contains("FR171")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 104 / FR171 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 104")
            && (spine.contains("已关闭") || spine.contains("104.3"))
            && spine.contains("FR171"),
        "spine must note Epic 104 / FR171 closed"
    );
    assert!(
        spine.contains("Epic 99–104")
            || spine.contains("全部已关闭")
            || spine.contains("规划故事已齐"),
        "spine must declare Phase 20 stories complete"
    );
}

#[test]
fn fr171_nfr14_epic104_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic104-phase20-claim-honesty-fr171.md",
    );
    for needle in [
        "- [x] **FR171 诚实面落地**",
        "- [x] **文档 / deferred / README / spine 收口**",
        "- [x] **NFR73/76/77：**",
        "- [x] **品牌 / AD-6：**",
        "- [x] **Phase 20 规划故事齐：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 104 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 104.3"),
        "NFR14 risk record must be closed after Story 104.3"
    );
}

#[test]
fn fr171_sprint_epic104_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-104: done") || sprint.contains("epic-104:done"),
        "epic-104 must be done"
    );
    assert!(
        sprint.contains("104-3-fr171-收口与-phase-20-故事清单指针: done")
            || sprint.contains("104-3-fr171-收口与-phase-20-故事清单指针:done"),
        "104-3 must be done"
    );
}

#[test]
fn fr171_epics_phase20_epic104_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase20Epic104Status: complete")
            || epics.contains("phase20Epic104Status:complete"),
        "epics.md must stamp phase20Epic104Status complete"
    );
    assert!(
        epics.contains("phase20Status: complete") || epics.contains("phase20Status:complete"),
        "epics.md must stamp phase20Status complete"
    );
}

#[test]
fn fr171_product_doc_closed_status() {
    let doc = read("docs/fr171-phase20-claim-honesty.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("104.3"),
        "fr171 product doc must note closed status"
    );
}
