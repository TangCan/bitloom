//! ATDD Story 68.3 — FR128 / Epic 68 closeout.

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
fn fr128_closeout() {
    let nfr = read("_agile-output/implementation-artifacts/nfr14-risk-epic68-soc-pad.md");
    assert!(nfr.contains("- [x] **68.2 / FR128") && nfr.contains("closed"));
    let docs = read("docs/fr128-soc-pad.md");
    assert!(docs.contains("closed") || docs.contains("已关闭"));
    assert!(docs.contains("FR120") && (docs.contains("仍有效") || docs.contains("remain")));
    let readme = read("README.md");
    assert!(readme.contains("FR128") && readme.contains("Epic 68 已关闭"));
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(deferred.contains("FR128") && deferred.contains("已关闭"));
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(sprint.contains("epic-68: done") || sprint.contains("epic-68:done"));
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(epics.contains("phase15Epic68Status: complete"));
}
