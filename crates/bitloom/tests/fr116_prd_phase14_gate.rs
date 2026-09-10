//! ATDD / guardrail: Story 57.2 / FR116 — Correct Course + PRD/addendum
//! Phase 14 NFR47 deferred-deepen gate.
//!
//! Red if the approved sprint-change proposal or Phase 14 addendum/prd
//! contract stamp is missing or incomplete. Does **not** require README/
//! deferred sync (57.3) or AD pointer closeout (57.4).
//!
//! ```text
//! cargo test -p bitloom --test fr116_prd_phase14_gate
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
fn fr116_correct_course_approved() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-10-phase14-nfr47-deferred-deepen.md",
    );
    assert!(
        text.contains("status: approved") || text.contains("status:approved"),
        "Correct Course proposal must be status: approved"
    );
    assert!(
        text.contains("Phase 14")
            && (text.contains("NFR47")
                || text.contains("未选加深")
                || text.contains("deferred-deepen")
                || text.contains("deferred deepen")),
        "Correct Course must authorize Phase 14 NFR47 deferred deepen"
    );
    assert!(
        text.contains("FR116") || text.contains("FR116–"),
        "Correct Course must cite FR116 gate / FR116–123"
    );
}

#[test]
fn fr116_addendum_phase14_deferred_deepen() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    assert!(
        text.contains("Phase 14")
            && (text.contains("NFR47") || text.contains("未选加深") || text.contains("加深升格")),
        "addendum must contain Phase 14 NFR47 deferred-deepen section"
    );
    assert!(
        (text.contains("FR116") && text.contains("FR123"))
            || text.contains("FR116–FR123")
            || text.contains("FR116–123"),
        "addendum Phase 14 must cite FR116–FR123"
    );
    assert!(
        (text.contains("NFR48") && text.contains("NFR51"))
            || text.contains("NFR48–NFR51")
            || text.contains("NFR48–51"),
        "addendum Phase 14 must cite NFR48–NFR51"
    );
}

#[test]
fn fr116_phase12_13_closes_remain_valid() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase14_idx = text
        .find("Phase 14")
        .expect("addendum must mention Phase 14");
    let phase14 = &text[phase14_idx..];
    assert!(
        (phase14.contains("FR94")
            || phase14.contains("FR106")
            || phase14.contains("Phase 12")
            || phase14.contains("Phase 13"))
            && (phase14.contains("仍有效")
                || phase14.contains("不得")
                || phase14.contains("不回滚")
                || phase14.contains("NFR48")),
        "Phase 14 section must keep Phase 12/13 closes valid (NFR48)"
    );
    assert!(
        phase14.contains("禁止")
            && (phase14.contains("冒充")
                || phase14.contains("Phase 13")
                || phase14.contains("加深")),
        "Phase 14 must forbid claiming deferred deepen via Phase 13 alone"
    );
}

#[test]
fn fr116_deepen_fr_mapping_and_epics_pointer() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    for fr in ["FR117", "FR118", "FR119", "FR120", "FR121", "FR122"] {
        assert!(
            addendum.contains(fr),
            "addendum Phase 14 must map deepen FR {fr}"
        );
    }
    assert!(
        addendum.contains("epics.md")
            && (addendum.contains("Epic 57") || addendum.contains("57–63")),
        "addendum must point at epics.md Epic 57–63"
    );
    assert!(
        epics.contains("FR116")
            && epics.contains("Epic 57")
            && epics.contains("Epic 63")
            && (epics.contains("correctCoursePhase14Approved")
                || epics.contains("phase14Contract")),
        "epics.md must carry Phase 14 inventory / approval stamp"
    );
}

#[test]
fn fr116_bitloom_brand_unchanged() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    let phase14_idx = addendum
        .find("Phase 14")
        .expect("addendum must mention Phase 14");
    let phase14_slice = &addendum[phase14_idx..];
    assert!(
        phase14_slice.contains("Bitloom")
            && (phase14_slice.contains("bitloom") || phase14_slice.contains("`bitloom")),
        "addendum Phase 14 section must affirm Bitloom / bitloom-* brand"
    );
    assert!(
        prd.contains("Bitloom")
            && (prd.contains("phase14-nfr47-deferred-deepen") || prd.contains("Phase 14")),
        "prd.md must keep Bitloom brand and Phase 14 amendment stamp"
    );
    assert!(
        phase14_slice.contains("禁止")
            && (phase14_slice.contains("rhdl") || phase14_slice.contains("`rhdl")),
        "Phase 14 brand clause must forbid publishing rhdl / rhdl-bits"
    );
}

#[test]
fn fr116_prd_amendment_stamp() {
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    assert!(
        prd.contains("phase14-nfr47-deferred-deepen-2026-09-10")
            || prd.contains("phase14-nfr47-deferred-deepen"),
        "prd.md frontmatter/body must include phase14-nfr47-deferred-deepen amendment"
    );
    assert!(
        prd.contains("FR116") && (prd.contains("NFR47") || prd.contains("Phase 14")),
        "prd.md must cite FR116 and Phase 14 NFR47 deepen"
    );
}

#[test]
fn fr116_claim_discipline_fr123() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase14_idx = text
        .find("Phase 14")
        .expect("addendum must mention Phase 14");
    let phase14 = &text[phase14_idx..];
    assert!(
        phase14.contains("FR123")
            || (phase14.contains("FR116") && phase14.contains("FR122") && phase14.contains("宣称")),
        "Phase 14 must cite FR123 claim discipline or FR116–122 claim gate"
    );
}
