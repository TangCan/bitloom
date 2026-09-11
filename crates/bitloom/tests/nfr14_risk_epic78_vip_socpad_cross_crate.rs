//! ATDD / guardrail: Epic 78 NFR14 risk record for FR139
//! VIP/SocPad further split or cross-crate (Story 78.1 / NFR56–59).
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
fn nfr14_risk_epic78_vip_socpad_cross_crate_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic78-vip-socpad-cross-crate.md");
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

    // FR139 / Epic 78 identity
    assert!(text.contains("FR139"), "risk record must cite FR139");
    assert!(
        text.contains("Epic 78") || text.contains("Epic78"),
        "risk record must name Epic 78"
    );

    // Split / cross-crate diagram C1–C4
    assert!(
        text.contains("C1") && text.contains("C2") && text.contains("C3") && text.contains("C4"),
        "risk record must nail split/cross-crate diagram C1–C4"
    );
    assert!(
        (text.contains("VIP") || text.contains("SocPad") || text.contains("GpioVip"))
            && (text.contains("再细拆") || text.contains("细拆") || text.contains("拆分")),
        "risk record must nail VIP/SocPad further split"
    );
    assert!(
        text.contains("跨 crate") || text.contains("cross-crate") || text.contains("跨 crate"),
        "risk record must document cross-crate option"
    );

    // Public API / bitloom_prelude::ip::* stability or migration
    assert!(
        text.contains("bitloom_prelude::ip")
            || text.contains("bitloom_prelude::ip::*")
            || (text.contains("bitloom-prelude") && text.contains("ip::")),
        "risk record must cite public bitloom_prelude::ip::* path"
    );
    assert!(
        text.contains("稳定") || text.contains("迁移"),
        "risk record must state API stability or migration obligation"
    );

    // Regression duties: FR98 + FR108/120/128 + FR131
    assert!(
        text.contains("FR98"),
        "risk record must list FR98 regression"
    );
    assert!(
        text.contains("FR108"),
        "risk record must list FR108 regression"
    );
    assert!(
        text.contains("FR120"),
        "risk record must list FR120 regression"
    );
    assert!(
        text.contains("FR128"),
        "risk record must list FR128 regression"
    );
    assert!(
        text.contains("FR131"),
        "risk record must list FR131 regression"
    );

    // Forbid FR131 P1–P4 alone
    assert!(
        text.contains("P1")
            && text.contains("P4")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must forbid closing on FR131 P1–P4 alone"
    );

    // Forbid silent export breaks
    assert!(
        (text.contains("silent") || text.contains("静默"))
            && (text.contains("导出") || text.contains("export") || text.contains("路径")),
        "risk record must forbid silent export breaks"
    );

    // Forbid undocumented AD-6 boundary change
    assert!(
        text.contains("AD-6")
            && (text.contains("不得") || text.contains("禁止") || text.contains("除非"))
            && (text.contains("bitloom-prelude") || text.contains("设计 crate")),
        "risk record must forbid undocumented AD-6 boundary change"
    );

    // Gate: 78.2–78.3 must not be ready without this record
    assert!(
        text.contains("78.2")
            && text.contains("78.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 78.2–78.3 from ready without this record"
    );

    // Soft order before Epic 74/75
    assert!(
        (text.contains("74") || text.contains("75"))
            && text.contains("78")
            && (text.contains("软序") || text.contains("先于") || text.contains("→")),
        "risk record must document soft order Epic 78 before 74/75"
    );

    // Owner including NFR58
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR58") && text.contains("负责人"),
        "risk record must assign NFR58 ownership (crate boundary / AD-6)"
    );
    assert!(
        text.contains("NFR14-crates"),
        "risk record must disambiguate NFR14-crates from NFR14 gate"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom"),
        "risk record must cite Bitloom brand"
    );
    assert!(
        text.contains("NFR56")
            && (text.contains("Phase 12")
                || text.contains("Phase 15")
                || text.contains("仍有效")
                || text.contains("不得")),
        "risk record must cite NFR56 isolation of prior closed faces"
    );
}
