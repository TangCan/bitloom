//! ATDD / guardrail: Epic 21 NFR14 risk record for dual-sim generation
//! (Story 21.1 / AD-28 / FR47+FR30). Red if file missing or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_dual_sim_generation_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-dual-sim-generation.md");
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

    // Functional sim form = generated Rust crate (Open Q6 closed)
    assert!(
        text.contains("Rust")
            && (text.contains("crate") || text.contains("Crate"))
            && (text.contains("生成") || text.contains("generate")),
        "risk record must state functional simulator form = generated Rust crate"
    );
    assert!(
        text.contains("Open Q6") || text.contains("Q6"),
        "risk record must cite Open Q6 (closed: Rust crate)"
    );

    // Forbidden silent downgrade: no delete-generation / hand-written-only claim
    assert!(
        (text.contains("仅手写") || text.contains("手写对照") || text.contains("手写 functional"))
            && (text.contains("不得") || text.contains("禁止"))
            && (text.contains("生成") || text.contains("FR47")),
        "risk record must forbid silently dropping generation back to handwritten-only"
    );
    assert!(
        text.contains("PRD") || text.contains("prd"),
        "silent-downgrade ban must tie to not changing the PRD"
    );

    // Forbidden: claim SystemC TLM delivered
    assert!(
        text.contains("SystemC")
            && (text.contains("TLM") || text.contains("TLM-2.0"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("不承诺")),
        "risk record must forbid claiming SystemC TLM as delivered"
    );

    // Gate: 21.2–21.5 must not be ready without this record
    assert!(
        text.contains("21.2")
            && text.contains("21.5")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 21.2–21.5 from ready without this record"
    );

    // Cite covering FRs / epic
    assert!(text.contains("FR47"), "risk record must cover FR47");
    assert!(text.contains("FR30"), "risk record must cover FR30");
    assert!(
        text.contains("Epic 21") || text.contains("Epic21"),
        "risk record must name Epic 21"
    );
    assert!(
        text.contains("AD-28") || text.contains("NFR14"),
        "risk record must cite NFR14/AD-28 gate"
    );
}
