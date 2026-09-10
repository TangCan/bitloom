//! ATDD: Epic 61 NFR14 for FR120 commercial VIP GPIO (Story 61.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic61_commercial_vip_gpio_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic61-commercial-vip-gpio.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(text.contains("禁止的静默降级") && text.contains("(c)"));
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR120"));
    assert!(text.contains("GPIO") || text.contains("gpio"));

    // Protocol / mode list beyond FR108 P1–P4
    assert!(
        text.contains("P1") && text.contains("P4"),
        "must reference FR108 P1–P4 baseline"
    );
    assert!(
        text.contains("C1")
            && (text.contains("IRQ") || text.contains("中断") || text.contains("irq")),
        "must nail C1 IRQ mode"
    );
    assert!(
        text.contains("C2")
            && (text.contains("开漏") || text.contains("od") || text.contains("OE")),
        "must nail C2 open-drain / OE"
    );
    assert!(
        text.contains("C3")
            && (text.contains("set") || text.contains("clear") || text.contains("原子")),
        "must nail C3 atomic set/clear"
    );
    assert!(
        text.contains("C4") && text.contains("ATDD"),
        "must nail C4 ATDD depth"
    );

    // Assertion depth + elaborate/emit/tick obligations
    assert!(
        text.contains("elaborate") && text.contains("emit") && text.contains("tick"),
        "must nail elaborate/emit/tick obligations"
    );
    assert!(
        text.contains("断言") || text.contains("负向") || text.contains("边界"),
        "must nail assertion / negative depth"
    );

    // Regression boundary vs UART/SPI/I2C/AXI and FR108
    assert!(
        text.contains("UART")
            && text.contains("SPI")
            && text.contains("I2C")
            && (text.contains("AXI") || text.contains("Axi")),
        "must state regression boundary vs FR98 four-class IP"
    );
    assert!(
        text.contains("FR108") && (text.contains("回归") || text.contains("Gpio")),
        "must state FR108 Gpio regression boundary"
    );

    // ip.rs split assessment (not close condition)
    assert!(
        text.contains("ip.rs")
            && (text.contains("拆分") || text.contains("体积"))
            && (text.contains("非关闭") || (text.contains("不是") && text.contains("关闭条件"))),
        "must assess ip.rs split as non-close-condition"
    );

    // Forbidden closes
    assert!(
        text.contains("FR108")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "must ban FR108 alone"
    );
    assert!(
        text.contains("FR98")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "must ban FR98 alone"
    );
    assert!(
        ((text.contains("口头") || text.contains("无夹具"))
            || (text.contains("无") && text.contains("夹具")))
            && (text.contains("商业 VIP") || text.contains("VIP")),
        "must ban verbal commercial VIP without fixtures"
    );
    assert!(
        text.contains("docs-only") || text.contains("仅改文档"),
        "must ban docs-only close"
    );

    // Deferred honesty (NFR51)
    assert!(
        text.contains("NFR51")
            && (text.contains("deferred")
                || text.contains("非目标")
                || text.contains("未列入")
                || text.contains("全 SoC")),
        "must disclose uncovered protocols / NFR51 deferred"
    );

    // Gate + owners
    assert!(
        text.contains("61.2")
            && text.contains("61.3")
            && text.contains("ready")
            && text.contains("不得")
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR51"));
    assert!(text.contains("NFR48"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(
        text.contains("bitloom-prelude") || text.contains("bitloom_prelude"),
        "must cite bitloom-prelude design dependency boundary"
    );
}
