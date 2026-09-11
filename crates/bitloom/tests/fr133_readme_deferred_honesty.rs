//! ATDD / guardrail: Story 72.3 / FR133+FR140 — README / deferred /
//! doc-19 honesty surface for Phase 16 NFR55 final-closeout deepen.
//!
//! ```text
//! cargo test -p bitloom --test fr133_readme_deferred_honesty
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
fn fr133_readme_distinguishes_phase15_closed_vs_phase16() {
    let readme = read("README.md");
    assert!(
        readme.contains("Phase 15")
            && (readme.contains("FR124") || readme.contains("NFR51") || readme.contains("FR125"))
            && (readme.contains("已关闭") || readme.contains("Epic 64")),
        "README must keep Phase 15 deepen closed framing"
    );
    assert!(
        readme.contains("Phase 16")
            && (readme.contains("NFR55") || readme.contains("FR133") || readme.contains("终局")),
        "README must declare Phase 16 NFR55 final-closeout deepen contract"
    );
    assert!(
        readme.contains("NFR56")
            || (readme.contains("不得")
                && readme.contains("失败")
                && (readme.contains("Phase 15") || readme.contains("Phase 12"))),
        "README must keep NFR56 Phase 12–15 isolation honesty"
    );
}

#[test]
fn fr133_readme_lists_phase16_deepen_fr_table() {
    let readme = read("README.md");
    for fr in ["FR134", "FR135", "FR136", "FR137", "FR138", "FR139"] {
        assert!(
            readme.contains(fr),
            "README Phase 16 deepen table must list {fr}"
        );
    }
}

#[test]
fn fr133_readme_fr140_claim_discipline() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR140"),
        "README must cite FR140 claim discipline"
    );
    let needles = [
        "Tywaves",
        "IP FL",
        "pad",
        "CIRCT",
        "Parser",
        "跨 crate",
        "终局",
    ];
    let hit = needles.iter().filter(|n| readme.contains(**n)).count();
    assert!(
        hit >= 5,
        "README must name Phase 16 claim topics (got {hit}/7 of {needles:?})"
    );
    assert!(
        readme.contains("关闭") && (readme.contains("宣称") || readme.contains("勾选")),
        "README must state claims only after corresponding FR closes"
    );
    assert!(
        readme.contains("不得")
            && (readme.contains("FR125")
                || readme.contains("FR125–131")
                || readme.contains("alone")
                || readme.contains("冒充")),
        "must forbid writing FR125–131 alone as Phase 16 / final complete face"
    );
}

#[test]
fn fr133_readme_lists_nfr59_deferred() {
    let readme = read("README.md");
    assert!(
        readme.contains("NFR59"),
        "README must cite NFR59 deferred honesty"
    );
    let needles = ["FSM", "LCOV", "MemRead", "monorepo", "formal-sby"];
    let hit = needles.iter().filter(|n| readme.contains(**n)).count();
    assert!(
        hit >= 3,
        "README must honestly name NFR59 deferred topics (got {hit}/5 of {needles:?})"
    );
}

#[test]
fn fr133_deferred_phase16_pointer() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Phase 16") && (deferred.contains("FR133") || deferred.contains("NFR55")),
        "deferred-work must have Phase 16 pointer"
    );
    for fr in ["FR134", "FR135", "FR136", "FR137", "FR138", "FR139"] {
        assert!(
            deferred.contains(fr),
            "deferred must map upgraded items to {fr}"
        );
    }
    assert!(
        deferred.contains("NFR56")
            || deferred.contains("仍有效")
            || (deferred.contains("Phase 15") && deferred.contains("已关闭")),
        "deferred must keep Phase 12–15 closes valid"
    );
    assert!(
        deferred.contains("NFR59"),
        "deferred Phase 16 path must list NFR59 honesty"
    );
    assert!(
        deferred.contains("Bitloom") || deferred.contains("bitloom"),
        "deferred Phase 16 path must keep Bitloom brand visible in tree"
    );
}

#[test]
fn fr133_keeps_phase12_15_close_evidence() {
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        readme.contains("Phase 12")
            && (readme.contains("FR94") || readme.contains("字面绿") || readme.contains("Epic 40")),
        "README must retain Phase 12 close evidence"
    );
    assert!(
        readme.contains("Phase 13") && readme.contains("FR106"),
        "README must retain Phase 13 close evidence"
    );
    assert!(
        readme.contains("Phase 14") && readme.contains("FR116"),
        "README must retain Phase 14 close evidence"
    );
    assert!(
        readme.contains("Phase 15") && readme.contains("FR124"),
        "README must retain Phase 15 close evidence"
    );
    assert!(
        deferred.contains("Phase 15 pointer") || deferred.contains("FR124–132"),
        "deferred must retain Phase 15 pointer"
    );
}

#[test]
fn fr133_doc19_cross_link_phase16_without_redefining_mvp() {
    let doc19 = read("docs/requirements/19. 实施路线图.md");
    assert!(
        doc19.contains("Phase 16") || doc19.contains("FR133"),
        "doc-19 should cross-link Phase 16 final-closeout contract"
    );
    assert!(
        doc19.contains("字面绿完成定义") && doc19.contains("FR94"),
        "doc-19 must retain Phase 12 literal-green MVP definitions"
    );
}

#[test]
fn fr133_sprint_epic73_to_78_seeded_not_ready_while_gate_open() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("72-2-correct-course-prd-批准-phase-16-fr133: done")
            || sprint.contains("72-2-correct-course-prd-批准-phase-16-fr133:done"),
        "Story 72.2 must be done before 72.3 honesty surface closes"
    );
    for epic in 73..=78 {
        let key = format!("epic-{epic}:");
        assert!(
            sprint.lines().any(|l| l.trim_start().starts_with(&key)),
            "Epic {epic} must remain seeded during Phase 16 gate stories"
        );
    }
    // Hard gate: while epic-72 not done, deepen stories must not be ready-for-dev
    let epic72_done = sprint.contains("epic-72: done") || sprint.contains("epic-72:done");
    if !epic72_done {
        for line in sprint.lines() {
            let t = line.trim();
            if (t.starts_with("73-")
                || t.starts_with("74-")
                || t.starts_with("75-")
                || t.starts_with("76-")
                || t.starts_with("77-")
                || t.starts_with("78-"))
                && t.contains("ready-for-dev")
            {
                panic!("Epic 73–78 must not be ready-for-dev before epic-72 done: {t}");
            }
        }
    }
}

#[test]
fn fr133_readme_bitloom_brand() {
    let readme = read("README.md");
    assert!(
        readme.contains("Bitloom"),
        "README public brand must remain Bitloom"
    );
}
