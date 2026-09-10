//! ATDD Story 69.3 — FR129 / Epic 69 closeout.

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
fn fr129_closeout() {
    let nfr = read("_agile-output/implementation-artifacts/nfr14-risk-epic69-circt-handshake.md");
    assert!(nfr.contains("- [x] **69.2 / FR129") && nfr.contains("closed"));
    let docs = read("docs/fr129-circt-handshake.md");
    assert!(docs.contains("closed") || docs.contains("已关闭"));
    assert!(docs.contains("FR121") && (docs.contains("仍有效") || docs.contains("remain")));
    let readme = read("README.md");
    assert!(readme.contains("FR129") && readme.contains("Epic 69 已关闭"));
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(deferred.contains("FR129") && deferred.contains("已关闭"));
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(sprint.contains("epic-69: done") || sprint.contains("epic-69:done"));
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(epics.contains("phase15Epic69Status: complete"));
}
