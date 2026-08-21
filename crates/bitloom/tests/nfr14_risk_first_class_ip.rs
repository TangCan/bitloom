//! ATDD / guardrail: Epic 22 NFR14 risk record for first-class IP
//! (Story 22.1 / AD-28 / FR37+FR48). Red if file missing or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_first_class_ip_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-first-class-ip.md");
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

    // Forbidden silent downgrade: must not shrink five categories to FIFO-only
    assert!(
        (text.contains("仅 FIFO") || text.contains("仅一类"))
            && (text.contains("不得") || text.contains("禁止")),
        "risk record must forbid silently shrinking five IP categories to FIFO-only"
    );
    assert!(
        text.contains("PRD") || text.contains("prd"),
        "silent-downgrade ban must tie to not changing the PRD"
    );

    // AXI locked to AXI4-Lite min slave
    assert!(
        text.contains("AXI4-Lite")
            && (text.contains("最小从") || text.contains("从接口"))
            && (text.contains("Open Q7") || text.contains("Q7")),
        "risk record must lock AXI scope to AXI4-Lite min slave (Open Q7)"
    );

    // Governance: in-tree vs org-published
    assert!(
        (text.contains("树内") || text.contains("workspace"))
            && (text.contains("组织") || text.contains("crates.io") || text.contains("发布")),
        "risk record must note in-tree vs org-published governance preference"
    );
    assert!(
        text.contains("稳定") && (text.contains("深绑") || text.contains("prelude")),
        "risk record must prefer stabilizing before deep-binding into prelude"
    );

    // Gate: 22.2–22.6 must not be ready without this record
    assert!(
        text.contains("22.2")
            && text.contains("22.6")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 22.2–22.6 from ready without this record"
    );

    // Cite covering FRs / epic
    assert!(text.contains("FR37"), "risk record must cover FR37");
    assert!(text.contains("FR48"), "risk record must cover FR48");
    assert!(
        text.contains("Epic 22") || text.contains("Epic22"),
        "risk record must name Epic 22"
    );
    assert!(
        text.contains("AD-28") || text.contains("NFR14"),
        "risk record must cite NFR14/AD-28 gate"
    );
}
