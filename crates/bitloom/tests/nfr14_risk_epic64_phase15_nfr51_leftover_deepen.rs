//! ATDD / guardrail: Epic 64 NFR14 risk record for Phase 15
//! NFR51 leftover-deepen / FR124 (Story 64.1 / AD-28 / NFR52–55).
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
fn nfr14_risk_epic64_phase15_nfr51_leftover_deepen_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic64-phase15-nfr51-leftover-deepen.md",
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

    // NFR52: Phase 12–14 closed faces must not be rewritten as failure
    assert!(
        text.contains("NFR52")
            && (text.contains("Phase 12")
                || text.contains("Phase 13")
                || text.contains("Phase 14")
                || text.contains("FR94")
                || text.contains("FR116"))
            && (text.contains("不得") || text.contains("保留") || text.contains("仍有效")),
        "risk record must state NFR52 Phase 12–14 isolation"
    );
    assert!(
        text.contains("已关闭")
            && (text.contains("失败") || text.contains("改写") || text.contains("不得")),
        "risk record must forbid rewriting FR94–123 closed as failed"
    );

    // FR125–131 deepen scope summary
    assert!(text.contains("FR125"), "risk record must summarize FR125");
    assert!(text.contains("FR126"), "risk record must summarize FR126");
    assert!(text.contains("FR127"), "risk record must summarize FR127");
    assert!(text.contains("FR128"), "risk record must summarize FR128");
    assert!(text.contains("FR129"), "risk record must summarize FR129");
    assert!(text.contains("FR130"), "risk record must summarize FR130");
    assert!(text.contains("FR131"), "risk record must summarize FR131");

    // ADs that must be synced (at least AD-25 / AD-27); sby CI ops
    assert!(
        text.contains("AD-25"),
        "risk record must list AD-25 for sync (CIRCT Handshake)"
    );
    assert!(
        text.contains("AD-27"),
        "risk record must list AD-27 for sync (Style Guide/Parser)"
    );
    assert!(
        text.contains("sby")
            || text.contains("SBY")
            || text.contains("SymbiYosys")
            || text.contains("CI"),
        "risk record must document sby CI ops notes"
    );

    // Forbidden: open 65–71 before FR124 / Epic 64 gate
    assert!(
        (text.contains("65") || text.contains("65–71") || text.contains("65-71"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("缺"))
            && (text.contains("FR124") || text.contains("Epic 64") || text.contains("ready")),
        "risk record must forbid opening Epic 65–71 before FR124 / Epic 64 gate"
    );

    // Forbidden: silent expand beyond risk-record subset (NFR55)
    assert!(
        text.contains("NFR55")
            && (text.contains("静默") || text.contains("扩大") || text.contains("子集")),
        "risk record must forbid silent scope expand beyond risk-record subset"
    );

    // Gate: 64.2–64.4 must not be ready without this record
    assert!(
        text.contains("64.2")
            && text.contains("64.3")
            && text.contains("64.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 64.2–64.4 from ready without this record"
    );

    assert!(text.contains("FR124"), "risk record must cite FR124");
    assert!(
        text.contains("Epic 64") || text.contains("Epic64"),
        "risk record must name Epic 64"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR52") && text.contains("负责人"),
        "risk record must assign NFR52 ownership alongside NFR14"
    );
    assert!(
        (text.contains("NFR53") || text.contains("NFR54") || text.contains("NFR55"))
            && text.contains("负责人"),
        "risk record must assign NFR52–55 ownership (at least one of NFR53–55 named with owner)"
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
        text.contains("NFR51")
            && (text.contains("剩余") || text.contains("加深") || text.contains("leftover")),
        "risk record must name Phase 15 NFR51 leftover-deepen contract"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom"),
        "risk record must cite Bitloom brand"
    );
}
