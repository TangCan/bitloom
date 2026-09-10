//! ATDD / guardrail: Story 64.2 / FR124 — Correct Course + PRD/addendum
//! Phase 15 NFR51 leftover-deepen gate.
//!
//! ```text
//! cargo test -p bitloom --test fr124_prd_phase15_gate
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
fn fr124_correct_course_approved() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-10-phase15-nfr51-leftover-deepen.md",
    );
    assert!(
        text.contains("status: approved") || text.contains("status:approved"),
        "Correct Course proposal must be status: approved"
    );
    assert!(
        text.contains("Phase 15")
            && (text.contains("NFR51") || text.contains("剩余") || text.contains("leftover")),
        "Correct Course must authorize Phase 15 NFR51 leftover deepen"
    );
    assert!(
        text.contains("FR124") || text.contains("FR124–"),
        "Correct Course must cite FR124 gate / FR124–132"
    );
}

#[test]
fn fr124_addendum_phase15_leftover_deepen() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    assert!(
        text.contains("Phase 15")
            && (text.contains("NFR51") || text.contains("剩余升格") || text.contains("加深")),
        "addendum must contain Phase 15 NFR51 leftover-deepen section"
    );
    assert!(
        (text.contains("FR124") && text.contains("FR132"))
            || text.contains("FR124–FR132")
            || text.contains("FR124–132"),
        "addendum Phase 15 must cite FR124–FR132"
    );
    assert!(
        (text.contains("NFR52") && text.contains("NFR55"))
            || text.contains("NFR52–NFR55")
            || text.contains("NFR52–55"),
        "addendum Phase 15 must cite NFR52–NFR55"
    );
}

#[test]
fn fr124_phase12_14_closes_remain_valid() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase15_idx = text
        .find("Phase 15")
        .expect("addendum must mention Phase 15");
    let phase15 = &text[phase15_idx..];
    assert!(
        (phase15.contains("FR94")
            || phase15.contains("FR116")
            || phase15.contains("Phase 12")
            || phase15.contains("Phase 14"))
            && (phase15.contains("仍有效")
                || phase15.contains("不得")
                || phase15.contains("不回滚")
                || phase15.contains("NFR52")),
        "Phase 15 section must keep Phase 12–14 closes valid (NFR52)"
    );
    assert!(
        phase15.contains("禁止")
            && (phase15.contains("冒充")
                || phase15.contains("Phase 14")
                || phase15.contains("加深")),
        "Phase 15 must forbid claiming leftover deepen via Phase 14 alone"
    );
}

#[test]
fn fr124_deepen_fr_mapping_and_epics_pointer() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    for fr in [
        "FR125", "FR126", "FR127", "FR128", "FR129", "FR130", "FR131",
    ] {
        assert!(
            addendum.contains(fr),
            "addendum Phase 15 must map deepen FR {fr}"
        );
    }
    assert!(
        addendum.contains("epics.md")
            && (addendum.contains("Epic 64") || addendum.contains("64–71")),
        "addendum must point at epics.md Epic 64–71"
    );
    assert!(
        epics.contains("FR124")
            && epics.contains("Epic 64")
            && epics.contains("Epic 71")
            && (epics.contains("correctCoursePhase15Approved")
                || epics.contains("phase15Contract")),
        "epics.md must carry Phase 15 inventory / approval stamp"
    );
}

#[test]
fn fr124_bitloom_brand_unchanged() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    let phase15_idx = addendum
        .find("Phase 15")
        .expect("addendum must mention Phase 15");
    let phase15_slice = &addendum[phase15_idx..];
    assert!(
        phase15_slice.contains("Bitloom")
            && (phase15_slice.contains("bitloom") || phase15_slice.contains("`bitloom")),
        "addendum Phase 15 section must affirm Bitloom / bitloom-* brand"
    );
    assert!(
        prd.contains("Bitloom")
            && (prd.contains("phase15-nfr51-leftover-deepen") || prd.contains("Phase 15")),
        "prd.md must keep Bitloom brand and Phase 15 amendment stamp"
    );
    assert!(
        phase15_slice.contains("禁止")
            && (phase15_slice.contains("rhdl") || phase15_slice.contains("`rhdl")),
        "Phase 15 brand clause must forbid publishing rhdl / rhdl-bits"
    );
}

#[test]
fn fr124_prd_amendment_stamp() {
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    assert!(
        prd.contains("phase15-nfr51-leftover-deepen-2026-09-10")
            || prd.contains("phase15-nfr51-leftover-deepen"),
        "prd.md frontmatter/body must include phase15-nfr51-leftover-deepen amendment"
    );
    assert!(
        prd.contains("FR124") && (prd.contains("NFR51") || prd.contains("Phase 15")),
        "prd.md must cite FR124 and Phase 15 NFR51 deepen"
    );
}

#[test]
fn fr124_claim_discipline_fr132() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase15_idx = text
        .find("Phase 15")
        .expect("addendum must mention Phase 15");
    let phase15 = &text[phase15_idx..];
    assert!(
        phase15.contains("FR132")
            || (phase15.contains("FR124") && phase15.contains("FR131") && phase15.contains("宣称")),
        "Phase 15 must cite FR132 claim discipline or FR124–131 claim gate"
    );
}

#[test]
fn fr124_nfr14_gate_present() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("64-1-epic-64-nfr14-风险记录: done")
            || sprint.contains("64-1-epic-64-nfr14-风险记录:done"),
        "Story 64.1 NFR14 must be done before 64.2 closes"
    );
    let nfr14 = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic64-phase15-nfr51-leftover-deepen.md",
    );
    assert!(
        nfr14.contains("FR124") && nfr14.contains("NFR52"),
        "Epic 64 NFR14 risk record must exist with FR124/NFR52"
    );
}
