//! ATDD Story 76.3 — FR137 / Epic 76 closeout + Phase 16 story-list pointer.
//!
//! ```text
//! cargo test -p bitloom --test fr137_epic76_closeout
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
fn fr137_closeout() {
    let nfr = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic76-external-circt-compile-sim-gate.md",
    );
    assert!(
        nfr.contains("- [x] **76.2 / FR137")
            && nfr.contains("- [x] **文档 / deferred")
            && nfr.contains("- [x] **禁止事项未触发")
            && nfr.contains("- [x] **品牌 / 依赖")
            && nfr.contains("- [x] **FR129 / FR121 / FR110 / FR95 / FR96")
            && nfr.contains("**closed**"),
        "NFR14 must check all five Epic 76 close conditions and mark closed"
    );
    assert!(
        nfr.contains("仿真门禁") && nfr.contains("NFR59"),
        "NFR14 must keep sim-gate deepen / broader CIRCT honesty under NFR59"
    );
    let docs = read("docs/fr137-external-circt-gate.md");
    assert!(
        docs.contains("Epic 76 / FR137 closed") || docs.contains("Epic 76 / FR137 已关闭"),
        "docs/fr137-* must declare Epic 76 / FR137 closed"
    );
    assert!(
        docs.contains("Not selected") || docs.contains("未选"),
        "docs/fr137-* must keep sim gate Not selected honesty"
    );
    assert!(
        docs.contains("Epic 77")
            && (docs.contains("已关闭") || docs.contains("closed"))
            && !docs.contains("Epic 77** 仍须")
            && !docs.contains("**Epic 77** 仍须"),
        "docs/fr137-* must declare Epic 77 closed (Phase 16 all-closed honesty)"
    );
    let readme = read("README.md");
    assert!(
        readme.contains("FR137") && readme.contains("Epic 76 已关闭"),
        "README must declare Epic 76 / FR137 closed"
    );
    assert!(
        !readme.contains("Epic 76–77")
            && !readme.contains("76–77")
            && !readme.contains("Epic 76-77")
            && !readme.contains("76-77"),
        "README must not keep Epic 76–77 in the still-open set"
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
    assert!(
        readme.contains("CIRCT")
            && (readme.contains("仿真") || readme.contains("MLIR") || readme.contains("lower")),
        "README NFR59 surface must cite broader CIRCT/sim deferral"
    );
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("FR137") && agents.contains("Epic 76") && agents.contains("closed"),
        "AGENTS.md must declare Epic 76 / FR137 closed"
    );
    assert!(
        agents.contains("FR138") && agents.contains("Epic 77") && agents.contains("closed"),
        "AGENTS.md must declare Epic 77 / FR138 closed"
    );
    assert!(
        !agents.contains("76–77") && !agents.contains("76-77"),
        "AGENTS.md must not keep Epic 76–77 as the still-open set"
    );
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("FR137") && deferred.contains("已关闭"),
        "deferred must mark FR137 closed"
    );
    assert!(
        deferred.contains("NFR59")
            && (deferred.contains("CIRCT")
                || deferred.contains("MLIR")
                || deferred.contains("lower")
                || deferred.contains("更广")
                || deferred.contains("未列")),
        "deferred/docs honesty: broader CIRCT/MLIR lower remain NFR59"
    );
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-76: done") || sprint.contains("epic-76:done"),
        "sprint must mark epic-76 done"
    );
    assert!(
        sprint.contains("76-3-fr137-收口与文档指针: done")
            || sprint.contains("76-3-fr137-收口与文档指针:done"),
        "sprint must mark 76-3 done"
    );
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase16Epic76Status: complete"),
        "epics frontmatter must set phase16Epic76Status: complete"
    );
    // FR129 C1–C4 still valid; broader CIRCT/MLIR = NFR59; FR140 claim discipline
    assert!(
        docs.contains("FR129")
            || nfr.contains("FR129")
            || readme.contains("FR129")
            || docs.contains("≠ FR129")
            || docs.contains("C1–C4")
            || docs.contains("C1-C4"),
        "must keep FR129 C1–C4 isolation (C1–C4 alone ≠ FR137)"
    );
    let fr129 = read("docs/fr129-circt-handshake.md");
    assert!(
        fr129.contains("FR137") && (fr129.contains("closed") || fr129.contains("已关闭")),
        "docs/fr129-* must point to FR137 closed"
    );
    assert!(
        docs.contains("NFR59")
            || readme.contains("NFR59")
            || deferred.contains("NFR59")
            || nfr.contains("NFR59"),
        "closeout surface must cite NFR59 for broader CIRCT/MLIR lower"
    );
    assert!(
        docs.contains("FR140")
            || readme.contains("FR140")
            || deferred.contains("FR140")
            || nfr.contains("FR140"),
        "closeout surface must cite FR140 claim discipline"
    );
}
