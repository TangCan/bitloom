//! ATDD / guardrail: Story 40.2 / FR94 — Correct Course + PRD/addendum
//! Path B gate (overturn FR93; NFR42; Bitloom brand).
//!
//! Red if the approved sprint-change proposal or Phase 12 addendum/prd
//! contract stamp is missing or incomplete. Does **not** require doc-19
//! rewrite (40.3) or AD/README lock revocation (40.4).
//!
//! ```text
//! cargo test -p bitloom --test fr94_prd_path_b_gate
//! ```

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    let path = workspace_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn fr94_correct_course_approved() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-09-phase12-path-b.md",
    );
    assert!(
        text.contains("status: approved") || text.contains("status:approved"),
        "Correct Course proposal must be status: approved"
    );
    assert!(
        text.contains("Phase 12")
            && (text.contains("Path B") || text.contains("字面"))
            && (text.contains("推翻") || text.contains("FR93")),
        "Correct Course must authorize Phase 12 Path B and FR93 overturn"
    );
    assert!(
        text.contains("FR94") || text.contains("FR94–"),
        "Correct Course must cite FR94 gate / FR94–105"
    );
}

#[test]
fn fr94_addendum_phase12_literal_green() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    assert!(
        text.contains("Phase 12")
            && (text.contains("字面绿") || text.contains("字面七阶段"))
            && (text.contains("Path B") || text.contains("B1")),
        "addendum must contain Phase 12 literal-green / Path B section"
    );
    assert!(
        (text.contains("FR94") && text.contains("FR105"))
            || text.contains("FR94–FR105")
            || text.contains("FR94–105"),
        "addendum Phase 12 must cite FR94–FR105"
    );
    assert!(
        (text.contains("NFR40") && text.contains("NFR43"))
            || text.contains("NFR40–NFR43")
            || text.contains("NFR40–43"),
        "addendum Phase 12 must cite NFR40–NFR43"
    );
}

#[test]
fn fr94_addendum_overturns_fr93_five_items() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    assert!(
        text.contains("推翻 FR93") || (text.contains("推翻") && text.contains("FR93")),
        "addendum must explicitly overturn FR93"
    );
    // Five historical non-goals mapped to delivery FRs
    assert!(
        (text.contains("HLS") || text.contains("调度")) && text.contains("FR95"),
        "addendum must map FR93#1 HLS → FR95"
    );
    assert!(
        (text.contains("idiomatic") || text.contains("可维护") || text.contains("Chisel"))
            && text.contains("FR97"),
        "addendum must map FR93#2 idiomatic Chisel → FR97"
    );
    assert!(
        (text.contains("TLM") || text.contains("形式"))
            && (text.contains("FR100") || text.contains("FR101")),
        "addendum must map FR93#3 formal/TLM → FR100/FR101"
    );
    assert!(
        (text.contains("VIP") || text.contains("全协议")) && text.contains("FR98"),
        "addendum must map FR93#4 VIP IP → FR98"
    );
    assert!(
        text.contains("LSP")
            && (text.contains("elaborate") || text.contains("按键") || text.contains("netlist"))
            && text.contains("FR99"),
        "addendum must map FR93#5 full-elaborate LSP → FR99"
    );
}

#[test]
fn fr94_nfr42_claim_discipline_and_fr87_historical() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    assert!(
        text.contains("NFR42"),
        "addendum must cite NFR42 claim discipline"
    );
    assert!(
        text.contains("FR87")
            && (text.contains("历史") || text.contains("里程碑"))
            && (text.contains("不再") || text.contains("唯一") || text.contains("禁止")),
        "addendum must keep FR87 as historical milestone and not sole product-done definition"
    );
    assert!(
        (text.contains("FR94–105")
            || text.contains("FR94–FR105")
            || (text.contains("FR94") && text.contains("FR105")))
            && (text.contains("宣称") || text.contains("字面")),
        "addendum must restrict literal-green claims to FR94–105"
    );
    assert!(
        text.contains("禁止")
            && text.contains("FR87")
            && (text.contains("冒充") || text.contains("字面")),
        "addendum must forbid using FR87 to claim literal Path B done"
    );
}

#[test]
fn fr94_bitloom_brand_unchanged() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    // Phase 12 section (or surrounding contract) must keep Bitloom brand.
    let phase12_idx = addendum
        .find("Phase 12")
        .expect("addendum must mention Phase 12");
    let phase12_slice = &addendum[phase12_idx..];
    assert!(
        phase12_slice.contains("Bitloom")
            && (phase12_slice.contains("bitloom") || phase12_slice.contains("`bitloom")),
        "addendum Phase 12 section must affirm Bitloom / bitloom-* brand"
    );
    assert!(
        prd.contains("Bitloom")
            && (prd.contains("phase12-literal-green-path-b") || prd.contains("Phase 12 字面绿")),
        "prd.md must keep Bitloom brand and Phase 12 amendment stamp"
    );
    assert!(
        phase12_slice.contains("禁止")
            && (phase12_slice.contains("rhdl") || phase12_slice.contains("`rhdl")),
        "Phase 12 brand clause must forbid publishing rhdl / rhdl-bits"
    );
}

#[test]
fn fr94_prd_amendment_stamp() {
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    assert!(
        prd.contains("phase12-literal-green-path-b-2026-09-09")
            || prd.contains("phase12-literal-green-path-b"),
        "prd.md frontmatter/body must include phase12-literal-green-path-b amendment"
    );
    assert!(
        prd.contains("FR94") && prd.contains("推翻 FR93"),
        "prd.md must cite FR94 and FR93 overturn"
    );
}
