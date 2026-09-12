//! ATDD / guardrail: Story 101.3 / FR168 — Epic 101 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr168_closeout_epic101
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
fn fr168_readme_marks_fr168_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR168")
            && (readme.contains("已关闭") || readme.contains("Epic 101"))
            && (readme.contains("fr168") || readme.contains("SPI") || readme.contains("SpiMaster")),
        "README must mark FR168 / Epic 101 closed"
    );
    assert!(
        readme.contains("FR169")
            && (readme.contains("未关闭前不得宣称") || readme.contains("不得宣称")),
        "README must keep FR169+ unclaimed"
    );
    assert!(readme.contains("Bitloom"));
}

#[test]
fn fr168_deferred_marks_epic101_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 101")
            && deferred.contains("FR168")
            && (deferred.contains("已关闭") || deferred.contains("Story 101.3")),
        "deferred must mark Epic 101 / FR168 closed"
    );
    assert!(
        deferred.contains("FR169") && deferred.contains("Epic 102"),
        "deferred must point remaining deepen to Epic 102+"
    );
}

#[test]
fn fr168_agents_and_spine_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 101")
            && agents.contains("FR168")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 101 / FR168 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 101")
            && (spine.contains("已关闭") || spine.contains("101.3"))
            && spine.contains("FR168"),
        "spine must note Epic 101 / FR168 closed"
    );
}

#[test]
fn fr168_nfr14_epic101_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic101-spi-i2c-axi-handwritten-fl-fr168.md",
    );
    for needle in [
        "- [x] **FR168 钉死子集实现 + 验收**",
        "- [x] **文档 / deferred / README / docs/ip 收口**",
        "- [x] **NFR73/76：**",
        "- [x] **品牌 / AD-6：**",
        "- [x] **其余 FR169–171：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 101 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 101.3"),
        "NFR14 risk record must be closed after Story 101.3"
    );
}

#[test]
fn fr168_sprint_epic101_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-101: done") || sprint.contains("epic-101:done"),
        "epic-101 must be done"
    );
    assert!(
        sprint.contains("101-3-fr168-收口与文档指针: done")
            || sprint.contains("101-3-fr168-收口与文档指针:done"),
        "101-3 must be done"
    );
}

#[test]
fn fr168_epics_phase20_epic101_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase20Epic101Status: complete")
            || epics.contains("phase20Epic101Status:complete"),
        "epics.md must stamp phase20Epic101Status complete"
    );
}

#[test]
fn fr168_product_doc_and_ip_readme() {
    let doc = read("docs/fr168-spi-i2c-axi-handwritten-fl.md");
    assert!(
        doc.contains("closed") || doc.contains("已关闭") || doc.contains("101.3"),
        "fr168 product doc must note closed status"
    );
    let ip = read("docs/ip/README.md");
    assert!(
        ip.contains("FR168") && (ip.contains("SpiMasterFunctional") || ip.contains("手写")),
        "docs/ip README must point at FR168 handwritten FL"
    );
}
