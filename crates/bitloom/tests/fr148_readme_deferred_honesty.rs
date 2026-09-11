//! ATDD / guardrail: Story 84.3 / FR148 — README / deferred /
//! doc-19 honesty surface for Phase 18 CLI crates.io publishability.
//!
//! ```text
//! cargo test -p bitloom --test fr148_readme_deferred_honesty
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
fn fr148_readme_distinguishes_phase17_closed_vs_phase18() {
    let readme = read("README.md");
    assert!(
        readme.contains("Phase 17")
            && (readme.contains("FR141") || readme.contains("1.0") || readme.contains("稳定门"))
            && (readme.contains("已关闭") || readme.contains("Epic 83")),
        "README must keep Phase 17 API stability closed framing"
    );
    assert!(
        readme.contains("Phase 18")
            && (readme.contains("FR148") || readme.contains("CLI") || readme.contains("crates.io")),
        "README must declare Phase 18 CLI crates.io publishability contract"
    );
    assert!(
        readme.contains("NFR64")
            || (readme.contains("不得")
                && readme.contains("失败")
                && (readme.contains("Phase 17") || readme.contains("Phase 12"))),
        "README must keep NFR64 Phase 12–17 isolation honesty"
    );
}

#[test]
fn fr148_readme_lists_phase18_fr_table() {
    let readme = read("README.md");
    for fr in ["FR149", "FR150", "FR151", "FR152", "FR153"] {
        assert!(readme.contains(fr), "README Phase 18 table must list {fr}");
    }
}

#[test]
fn fr148_readme_cli_not_installable_until_fr151() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR151"),
        "README must cite FR151 for CLI publish"
    );
    assert!(
        (readme.contains("cargo install") || readme.contains("`cargo install"))
            && (readme.contains("禁止")
                || readme.contains("尚不可")
                || readme.contains("仍待")
                || readme.contains("不得")
                || readme.contains("暗示")),
        "README must honestly state cargo install bitloom is not available until FR151"
    );
    assert!(
        readme.contains("1.0.0")
            && (readme.contains("库")
                || readme.contains("prelude")
                || readme.contains("crates.io")),
        "README must state library crates are 1.0.0"
    );
}

#[test]
fn fr148_readme_lists_nfr59_deferred() {
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
        readme.contains("NFR67")
            || (readme.contains("CLI")
                && readme.contains("NFR59")
                && (readme.contains("不等于") || readme.contains("≠") || readme.contains("清空"))),
        "README must state CLI publish ≠ clear NFR59 (NFR67)"
    );
}

#[test]
fn fr148_deferred_phase18_pointer() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Phase 18") && (deferred.contains("FR148") || deferred.contains("CLI")),
        "deferred-work must have Phase 18 pointer"
    );
    for fr in ["FR149", "FR150", "FR151", "FR152", "FR153"] {
        assert!(
            deferred.contains(fr),
            "deferred must map Phase 18 items to {fr}"
        );
    }
    assert!(
        deferred.contains("NFR64")
            || deferred.contains("仍有效")
            || (deferred.contains("Phase 17") && deferred.contains("已关闭")),
        "deferred must keep Phase 12–17 closes valid"
    );
    assert!(
        deferred.contains("NFR59"),
        "deferred Phase 18 path must list NFR59 honesty"
    );
    assert!(
        deferred.contains("Bitloom") || deferred.contains("bitloom"),
        "deferred Phase 18 path must keep Bitloom brand visible in tree"
    );
    assert!(
        deferred.contains("FR151")
            && (deferred.contains("cargo install")
                || deferred.contains("禁止")
                || deferred.contains("暗示")),
        "deferred must forbid implying cargo install before FR151"
    );
}

#[test]
fn fr148_keeps_phase12_17_close_evidence() {
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        readme.contains("Phase 12")
            && (readme.contains("FR94") || readme.contains("字面绿") || readme.contains("Epic 40")),
        "README must retain Phase 12 close evidence"
    );
    assert!(
        readme.contains("Phase 17") && readme.contains("FR141"),
        "README must retain Phase 17 close evidence"
    );
    assert!(
        deferred.contains("Phase 17 pointer") || deferred.contains("FR141–147"),
        "deferred must retain Phase 17 pointer"
    );
}

#[test]
fn fr148_doc19_cross_link_phase18_without_redefining_mvp() {
    let doc19 = read("docs/requirements/19. 实施路线图.md");
    assert!(
        doc19.contains("Phase 18") || doc19.contains("FR148"),
        "doc-19 should cross-link Phase 18 CLI publishability contract"
    );
    assert!(
        doc19.contains("字面绿完成定义") && doc19.contains("FR94"),
        "doc-19 must retain Phase 12 literal-green MVP definitions"
    );
    assert!(
        doc19.contains("禁止")
            && (doc19.contains("cargo install")
                || doc19.contains("FR151")
                || doc19.contains("暗示")),
        "doc-19 must forbid cargo install claim before FR151"
    );
}

#[test]
fn fr148_sprint_epic85_to_86_seeded_not_ready_while_gate_open() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("84-2-correct-course-prd-批准-phase-18-fr148: done")
            || sprint.contains("84-2-correct-course-prd-批准-phase-18-fr148:done"),
        "Story 84.2 must be done before 84.3 honesty surface closes"
    );
    for epic in 85..=86 {
        let key = format!("epic-{epic}:");
        assert!(
            sprint.lines().any(|l| l.trim_start().starts_with(&key)),
            "Epic {epic} must remain seeded during Phase 18 gate stories"
        );
    }
    let epic84_done = sprint.contains("epic-84: done") || sprint.contains("epic-84:done");
    if !epic84_done {
        for line in sprint.lines() {
            let t = line.trim();
            if (t.starts_with("85-") || t.starts_with("86-")) && t.contains("ready-for-dev") {
                panic!("Epic 85–86 must not be ready-for-dev before epic-84 done: {t}");
            }
        }
    }
}

#[test]
fn fr148_readme_bitloom_brand() {
    let readme = read("README.md");
    assert!(
        readme.contains("Bitloom"),
        "README public brand must remain Bitloom"
    );
}
