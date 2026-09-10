//! ATDD: Epic 59 NFR14 for FR118 full-tree #[bitloom::top] syn-scan (Story 59.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic59_syn_scan_design_root_discovery_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic59-syn-scan-design-root-discovery.md",
    );
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(text.contains("禁止的静默降级") && text.contains("(c)"));
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR118"));

    assert!(
        text.contains("扫描范围") || text.contains("Workspace 扫描"),
        "must nail workspace scan scope"
    );
    assert!(
        text.contains("#[bitloom::top]")
            && (text.contains("识别") || text.contains("syn-scan") || text.contains("syn")),
        "must nail #[bitloom::top] recognition / syn-scan strategy"
    );
    assert!(
        (text.contains("失败") || text.contains("无 metadata"))
            && (text.contains("根") || text.contains("root") || text.contains("解析")),
        "must nail no-metadata root resolution / failure semantics"
    );
    assert!(
        (text.contains("共存") || text.contains("metadata"))
            && (text.contains("design_roots") || text.contains("Cargo") || text.contains("FR113")),
        "must nail coexistence with Cargo-graph+metadata"
    );

    assert!(
        text.contains("DesignFixture")
            && (text.contains("不得") || text.contains("alone") || text.contains("≠")),
        "must ban DesignFixture alone"
    );
    assert!(
        text.contains("design_roots")
            && (text.contains("不得") || text.contains("alone") || text.contains("≠")),
        "must ban metadata design_roots alone"
    );
    assert!(
        (text.contains("shallow") || text.contains("Shallow") || text.contains("浅层"))
            && (text.contains("finish") || text.contains("伪装") || text.contains("不得")),
        "must ban shallow faking finish"
    );
    assert!(
        text.contains("docs-only")
            || text.contains("仅改文档")
            || (text.contains("文档") && text.contains("不得")),
        "must ban docs-only close"
    );
    assert!(
        text.contains("59.2")
            && text.contains("59.3")
            && text.contains("ready")
            && text.contains("不得")
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR51"));
    assert!(text.contains("NFR48"));
    assert!(text.contains("NFR14-crates"));
    assert!(
        text.contains("证明义务") || text.contains("夹具") || text.contains("工具"),
        "must document proof/fixture/tool obligations"
    );
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(
        text.contains("bitloom-prelude") || text.contains("bitloom_prelude"),
        "must cite bitloom-prelude design dependency boundary"
    );
}
