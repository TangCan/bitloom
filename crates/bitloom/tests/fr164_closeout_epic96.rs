//! ATDD / guardrail: Story 96.3 / FR164 — Epic 96 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr164_closeout_epic96
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
fn fr164_readme_marks_fr164_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR164")
            && (readme.contains("已关闭") || readme.contains("Epic 96"))
            && (readme.contains("fr164")
                || readme.contains("仿真")
                || readme.contains("sim")
                || readme.contains("CIRCT")),
        "README must mark FR164 / Epic 96 closed"
    );
    assert!(
        readme.contains("FR156")
            && (readme.contains("未关闭前不得宣称")
                || readme.contains("不得宣称")
                || readme.contains("Epic 98")),
        "README must keep FR156 claim honesty open"
    );
    assert!(
        (readme.contains("全清") || readme.contains("NFR59"))
            && (readme.contains("不得") || readme.contains("禁止")),
        "README must forbid NFR59 fully-cleared claim"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr164_deferred_marks_epic96_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 96")
            && deferred.contains("FR164")
            && (deferred.contains("已关闭") || deferred.contains("Story 96.3")),
        "deferred must mark Epic 96 / FR164 closed"
    );
    assert!(
        deferred.contains("FR165") && deferred.contains("Epic 97"),
        "deferred must point remaining deepen to Epic 97+"
    );
}

#[test]
fn fr164_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 96")
            && agents.contains("FR164")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 96 / FR164 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 96")
            && (spine.contains("已关闭") || spine.contains("96.3"))
            && spine.contains("FR164"),
        "spine must note Epic 96 / FR164 closed"
    );
}

#[test]
fn fr164_nfr14_epic96_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic96-broader-circt-mlir-sim-gate-fr164.md",
    );
    for needle in [
        "- [x] **FR164 钉死子集实现 + 验收**",
        "- [x] **文档 / deferred / README / spine 收口**",
        "- [x] **NFR68/70/71/72：**",
        "- [x] **品牌 / AD-6：**",
        "- [x] **其余 FR165：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 96 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 96.3"),
        "NFR14 risk record must be closed after Story 96.3"
    );
}

#[test]
fn fr164_sprint_epic96_done_97_ready() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-96: done") || sprint.contains("epic-96:done"),
        "epic-96 must be done"
    );
    assert!(
        sprint.contains("96-3-fr164-收口与文档指针: done")
            || sprint.contains("96-3-fr164-收口与文档指针:done"),
        "96-3 must be done"
    );
    assert!(
        sprint.contains("97-1-epic-97-nfr14-风险记录: ready-for-dev")
            || sprint.contains("97-1-epic-97-nfr14-风险记录:ready-for-dev")
            || sprint.contains("97-1-epic-97-nfr14-风险记录: done")
            || sprint.contains("97-1-epic-97-nfr14-风险记录: in-progress"),
        "97-1 must be ready-for-dev after Epic 96 closes"
    );
}

#[test]
fn fr164_epics_phase19_epic96_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase19Epic96Status: complete")
            || epics.contains("phase19Epic96Status:complete"),
        "epics.md must stamp phase19Epic96Status complete"
    );
}

#[test]
fn fr164_product_doc_closed_status() {
    let doc = read("docs/fr164-circt-external-sim-gate.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("96.3"),
        "fr164 product doc must note closed status"
    );
}
