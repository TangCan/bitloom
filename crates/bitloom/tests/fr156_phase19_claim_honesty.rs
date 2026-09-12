//! ATDD Story 98.2 / FR156 — Phase 19 claim honesty surface.

use std::fs;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr156_docs_claim_map() {
    let docs = read("docs/fr156-phase19-claim-honesty.md");
    assert!(docs.contains("FR156") && docs.contains("Bitloom"));
    assert!(docs.contains("FR155") && docs.contains("FR157") && docs.contains("FR165"));
    assert!(docs.contains("NFR72") && docs.contains("NFR71"));
    assert!(
        docs.contains("Phase 18") && (docs.contains("alone") || docs.contains("≠")),
        "must forbid Phase 18 alone"
    );
    assert!(docs.contains("FR142") || docs.contains("expand"));
    for fr in [
        "FR154", "FR155", "FR157", "FR158", "FR159", "FR160", "FR161", "FR162", "FR163", "FR164",
        "FR165", "FR156",
    ] {
        assert!(docs.contains(fr), "claim map missing {fr}");
    }
}

#[test]
fn fr156_readme_honesty_pointers() {
    let readme = read("README.md");
    assert!(readme.contains("fr156-phase19-claim-honesty"));
    assert!(readme.contains("FR155") && readme.contains("≠ Phase 18"));
    assert!(readme.contains("FR157") && readme.contains("FR165"));
    assert!(
        readme.contains("FR156")
            && (readme.contains("已关闭")
                || readme.contains("complete")
                || readme.contains("Story 98.3")
                || readme.contains("未关前不得宣称")
                || readme.contains("Story 98.2")
                || readme.contains("Epic 98")),
        "README must keep FR156 claim honesty visible"
    );
    assert!(readme.contains("Bitloom"));
    assert!(readme.contains("NFR71") && readme.contains("NFR72"));
}

#[test]
fn fr156_deferred_points_to_honesty_doc() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(deferred.contains("FR156") && deferred.contains("Epic 98"));
    assert!(
        deferred.contains("fr156-phase19-claim-honesty") || deferred.contains("98.2"),
        "deferred must point at FR156 honesty work"
    );
    assert!(
        deferred.contains("NFR72") || deferred.contains("Phase 18 alone"),
        "deferred must keep NFR72 honesty"
    );
}

#[test]
fn fr156_nfr14_gates_98_2() {
    let risk = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic98-phase19-claim-honesty-fr156.md",
    );
    assert!(risk.contains("FR156") && risk.contains("98.2"));
}
