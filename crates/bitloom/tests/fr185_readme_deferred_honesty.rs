//! ATDD / guardrail: Story 118.3 / FR185 — README / deferred honesty
//! for Phase 23 NFR86 leftovers.
//!
//! ```text
//! cargo test -p bitloom --test fr185_readme_deferred_honesty
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
fn fr185_readme_distinguishes_closeout_vs_phase23() {
    let readme = read("README.md");
    assert!(
        (readme.contains("工程结项") || readme.contains("结项"))
            && (readme.contains("Phase 22") || readme.contains("FR184")),
        "README must keep Phase 22 / closeout framing"
    );
    assert!(
        readme.contains("Phase 23") && (readme.contains("FR185") || readme.contains("NFR86")),
        "README must declare Phase 23 NFR86 contract"
    );
    assert!(
        readme.contains("NFR88")
            || (readme.contains("不得")
                && readme.contains("失败")
                && (readme.contains("Phase 22") || readme.contains("结项"))),
        "README must keep NFR88 Phase 12–22 / closeout isolation honesty"
    );
}

#[test]
fn fr185_readme_lists_phase23_fr_tokens() {
    let readme = read("README.md");
    for fr in [
        "FR185", "FR186", "FR187", "FR188", "FR189", "FR190", "FR191",
    ] {
        assert!(
            readme.contains(fr),
            "README Phase 23 surface must list {fr}"
        );
    }
}

#[test]
fn fr185_readme_forbids_claim_before_fr_close() {
    let readme = read("README.md");
    assert!(
        readme.contains("未关闭")
            || (readme.contains("不得") && readme.contains("FR186"))
            || readme.contains("不得宣称"),
        "README must forbid claiming Phase 23 delivers before FR closes"
    );
    assert!(
        ((readme.contains("119") || readme.contains("Epic 119"))
            && (readme.contains("不得 ready")
                || readme.contains("不得标 ready")
                || readme.contains("须各自 NFR14")
                || readme.contains("关闭前")))
            || (readme.contains("Epic 118")
                && (readme.contains("进行中") || readme.contains("闸门"))),
        "README must note Epic 119–124 gated until Epic 118 closes"
    );
    assert!(
        (readme.contains("git push") || readme.contains("`git push`"))
            && (readme.contains("不是") || readme.contains("非")),
        "README must state git push is not an FR"
    );
}

#[test]
fn fr185_deferred_phase23_pointer() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Phase 23") && deferred.contains("FR185"),
        "deferred-work must have Phase 23 pointer"
    );
    assert!(
        deferred.contains("Epic 118")
            && ((deferred.contains("不得 ready")
                || deferred.contains("未关闭前")
                || deferred.contains("119–124")
                || deferred.contains("119-124"))
                || (deferred.contains("进行中") && deferred.contains("118"))),
        "deferred must gate Epic 119–124 until Epic 118 closes"
    );
    for fr in ["FR186", "FR187", "FR188", "FR189", "FR190", "FR191"] {
        assert!(
            deferred.contains(fr),
            "deferred Phase 23 pointer must map {fr}"
        );
    }
    assert!(
        deferred.contains("NFR88")
            && (deferred.contains("Phase 22")
                || deferred.contains("结项")
                || deferred.contains("仍有效")),
        "deferred must keep NFR88 Phase 22 / closeout isolation"
    );
    assert!(
        deferred.contains("Bitloom")
            && ((deferred.contains("git push") || deferred.contains("`git push`"))
                && (deferred.contains("非") || deferred.contains("不是"))),
        "deferred Phase 23 must keep Bitloom brand and git push not FR"
    );
}
