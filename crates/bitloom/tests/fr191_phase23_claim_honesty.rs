//! ATDD Story 124.2 / FR191 — Phase 23 claim honesty surface.

use std::fs;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr191_docs_claim_map() {
    let docs = read("docs/fr191-phase23-claim-honesty.md");
    assert!(docs.contains("FR191") && docs.contains("Bitloom"));
    assert!(docs.contains("NFR92") && docs.contains("NFR91") && docs.contains("NFR88"));
    assert!(
        (docs.contains("Phase 22") || docs.contains("结项") || docs.contains("closeout"))
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("冒充")),
        "must forbid Phase 22 / closeout alone"
    );
    assert!(
        docs.contains("NFR86") && (docs.contains("账本已空") || docs.contains("已空")),
        "must forbid NFR86 ledger-empty claim"
    );
    assert!(
        docs.contains("FR189") && (docs.contains("blocked") || docs.contains("blocked-upstream")),
        "must list FR189 blocked honestly"
    );
    assert!(
        docs.contains("git push")
            && (docs.contains("not") || docs.contains("不是") || docs.contains("不得")),
        "must state git push is not an FR"
    );
    for fr in [
        "FR185", "FR186", "FR187", "FR188", "FR189", "FR190", "FR191",
    ] {
        assert!(docs.contains(fr), "claim map missing {fr}");
    }
    assert!(
        docs.contains("Epic 124 / FR191 closed") || docs.contains("124.3"),
        "FR191 product doc must note closed after Story 124.3"
    );
}

#[test]
fn fr191_readme_honesty_pointers() {
    let readme = read("README.md");
    assert!(readme.contains("fr191-phase23-claim-honesty"));
    assert!(readme.contains("FR186") && readme.contains("FR187"));
    assert!(readme.contains("FR188") && readme.contains("FR190"));
    assert!(
        readme.contains("FR189")
            && (readme.contains("blocked-upstream") || readme.contains("blocked")),
        "README must keep FR189 blocked honesty"
    );
    assert!(
        readme.contains("FR191")
            && (readme.contains("Story 124.2")
                || readme.contains("Epic 124")
                || readme.contains("宣称诚实")
                || readme.contains("实现中")),
        "README must keep FR191 claim honesty visible"
    );
    assert!(
        readme.contains("Phase 22") && (readme.contains("alone") || readme.contains("冒充")),
        "README must keep Phase 22 alone honesty"
    );
    assert!(readme.contains("Bitloom"));
    assert!(readme.contains("NFR91") && readme.contains("NFR92"));
    assert!(
        readme.contains("NFR86") && (readme.contains("账本已空") || readme.contains("已空")),
        "README must forbid NFR86 ledger-empty"
    );
}

#[test]
fn fr191_deferred_points_to_honesty_doc() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(deferred.contains("FR191") && deferred.contains("Epic 124"));
    assert!(
        deferred.contains("fr191-phase23-claim-honesty") || deferred.contains("124.2"),
        "deferred must point at FR191 honesty work"
    );
    assert!(
        deferred.contains("FR189")
            && (deferred.contains("blocked") || deferred.contains("blocked-upstream")),
        "deferred must keep FR189 blocked"
    );
    assert!(
        deferred.contains("NFR92")
            || deferred.contains("Phase 22 alone")
            || (deferred.contains("Phase 22") && deferred.contains("冒充")),
        "deferred must keep NFR92 honesty"
    );
}

#[test]
fn fr191_nfr14_gates_124_2() {
    let risk = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic124-phase23-claim-honesty-fr191.md",
    );
    assert!(risk.contains("FR191") && risk.contains("124.2"));
    assert!(
        risk.contains("FR189") && (risk.contains("blocked") || risk.contains("未关")),
        "NFR14 must list FR189 unclosed/blocked"
    );
}

#[test]
fn fr191_closed_and_blocked_fr_docs_exist() {
    for rel in [
        "docs/fr186-unbounded-circt-tip.md",
        "docs/fr187-handshake-lower-deepen.md",
        "docs/fr188-community-style-guide-pack.md",
        "docs/fr190-further-fr142-api-expand.md",
        "docs/fr191-phase23-claim-honesty.md",
    ] {
        assert!(
            root().join(rel).is_file(),
            "missing claim pointer doc {rel}"
        );
    }
}
