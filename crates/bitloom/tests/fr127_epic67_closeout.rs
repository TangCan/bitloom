//! ATDD Story 67.3 — FR127 closeout.

use std::fs;
use std::path::PathBuf;

fn read(rel: &str) -> String {
    fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..").join(rel)).unwrap()
}

#[test]
fn fr127_closeout() {
    let nfr = read("_agile-output/implementation-artifacts/nfr14-risk-epic67-forced-sby-ci.md");
    assert!(nfr.contains("- [x] **67.2 / FR127") && nfr.contains("closed"));
    let docs = read("docs/fr127-forced-sby-ci.md");
    assert!(docs.contains("closed") || docs.contains("已关闭"));
    let readme = read("README.md");
    assert!(readme.contains("FR127") && readme.contains("Epic 67 已关闭"));
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(deferred.contains("FR127") && deferred.contains("已关闭"));
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(sprint.contains("epic-67: done") || sprint.contains("epic-67:done"));
    assert!(sprint.contains("epic-60: done") || sprint.contains("epic-60:done"));
}
