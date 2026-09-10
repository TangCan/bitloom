//! ATDD Story 49.3 — FR107 / Epic 49 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr107_epic49_closeout
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
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr107_nfr14_epic49_close_conditions_checked() {
    let text = read("_agile-output/implementation-artifacts/nfr14-risk-epic49-systemc-tlm-at.md");
    for needle in [
        "- [x] **49.2 / FR107：",
        "- [x] **文档 / deferred：",
        "- [x] **禁止事项未触发",
        "- [x] **品牌 / 依赖：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 49 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 49.3") || text.contains("49.3")),
        "NFR14 must be closed with Story 49.3 pointer"
    );
}

#[test]
fn fr107_docs_readme_deferred_closed() {
    let fr107 = read("docs/fr107-systemc-tlm-at.md");
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        fr107.contains("closed") || fr107.contains("已关闭") || fr107.contains("49.3"),
        "fr107 doc must declare Epic 49 closed"
    );
    assert!(
        (readme.contains("FR107") || readme.contains("gen-tlm-at"))
            && (readme.contains("已关闭") || readme.contains("Epic 49")),
        "README must note FR107 / Epic 49 closed"
    );
    assert!(
        deferred.contains("FR107")
            && (deferred.contains("已关闭")
                || deferred.contains("closed")
                || deferred.contains("49.3")),
        "deferred must close AT / FR107 item"
    );
    assert!(
        fr107.contains("NFR47")
            || fr107.contains("Out of this FR")
            || fr107.contains("quantum")
            || fr107.contains("PEQ"),
        "fr107 must keep honest AT subset / NFR47 boundary"
    );
}

#[test]
fn fr107_sprint_epic49_done_lt_still_present() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("49-3-fr107-收口与文档边界: done")
            || sprint.contains("49-3-fr107-收口与文档边界:done")
    );
    assert!(sprint.contains("epic-49: done") || sprint.contains("epic-49:done"));
    assert!(
        sprint.contains("epic-46: done") || sprint.contains("46-3-"),
        "FR101 LT epic must remain closed (NFR44)"
    );
}

#[test]
fn fr107_ad5_and_lt_path_untouched_markers() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    let fr101 = read("docs/fr101-systemc-tlm.md");
    assert!(spine.contains("FR107") && spine.contains("nb_transport"));
    assert!(
        fr101.contains("LT-only") || fr101.contains("b_transport"),
        "FR101 LT docs must remain"
    );
}
