//! ATDD / guardrail: Epic 38 NFR14 risk record for UartTx explicit deepen
//! (Story 38.1 / AD-28 / FR89 / FR82 / NFR39). Red if file missing, branch A
//! unset, or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic38_uarttx_deepen_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic38-uarttx-deepen.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    // (a)–(d) mandatory NFR14 fields
    assert!(
        text.contains("上游约束") && (text.contains("(a)") || text.contains("（a）")),
        "risk record must include labeled field (a) 上游约束"
    );
    assert!(
        text.contains("粗工期带") && (text.contains("(b)") || text.contains("（b）")),
        "risk record must include labeled field (b) 粗工期带"
    );
    assert!(
        (text.contains("禁止的静默降级") || text.contains("禁止静默降级"))
            && (text.contains("(c)") || text.contains("（c）")),
        "risk record must include labeled field (c) 禁止的静默降级清单"
    );
    assert!(
        text.contains("负责人") && (text.contains("(d)") || text.contains("（d）")),
        "risk record must include labeled field (d) 负责人"
    );

    // Branch A selected (programmable baud); not B as delivered
    assert!(
        (text.contains("可编程波特")
            || text.contains("可编程 baud")
            || text.contains("programmable"))
            && (text.contains("分支 A")
                || text.contains("分支**A**")
                || text.contains("选定：分支 A")
                || text.contains("选用")),
        "risk record must explicitly select deepen branch A (programmable baud)"
    );
    assert!(
        text.contains("RX")
            && (text.contains("不选用")
                || text.contains("未选")
                || text.contains("不得声称")
                || text.contains("非交付")),
        "risk record must state branch B (minimal RX) is not delivered"
    );

    // FR82 boundary contrast
    assert!(
        text.contains("FR82")
            && (text.contains("baud") || text.contains("波特") || text.contains("clk"))
            && (text.contains("8N1") || text.contains("基线")),
        "risk record must contrast FR82 UartTx baseline (8N1 / baud=clk)"
    );
    assert!(
        text.contains("UartTx") || text.contains("uart"),
        "risk record must name UartTx as deepen target"
    );

    // Forbidden: full-duplex / full programmable / VIP
    assert!(
        (text.contains("不得") || text.contains("禁止"))
            && (text.contains("全双工") || text.contains("VIP") || text.contains("全协议"))
            && (text.contains("VIP") || text.contains("全特性") || text.contains("可编程全")),
        "risk record must forbid full-duplex / full-feature / VIP claims"
    );

    // Forbidden: silent expand to SPI/I2C/AXI
    assert!(
        (text.contains("不得") || text.contains("禁止"))
            && text.contains("SPI")
            && text.contains("I2C")
            && text.contains("AXI"),
        "risk record must forbid silent expansion to SPI/I2C/AXI"
    );

    // Gate: 38.2–38.3 must not be ready without this record
    assert!(
        text.contains("38.2")
            && text.contains("38.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 38.2–38.3 from ready without this record"
    );

    assert!(text.contains("FR89"), "risk record must cite FR89");
    assert!(text.contains("NFR39"), "risk record must cite NFR39");
    assert!(
        text.contains("Epic 38") || text.contains("Epic38"),
        "risk record must name Epic 38"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR39") && text.contains("负责人"),
        "risk record must assign NFR39 ownership alongside NFR14"
    );
    assert!(
        text.contains("AD-28") || text.contains("NFR14"),
        "risk record must cite NFR14/AD-28 gate"
    );
    assert!(
        text.contains("NFR14-crates"),
        "risk record must disambiguate NFR14-crates from NFR14 gate"
    );
    assert!(
        text.contains("bitloom-prelude") || text.contains("bitloom_prelude"),
        "risk record must cite bitloom-prelude design dependency boundary"
    );
}
