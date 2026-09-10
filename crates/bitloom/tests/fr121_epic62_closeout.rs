//! ATDD Story 62.3 — FR121 / Epic 62 closeout + FR95/96/FR110 cross-link honesty.
//!
//! ```text
//! cargo test -p bitloom --test fr121_epic62_closeout
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
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr121_nfr14_epic62_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic62-handshake-default.md");
    for needle in [
        "- [x] **62.2 / FR121：",
        "- [x] **文档 / deferred / HLS 文档 / AD-25 修订戳",
        "- [x] **禁止事项未触发",
        "- [x] **品牌 / 依赖：",
        "- [x] **FR95/96 / FR110 关闭仍有效",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 62 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 62.3") || text.contains("62.3")),
        "NFR14 must be closed with Story 62.3 pointer"
    );
}

#[test]
fn fr121_docs_readme_deferred_closed() {
    let fr121 = read("docs/fr121-handshake-default.md");
    let fr35 = read("docs/fr35-hls.md");
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        (fr121.contains("closed") || fr121.contains("已关闭"))
            && (fr121.contains("Epic 62") || fr121.contains("62.3")),
        "fr121 doc must declare Epic 62 / FR121 closed"
    );
    assert!(
        fr35.contains("FR121")
            && (fr35.contains("fr121") || fr35.contains("Handshake") || fr35.contains("handshake")),
        "fr35 must cross-link FR121"
    );
    assert!(
        readme.contains("FR121")
            && (readme.contains("Epic 62 已关闭") || readme.contains("FR121 / Epic 62 已关闭")),
        "README must note FR121 / Epic 62 closed"
    );
    assert!(
        deferred.contains("FR121")
            && (deferred.contains("已关闭") || deferred.contains("closed"))
            && (deferred.contains("62.3") || deferred.contains("Epic 62")),
        "deferred must close FR121 item"
    );
    assert!(
        epics.contains("phase14Epic62Status: complete"),
        "epics.md must stamp Epic 62 complete"
    );
    assert!(
        spine.contains("FR121")
            && spine.contains("AD-25")
            && (spine.contains("Revised") || spine.contains("修订")),
        "spine must retain AD-25 FR121 revise stamp"
    );
    // Honest uncovered / NFR51
    assert!(
        (fr121.contains("allocation")
            || fr121.contains("CIRCT")
            || fr121.contains("NFR51")
            || fr121.contains("全优化"))
            && (fr121.contains("deferred") || fr121.contains("Non-goals")),
        "must honestly disclose NFR51 deferred non-goals"
    );
    assert!(
        deferred.contains("FR122")
            && (deferred.contains("已关闭")
                || deferred.contains("closed")
                || deferred.contains("Epic 63")
                || deferred.contains("仍 deferred")),
        "deferred must still track FR122 (closed or deferred)"
    );
}

#[test]
fn fr121_fr95_fr110_mvp_still_valid() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-41: done") || sprint.contains("epic-41:done"),
        "FR95/96 / Epic 41 must remain closed (NFR48)"
    );
    assert!(
        sprint.contains("epic-52: done") || sprint.contains("epic-52:done"),
        "FR110 / Epic 52 must remain closed (NFR48)"
    );
    let fr35 = read("docs/fr35-hls.md");
    let fr110 = read("docs/fr110-hls-commercial-depth.md");
    assert!(
        fr35.contains("FR95")
            && (fr35.contains("closed") || fr35.contains("完成面") || fr35.contains("FR96")),
        "fr35 must keep FR95/96 closed surface"
    );
    assert!(
        fr110.contains("FR110")
            && (fr110.contains("closed") || fr110.contains("已关闭") || fr110.contains("52.3")),
        "fr110 must keep FR110 closed surface"
    );
}

#[test]
fn fr121_sprint_epic62_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("62-3-fr121-收口与文档指针: done")
            || sprint.contains("62-3-fr121-收口与文档指针:done")
    );
    assert!(sprint.contains("epic-62: done") || sprint.contains("epic-62:done"));
    assert!(
        sprint.contains("62-2-ad-25-修订-handshake-默认可综合路径-fr121: done")
            || sprint.contains("62-2-ad-25-修订-handshake-默认可综合路径-fr121:done")
    );
    assert!(
        sprint.contains("62-1-epic-62-nfr14-风险记录: done")
            || sprint.contains("62-1-epic-62-nfr14-风险记录:done")
    );
    assert!(
        sprint.contains("epic-63: backlog")
            || sprint.contains("epic-63:backlog")
            || sprint.contains("epic-63: in-progress")
            || sprint.contains("epic-63:in-progress")
            || sprint.contains("epic-63: done")
            || sprint.contains("epic-63:done"),
        "epic-63 must be backlog, in-progress, or done"
    );
    if !sprint.contains("epic-63: backlog") && !sprint.contains("epic-63:backlog") {
        assert!(
            sprint.contains("63-1-epic-63-nfr14-风险记录: done")
                || sprint.contains("63-1-epic-63-nfr14-风险记录:done"),
            "leaving epic-63 backlog requires Story 63.1 NFR14 done (gate)"
        );
    }
}
