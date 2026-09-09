//! ATDD / guardrail: Epic 32 NFR14 risk record for nested Bundle depth
//! (Story 32.1 / AD-28 / AD-20 / FR80). Red if file missing or required
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
fn nfr14_risk_epic32_nested_bundle_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic32-nested-bundle.md");
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

    // Nested depth limit assumptions
    assert!(
        (text.contains("嵌套深度") || text.contains("深度上限") || text.contains("一层"))
            && (text.contains("假设") || text.contains("合同") || text.contains("上限")),
        "risk record must document nested depth limit assumptions"
    );

    // Distinction from FR51 minimal contract
    assert!(
        text.contains("FR51")
            && (text.contains("最小合同")
                || text.contains("flatten")
                || text.contains("OUT OF SCOPE"))
            && (text.contains("区别")
                || text.contains("对照")
                || text.contains("vs")
                || text.contains("加深")),
        "risk record must distinguish FR80 depth from FR51 minimal contract"
    );
    assert!(
        text.contains("NFR37"),
        "risk record must cite NFR37 (planning done ≠ depth done)"
    );

    // Forbidden: must not only delete OUT OF SCOPE comment without implementation
    assert!(
        text.contains("OUT OF SCOPE")
            && (text.contains("删除") || text.contains("删") || text.contains("改写"))
            && (text.contains("不得") || text.contains("禁止")),
        "risk record must forbid deleting OUT OF SCOPE comments without implementation"
    );

    // Gate: 32.2–32.4 must not be ready without this record
    assert!(
        text.contains("32.2")
            && text.contains("32.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 32.2–32.4 from ready without this record"
    );

    assert!(text.contains("FR80"), "risk record must cover FR80");
    assert!(
        text.contains("AD-20"),
        "risk record must cite AD-20 Bundle/Vec synthesizable rule"
    );
    assert!(
        text.contains("Bundle") && (text.contains("嵌套") || text.contains("nested")),
        "risk record must name nested Bundle"
    );
    assert!(
        text.contains("Epic 32") || text.contains("Epic32"),
        "risk record must name Epic 32"
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
