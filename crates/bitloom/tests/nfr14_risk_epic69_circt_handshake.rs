//! ATDD Epic 69 NFR14 FR129 (Story 69.1).

use std::fs;
use std::path::PathBuf;

#[test]
fn nfr14_epic69_circt_handshake() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let text = fs::read_to_string(
        root.join("_agile-output/implementation-artifacts/nfr14-risk-epic69-circt-handshake.md"),
    )
    .unwrap();
    assert!(
        text.contains("(a)")
            && text.contains("(b)")
            && text.contains("(c)")
            && text.contains("(d)")
    );
    assert!(text.contains("FR129") && text.contains("C1") && text.contains("C4"));
    assert!(text.contains("CIRCT") && text.contains("handshake"));
    assert!(text.contains("AD-25") && (text.contains("修订") || text.contains("revise")));
    assert!(text.contains("FR121") && (text.contains("不得仅") || text.contains("alone")));
    assert!(text.contains("FR95") && text.contains("FR110"));
    assert!(text.contains("docs-only") || text.contains("不得 docs"));
    assert!(text.contains("69.2") && text.contains("69.3") && text.contains("ready"));
    assert!(text.contains("Richard") && text.contains("NFR14") && text.contains("NFR54"));
    assert!(text.contains("NFR14-crates") && text.contains("bitloom"));
    assert!(text.contains("elastic") || text.contains("弹性"));
}
