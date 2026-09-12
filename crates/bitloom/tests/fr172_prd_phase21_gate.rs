//! ATDD / guardrail: Story 105.2 / FR172 — Correct Course + PRD/addendum
//! Phase 21 NFR76 leftovers gate.
//!
//! ```text
//! cargo test -p bitloom --test fr172_prd_phase21_gate
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
fn fr172_correct_course_approved() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase21-nfr76-leftovers.md",
    );
    assert!(
        text.contains("status: approved") || text.contains("status:approved"),
        "Correct Course proposal must be status: approved"
    );
    assert!(
        text.contains("Phase 21")
            && (text.contains("NFR76") || text.contains("FR172") || text.contains("leftover")),
        "Correct Course must authorize Phase 21 NFR76 leftovers"
    );
    assert!(
        text.contains("FR172") || text.contains("FR172–"),
        "Correct Course must cite FR172 gate / FR172–177"
    );
}

#[test]
fn fr172_addendum_phase21() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    assert!(
        text.contains("Phase 21")
            && (text.contains("NFR76") || text.contains("FR172") || text.contains("leftover")),
        "addendum must contain Phase 21 section"
    );
    assert!(
        (text.contains("FR172") && text.contains("FR177"))
            || text.contains("FR172–FR177")
            || text.contains("FR172–177"),
        "addendum Phase 21 must cite FR172–FR177"
    );
    assert!(
        (text.contains("NFR78") && text.contains("NFR82"))
            || text.contains("NFR78–NFR82")
            || text.contains("NFR78–82"),
        "addendum Phase 21 must cite NFR78–NFR82"
    );
}

#[test]
fn fr172_phase12_20_closes_remain_valid() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase21_idx = text
        .find("Phase 21")
        .expect("addendum must mention Phase 21");
    let phase21 = &text[phase21_idx..];
    assert!(
        (phase21.contains("FR94")
            || phase21.contains("FR171")
            || phase21.contains("Phase 20")
            || phase21.contains("Phase 12"))
            && (phase21.contains("仍有效")
                || phase21.contains("不得")
                || phase21.contains("不回滚")
                || phase21.contains("NFR78")),
        "Phase 21 section must keep Phase 12–20 closes valid (NFR78)"
    );
}

#[test]
fn fr172_q_defaults_four_leftovers() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase21_idx = text
        .find("Phase 21")
        .expect("addendum must mention Phase 21");
    let phase21 = &text[phase21_idx..];
    assert!(
        (phase21.contains("Q1") || phase21.contains("批准默认"))
            && phase21.contains("FR173")
            && phase21.contains("FR176"),
        "Phase 21 must record Q1 four leftovers FR173–176"
    );
    assert!(
        phase21.contains("FR173")
            && (phase21.contains("AD-9")
                || phase21.contains("firtool")
                || phase21.contains("配对")),
        "Phase 21 must record Q2 FR173 firtool + AD-9 pairing"
    );
    assert!(
        phase21.contains("NFR78")
            && (phase21.contains("不得")
                || phase21.contains("不回滚")
                || phase21.contains("已关闭")),
        "Phase 21 must record Q3 NFR78 no rewrite of closed faces"
    );
    assert!(
        phase21.contains("FR142") && (phase21.contains("FR177") || phase21.contains("NFR82")),
        "Phase 21 must record Q4 claim / FR142 discipline"
    );
    assert!(
        phase21.contains("MSRV")
            && (phase21.contains("NFR81")
                || phase21.contains("Q5")
                || phase21.contains("批准默认")),
        "Phase 21 must record Q5 MSRV / NFR81 default"
    );
}

#[test]
fn fr172_fr_mapping_and_epics_pointer() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    let phase21_idx = addendum
        .find("Phase 21")
        .expect("addendum must mention Phase 21");
    let phase21 = &addendum[phase21_idx..];
    for fr in ["FR172", "FR173", "FR174", "FR175", "FR176", "FR177"] {
        assert!(phase21.contains(fr), "addendum Phase 21 must map FR {fr}");
    }
    assert!(
        addendum.contains("epics.md")
            && (addendum.contains("Epic 105") || addendum.contains("105–110")),
        "addendum must point at epics.md Epic 105–110"
    );
    assert!(
        epics.contains("FR172")
            && epics.contains("Epic 105")
            && epics.contains("Epic 110")
            && (epics.contains("correctCoursePhase21Approved")
                || epics.contains("phase21Contract")),
        "epics.md must carry Phase 21 inventory / approval stamp"
    );
}

#[test]
fn fr172_correct_course_phase21_approved_stamp() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("correctCoursePhase21Approved: 2026-09-12")
            || epics.contains("correctCoursePhase21Approved:2026-09-12"),
        "epics.md must carry verifiable correctCoursePhase21Approved stamp"
    );
}

#[test]
fn fr172_bitloom_brand_unchanged() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    let phase21_idx = addendum
        .find("Phase 21")
        .expect("addendum must mention Phase 21");
    let phase21_slice = &addendum[phase21_idx..];
    assert!(
        phase21_slice.contains("Bitloom")
            && (phase21_slice.contains("bitloom") || phase21_slice.contains("`bitloom")),
        "addendum Phase 21 section must affirm Bitloom / bitloom-* brand"
    );
    assert!(
        prd.contains("Bitloom")
            && (prd.contains("phase21-nfr76-leftovers") || prd.contains("Phase 21")),
        "prd.md must keep Bitloom brand and Phase 21 amendment stamp"
    );
    assert!(
        phase21_slice.contains("禁止")
            && (phase21_slice.contains("rhdl") || phase21_slice.contains("`rhdl")),
        "Phase 21 brand clause must forbid publishing rhdl / rhdl-bits"
    );
}

#[test]
fn fr172_prd_amendment_stamp() {
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    assert!(
        prd.contains("phase21-nfr76-leftovers-2026-09-12")
            || prd.contains("phase21-nfr76-leftovers"),
        "prd.md frontmatter/body must include phase21-nfr76-leftovers amendment"
    );
    assert!(
        prd.contains("FR172") && (prd.contains("NFR76") || prd.contains("Phase 21")),
        "prd.md must cite FR172 and Phase 21"
    );
}

#[test]
fn fr172_forbid_instant_firtool_or_head_on_approve() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase21_idx = text
        .find("Phase 21")
        .expect("addendum must mention Phase 21");
    let phase21 = &text[phase21_idx..];
    assert!(
        (phase21.contains("firtool") || phase21.contains("HEAD") || phase21.contains("升钉"))
            && (phase21.contains("不") || phase21.contains("禁止") || phase21.contains("瞬间")),
        "Phase 21 must not force firtool bump / HEAD binaries at contract approval instant"
    );
}

#[test]
fn fr172_nfr14_gate_present() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("105-1-epic-105-nfr14-风险记录: done")
            || sprint.contains("105-1-epic-105-nfr14-风险记录:done"),
        "Story 105.1 NFR14 must be done before 105.2 closes"
    );
    let nfr14 = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic105-phase21-nfr76-leftovers.md",
    );
    assert!(
        nfr14.contains("FR172") && nfr14.contains("NFR78"),
        "Epic 105 NFR14 risk record must exist with FR172/NFR78"
    );
}

#[test]
fn fr172_git_push_not_fr() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase21-nfr76-leftovers.md",
    );
    assert!(
        (text.contains("git push") || text.contains("`git push`"))
            && (text.contains("不是") || text.contains("非") || text.contains("不是 FR")),
        "Correct Course must state git push is not an FR"
    );
}
