//! ATDD Story 47.3 — FR105 / Epic 47 closeout + Phase 12 story-list honesty.
//!
//! ```text
//! cargo test -p bitloom --test fr105_epic47_closeout
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
fn fr105_nfr14_epic47_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic47-waveform-coverage.md");
    for needle in [
        "- [x] **47.2 / FR104：",
        "- [x] **47.3 / FR105：",
        "- [x] **文档 / deferred / doc-19：",
        "- [x] **禁止事项未触发：",
        "- [x] **品牌 / 依赖：",
        "- [x] **Phase 12 故事清单：",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 47 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 47.3") || text.contains("47.3")),
        "NFR14 Epic 47 record must be closed with Story 47.3 pointer"
    );
}

#[test]
fn fr105_deferred_readme_epic47_closed() {
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    let agents = read("AGENTS.md");
    let fr105 = read("docs/fr105-sim-coverage-ext.md");
    let doc19 = read("docs/requirements/19. 实施路线图.md");

    assert!(
        fr105.contains("已关闭")
            || fr105.contains("closed")
            || fr105.contains("Epic 47") && fr105.contains("done"),
        "fr105 doc must declare Epic 47 / FR105 closed"
    );
    assert!(
        (readme.contains("FR105") || readme.contains("覆盖率"))
            && (readme.contains("已关闭")
                || readme.contains("Epic 47 已")
                || readme.contains("epic-47 done")
                || readme.contains("Epic 47 closed")),
        "README must note FR105 / Epic 47 closed"
    );
    assert!(
        deferred.contains("覆盖率")
            && (deferred.contains("closed")
                || deferred.contains("已关闭")
                || deferred.contains("47.3")),
        "deferred-work must close coverage defer via 47.3"
    );
    assert!(
        agents.contains("FR105") || agents.contains("Epic 47") || agents.contains("FR104"),
        "AGENTS.md should mention Epic 47 / FR104/105 close"
    );
    assert!(
        readme.contains("剩余门")
            || readme.contains("remaining")
            || doc19.contains("剩余门")
            || fr105.contains("剩余门")
            || (readme.contains("各 epic") && readme.contains("关闭"))
            || (readme.contains("Epic 40–47") || readme.contains("Epic 40-47"))
                && (readme.contains("已关闭") || readme.contains("Phase 13"))
            || doc19.contains("Epic 40–47 已关闭")
            || doc19.contains("Phase 13 加深"),
        "status surface must declare Phase 12 epic close and/or Phase 13 deepen pointer"
    );
}

#[test]
fn fr105_sprint_epic47_done_phase12_closed() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("47-3-仿真覆盖率记录扩展-phase-12-故事收口-fr105: done")
            || sprint.contains("47-3-仿真覆盖率记录扩展-phase-12-故事收口-fr105:done")
            || sprint.contains("47-3-仿真覆盖率扩展-fr105-收口: done")
            || sprint.contains("47-3-仿真覆盖率扩展-fr105-收口:done"),
        "47-3 must be done"
    );
    assert!(
        sprint.contains("epic-47: done") || sprint.contains("epic-47:done"),
        "epic-47 must be done"
    );
    // Phase 13 Correct Course (2026-09-10) may seed epic-48+; Phase 12 closeout
    // no longer forbids starting Epic 48 under the new contract. Gate for 49–56
    // remains Epic 48 done (see nfr14-risk-epic48 / Story 48.1).
    // Phase 12 epic keys 40–47 should be done (retros may stay optional)
    for epic in 40..=47 {
        let key = format!("epic-{epic}: done");
        let key_tight = format!("epic-{epic}:done");
        assert!(
            sprint.contains(&key) || sprint.contains(&key_tight),
            "Phase 12 {key} required"
        );
    }
}
