//! ATDD Story 68.1 — Epic 68 NFR14 SoC pad (FR128).

use std::fs;
use std::path::PathBuf;

#[test]
fn nfr14_epic68_soc_pad() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let text = fs::read_to_string(
        root.join("_agile-output/implementation-artifacts/nfr14-risk-epic68-soc-pad.md"),
    )
    .unwrap();
    assert!(
        text.contains("(a)")
            && text.contains("(b)")
            && text.contains("(c)")
            && text.contains("(d)")
    );
    assert!(text.contains("FR128") && text.contains("D1") && text.contains("D4"));
    assert!(text.contains("GpioSocPad") || text.contains("双 bank") || text.contains("dual"));
    assert!(text.contains("降沿") || text.contains("fall") || text.contains("falling"));
    assert!(text.contains("CSR") || text.contains("csr"));
    assert!(text.contains("FR120") && text.contains("C1") && text.contains("不得"));
    assert!(text.contains("68.2") && text.contains("68.3"));
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("Bitloom") || text.contains("bitloom-prelude"));
}
