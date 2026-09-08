//! ATDD / guardrail: Epic 29 NFR14 risk record for HLS/IP closure customization
//! (Story 29.1 / AD-28 / FR76+FR77). Red if file missing or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic29_hls_ip_closures_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic29-hls-ip-closures.md");
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

    // D1: HLS free only on external path; SynthesizableClosure for synthesizable IP leg
    assert!(
        text.contains("D1"),
        "risk record must cite decision-table D1"
    );
    assert!(
        (text.contains("HLS 自由") || text.contains("自由闭包"))
            && (text.contains("外挂") || text.contains("AD-25")),
        "risk record must place HLS-free closures only on external/AD-25 path"
    );
    assert!(
        text.contains("SynthesizableClosure"),
        "risk record must require SynthesizableClosure on synthesizable IP leg"
    );

    // AD-25 / FR86: no in-tree scheduler
    assert!(
        (text.contains("AD-25") || text.contains("FR86"))
            && (text.contains("scheduler") || text.contains("调度"))
            && (text.contains("不得") || text.contains("禁止")),
        "risk record must forbid in-tree HLS scheduler (AD-25 / FR86)"
    );

    // FR35/FR50 external backend interaction
    assert!(
        text.contains("FR35") && text.contains("FR50"),
        "risk record must address FR35/FR50 product HLS path"
    );

    // Epic 34 IP baseline precedes 29.3 (sequencing / silent downgrade)
    assert!(
        text.contains("Epic 34")
            && text.contains("29.3")
            && (text.contains("先于") || text.contains("优先") || text.contains("基线")),
        "risk record must note Epic 34 IP baseline precedes 29.3"
    );

    // Gate: 29.2–29.4 must not be ready without this record
    assert!(
        text.contains("29.2")
            && text.contains("29.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 29.2–29.4 from ready without this record"
    );

    assert!(text.contains("FR76"), "risk record must cover FR76");
    assert!(text.contains("FR77"), "risk record must cover FR77");
    assert!(
        text.contains("Epic 29") || text.contains("Epic29"),
        "risk record must name Epic 29"
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
        text.contains("NFR14-crates"),
        "risk record must disambiguate NFR14-crates from NFR14 gate"
    );
}
