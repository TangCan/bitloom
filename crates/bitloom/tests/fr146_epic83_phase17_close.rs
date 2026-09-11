//! ATDD: Story 83.3 / FR146 Epic 83 + Phase 17 closeout.

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
fn fr146_nfr14_epic83_closed() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic83-bitloom-1-0-0-release.md");
    for needle in [
        "- [x] **发版 1.0.0 + tag + CHANGELOG**",
        "- [x] **README / deferred / Phase 17 指针**",
        "- [x] **禁止事项未触发**",
        "- [x] **品牌：**",
    ] {
        assert!(text.contains(needle), "missing: {needle}");
    }
    assert!(text.contains("closed") && text.contains("83.3"));
    assert!(text.contains("FR147") || text.contains("Phase 17"));
}

#[test]
fn fr146_sprint_epic83_and_phase17_stories_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("83-3-fr146-收口与-phase-17-故事清单指针: done")
            || sprint.contains("83-3-fr146-收口与-phase-17-故事清单指针:done")
    );
    assert!(sprint.contains("epic-83: done") || sprint.contains("epic-83:done"));
    for prefix in ["79-", "80-", "81-", "82-", "83-"] {
        for line in sprint.lines() {
            let t = line.trim();
            if t.starts_with(prefix)
                && !t.contains("retrospective")
                && t.contains(':')
                && !t.contains("done")
            {
                panic!("Phase 17 story not done: {t}");
            }
        }
    }
}

#[test]
fn fr146_epics_phase17_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(epics.contains("phase17Status: complete") || epics.contains("phase17Status:complete"));
    assert!(
        epics.contains("phase17Epic83Status: complete")
            || epics.contains("phase17Epic83Status:complete")
    );
}

#[test]
fn fr146_readme_deferred_fr147() {
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(readme.contains("v1.0.0") || readme.contains("1.0.0"));
    assert!(readme.contains("NFR59"));
    assert!(deferred.contains("Epic 83") && deferred.contains("FR146"));
    assert!(deferred.contains("NFR59") && deferred.contains("NFR63"));
}
