//! ATDD / guardrail: Story 102.3 / FR169 — Epic 102 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr169_closeout_epic102
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
fn fr169_readme_marks_fr169_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR169")
            && (readme.contains("已关闭") || readme.contains("Epic 102"))
            && (readme.contains("fr169") || readme.contains("multi-lower") || readme.contains("allocation")),
        "README must mark FR169 / Epic 102 closed"
    );
    assert!(
        readme.contains("FR170")
            && (readme.contains("未关闭前不得宣称") || readme.contains("不得宣称")),
        "README must keep FR170+ unclaimed"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr169_deferred_marks_epic102_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 102")
            && deferred.contains("FR169")
            && (deferred.contains("已关闭") || deferred.contains("Story 102.3")),
        "deferred must mark Epic 102 / FR169 closed"
    );
    assert!(
        deferred.contains("FR170") && deferred.contains("Epic 103"),
        "deferred must point remaining deepen to Epic 103+"
    );
}

#[test]
fn fr169_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 102")
            && agents.contains("FR169")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 102 / FR169 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 102")
            && (spine.contains("已关闭") || spine.contains("102.3"))
            && spine.contains("FR169"),
        "spine must note Epic 102 / FR169 closed"
    );
}

#[test]
fn fr169_nfr14_epic102_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic102-circt-mlir-firtool-fr169.md",
    );
    for needle in [
        "- [x] **FR169 钉死子集实现 + 验收**",
        "- [x] **文档 / deferred / README / spine 收口**",
        "- [x] **NFR73/76：**",
        "- [x] **品牌 / AD-6：**",
        "- [x] **其余 FR170–171：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 102 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 102.3"),
        "NFR14 risk record must be closed after Story 102.3"
    );
}

#[test]
fn fr169_sprint_epic102_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-102: done") || sprint.contains("epic-102:done"),
        "epic-102 must be done"
    );
    assert!(
        sprint.contains("102-3-fr169-收口与文档指针: done")
            || sprint.contains("102-3-fr169-收口与文档指针:done"),
        "102-3 must be done"
    );
}

#[test]
fn fr169_epics_phase20_epic102_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase20Epic102Status: complete")
            || epics.contains("phase20Epic102Status:complete"),
        "epics.md must stamp phase20Epic102Status complete"
    );
}

#[test]
fn fr169_product_doc_closed() {
    let doc = read("docs/fr169-circt-mlir-allocation.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("102.3"),
        "fr169 product doc must note closed status"
    );
    assert!(
        (doc.contains("bump") || doc.contains("升钉") || doc.contains("(B)"))
            && (doc.contains("Deferred")
                || doc.contains("deferred")
                || doc.contains("NFR76")
                || doc.contains("未")),
        "product doc must keep option B / firtool bump deferred"
    );
}
