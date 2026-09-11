//! ATDD / guardrail: Story 84.2 / FR148 — Correct Course + PRD/addendum
//! Phase 18 CLI / dependency crate crates.io publishability gate.
//!
//! ```text
//! cargo test -p bitloom --test fr148_prd_phase18_gate
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
fn fr148_correct_course_approved() {
    let text = read(
        "_agile-output/planning-artifacts/sprint-change-proposal-2026-09-11-phase18-cli-crates-io-publish.md",
    );
    assert!(
        text.contains("status: approved") || text.contains("status:approved"),
        "Correct Course proposal must be status: approved"
    );
    assert!(
        text.contains("Phase 18")
            && (text.contains("CLI")
                || text.contains("crates.io")
                || text.contains("可发布")
                || text.contains("cli-crates-io")),
        "Correct Course must authorize Phase 18 CLI crates.io publishability"
    );
    assert!(
        text.contains("FR148") || text.contains("FR148–"),
        "Correct Course must cite FR148 gate / FR148–153"
    );
}

#[test]
fn fr148_addendum_phase18_cli_publish() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    assert!(
        text.contains("Phase 18")
            && (text.contains("CLI")
                || text.contains("crates.io")
                || text.contains("可发布")
                || text.contains("cli-crates-io")),
        "addendum must contain Phase 18 CLI crates.io publishability section"
    );
    assert!(
        (text.contains("FR148") && text.contains("FR153"))
            || text.contains("FR148–FR153")
            || text.contains("FR148–153"),
        "addendum Phase 18 must cite FR148–FR153"
    );
    assert!(
        (text.contains("NFR64") && text.contains("NFR67"))
            || text.contains("NFR64–NFR67")
            || text.contains("NFR64–67"),
        "addendum Phase 18 must cite NFR64–NFR67"
    );
}

#[test]
fn fr148_phase17_closes_remain_valid() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase18_idx = text
        .find("Phase 18")
        .expect("addendum must mention Phase 18");
    let phase18 = &text[phase18_idx..];
    assert!(
        (phase18.contains("FR141")
            || phase18.contains("FR147")
            || phase18.contains("Phase 17")
            || phase18.contains("Phase 12"))
            && (phase18.contains("仍有效")
                || phase18.contains("不得")
                || phase18.contains("不回滚")
                || phase18.contains("NFR64")),
        "Phase 18 section must keep Phase 17 / 12–17 closes valid (NFR64)"
    );
}

#[test]
fn fr148_cli_publish_not_clear_nfr59() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase18_idx = text
        .find("Phase 18")
        .expect("addendum must mention Phase 18");
    let phase18 = &text[phase18_idx..];
    assert!(
        (phase18.contains("不等于") || phase18.contains("≠") || phase18.contains("不是"))
            && phase18.contains("NFR59"),
        "Phase 18 must state CLI publish ≠ clear NFR59 (NFR67)"
    );
}

#[test]
fn fr148_q_defaults_rename_and_fr152b() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase18_idx = text
        .find("Phase 18")
        .expect("addendum must mention Phase 18");
    let phase18 = &text[phase18_idx..];
    assert!(
        phase18.contains("bitloom-firrtl")
            && (phase18.contains("Q1")
                || phase18.contains("批准默认")
                || phase18.contains("rename")),
        "Phase 18 must record rename → bitloom-firrtl default"
    );
    assert!(
        phase18.contains("bitloom-viz"),
        "Phase 18 must record rename → bitloom-viz default"
    );
    assert!(
        phase18.contains("FR152")
            && (phase18.contains("publish=false")
                || phase18.contains("(b)")
                || phase18.contains("bitloom-lsp")),
        "Phase 18 must record FR152(b) lsp publish strategy"
    );
    assert!(
        phase18.contains("NFR59")
            && (phase18.contains("不以") || phase18.contains("前提") || phase18.contains("Q4")),
        "Phase 18 must record no NFR59 prerequisite"
    );
    assert!(
        phase18.contains("MSRV") || phase18.contains("Q5"),
        "Phase 18 must record MSRV keep default"
    );
}

#[test]
fn fr148_fr_mapping_and_epics_pointer() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    for fr in ["FR149", "FR150", "FR151", "FR152", "FR153"] {
        assert!(
            addendum.contains(fr),
            "addendum Phase 18 must map deepen FR {fr}"
        );
    }
    assert!(
        addendum.contains("epics.md")
            && (addendum.contains("Epic 84") || addendum.contains("84–86")),
        "addendum must point at epics.md Epic 84–86"
    );
    assert!(
        epics.contains("FR148")
            && epics.contains("Epic 84")
            && epics.contains("Epic 86")
            && (epics.contains("correctCoursePhase18Approved")
                || epics.contains("phase18Contract")),
        "epics.md must carry Phase 18 inventory / approval stamp"
    );
}

#[test]
fn fr148_correct_course_phase18_approved_stamp() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("correctCoursePhase18Approved: 2026-09-11")
            || epics.contains("correctCoursePhase18Approved:2026-09-11"),
        "epics.md must carry verifiable correctCoursePhase18Approved stamp"
    );
}

#[test]
fn fr148_bitloom_brand_unchanged() {
    let addendum = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    let phase18_idx = addendum
        .find("Phase 18")
        .expect("addendum must mention Phase 18");
    let phase18_slice = &addendum[phase18_idx..];
    assert!(
        phase18_slice.contains("Bitloom")
            && (phase18_slice.contains("bitloom") || phase18_slice.contains("`bitloom")),
        "addendum Phase 18 section must affirm Bitloom / bitloom-* brand"
    );
    assert!(
        prd.contains("Bitloom")
            && (prd.contains("phase18-cli-crates-io-publish") || prd.contains("Phase 18")),
        "prd.md must keep Bitloom brand and Phase 18 amendment stamp"
    );
    assert!(
        phase18_slice.contains("禁止")
            && (phase18_slice.contains("rhdl") || phase18_slice.contains("`rhdl")),
        "Phase 18 brand clause must forbid publishing rhdl / rhdl-bits"
    );
}

#[test]
fn fr148_prd_amendment_stamp() {
    let prd = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md");
    assert!(
        prd.contains("phase18-cli-crates-io-publish-2026-09-11")
            || prd.contains("phase18-cli-crates-io-publish"),
        "prd.md frontmatter/body must include phase18-cli-crates-io-publish amendment"
    );
    assert!(
        prd.contains("FR148") && (prd.contains("CLI") || prd.contains("Phase 18")),
        "prd.md must cite FR148 and Phase 18 CLI publishability"
    );
}

#[test]
fn fr148_forbid_cargo_install_before_fr151() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let phase18_idx = text
        .find("Phase 18")
        .expect("addendum must mention Phase 18");
    let phase18 = &text[phase18_idx..];
    assert!(
        phase18.contains("FR151")
            && (phase18.contains("cargo install")
                || phase18.contains("禁止")
                || phase18.contains("暗示")),
        "Phase 18 must forbid implying cargo install bitloom before FR151"
    );
}

#[test]
fn fr148_nfr14_gate_present() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("84-1-epic-84-nfr14-风险记录: done")
            || sprint.contains("84-1-epic-84-nfr14-风险记录:done"),
        "Story 84.1 NFR14 must be done before 84.2 closes"
    );
    let nfr14 = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic84-phase18-cli-crates-io-publish.md",
    );
    assert!(
        nfr14.contains("FR148") && nfr14.contains("NFR64"),
        "Epic 84 NFR14 risk record must exist with FR148/NFR64"
    );
}
