//! ATDD: Epic 62 NFR14 for FR121 Handshake default synthesizable (Story 62.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic62_handshake_default_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic62-handshake-default.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(text.contains("禁止的静默降级") && text.contains("(c)"));
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR121"));
    assert!(
        text.contains("Handshake") || text.contains("handshake"),
        "must name Handshake"
    );

    // H1–H4 checklist
    assert!(
        text.contains("H1")
            && (text.contains("默认语义") || text.contains("默认") && text.contains("范围")),
        "must nail H1 default semantic scope"
    );
    assert!(
        text.contains("H2")
            && (text.contains("AD-18") || text.contains("dissolve") || text.contains("溶解")),
        "must nail H2 AD-18 dissolve relation"
    );
    assert!(
        text.contains("H3")
            && (text.contains("发射") || text.contains("验收") || text.contains("emit")),
        "must nail H3 emit/acceptance predicates"
    );
    assert!(
        text.contains("H4")
            && (text.contains("失败") || text.contains("failure") || text.contains("可读")),
        "must nail H4 failure semantics"
    );

    // Must revise AD-25
    assert!(
        text.contains("AD-25")
            && (text.contains("修订") || text.contains("revise") || text.contains("Revised")),
        "must require AD-25 revision"
    );

    // ready/valid or token handshake
    assert!(
        text.contains("ready") && text.contains("valid")
            || text.contains("握手")
            || text.contains("token"),
        "must describe ready/valid or equivalent handshake channels"
    );

    // Forbidden closes
    assert!(
        (text.contains("loop-unroll") || text.contains("in-tree-mvp") || text.contains("MVP"))
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "must ban MVP stub / loop-unroll alone"
    );
    assert!(
        text.contains("FR110")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "must ban FR110 alone"
    );
    assert!(
        text.contains("docs-only") || text.contains("仅改文档"),
        "must ban docs-only close"
    );
    assert!(
        text.contains("AD-25")
            && (text.contains("未修订") || text.contains("未改") || text.contains("不得未")),
        "must ban claim without AD-25 revise"
    );

    // NFR48 isolation of prior closes
    assert!(
        text.contains("FR95") && text.contains("FR96") && text.contains("FR110"),
        "must reference FR95/96/FR110 isolation"
    );
    assert!(text.contains("NFR48"));

    // Deferred honesty (NFR51)
    assert!(
        text.contains("NFR51")
            && (text.contains("deferred")
                || text.contains("非目标")
                || text.contains("未列入")
                || text.contains("allocation")),
        "must disclose NFR51 deferred / non-goals"
    );

    // Gate + owners
    assert!(
        text.contains("62.2")
            && text.contains("62.3")
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
