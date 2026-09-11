//! ATDD: Story 81.4 / FR143+FR144 Epic 81 closeout.

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
fn fr143_fr144_nfr14_epic81_closed() {
    let text = read("_agile-output/implementation-artifacts/nfr14-risk-epic81-semver-1-0-ci.md");
    for needle in [
        "- [x] **SemVer 1.0 政策成文**",
        "- [x] **semver CI / just 门禁**",
        "- [x] **README / deferred 指针**",
        "- [x] **禁止事项未触发**",
        "- [x] **品牌：**",
    ] {
        assert!(
            text.contains(needle),
            "missing checked close condition: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("81.4") || text.contains("Story 81.4")),
        "NFR14 must be closed with Story 81.4"
    );
}

#[test]
fn fr143_fr144_sprint_epic81_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("81-4-fr143-fr144-收口与文档指针: done")
            || sprint.contains("81-4-fr143-fr144-收口与文档指针:done"),
        "81-4 must be done"
    );
    assert!(
        sprint.contains("epic-81: done") || sprint.contains("epic-81:done"),
        "epic-81 must be done"
    );
}

#[test]
fn fr143_fr144_readme_deferred_pointer() {
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        readme.contains("semver-1-0-policy") && readme.contains("FR143"),
        "README must point at FR143 policy"
    );
    assert!(
        readme.contains("FR144") && (readme.contains("semver-check") || readme.contains("Epic 81")),
        "README must note FR144 / Epic 81 close"
    );
    assert!(
        deferred.contains("FR143") && deferred.contains("FR144") && deferred.contains("Epic 81"),
        "deferred must note Epic 81 / FR143+FR144 close"
    );
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase17Epic81Status: complete")
            || epics.contains("phase17Epic81Status:complete"),
        "epics.md must mark Epic 81 complete"
    );
}
