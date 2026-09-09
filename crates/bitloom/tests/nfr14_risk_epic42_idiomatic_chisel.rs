//! ATDD / guardrail: Epic 42 NFR14 risk record for idiomatic Chisel
//! (Story 42.1 / AD-28 / AD-27 / FR97). Red if file missing
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
fn nfr14_risk_epic42_idiomatic_chisel_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic42-idiomatic-chisel.md");
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

    // Idiomatic acceptance criteria: naming / structure / readability or official style subset
    assert!(
        text.contains("idiomatic") || text.contains("可维护"),
        "risk record must discuss idiomatic / maintainable acceptance"
    );
    assert!(
        text.contains("命名")
            && (text.contains("结构") || text.contains("层次"))
            && (text.contains("可读") || text.contains("可读性")),
        "risk record must define idiomatic criteria covering naming, structure, and readability"
    );
    assert!(
        text.contains("官方风格") || text.contains("Style Guide") || text.contains("风格子集"),
        "risk record must mention official style subset (optional nail-down path)"
    );

    // Relationship to official non-support of FIRRTL→Scala Circuit
    assert!(
        (text.contains("FIRRTL") || text.contains(".fir"))
            && (text.contains("Scala") || text.contains("Circuit"))
            && (text.contains("官方") || text.contains("不支持") || text.contains("#4899")),
        "risk record must relate FR97 to official FIRRTL→Scala Circuit non-support reality"
    );
    assert!(
        text.contains("Parser") || text.contains("Parser.parse") || text.contains("firrtl.Parser"),
        "risk record must address abandoned Scala FIRRTL Parser"
    );

    // Forbidden: docs-only re-label mechanical emit as idiomatic
    assert!(
        (text.contains("机械") || text.contains("emit_chisel") || text.contains("FR28"))
            && (text.contains("文案") || text.contains("文档") || text.contains("仅改"))
            && (text.contains("不得") || text.contains("禁止"))
            && (text.contains("idiomatic") || text.contains("FR97")),
        "risk record must forbid re-labeling mechanical emit as idiomatic by docs alone"
    );

    // Forbidden: require upstream to restore abandoned Parser without alternate contract
    assert!(
        (text.contains("恢复") || text.contains("要求上游"))
            && (text.contains("Parser") || text.contains("Parser.parse"))
            && (text.contains("替代") || text.contains("合同"))
            && (text.contains("不得") || text.contains("禁止")),
        "risk record must forbid requiring upstream Parser restore without an alternate contract"
    );

    // Gate: 42.2–42.3 must not be ready without this record
    assert!(
        text.contains("42.2")
            && text.contains("42.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 42.2–42.3 from ready without this record"
    );

    assert!(
        text.contains("AD-27"),
        "risk record must cite revised AD-27"
    );
    assert!(text.contains("FR97"), "risk record must name FR97");
    assert!(
        text.contains("Epic 42") || text.contains("Epic42"),
        "risk record must name Epic 42"
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
    assert!(
        text.contains("FR28") || text.contains("FR46"),
        "risk record must contrast mechanical FR28/FR46 path with FR97"
    );
}
