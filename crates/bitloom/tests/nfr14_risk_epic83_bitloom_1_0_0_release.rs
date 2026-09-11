//! ATDD: Epic 83 NFR14 for FR146 Bitloom 1.0.0 release.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic83_bitloom_1_0_0_release_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic83-bitloom-1-0-0-release.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(
        (text.contains("禁止的静默降级") || text.contains("禁止静默降级")) && text.contains("(c)")
    );
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR146"));
    assert!(text.contains("1.0.0") && text.contains("v1.0.0"));
    assert!(text.contains("CHANGELOG") || text.contains("Changelog"));
    assert!(text.contains("dry-run") || text.contains("dry run") || text.contains("dry_run"));
    assert!(
        text.contains("bitloom-prelude")
            && text.contains("bitloom-sim")
            && (text.contains("bitloom-macro") || text.contains("macro"))
    );
    assert!(
        text.contains("83.2")
            && text.contains("83.3")
            && (text.contains("ready") || text.contains("`ready`"))
    );
    assert!(
        text.contains("FR141")
            || text.contains("FR145")
            || text.contains("145-skip")
            || text.contains("未关")
    );
    assert!(text.contains("NFR59") && text.contains("NFR14-crates"));
    assert!(text.contains("Richard"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
}
