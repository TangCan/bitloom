//! ATDD / guardrail: Story 118.2 / FR185 — Correct Course + PRD/addendum
//! Phase 23 NFR86 leftovers gate.
//!
//! ```text
//! cargo test -p bitloom --test fr185_prd_phase23_gate
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
fn fr185_correct_course_approved() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-14-phase23-nfr86-leftovers.md",
    );
    assert!(
        text.contains("status: approved") || text.contains("status:approved"),
        "Correct Course proposal must be status: approved"
    );
    assert!(
        text.contains("Phase 23")
            && (text.contains("NFR86") || text.contains("FR185") || text.contains("leftover")),
        "Correct Course must authorize Phase 23 NFR86 leftovers"
    );
    assert!(
        text.contains("FR185") || text.contains("FR185–"),
        "Correct Course must cite FR185 gate / FR185–191"
    );
}

#[test]
fn fr185_addendum_phase23() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    assert!(
        text.contains("Phase 23")
            && (text.contains("NFR86") || text.contains("FR185") || text.contains("leftover")),
        "addendum must contain Phase 23 section"
    );
    assert!(
        (text.contains("FR185") && text.contains("FR191"))
            || text.contains("FR185–FR191")
            || text.contains("FR185–191"),
        "addendum Phase 23 must cite FR185–FR191"
    );
    assert!(
        (text.contains("NFR88") && text.contains("NFR92"))
            || text.contains("NFR88–NFR92")
            || text.contains("NFR88–92"),
        "addendum Phase 23 must cite NFR88–NFR92"
    );
}

#[test]
fn fr185_phase12_22_closeout_remain_valid() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase23_idx = text
        .find("Phase 23")
        .expect("addendum must mention Phase 23");
    let phase23 = &text[phase23_idx..];
    assert!(
        (phase23.contains("FR94")
            || phase23.contains("FR184")
            || phase23.contains("Phase 22")
            || phase23.contains("结项")
            || phase23.contains("Phase 12"))
            && (phase23.contains("仍有效")
                || phase23.contains("不得")
                || phase23.contains("不回滚")
                || phase23.contains("NFR88")),
        "Phase 23 section must keep Phase 12–22 / closeout valid (NFR88)"
    );
}

#[test]
fn fr185_q_defaults_five_leftovers() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase23_idx = text
        .find("Phase 23")
        .expect("addendum must mention Phase 23");
    let phase23 = &text[phase23_idx..];
    assert!(
        (phase23.contains("Q1") || phase23.contains("批准默认"))
            && phase23.contains("FR186")
            && phase23.contains("FR190"),
        "Phase 23 must record Q1 five leftovers FR186–190"
    );
    assert!(
        phase23.contains("FR190")
            && phase23.contains("FR142")
            && (phase23.contains("独立") || phase23.contains("Q5") || phase23.contains("显式")),
        "Phase 23 must record Q5 FR142 = independent FR190"
    );
    assert!(
        phase23.contains("NFR90")
            && (phase23.contains("AD-9") || phase23.contains("AD-25") || phase23.contains("AD-27")),
        "Phase 23 must record Q7 NFR90 AD revise-before-ready"
    );
    assert!(
        phase23.contains("NFR88")
            && (phase23.contains("不得")
                || phase23.contains("不回滚")
                || phase23.contains("已关闭")),
        "Phase 23 must record Q3 NFR88 no rewrite of closed faces"
    );
    assert!(
        phase23.contains("FR191") || phase23.contains("NFR92"),
        "Phase 23 must record Q4 claim honesty via FR191 / NFR92"
    );
    assert!(
        phase23.contains("MSRV")
            && (phase23.contains("NFR91")
                || phase23.contains("Q6")
                || phase23.contains("批准默认")),
        "Phase 23 must record Q6 MSRV / NFR91 default"
    );
    assert!(
        (phase23.contains("git push") || phase23.contains("`git push`"))
            && (phase23.contains("不是") || phase23.contains("非") || phase23.contains("不在")),
        "Phase 23 must record Q8 git push not in contract"
    );
}

