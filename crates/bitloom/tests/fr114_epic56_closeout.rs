//! ATDD Story 56.3 — FR114 / Epic 56 closeout + Phase 13 story-list pointer.
//!
//! ```text
//! cargo test -p bitloom --test fr114_epic56_closeout
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
fn fr114_nfr14_epic56_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic56-waveform-coverage-gui.md");
    for needle in [
        "- [x] **56.2 / FR114：",
        "- [x] **文档 / deferred / FR104·FR105 交叉链",
        "- [x] **禁止事项未触发",
        "- [x] **品牌 / 依赖：",
        "- [x] **FR104/105 MVP 关闭仍有效",
        "- [x] **Phase 13 规划故事齐",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 56 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 56.3") || text.contains("56.3")),
        "NFR14 must be closed with Story 56.3 pointer"
    );
}

#[test]
fn fr114_docs_readme_deferred_phase13_complete() {
    let fr114 = read("docs/fr114-lcov-coverage-gui.md");
    let fr104 = read("docs/fr104-interactive-wave.md");
    let fr105 = read("docs/fr105-sim-coverage-ext.md");
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        fr114.contains("closed")
            || fr114.contains("已关闭")
            || fr114.contains("56.3")
            || fr114.contains("Epic 56"),
        "fr114 doc must declare Epic 56 closed"
    );
    assert!(
        (readme.contains("FR114") || readme.contains("LCOV"))
            && (readme.contains("已关闭") || readme.contains("Epic 56")),
        "README must note FR114 / Epic 56 closed"
    );
    assert!(
        (readme.contains("Phase 13") || readme.contains("Epic 48"))
            && (readme.contains("已齐")
                || readme.contains("48–56")
                || readme.contains("Epic 48–56")),
        "README must declare Phase 13 Epic 48–56 story list complete"
    );
    assert!(
        deferred.contains("FR114")
            && (deferred.contains("已关闭")
                || deferred.contains("closed")
                || deferred.contains("56.3")),
        "deferred must close FR114 item"
    );
    assert!(
        deferred.contains("48–56")
            || deferred.contains("Epic 48–56")
            || deferred.contains("规划/实现故事已齐"),
        "deferred must note Phase 13 stories complete"
    );
    assert!(
        fr104.contains("fr114") || fr104.contains("FR114"),
        "FR104 must cross-link FR114"
    );
    assert!(
        fr105.contains("fr114") || fr105.contains("FR114"),
        "FR105 must cross-link FR114"
    );
    assert!(
        fr114.contains("deferred") && fr114.contains("Tywaves"),
        "NFR47: Tywaves remains deferred"
    );
}

#[test]
fn fr114_fr104_fr105_mvp_still_valid() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-47: done") || sprint.contains("47-3-"),
        "FR104/105 / Epic 47 must remain closed (NFR44)"
    );
}

#[test]
fn fr114_sprint_epic56_done_phase13_epics_closed() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("56-3-fr114-收口与-phase13-指针: done")
            || sprint.contains("56-3-fr114-收口与-phase13-指针:done")
    );
    assert!(sprint.contains("epic-56: done") || sprint.contains("epic-56:done"));
    assert!(
        sprint.contains("56-2-富波形-覆盖率-gui-加深-fr114: done")
            || sprint.contains("56-2-富波形-覆盖率-gui-加深-fr114:done")
    );
    for epic in 48..=56 {
        let key = format!("epic-{epic}: done");
        let key2 = format!("epic-{epic}:done");
        assert!(
            sprint.contains(&key) || sprint.contains(&key2),
            "Phase 13 epic-{epic} must be done"
        );
    }
}
