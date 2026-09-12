//! ATDD / guardrail: Story 99.3 / FR166 — README / deferred honesty
//! for Phase 20 NFR71 four leftovers.
//!
//! ```text
//! cargo test -p bitloom --test fr166_readme_deferred_honesty
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
fn fr166_readme_distinguishes_phase19_closed_vs_phase20() {
    let readme = read("README.md");
    assert!(
        readme.contains("Phase 19")
            && (readme.contains("FR154") || readme.contains("已关闭"))
            && (readme.contains("Epic 87") || readme.contains("FR165") || readme.contains("FR156")),
        "README must keep Phase 19 closed framing"
    );
    assert!(
        readme.contains("Phase 20") && (readme.contains("FR166") || readme.contains("NFR71")),
        "README must declare Phase 20 NFR71 contract"
    );
    assert!(
        readme.contains("NFR73")
            || (readme.contains("不得")
                && readme.contains("失败")
                && (readme.contains("Phase 19") || readme.contains("Phase 12"))),
        "README must keep NFR73 Phase 12–19 isolation honesty"
    );
}

#[test]
fn fr166_readme_lists_phase20_fr_tokens() {
    let readme = read("README.md");
    for fr in ["FR166", "FR167", "FR168", "FR169", "FR170", "FR171"] {
        assert!(
            readme.contains(fr),
            "README Phase 20 surface must list {fr}"
        );
    }
}

#[test]
fn fr166_readme_forbids_claim_before_fr_close() {
    let readme = read("README.md");
    assert!(
        readme.contains("未关闭前不得宣称")
            || (readme.contains("不得宣称") && readme.contains("FR167")),
        "README must forbid claiming Phase 20 delivers before FR closes"
    );
    assert!(
        (readme.contains("100") || readme.contains("Epic 100"))
            && (readme.contains("不得 ready")
                || readme.contains("不得标 ready")
                || readme.contains("不得 ready")),
        "README must note Epic 100–104 not ready before Epic 99 closes"
    );
    assert!(
        (readme.contains("git push") || readme.contains("`git push`"))
            && (readme.contains("不是") || readme.contains("非")),
        "README must state git push is not an FR"
    );
}

#[test]
fn fr166_deferred_phase20_pointer() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Phase 20") && deferred.contains("FR166"),
        "deferred-work must have Phase 20 pointer"
    );
    assert!(
        deferred.contains("Epic 99")
            && (deferred.contains("不得 ready")
                || deferred.contains("未关闭前")
                || deferred.contains("100–104")
                || deferred.contains("100-104")),
        "deferred must gate Epic 100–104 until Epic 99 closes"
    );
    for fr in ["FR167", "FR168", "FR169", "FR170", "FR171"] {
        assert!(
            deferred.contains(fr),
            "deferred Phase 20 pointer must map {fr}"
        );
    }
    assert!(
        deferred.contains("NFR73")
            && (deferred.contains("Phase 19") || deferred.contains("仍有效")),
        "deferred must keep NFR73 Phase 19 close isolation"
    );
    assert!(
        deferred.contains("Bitloom")
            && ((deferred.contains("git push") || deferred.contains("`git push`"))
                && (deferred.contains("非") || deferred.contains("不是"))),
        "deferred Phase 20 must keep Bitloom brand and git push not FR"
    );
}
