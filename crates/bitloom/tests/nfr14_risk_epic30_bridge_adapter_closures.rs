//! ATDD / guardrail: Epic 30 NFR14 risk record for bridge-adapter / multi-view
//! closures (Story 30.1 / AD-28 / FR78). Red if file missing or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic30_bridge_adapter_closures_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic30-bridge-adapter-closures.md",
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

    // AC: TLM↔信号泄漏风险
    assert!(
        (text.contains("TLM") || text.contains("事务"))
            && (text.contains("泄漏") || text.contains("信号")),
        "risk record must address TLM↔signal leakage risk"
    );

    // AC: 模板误用
    assert!(
        text.contains("模板") && (text.contains("误用") || text.contains("不得把桥接")),
        "risk record must address template misuse"
    );

    // AC: 与 FR16 混淆
    assert!(
        text.contains("FR16") && (text.contains("混淆") || text.contains("捕获")),
        "risk record must address confusion with FR16 / capture ban"
    );

    // Gate: 30.2–30.4 must not be ready without this record
    assert!(
        text.contains("30.2")
            && text.contains("30.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 30.2–30.4 from ready without this record"
    );

    // FR78 / Epic 30 / FR47 dependency
    assert!(text.contains("FR78"), "risk record must cover FR78");
    assert!(
        text.contains("Epic 30") || text.contains("Epic30"),
        "risk record must name Epic 30"
    );
    assert!(
        text.contains("FR47"),
        "risk record must cite FR47 dual-view path dependency"
    );

    // View boundary + NFR36 / AD-18
    assert!(
        (text.contains("功能") && text.contains("周期")) || text.contains("视图边界"),
        "risk record must document functional vs cycle-accurate view boundary"
    );
    assert!(
        text.contains("NFR36") || text.contains("闭包残留") || text.contains("闭包 IR"),
        "risk record must address dissolve-before-freeze / no closure IR (NFR36)"
    );
    assert!(
        text.contains("AD-18") || text.contains("freeze"),
        "risk record must cite AD-18 dissolve-before-freeze contract"
    );

    assert!(
        text.contains("负责人") && text.contains("Richard"),
        "risk record must name an owner (NFR14)"
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
        text.contains("start_wait_complete") || text.contains("Cap-R-65"),
        "risk record must name start_wait_complete template or Cap-R-65"
    );
}
