//! ATDD / guardrail: Epic 73 NFR14 risk record for FR134
//! Upstream Tywaves GUI / IDE plugin depth (Story 73.1 / NFR56–59).
//! Red if file missing or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic73_upstream_tywaves_gui_ide_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic73-upstream-tywaves-gui-ide.md",
    );
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    // (a)–(d) mandatory NFR14 fields
    assert!(
        text.contains("上游约束") && (text.contains("(a)") || text.contains("（a）")),
        "risk record must include labeled field (a) 上游约束"
    );
    assert!(
        text.contains("粗工期带") && (text.contains("(b)") || text.contains("（b）")),
        "risk record must include labeled field (b) 粗工期带"
    );
    assert!(
        (text.contains("禁止的静默降级") || text.contains("禁止静默降级"))
            && (text.contains("(c)") || text.contains("（c）")),
        "risk record must include labeled field (c) 禁止的静默降级清单"
    );
    assert!(
        text.contains("负责人") && (text.contains("(d)") || text.contains("（d）")),
        "risk record must include labeled field (d) 负责人"
    );

    // FR134 / Epic 73 identity
    assert!(text.contains("FR134"), "risk record must cite FR134");
    assert!(
        text.contains("Epic 73") || text.contains("Epic73"),
        "risk record must name Epic 73"
    );

    // GUI install package and/or IDE plugin shape G1–G4
    assert!(
        text.contains("G1") && text.contains("G2") && text.contains("G3") && text.contains("G4"),
        "risk record must nail GUI/IDE depth shape G1–G4"
    );
    assert!(
        (text.contains("GUI") || text.contains("安装包"))
            && (text.contains("IDE") || text.contains("插件")),
        "risk record must nail upstream GUI install and/or IDE plugin"
    );
    assert!(
        text.contains("版本") && (text.contains("发行") || text.contains("渠道")),
        "risk record must nail version / distribution channel"
    );
    assert!(
        text.contains("元数据") || text.contains("契约") || text.contains("schemaVersion"),
        "risk record must nail metadata contract"
    );

    // Acceptance predicates + failure semantics
    assert!(
        text.contains("验收") || text.contains("谓词") || text.contains("ATDD"),
        "risk record must nail acceptance predicates"
    );
    assert!(
        (text.contains("失败") || text.contains("非零") || text.contains("FORCE"))
            && (text.contains("silent") || text.contains("静默") || text.contains("不得")),
        "risk record must nail failure semantics (no silent green)"
    );

    // Boundary with FR125 T1–T4
    assert!(
        text.contains("FR125")
            && text.contains("T1")
            && text.contains("T4")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must bound against FR125 T1–T4 alone"
    );

    // Forbid FR104 / FR114 / FR117 alone + docs-only
    assert!(
        text.contains("FR104")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must forbid closing on FR104 alone"
    );
    assert!(
        text.contains("FR114")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must forbid closing on FR114 alone"
    );
    assert!(
        text.contains("FR117")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must forbid closing on FR117 alone"
    );
    assert!(
        text.contains("docs-only") || text.contains("仅改文档") || text.contains("不得 docs-only"),
        "risk record must forbid docs-only closeout"
    );

    // Gate: 73.2–73.3 must not be ready without this record
    assert!(
        text.contains("73.2")
            && text.contains("73.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 73.2–73.3 from ready without this record"
    );

    // Owner NFR14 / NFR56 / NFR59
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR56"),
        "risk record must cite NFR56 (prior closed faces remain valid)"
    );
    assert!(
        text.contains("NFR59"),
        "risk record must cite NFR59 (unlisted deeper GUI/IDE still needs new contract)"
    );
    assert!(
        text.contains("NFR14-crates"),
        "risk record must disambiguate NFR14-crates from NFR14 gate"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom"),
        "risk record must cite Bitloom brand"
    );
    assert!(
        text.contains("bitloom-prelude")
            || text.contains("bitloom_prelude")
            || text.contains("设计 crate"),
        "risk record must keep Tywaves runtime out of design-crate deps / cite prelude boundary"
    );
}
