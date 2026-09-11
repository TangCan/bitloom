//! ATDD Story 74.3 — FR135 / Epic 74 closeout + Phase 16 story-list pointer.
//!
//! ```text
//! cargo test -p bitloom --test fr135_epic74_closeout
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
fn fr135_closeout() {
    let nfr =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic74-more-ip-handwritten-fl.md");
    assert!(
        nfr.contains("- [x] **74.2 / FR135") && nfr.contains("closed"),
        "NFR14 must check 74.2/FR135 and mark closed"
    );
    let docs = read("docs/fr135-more-ip-handwritten-fl.md");
    assert!(
        docs.contains("closed") || docs.contains("已关闭"),
        "docs/fr135-* must declare FR135 closed"
    );
    assert!(
        docs.contains("Epic 77")
            && (docs.contains("已关闭") || docs.contains("closed"))
            && !docs.contains("Epic 77** 仍须")
            && !docs.contains("**Epic 77** 仍须"),
        "docs/fr135-* must declare Epic 77 closed (Phase 16 all-closed honesty)"
    );
    let readme = read("README.md");
    assert!(
        readme.contains("FR135") && readme.contains("Epic 74 已关闭"),
        "README must declare Epic 74 / FR135 closed"
    );
    assert!(
        !readme.contains("Epic 74–77 · FR135–FR138 仍须")
            && !readme.contains("Epic 74–77 仍须各自实现关闭"),
        "README must not keep Epic 74 in the still-open set"
    );
    assert!(
        readme.contains("Phase 16")
            && (readme.contains("规划故事已齐") || readme.contains("Epic 72–78")),
        "README must keep Phase 16 planning story-list pointer (Epic 72–78)"
    );
    // Progress honesty: Epic 72–78 all closed for planning stories
    assert!(
        (readme.contains("Epic 72") && readme.contains("已关闭"))
            || readme.contains("闸门 Epic 72 / FR133 已关闭"),
        "README must keep Epic 72 gate closed"
    );
    assert!(
        readme.contains("Epic 77 已关闭"),
        "README must declare Epic 77 closed with Phase 16 all-closed honesty"
    );
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("FR135") && agents.contains("Epic 74") && agents.contains("closed"),
        "AGENTS.md must declare Epic 74 / FR135 closed"
    );
    assert!(
        agents.contains("FR138") && agents.contains("Epic 77") && agents.contains("closed"),
        "AGENTS.md must declare Epic 77 / FR138 closed"
    );
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("FR135") && deferred.contains("已关闭"),
        "deferred must mark FR135 closed"
    );
    assert!(
        deferred.contains("NFR59")
            && (deferred.contains("协议")
                || deferred.contains("IP")
                || deferred.contains("未列入")),
        "deferred/docs honesty: non-listed protocols remain NFR59"
    );
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-74: done") || sprint.contains("epic-74:done"),
        "sprint must mark epic-74 done"
    );
    assert!(
        sprint.contains("74-3-fr135-收口与文档指针: done")
            || sprint.contains("74-3-fr135-收口与文档指针:done"),
        "sprint must mark 74-3 done"
    );
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase16Epic74Status: complete"),
        "epics frontmatter must set phase16Epic74Status: complete"
    );
    // FR126 Gpio still valid; non-listed protocols = NFR59; FR140 claim discipline
    assert!(
        docs.contains("FR126")
            || nfr.contains("FR126")
            || readme.contains("FR126")
            || docs.contains("≠ FR126")
            || docs.contains("Gpio"),
        "must keep FR126 Gpio isolation (Gpio alone ≠ FR135)"
    );
    let fr126 = read("docs/fr126-more-ip-handwritten-fl.md");
    assert!(
        fr126.contains("FR135") && (fr126.contains("closed") || fr126.contains("已关闭")),
        "docs/fr126-* must point to FR135 closed"
    );
    assert!(
        docs.contains("NFR59")
            || readme.contains("NFR59")
            || deferred.contains("NFR59")
            || nfr.contains("NFR59"),
        "closeout surface must cite NFR59 for non-listed protocols"
    );
    assert!(
        docs.contains("FR140")
            || readme.contains("FR140")
            || deferred.contains("FR140")
            || nfr.contains("FR140"),
        "closeout surface must cite FR140 claim discipline"
    );
}
