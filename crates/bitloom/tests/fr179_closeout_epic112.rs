//! ATDD / guardrail: Story 112.3 / FR179 — Epic 112 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr179_closeout_epic112
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
fn fr179_readme_marks_fr179_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR179")
            && (readme.contains("已关闭") || readme.contains("Epic 112"))
            && (readme.contains("1.159.0") || readme.contains("fr179")),
        "README must mark FR179 / Epic 112 closed"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr179_deferred_marks_epic112_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 112")
            && deferred.contains("FR179")
            && (deferred.contains("已关闭") || deferred.contains("Story 112.3")),
        "deferred must mark Epic 112 / FR179 closed"
    );
    assert!(
        deferred.contains("FR180") && deferred.contains("Epic 113"),
        "deferred must point remaining deepen to Epic 113+"
    );
}

#[test]
fn fr179_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 112")
            && agents.contains("FR179")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 112 / FR179 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 112")
            && (spine.contains("已关闭") || spine.contains("112.3"))
            && spine.contains("FR179"),
        "spine must note Epic 112 / FR179 closed"
    );
}

#[test]
fn fr179_nfr14_epic112_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic112-floating-circt-git-head-fr179.md",
    );
    for needle in [
        "- [x] **浮动 HEAD 通道 + 验收谓词：**",
        "- [x] **AD-9 修订（NFR85）：**",
        "- [x] **docs/fr179-* + README/deferred：**",
        "- [x] **NFR83：**",
        "- [x] **禁止事项未触发：**",
        "- [x] **品牌 / 依赖：**",
        "- [x] **超子集：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 112 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 112.3") || text.contains("可宣称"),
        "NFR14 must be closed after Story 112.3"
    );
}

#[test]
fn fr179_sprint_epic112_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-112: done") || sprint.contains("epic-112:done"),
        "epic-112 must be done"
    );
    assert!(
        sprint.contains("112-3-fr179-收口与文档指针: done")
            || sprint.contains("112-3-fr179-收口与文档指针:done"),
        "112-3 must be done"
    );
}

#[test]
fn fr179_epics_phase22_epic112_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase22Epic112Status: complete")
            || epics.contains("phase22Epic112Status:complete"),
        "epics.md must stamp phase22Epic112Status complete"
    );
}

#[test]
fn fr179_product_doc_closed() {
    let doc = read("docs/fr179-floating-circt-git-head.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("112.3"),
        "fr179 product doc must note closed status"
    );
    assert!(
        doc.contains("FR186") || doc.contains("unbounded") || doc.contains("live tip"),
        "must leave unbounded tip as FR186 honesty"
    );
}
