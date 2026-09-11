//! ATDD / guardrail: Epic 72 NFR14 risk record for Phase 16
//! NFR55 final-closeout / FR133 (Story 72.1 / AD-28 / NFR56–59).
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
fn nfr14_risk_epic72_phase16_nfr55_final_closeout_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic72-phase16-nfr55-final-closeout.md",
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

    // NFR56: Phase 12–15 closed faces must not be rewritten as failure
    assert!(
        text.contains("NFR56")
            && (text.contains("Phase 12")
                || text.contains("Phase 13")
                || text.contains("Phase 14")
                || text.contains("Phase 15")
                || text.contains("FR94")
                || text.contains("FR124"))
            && (text.contains("不得") || text.contains("保留") || text.contains("仍有效")),
        "risk record must state NFR56 Phase 12–15 isolation"
    );
    assert!(
        text.contains("已关闭")
            && (text.contains("失败") || text.contains("改写") || text.contains("不得")),
        "risk record must forbid rewriting FR94–132 closed as failed"
    );

    // FR134–139 deepen scope summary
    assert!(text.contains("FR134"), "risk record must summarize FR134");
    assert!(text.contains("FR135"), "risk record must summarize FR135");
    assert!(text.contains("FR136"), "risk record must summarize FR136");
    assert!(text.contains("FR137"), "risk record must summarize FR137");
    assert!(text.contains("FR138"), "risk record must summarize FR138");
    assert!(text.contains("FR139"), "risk record must summarize FR139");

    // ADs / toolchain that must be synced (AD-27→FR138; CIRCT→FR137; crate→FR139)
    assert!(
        text.contains("AD-27"),
        "risk record must list AD-27 for sync (Parser restore / FR138)"
    );
    assert!(
        text.contains("CIRCT") || text.contains("circt"),
        "risk record must list external CIRCT for sync (FR137)"
    );
    assert!(
        text.contains("crate") || text.contains("跨 crate") || text.contains("bitloom-prelude"),
        "risk record must document crate-boundary sync for FR139"
    );

    // Forbidden: open 73–78 before FR133 / Epic 72 gate
    assert!(
        (text.contains("73") || text.contains("73–78") || text.contains("73-78"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("缺"))
            && (text.contains("FR133") || text.contains("Epic 72") || text.contains("ready")),
        "risk record must forbid opening Epic 73–78 before FR133 / Epic 72 gate"
    );

    // Forbidden: silent swallow NFR59 under "终局" slogan
    assert!(
        text.contains("NFR59")
            && (text.contains("静默")
                || text.contains("吞并")
                || text.contains("终局")
                || text.contains("子集")),
        "risk record must forbid silent swallow of NFR59 under final-closeout slogan"
    );

    // Gate: 72.2–72.4 must not be ready without this record
    assert!(
        text.contains("72.2")
            && text.contains("72.3")
            && text.contains("72.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 72.2–72.4 from ready without this record"
    );

    assert!(text.contains("FR133"), "risk record must cite FR133");
    assert!(
        text.contains("Epic 72") || text.contains("Epic72"),
        "risk record must name Epic 72"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR56") && text.contains("负责人"),
        "risk record must assign NFR56 ownership alongside NFR14"
    );
    assert!(
        (text.contains("NFR57") || text.contains("NFR58") || text.contains("NFR59"))
            && text.contains("负责人"),
        "risk record must assign NFR56–59 ownership (at least one of NFR57–59 named with owner)"
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
        text.contains("NFR55")
            && (text.contains("终局")
                || text.contains("升格")
                || text.contains("closeout")
                || text.contains("加深")),
        "risk record must name Phase 16 NFR55 final-closeout contract"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom"),
        "risk record must cite Bitloom brand"
    );
    // Soft order 78 → 74/75
    assert!(
        (text.contains("78") && (text.contains("74") || text.contains("75")))
            && (text.contains("软序") || text.contains("先于") || text.contains("→")),
        "risk record must document soft order 78 before 74/75"
    );
}
