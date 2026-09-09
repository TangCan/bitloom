//! ATDD / guardrail: Epic 36 NFR14 risk record for contract-green /
//! roadmap redefine (Story 36.1 / AD-28 / FR87 / FR93 / NFR38). Red if
//! file missing or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic36_contract_green_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic36-contract-green.md");
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

    // Scope: only rewrite P5–P7 green labels / remaining gaps; no rollback of FR46–86
    assert!(
        (text.contains("P5") || text.contains("阶段五") || text.contains("阶段 五"))
            && (text.contains("P7") || text.contains("阶段七") || text.contains("阶段 七")),
        "risk record must mention roadmap stages P5–P7"
    );
    assert!(
        (text.contains("绿") || text.contains("完成定义") || text.contains("合同绿"))
            && (text.contains("改写") || text.contains("重定义") || text.contains("重写")),
        "risk record must state this phase rewrites green / completion definitions"
    );
    assert!(
        text.contains("FR46")
            && (text.contains("FR86") || text.contains("FR46–86") || text.contains("FR46-86"))
            && (text.contains("不回滚") || text.contains("不 回滚") || text.contains("禁止回滚")),
        "risk record must forbid rolling back delivered FR46–FR86"
    );

    // Relationship to 2026-08-21 ①C
    assert!(
        (text.contains("2026-08-21") || text.contains("①C") || text.contains("1C"))
            && (text.contains("拒绝") || text.contains("重定义") || text.contains("概述")),
        "risk record must explain relationship to 2026-08-21 ①C / redefine-done decision"
    );

    // Forbidden: claim seven-stage literal green without doc updates
    assert!(
        (text.contains("七阶段") || text.contains("字面"))
            && (text.contains("全绿") || text.contains("产品做完") || text.contains("字面全绿"))
            && (text.contains("不得") || text.contains("禁止"))
            && (text.contains("文档") || text.contains("未改") || text.contains("doc")),
        "risk record must forbid claiming seven-stage literal green without doc updates"
    );

    // Forbidden: mark permanent non-goals as done
    assert!(
        (text.contains("永久非目标") || text.contains("FR93"))
            && (text.contains("不得") || text.contains("禁止"))
            && (text.contains("done") || text.contains("标成") || text.contains("交差")),
        "risk record must forbid marking permanent non-goals as done"
    );

    // Gate: 36.2–36.3 must not be ready without this record
    assert!(
        text.contains("36.2")
            && text.contains("36.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 36.2–36.3 from ready without this record"
    );

    assert!(
        text.contains("FR87") && text.contains("FR93"),
        "risk record must cite FR87 and FR93"
    );
    assert!(
        text.contains("NFR38"),
        "risk record must cite NFR38 (contract-green ≠ literal seven-stage)"
    );
    assert!(
        text.contains("Epic 36") || text.contains("Epic36"),
        "risk record must name Epic 36"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR38") && text.contains("负责人"),
        "risk record must assign NFR38 ownership alongside NFR14"
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
        text.contains("contract-green")
            || text.contains("合同绿")
            || text.contains("Correct Course"),
        "risk record must name contract-green / Correct Course context"
    );
}
