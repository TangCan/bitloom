//! ATDD / guardrail: Story 103.3 / FR170 — Epic 103 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr170_closeout_epic103
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
fn fr170_readme_marks_fr170_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR170")
            && (readme.contains("已关闭") || readme.contains("Epic 103"))
            && (readme.contains("fr170")
                || readme.contains("update-mainline")
                || readme.contains("parseUpdateMainline")
                || readme.contains("Parser")),
        "README must mark FR170 / Epic 103 closed"
    );
    assert!(
        readme.contains("FR171")
            && (readme.contains("已关闭")
                || readme.contains("Epic 104")
                || readme.contains("fr171-phase20-claim-honesty")),
        "README must keep FR171 honesty / closed pointer"
    );
    assert!(
        readme.contains("NFR76")
            && (readme.contains("新合同") || readme.contains("仍须") || readme.contains("超出")),
        "README must keep NFR76 leftovers honesty"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr170_deferred_marks_epic103_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 103")
            && deferred.contains("FR170")
            && (deferred.contains("已关闭") || deferred.contains("Story 103.3")),
        "deferred must mark Epic 103 / FR170 closed"
    );
    assert!(
        deferred.contains("FR171") && deferred.contains("Epic 104"),
        "deferred must point remaining deepen to Epic 104"
    );
}

#[test]
fn fr170_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 103")
            && agents.contains("FR170")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 103 / FR170 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 103")
            && (spine.contains("已关闭") || spine.contains("103.3"))
            && spine.contains("FR170"),
        "spine must note Epic 103 / FR170 closed"
    );
}

#[test]
fn fr170_nfr14_epic103_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic103-chisel-head-parser-fr170.md",
    );
    for needle in [
        "- [x] **FR170 钉死子集实现 + 验收**",
        "- [x] **文档 / deferred / README / spine 收口**",
        "- [x] **NFR73/76：**",
        "- [x] **品牌 / AD-6：**",
        "- [x] **其余 FR171：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 103 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 103.3"),
        "NFR14 risk record must be closed after Story 103.3"
    );
}

#[test]
fn fr170_sprint_epic103_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-103: done") || sprint.contains("epic-103:done"),
        "epic-103 must be done"
    );
    assert!(
        sprint.contains("103-3-fr170-收口与文档指针: done")
            || sprint.contains("103-3-fr170-收口与文档指针:done"),
        "103-3 must be done"
    );
}

#[test]
fn fr170_epics_phase20_epic103_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase20Epic103Status: complete")
            || epics.contains("phase20Epic103Status:complete"),
        "epics.md must stamp phase20Epic103Status complete"
    );
}

#[test]
fn fr170_product_doc_closed() {
    let doc = read("docs/fr170-chisel-head-parser.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("103.3"),
        "fr170 product doc must note closed status"
    );
    assert!(
        (doc.contains("HEAD") || doc.contains("FR174"))
            && (doc.contains("NFR76")
                || doc.contains("FR174")
                || doc.contains("deferred")
                || doc.contains("仍")
                || doc.contains("remain")),
        "product doc must keep unpaired HEAD deferred (FR174 / NFR76 era)"
    );
}
