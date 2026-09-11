//! ATDD Story 77.3 — FR138 / Epic 77 closeout + Phase 16 story-list pointer.
//!
//! ```text
//! cargo test -p bitloom --test fr138_epic77_closeout
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
fn fr138_closeout() {
    let nfr =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic77-restore-parser-ad27.md");
    assert!(
        nfr.contains("- [x] **77.2 / FR138")
            && nfr.contains("- [x] **文档 / deferred")
            && nfr.contains("- [x] **禁止事项未触发")
            && nfr.contains("- [x] **品牌 / 依赖")
            && nfr.contains("- [x] **FR130 / FR122 / FR111 / FR97")
            && nfr.contains("**closed**"),
        "NFR14 must check all five Epic 77 close conditions and mark closed"
    );
    assert!(
        nfr.contains("NFR59")
            && (nfr.contains("Parser") || nfr.contains("Chisel") || nfr.contains("更深")),
        "NFR14 must keep deeper Chisel/Parser ecosystem honesty under NFR59"
    );
    let docs = read("docs/fr138-parser-restore.md");
    assert!(
        docs.contains("Epic 77 / FR138 closed") || docs.contains("Epic 77 / FR138 已关闭"),
        "docs/fr138-* must declare Epic 77 / FR138 closed"
    );
    assert!(
        docs.contains("FR130")
            && (docs.contains("仍有效")
                || docs.contains("remain")
                || docs.contains("still")
                || docs.contains("NFR56")),
        "docs/fr138-* must keep FR130 Style Guide still-valid honesty"
    );
    assert!(
        docs.contains("NFR59")
            && (docs.contains("更深")
                || docs.contains("Parser")
                || docs.contains("Chisel")
                || docs.contains("生态")),
        "docs/fr138-* must keep deeper Chisel/Parser ecosystem NFR59 honesty"
    );
    let readme = read("README.md");
    assert!(
        readme.contains("FR138") && readme.contains("Epic 77 已关闭"),
        "README must declare Epic 77 / FR138 closed"
    );
    assert!(
        !readme.contains("Epic 77 · FR138 仍须")
            && !readme.contains("Epic 77 仍须各自实现关闭")
            && !readme.contains("Epic 77 仍须"),
        "README must not keep Epic 77 in the still-open set"
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
        readme.contains("Epic 77 已关闭")
            && readme.contains("Epic 76 已关闭")
            && readme.contains("Epic 78 已关闭"),
        "README must declare Phase 16 implementation epics 76–78 closed with 77"
    );
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("FR138") && agents.contains("Epic 77") && agents.contains("closed"),
        "AGENTS.md must declare Epic 77 / FR138 closed"
    );
    assert!(
        !agents.contains("deepen epic **77** still needs")
            && !agents.contains("Epic 77 still need")
            && !agents.contains("77 still open"),
        "AGENTS.md must not keep Epic 77 still-open honesty"
    );
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("FR138") && deferred.contains("已关闭"),
        "deferred must mark FR138 closed"
    );
    assert!(
        deferred.contains("NFR59")
            && (deferred.contains("Parser")
                || deferred.contains("Chisel")
                || deferred.contains("更深")
                || deferred.contains("生态")
                || deferred.contains("未列")),
        "deferred/docs honesty: deeper Chisel/Parser ecosystem remain NFR59"
    );
    assert!(
        deferred.contains("Epic 77")
            && deferred.contains("已关闭")
            && !deferred.contains("Epic 77** 仍须")
            && !deferred.contains("**Epic 77** 仍须"),
        "deferred must mark Epic 77 closed (not still-open)"
    );
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-77: done") || sprint.contains("epic-77:done"),
        "sprint must mark epic-77 done"
    );
    assert!(
        sprint.contains("77-3-fr138-收口与文档指针: done")
            || sprint.contains("77-3-fr138-收口与文档指针:done"),
        "sprint must mark 77-3 done"
    );
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase16Epic77Status: complete"),
        "epics frontmatter must set phase16Epic77Status: complete"
    );
    // FR130 Style Guide still valid; deeper Parser = NFR59; FR140 claim discipline
    assert!(
        docs.contains("FR130")
            || nfr.contains("FR130")
            || readme.contains("FR130")
            || docs.contains("≠ FR130")
            || docs.contains("Style Guide"),
        "must keep FR130 Style Guide isolation (Style Guide alone ≠ FR138)"
    );
    let fr130 = read("docs/fr130-style-guide.md");
    assert!(
        fr130.contains("FR138") && (fr130.contains("closed") || fr130.contains("已关闭")),
        "docs/fr130-* must point to FR138 closed"
    );
    assert!(
        docs.contains("NFR59")
            || readme.contains("NFR59")
            || deferred.contains("NFR59")
            || nfr.contains("NFR59"),
        "closeout surface must cite NFR59 for deeper Chisel/Parser ecosystem"
    );
    assert!(
        docs.contains("FR140")
            || readme.contains("FR140")
            || deferred.contains("FR140")
            || nfr.contains("FR140"),
        "closeout surface must cite FR140 claim discipline"
    );
}
