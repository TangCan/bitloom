//! ATDD / guardrail: Epic 20 NFR14 risk record for Chisel bidirectional
//! (Story 20.1 / AD-28 / FR28+FR46). Red if file missing or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_chisel_bidirectional_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-chisel-bidirectional.md");
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

    // Upstream pins + CIRCT / no Scala FIRRTL Parser / #4899
    assert!(
        text.contains("Chisel") && text.contains("firtool"),
        "risk record must name Chisel/firtool pin pair"
    );
    assert!(
        text.contains("7.14.0") && text.contains("1.155.0"),
        "risk record must cite pinned Chisel 7.14.0 and firtool 1.155.0"
    );
    assert!(
        text.contains("4899")
            && (text.contains("Parser")
                || text.contains("Scala FIRRTL")
                || text.contains("FIRRTL Parser")),
        "risk record must cite issue #4899 / no Scala FIRRTL Parser"
    );

    // Forbidden silent downgrade includes FR28 结构化尽力失败 / 尽力失败
    assert!(
        text.contains("FR28")
            && (text.contains("结构化尽力失败") || text.contains("尽力失败"))
            && (text.contains("不得") || text.contains("禁止")),
        "risk record must forbid silently downgrading FR28 back to structured best-effort failure"
    );
    assert!(
        text.contains("PRD") || text.contains("prd"),
        "silent-downgrade ban for FR28 must tie to not changing the PRD"
    );

    // FR46 option A/B/C direction (addendum sketch)
    assert!(
        (text.contains("选项 A") || text.contains("A."))
            && (text.contains("选项 B") || text.contains("B."))
            && (text.contains("选项 C") || text.contains("C.")),
        "risk record must reference FR46 options A, B, and C"
    );
    assert!(
        text.contains("拟选")
            && (text.contains("ASSUMPTION")
                || text.contains("[ASSUMPTION]")
                || text.contains("选项 A")
                || text.contains("A.")),
        "risk record must state chosen FR46 option direction (may be ASSUMPTION)"
    );

    // Gate: 20.2–20.5 must not be ready without this record
    assert!(
        text.contains("20.2")
            && text.contains("20.5")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 20.2–20.5 from ready without this record"
    );

    // Cite covering FRs / epic
    assert!(text.contains("FR46"), "risk record must cover FR46");
    assert!(
        text.contains("Epic 20") || text.contains("Epic20"),
        "risk record must name Epic 20"
    );
    assert!(
        text.contains("AD-28") || text.contains("NFR14"),
        "risk record must cite NFR14/AD-28 gate"
    );
}