#[test]
fn fr185_fr_mapping_and_epics_pointer() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    let phase23_idx = addendum
        .find("Phase 23")
        .expect("addendum must mention Phase 23");
    let phase23 = &addendum[phase23_idx..];
    for fr in [
        "FR185", "FR186", "FR187", "FR188", "FR189", "FR190", "FR191",
    ] {
        assert!(phase23.contains(fr), "addendum Phase 23 must map FR {fr}");
    }
    assert!(
        addendum.contains("epics.md")
            && (addendum.contains("Epic 118") || addendum.contains("118–124")),
        "addendum must point at epics.md Epic 118–124"
    );
    assert!(
        epics.contains("FR185")
            && epics.contains("Epic 118")
            && epics.contains("Epic 124")
            && (epics.contains("correctCoursePhase23Approved")
                || epics.contains("phase23Contract")),
        "epics.md must carry Phase 23 inventory / approval stamp"
    );
}

#[test]
fn fr185_correct_course_phase23_approved_stamp() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("correctCoursePhase23Approved: 2026-09-14")
            || epics.contains("correctCoursePhase23Approved:2026-09-14"),
        "epics.md must carry verifiable correctCoursePhase23Approved stamp"
    );
}

#[test]
fn fr185_bitloom_brand_unchanged() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    let phase23_idx = addendum
        .find("Phase 23")
        .expect("addendum must mention Phase 23");
    let phase23_slice = &addendum[phase23_idx..];
    assert!(
        phase23_slice.contains("Bitloom")
            && (phase23_slice.contains("bitloom") || phase23_slice.contains("`bitloom")),
        "addendum Phase 23 section must affirm Bitloom / bitloom-* brand"
    );
    assert!(
        prd.contains("Bitloom")
            && (prd.contains("phase23-nfr86-leftovers") || prd.contains("Phase 23")),
        "prd.md must keep Bitloom brand and Phase 23 amendment stamp"
    );
    assert!(
        phase23_slice.contains("禁止")
            && (phase23_slice.contains("rhdl") || phase23_slice.contains("`rhdl")),
        "Phase 23 brand clause must forbid publishing rhdl / rhdl-bits"
    );
}

#[test]
fn fr185_prd_amendment_stamp() {
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    assert!(
        prd.contains("phase23-nfr86-leftovers-2026-09-14")
            || prd.contains("phase23-nfr86-leftovers"),
        "prd.md frontmatter/body must include phase23-nfr86-leftovers amendment"
    );
    assert!(
        prd.contains("FR185") && (prd.contains("NFR86") || prd.contains("Phase 23")),
        "prd.md must cite FR185 and Phase 23"
    );
}

#[test]
fn fr185_forbid_instant_deepen_on_approve() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase23_idx = text
        .find("Phase 23")
        .expect("addendum must mention Phase 23");
    let phase23 = &text[phase23_idx..];
    assert!(
        (phase23.contains("无界")
            || phase23.contains("tip")
            || phase23.contains("升钉")
            || phase23.contains("发版")
            || phase23.contains("实现加深"))
            && (phase23.contains("不")
                || phase23.contains("禁止")
                || phase23.contains("不得")
                || phase23.contains("Epic 119")),
        "Phase 23 must not force deepen delivery at approval instant"
    );
}

#[test]
fn fr185_nfr14_gate_present() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("118-1-epic-118-nfr14-风险记录: done")
            || sprint.contains("118-1-epic-118-nfr14-风险记录:done"),
        "Story 118.1 NFR14 must be done before 118.2 closes"
    );
    let nfr14 = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic118-phase23-nfr86-leftovers.md",
    );
    assert!(
        nfr14.contains("FR185") && nfr14.contains("NFR88"),
        "Epic 118 NFR14 risk record must exist with FR185/NFR88"
    );
}

#[test]
fn fr185_git_push_not_fr() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-14-phase23-nfr86-leftovers.md",
    );
    assert!(
        (text.contains("git push") || text.contains("`git push`"))
            && (text.contains("不是") || text.contains("非") || text.contains("不是 FR")),
        "Correct Course must state git push is not an FR"
    );
}
