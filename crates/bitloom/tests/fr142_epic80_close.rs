//! ATDD / guardrail: Story 80.3 / FR142 — Epic 80 closeout.
//!
//! ```text
//! cargo test -p bitloom --test fr142_epic80_close
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
fn fr142_nfr14_epic80_closed() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic80-public-api-1-0-surface.md");
    for needle in [
        "- [x] **表面清单成文 + ATDD**",
        "- [x] **README / deferred 指针**",
        "- [x] **禁止事项未触发**",
        "- [x] **品牌 / 依赖：**",
    ] {
        assert!(
            text.contains(needle),
            "missing checked close condition: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("80.3") || text.contains("Story 80.3")),
        "NFR14 must be closed with Story 80.3"
    );
    assert!(
        text.contains("不得静默扩大") || text.contains("静默扩大"),
        "must forbid silent surface expansion"
    );
}

#[test]
fn fr142_sprint_epic80_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("80-3-fr142-收口与文档指针: done")
            || sprint.contains("80-3-fr142-收口与文档指针:done"),
        "80-3 must be done"
    );
    assert!(
        sprint.contains("epic-80: done") || sprint.contains("epic-80:done"),
        "epic-80 must be done"
    );
}

#[test]
fn fr142_readme_deferred_pointer() {
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        readme.contains("public-api-1-0-surface") || readme.contains("FR142"),
        "README must point at FR142 surface"
    );
    assert!(
        deferred.contains("public-api-1-0-surface") || deferred.contains("FR142 / Epic 80"),
        "deferred must point at FR142 surface close"
    );
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase17Epic80Status: complete")
            || epics.contains("phase17Epic80Status:complete"),
        "epics.md must mark Epic 80 complete"
    );
}
