//! ATDD / guardrail: Story 64.3 / FR124+FR132 — README / deferred /
//! doc-19 honesty surface for Phase 15 NFR51 leftover deepen.
//!
//! ```text
//! cargo test -p bitloom --test fr124_readme_deferred_honesty
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
fn fr124_readme_distinguishes_phase14_closed_vs_phase15() {
    let readme = read("README.md");
    assert!(
        readme.contains("Phase 14")
            && (readme.contains("FR116") || readme.contains("NFR47"))
            && (readme.contains("已关闭") || readme.contains("Epic 57")),
        "README must keep Phase 14 deepen closed framing"
    );
    assert!(
        readme.contains("Phase 15")
            && (readme.contains("NFR51") || readme.contains("FR124") || readme.contains("加深")),
        "README must declare Phase 15 NFR51 leftover deepen contract"
    );
    assert!(
        readme.contains("NFR52")
            || (readme.contains("不得")
                && readme.contains("失败")
                && (readme.contains("Phase 14") || readme.contains("Phase 12"))),
        "README must keep NFR52 Phase 12–14 isolation honesty"
    );
}

#[test]
fn fr124_readme_lists_phase15_deepen_fr_table() {
    let readme = read("README.md");
    for fr in [
        "FR125", "FR126", "FR127", "FR128", "FR129", "FR130", "FR131",
    ] {
        assert!(
            readme.contains(fr),
            "README Phase 15 deepen table must list {fr}"
        );
    }
}

#[test]
fn fr124_readme_fr132_claim_discipline() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR132"),
        "README must cite FR132 claim discipline"
    );
    let needles = [
        "Tywaves",
        "IP FL",
        "sby",
        "SoC pad",
        "Handshake",
        "Style Guide",
        "ip.rs",
    ];
    let hit = needles.iter().filter(|n| readme.contains(**n)).count();
    assert!(
        hit >= 5,
        "README must name Phase 15 claim topics (got {hit}/7 of {needles:?})"
    );
    assert!(
        readme.contains("关闭") && (readme.contains("宣称") || readme.contains("勾选")),
        "README must state claims only after corresponding FR closes"
    );
    assert!(
        !readme.contains("FR117 alone") || readme.contains("不得") && readme.contains("FR117"),
        "must not present FR117 alone as Phase 15 complete"
    );
    assert!(
        readme.contains("不得")
            && (readme.contains("FR117") || readme.contains("alone") || readme.contains("冒充")),
        "must forbid writing FR117/119/120/121/122 alone as Phase 15 complete face"
    );
}

#[test]
fn fr124_deferred_phase15_pointer() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Phase 15") && (deferred.contains("FR124") || deferred.contains("NFR51")),
        "deferred-work must have Phase 15 pointer"
    );
    for fr in [
        "FR125", "FR126", "FR127", "FR128", "FR129", "FR130", "FR131",
    ] {
        assert!(
            deferred.contains(fr),
            "deferred must map upgraded items to {fr}"
        );
    }
    assert!(
        deferred.contains("NFR52")
            || deferred.contains("仍有效")
            || (deferred.contains("Phase 14") && deferred.contains("已关闭")),
        "deferred must keep Phase 12–14 closes valid"
    );
    assert!(
        deferred.contains("Bitloom") || deferred.contains("bitloom"),
        "deferred Phase 15 path must keep Bitloom brand visible in tree"
    );
}

#[test]
fn fr124_keeps_phase12_14_close_evidence() {
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
        deferred.contains("Phase 14 pointer") || deferred.contains("FR116–123"),
        "deferred must retain Phase 14 pointer"
    );
}

#[test]
fn fr124_doc19_cross_link_phase15_without_redefining_mvp() {
    let doc19 = read("docs/requirements/19. 实施路线图.md");
    assert!(
        doc19.contains("Phase 15") || doc19.contains("FR124"),
        "doc-19 should cross-link Phase 15 deepen contract"
    );
    assert!(
        doc19.contains("字面绿完成定义") && doc19.contains("FR94"),
        "doc-19 must retain Phase 12 literal-green MVP definitions"
    );
}

#[test]
fn fr124_sprint_epic65_to_71_seeded_not_ready_while_gate_open() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    for epic in 65..=71 {
        let key = format!("epic-{epic}:");
        assert!(
            sprint.lines().any(|l| l.trim_start().starts_with(&key)),
            "Epic {epic} must remain seeded during Phase 15 gate stories"
        );
    }
    // Hard gate: while epic-64 not done, deepen stories must not be ready-for-dev
    let epic64_done = sprint.contains("epic-64: done") || sprint.contains("epic-64:done");
    if !epic64_done {
        for line in sprint.lines() {
            let t = line.trim();
            if (t.starts_with("65-")
                || t.starts_with("66-")
                || t.starts_with("67-")
                || t.starts_with("68-")
                || t.starts_with("69-")
                || t.starts_with("70-")
                || t.starts_with("71-"))
                && t.contains("ready-for-dev")
            {
                panic!("Epic 65–71 must not be ready-for-dev before epic-64 done: {t}");
            }
        }
    }
}
