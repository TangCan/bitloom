//! ATDD / guardrail: Story 79.3 / FR141+FR147 — README / deferred /
//! doc-19 honesty surface for Phase 17 API stability / Bitloom 1.0.
//!
//! ```text
//! cargo test -p bitloom --test fr141_readme_deferred_honesty
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
fn fr141_readme_distinguishes_phase16_closed_vs_phase17() {
    let readme = read("README.md");
    assert!(
        readme.contains("Phase 16")
            && (readme.contains("FR133") || readme.contains("NFR55") || readme.contains("终局"))
            && (readme.contains("已关闭") || readme.contains("Epic 72")),
        "README must keep Phase 16 final-closeout closed framing"
    );
    assert!(
        readme.contains("Phase 17")
            && (readme.contains("FR141") || readme.contains("稳定门") || readme.contains("1.0")),
        "README must declare Phase 17 API stability / 1.0 contract"
    );
    assert!(
        readme.contains("NFR60")
            || (readme.contains("不得")
                && readme.contains("失败")
                && (readme.contains("Phase 16") || readme.contains("Phase 12"))),
        "README must keep NFR60 Phase 12–16 isolation honesty"
    );
}

#[test]
fn fr141_readme_lists_phase17_stability_fr_table() {
    let readme = read("README.md");
    for fr in ["FR142", "FR143", "FR144", "FR145", "FR146"] {
        assert!(
            readme.contains(fr),
            "README Phase 17 stability table must list {fr}"
        );
    }
}

#[test]
fn fr141_readme_fr147_claim_discipline() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR147"),
        "README must cite FR147 claim discipline"
    );
    let needles = [
        "公开 API",
        "表面",
        "SemVer",
        "semver",
        "1.0.0",
        "publish",
        "稳定",
    ];
    let hit = needles.iter().filter(|n| readme.contains(**n)).count();
    assert!(
        hit >= 4,
        "README must name Phase 17 claim topics (got {hit}/7 of {needles:?})"
    );
    assert!(
        readme.contains("关闭") && (readme.contains("宣称") || readme.contains("勾选")),
        "README must state claims only after corresponding FR closes"
    );
    assert!(
        readme.contains("不得")
            && (readme.contains("Phase 16") || readme.contains("alone") || readme.contains("冒充")),
        "must forbid writing Phase 16 alone as 1.0 / public API stable"
    );
}

#[test]
fn fr141_readme_lists_nfr59_deferred() {
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
    assert!(
        readme.contains("NFR63")
            || (readme.contains("1.0")
                && readme.contains("NFR59")
                && (readme.contains("不等于") || readme.contains("≠") || readme.contains("清空"))),
        "README must state 1.0 ≠ clear NFR59 (NFR63)"
    );
}

#[test]
fn fr141_deferred_phase17_pointer() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Phase 17") && (deferred.contains("FR141") || deferred.contains("1.0")),
        "deferred-work must have Phase 17 pointer"
    );
    for fr in ["FR142", "FR143", "FR144", "FR145", "FR146"] {
        assert!(
            deferred.contains(fr),
            "deferred must map Phase 17 items to {fr}"
        );
    }
    assert!(
        deferred.contains("NFR60")
            || deferred.contains("仍有效")
            || (deferred.contains("Phase 16") && deferred.contains("已关闭")),
        "deferred must keep Phase 12–16 closes valid"
    );
    assert!(
        deferred.contains("NFR59"),
        "deferred Phase 17 path must list NFR59 honesty"
    );
    assert!(
        deferred.contains("Bitloom") || deferred.contains("bitloom"),
        "deferred Phase 17 path must keep Bitloom brand visible in tree"
    );
}

#[test]
fn fr141_keeps_phase12_16_close_evidence() {
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
        readme.contains("Phase 16") && readme.contains("FR133"),
        "README must retain Phase 16 close evidence"
    );
    assert!(
        deferred.contains("Phase 16 pointer") || deferred.contains("FR133–140"),
        "deferred must retain Phase 16 pointer"
    );
}

#[test]
fn fr141_doc19_cross_link_phase17_without_redefining_mvp() {
    let doc19 = read("docs/requirements/19. 实施路线图.md");
    assert!(
        doc19.contains("Phase 17") || doc19.contains("FR141"),
        "doc-19 should cross-link Phase 17 API stability contract"
    );
    assert!(
        doc19.contains("字面绿完成定义") && doc19.contains("FR94"),
        "doc-19 must retain Phase 12 literal-green MVP definitions"
    );
    assert!(
        doc19.contains("禁止")
            && (doc19.contains("Phase 16") || doc19.contains("alone") || doc19.contains("1.0")),
        "doc-19 must forbid Phase 16 alone as 1.0"
    );
}

#[test]
fn fr141_sprint_epic80_to_83_seeded_not_ready_while_gate_open() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("79-2-correct-course-prd-批准-phase-17-fr141: done")
            || sprint.contains("79-2-correct-course-prd-批准-phase-17-fr141:done"),
        "Story 79.2 must be done before 79.3 honesty surface closes"
    );
    for epic in 80..=83 {
        let key = format!("epic-{epic}:");
        assert!(
            sprint.lines().any(|l| l.trim_start().starts_with(&key)),
            "Epic {epic} must remain seeded during Phase 17 gate stories"
        );
    }
    let epic79_done = sprint.contains("epic-79: done") || sprint.contains("epic-79:done");
    if !epic79_done {
        for line in sprint.lines() {
            let t = line.trim();
            if (t.starts_with("80-")
                || t.starts_with("81-")
                || t.starts_with("82-")
                || t.starts_with("83-"))
                && t.contains("ready-for-dev")
            {
                panic!("Epic 80–83 must not be ready-for-dev before epic-79 done: {t}");
            }
        }
    }
}

#[test]
fn fr141_readme_bitloom_brand() {
    let readme = read("README.md");
    assert!(
        readme.contains("Bitloom"),
        "README public brand must remain Bitloom"
    );
    assert!(
        readme.contains("0.x") || readme.contains("1.0.0") || readme.contains("当前仍为"),
        "README must keep version honesty (0.x history and/or 1.0.0 after FR146)"
    );
}
