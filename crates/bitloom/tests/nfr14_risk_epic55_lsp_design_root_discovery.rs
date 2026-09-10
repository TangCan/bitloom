//! ATDD: Epic 55 NFR14 for FR113 LSP design-root discovery deepen (Story 55.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic55_lsp_design_root_discovery_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic55-lsp-design-root-discovery.md",
    );
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(text.contains("禁止的静默降级") && text.contains("(c)"));
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR113"));

    assert!(
        text.contains("选定") || text.contains("钉死") || text.contains("Selected"),
        "must nail discovery strategy"
    );
    assert!(
        text.contains("Cargo")
            && (text.contains("metadata")
                || text.contains("design_roots")
                || text.contains("Cargo-graph")),
        "must name Cargo-graph / metadata design_roots strategy"
    );
    assert!(
        text.contains("DesignFixture")
            && (text.contains("alone") || text.contains("≠") || text.contains("不得")),
        "must ban DesignFixture MVP alone as FR113"
    );
    assert!(
        text.contains("FR90")
            && (text.contains("rust-analyzer")
                || text.contains("不替代")
                || text.contains("alone")),
        "must isolate vs FR90"
    );
    assert!(
        (text.contains("HTML") || text.contains("FR38") || text.contains("可视化"))
            && (text.contains("不得") || text.contains("≠")),
        "must ban HTML viz as FR113"
    );
    assert!(
        (text.contains("浅层") || text.contains("Shallow") || text.contains("浅层诊断"))
            && (text.contains("不得") || text.contains("alone")),
        "must ban shallow-only close"
    );
    assert!(
        text.contains("半成品") || text.contains("二进制"),
        "must ban half-baked binary delivery"
    );
    assert!(
        (text.contains("deferred") || text.contains("未选"))
            && (text.contains("syn") || text.contains("#[bitloom::top]") || text.contains("全")),
        "unselected full syn-scan must stay deferred"
    );
    assert!(
        text.contains("55.2")
            && text.contains("55.3")
            && text.contains("ready")
            && text.contains("不得")
    );
    assert!(text.contains("性能") || text.contains("超时") || text.contains("P3"));
    assert!(text.contains("范围") || text.contains("oversized") || text.contains("P4"));
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR47"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("FR99") && (text.contains("仍") || text.contains("不得")));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
}
