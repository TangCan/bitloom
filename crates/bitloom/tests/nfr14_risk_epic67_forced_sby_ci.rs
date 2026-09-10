//! ATDD Epic 67 NFR14 FR127 (Story 67.1).

use std::fs;
use std::path::PathBuf;

#[test]
fn nfr14_epic67_forced_sby_ci() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let text = fs::read_to_string(
        root.join("_agile-output/implementation-artifacts/nfr14-risk-epic67-forced-sby-ci.md"),
    )
    .unwrap();
    assert!(text.contains("(a)") && text.contains("(b)") && text.contains("(c)") && text.contains("(d)"));
    assert!(text.contains("FR127") && text.contains("formal-sby") && text.contains("S1"));
    assert!(text.contains("continue-on-error") || text.contains("silent skip") || text.contains("不得 silent"));
    assert!(text.contains("FR119") && text.contains("67.2") && text.contains("67.3"));
    assert!(text.contains("Richard") && text.contains("NFR14") && text.contains("NFR54"));
    assert!(text.contains("NFR14-crates") && text.contains("bitloom"));
}
