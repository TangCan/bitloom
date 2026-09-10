//! ATDD Story 66.3 — FR126 closeout.

use std::fs;
use std::path::PathBuf;

fn read(rel: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    fs::read_to_string(root.join(rel)).unwrap()
}

#[test]
fn fr126_closeout() {
    let nfr =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic66-more-ip-handwritten-fl.md");
    assert!(nfr.contains("- [x] **66.2 / FR126") && nfr.contains("closed"));
    let docs = read("docs/fr126-more-ip-handwritten-fl.md");
    assert!(docs.contains("closed") || docs.contains("已关闭"));
    let readme = read("README.md");
    assert!(readme.contains("FR126") && readme.contains("Epic 66 已关闭"));
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(deferred.contains("FR126") && deferred.contains("已关闭"));
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(sprint.contains("epic-66: done") || sprint.contains("epic-66:done"));
    assert!(sprint.contains("epic-54: done") || sprint.contains("epic-54:done"));
    assert!(sprint.contains("epic-60: done") || sprint.contains("epic-60:done"));
}
