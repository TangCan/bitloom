//! ATDD / guardrail: Story 95.3 / FR163 — Epic 95 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr163_closeout_epic95
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
fn fr163_readme_marks_fr163_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR163")
            && (readme.contains("已关闭") || readme.contains("Epic 95"))
            && (readme.contains("fr163") || readme.contains("UartRx") || readme.contains("手写")),
        "README must mark FR163 / Epic 95 closed"
    );
    assert!(
        readme.contains("FR164")
            && (readme.contains("未关闭前不得宣称") || readme.contains("不得宣称")),
        "README must keep FR164+ unclaimed"
    );
    assert!(
        (readme.contains("全清") || readme.contains("NFR59"))
            && (readme.contains("不得") || readme.contains("禁止")),
        "README must forbid NFR59 fully-cleared claim"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr163_deferred_marks_epic95_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 95")
            && deferred.contains("FR163")
            && (deferred.contains("已关闭") || deferred.contains("Story 95.3")),
        "deferred must mark Epic 95 / FR163 closed"
    );
    assert!(
        deferred.contains("FR164") && deferred.contains("Epic 96"),
        "deferred must point remaining deepen to Epic 96+"
    );
}

#[test]
fn fr163_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 95")
            && agents.contains("FR163")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 95 / FR163 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 95")
            && (spine.contains("已关闭") || spine.contains("95.3"))
            && spine.contains("FR163"),
        "spine must note Epic 95 / FR163 closed"
    );
}

#[test]
fn fr163_nfr14_epic95_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic95-unlisted-protocol-handwritten-fl-fr163.md",
    );
    for needle in [
        "- [x] **FR163 钉死子集实现 + 验收**",
        "- [x] **文档 / deferred / README / docs/ip 收口**",
        "- [x] **NFR68/71/72：**",
        "- [x] **品牌 / AD-6：**",
        "- [x] **其余 FR164–165：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 95 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 95.3"),
        "NFR14 risk record must be closed after Story 95.3"
    );
}

#[test]
fn fr163_sprint_epic95_done_96_ready() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-95: done") || sprint.contains("epic-95:done"),
        "epic-95 must be done"
    );
    assert!(
        sprint.contains("95-3-fr163-收口与文档指针: done")
            || sprint.contains("95-3-fr163-收口与文档指针:done"),
        "95-3 must be done"
    );
    assert!(
        sprint.contains("96-1-epic-96-nfr14-风险记录: ready-for-dev")
            || sprint.contains("96-1-epic-96-nfr14-风险记录:ready-for-dev")
            || sprint.contains("96-1-epic-96-nfr14-风险记录: done")
            || sprint.contains("96-1-epic-96-nfr14-风险记录: in-progress"),
        "96-1 must be ready-for-dev after Epic 95 closes"
    );
}

#[test]
fn fr163_epics_phase19_epic95_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase19Epic95Status: complete")
            || epics.contains("phase19Epic95Status:complete"),
        "epics.md must stamp phase19Epic95Status complete"
    );
}

#[test]
fn fr163_product_doc_and_ip_readme() {
    let doc = read("docs/fr163-unlisted-protocol-handwritten-fl.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("95.3"),
        "fr163 product doc must note closed status"
    );
    let ip = read("docs/ip/README.md");
    assert!(
        ip.contains("FR163") && (ip.contains("UartRxFunctional") || ip.contains("fr163")),
        "docs/ip README must point at FR163 UartRx handwritten FL"
    );
}
