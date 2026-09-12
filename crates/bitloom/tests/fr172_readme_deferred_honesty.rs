//! ATDD / guardrail: Story 105.3 / FR172 — README / deferred honesty
//! for Phase 21 NFR76 leftovers.
//!
//! ```text
//! cargo test -p bitloom --test fr172_readme_deferred_honesty
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
fn fr172_readme_distinguishes_phase20_closed_vs_phase21() {
    let readme = read("README.md");
    assert!(
        readme.contains("Phase 20")
            && (readme.contains("FR166") || readme.contains("已关闭"))
            && (readme.contains("Epic 99") || readme.contains("FR171")),
        "README must keep Phase 20 closed framing"
    );
    assert!(
        readme.contains("Phase 21") && (readme.contains("FR172") || readme.contains("NFR76")),
        "README must declare Phase 21 NFR76 contract"
    );
    assert!(
        readme.contains("NFR78")
            || (readme.contains("不得")
                && readme.contains("失败")
                && (readme.contains("Phase 20") || readme.contains("Phase 12"))),
        "README must keep NFR78 Phase 12–20 isolation honesty"
    );
}

#[test]
fn fr172_readme_lists_phase21_fr_tokens() {
    let readme = read("README.md");
    for fr in ["FR172", "FR173", "FR174", "FR175", "FR176", "FR177"] {
        assert!(
            readme.contains(fr),
            "README Phase 21 surface must list {fr}"
        );
    }
}

#[test]
fn fr172_readme_forbids_claim_before_fr_close() {
    let readme = read("README.md");
    assert!(
        readme.contains("未关闭")
            || (readme.contains("不得") && readme.contains("FR173"))
            || readme.contains("不得宣称"),
        "README must forbid claiming Phase 21 delivers before FR closes"
    );
    assert!(
        ((readme.contains("106") || readme.contains("Epic 106"))
            && (readme.contains("不得 ready")
                || readme.contains("不得标 ready")
                || readme.contains("须各自 NFR14")
                || readme.contains("NFR14")))
            || (readme.contains("Epic 105")
                && (readme.contains("进行中") || readme.contains("闸门"))),
        "README must note Epic 106–110 gated until Epic 105 closes"
    );
    assert!(
        (readme.contains("git push") || readme.contains("`git push`"))
            && (readme.contains("不是") || readme.contains("非")),
        "README must state git push is not an FR"
    );
}

#[test]
fn fr172_deferred_phase21_pointer() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Phase 21") && deferred.contains("FR172"),
        "deferred-work must have Phase 21 pointer"
    );
    assert!(
        deferred.contains("Epic 105")
            && ((deferred.contains("不得 ready")
                || deferred.contains("未关闭前")
                || deferred.contains("106–110")
                || deferred.contains("106-110"))
                || (deferred.contains("进行中") && deferred.contains("105"))),
        "deferred must gate Epic 106–110 until Epic 105 closes"
    );
    for fr in ["FR173", "FR174", "FR175", "FR176", "FR177"] {
        assert!(
            deferred.contains(fr),
            "deferred Phase 21 pointer must map {fr}"
        );
    }
    assert!(
        deferred.contains("NFR78")
            && (deferred.contains("Phase 20") || deferred.contains("仍有效")),
        "deferred must keep NFR78 Phase 20 close isolation"
    );
    assert!(
        deferred.contains("Bitloom")
            && ((deferred.contains("git push") || deferred.contains("`git push`"))
                && (deferred.contains("非") || deferred.contains("不是"))),
        "deferred Phase 21 must keep Bitloom brand and git push not FR"
    );
}
