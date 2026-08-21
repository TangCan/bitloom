//! ATDD / guardrail: Epic 24 NFR14 risk record for HLS product path
//! (Story 24.1 / AD-28 / FR35+FR50). Red if file missing or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_hls_has_required_fields() {
    let path = workspace_root().join("_agile-output/implementation-artifacts/nfr14-risk-hls.md");
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

    // Single pinned backend + version strategy
    assert!(
        text.contains("Bambu") || text.contains("bambu"),
        "risk record must name selected backend Bambu"
    );
    assert!(
        text.contains("2024.10"),
        "risk record must cite pinned Bambu version 2024.10"
    );
    assert!(
        text.contains("Vitis") || text.contains("XLS"),
        "risk record must acknowledge Bambu|Vitis choice (and state non-selected)"
    );
    assert!(
        text.contains("GPLv3") || text.contains("GPL"),
        "risk record must note Bambu license constraint"
    );
    assert!(
        text.contains("CI") || text.contains("烟测"),
        "risk record must address CI availability"
    );

    // Forbidden silent downgrades
    assert!(
        (text.contains("永久 unsupported") || text.contains("unsupported"))
            && (text.contains("不得") || text.contains("禁止")),
        "risk record must forbid permanent-unsupported as product answer"
    );
    assert!(
        (text.contains("scheduler") || text.contains("调度"))
            && (text.contains("不得") || text.contains("禁止")),
        "risk record must forbid in-tree scheduler"
    );

    // Gate: 24.2–24.4 must not be ready without this record
    assert!(
        text.contains("24.2")
            && text.contains("24.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 24.2–24.4 from ready without this record"
    );

    assert!(text.contains("FR35"), "risk record must cover FR35");
    assert!(text.contains("FR50"), "risk record must cover FR50");
    assert!(
        text.contains("Epic 24") || text.contains("Epic24"),
        "risk record must name Epic 24"
    );
    assert!(
        text.contains("AD-28") || text.contains("NFR14"),
        "risk record must cite NFR14/AD-28 gate"
    );
}
