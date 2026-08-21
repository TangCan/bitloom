//! ATDD / guardrail: Epic 23 NFR14 risk record for visualization
//! (Story 23.1 / AD-28 / FR38+FR49). Red if file missing or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_visualization_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-visualization.md");
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

    // Forbidden silent downgrade: GTKWave-only is not FR49 sole path
    assert!(
        (text.contains("GTKWave") || text.contains("gtkwave"))
            && (text.contains("不得") || text.contains("禁止")),
        "risk record must forbid GTKWave-only as the sole FR49 completion path"
    );
    assert!(
        (text.contains("wave") || text.contains("visualize"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("删")),
        "risk record must forbid silently dropping wave/visualize product entry"
    );
    assert!(
        text.contains("PRD") || text.contains("prd"),
        "silent-downgrade ban must tie to not changing the PRD"
    );

    // Allow VCD/FST render but require product entry
    assert!(
        text.contains("VCD")
            && (text.contains("FST") || text.contains("fst"))
            && (text.contains("产品") || text.contains("入口") || text.contains("命令")),
        "risk record must allow VCD/FST render while requiring a product entry"
    );

    // Gate: 23.2–23.5 must not be ready without this record
    assert!(
        text.contains("23.2")
            && text.contains("23.5")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 23.2–23.5 from ready without this record"
    );

    // Cite covering FRs / epic
    assert!(text.contains("FR38"), "risk record must cover FR38");
    assert!(text.contains("FR49"), "risk record must cover FR49");
    assert!(
        text.contains("Epic 23") || text.contains("Epic23"),
        "risk record must name Epic 23"
    );
    assert!(
        text.contains("AD-28") || text.contains("NFR14"),
        "risk record must cite NFR14/AD-28 gate"
    );
}
