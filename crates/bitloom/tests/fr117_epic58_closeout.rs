//! ATDD Story 58.3 — FR117 / Epic 58 closeout + FR104/FR114 cross-link honesty.
//!
//! ```text
//! cargo test -p bitloom --test fr117_epic58_closeout
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
fn fr117_nfr14_epic58_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic58-tywaves-typed-ide-waveform.md",
    );
    for needle in [
        "- [x] **58.2 / FR117：",
        "- [x] **文档 / deferred / FR104·FR114 交叉链",
        "- [x] **禁止事项未触发",
        "- [x] **品牌 / 依赖：",
        "- [x] **FR104/114 关闭仍有效",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 58 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 58.3") || text.contains("58.3")),
        "NFR14 must be closed with Story 58.3 pointer"
    );
}

#[test]
fn fr117_docs_readme_deferred_closed() {
    let fr117 = read("docs/fr117-typed-ide-wave.md");
    let fr104 = read("docs/fr104-interactive-wave.md");
    let fr114 = read("docs/fr114-lcov-coverage-gui.md");
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        (fr117.contains("closed") || fr117.contains("已关闭"))
            && (fr117.contains("Epic 58") || fr117.contains("58.3")),
        "fr117 doc must declare Epic 58 / FR117 closed"
    );
    assert!(
        readme.contains("FR117")
            && (readme.contains("Epic 58 已关闭") || readme.contains("FR117 / Epic 58 已关闭")),
        "README must note FR117 / Epic 58 closed"
    );
    assert!(
        !readme.contains("待 Epic 57 闸门"),
        "README must not still say waiting on Epic 57 gate"
    );
    assert!(
        readme.contains("子集 B") || readme.contains("自研 typed") || readme.contains("in-house"),
        "README FR123 honesty must distinguish in-house B from upstream Tywaves A"
    );
    assert!(
        deferred.contains("FR117")
            && (deferred.contains("已关闭") || deferred.contains("closed"))
            && (deferred.contains("58.3") || deferred.contains("Epic 58")),
        "deferred must close FR117 item"
    );
    assert!(
        epics.contains("phase14Epic58Status: complete"),
        "epics.md must stamp Epic 58 complete"
    );
    assert!(
        fr104.contains("fr117") || fr104.contains("FR117"),
        "FR104 must cross-link FR117"
    );
    assert!(
        fr114.contains("fr117") || fr114.contains("FR117"),
        "FR114 must cross-link FR117"
    );
    assert!(
        fr117.contains("deferred") && fr117.contains("Tywaves"),
        "NFR51: Tywaves A remains deferred"
    );
    assert!(
        (fr104.contains("Epic 58") || fr104.contains("58.3"))
            && (fr104.contains("closed") || fr104.contains("已关闭") || fr104.contains("FR117")),
        "FR104 cross-link must affirm Epic 58 / FR117 closed"
    );
    assert!(
        fr114.contains("deferred") && fr114.contains("Tywaves"),
        "FR114 must keep Tywaves A deferred"
    );
}

#[test]
fn fr117_fr104_fr114_mvp_still_valid() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-47: done") || sprint.contains("epic-47:done"),
        "FR104/105 / Epic 47 must remain closed (NFR48)"
    );
    assert!(
        sprint.contains("epic-56: done") || sprint.contains("epic-56:done"),
        "FR114 / Epic 56 must remain closed (NFR48)"
    );
    let fr104 = read("docs/fr104-interactive-wave.md");
    let fr114 = read("docs/fr114-lcov-coverage-gui.md");
    assert!(
        fr104.contains("interactive.html")
            && (fr104.contains("closed")
                || fr104.contains("已关闭")
                || fr104.contains("Epic 47")
                || fr104.contains("FR104")),
        "FR104 docs must remain valid"
    );
    assert!(
        (fr114.contains("closed") || fr114.contains("已关闭") || fr114.contains("Epic 56"))
            && (fr114.contains("coverage.lcov") || fr114.contains("coverage.html")),
        "FR114 docs must remain closed/valid"
    );
}

