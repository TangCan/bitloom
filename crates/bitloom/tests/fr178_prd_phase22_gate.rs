//! ATDD / guardrail: Story 111.2 / FR178 — Correct Course + PRD/addendum
//! Phase 22 NFR81 leftovers gate.
//!
//! ```text
//! cargo test -p bitloom --test fr178_prd_phase22_gate
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
fn fr178_correct_course_approved() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase22-nfr81-leftovers.md",
    );
    assert!(
        text.contains("status: approved") || text.contains("status:approved"),
        "Correct Course proposal must be status: approved"
    );
    assert!(
        text.contains("Phase 22")
            && (text.contains("NFR81") || text.contains("FR178") || text.contains("leftover")),
        "Correct Course must authorize Phase 22 NFR81 leftovers"
    );
    assert!(
        text.contains("FR178") || text.contains("FR178–"),
        "Correct Course must cite FR178 gate / FR178–184"
    );
}

#[test]
fn fr178_addendum_phase22() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    assert!(
        text.contains("Phase 22")
            && (text.contains("NFR81") || text.contains("FR178") || text.contains("leftover")),
        "addendum must contain Phase 22 section"
    );
    assert!(
        (text.contains("FR178") && text.contains("FR184"))
            || text.contains("FR178–FR184")
            || text.contains("FR178–184"),
        "addendum Phase 22 must cite FR178–FR184"
    );
    assert!(
        (text.contains("NFR83") && text.contains("NFR87"))
            || text.contains("NFR83–NFR87")
            || text.contains("NFR83–87"),
        "addendum Phase 22 must cite NFR83–NFR87"
    );
}

#[test]
fn fr178_phase12_21_closes_remain_valid() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase22_idx = text
        .find("Phase 22")
        .expect("addendum must mention Phase 22");
    let phase22 = &text[phase22_idx..];
    assert!(
        (phase22.contains("FR94")
            || phase22.contains("FR177")
            || phase22.contains("Phase 21")
            || phase22.contains("Phase 12"))
            && (phase22.contains("仍有效")
                || phase22.contains("不得")
                || phase22.contains("不回滚")
                || phase22.contains("NFR83")),
        "Phase 22 section must keep Phase 12–21 closes valid (NFR83)"
    );
}

#[test]
fn fr178_q_defaults_five_leftovers() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase22_idx = text
        .find("Phase 22")
        .expect("addendum must mention Phase 22");
    let phase22 = &text[phase22_idx..];
    assert!(
        (phase22.contains("Q1") || phase22.contains("批准默认"))
            && phase22.contains("FR179")
            && phase22.contains("FR183"),
        "Phase 22 must record Q1 five leftovers FR179–183"
    );
    assert!(
        phase22.contains("FR183")
            && phase22.contains("FR142")
            && (phase22.contains("独立") || phase22.contains("Q5") || phase22.contains("显式")),
        "Phase 22 must record Q5 FR142 = independent FR183"
    );
    assert!(
        phase22.contains("FR182")
            && (phase22.contains("AD-9")
                || phase22.contains("unpaired")
                || phase22.contains("产品钉")),
        "Phase 22 must record Q7 FR182 unpaired product-pin + AD-9"
    );
    assert!(
        phase22.contains("NFR83")
            && (phase22.contains("不得")
                || phase22.contains("不回滚")
                || phase22.contains("已关闭")),
        "Phase 22 must record Q3 NFR83 no rewrite of closed faces"
    );
    assert!(
        phase22.contains("FR184") || phase22.contains("NFR87"),
        "Phase 22 must record Q4 claim honesty via FR184 / NFR87"
    );
    assert!(
        phase22.contains("MSRV")
            && (phase22.contains("NFR86")
                || phase22.contains("Q6")
                || phase22.contains("批准默认")),
        "Phase 22 must record Q6 MSRV / NFR86 default"
    );
}

