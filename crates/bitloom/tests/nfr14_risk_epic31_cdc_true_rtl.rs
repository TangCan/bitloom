//! ATDD / guardrail: Epic 31 NFR14 risk record for CDC true-RTL depth
//! (Story 31.1 / AD-28 / AD-29 / FR79). Red if file missing or required
//! sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic31_cdc_true_rtl_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic31-cdc-true-rtl.md");
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

    // Honesty risk vs historical Epic 7 「done」(NFR37)
    assert!(
        text.contains("Epic 7")
            && (text.contains("done") || text.contains("`done`") || text.contains("「done」"))
            && (text.contains("诚实") || text.contains("NFR37") || text.contains("深度")),
        "risk record must address honesty risk vs historical Epic 7 done"
    );
    assert!(
        text.contains("NFR37"),
        "risk record must cite NFR37 (planning done ≠ depth done)"
    );

    // Dual-FF latency / metastability documentation boundaries
    assert!(
        (text.contains("双 FF")
            || text.contains("双触发")
            || text.contains("两级")
            || text.contains("延迟"))
            && (text.contains("亚稳态") || text.contains("metastability") || text.contains("MTBF")),
        "risk record must document dual-FF latency / metastability boundaries"
    );

    // Forbidden: must not only change docs to claim true RTL
    assert!(
        (text.contains("仅改文档") || text.contains("只改文档") || text.contains("仅文档"))
            && (text.contains("真 RTL") || text.contains("可综合"))
            && (text.contains("不得") || text.contains("禁止")),
        "risk record must forbid claiming true RTL by documentation-only changes"
    );

    // Current crates posture: ZST + mark_cdc_bridge
    assert!(
        text.contains("ZST") && text.contains("mark_cdc_bridge"),
        "risk record must acknowledge current ZST + mark_cdc_bridge posture"
    );

    // Gate: 31.2–31.4 must not be ready without this record
    assert!(
        text.contains("31.2")
            && text.contains("31.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 31.2–31.4 from ready without this record"
    );

    assert!(text.contains("FR79"), "risk record must cover FR79");
    assert!(
        text.contains("AD-29"),
        "risk record must cite AD-29 true-RTL CDC rule"
    );
    assert!(
        text.contains("DoubleFlop") && text.contains("SyncFIFO"),
        "risk record must name DoubleFlop and SyncFIFO"
    );
    assert!(
        text.contains("Epic 31") || text.contains("Epic31"),
        "risk record must name Epic 31"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR37") && text.contains("负责人"),
        "risk record must assign NFR37 ownership alongside NFR14"
    );
    assert!(
        text.contains("AD-28") || text.contains("NFR14"),
        "risk record must cite NFR14/AD-28 gate"
    );
    assert!(
        text.contains("NFR14-crates"),
        "risk record must disambiguate NFR14-crates from NFR14 gate"
    );
}