#[test]
fn fr117_sprint_epic58_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("58-3-fr117-收口与文档指针: done")
            || sprint.contains("58-3-fr117-收口与文档指针:done")
    );
    assert!(sprint.contains("epic-58: done") || sprint.contains("epic-58:done"));
    assert!(
        sprint.contains("58-2-typed-ide-波形路径实现与验收-fr117: done")
            || sprint.contains("58-2-typed-ide-波形路径实现与验收-fr117:done")
    );
    // Coupled: story and epic close together.
    let story_done = sprint.contains("58-3-fr117-收口与文档指针: done")
        || sprint.contains("58-3-fr117-收口与文档指针:done");
    let epic_done = sprint.contains("epic-58: done") || sprint.contains("epic-58:done");
    assert!(
        story_done == epic_done,
        "58-3 done and epic-58 done must stay coupled"
    );
}

#[test]
fn fr117_tywaves_a_remains_deferred() {
    let fr117 = read("docs/fr117-typed-ide-wave.md");
    let nfr14 = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic58-tywaves-typed-ide-waveform.md",
    );
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    let readme = read("README.md");
    assert!(
        fr117.contains("deferred") && fr117.contains("Tywaves"),
        "docs must keep Tywaves A deferred (NFR51)"
    );
    assert!(
        nfr14.contains("deferred") && (nfr14.contains("(A)") || nfr14.contains("Tywaves")),
        "NFR14 must keep subset A deferred"
    );
    // Reject claiming upstream Tywaves delivered as the FR117 face.
    let lower = fr117.to_lowercase();
    let claims_tywaves_delivered = lower.contains("tywaves")
        && (lower.contains("tywaves first-class")
            || lower.contains("tywaves delivered")
            || lower.contains("upstream tywaves"))
        && !fr117.contains("deferred");
    assert!(
        !claims_tywaves_delivered,
        "must not silent-claim Tywaves A delivered"
    );
    assert!(
        readme.contains("不得") && (readme.contains("Tywaves") || readme.contains("上游")),
        "README must forbid claiming upstream Tywaves from FR117 close"
    );
    // FR120–122 remain deferred delivery (FR118/FR119 may now be closed).
    assert!(
        deferred.contains("FR120")
            && (deferred.contains("仍 deferred")
                || deferred.contains("FR120–122")
                || deferred.contains("FR120–FR122")),
        "deferred must keep FR120–122 as remaining deferred delivery"
    );
    for fr in ["FR118", "FR119", "FR120", "FR121", "FR122"] {
        assert!(deferred.contains(fr), "deferred must still mention {fr}");
    }
    // Sprint: Epic 61–63 stay backlog; Epic 59/60 may be done.
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    for epic in 61..=63 {
        let done = format!("epic-{epic}: done");
        let done2 = format!("epic-{epic}:done");
        assert!(
            !sprint.contains(&done) && !sprint.contains(&done2),
            "epic-{epic} must not be done yet"
        );
        assert!(
            sprint.contains(&format!("epic-{epic}: backlog"))
                || sprint.contains(&format!("epic-{epic}:backlog")),
            "epic-{epic} must remain backlog"
        );
    }
    assert!(
        sprint.contains("epic-60: backlog")
            || sprint.contains("epic-60:backlog")
            || sprint.contains("epic-60: in-progress")
            || sprint.contains("epic-60:in-progress")
            || sprint.contains("epic-60: done")
            || sprint.contains("epic-60:done"),
        "epic-60 must be backlog, in-progress, or done"
    );
    assert!(
        sprint.contains("epic-59: backlog")
            || sprint.contains("epic-59:backlog")
            || sprint.contains("epic-59: in-progress")
            || sprint.contains("epic-59:in-progress")
            || sprint.contains("epic-59: done")
            || sprint.contains("epic-59:done"),
        "epic-59 must be backlog, in-progress, or done"
    );
}