#[test]
fn fr178_fr_mapping_and_epics_pointer() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    let phase22_idx = addendum
        .find("Phase 22")
        .expect("addendum must mention Phase 22");
    let phase22 = &addendum[phase22_idx..];
    for fr in [
        "FR178", "FR179", "FR180", "FR181", "FR182", "FR183", "FR184",
    ] {
        assert!(phase22.contains(fr), "addendum Phase 22 must map FR {fr}");
    }
    assert!(
        addendum.contains("epics.md")
            && (addendum.contains("Epic 111") || addendum.contains("111–117")),
        "addendum must point at epics.md Epic 111–117"
    );
    assert!(
        epics.contains("FR178")
            && epics.contains("Epic 111")
            && epics.contains("Epic 117")
            && (epics.contains("correctCoursePhase22Approved")
                || epics.contains("phase22Contract")),
        "epics.md must carry Phase 22 inventory / approval stamp"
    );
}

#[test]
fn fr178_correct_course_phase22_approved_stamp() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("correctCoursePhase22Approved: 2026-09-12")
            || epics.contains("correctCoursePhase22Approved:2026-09-12"),
        "epics.md must carry verifiable correctCoursePhase22Approved stamp"
    );
}

#[test]
fn fr178_bitloom_brand_unchanged() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    let phase22_idx = addendum
        .find("Phase 22")
        .expect("addendum must mention Phase 22");
    let phase22_slice = &addendum[phase22_idx..];
    assert!(
        phase22_slice.contains("Bitloom")
            && (phase22_slice.contains("bitloom") || phase22_slice.contains("`bitloom")),
        "addendum Phase 22 section must affirm Bitloom / bitloom-* brand"
    );
    assert!(
        prd.contains("Bitloom")
            && (prd.contains("phase22-nfr81-leftovers") || prd.contains("Phase 22")),
        "prd.md must keep Bitloom brand and Phase 22 amendment stamp"
    );
    assert!(
        phase22_slice.contains("禁止")
            && (phase22_slice.contains("rhdl") || phase22_slice.contains("`rhdl")),
        "Phase 22 brand clause must forbid publishing rhdl / rhdl-bits"
    );
}

#[test]
fn fr178_prd_amendment_stamp() {
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    assert!(
        prd.contains("phase22-nfr81-leftovers-2026-09-12")
            || prd.contains("phase22-nfr81-leftovers"),
        "prd.md frontmatter/body must include phase22-nfr81-leftovers amendment"
    );
    assert!(
        prd.contains("FR178") && (prd.contains("NFR81") || prd.contains("Phase 22")),
        "prd.md must cite FR178 and Phase 22"
    );
}

#[test]
fn fr178_forbid_instant_head_or_unpaired_pin_on_approve() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase22_idx = text
        .find("Phase 22")
        .expect("addendum must mention Phase 22");
    let phase22 = &text[phase22_idx..];
    assert!(
        (phase22.contains("HEAD")
            || phase22.contains("浮动")
            || phase22.contains("unpaired")
            || phase22.contains("升钉")
            || phase22.contains("发版"))
            && (phase22.contains("不") || phase22.contains("禁止") || phase22.contains("瞬间")),
        "Phase 22 must not force floating HEAD / unpaired pin / crates.io break at approval instant"
    );
}

#[test]
fn fr178_nfr14_gate_present() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("111-1-epic-111-nfr14-风险记录: done")
            || sprint.contains("111-1-epic-111-nfr14-风险记录:done"),
        "Story 111.1 NFR14 must be done before 111.2 closes"
    );
    let nfr14 = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic111-phase22-nfr81-leftovers.md",
    );
    assert!(
        nfr14.contains("FR178") && nfr14.contains("NFR83"),
        "Epic 111 NFR14 risk record must exist with FR178/NFR83"
    );
}

#[test]
fn fr178_git_push_not_fr() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase22-nfr81-leftovers.md",
    );
    assert!(
        (text.contains("git push") || text.contains("`git push`"))
            && (text.contains("不是") || text.contains("非") || text.contains("不是 FR")),
        "Correct Course must state git push is not an FR"
    );
}
