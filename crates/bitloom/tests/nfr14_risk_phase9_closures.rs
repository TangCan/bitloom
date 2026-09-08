//! ATDD / guardrail: Phase 9 NFR14 risk record for controlled generic closures
//! (Story 26.1 / AD-28 / FR72–78). Red if file missing or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_phase9_closures_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-phase9-closures.md");
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

    // FR16 / AD-18 conflict surface
    assert!(
        text.contains("FR16") && text.contains("AD-18"),
        "risk record must address FR16/AD-18 conflict"
    );
    assert!(
        text.contains("捕获") || text.contains("capture"),
        "risk record must discuss capturing closures"
    );

    // Terminology: generator closures ≠ FR47 ≠ Phase 7 闭环
    assert!(
        text.contains("FR47"),
        "risk record must disambiguate FR47 sim generators"
    );
    assert!(
        text.contains("生成器闭包") || text.contains("generator"),
        "risk record must name generator closures"
    );
    assert!(
        text.contains("闭环") || text.contains("closure"),
        "risk record must disambiguate Phase 7 closure wording"
    );

    // Diagnostic escape / backend ban
    assert!(
        text.contains("诊断") || text.contains("逃逸"),
        "risk record must address diagnostic escape"
    );
    assert!(
        (text.contains("FIRRTL") || text.contains("Chisel"))
            && (text.contains("不得") || text.contains("禁止")),
        "risk record must forbid encoding closures in FIRRTL/Chisel"
    );

    // Hard bans from Story 26.1 AC
    assert!(
        text.contains("AD-18")
            && (text.contains("未修订") || text.contains("26.3"))
            && (text.contains("不得") || text.contains("禁止")),
        "risk record must forbid positive closure API before AD-18 revision"
    );

    // Gate: 26.3–26.4 and Epic 27+
    assert!(
        text.contains("26.3")
            && text.contains("26.4")
            && (text.contains("Epic 27") || text.contains("27+"))
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate 26.3–26.4 and Epic 27+ from ready"
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
        text.contains("Phase 9") || text.contains("FR72"),
        "risk record must situate Phase 9 / FR72+"
    );

    // Automate expansion: identity disambiguation + AD-25/FR86 boundary
    assert!(
        text.contains("NFR14-crates"),
        "risk record must disambiguate NFR14-crates from NFR14 gate"
    );
    assert!(
        (text.contains("AD-25") || text.contains("FR86"))
            && (text.contains("scheduler") || text.contains("调度")),
        "risk record must keep HLS in-tree scheduler out of scope"
    );
}
