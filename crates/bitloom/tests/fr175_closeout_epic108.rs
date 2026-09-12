//! ATDD / guardrail: Story 108.3 / FR175 — Epic 108 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr175_closeout_epic108
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
fn fr175_readme_marks_fr175_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR175")
            && (readme.contains("已关闭") || readme.contains("Epic 108"))
            && (readme.contains("ir-sv") || readme.contains("fr175") || readme.contains("--ir-sv")),
        "README must mark FR175 / Epic 108 closed"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr175_deferred_marks_epic108_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 108")
            && deferred.contains("FR175")
            && (deferred.contains("已关闭") || deferred.contains("Story 108.3")),
        "deferred must mark Epic 108 / FR175 closed"
    );
    assert!(
        deferred.contains("FR176") && deferred.contains("Epic 109"),
        "deferred must point remaining deepen to Epic 109+"
    );
}

#[test]
fn fr175_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 108")
            && agents.contains("FR175")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 108 / FR175 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 108")
            && (spine.contains("已关闭") || spine.contains("108.3"))
            && spine.contains("FR175"),
        "spine must note Epic 108 / FR175 closed"
    );
}

#[test]
fn fr175_nfr14_epic108_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic108-broader-circt-mlir-sim-fr175.md",
    );
    for needle in [
        "- [x] **FR175 钉死子集实现 + 验收**",
        "- [x] **文档 / deferred / README / CI 收口**",
        "- [x] **NFR78/81：",
        "- [x] **品牌 / AD-6：",
        "- [x] **其余 FR176–177：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 108 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 108.3") || text.contains("可宣称"),
        "NFR14 must be closed after Story 108.3"
    );
}

#[test]
fn fr175_sprint_epic108_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-108: done") || sprint.contains("epic-108:done"),
        "epic-108 must be done"
    );
    assert!(
        sprint.contains("108-3-fr175-收口与文档指针: done")
            || sprint.contains("108-3-fr175-收口与文档指针:done"),
        "108-3 must be done"
    );
}

#[test]
fn fr175_epics_phase21_epic108_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase21Epic108Status: complete")
            || epics.contains("phase21Epic108Status:complete"),
        "epics.md must stamp phase21Epic108Status complete"
    );
}

#[test]
fn fr175_product_doc_closed() {
    let doc = read("docs/fr175-broader-circt-mlir-sim.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("108.3"),
        "fr175 product doc must note closed status"
    );
}
