//! ATDD Story 117.2 / FR184 — Phase 22 claim honesty surface.

use std::fs;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr184_docs_claim_map() {
    let docs = read("docs/fr184-phase22-claim-honesty.md");
    assert!(docs.contains("FR184") && docs.contains("Bitloom"));
    assert!(docs.contains("NFR87") && docs.contains("NFR86"));
    assert!(
        docs.contains("Phase 21")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("冒充")),
        "must forbid Phase 21 alone"
    );
    assert!(
        docs.contains("NFR81") && (docs.contains("账本已空") || docs.contains("已空")),
        "must forbid NFR81 ledger-empty claim"
    );
    assert!(
        docs.contains("git push")
            && (docs.contains("not") || docs.contains("不是") || docs.contains("不得")),
        "must state git push is not an FR"
    );
    for fr in [
        "FR178", "FR179", "FR180", "FR181", "FR182", "FR183", "FR184",
    ] {
        assert!(docs.contains(fr), "claim map missing {fr}");
    }
}

#[test]
fn fr184_readme_honesty_pointers() {
    let readme = read("README.md");
    assert!(readme.contains("fr184-phase22-claim-honesty"));
    assert!(readme.contains("FR179") && readme.contains("FR180"));
    assert!(readme.contains("FR181") && readme.contains("FR182") && readme.contains("FR183"));
    assert!(
        readme.contains("FR184")
            && (readme.contains("Story 117.2")
                || readme.contains("Epic 117")
                || readme.contains("宣称诚实")
                || readme.contains("实现中")),
        "README must keep FR184 claim honesty visible"
    );
    assert!(
        readme.contains("≠ Phase 21")
            || readme.contains("Phase 21 alone")
            || (readme.contains("Phase 21") && readme.contains("冒充")),
        "README must keep Phase 21 alone honesty"
    );
    assert!(readme.contains("Bitloom"));
    assert!(readme.contains("NFR86") && readme.contains("NFR87"));
    assert!(
        readme.contains("NFR81") && (readme.contains("账本已空") || readme.contains("已空")),
        "README must forbid NFR81 ledger-empty"
    );
}

#[test]
fn fr184_deferred_points_to_honesty_doc() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(deferred.contains("FR184") && deferred.contains("Epic 117"));
    assert!(
        deferred.contains("fr184-phase22-claim-honesty") || deferred.contains("117.2"),
        "deferred must point at FR184 honesty work"
    );
    assert!(
        deferred.contains("NFR87")
            || deferred.contains("Phase 21 alone")
            || (deferred.contains("Phase 21") && deferred.contains("冒充")),
        "deferred must keep NFR87 honesty"
    );
}

#[test]
fn fr184_nfr14_gates_117_2() {
    let risk = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic117-phase22-claim-honesty-fr184.md",
    );
    assert!(risk.contains("FR184") && risk.contains("117.2"));
}

#[test]
fn fr184_closed_fr_docs_exist() {
    for rel in [
        "docs/fr179-floating-circt-git-head.md",
        "docs/fr180-handshake-dialect-deepen.md",
        "docs/fr181-deeper-style-guide-linter.md",
        "docs/fr182-unpaired-firtool-product-pin.md",
        "docs/fr183-explicit-fr142-api-expand.md",
        "docs/fr184-phase22-claim-honesty.md",
    ] {
        assert!(
            root().join(rel).is_file(),
            "missing claim pointer doc {rel}"
        );
    }
}
