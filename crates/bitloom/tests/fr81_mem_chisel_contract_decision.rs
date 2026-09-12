//! ATDD / guardrail: FR81 Mem→Chisel contract decision (Story 33.2).
//! Red if decision file missing, dual-path / Path-B-only, or required
//! subset / NFR12 / NFR37 sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn decision_text() -> String {
    let path = workspace_root().join(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/fr81-mem-chisel-contract-decision-2026-09-09.md",
    );
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn fr81_decision_exists_and_picks_exactly_path_a() {
    let text = decision_text();

    assert!(
        text.contains("FR81") && (text.contains("ADOPTED") || text.contains("裁决")),
        "decision page must situate FR81 as adopted/ruled"
    );

    // Exactly Path A — support documented Mem subset
    assert!(
        text.contains("Path A")
            && (text.contains("支持文档化 Mem 子集")
                || text.contains("支持文档化")
                || (text.contains("支持") && text.contains("子集"))),
        "decision must select Path A (documented Mem subset)"
    );
    assert!(
        (text.contains("Path B") || text.contains("永久非目标"))
            && (text.contains("拒绝")
                || text.contains("未选定")
                || text.contains("不作为")
                || text.contains("明确拒绝")),
        "decision must explicitly reject / not select Path B"
    );
    // Must not claim Path B as the adopted close path
    assert!(
        !text.contains("选定路径** | **Path B")
            && !text.contains("选定路径**|**Path B")
            && !text.contains("**选定路径：** Path B")
            && !text.contains("选定 **Path B"),
        "decision must not adopt Path B as the selected path"
    );

    // Supported Mem shapes (subset)
    assert!(
        text.contains("SyncReadMem") && text.contains("Mem"),
        "Path A must list SyncReadMem and Mem shapes"
    );
    assert!(
        text.contains("sync_read")
            || text.contains("单时钟")
            || text.contains("AD-21")
            || text.contains("单口"),
        "subset must situate AD-21 single-clock / sync_read surface"
    );

    // NFR12 pinned pair
    assert!(
        text.contains("NFR12")
            && (text.contains("7.14.0") || text.contains("7.15.0"))
            && (text.contains("1.155.0") || text.contains("1.158.0")),
        "decision must document NFR12 Chisel 7.14.0 ↔ firtool 1.155.0"
    );

    // NFR37: historical done ≠ depth close
    assert!(
        text.contains("NFR37")
            && (text.contains("done") || text.contains("深度") || text.contains("关闭"))
            && (text.contains("FR28") || text.contains("E0901") || text.contains("≠")),
        "decision must cite NFR37 (historical done ≠ depth close)"
    );

    // Keep E0901 for out-of-subset; do not break FR71
    assert!(
        text.contains("E0901")
            && (text.contains("子集外") || text.contains("保留") || text.contains("不得")),
        "decision must keep E0901 for unsupported shapes"
    );
    assert!(
        text.contains("FR71")
            && (text.contains("不得") || text.contains("破坏") || text.contains("变红")),
        "decision must forbid breaking FR71"
    );

    assert!(
        text.contains("AD-27") && text.contains("Epic 33"),
        "decision must cite AD-27 and link Epic 33"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom-prelude"),
        "decision must use Bitloom brand / prelude boundary"
    );
}

#[test]
fn fr81_and_epic33_link_to_decision_page() {
    let epics = workspace_root().join("_agile-output/planning-artifacts/epics.md");
    let text =
        fs::read_to_string(&epics).unwrap_or_else(|e| panic!("read {}: {e}", epics.display()));

    assert!(
        text.contains("FR81")
            && text.contains("fr81-mem-chisel-contract-decision-2026-09-09")
            && (text.contains("决策") || text.contains("Path A") || text.contains("合同")),
        "FR81 / Epic 33 in epics.md must link to the FR81 decision artifact"
    );

    let addendum = workspace_root()
        .join("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");
    let add = fs::read_to_string(&addendum)
        .unwrap_or_else(|e| panic!("read {}: {e}", addendum.display()));
    assert!(
        add.contains("fr81-mem-chisel-contract-decision-2026-09-09")
            && add.contains("FR81")
            && (add.contains("Path A") || add.contains("Mem")),
        "PRD addendum must link FR81 Path A decision page"
    );

    let risk = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic33-chisel-mem.md");
    let risk_text =
        fs::read_to_string(&risk).unwrap_or_else(|e| panic!("read {}: {e}", risk.display()));
    assert!(
        risk_text.contains("fr81-mem-chisel-contract-decision-2026-09-09")
            && (risk_text.contains("Path A") || risk_text.contains("选定")),
        "NFR14 Epic 33 risk record must back-link selected Path A"
    );
}
