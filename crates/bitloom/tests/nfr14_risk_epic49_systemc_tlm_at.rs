//! ATDD: Epic 49 NFR14 for FR107 SystemC TLM AT (Story 49.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic49_systemc_tlm_at_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic49-systemc-tlm-at.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(text.contains("禁止的静默降级") && text.contains("(c)"));
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR107"));
    assert!(text.contains("nb_transport") || text.contains("AT"));
    assert!(text.contains("FR101") && (text.contains("LT") || text.contains("不得")));
    assert!(text.contains("FR47") || text.contains("Rust"));
    assert!(text.contains("AD-5") && text.contains("NFR46"));
    assert!(
        text.contains("49.2")
            && text.contains("49.3")
            && text.contains("ready")
            && text.contains("不得")
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("bitloom-prelude") || text.contains("Bitloom"));
    assert!(text.contains("文档口号") || (text.contains("不得") && text.contains("文档")));
}
