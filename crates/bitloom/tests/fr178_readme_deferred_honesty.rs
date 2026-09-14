//! ATDD / guardrail: Story 111.3 / FR178 — README / deferred honesty
//! for Phase 22 NFR81 leftovers.
//!
//! ```text
//! cargo test -p bitloom --test fr178_readme_deferred_honesty
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
fn fr178_readme_distinguishes_phase21_closed_vs_phase22() {
    let readme = read("README.md");
    assert!(
        readme.contains("Phase 21")
            && (readme.contains("FR172") || readme.contains("已关闭"))
            && (readme.contains("Epic 105") || readme.contains("FR177")),
        "README must keep Phase 21 closed framing"
    );
    assert!(
        readme.contains("Phase 22") && (readme.contains("FR178") || readme.contains("NFR81")),
        "README must declare Phase 22 NFR81 contract"
    );
    assert!(
        readme.contains("NFR83")
            || (readme.contains("不得")
                && readme.contains("失败")
                && (readme.contains("Phase 21") || readme.contains("Phase 12"))),
        "README must keep NFR83 Phase 12–21 isolation honesty"
    );
}

#[test]
fn fr178_readme_lists_phase22_fr_tokens() {
    let readme = read("README.md");
    for fr in [
        "FR178", "FR179", "FR180", "FR181", "FR182", "FR183", "FR184",
    ] {
        assert!(
            readme.contains(fr),
            "README Phase 22 surface must list {fr}"
        );
    }
}

#[test]
fn fr178_readme_forbids_claim_before_fr_close() {
    let readme = read("README.md");
    assert!(
        readme.contains("未关闭")
            || (readme.contains("不得") && readme.contains("FR179"))
            || readme.contains("不得宣称"),
        "README must forbid claiming Phase 22 delivers before FR closes"
    );
    assert!(
        ((readme.contains("112") || readme.contains("Epic 112"))
            && (readme.contains("不得 ready")
                || readme.contains("不得标 ready")
                || readme.contains("须各自 NFR14")
                || readme.contains("NFR14")))
            || (readme.contains("Epic 111")
                && (readme.contains("进行中") || readme.contains("闸门"))),
        "README must note Epic 112–117 gated until Epic 111 closes"
    );
    assert!(
        (readme.contains("git push") || readme.contains("`git push`"))
            && (readme.contains("不是") || readme.contains("非")),
        "README must state git push is not an FR"
    );
}

#[test]
fn fr178_deferred_phase22_pointer() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Phase 22") && deferred.contains("FR178"),
        "deferred-work must have Phase 22 pointer"
    );
    assert!(
        deferred.contains("Epic 111")
            && ((deferred.contains("不得 ready")
                || deferred.contains("未关闭前")
                || deferred.contains("112–117")
                || deferred.contains("112-117"))
                || (deferred.contains("进行中") && deferred.contains("111"))),
        "deferred must gate Epic 112–117 until Epic 111 closes"
    );
    for fr in ["FR179", "FR180", "FR181", "FR182", "FR183", "FR184"] {
        assert!(
            deferred.contains(fr),
            "deferred Phase 22 pointer must map {fr}"
        );
    }
    assert!(
        deferred.contains("NFR83")
            && (deferred.contains("Phase 21") || deferred.contains("仍有效")),
        "deferred must keep NFR83 Phase 21 close isolation"
    );
    assert!(
        deferred.contains("Bitloom")
            && ((deferred.contains("git push") || deferred.contains("`git push`"))
                && (deferred.contains("非") || deferred.contains("不是"))),
        "deferred Phase 22 must keep Bitloom brand and git push not FR"
    );
}
