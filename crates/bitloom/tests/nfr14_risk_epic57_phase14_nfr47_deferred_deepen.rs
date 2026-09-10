//! ATDD / guardrail: Epic 57 NFR14 risk record for Phase 14
//! NFR47 deferred-deepen / FR116 (Story 57.1 / AD-28 / NFR48–51).
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
fn nfr14_risk_epic57_phase14_nfr47_deferred_deepen_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic57-phase14-nfr47-deferred-deepen.md",
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

    // NFR48: Phase 12/13 closed faces must not be rewritten as failure
    assert!(
        text.contains("NFR48")
            && (text.contains("Phase 12")
                || text.contains("Phase 13")
                || text.contains("FR94")
                || text.contains("FR106"))
            && (text.contains("不得") || text.contains("保留") || text.contains("仍有效")),
        "risk record must state NFR48 Phase 12/13 isolation"
    );
    assert!(
        text.contains("已关闭")
            && (text.contains("失败") || text.contains("改写") || text.contains("不得")),
        "risk record must forbid rewriting FR94–115 closed as failed"
    );

    // FR117–122 deepen scope summary
    assert!(text.contains("FR117"), "risk record must summarize FR117");
    assert!(text.contains("FR118"), "risk record must summarize FR118");
    assert!(text.contains("FR119"), "risk record must summarize FR119");
    assert!(text.contains("FR120"), "risk record must summarize FR120");
    assert!(text.contains("FR121"), "risk record must summarize FR121");
    assert!(text.contains("FR122"), "risk record must summarize FR122");

    // ADs that must be synced (at least AD-25 / AD-27); formal/SBY toolchain
    assert!(
        text.contains("AD-25"),
        "risk record must list AD-25 for sync (Handshake)"
    );
    assert!(
        text.contains("AD-27"),
        "risk record must list AD-27 for sync (Chisel style)"
    );
    assert!(
        text.contains("SBY")
            || text.contains("SymbiYosys")
            || text.contains("SMT")
            || text.contains("formal"),
        "risk record must document formal/SBY toolchain notes"
    );

    // Forbidden: open 58–63 before FR116 / Epic 57 gate
    assert!(
        (text.contains("58") || text.contains("58–63") || text.contains("58-63"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("缺"))
            && (text.contains("FR116") || text.contains("Epic 57") || text.contains("ready")),
        "risk record must forbid opening Epic 58–63 before FR116 / Epic 57 gate"
    );

    // Forbidden: silent expand beyond risk-record subset (NFR51)
    assert!(
        text.contains("NFR51")
            && (text.contains("静默") || text.contains("扩大") || text.contains("子集")),
        "risk record must forbid silent scope expand beyond risk-record subset"
    );

    // Gate: 57.2–57.4 must not be ready without this record
    assert!(
        text.contains("57.2")
            && text.contains("57.3")
            && text.contains("57.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 57.2–57.4 from ready without this record"
    );

    assert!(text.contains("FR116"), "risk record must cite FR116");
    assert!(
        text.contains("Epic 57") || text.contains("Epic57"),
        "risk record must name Epic 57"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR48") && text.contains("负责人"),
        "risk record must assign NFR48 ownership alongside NFR14"
    );
    assert!(
        (text.contains("NFR49") || text.contains("NFR50") || text.contains("NFR51"))
            && text.contains("负责人"),
        "risk record must assign NFR48–51 ownership (at least one of NFR49–51 named with owner)"
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
        text.contains("NFR47")
            && (text.contains("未选") || text.contains("加深") || text.contains("deferred")),
        "risk record must name Phase 14 NFR47 deferred-deepen contract"
    );
}
