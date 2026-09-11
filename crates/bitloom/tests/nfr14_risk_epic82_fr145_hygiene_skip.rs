//! ATDD: Epic 82 NFR14 for FR145 optional hygiene / skip.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic82_fr145_hygiene_skip_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic82-fr145-hygiene-skip.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(
        (text.contains("禁止的静默降级") || text.contains("禁止静默降级")) && text.contains("(c)")
    );
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR145"));
    assert!(
        text.contains("skip") || text.contains("Skip") || text.contains("SKIP"),
        "must document skip path"
    );
    assert!(
        text.contains("NFR59") && (text.contains("禁止") || text.contains("不得")),
        "must forbid NFR59 deepen via hygiene"
    );
    assert!(
        text.contains("82.2")
            && text.contains("82.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 82.2–82.3"
    );
    assert!(text.contains("PortField") || text.contains("prelude") || text.contains("semver"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("Richard") && text.contains("Bitloom") || text.contains("bitloom"));
}
