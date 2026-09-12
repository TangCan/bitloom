//! ATDD / guardrail: Story 91.3 / FR159 — Epic 91 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr159_closeout_epic91
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
fn fr159_readme_marks_fr159_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR159")
            && (readme.contains("已关闭") || readme.contains("Epic 91"))
            && (readme.contains("MemRead") || readme.contains("fr159")),
        "README must mark FR159 / Epic 91 closed"
    );
    assert!(
        readme.contains("FR160")
            && (readme.contains("未关闭前不得宣称") || readme.contains("不得宣称")),
        "README must keep FR160+ unclaimed"
    );
    assert!(
        (readme.contains("全清") || readme.contains("NFR59"))
            && (readme.contains("不得") || readme.contains("禁止")),
        "README must forbid NFR59 fully-cleared claim"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr159_deferred_marks_epic91_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 91")
            && deferred.contains("FR159")
            && (deferred.contains("已关闭") || deferred.contains("Story 91.3")),
        "deferred must mark Epic 91 / FR159 closed"
    );
    assert!(
        deferred.contains("FR160") && deferred.contains("Epic 92"),
        "deferred must point remaining deepen to Epic 92+"
    );
}

#[test]
fn fr159_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 91")
            && agents.contains("FR159")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 91 / FR159 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 91")
            && (spine.contains("已关闭") || spine.contains("91.3"))
            && spine.contains("FR159"),
        "spine must note Epic 91 / FR159 closed"
    );
}

#[test]
fn fr159_nfr14_epic91_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic91-memread-full-emit-fr159.md");
    for needle in [
        "- [x] **FR159 缺口 G1–G3 关闭 + ATDD**",
        "- [x] **文档 / deferred / README 收口**",
        "- [x] **NFR68/71/72：**",
        "- [x] **品牌 / AD-6：**",
        "- [x] **其余 FR160–165：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 91 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 91.3"),
        "NFR14 risk record must be closed after Story 91.3"
    );
}

#[test]
fn fr159_sprint_epic91_done_92_ready() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-91: done") || sprint.contains("epic-91:done"),
        "epic-91 must be done"
    );
    assert!(
        sprint.contains("91-3-fr159-收口与文档指针: done")
            || sprint.contains("91-3-fr159-收口与文档指针:done"),
        "91-3 must be done"
    );
    assert!(
        sprint.contains("92-1-epic-92-nfr14-风险记录: ready-for-dev")
            || sprint.contains("92-1-epic-92-nfr14-风险记录:ready-for-dev")
            || sprint.contains("92-1-epic-92-nfr14-风险记录: done")
            || sprint.contains("92-1-epic-92-nfr14-风险记录: in-progress"),
        "92-1 must be ready-for-dev after Epic 91 closes"
    );
}

#[test]
fn fr159_epics_phase19_epic91_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase19Epic91Status: complete")
            || epics.contains("phase19Epic91Status:complete"),
        "epics.md must stamp phase19Epic91Status complete"
    );
}

#[test]
fn fr159_product_doc_closed_status() {
    let doc = read("docs/fr159-memread-full-emit.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("91.3"),
        "fr159 product doc must note closed status"
    );
}
