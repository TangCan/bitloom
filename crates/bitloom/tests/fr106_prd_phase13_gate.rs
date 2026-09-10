//! ATDD / guardrail: Story 48.2 / FR106 — Correct Course + PRD/addendum
//! Phase 13 MVP→commercial deepen gate.
//!
//! Red if the approved sprint-change proposal or Phase 13 addendum/prd
//! contract stamp is missing or incomplete. Does **not** require README/
//! deferred sync (48.3) or AD pointer closeout (48.4).
//!
//! ```text
//! cargo test -p bitloom --test fr106_prd_phase13_gate
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
fn fr106_correct_course_approved() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-10-phase13-mvp-commercial-deepen.md",
    );
    assert!(
        text.contains("status: approved") || text.contains("status:approved"),
        "Correct Course proposal must be status: approved"
    );
    assert!(
        text.contains("Phase 13")
            && (text.contains("商业加深")
                || text.contains("mvp-to-commercial")
                || text.contains("MVP→")),
        "Correct Course must authorize Phase 13 commercial deepen"
    );
    assert!(
        text.contains("FR106") || text.contains("FR106–"),
        "Correct Course must cite FR106 gate / FR106–115"
    );
}

#[test]
fn fr106_addendum_phase13_commercial_deepen() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    assert!(
        text.contains("Phase 13")
            && (text.contains("商业加深") || text.contains("MVP→") || text.contains("加深")),
        "addendum must contain Phase 13 commercial-deepen section"
    );
    assert!(
        (text.contains("FR106") && text.contains("FR115"))
            || text.contains("FR106–FR115")
            || text.contains("FR106–115"),
        "addendum Phase 13 must cite FR106–FR115"
    );
    assert!(
        (text.contains("NFR44") && text.contains("NFR47"))
            || text.contains("NFR44–NFR47")
            || text.contains("NFR44–47"),
        "addendum Phase 13 must cite NFR44–NFR47"
    );
}

#[test]
fn fr106_phase12_mvp_remains_valid() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase13_idx = text
        .find("Phase 13")
        .expect("addendum must mention Phase 13");
    let phase13 = &text[phase13_idx..];
    assert!(
        (phase13.contains("FR94") || phase13.contains("Phase 12"))
            && (phase13.contains("仍有效")
                || phase13.contains("不得")
                || phase13.contains("不回滚")
                || phase13.contains("NFR44")),
        "Phase 13 section must keep Phase 12 MVP closes valid (NFR44)"
    );
    assert!(
        phase13.contains("禁止")
            && (phase13.contains("冒充") || phase13.contains("商业") || phase13.contains("MVP")),
        "Phase 13 must forbid claiming commercial deepen via Phase 12 MVP alone"
    );
}

#[test]
fn fr106_deepen_fr_mapping_and_epics_pointer() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    for fr in [
        "FR107", "FR108", "FR109", "FR110", "FR111", "FR112", "FR113", "FR114",
    ] {
        assert!(
            addendum.contains(fr),
            "addendum Phase 13 must map deepen FR {fr}"
        );
    }
    assert!(
        addendum.contains("epics.md")
            && (addendum.contains("Epic 48") || addendum.contains("48–56")),
        "addendum must point at epics.md Epic 48–56"
    );
    assert!(
        epics.contains("FR106")
            && epics.contains("Epic 48")
            && epics.contains("Epic 56")
            && (epics.contains("correctCoursePhase13Approved")
                || epics.contains("phase13Contract")),
        "epics.md must carry Phase 13 inventory / approval stamp"
    );
}

#[test]
fn fr106_bitloom_brand_unchanged() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    let phase13_idx = addendum
        .find("Phase 13")
        .expect("addendum must mention Phase 13");
    let phase13_slice = &addendum[phase13_idx..];
    assert!(
        phase13_slice.contains("Bitloom")
            && (phase13_slice.contains("bitloom") || phase13_slice.contains("`bitloom")),
        "addendum Phase 13 section must affirm Bitloom / bitloom-* brand"
    );
    assert!(
        prd.contains("Bitloom")
            && (prd.contains("phase13-mvp-commercial-deepen") || prd.contains("Phase 13")),
        "prd.md must keep Bitloom brand and Phase 13 amendment stamp"
    );
    assert!(
        phase13_slice.contains("禁止")
            && (phase13_slice.contains("rhdl") || phase13_slice.contains("`rhdl")),
        "Phase 13 brand clause must forbid publishing rhdl / rhdl-bits"
    );
}

#[test]
fn fr106_prd_amendment_stamp() {
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    assert!(
        prd.contains("phase13-mvp-commercial-deepen-2026-09-10")
            || prd.contains("phase13-mvp-commercial-deepen"),
        "prd.md frontmatter/body must include phase13-mvp-commercial-deepen amendment"
    );
    assert!(
        prd.contains("FR106") && (prd.contains("商业加深") || prd.contains("Phase 13")),
        "prd.md must cite FR106 and Phase 13 commercial deepen"
    );
}

#[test]
fn fr106_claim_discipline_fr115() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase13_idx = text
        .find("Phase 13")
        .expect("addendum must mention Phase 13");
    let phase13 = &text[phase13_idx..];
    assert!(
        phase13.contains("FR115")
            || (phase13.contains("FR106") && phase13.contains("FR114") && phase13.contains("宣称")),
        "Phase 13 must cite FR115 claim discipline or FR106–114 claim gate"
    );
}
