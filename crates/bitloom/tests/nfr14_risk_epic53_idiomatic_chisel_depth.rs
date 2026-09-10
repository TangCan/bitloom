//! ATDD: Epic 53 NFR14 for FR111 idiomatic Chisel maintainability depth (Story 53.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic53_idiomatic_chisel_depth_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic53-idiomatic-chisel-depth.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(text.contains("禁止的静默降级") && text.contains("(c)"));
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR111"));
    assert!(
        text.contains("D1") || text.contains("D2") || text.contains("D3"),
        "must expose D* deepen gates"
    );
    assert!(
        (text.contains("多模块")
            || text.contains("命名")
            || text.contains("层次")
            || text.contains("加严"))
            && (text.contains("风格") || text.contains("合同") || text.contains("规则")),
        "must nail at least one deepen acceptance class"
    );
    assert!(
        (text.contains("机械") || text.contains("emit_chisel") || text.contains("FR28"))
            && (text.contains("不得") || text.contains("禁止")),
        "must ban mechanical emit alone as FR111"
    );
    assert!(
        text.contains("Parser") && (text.contains("不得") || text.contains("禁止")),
        "must ban requiring dead Parser"
    );
    assert!(
        text.contains("53.2")
            && text.contains("53.3")
            && text.contains("ready")
            && text.contains("不得")
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR46") || text.contains("AD-27"));
    assert!(text.contains("NFR47"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("FR97") && (text.contains("仍") || text.contains("不得")));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
}
