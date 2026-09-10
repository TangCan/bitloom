//! ATDD / guardrail: Epic 48 NFR14 risk record for Phase 13
//! MVP→commercial deepen / FR106 (Story 48.1 / AD-28 / NFR44–47).
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
fn nfr14_risk_epic48_mvp_commercial_deepen_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic48-mvp-commercial-deepen.md");
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

    // NFR44: Phase 12 MVP boundary must not be rewritten as failure
    assert!(
        text.contains("NFR44")
            && (text.contains("Phase 12") || text.contains("FR94"))
            && (text.contains("不得") || text.contains("保留") || text.contains("仍有效")),
        "risk record must state NFR44 Phase 12 MVP isolation"
    );
    assert!(
        text.contains("已关闭")
            && (text.contains("失败") || text.contains("改写") || text.contains("不得")),
        "risk record must forbid rewriting FR94–105 closed as failed"
    );

    // FR107–114 deepen scope summary
    assert!(text.contains("FR107"), "risk record must summarize FR107");
    assert!(text.contains("FR108"), "risk record must summarize FR108");
    assert!(text.contains("FR109"), "risk record must summarize FR109");
    assert!(text.contains("FR110"), "risk record must summarize FR110");
    assert!(text.contains("FR111"), "risk record must summarize FR111");
    assert!(text.contains("FR112"), "risk record must summarize FR112");
    assert!(text.contains("FR113"), "risk record must summarize FR113");
    assert!(text.contains("FR114"), "risk record must summarize FR114");

    // ADs that must be synced (at least AD-5 / AD-25 / AD-27)
    assert!(text.contains("AD-5"), "risk record must list AD-5 for sync");
    assert!(
        text.contains("AD-25"),
        "risk record must list AD-25 for sync"
    );
    assert!(
        text.contains("AD-27"),
        "risk record must list AD-27 for sync"
    );

    // Forbidden: open 49–56 before FR106 / Epic 48 gate
    assert!(
        (text.contains("49") || text.contains("49–56") || text.contains("49-56"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("缺"))
            && (text.contains("FR106") || text.contains("Epic 48") || text.contains("ready")),
        "risk record must forbid opening Epic 49–56 before FR106 / Epic 48 gate"
    );

    // Forbidden: silent expand beyond risk-record subset (NFR47)
    assert!(
        text.contains("NFR47")
            && (text.contains("静默") || text.contains("扩大") || text.contains("子集")),
        "risk record must forbid silent scope expand beyond risk-record subset"
    );

    // Gate: 48.2–48.4 must not be ready without this record
    assert!(
        text.contains("48.2")
            && text.contains("48.3")
            && text.contains("48.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 48.2–48.4 from ready without this record"
    );

    assert!(text.contains("FR106"), "risk record must cite FR106");
    assert!(
        text.contains("Epic 48") || text.contains("Epic48"),
        "risk record must name Epic 48"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR44") && text.contains("负责人"),
        "risk record must assign NFR44 ownership alongside NFR14"
    );
    assert!(
        (text.contains("NFR45") || text.contains("NFR46") || text.contains("NFR47"))
            && text.contains("负责人"),
        "risk record must assign NFR44–47 ownership (at least one of NFR45–47 named with owner)"
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
        text.contains("商业加深") || text.contains("MVP→") || text.contains("mvp-to-commercial"),
        "risk record must name Phase 13 commercial-deepen contract"
    );
}
