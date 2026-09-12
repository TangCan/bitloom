//! ATDD / guardrail: Story 87.2 / FR154 — Correct Course + PRD/addendum
//! Phase 19 NFR59 full upgrade + FR152(a) gate.
//!
//! ```text
//! cargo test -p bitloom --test fr154_prd_phase19_gate
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
fn fr154_correct_course_approved() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase19-nfr59-fr152a.md",
    );
    assert!(
        text.contains("status: approved") || text.contains("status:approved"),
        "Correct Course proposal must be status: approved"
    );
    assert!(
        text.contains("Phase 19")
            && (text.contains("NFR59") || text.contains("FR152") || text.contains("FR154")),
        "Correct Course must authorize Phase 19 NFR59 / FR152(a)"
    );
    assert!(
        text.contains("FR154") || text.contains("FR154–"),
        "Correct Course must cite FR154 gate / FR154–165"
    );
}

#[test]
fn fr154_addendum_phase19() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    assert!(
        text.contains("Phase 19")
            && (text.contains("NFR59") || text.contains("FR152") || text.contains("FR154")),
        "addendum must contain Phase 19 section"
    );
    assert!(
        (text.contains("FR154") && text.contains("FR165"))
            || text.contains("FR154–FR165")
            || text.contains("FR154–165"),
        "addendum Phase 19 must cite FR154–FR165"
    );
    assert!(
        (text.contains("NFR68") && text.contains("NFR72"))
            || text.contains("NFR68–NFR72")
            || text.contains("NFR68–72"),
        "addendum Phase 19 must cite NFR68–NFR72"
    );
}

#[test]
fn fr154_phase12_18_closes_remain_valid() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase19_idx = text
        .find("Phase 19")
        .expect("addendum must mention Phase 19");
    let phase19 = &text[phase19_idx..];
    assert!(
        (phase19.contains("FR94")
            || phase19.contains("FR153")
            || phase19.contains("Phase 18")
            || phase19.contains("Phase 12"))
            && (phase19.contains("仍有效")
                || phase19.contains("不得")
                || phase19.contains("不回滚")
                || phase19.contains("NFR68")),
        "Phase 19 section must keep Phase 12–18 closes valid (NFR68)"
    );
}

#[test]
fn fr154_q_defaults_nfr59_full_and_fr152a() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase19_idx = text
        .find("Phase 19")
        .expect("addendum must mention Phase 19");
    let phase19 = &text[phase19_idx..];
    assert!(
        (phase19.contains("Q1") || phase19.contains("批准默认"))
            && phase19.contains("FR157")
            && phase19.contains("FR165"),
        "Phase 19 must record Q1 NFR59 full FR157–165"
    );
    assert!(
        phase19.contains("FR152")
            && (phase19.contains("(a)") || phase19.contains("FR152(a)"))
            && phase19.contains("live"),
        "Phase 19 must record Q2 FR152(a) live publish"
    );
    assert!(
        phase19.contains("NFR68")
            && (phase19.contains("不得")
                || phase19.contains("不回滚")
                || phase19.contains("已关闭")),
        "Phase 19 must record Q3 NFR68 no rewrite of closed faces"
    );
    assert!(
        phase19.contains("FR142") && (phase19.contains("FR156") || phase19.contains("NFR72")),
        "Phase 19 must record Q4 claim / FR142 discipline"
    );
    assert!(
        phase19.contains("MSRV")
            && (phase19.contains("1.0.0")
                || phase19.contains("Q5")
                || phase19.contains("批准默认")),
        "Phase 19 must record Q5 MSRV default"
    );
}

#[test]
fn fr154_fr_mapping_and_epics_pointer() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    let phase19_idx = addendum
        .find("Phase 19")
        .expect("addendum must mention Phase 19");
    let phase19 = &addendum[phase19_idx..];
    for fr in ["FR154", "FR155", "FR156"] {
        assert!(
            phase19.contains(fr),
            "addendum Phase 19 must map gate/lsp/claim FR {fr}"
        );
    }
    assert!(
        (phase19.contains("FR157") && phase19.contains("FR165"))
            || phase19.contains("FR157–FR165")
            || phase19.contains("FR157–165"),
        "addendum Phase 19 must map NFR59 range FR157–FR165"
    );
    assert!(
        addendum.contains("epics.md")
            && (addendum.contains("Epic 87") || addendum.contains("87–98")),
        "addendum must point at epics.md Epic 87–98"
    );
    assert!(
        epics.contains("FR154")
            && epics.contains("Epic 87")
            && epics.contains("Epic 98")
            && (epics.contains("correctCoursePhase19Approved")
                || epics.contains("phase19Contract")),
        "epics.md must carry Phase 19 inventory / approval stamp"
    );
}

#[test]
fn fr154_correct_course_phase19_approved_stamp() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("correctCoursePhase19Approved: 2026-09-12")
            || epics.contains("correctCoursePhase19Approved:2026-09-12"),
        "epics.md must carry verifiable correctCoursePhase19Approved stamp"
    );
}

#[test]
fn fr154_bitloom_brand_unchanged() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    let phase19_idx = addendum
        .find("Phase 19")
        .expect("addendum must mention Phase 19");
    let phase19_slice = &addendum[phase19_idx..];
    assert!(
        phase19_slice.contains("Bitloom")
            && (phase19_slice.contains("bitloom") || phase19_slice.contains("`bitloom")),
        "addendum Phase 19 section must affirm Bitloom / bitloom-* brand"
    );
    assert!(
        prd.contains("Bitloom")
            && (prd.contains("phase19-nfr59-fr152a") || prd.contains("Phase 19")),
        "prd.md must keep Bitloom brand and Phase 19 amendment stamp"
    );
    assert!(
        phase19_slice.contains("禁止")
            && (phase19_slice.contains("rhdl") || phase19_slice.contains("`rhdl")),
        "Phase 19 brand clause must forbid publishing rhdl / rhdl-bits"
    );
}

#[test]
fn fr154_prd_amendment_stamp() {
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    assert!(
        prd.contains("phase19-nfr59-fr152a-2026-09-12") || prd.contains("phase19-nfr59-fr152a"),
        "prd.md frontmatter/body must include phase19-nfr59-fr152a amendment"
    );
    assert!(
        prd.contains("FR154") && (prd.contains("NFR59") || prd.contains("Phase 19")),
        "prd.md must cite FR154 and Phase 19"
    );
}

#[test]
fn fr154_forbid_instant_cargo_publish_on_approve() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase19_idx = text
        .find("Phase 19")
        .expect("addendum must mention Phase 19");
    let phase19 = &text[phase19_idx..];
    assert!(
        phase19.contains("cargo publish")
            && (phase19.contains("不") || phase19.contains("禁止") || phase19.contains("瞬间")),
        "Phase 19 must not force cargo publish at contract approval instant"
    );
}

#[test]
fn fr154_nfr14_gate_present() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("87-1-epic-87-nfr14-风险记录: done")
            || sprint.contains("87-1-epic-87-nfr14-风险记录:done"),
        "Story 87.1 NFR14 must be done before 87.2 closes"
    );
    let nfr14 =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic87-phase19-nfr59-fr152a.md");
    assert!(
        nfr14.contains("FR154") && nfr14.contains("NFR68"),
        "Epic 87 NFR14 risk record must exist with FR154/NFR68"
    );
}

#[test]
fn fr154_git_push_not_fr() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase19-nfr59-fr152a.md",
    );
    assert!(
        (text.contains("git push") || text.contains("`git push`"))
            && (text.contains("不是") || text.contains("非") || text.contains("不是 FR")),
        "Correct Course must state git push is not an FR"
    );
}
