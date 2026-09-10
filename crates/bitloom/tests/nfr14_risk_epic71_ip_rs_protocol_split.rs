//! ATDD Epic 71 NFR14 FR131 (Story 71.1).

use std::fs;
use std::path::PathBuf;

#[test]
fn nfr14_epic71_ip_rs_split() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let text =
        fs::read_to_string(root.join(
            "_agile-output/implementation-artifacts/nfr14-risk-epic71-ip-rs-protocol-split.md",
        ))
        .unwrap();
    assert!(
        text.contains("(a)")
            && text.contains("(b)")
            && text.contains("(c)")
            && text.contains("(d)")
    );
    assert!(text.contains("FR131") && text.contains("P1") && text.contains("mod.rs"));
    assert!(text.contains("FR98") && text.contains("FR108") && text.contains("FR120"));
    assert!(text.contains("71.2") && text.contains("71.3") && text.contains("不得"));
    assert!(text.contains("Richard") && text.contains("NFR14") && text.contains("NFR14-crates"));
    assert!(text.contains("bitloom-prelude") && text.contains("Bitloom"));
}
