//! ATDD / guardrail: Epic 40 NFR14 risk record for literal Path B / FR94
//! (Story 40.1 / AD-28 / NFR40–43 / B1 vs research). Red if file missing
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
fn nfr14_risk_epic40_literal_path_b_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic40-literal-path-b.md");
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

    // B1 forced choice vs research advising against literal full green
    assert!(
        (text.contains("B1") || text.contains("强制"))
            && (text.contains("Path B") || text.contains("字面")),
        "risk record must state B1 / Path B forced product choice"
    );
    assert!(
        text.contains("research")
            || text.contains("调研")
            || text.contains("technical-doc19-seven-stage-full-green"),
        "risk record must cite research that advised against literal full green"
    );
    assert!(
        text.contains("勿走")
            || text.contains("建议勿")
            || text.contains("否决")
            || text.contains("冲突"),
        "risk record must explain conflict with research advice against literal green"
    );

    // FR93 five-item overturn scope
    assert!(text.contains("FR93"), "risk record must cite FR93");
    assert!(
        text.contains("树内") && (text.contains("HLS") || text.contains("调度")),
        "risk record must list FR93#1 in-tree HLS overturn scope"
    );
    assert!(
        text.contains("idiomatic") || text.contains("可维护") || text.contains("Scala"),
        "risk record must list FR93#2 idiomatic Chisel/Scala overturn scope"
    );
    assert!(
        text.contains("TLM") || text.contains("形式") || text.contains("FL≡RTL"),
        "risk record must list FR93#3 formal/TLM≡CA overturn scope"
    );
    assert!(
        text.contains("VIP") || text.contains("全协议"),
        "risk record must list FR93#4 VIP / full-protocol IP overturn scope"
    );
    assert!(
        text.contains("LSP")
            && (text.contains("elaborate") || text.contains("按键") || text.contains("netlist")),
        "risk record must list FR93#5 full-elaborate LSP overturn scope"
    );

    // NFR40 multi-year / high maintenance
    assert!(
        text.contains("NFR40")
            && (text.contains("多年") || text.contains("高维护") || text.contains("维护")),
        "risk record must cite NFR40 multi-year / high-maintenance risk"
    );

    // ADs that must be synced (at least AD-5 / AD-25 / AD-27)
    assert!(text.contains("AD-5"), "risk record must list AD-5 for sync");
    assert!(
        text.contains("AD-25"),
        "risk record must list AD-25 for sync"
    );
    assert!(
        text.contains("AD-27"),
        "risk record must list AD-27 for sync"
    );

    // Forbidden: open 41–47 before FR94 / Epic 40 gate
    assert!(
        (text.contains("41") || text.contains("41–47") || text.contains("41-47"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("缺"))
            && (text.contains("FR94") || text.contains("Epic 40") || text.contains("ready")),
        "risk record must forbid opening Epic 41–47 before FR94 / Epic 40 gate"
    );

    // Forbidden: claim literal green via FR87 contract-green
    assert!(
        text.contains("FR87")
            && (text.contains("不得") || text.contains("禁止") || text.contains("冒充"))
            && (text.contains("字面") || text.contains("Path B") || text.contains("全绿")),
        "risk record must forbid using FR87 contract-green to claim literal Path B done"
    );

    // Forbidden: silent half-baked LSP / HLS pretending to be literal bars
    assert!(
        (text.contains("半成品") || text.contains("静默"))
            && (text.contains("LSP") || text.contains("HLS"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("冒充")),
        "risk record must forbid silent half-baked LSP/HLS as literal completion"
    );

    // Gate: 40.2–40.4 must not be ready without this record
    assert!(
        text.contains("40.2")
            && text.contains("40.3")
            && text.contains("40.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 40.2–40.4 from ready without this record"
    );

    assert!(text.contains("FR94"), "risk record must cite FR94");
    assert!(
        text.contains("Epic 40") || text.contains("Epic40"),
        "risk record must name Epic 40"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR40") && text.contains("负责人"),
        "risk record must assign NFR40 ownership alongside NFR14"
    );
    assert!(
        (text.contains("NFR41") || text.contains("NFR42") || text.contains("NFR43"))
            && text.contains("负责人"),
        "risk record must assign NFR40–43 ownership (at least one of NFR41–43 named with owner)"
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
}
