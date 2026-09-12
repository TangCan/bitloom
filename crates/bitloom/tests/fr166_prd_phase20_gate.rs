//! ATDD / guardrail: Story 99.2 / FR166 — Correct Course + PRD/addendum
//! Phase 20 NFR71 four leftovers gate.
//!
//! ```text
//! cargo test -p bitloom --test fr166_prd_phase20_gate
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
fn fr166_correct_course_approved() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase20-nfr71-four-leftovers.md",
    );
    assert!(
        text.contains("status: approved") || text.contains("status:approved"),
        "Correct Course proposal must be status: approved"
    );
    assert!(
        text.contains("Phase 20")
            && (text.contains("NFR71") || text.contains("FR166") || text.contains("four")),
        "Correct Course must authorize Phase 20 NFR71 leftovers"
    );
    assert!(
        text.contains("FR166") || text.contains("FR166–"),
        "Correct Course must cite FR166 gate / FR166–171"
    );
}

#[test]
fn fr166_addendum_phase20() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    assert!(
        text.contains("Phase 20")
            && (text.contains("NFR71") || text.contains("FR166") || text.contains("四条")),
        "addendum must contain Phase 20 section"
    );
    assert!(
        (text.contains("FR166") && text.contains("FR171"))
            || text.contains("FR166–FR171")
            || text.contains("FR166–171"),
        "addendum Phase 20 must cite FR166–FR171"
    );
    assert!(
        (text.contains("NFR73") && text.contains("NFR77"))
            || text.contains("NFR73–NFR77")
            || text.contains("NFR73–77"),
        "addendum Phase 20 must cite NFR73–NFR77"
    );
}

#[test]
fn fr166_phase12_19_closes_remain_valid() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase20_idx = text
        .find("Phase 20")
        .expect("addendum must mention Phase 20");
    let phase20 = &text[phase20_idx..];
    assert!(
        (phase20.contains("FR94")
            || phase20.contains("FR165")
            || phase20.contains("Phase 19")
            || phase20.contains("Phase 12"))
            && (phase20.contains("仍有效")
                || phase20.contains("不得")
                || phase20.contains("不回滚")
                || phase20.contains("NFR73")),
        "Phase 20 section must keep Phase 12–19 closes valid (NFR73)"
    );
}

#[test]
fn fr166_q_defaults_four_leftovers() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase20_idx = text
        .find("Phase 20")
        .expect("addendum must mention Phase 20");
    let phase20 = &text[phase20_idx..];
    assert!(
        (phase20.contains("Q1") || phase20.contains("批准默认"))
            && phase20.contains("FR167")
            && phase20.contains("FR170"),
        "Phase 20 must record Q1 four leftovers FR167–170"
    );
    assert!(
        phase20.contains("FR168")
            && (phase20.contains("SPI") || phase20.contains("I2C") || phase20.contains("AXI")),
        "Phase 20 must record Q2 FR168 SPI+I2C+AXI"
    );
    assert!(
        phase20.contains("NFR73")
            && (phase20.contains("不得")
                || phase20.contains("不回滚")
                || phase20.contains("已关闭")),
        "Phase 20 must record Q3 NFR73 no rewrite of closed faces"
    );
    assert!(
        phase20.contains("FR142") && (phase20.contains("FR171") || phase20.contains("NFR77")),
        "Phase 20 must record Q4 claim / FR142 discipline"
    );
    assert!(
        phase20.contains("MSRV")
            && (phase20.contains("AD-9")
                || phase20.contains("AD-27")
                || phase20.contains("Q5")
                || phase20.contains("批准默认")),
        "Phase 20 must record Q5 MSRV / AD pin default"
    );
}

#[test]
fn fr166_fr_mapping_and_epics_pointer() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    let phase20_idx = addendum
        .find("Phase 20")
        .expect("addendum must mention Phase 20");
    let phase20 = &addendum[phase20_idx..];
    for fr in ["FR166", "FR167", "FR168", "FR169", "FR170", "FR171"] {
        assert!(phase20.contains(fr), "addendum Phase 20 must map FR {fr}");
    }
    assert!(
        addendum.contains("epics.md")
            && (addendum.contains("Epic 99") || addendum.contains("99–104")),
        "addendum must point at epics.md Epic 99–104"
    );
    assert!(
        epics.contains("FR166")
            && epics.contains("Epic 99")
            && epics.contains("Epic 104")
            && (epics.contains("correctCoursePhase20Approved")
                || epics.contains("phase20Contract")),
        "epics.md must carry Phase 20 inventory / approval stamp"
    );
}

#[test]
fn fr166_correct_course_phase20_approved_stamp() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("correctCoursePhase20Approved: 2026-09-12")
            || epics.contains("correctCoursePhase20Approved:2026-09-12"),
        "epics.md must carry verifiable correctCoursePhase20Approved stamp"
    );
}

#[test]
fn fr166_bitloom_brand_unchanged() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    let phase20_idx = addendum
        .find("Phase 20")
        .expect("addendum must mention Phase 20");
    let phase20_slice = &addendum[phase20_idx..];
    assert!(
        phase20_slice.contains("Bitloom")
            && (phase20_slice.contains("bitloom") || phase20_slice.contains("`bitloom")),
        "addendum Phase 20 section must affirm Bitloom / bitloom-* brand"
    );
    assert!(
        prd.contains("Bitloom")
            && (prd.contains("phase20-nfr71-four-leftovers") || prd.contains("Phase 20")),
        "prd.md must keep Bitloom brand and Phase 20 amendment stamp"
    );
    assert!(
        phase20_slice.contains("禁止")
            && (phase20_slice.contains("rhdl") || phase20_slice.contains("`rhdl")),
        "Phase 20 brand clause must forbid publishing rhdl / rhdl-bits"
    );
}

#[test]
fn fr166_prd_amendment_stamp() {
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    assert!(
        prd.contains("phase20-nfr71-four-leftovers-2026-09-12")
            || prd.contains("phase20-nfr71-four-leftovers"),
        "prd.md frontmatter/body must include phase20-nfr71-four-leftovers amendment"
    );
    assert!(
        prd.contains("FR166") && (prd.contains("NFR71") || prd.contains("Phase 20")),
        "prd.md must cite FR166 and Phase 20"
    );
}

#[test]
fn fr166_forbid_instant_publish_or_firtool_bump_on_approve() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase20_idx = text
        .find("Phase 20")
        .expect("addendum must mention Phase 20");
    let phase20 = &text[phase20_idx..];
    assert!(
        (phase20.contains("商店") || phase20.contains("firtool") || phase20.contains("publish"))
            && (phase20.contains("不") || phase20.contains("禁止") || phase20.contains("瞬间")),
        "Phase 20 must not force store publish / firtool bump at contract approval instant"
    );
}

#[test]
fn fr166_nfr14_gate_present() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("99-1-epic-99-nfr14-风险记录: done")
            || sprint.contains("99-1-epic-99-nfr14-风险记录:done"),
        "Story 99.1 NFR14 must be done before 99.2 closes"
    );
    let nfr14 = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic99-phase20-nfr71-four-leftovers.md",
    );
    assert!(
        nfr14.contains("FR166") && nfr14.contains("NFR73"),
        "Epic 99 NFR14 risk record must exist with FR166/NFR73"
    );
}

#[test]
fn fr166_git_push_not_fr() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase20-nfr71-four-leftovers.md",
    );
    assert!(
        (text.contains("git push") || text.contains("`git push`"))
            && (text.contains("不是") || text.contains("非") || text.contains("不是 FR")),
        "Correct Course must state git push is not an FR"
    );
}
