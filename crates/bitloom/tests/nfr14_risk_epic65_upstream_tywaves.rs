//! ATDD: Epic 65 NFR14 for FR125 upstream Tywaves (Story 65.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic65_upstream_tywaves_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic65-upstream-tywaves.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(text.contains("禁止的静默降级") && text.contains("(c)"));
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR125"));
    assert!(text.contains("T1") && text.contains("wave.tywaves.json"));
    assert!(text.contains("T2") && text.contains("BITLOOM_TYWAVES_BIN"));
    assert!(text.contains("T3") && (text.contains("非零") || text.contains("FORCE_MISSING")));
    assert!(text.contains("T4") && text.contains("ATDD"));
    assert!(
        text.contains("FR117")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得"))
    );
    assert!(text.contains("FR104") && text.contains("FR114"));
    assert!(text.contains("docs-only") || text.contains("仅改文档"));
    assert!(
        text.contains("65.2")
            && text.contains("65.3")
            && text.contains("ready")
            && text.contains("不得")
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR52") && text.contains("NFR55"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(text.contains("bitloom-prelude") || text.contains("bitloom_prelude"));
}
