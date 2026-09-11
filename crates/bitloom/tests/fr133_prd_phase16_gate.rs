//! ATDD / guardrail: Story 72.2 / FR133 — Correct Course + PRD/addendum
//! Phase 16 NFR55 final-closeout / 产品终局结项 gate.
//!
//! ```text
//! cargo test -p bitloom --test fr133_prd_phase16_gate
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
fn fr133_correct_course_approved() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-11-phase16-nfr55-final-closeout.md",
    );
    assert!(
        text.contains("status: approved") || text.contains("status:approved"),
        "Correct Course proposal must be status: approved"
    );
    assert!(
        text.contains("Phase 16")
            && (text.contains("NFR55")
                || text.contains("终局")
                || text.contains("final-closeout")
                || text.contains("final closeout")),
        "Correct Course must authorize Phase 16 NFR55 final closeout"
    );
    assert!(
        text.contains("FR133") || text.contains("FR133–"),
        "Correct Course must cite FR133 gate / FR133–140"
    );
}

#[test]
fn fr133_addendum_phase16_final_closeout() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    assert!(
        text.contains("Phase 16")
            && (text.contains("NFR55")
                || text.contains("终局结项")
                || text.contains("产品终局")
                || text.contains("final")),
        "addendum must contain Phase 16 NFR55 final-closeout section"
    );
    assert!(
        (text.contains("FR133") && text.contains("FR140"))
            || text.contains("FR133–FR140")
            || text.contains("FR133–140"),
        "addendum Phase 16 must cite FR133–FR140"
    );
    assert!(
        (text.contains("NFR56") && text.contains("NFR59"))
            || text.contains("NFR56–NFR59")
            || text.contains("NFR56–59"),
        "addendum Phase 16 must cite NFR56–NFR59"
    );
}

#[test]
fn fr133_phase12_15_closes_remain_valid() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase16_idx = text
        .find("Phase 16")
        .expect("addendum must mention Phase 16");
    let phase16 = &text[phase16_idx..];
    assert!(
        (phase16.contains("FR94")
            || phase16.contains("FR124")
            || phase16.contains("Phase 12")
            || phase16.contains("Phase 15"))
            && (phase16.contains("仍有效")
                || phase16.contains("不得")
                || phase16.contains("不回滚")
                || phase16.contains("NFR56")),
        "Phase 16 section must keep Phase 12–15 closes valid (NFR56)"
    );
    assert!(
        phase16.contains("禁止")
            && (phase16.contains("冒充")
                || phase16.contains("Phase 15")
                || phase16.contains("加深")),
        "Phase 16 must forbid claiming final deepen via Phase 15 alone"
    );
}

#[test]
fn fr133_finality_not_v1_or_empty_backlog() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase16_idx = text
        .find("Phase 16")
        .expect("addendum must mention Phase 16");
    let phase16 = &text[phase16_idx..];
    assert!(
        (phase16.contains("不等于") || phase16.contains("≠") || phase16.contains("不是"))
            && (phase16.contains("1.0")
                || phase16.contains("冲")
                || phase16.contains("backlog")
                || phase16.contains("永久空")),
        "Phase 16 must state finality ≠ ship 1.0 / ≠ permanently empty backlog"
    );
}

#[test]
fn fr133_deepen_fr_mapping_and_epics_pointer() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    for fr in ["FR134", "FR135", "FR136", "FR137", "FR138", "FR139"] {
        assert!(
            addendum.contains(fr),
            "addendum Phase 16 must map deepen FR {fr}"
        );
    }
    assert!(
        addendum.contains("epics.md")
            && (addendum.contains("Epic 72") || addendum.contains("72–78")),
        "addendum must point at epics.md Epic 72–78"
    );
    assert!(
        epics.contains("FR133")
            && epics.contains("Epic 72")
            && epics.contains("Epic 78")
            && (epics.contains("correctCoursePhase16Approved")
                || epics.contains("phase16Contract")),
        "epics.md must carry Phase 16 inventory / approval stamp"
    );
}

#[test]
fn fr133_correct_course_phase16_approved_stamp() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("correctCoursePhase16Approved: 2026-09-11")
            || epics.contains("correctCoursePhase16Approved:2026-09-11"),
        "epics.md must carry verifiable correctCoursePhase16Approved stamp"
    );
}

#[test]
fn fr133_bitloom_brand_unchanged() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    let phase16_idx = addendum
        .find("Phase 16")
        .expect("addendum must mention Phase 16");
    let phase16_slice = &addendum[phase16_idx..];
    assert!(
        phase16_slice.contains("Bitloom")
            && (phase16_slice.contains("bitloom") || phase16_slice.contains("`bitloom")),
        "addendum Phase 16 section must affirm Bitloom / bitloom-* brand"
    );
    assert!(
        prd.contains("Bitloom")
            && (prd.contains("phase16-nfr55-final-closeout") || prd.contains("Phase 16")),
        "prd.md must keep Bitloom brand and Phase 16 amendment stamp"
    );
    assert!(
        phase16_slice.contains("禁止")
            && (phase16_slice.contains("rhdl") || phase16_slice.contains("`rhdl")),
        "Phase 16 brand clause must forbid publishing rhdl / rhdl-bits"
    );
}

#[test]
fn fr133_prd_amendment_stamp() {
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    assert!(
        prd.contains("phase16-nfr55-final-closeout-2026-09-11")
            || prd.contains("phase16-nfr55-final-closeout"),
        "prd.md frontmatter/body must include phase16-nfr55-final-closeout amendment"
    );
    assert!(
        prd.contains("FR133") && (prd.contains("NFR55") || prd.contains("Phase 16")),
        "prd.md must cite FR133 and Phase 16 NFR55 closeout"
    );
}

#[test]
fn fr133_claim_discipline_fr140() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase16_idx = text
        .find("Phase 16")
        .expect("addendum must mention Phase 16");
    let phase16 = &text[phase16_idx..];
    assert!(
        phase16.contains("FR140")
            || (phase16.contains("FR133") && phase16.contains("FR139") && phase16.contains("宣称")),
        "Phase 16 must cite FR140 claim discipline or FR133–139 claim gate"
    );
}

#[test]
fn fr133_nfr14_gate_present() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("72-1-epic-72-nfr14-风险记录: done")
            || sprint.contains("72-1-epic-72-nfr14-风险记录:done"),
        "Story 72.1 NFR14 must be done before 72.2 closes"
    );
    let nfr14 = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic72-phase16-nfr55-final-closeout.md",
    );
    assert!(
        nfr14.contains("FR133") && nfr14.contains("NFR56"),
        "Epic 72 NFR14 risk record must exist with FR133/NFR56"
    );
}
