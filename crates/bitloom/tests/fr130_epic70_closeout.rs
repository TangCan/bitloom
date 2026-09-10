//! ATDD Story 70.3 — FR130 / Epic 70 closeout.

use std::fs;
use std::path::PathBuf;

fn read(rel: &str) -> String {
    fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(rel),
    )
    .unwrap()
}

#[test]
fn fr130_closeout() {
    let nfr = read("_agile-output/implementation-artifacts/nfr14-risk-epic70-style-guide.md");
    assert!(nfr.contains("- [x] **70.2 / FR130") && nfr.contains("closed"));
    let docs = read("docs/fr130-style-guide.md");
    assert!(docs.contains("closed") || docs.contains("已关闭"));
    assert!(docs.contains("FR122") && (docs.contains("仍有效") || docs.contains("remain")));
    assert!(
        docs.contains("Parser")
            && (docs.contains("not restored")
                || docs.contains("未恢复")
                || docs.contains("不恢复"))
    );
    let readme = read("README.md");
    assert!(readme.contains("FR130") && readme.contains("Epic 70 已关闭"));
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(deferred.contains("FR130") && deferred.contains("已关闭"));
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(sprint.contains("epic-70: done") || sprint.contains("epic-70:done"));
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(epics.contains("phase15Epic70Status: complete"));
}
