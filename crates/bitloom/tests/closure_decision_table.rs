//! ATDD / guardrail: Wave 0 closure decision table (Story 26.2 / FR72).
//! Red if decision page missing or required rulings / glossary absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn decision_table_text() -> String {
    let path = workspace_root().join(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/closure-decision-table-2026-09-08.md",
    );
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn closure_decision_table_has_required_rulings() {
    let text = decision_table_text();

    assert!(
        text.contains("FR72") && (text.contains("ADOPTED") || text.contains("裁决")),
        "decision table must situate FR72 / Wave 0 rulings"
    );

    // D1: HLS free vs synthesizable split
    assert!(
        (text.contains("HLS") || text.contains("AD-25"))
            && (text.contains("自由") || text.contains("外挂"))
            && text.contains("SynthesizableClosure")
            && (text.contains("29") || text.contains("Epic 29")),
        "D1: HLS free only on external/functional path; synthesizable uses SynthesizableClosure; Epic 29"
    );
    assert!(
        (text.contains("scheduler") || text.contains("调度"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("非目标")),
        "D1: must forbid in-tree HLS scheduler"
    );

    // D2: FR75 → Wave 2 / Epic 28
    assert!(
        text.contains("FR75")
            && (text.contains("Wave 2") || text.contains("Wave2"))
            && (text.contains("Epic 28") || text.contains("**28**")),
        "D2: FR75 must enter Wave 2 / Epic 28"
    );

    // D3: const fn vs generator dual-track
    assert!(
        (text.contains("const fn") || text.contains("`const fn`"))
            && (text.contains("生成器") || text.contains("FR73"))
            && (text.contains("双轨") || text.contains("dual"))
            && (text.contains("freeze") || text.contains("消解"))
            && (text.contains("27") || text.contains("Epic 27")),
        "D3: const fn + generator dual-track; dissolve before freeze; Epic 27"
    );

    // D4: Cap-R-58 non-goal
    assert!(
        text.contains("Cap-R-58")
            && (text.contains("FIRRTL") || text.contains("Chisel"))
            && (text.contains("非目标") || text.contains("不得") || text.contains("不编码")),
        "D4: Cap-R-58 must forbid encoding closures in FIRRTL/Chisel"
    );

    // D5: glossary
    assert!(
        text.contains("生成器闭包")
            && text.contains("FR47")
            && (text.contains("闭环") || text.contains("Phase 7"))
            && (text.contains("术语") || text.contains("≠") || text.contains("不是")),
        "D5: glossary must disambiguate generator closures ≠ FR47 ≠ Phase 7 闭环"
    );

    // Effective epics called out
    assert!(
        text.contains("26") && text.contains("27") && text.contains("28") && text.contains("29"),
        "decision table must name effective epics 26/27/28/29"
    );

    assert!(
        text.contains("Bitloom") || text.contains("bitloom-prelude"),
        "decision table must use Bitloom brand / prelude boundary"
    );
}

#[test]
fn fr72_links_to_closure_decision_table() {
    let epics = workspace_root().join("_agile-output/planning-artifacts/epics.md");
    let text =
        fs::read_to_string(&epics).unwrap_or_else(|e| panic!("read {}: {e}", epics.display()));

    assert!(
        text.contains("FR72")
            && text.contains("closure-decision-table-2026-09-08")
            && (text.contains("决策表") || text.contains("decision")),
        "FR72 in epics.md must link to the closure decision table artifact"
    );
}
