//! ATDD: Epic 81 NFR14 for FR143/FR144 SemVer 1.0 policy + CI.
//!
//! ```text
//! cargo test -p bitloom --test nfr14_risk_epic81_semver_1_0_ci
//! ```

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic81_semver_1_0_ci_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic81-semver-1-0-ci.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(
        text.contains("上游约束") && (text.contains("(a)") || text.contains("（a）")),
        "must include (a)"
    );
    assert!(
        text.contains("粗工期带") && (text.contains("(b)") || text.contains("（b）")),
        "must include (b)"
    );
    assert!(
        (text.contains("禁止的静默降级") || text.contains("禁止静默降级"))
            && (text.contains("(c)") || text.contains("（c）")),
        "must include (c)"
    );
    assert!(
        text.contains("负责人") && (text.contains("(d)") || text.contains("（d）")),
        "must include (d)"
    );

    assert!(
        text.contains("semver-1-0-policy") || text.contains("semver-1-0-policy.md"),
        "must name SemVer 1.0 policy path"
    );
    assert!(
        text.contains("cargo-semver-checks")
            && (text.contains("semver-check") || text.contains("just semver")),
        "must name cargo-semver-checks and just/CI entry"
    );
    assert!(
        (text.contains("缺工具") || text.contains("missing") || text.contains("不可用"))
            && (text.contains("非零") || text.contains("失败") || text.contains("refuse")),
        "must state missing-tool non-zero failure"
    );
    assert!(
        text.contains("continue-on-error") || text.contains("静默成功"),
        "must forbid continue-on-error / silent success"
    );
    assert!(
        text.contains("Q5") && (text.contains("MSRV") || text.contains("1.97")),
        "must document Q5 MSRV"
    );
    assert!(
        text.contains("NFR15") && (text.contains("0.x") || text.contains("1.0")),
        "must relate to NFR15 / 0.x"
    );
    assert!(
        text.contains("81.2")
            && text.contains("81.3")
            && text.contains("81.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "must gate 81.2–81.4"
    );
    assert!(
        text.contains("FR143") && text.contains("FR144"),
        "must cite FR143 and FR144"
    );
    assert!(
        text.contains("NFR14-crates"),
        "must disambiguate NFR14-crates"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom"),
        "must cite Bitloom"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard"),
        "must name owner"
    );
}
