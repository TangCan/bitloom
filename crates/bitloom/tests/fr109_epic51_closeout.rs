//! ATDD Story 51.3 — FR109 / Epic 51 closeout + FR105 cross-link honesty.
//!
//! ```text
//! cargo test -p bitloom --test fr109_epic51_closeout
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
fn fr109_nfr14_epic51_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic51-fsm-state-visit-coverage.md",
    );
    for needle in [
        "- [x] **51.2 / FR109：",
        "- [x] **文档 / deferred / FR105 交叉链",
        "- [x] **禁止事项未触发",
        "- [x] **品牌 / 依赖：",
        "- [x] **FR105 Mux v2 MVP 关闭仍有效",
        "- [x] **Tywaves/LCOV GUI 仍属 Epic 56",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 51 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 51.3") || text.contains("51.3")),
        "NFR14 must be closed with Story 51.3 pointer"
    );
}

#[test]
fn fr109_docs_readme_deferred_closed() {
    let fr109 = read("docs/fr109-fsm-state-visit-coverage.md");
    let fr105 = read("docs/fr105-sim-coverage-ext.md");
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        fr109.contains("closed")
            || fr109.contains("已关闭")
            || fr109.contains("51.3")
            || fr109.contains("Epic 51"),
        "fr109 doc must declare Epic 51 closed"
    );
    assert!(
        (readme.contains("FR109") || readme.contains("state-visit") || readme.contains("FSM"))
            && (readme.contains("已关闭") || readme.contains("Epic 51")),
        "README must note FR109 / Epic 51 closed"
    );
    assert!(
        deferred.contains("FR109")
            && (deferred.contains("已关闭")
                || deferred.contains("closed")
                || deferred.contains("51.3")),
        "deferred must close C3 / FR109 item"
    );
    assert!(
        fr109.contains("NFR47")
            || fr109.contains("Non-goals")
            || fr109.contains("Tywaves")
            || fr109.contains("Epic 56"),
        "fr109 must keep NFR47 / Epic 56 honesty"
    );
    assert!(
        fr105.contains("FR109")
            && (fr105.contains("C3") || fr105.contains("FSM") || fr105.contains("state")),
        "FR105 must cross-link FR109 for C3"
    );
}

#[test]
fn fr109_fr105_cross_link_and_mvp_still_valid() {
    let fr109 = read("docs/fr109-fsm-state-visit-coverage.md");
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        fr109.contains("FR105")
            && (fr109.contains("Mux") || fr109.contains("v2") || fr109.contains("仍")),
        "FR105 Mux v2 cross-link retained"
    );
    assert!(
        fr109.contains("Phase 13") || fr109.contains("Epic 51") || fr109.contains("C3"),
        "FR109 is Phase 13 C3 deepen face"
    );
    assert!(
        sprint.contains("epic-47: done") || sprint.contains("47-3-"),
        "FR105 / Epic 47 must remain closed (NFR44)"
    );
}

#[test]
fn fr109_sprint_epic51_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("51-3-fr109-收口与-fr105-交叉链: done")
            || sprint.contains("51-3-fr109-收口与-fr105-交叉链:done")
    );
    assert!(sprint.contains("epic-51: done") || sprint.contains("epic-51:done"));
    assert!(
        sprint.contains("51-2-fsm-state-visit-记录器与夹具-fr109: done")
            || sprint.contains("51-2-fsm-state-visit-记录器与夹具-fr109:done")
            || sprint.contains("51-2-fsm-state-visit-覆盖率-fr109: done")
            || sprint.contains("51-2-fsm-state-visit-覆盖率-fr109:done")
    );
}
