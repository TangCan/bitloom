//! ATDD Epic 70 NFR14 FR130 (Story 70.1).

use std::fs;
use std::path::PathBuf;

#[test]
fn nfr14_epic70_style_guide() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let text = fs::read_to_string(
        root.join("_agile-output/implementation-artifacts/nfr14-risk-epic70-style-guide.md"),
    )
    .unwrap();
    assert!(
        text.contains("(a)")
            && text.contains("(b)")
            && text.contains("(c)")
            && text.contains("(d)")
    );
    assert!(text.contains("FR130") && text.contains("G1") && text.contains("G4"));
    assert!(text.contains("Style Guide") || text.contains("style-guide"));
    assert!(text.contains("AD-27") && (text.contains("修订") || text.contains("revise")));
    assert!(text.contains("不恢复") || text.contains("不得静默恢复") || text.contains("Parser"));
    assert!(text.contains("FR122") && (text.contains("不得仅") || text.contains("alone")));
    assert!(text.contains("FR97") && text.contains("FR111"));
    assert!(text.contains("docs-only") || text.contains("不得 docs"));
    assert!(text.contains("70.2") && text.contains("70.3") && text.contains("ready"));
    assert!(text.contains("Richard") && text.contains("NFR14") && text.contains("NFR54"));
    assert!(text.contains("NFR14-crates") && text.contains("bitloom"));
}
