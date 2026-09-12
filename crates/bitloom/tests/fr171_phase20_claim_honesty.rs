//! ATDD Story 104.2 / FR171 — Phase 20 claim honesty surface.

use std::fs;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr171_docs_claim_map() {
    let docs = read("docs/fr171-phase20-claim-honesty.md");
    assert!(docs.contains("FR171") && docs.contains("Bitloom"));
    assert!(docs.contains("FR167") && docs.contains("FR168") && docs.contains("FR170"));
    assert!(docs.contains("NFR77") && docs.contains("NFR76"));
    assert!(
        docs.contains("Phase 19")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("冒充")),
        "must forbid Phase 19 alone"
    );
    assert!(docs.contains("FR142") || docs.contains("expand") || docs.contains("扩大"));
    assert!(
        docs.contains("NFR71") && (docs.contains("账本已空") || docs.contains("已空")),
        "must forbid NFR71 ledger-empty claim"
    );
    assert!(
        docs.contains("git push")
            && (docs.contains("not") || docs.contains("不是") || docs.contains("不得")),
        "must state git push is not an FR"
    );
    for fr in ["FR166", "FR167", "FR168", "FR169", "FR170", "FR171"] {
        assert!(docs.contains(fr), "claim map missing {fr}");
    }
}

#[test]
fn fr171_readme_honesty_pointers() {
    let readme = read("README.md");
    assert!(readme.contains("fr171-phase20-claim-honesty"));
    assert!(readme.contains("FR167") && readme.contains("FR168"));
    assert!(readme.contains("FR169") && readme.contains("FR170"));
    assert!(
        readme.contains("FR171")
            && (readme.contains("Story 104.2")
                || readme.contains("Epic 104")
                || readme.contains("宣称诚实")
                || readme.contains("未关闭前不得宣称")),
        "README must keep FR171 claim honesty visible"
    );
    assert!(
        readme.contains("≠ Phase 19")
            || readme.contains("Phase 19 alone")
            || (readme.contains("Phase 19") && readme.contains("冒充")),
        "README must keep Phase 19 alone honesty"
    );
    assert!(readme.contains("Bitloom"));
    assert!(readme.contains("NFR76") && readme.contains("NFR77"));
}

#[test]
fn fr171_deferred_points_to_honesty_doc() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(deferred.contains("FR171") && deferred.contains("Epic 104"));
    assert!(
        deferred.contains("fr171-phase20-claim-honesty") || deferred.contains("104.2"),
        "deferred must point at FR171 honesty work"
    );
    assert!(
        deferred.contains("NFR77")
            || deferred.contains("Phase 19 alone")
            || (deferred.contains("Phase 19") && deferred.contains("冒充")),
        "deferred must keep NFR77 honesty"
    );
}

#[test]
fn fr171_nfr14_gates_104_2() {
    let risk = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic104-phase20-claim-honesty-fr171.md",
    );
    assert!(risk.contains("FR171") && risk.contains("104.2"));
}

#[test]
fn fr171_closed_fr_docs_exist() {
    for rel in [
        "docs/fr167-chiselsim-ide-stores.md",
        "docs/fr168-spi-i2c-axi-handwritten-fl.md",
        "docs/fr169-circt-mlir-allocation.md",
        "docs/fr170-chisel-head-parser.md",
        "docs/fr171-phase20-claim-honesty.md",
    ] {
        assert!(
            root().join(rel).is_file(),
            "missing claim pointer doc {rel}"
        );
    }
}
