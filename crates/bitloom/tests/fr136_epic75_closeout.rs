//! ATDD Story 75.3 — FR136 / Epic 75 closeout + Phase 16 story-list pointer.
//!
//! ```text
//! cargo test -p bitloom --test fr136_epic75_closeout
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
fn fr136_closeout() {
    let nfr = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic75-multi-peripheral-full-chip-pad-ring.md",
    );
    assert!(
        nfr.contains("- [x] **75.2 / FR136") && nfr.contains("closed"),
        "NFR14 must check 75.2/FR136 and mark closed"
    );
    let docs = read("docs/fr136-multi-peripheral-full-chip-pad-ring.md");
    assert!(
        docs.contains("closed") || docs.contains("已关闭"),
        "docs/fr136-* must declare FR136 closed"
    );
    assert!(
        docs.contains("Epic 77")
            && (docs.contains("已关闭") || docs.contains("closed"))
            && !docs.contains("Epic 77** 仍须")
            && !docs.contains("**Epic 77** 仍须"),
        "docs/fr136-* must declare Epic 77 closed (Phase 16 all-closed honesty)"
    );
    let readme = read("README.md");
    assert!(
        readme.contains("FR136") && readme.contains("Epic 75 已关闭"),
        "README must declare Epic 75 / FR136 closed"
    );
    assert!(
        !readme.contains("Epic 75–77 · FR136–FR138 仍须")
            && !readme.contains("Epic 75–77 仍须各自实现关闭"),
        "README must not keep Epic 75 in the still-open set"
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
        agents.contains("FR136") && agents.contains("Epic 75") && agents.contains("closed"),
        "AGENTS.md must declare Epic 75 / FR136 closed"
    );
    assert!(
        agents.contains("FR138") && agents.contains("Epic 77") && agents.contains("closed"),
        "AGENTS.md must declare Epic 77 / FR138 closed"
    );
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("FR136") && deferred.contains("已关闭"),
        "deferred must mark FR136 closed"
    );
    assert!(
        deferred.contains("NFR59")
            && (deferred.contains("pad")
                || deferred.contains("外设")
                || deferred.contains("未列")
                || deferred.contains("更广")),
        "deferred/docs honesty: broader pad/peripherals remain NFR59"
    );
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-75: done") || sprint.contains("epic-75:done"),
        "sprint must mark epic-75 done"
    );
    assert!(
        sprint.contains("75-3-fr136-收口与文档指针: done")
            || sprint.contains("75-3-fr136-收口与文档指针:done"),
        "sprint must mark 75-3 done"
    );
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase16Epic75Status: complete"),
        "epics frontmatter must set phase16Epic75Status: complete"
    );
    // FR128 GpioSocPad still valid; broader pad = NFR59; FR140 claim discipline
    assert!(
        docs.contains("FR128")
            || nfr.contains("FR128")
            || readme.contains("FR128")
            || docs.contains("≠ FR128")
            || docs.contains("GpioSocPad"),
        "must keep FR128 GpioSocPad isolation (GpioSocPad alone ≠ FR136)"
    );
    let fr128 = read("docs/fr128-soc-pad.md");
    assert!(
        fr128.contains("FR136") && (fr128.contains("closed") || fr128.contains("已关闭")),
        "docs/fr128-* must point to FR136 closed"
    );
    assert!(
        docs.contains("NFR59")
            || readme.contains("NFR59")
            || deferred.contains("NFR59")
            || nfr.contains("NFR59"),
        "closeout surface must cite NFR59 for broader pad/peripherals"
    );
    assert!(
        docs.contains("FR140")
            || readme.contains("FR140")
            || deferred.contains("FR140")
            || nfr.contains("FR140"),
        "closeout surface must cite FR140 claim discipline"
    );
}
