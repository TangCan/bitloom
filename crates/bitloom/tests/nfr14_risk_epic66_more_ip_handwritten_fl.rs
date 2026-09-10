//! ATDD Epic 66 NFR14 FR126 (Story 66.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic66_more_ip_handwritten_fl() {
    let text = fs::read_to_string(workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic66-more-ip-handwritten-fl.md",
    ))
    .unwrap();
    assert!(text.contains("(a)") && text.contains("上游约束"));
    assert!(text.contains("(b)") && text.contains("粗工期带"));
    assert!(text.contains("(c)") && text.contains("禁止"));
    assert!(text.contains("(d)") && text.contains("负责人"));
    assert!(text.contains("FR126") && text.contains("Gpio") && text.contains("GpioFunctional"));
    assert!(text.contains("F1") && text.contains("F2") && text.contains("F3"));
    assert!(
        text.contains("FR92")
            && text.contains("FR100")
            && text.contains("FR112")
            && text.contains("FR119")
    );
    assert!(
        text.contains("66.2")
            && text.contains("66.3")
            && text.contains("ready")
            && text.contains("不得")
    );
    assert!(
        text.contains("Richard")
            && text.contains("NFR14")
            && text.contains("NFR52")
            && text.contains("NFR55")
    );
    assert!(text.contains("NFR14-crates") && text.contains("bitloom-prelude"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
}
