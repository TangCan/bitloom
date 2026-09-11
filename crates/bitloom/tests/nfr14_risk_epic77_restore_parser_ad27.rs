//! ATDD / guardrail: Epic 77 NFR14 risk record for FR138
//! restore deprecated Scala Parser.parse + re-revise AD-27 (Story 77.1 / NFR56–59).
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
fn nfr14_risk_epic77_restore_parser_ad27_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic77-restore-parser-ad27.md");
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

    // FR138 / Epic 77 identity
    assert!(text.contains("FR138"), "risk record must cite FR138");
    assert!(
        text.contains("Epic 77") || text.contains("Epic77"),
        "risk record must name Epic 77"
    );

    // Parser restore shape: API / workflow / version pairing
    assert!(
        text.contains("Parser.parse")
            || text.contains("firrtl.Parser")
            || (text.contains("Parser") && text.contains("API")),
        "risk record must nail Parser restore API shape"
    );
    assert!(
        text.contains("工作流") || text.contains("workflow") || text.contains("路径"),
        "risk record must nail Parser restore workflow"
    );
    assert!(
        (text.contains("7.14") || text.contains("Chisel"))
            && (text.contains("1.155") || text.contains("firtool") || text.contains("配对")),
        "risk record must nail Chisel/firtool version pairing"
    );

    // Acceptance predicates P1–P4
    assert!(
        text.contains("P1") && text.contains("P2") && text.contains("P3") && text.contains("P4"),
        "risk record must nail P1–P4 acceptance predicates"
    );

    // Failure semantics
    assert!(
        (text.contains("失败语义") || text.contains("failure"))
            && (text.contains("非零") || text.contains("non-zero") || text.contains("nonzero"))
            && (text.contains("可读") || text.contains("readable") || text.contains("失败")),
        "risk record must nail non-zero readable failure semantics"
    );

    // FR130 Style Guide / Parser-not-restored boundary
    assert!(
        text.contains("FR130")
            && (text.contains("Style Guide")
                || text.contains("style-guide")
                || text.contains("S3"))
            && (text.contains("未恢复")
                || text.contains("不恢复")
                || text.contains("not restored")
                || text.contains("alone")),
        "risk record must nail FR130 Style Guide / Parser-not-restored boundary"
    );

    // Forbid FR97 / FR111 / FR122 O1–O4 alone
    assert!(
        text.contains("FR97")
            && text.contains("FR111")
            && text.contains("FR122")
            && (text.contains("O1") || text.contains("O4"))
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must forbid closing on FR97/111/122 O1–O4 alone"
    );

    // Forbid FR130 Style Guide alone
    assert!(
        text.contains("FR130")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must forbid closing on FR130 Style Guide alone"
    );

    // Forbid docs-only
    assert!(
        text.contains("docs-only") || (text.contains("docs") && text.contains("不得")),
        "risk record must forbid docs-only close"
    );

    // Forbid claim close without revising AD-27
    assert!(
        text.contains("AD-27")
            && (text.contains("修订") || text.contains("revise") || text.contains("再修订"))
            && (text.contains("不得") || text.contains("未修订") || text.contains("NFR58")),
        "risk record must forbid claiming close without revising AD-27"
    );

    // Correct Course trail (may reuse Phase 16) + re-revise AD-27 before implementation
    assert!(
        (text.contains("Correct Course") || text.contains("correctCourse"))
            && (text.contains("Phase 16")
                || text.contains("2026-09-11")
                || text.contains("复用")
                || text.contains("reuse")),
        "risk record must require Correct Course trail (may reuse Phase 16 gate)"
    );
    assert!(
        text.contains("AD-27") && text.contains("NFR58"),
        "risk record must require re-revising AD-27 under NFR58 before implementation close"
    );

    // Gate: 77.2–77.3 must not be ready without this record
    assert!(
        text.contains("77.2")
            && text.contains("77.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 77.2–77.3 from ready without this record"
    );

    // Owner including NFR58 AD-27
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR58")
            && (text.contains("AD-27") || text.contains("修订") || text.contains("同步")),
        "risk record must assign NFR58 AD-27 revise ownership"
    );
    assert!(
        text.contains("NFR56") && text.contains("NFR59"),
        "risk record must assign NFR56/NFR59 ownership"
    );
    assert!(
        text.contains("NFR14-crates"),
        "risk record must disambiguate NFR14-crates from NFR14 gate"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom"),
        "risk record must cite Bitloom brand"
    );
}
