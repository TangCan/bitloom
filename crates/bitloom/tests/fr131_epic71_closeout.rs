//! ATDD Story 71.3 — FR131 / Epic 71 closeout.

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
fn fr131_closeout() {
    let nfr =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic71-ip-rs-protocol-split.md");
    assert!(nfr.contains("- [x] **71.2 / FR131") && nfr.contains("closed"));
    let docs = read("docs/fr131-ip-protocol-split.md");
    assert!(docs.contains("closed") || docs.contains("已关闭"));
    let readme = read("README.md");
    assert!(readme.contains("FR131") && readme.contains("Epic 71 已关闭"));
    assert!(
        readme.contains("Phase 15")
            && (readme.contains("规划故事已齐") || readme.contains("Epic 64–71"))
    );
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(deferred.contains("FR131") && deferred.contains("已关闭"));
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(sprint.contains("epic-71: done") || sprint.contains("epic-71:done"));
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(epics.contains("phase15Epic71Status: complete"));
    assert!(
        docs.contains("FR128")
            || docs.contains("SoC pad")
            || nfr.contains("FR128")
            || readme.contains("FR128")
    );
}
