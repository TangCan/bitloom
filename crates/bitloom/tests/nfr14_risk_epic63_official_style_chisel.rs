//! ATDD: Epic 63 NFR14 for FR122 official-style Chisel full pack (Story 63.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic63_official_style_chisel_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic63-official-style-chisel.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(text.contains("禁止的静默降级") && text.contains("(c)"));
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR122"));

    // O1–O4 checklist beyond FR111 D1+D3
    assert!(
        text.contains("O1")
            && (text.contains("package") || text.contains("包"))
            && (text.contains("FR122") || text.contains("official")),
        "must nail O1 package + FR122 claim"
    );
    assert!(
        text.contains("O2")
            && (text.contains("分节") || text.contains("顺序") || text.contains("order")),
        "must nail O2 official section order"
    );
    assert!(
        text.contains("O3")
            && (text.contains("FR122 official")
                || text.contains("每模块")
                || text.contains("per-module")
                || text.contains("per module")),
        "must nail O3 per-module FR122 marker"
    );
    assert!(
        text.contains("O4")
            && text.contains("emit_chisel_idiomatic_fr122")
            && text.contains("check_idiomatic_chisel_fr122"),
        "must nail O4 emit/check API shape"
    );

    // Beyond D1+D3
    assert!(
        text.contains("D1") && text.contains("D3"),
        "must position relative to FR111 D1+D3"
    );
    assert!(
        text.contains("增量") || text.contains("beyond") || text.contains("超出"),
        "must state rule-set increment beyond D1+D3"
    );

    // AD-27 revise-if-needed; default forbid Parser
    assert!(
        text.contains("AD-27")
            && (text.contains("修订") || text.contains("revise") || text.contains("Revised")),
        "must address AD-27 revision obligation"
    );
    assert!(
        text.contains("Parser")
            && (text.contains("禁止")
                || text.contains("不得")
                || text.contains("不恢复")
                || text.contains("NOT restoring")
                || text.contains("不另开")),
        "must default-forbid Parser.parse restore"
    );

    // Forbidden closes
    assert!(
        text.contains("FR97")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "must ban FR97 alone"
    );
    assert!(
        text.contains("FR111")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "must ban FR111 alone"
    );
    assert!(
        (text.contains("机械") || text.contains("emit_chisel") || text.contains("FR28"))
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "must ban mechanical emit alone"
    );
    assert!(
        text.contains("docs-only") || text.contains("仅改文档"),
        "must ban docs-only close"
    );

    // NFR48 isolation
    assert!(text.contains("FR97") && text.contains("FR111") && text.contains("NFR48"));

    // Deferred honesty (NFR51)
    assert!(
        text.contains("NFR51")
            && (text.contains("deferred")
                || text.contains("非目标")
                || text.contains("未列入")
                || text.contains("Style Guide")),
        "must disclose NFR51 deferred / non-goals"
    );

    // Gate + owners
    assert!(
        text.contains("63.2")
            && text.contains("63.3")
            && text.contains("ready")
            && text.contains("不得")
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR50"));
    assert!(text.contains("NFR51"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(
        text.contains("bitloom-prelude") || text.contains("bitloom_prelude"),
        "must cite bitloom-prelude design dependency boundary"
    );
}
