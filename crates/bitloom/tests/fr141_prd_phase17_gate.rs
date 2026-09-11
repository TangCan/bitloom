//! ATDD / guardrail: Story 79.2 / FR141 — Correct Course + PRD/addendum
//! Phase 17 API stability gate / Bitloom 1.0.
//!
//! ```text
//! cargo test -p bitloom --test fr141_prd_phase17_gate
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
fn fr141_correct_course_approved() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-11-phase17-api-stability-1-0.md",
    );
    assert!(
        text.contains("status: approved") || text.contains("status:approved"),
        "Correct Course proposal must be status: approved"
    );
    assert!(
        text.contains("Phase 17")
            && (text.contains("API")
                || text.contains("稳定门")
                || text.contains("1.0")
                || text.contains("api-stability")),
        "Correct Course must authorize Phase 17 API stability / 1.0 gate"
    );
    assert!(
        text.contains("FR141") || text.contains("FR141–"),
        "Correct Course must cite FR141 gate / FR141–147"
    );
}

#[test]
fn fr141_addendum_phase17_api_stability() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    assert!(
        text.contains("Phase 17")
            && (text.contains("公开 API")
                || text.contains("稳定门")
                || text.contains("1.0")
                || text.contains("api-stability")),
        "addendum must contain Phase 17 API stability / 1.0 section"
    );
    assert!(
        (text.contains("FR141") && text.contains("FR147"))
            || text.contains("FR141–FR147")
            || text.contains("FR141–147"),
        "addendum Phase 17 must cite FR141–FR147"
    );
    assert!(
        (text.contains("NFR60") && text.contains("NFR63"))
            || text.contains("NFR60–NFR63")
            || text.contains("NFR60–63"),
        "addendum Phase 17 must cite NFR60–NFR63"
    );
}

#[test]
fn fr141_phase12_16_closes_remain_valid() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase17_idx = text
        .find("Phase 17")
        .expect("addendum must mention Phase 17");
    let phase17 = &text[phase17_idx..];
    assert!(
        (phase17.contains("FR94")
            || phase17.contains("FR133")
            || phase17.contains("Phase 12")
            || phase17.contains("Phase 16"))
            && (phase17.contains("仍有效")
                || phase17.contains("不得")
                || phase17.contains("不回滚")
                || phase17.contains("NFR60")),
        "Phase 17 section must keep Phase 12–16 closes valid (NFR60)"
    );
    assert!(
        phase17.contains("禁止")
            && (phase17.contains("冒充")
                || phase17.contains("Phase 16")
                || phase17.contains("alone")),
        "Phase 17 must forbid claiming 1.0 via Phase 16 alone"
    );
}

#[test]
fn fr141_one_oh_not_clear_nfr59() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase17_idx = text
        .find("Phase 17")
        .expect("addendum must mention Phase 17");
    let phase17 = &text[phase17_idx..];
    assert!(
        (phase17.contains("不等于") || phase17.contains("≠") || phase17.contains("不是"))
            && phase17.contains("NFR59"),
        "Phase 17 must state 1.0 ≠ clear NFR59 (NFR63)"
    );
}

#[test]
fn fr141_q1_q5_defaults() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase17_idx = text
        .find("Phase 17")
        .expect("addendum must mention Phase 17");
    let phase17 = &text[phase17_idx..];
    assert!(
        phase17.contains("bitloom-sim") && (phase17.contains("Q1") || phase17.contains("批准默认")),
        "Phase 17 must record Q1 bitloom-sim surface default"
    );
    assert!(
        phase17.contains("bitloom-hir")
            || phase17.contains("bitloom-builder")
            || phase17.contains("bitloom-vlog"),
        "Phase 17 must record Q2 hir/builder/vlog publish-not-in-1.0-promise"
    );
    assert!(
        phase17.contains("Epic 82") || phase17.contains("FR145") || phase17.contains("skip"),
        "Phase 17 must record Q3 Epic 82 skip-if-no-blockers"
    );
    assert!(
        phase17.contains("NFR59")
            && (phase17.contains("不以") || phase17.contains("前提") || phase17.contains("Q4")),
        "Phase 17 must record Q4 no NFR59 prerequisite"
    );
    assert!(
        phase17.contains("MSRV") || phase17.contains("Q5"),
        "Phase 17 must record Q5 MSRV keep default"
    );
}

