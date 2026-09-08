//! ATDD / guardrail: PRD Phase 9 closures contract (Story 26.4).
//! Asserts FR72–FR78 / NFR35–NFR36 / FR16 clarification are writable
//! acceptance text in prd.md (+ addendum gate links).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn prd_text() -> String {
    let path =
        workspace_root().join("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn addendum_text() -> String {
    let path = workspace_root()
        .join("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn prd_phase9_has_fr72_through_fr78_and_nfr35_36() {
    let text = prd_text();

    for id in ["FR72", "FR73", "FR74", "FR75", "FR76", "FR77", "FR78"] {
        assert!(
            text.contains(id),
            "prd.md must contain writable acceptance text for {id}"
        );
    }
    for id in ["NFR35", "NFR36"] {
        assert!(
            text.contains(id),
            "prd.md must contain writable acceptance text for {id}"
        );
    }

    // Changelog / dating
    assert!(
        text.contains("2026-09-08")
            && (text.contains("Phase 9") || text.contains("phase9") || text.contains("闭包")),
        "prd.md changelog header must note Phase 9 closures dated 2026-09-08"
    );

    // ID notes: after FR71/NFR34; no confusion with FR47 / Phase 7
    assert!(
        text.contains("FR71")
            && text.contains("NFR34")
            && (text.contains("之后") || text.contains("接在")),
        "ID notes must place FR72+/NFR35+ after FR71/NFR34"
    );
    assert!(
        text.contains("FR47")
            && (text.contains("不是")
                || text.contains("禁止")
                || text.contains("混淆")
                || text.contains("≠")),
        "prd must disambiguate Phase 9 closures from FR47"
    );
}

#[test]
fn prd_phase9_key_phrases_elaborate_freeze_cap_r58_fr16_gate() {
    let text = prd_text();

    // elaborate-time
    assert!(
        text.contains("elaborate-time") || text.contains("Elaborate-time"),
        "prd Phase 9 must name elaborate-time closures"
    );

    // freeze dissolve
    assert!(
        (text.contains("freeze") || text.contains("`freeze`"))
            && (text.contains("消解") || text.contains("溶解")),
        "prd must require dissolve before freeze"
    );

    // Cap-R-58 / no FIRRTL closure nodes
    assert!(
        text.contains("Cap-R-58")
            && (text.contains("FIRRTL") || text.contains("Chisel"))
            && (text.contains("不") || text.contains("不得") || text.contains("非目标")),
        "prd must state Cap-R-58: no FIRRTL/Chisel closure nodes"
    );

    // FR16 capture ban still
    assert!(
        text.contains("FR16")
            && text.contains("捕获")
            && (text.contains("拒绝") || text.contains("拒") || text.contains("禁")),
        "prd must clarify FR16 capture ban still holds"
    );

    // Epic 27–30 gate on NFR14 + this amendment
    assert!(
        (text.contains("Epic 27") || text.contains("27–30") || text.contains("Epic **27"))
            && text.contains("NFR14")
            && (text.contains("ready") || text.contains("门禁")),
        "prd must gate Epic 27–30 ready on NFR14 + this amendment"
    );

    // Public success criteria Waves: generator → synthesizable → HLS/IP → bridge
    assert!(
        (text.contains("生成器") || text.contains("FR73"))
            && (text.contains("可综合") || text.contains("FR74") || text.contains("FR75"))
            && (text.contains("HLS") || text.contains("FR76"))
            && (text.contains("桥接") || text.contains("FR78")),
        "prd success criteria must align with Waves: generator → synthesizable → HLS/IP → bridge"
    );
}

#[test]
fn addendum_phase9_links_decision_table_and_nfr14() {
    let text = addendum_text();

    assert!(
        text.contains("2026-09-08")
            && (text.contains("FR72") || text.contains("Phase 9"))
            && (text.contains("NFR35") || text.contains("NFR36")),
        "addendum must have Phase 9 note dated 2026-09-08"
    );
    assert!(
        text.contains("closure-decision-table-2026-09-08"),
        "addendum must link closure decision table"
    );
    assert!(
        text.contains("nfr14-risk-phase9-closures"),
        "addendum must link NFR14 Phase 9 risk record"
    );
    assert!(
        (text.contains("Epic 27") || text.contains("27–30"))
            && text.contains("NFR14")
            && (text.contains("ready") || text.contains("门禁")),
        "addendum must restate Epic 27–30 ready gate"
    );
}
