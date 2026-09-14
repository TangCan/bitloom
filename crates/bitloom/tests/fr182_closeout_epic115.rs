//! ATDD / guardrail: Story 115.3 / FR182 — Epic 115 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr182_closeout_epic115
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
fn fr182_readme_marks_fr182_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR182")
            && (readme.contains("已关闭") || readme.contains("Epic 115"))
            && (readme.contains("1.159.0") || readme.contains("fr182")),
        "README must mark FR182 / Epic 115 closed"
    );
    assert!(
        !readme.contains(
            "FR182 / 115 | AD-9 *unpaired product-pin* 例外（≠ FR173/FR174）— **未关闭**"
        ),
        "README must not keep the open FR182 duplicate row"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr182_deferred_marks_epic115_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 115")
            && deferred.contains("FR182")
            && (deferred.contains("已关闭") || deferred.contains("Story 115.3")),
        "deferred must mark Epic 115 / FR182 closed"
    );
    assert!(
        deferred.contains("FR183") && deferred.contains("Epic 116"),
        "deferred must point remaining deepen to Epic 116+"
    );
}

#[test]
fn fr182_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 115")
            && agents.contains("FR182")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 115 / FR182 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 115")
            && (spine.contains("已关闭") || spine.contains("115.3"))
            && spine.contains("FR182"),
        "spine must note Epic 115 / FR182 closed"
    );
}

#[test]
fn fr182_nfr14_epic115_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic115-unpaired-firtool-product-pin-fr182.md",
    );
    for needle in [
        "- [x] **unpaired 产品钉再升钉 + 验收谓词：**",
        "- [x] **AD-9 *unpaired product-pin* 例外修订（NFR85）：**",
        "- [x] **docs/fr182-* + README/deferred：**",
        "- [x] **NFR83：**",
        "- [x] **禁止事项未触发：**",
        "- [x] **品牌 / 依赖：**",
        "- [x] **超子集：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 115 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 115.3") || text.contains("可宣称"),
        "NFR14 must be closed after Story 115.3"
    );
}

#[test]
fn fr182_sprint_epic115_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-115: done") || sprint.contains("epic-115:done"),
        "epic-115 must be done"
    );
    assert!(
        sprint.contains("115-3-fr182-收口与文档指针: done")
            || sprint.contains("115-3-fr182-收口与文档指针:done"),
        "115-3 must be done"
    );
}

#[test]
fn fr182_epics_phase22_epic115_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase22Epic115Status: complete")
            || epics.contains("phase22Epic115Status:complete"),
        "epics.md must stamp phase22Epic115Status complete"
    );
}

#[test]
fn fr182_product_doc_closed() {
    let doc = read("docs/fr182-unpaired-firtool-product-pin.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("115.3"),
        "fr182 product doc must note closed status"
    );
    assert!(
        doc.contains("NFR86") || doc.contains("Further bumps"),
        "must leave further unpaired bumps as NFR86"
    );
}
