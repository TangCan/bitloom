//! ATDD / guardrail: Story 100.3 / FR167 — Epic 100 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr167_closeout_epic100
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
fn fr167_readme_marks_fr167_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR167")
            && (readme.contains("已关闭") || readme.contains("Epic 100"))
            && (readme.contains("fr167") || readme.contains("ChiselSim") || readme.contains("IDE")),
        "README must mark FR167 / Epic 100 closed"
    );
    assert!(
        readme.contains("FR168")
            && (readme.contains("未关闭前不得宣称") || readme.contains("不得宣称")),
        "README must keep FR168+ unclaimed"
    );
    assert!(
        (readme.contains("NFR76") || readme.contains("NFR71"))
            && (readme.contains("不得") || readme.contains("禁止") || readme.contains("新合同")),
        "README must forbid claiming beyond closed FR167 subset"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr167_deferred_marks_epic100_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 100")
            && deferred.contains("FR167")
            && (deferred.contains("已关闭") || deferred.contains("Story 100.3")),
        "deferred must mark Epic 100 / FR167 closed"
    );
    assert!(
        deferred.contains("FR168") && deferred.contains("Epic 101"),
        "deferred must point remaining deepen to Epic 101+"
    );
}

#[test]
fn fr167_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 100")
            && agents.contains("FR167")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 100 / FR167 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 100")
            && (spine.contains("已关闭") || spine.contains("100.3"))
            && spine.contains("FR167"),
        "spine must note Epic 100 / FR167 closed"
    );
}

#[test]
fn fr167_nfr14_epic100_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic100-chiselsim-ide-stores-fr167.md",
    );
    for needle in [
        "- [x] **FR167 钉死子集实现 + 验收**",
        "- [x] **文档 / deferred / README 收口**",
        "- [x] **NFR73/75/76：**",
        "- [x] **品牌 / AD-6：**",
        "- [x] **其余 FR168–171：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 100 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 100.3"),
        "NFR14 risk record must be closed after Story 100.3"
    );
}

#[test]
fn fr167_sprint_epic100_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-100: done") || sprint.contains("epic-100:done"),
        "epic-100 must be done"
    );
    assert!(
        sprint.contains("100-3-fr167-收口与文档指针: done")
            || sprint.contains("100-3-fr167-收口与文档指针:done"),
        "100-3 must be done"
    );
}

#[test]
fn fr167_epics_phase20_epic100_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase20Epic100Status: complete")
            || epics.contains("phase20Epic100Status:complete"),
        "epics.md must stamp phase20Epic100Status complete"
    );
}

#[test]
fn fr167_product_doc_closed_status() {
    let doc = read("docs/fr167-chiselsim-ide-stores.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("100.3"),
        "fr167 product doc must note closed status"
    );
}
