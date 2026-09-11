//! ATDD / guardrail: Epic 79 NFR14 risk record for Phase 17
//! API stability / Bitloom 1.0 / FR141 (Story 79.1 / AD-28 / NFR60–63).
//! Red if file missing or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic79_phase17_api_stability_1_0_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic79-phase17-api-stability-1-0.md",
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

    // NFR60: Phase 12–16 closed faces must not be rewritten as failure
    assert!(
        text.contains("NFR60")
            && (text.contains("Phase 12")
                || text.contains("Phase 16")
                || text.contains("FR94")
                || text.contains("FR140"))
            && (text.contains("不得") || text.contains("保留") || text.contains("仍有效")),
        "risk record must state NFR60 Phase 12–16 isolation"
    );
    assert!(
        text.contains("已关闭")
            && (text.contains("失败") || text.contains("改写") || text.contains("不得")),
        "risk record must forbid rewriting FR94–140 closed as failed"
    );

    // FR142–146 scope summary
    assert!(text.contains("FR142"), "risk record must summarize FR142");
    assert!(text.contains("FR143"), "risk record must summarize FR143");
    assert!(text.contains("FR144"), "risk record must summarize FR144");
    assert!(text.contains("FR145"), "risk record must summarize FR145");
    assert!(text.contains("FR146"), "risk record must summarize FR146");

    // Q1–Q5 approval defaults
    assert!(
        text.contains("Q1") && (text.contains("bitloom-sim") || text.contains("sim")),
        "risk record must document Q1 bitloom-sim surface default"
    );
    assert!(
        text.contains("Q2")
            && (text.contains("hir") || text.contains("builder") || text.contains("vlog")),
        "risk record must document Q2 hir/builder/vlog out-of-promise default"
    );
    assert!(
        text.contains("Q3")
            && (text.contains("skip") || text.contains("82") || text.contains("FR145")),
        "risk record must document Q3 Epic 82 skip path"
    );
    assert!(
        text.contains("Q4") && text.contains("NFR59"),
        "risk record must document Q4 no NFR59 prerequisite"
    );
    assert!(
        text.contains("Q5") && (text.contains("MSRV") || text.contains("msrv")),
        "risk record must document Q5 MSRV default"
    );

    // Forbidden: open 80–83 before FR141 / Epic 79 gate
    assert!(
        (text.contains("80") || text.contains("80–83") || text.contains("80-83"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("缺"))
            && (text.contains("FR141") || text.contains("Epic 79") || text.contains("ready")),
        "risk record must forbid opening Epic 80–83 before FR141 / Epic 79 gate"
    );

    // Forbidden: Phase 16 alone as 1.0; silent swallow NFR59
    assert!(
        text.contains("Phase 16")
            && (text.contains("1.0") || text.contains("公开 API"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("冒充")),
        "risk record must forbid Phase 16 alone posing as 1.0"
    );
    assert!(
        text.contains("NFR59")
            && text.contains("NFR63")
            && (text.contains("静默") || text.contains("吞并") || text.contains("1.0")),
        "risk record must forbid silent swallow of NFR59 under 1.0 slogan (NFR63)"
    );

    // Gate: 79.2–79.4 must not be ready without this record
    assert!(
        text.contains("79.2")
            && text.contains("79.3")
            && text.contains("79.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 79.2–79.4 from ready without this record"
    );

    assert!(text.contains("FR141"), "risk record must cite FR141");
    assert!(
        text.contains("Epic 79") || text.contains("Epic79"),
        "risk record must name Epic 79"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR60") && text.contains("负责人"),
        "risk record must assign NFR60 ownership alongside NFR14"
    );
    assert!(
        (text.contains("NFR61") || text.contains("NFR62") || text.contains("NFR63"))
            && text.contains("负责人"),
        "risk record must assign NFR60–63 ownership (at least one of NFR61–63 named with owner)"
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
        text.contains("1.0")
            && (text.contains("稳定") || text.contains("stability") || text.contains("SemVer")),
        "risk record must name Phase 17 API stability / 1.0 contract"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom"),
        "risk record must cite Bitloom brand"
    );
    // Soft order 79 → 80 → 81
    assert!(
        text.contains("80")
            && text.contains("81")
            && (text.contains("软序") || text.contains("→") || text.contains("79")),
        "risk record must document soft order 79 → 80 → 81"
    );
    // Must not publish before Epic 83
    assert!(
        (text.contains("publish") || text.contains("1.0.0"))
            && (text.contains("83") || text.contains("FR146"))
            && (text.contains("不得") || text.contains("禁止")),
        "risk record must forbid publish/1.0.0 before Epic 83 / FR146"
    );
}
