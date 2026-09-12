//! ATDD / guardrail: Story 92.3 / FR160 — Epic 92 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr160_closeout_epic92
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
fn fr160_readme_marks_fr160_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR160")
            && (readme.contains("已关闭") || readme.contains("Epic 92"))
            && (readme.contains("discover_design_roots_under")
                || readme.contains("非 Cargo")
                || readme.contains("fr160")),
        "README must mark FR160 / Epic 92 closed"
    );
    assert!(
        readme.contains("FR161")
            && (readme.contains("未关闭前不得宣称") || readme.contains("不得宣称")),
        "README must keep FR161+ unclaimed"
    );
    assert!(
        (readme.contains("全清") || readme.contains("NFR59"))
            && (readme.contains("不得") || readme.contains("禁止")),
        "README must forbid NFR59 fully-cleared claim"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr160_deferred_marks_epic92_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 92")
            && deferred.contains("FR160")
            && (deferred.contains("已关闭") || deferred.contains("Story 92.3")),
        "deferred must mark Epic 92 / FR160 closed"
    );
    assert!(
        deferred.contains("FR161") && deferred.contains("Epic 93"),
        "deferred must point remaining deepen to Epic 93+"
    );
}

#[test]
fn fr160_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 92")
            && agents.contains("FR160")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 92 / FR160 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 92")
            && (spine.contains("已关闭") || spine.contains("92.3"))
            && spine.contains("FR160"),
        "spine must note Epic 92 / FR160 closed"
    );
}

#[test]
fn fr160_nfr14_epic92_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic92-non-cargo-path-scan-fr160.md",
    );
    for needle in [
        "- [x] **FR160 钉死子集实现 + 验收**",
        "- [x] **文档 / deferred / README 收口**",
        "- [x] **NFR68/71/72：**",
        "- [x] **品牌 / AD-6：**",
        "- [x] **其余 FR161–165：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 92 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 92.3"),
        "NFR14 risk record must be closed after Story 92.3"
    );
}

#[test]
fn fr160_sprint_epic92_done_93_ready() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-92: done") || sprint.contains("epic-92:done"),
        "epic-92 must be done"
    );
    assert!(
        sprint.contains("92-3-fr160-收口与文档指针: done")
            || sprint.contains("92-3-fr160-收口与文档指针:done"),
        "92-3 must be done"
    );
    assert!(
        sprint.contains("93-1-epic-93-nfr14-风险记录: ready-for-dev")
            || sprint.contains("93-1-epic-93-nfr14-风险记录:ready-for-dev")
            || sprint.contains("93-1-epic-93-nfr14-风险记录: done")
            || sprint.contains("93-1-epic-93-nfr14-风险记录: in-progress"),
        "93-1 must be ready-for-dev after Epic 92 closes"
    );
}

#[test]
fn fr160_epics_phase19_epic92_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase19Epic92Status: complete")
            || epics.contains("phase19Epic92Status:complete"),
        "epics.md must stamp phase19Epic92Status complete"
    );
}

#[test]
fn fr160_product_doc_closed_status() {
    let doc = read("docs/fr160-non-cargo-path-scan.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("92.3"),
        "fr160 product doc must note closed status"
    );
}