#[test]
fn fr141_fr_mapping_and_epics_pointer() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    for fr in ["FR142", "FR143", "FR144", "FR145", "FR146"] {
        assert!(
            addendum.contains(fr),
            "addendum Phase 17 must map deepen FR {fr}"
        );
    }
    assert!(
        addendum.contains("epics.md")
            && (addendum.contains("Epic 79") || addendum.contains("79–83")),
        "addendum must point at epics.md Epic 79–83"
    );
    assert!(
        epics.contains("FR141")
            && epics.contains("Epic 79")
            && epics.contains("Epic 83")
            && (epics.contains("correctCoursePhase17Approved")
                || epics.contains("phase17Contract")),
        "epics.md must carry Phase 17 inventory / approval stamp"
    );
}

#[test]
fn fr141_correct_course_phase17_approved_stamp() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("correctCoursePhase17Approved: 2026-09-11")
            || epics.contains("correctCoursePhase17Approved:2026-09-11"),
        "epics.md must carry verifiable correctCoursePhase17Approved stamp"
    );
}

#[test]
fn fr141_bitloom_brand_unchanged() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    let phase17_idx = addendum
        .find("Phase 17")
        .expect("addendum must mention Phase 17");
    let phase17_slice = &addendum[phase17_idx..];
    assert!(
        phase17_slice.contains("Bitloom")
            && (phase17_slice.contains("bitloom") || phase17_slice.contains("`bitloom")),
        "addendum Phase 17 section must affirm Bitloom / bitloom-* brand"
    );
    assert!(
        prd.contains("Bitloom")
            && (prd.contains("phase17-api-stability-1-0") || prd.contains("Phase 17")),
        "prd.md must keep Bitloom brand and Phase 17 amendment stamp"
    );
    assert!(
        phase17_slice.contains("禁止")
            && (phase17_slice.contains("rhdl") || phase17_slice.contains("`rhdl")),
        "Phase 17 brand clause must forbid publishing rhdl / rhdl-bits"
    );
}

#[test]
fn fr141_prd_amendment_stamp() {
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    assert!(
        prd.contains("phase17-api-stability-1-0-2026-09-11")
            || prd.contains("phase17-api-stability-1-0"),
        "prd.md frontmatter/body must include phase17-api-stability-1-0 amendment"
    );
    assert!(
        prd.contains("FR141") && (prd.contains("1.0") || prd.contains("Phase 17")),
        "prd.md must cite FR141 and Phase 17 API stability / 1.0"
    );
}

#[test]
fn fr141_claim_discipline_fr147() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase17_idx = text
        .find("Phase 17")
        .expect("addendum must mention Phase 17");
    let phase17 = &text[phase17_idx..];
    assert!(
        phase17.contains("FR147")
            || (phase17.contains("FR141") && phase17.contains("FR146") && phase17.contains("宣称")),
        "Phase 17 must cite FR147 claim discipline or FR141–146 claim gate"
    );
}

#[test]
fn fr141_nfr14_gate_present() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("79-1-epic-79-nfr14-风险记录: done")
            || sprint.contains("79-1-epic-79-nfr14-风险记录:done"),
        "Story 79.1 NFR14 must be done before 79.2 closes"
    );
    let nfr14 = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic79-phase17-api-stability-1-0.md",
    );
    assert!(
        nfr14.contains("FR141") && nfr14.contains("NFR60"),
        "Epic 79 NFR14 risk record must exist with FR141/NFR60"
    );
}
