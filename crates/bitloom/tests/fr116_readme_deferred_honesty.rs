//! ATDD / guardrail: Story 57.3 / FR116+FR123 — README / deferred /
//! doc-19 honesty surface for Phase 14 NFR47 deepen.
//!
//! ```text
//! cargo test -p bitloom --test fr116_readme_deferred_honesty
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
fn fr116_readme_distinguishes_phase13_closed_vs_phase14() {
    let readme = read("README.md");
    assert!(
        readme.contains("Phase 13")
            && (readme.contains("FR106") || readme.contains("商业加深"))
            && (readme.contains("已关闭") || readme.contains("Epic 48")),
        "README must keep Phase 13 commercial deepen closed framing"
    );
    assert!(
        readme.contains("Phase 14")
            && (readme.contains("NFR47") || readme.contains("FR116") || readme.contains("加深")),
        "README must declare Phase 14 NFR47 deepen contract"
    );
    assert!(
        readme.contains("NFR48")
            || (readme.contains("不得")
                && readme.contains("失败")
                && (readme.contains("Phase 13") || readme.contains("Phase 12"))),
        "README must keep NFR48 Phase 12/13 isolation honesty"
    );
}

#[test]
fn fr116_readme_lists_phase14_deepen_fr_table() {
    let readme = read("README.md");
    for fr in ["FR117", "FR118", "FR119", "FR120", "FR121", "FR122"] {
        assert!(
            readme.contains(fr),
            "README Phase 14 deepen table must list {fr}"
        );
    }
}

#[test]
fn fr116_readme_fr123_claim_discipline() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR123"),
        "README must cite FR123 claim discipline"
    );
    let needles = [
        "Tywaves",
        "syn-scan",
        "SBY",
        "VIP GPIO",
        "Handshake",
        "官方风格",
    ];
    let hit = needles.iter().filter(|n| readme.contains(**n)).count();
    assert!(
        hit >= 4,
        "README must name Phase 14 claim topics (got {hit}/6 of {needles:?})"
    );
    assert!(
        readme.contains("关闭") && (readme.contains("宣称") || readme.contains("勾选")),
        "README must state claims only after corresponding FR closes"
    );
}

#[test]
fn fr116_deferred_phase14_pointer_and_upgrade() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Phase 14") && (deferred.contains("FR116") || deferred.contains("NFR47")),
        "deferred-work must have Phase 14 pointer"
    );
    for fr in ["FR117", "FR118", "FR119", "FR120", "FR121", "FR122"] {
        assert!(
            deferred.contains(fr),
            "deferred must map upgraded items to {fr}"
        );
    }
    assert!(
        !deferred.contains("尚无合同")
            || deferred.contains("Phase 14 contract approved")
            || deferred.contains("Phase 14") && deferred.contains("已批准"),
        "must not claim no contract exists without Phase 14 approval note"
    );
    assert!(
        deferred.contains("NFR48")
            || deferred.contains("仍有效")
            || (deferred.contains("Phase 13") && deferred.contains("已关闭")),
        "deferred must keep Phase 12/13 closes valid"
    );
}

#[test]
fn fr116_keeps_phase12_phase13_close_evidence() {
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
        deferred.contains("Literal-green") || deferred.contains("Phase 12 MVP"),
        "deferred must retain Phase 12 MVP pointer"
    );
    assert!(
        deferred.contains("Phase 13 pointer") || deferred.contains("FR106–115"),
        "deferred must retain Phase 13 pointer"
    );
}

#[test]
fn fr116_doc19_cross_link_phase14_without_redefining_mvp() {
    let doc19 = read("docs/requirements/19. 实施路线图.md");
    assert!(
        doc19.contains("Phase 14") || doc19.contains("FR116"),
        "doc-19 should cross-link Phase 14 deepen contract"
    );
    assert!(
        doc19.contains("字面绿完成定义") && doc19.contains("FR94"),
        "doc-19 must retain Phase 12 literal-green MVP definitions"
    );
}

#[test]
fn fr116_sprint_epic58_to_63_seeded_after_gate() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    // Epic 57 close unlocks 58–63 from permanent freeze; they remain seeded and stay
    // backlog until each epic's own NFR14 / create-story (do not over-freeze forever).
    for epic in 58..=63 {
        let key = format!("epic-{epic}:");
        assert!(
            sprint.lines().any(|l| l.trim_start().starts_with(&key)),
            "Epic {epic} must remain seeded after Phase 14 gate stories"
        );
    }
    assert!(
        sprint.contains("epic-57:") || sprint.contains("57-1-"),
        "Epic 57 tracking must remain present"
    );
}
