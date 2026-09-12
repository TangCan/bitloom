//! ATDD / guardrail: Story 87.3 / FR154 — README / deferred honesty
//! for Phase 19 NFR59 + FR152(a).
//!
//! ```text
//! cargo test -p bitloom --test fr154_readme_deferred_honesty
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
fn fr154_readme_distinguishes_phase18_closed_vs_phase19() {
    let readme = read("README.md");
    assert!(
        readme.contains("Phase 18")
            && (readme.contains("FR148") || readme.contains("CLI"))
            && (readme.contains("已关闭") || readme.contains("Epic 86")),
        "README must keep Phase 18 CLI closed framing"
    );
    assert!(
        readme.contains("Phase 19")
            && (readme.contains("FR154") || readme.contains("NFR59") || readme.contains("FR152")),
        "README must declare Phase 19 NFR59 / FR152(a) contract"
    );
    assert!(
        readme.contains("NFR68")
            || (readme.contains("不得")
                && readme.contains("失败")
                && (readme.contains("Phase 18") || readme.contains("Phase 12"))),
        "README must keep NFR68 Phase 12–18 isolation honesty"
    );
}

#[test]
fn fr154_readme_lists_phase19_fr_table() {
    let readme = read("README.md");
    for fr in [
        "FR154", "FR155", "FR156", "FR157", "FR158", "FR159", "FR160", "FR161", "FR162", "FR163",
        "FR164", "FR165",
    ] {
        assert!(readme.contains(fr), "README Phase 19 table must list {fr}");
    }
}

#[test]
fn fr154_readme_forbids_claim_before_fr_close() {
    let readme = read("README.md");
    assert!(
        readme.contains("未关闭前不得宣称")
            || (readme.contains("不得宣称") && readme.contains("FR155")),
        "README must forbid claiming Phase 19 delivers before FR closes"
    );
    assert!(
        (readme.contains("git push") || readme.contains("`git push`"))
            && (readme.contains("不是") || readme.contains("非")),
        "README must state git push is not an FR"
    );
}

#[test]
fn fr154_deferred_phase19_pointer() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Phase 19") && deferred.contains("FR154"),
        "deferred-work must have Phase 19 pointer"
    );
    assert!(
        deferred.contains("FR155")
            && deferred.contains("FR157")
            && deferred.contains("FR165")
            && deferred.contains("FR156"),
        "deferred must map Phase 19 FR155 / FR157–165 / FR156"
    );
    assert!(
        deferred.contains("NFR68")
            || (deferred.contains("仍有效") && deferred.contains("Phase 18")),
        "deferred must keep Phase 12–18 closes valid"
    );
    assert!(
        deferred.contains("Bitloom") || deferred.contains("bitloom"),
        "deferred Phase 19 path must keep Bitloom brand"
    );
}

#[test]
fn fr154_sprint_gate_87_2_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("87-2-correct-course-prd-批准-phase-19-fr154: done")
            || sprint.contains("87-2-correct-course-prd-批准-phase-19-fr154:done"),
        "Story 87.2 must be done before 87.3 honesty surface closes"
    );
    let epic87_done = sprint.contains("epic-87: done") || sprint.contains("epic-87:done");
    if !epic87_done {
        for line in sprint.lines() {
            let t = line.trim();
            if (t.starts_with("88-") || t.starts_with("98-")) && t.contains("ready-for-dev") {
                panic!("Epic 88–98 must not be ready-for-dev before epic-87 done: {t}");
            }
        }
    }
}

#[test]
fn fr154_readme_bitloom_brand() {
    let readme = read("README.md");
    assert!(
        readme.contains("Bitloom") && readme.contains("bitloom"),
        "README must keep Bitloom / bitloom brand"
    );
}
