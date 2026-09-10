//! ATDD: Epic 52 NFR14 for FR110 in-tree HLS commercial depth (Story 52.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic52_hls_commercial_depth_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic52-hls-commercial-depth.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(text.contains("禁止的静默降级") && text.contains("(c)"));
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR110"));
    assert!(
        text.contains("流水")
            || text.contains("pipeline")
            || text.contains("II")
            || text.contains("ii")
            || text.contains("质量"),
        "must nail at least one quality gate class"
    );
    assert!(
        text.contains("Q1") || text.contains("Q2") || text.contains("Q3"),
        "must expose Q* acceptance gates"
    );
    assert!(
        (text.contains("MVP") || text.contains("stub") || text.contains("展开"))
            && (text.contains("不得") || text.contains("禁止")),
        "must ban MVP stub / unroll rebrand as commercial"
    );
    assert!(
        (text.contains("Bambu") || text.contains("外挂"))
            && (text.contains("不得") || text.contains("禁止")),
        "must ban Bambu stub alone closing FR110"
    );
    assert!(
        text.contains("52.2")
            && text.contains("52.3")
            && text.contains("ready")
            && text.contains("不得")
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR46") || text.contains("AD-25"));
    assert!(text.contains("NFR47"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("FR95") && (text.contains("仍") || text.contains("不得")));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
}
