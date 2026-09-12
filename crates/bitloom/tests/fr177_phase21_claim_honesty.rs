//! ATDD Story 110.2 / FR177 — Phase 21 claim honesty surface.

use std::fs;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr177_docs_claim_map() {
    let docs = read("docs/fr177-phase21-claim-honesty.md");
    assert!(docs.contains("FR177") && docs.contains("Bitloom"));
    assert!(docs.contains("FR173") && docs.contains("FR174") && docs.contains("FR176"));
    assert!(docs.contains("NFR82") && docs.contains("NFR81"));
    assert!(
        docs.contains("Phase 20")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("冒充")),
        "must forbid Phase 20 alone"
    );
    assert!(docs.contains("FR142") || docs.contains("expand") || docs.contains("扩大"));
    assert!(
        docs.contains("NFR76") && (docs.contains("账本已空") || docs.contains("已空")),
        "must forbid NFR76 ledger-empty claim"
    );
    assert!(
        docs.contains("git push")
            && (docs.contains("not") || docs.contains("不是") || docs.contains("不得")),
        "must state git push is not an FR"
    );
    for fr in ["FR172", "FR173", "FR174", "FR175", "FR176", "FR177"] {
        assert!(docs.contains(fr), "claim map missing {fr}");
    }
}

#[test]
fn fr177_readme_honesty_pointers() {
    let readme = read("README.md");
    assert!(readme.contains("fr177-phase21-claim-honesty"));
    assert!(readme.contains("FR173") && readme.contains("FR174"));
    assert!(readme.contains("FR175") && readme.contains("FR176"));
    assert!(
        readme.contains("FR177")
            && (readme.contains("Story 110.2")
                || readme.contains("Epic 110")
                || readme.contains("宣称诚实")),
        "README must keep FR177 claim honesty visible"
    );
    assert!(
        readme.contains("≠ Phase 20")
            || readme.contains("Phase 20 alone")
            || (readme.contains("Phase 20") && readme.contains("冒充")),
        "README must keep Phase 20 alone honesty"
    );
    assert!(readme.contains("Bitloom"));
    assert!(readme.contains("NFR81") && readme.contains("NFR82"));
    assert!(
        readme.contains("NFR76") && (readme.contains("账本已空") || readme.contains("已空")),
        "README must forbid NFR76 ledger-empty"
    );
}

#[test]
fn fr177_deferred_points_to_honesty_doc() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(deferred.contains("FR177") && deferred.contains("Epic 110"));
    assert!(
        deferred.contains("fr177-phase21-claim-honesty") || deferred.contains("110.2"),
        "deferred must point at FR177 honesty work"
    );
    assert!(
        deferred.contains("NFR82")
            || deferred.contains("Phase 20 alone")
            || (deferred.contains("Phase 20") && deferred.contains("冒充")),
        "deferred must keep NFR82 honesty"
    );
}

#[test]
fn fr177_nfr14_gates_110_2() {
    let risk = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic110-phase21-claim-honesty-fr177.md",
    );
    assert!(risk.contains("FR177") && risk.contains("110.2"));
}

#[test]
fn fr177_closed_fr_docs_exist() {
    for rel in [
        "docs/fr173-firtool-bump-ad9.md",
        "docs/fr174-unpaired-head.md",
        "docs/fr175-broader-circt-mlir-sim.md",
        "docs/fr176-deeper-parser-chisel-ecosystem.md",
        "docs/fr177-phase21-claim-honesty.md",
    ] {
        assert!(
            root().join(rel).is_file(),
            "missing claim pointer doc {rel}"
        );
    }
}
