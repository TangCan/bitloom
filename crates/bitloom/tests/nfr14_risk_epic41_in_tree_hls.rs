//! ATDD / guardrail: Epic 41 NFR14 risk record for in-tree HLS
//! (Story 41.1 / AD-28 / AD-25 / FR95 / FR96). Red if file missing
//! or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic41_in_tree_hls_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic41-in-tree-hls.md");
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

    // Coexistence: in-tree vs external (Bambu / FR35 / FR86)
    assert!(
        (text.contains("树内") || text.contains("in-tree") || text.contains("in_tree"))
            && (text.contains("外挂") || text.contains("Bambu") || text.contains("可选")),
        "risk record must state in-tree vs external coexistence strategy"
    );
    assert!(
        text.contains("FR95") && (text.contains("FR35") || text.contains("FR86")),
        "risk record must relate FR95 completion to existing FR35/FR86 external path"
    );

    // Scope: scheduling/allocation entering crates
    assert!(
        (text.contains("scheduling") || text.contains("调度"))
            && (text.contains("allocation") || text.contains("分配") || text.contains("crate")),
        "risk record must bound scheduling/allocation entering bitloom/rhdl crates"
    );

    // FR96 ↔ AD-18 dissolve rules
    assert!(
        text.contains("FR96")
            && text.contains("AD-18")
            && (text.contains("溶解")
                || text.contains("消解")
                || text.contains("freeze")
                || text.contains("非捕获")),
        "risk record must relate FR96 closure transform to AD-18 dissolve rules"
    );

    // Forbidden: docs-only claim of in-tree HLS delivered
    assert!(
        (text.contains("文档") || text.contains("仅改文档"))
            && (text.contains("不得") || text.contains("禁止"))
            && (text.contains("树内") || text.contains("HLS") || text.contains("FR95")),
        "risk record must forbid claiming in-tree HLS delivered by docs alone"
    );

    // Forbidden: stub / BITLOOM_HLS_USE_REAL external path as FR95 done
    assert!(
        (text.contains("stub") || text.contains("BITLOOM_HLS_USE_REAL"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("冒充"))
            && text.contains("FR95"),
        "risk record must forbid marking stub/BITLOOM_HLS_USE_REAL external path as FR95 done"
    );

    // Forbidden: silent expand to uncontracted dynamic dataflow default
    assert!(
        (text.contains("动态数据流") || text.contains("Handshake") || text.contains("动态"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("silent"))
            && (text.contains("默认") || text.contains("扩大") || text.contains("语义")),
        "risk record must forbid silent expansion to uncontracted dynamic dataflow defaults"
    );

    // Gate: 41.2–41.4 must not be ready without this record
    assert!(
        text.contains("41.2")
            && text.contains("41.3")
            && text.contains("41.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 41.2–41.4 from ready without this record"
    );

    assert!(
        text.contains("AD-25"),
        "risk record must cite revised AD-25"
    );
    assert!(
        text.contains("Epic 41") || text.contains("Epic41"),
        "risk record must name Epic 41"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR41") && text.contains("负责人"),
        "risk record must assign NFR41 ownership alongside NFR14"
    );
    assert!(
        text.contains("AD-28") || text.contains("NFR14"),
        "risk record must cite NFR14/AD-28 gate"
    );
    assert!(
        text.contains("NFR14-crates"),
        "risk record must disambiguate NFR14-crates from NFR14 gate"
    );
    assert!(
        text.contains("bitloom-prelude") || text.contains("bitloom_prelude"),
        "risk record must cite bitloom-prelude design dependency boundary"
    );
    assert!(
        text.contains("Epic 40") || text.contains("FR94"),
        "risk record must acknowledge Epic 40 / FR94 gate already closed"
    );
}
