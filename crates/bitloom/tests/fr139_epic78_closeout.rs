//! ATDD Story 78.3 — FR139 / Epic 78 closeout + Phase 16 story-list pointer.
//!
//! ```text
//! cargo test -p bitloom --test fr139_epic78_closeout
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
fn fr139_closeout() {
    let nfr =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic78-vip-socpad-cross-crate.md");
    assert!(
        nfr.contains("- [x] **78.2 / FR139") && nfr.contains("closed"),
        "NFR14 must check 78.2/FR139 and mark closed"
    );
    let docs = read("docs/fr139-vip-socpad-split.md");
    assert!(
        docs.contains("closed") || docs.contains("已关闭"),
        "docs/fr139-* must declare FR139 closed"
    );
    let readme = read("README.md");
    assert!(
        readme.contains("FR139") && readme.contains("Epic 78 已关闭"),
        "README must declare Epic 78 / FR139 closed"
    );
    assert!(
        readme.contains("Phase 16")
            && (readme.contains("规划故事已齐") || readme.contains("Epic 72–78")),
        "README must keep Phase 16 planning story-list pointer (Epic 72–78)"
    );
    // Progress honesty: 72+78 done; 73–77 still open (not silently all-done)
    assert!(
        (readme.contains("Epic 72") && readme.contains("已关闭"))
            || readme.contains("闸门 Epic 72 / FR133 已关闭"),
        "README must keep Epic 72 gate closed"
    );
    assert!(
        readme.contains("73")
            && (readme.contains("仍须") || readme.contains("仍开") || readme.contains("still")),
        "README must not pretend Epic 73–77 are closed"
    );
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("FR139") && deferred.contains("已关闭"),
        "deferred must mark FR139 closed"
    );
    assert!(
        deferred.contains("NFR59")
            && (deferred.contains("更深") || deferred.contains("IP") || deferred.contains("布局")),
        "deferred/docs honesty: further IP layout remains NFR59"
    );
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-78: done") || sprint.contains("epic-78:done"),
        "sprint must mark epic-78 done"
    );
    assert!(
        sprint.contains("78-3-fr139-收口与-phase-16-故事清单指针: done")
            || sprint.contains("78-3-fr139-收口与-phase-16-故事清单指针:done"),
        "sprint must mark 78-3 done"
    );
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase16Epic78Status: complete"),
        "epics frontmatter must set phase16Epic78Status: complete"
    );
    // FR140 claim discipline + NFR59 further IP layout
    assert!(
        docs.contains("FR140")
            || docs.contains("NFR59")
            || readme.contains("FR140")
            || nfr.contains("NFR59"),
        "closeout surface must cite FR140 and/or NFR59"
    );
    assert!(
        docs.contains("FR131")
            || nfr.contains("FR131")
            || readme.contains("FR131")
            || docs.contains("≠ FR131"),
        "must keep FR131 isolation (split alone ≠ FR139 close without C1)"
    );
}
