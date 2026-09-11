//! ATDD: Story 82.3 / FR145 Epic 82 closeout.

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
fn fr145_nfr14_epic82_closed() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic82-fr145-hygiene-skip.md");
    for needle in [
        "- [x] **阻塞毛刺处理或 skip 文档化**",
        "- [x] **README / deferred 指针**",
        "- [x] **禁止事项未触发**",
        "- [x] **品牌：**",
    ] {
        assert!(text.contains(needle), "missing: {needle}");
    }
    assert!(text.contains("closed") && text.contains("82.3"));
}

#[test]
fn fr145_sprint_epic82_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("82-3-fr145-收口与文档指针: done")
            || sprint.contains("82-3-fr145-收口与文档指针:done")
    );
    assert!(sprint.contains("epic-82: done") || sprint.contains("epic-82:done"));
}

#[test]
fn fr145_readme_deferred_pointer() {
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(readme.contains("fr145-pre-1-0-hygiene-skip") || readme.contains("FR145-skip"));
    assert!(deferred.contains("FR145") && deferred.contains("Epic 82"));
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase17Epic82Status: complete")
            || epics.contains("phase17Epic82Status:complete")
    );
}
