//! ATDD / guardrail: Epic 46 NFR14 risk record for SystemC TLM-2.0
//! product path (Story 46.1 / AD-28 / FR101 / revised AD-5 / NFR41).
//! Red if file missing or required sections absent.
//!
//! ```text
//! cargo test -p bitloom --test nfr14_risk_epic46_systemc_tlm
//! ```

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic46_systemc_tlm_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic46-systemc-tlm.md");
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

    // TLM-2.0 deliverables: lib / generator / examples / toolchain deps
    assert!(
        (text.contains("TLM-2.0") || text.contains("TLM 2.0") || text.contains("SystemC TLM"))
            && (text.contains("库") || text.contains("lib") || text.contains("D1"))
            && (text.contains("生成器")
                || text.contains("集成")
                || text.contains("generator")
                || text.contains("D2"))
            && (text.contains("示例") || text.contains("example") || text.contains("D3"))
            && (text.contains("依赖")
                || text.contains("工具链")
                || text.contains("toolchain")
                || text.contains("D4")),
        "risk record must nail TLM-2.0 deliverables (lib/generator/examples/toolchain deps)"
    );
    assert!(
        text.contains("D1") && text.contains("D2") && text.contains("D3") && text.contains("D4"),
        "risk record must define concrete deliverable markers (D1–D4 style)"
    );

    // LT / AT scope
    assert!(
        (text.contains("LT") || text.contains("Loosely-Timed") || text.contains("松定时"))
            && (text.contains("AT")
                || text.contains("Approximately-Timed")
                || text.contains("近似定时")),
        "risk record must nail LT/AT scope"
    );

    // Relationship to cycle-accurate path
    assert!(
        (text.contains("周期精确")
            || text.contains("cycle-accurate")
            || text.contains("Cycle-accurate")
            || text.contains("FrozenHir")
            || text.contains("tick"))
            && (text.contains("不替代")
                || text.contains("保留")
                || text.contains("关系")
                || text.contains("≠")
                || text.contains("不替换")),
        "risk record must state relationship to cycle-accurate / FrozenHir tick path"
    );

    // Forbidden: close FR101 on docs slogans alone
    assert!(
        (text.contains("文档")
            || text.contains("口号")
            || text.contains("README")
            || text.contains("stub"))
            && (text.contains("FR101") || text.contains("关闭"))
            && (text.contains("不得")
                || text.contains("禁止")
                || text.contains("仅")
                || text.contains("alone")),
        "risk record must forbid closing FR101 on docs slogans alone"
    );

    // Forbidden: label host Rust FL as SystemC TLM
    assert!(
        (text.contains("Rust") || text.contains("FR47") || text.contains("functional"))
            && (text.contains("SystemC") || text.contains("TLM"))
            && (text.contains("不得")
                || text.contains("禁止")
                || text.contains("冒充")
                || text.contains("≠")
                || text.contains("标成")),
        "risk record must forbid labeling host Rust FL as SystemC TLM"
    );

    // Must not confuse with FR47
    assert!(
        text.contains("FR47")
            && (text.contains("≠")
                || text.contains("不是")
                || text.contains("不得")
                || text.contains("对照")
                || text.contains("混淆")),
        "risk record must contrast FR47 Rust functional sim with FR101"
    );

    // Gate: 46.2–46.3 must not be ready without this record
    assert!(
        text.contains("46.2")
            && text.contains("46.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 46.2–46.3 from ready without this record"
    );

    assert!(text.contains("FR101"), "risk record must name FR101");
    assert!(
        text.contains("Epic 46") || text.contains("Epic46"),
        "risk record must name Epic 46"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR41") && text.contains("负责人"),
        "risk record must assign NFR41 ownership alongside NFR14"
    );
    // Revised AD-5 citation (NFR41)
    assert!(
        text.contains("AD-5")
            && (text.contains("修订")
                || text.contains("revised")
                || text.contains("2026-09-09")
                || text.contains("允许")),
        "risk record must cite revised AD-5 (NFR41)"
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
