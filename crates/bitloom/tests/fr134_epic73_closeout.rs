//! ATDD Story 73.3 — FR134 / Epic 73 closeout + Phase 16 story-list pointer.
//!
//! ```text
//! cargo test -p bitloom --test fr134_epic73_closeout
//! ```

use std::fs;
use std::path::PathBuf;

fn read(rel: &str) -> String {
    fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(rel),
    )
    .unwrap()
}

#[test]
fn fr134_closeout() {
    let nfr = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic73-upstream-tywaves-gui-ide.md",
    );
    assert!(
        nfr.contains("- [x] **73.2 / FR134") && nfr.contains("closed"),
        "NFR14 must check 73.2/FR134 and mark closed"
    );
    let docs = read("docs/fr134-upstream-tywaves-gui-ide.md");
    assert!(
        docs.contains("closed") || docs.contains("已关闭"),
        "docs/fr134-* must declare FR134 closed"
    );
    let readme = read("README.md");
    assert!(
        readme.contains("FR134") && readme.contains("Epic 73 已关闭"),
        "README must declare Epic 73 / FR134 closed"
    );
    assert!(
        readme.contains("Phase 16")
            && (readme.contains("规划故事已齐") || readme.contains("Epic 72–78")),
        "README must keep Phase 16 planning story-list pointer (Epic 72–78)"
    );
    // Progress honesty: 72+73+78 done; 74–77 still open (not silently all-done)
    assert!(
        (readme.contains("Epic 72") && readme.contains("已关闭"))
            || readme.contains("闸门 Epic 72 / FR133 已关闭"),
        "README must keep Epic 72 gate closed"
    );
    assert!(
        readme.contains("74")
            && (readme.contains("仍须") || readme.contains("仍开") || readme.contains("still")),
        "README must not pretend Epic 74–77 are closed"
    );
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("FR134") && deferred.contains("已关闭"),
        "deferred must mark FR134 closed"
    );
    assert!(
        deferred.contains("NFR59")
            && (deferred.contains("更深") || deferred.contains("GUI") || deferred.contains("IDE")),
        "deferred/docs honesty: further GUI/IDE subsets remain NFR59"
    );
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-73: done") || sprint.contains("epic-73:done"),
        "sprint must mark epic-73 done"
    );
    assert!(
        sprint.contains("73-3-fr134-收口与文档指针: done")
            || sprint.contains("73-3-fr134-收口与文档指针:done"),
        "sprint must mark 73-3 done"
    );
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase16Epic73Status: complete"),
        "epics frontmatter must set phase16Epic73Status: complete"
    );
    // FR125 T1–T4 still valid; deeper GUI/IDE = NFR59; FR140 claim discipline
    assert!(
        docs.contains("FR125")
            || nfr.contains("FR125")
            || readme.contains("FR125")
            || docs.contains("≠ FR125"),
        "must keep FR125 T1–T4 isolation (sidecar alone ≠ FR134)"
    );
    assert!(
        docs.contains("NFR59")
            || readme.contains("NFR59")
            || deferred.contains("NFR59")
            || nfr.contains("NFR59"),
        "closeout surface must cite NFR59 for deeper GUI/IDE"
    );
    assert!(
        docs.contains("FR140")
            || readme.contains("FR140")
            || deferred.contains("FR140")
            || nfr.contains("FR140"),
        "closeout surface must cite FR140 claim discipline"
    );
}
