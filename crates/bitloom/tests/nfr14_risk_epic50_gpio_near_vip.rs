//! ATDD: Epic 50 NFR14 for FR108 GPIO near-VIP (Story 50.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic50_gpio_near_vip_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic50-gpio-near-vip.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(text.contains("禁止的静默降级") && text.contains("(c)"));
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR108"));
    assert!(text.contains("GPIO") || text.contains("gpio"));
    assert!(text.contains("方向") || text.contains("P1"));
    assert!(text.contains("读写") || text.contains("P2"));
    assert!(text.contains("掩码") || text.contains("P3"));
    assert!(text.contains("ATDD") || text.contains("P4"));
    assert!(text.contains("商业 VIP") && (text.contains("不得") || text.contains("禁止")));
    assert!(text.contains("ATDD") && (text.contains("不得") || text.contains("无")));
    assert!(
        text.contains("50.2")
            && text.contains("50.3")
            && text.contains("ready")
            && text.contains("不得")
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR47"));
    assert!(text.contains("ip.rs") || text.contains("体积"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("FR98") && (text.contains("仍") || text.contains("不得")));
    assert!(text.contains("bitloom-prelude") || text.contains("Bitloom"));
}
