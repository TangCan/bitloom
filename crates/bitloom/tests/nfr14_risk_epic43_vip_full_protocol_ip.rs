//! ATDD / guardrail: Epic 43 NFR14 risk record for VIP / full-protocol
//! first-class IP (Story 43.1 / AD-28 / FR98). Red if file missing
//! or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic43_vip_full_protocol_ip_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic43-vip-full-protocol-ip.md");
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

    // Per-class near-VIP / full-protocol required acceptance items
    for proto in ["UART", "SPI", "I2C", "AXI"] {
        assert!(
            text.contains(proto),
            "risk record must list {proto} near-VIP / full-protocol acceptance items"
        );
    }
    assert!(
        text.contains("GPIO"),
        "risk record must address optional GPIO"
    );
    assert!(
        (text.contains("全协议") || text.contains("近 VIP") || text.contains("近VIP"))
            && (text.contains("必选") || text.contains("验收")),
        "risk record must define 全协议/近 VIP required acceptance items"
    );
    // Concrete checklist markers (U1/S1/I1/A1 style or equivalent section headers)
    assert!(
        (text.contains("U1") || text.contains("TX") && text.contains("RX"))
            && (text.contains("S1") || text.contains("CPOL") || text.contains("CPHA"))
            && (text.contains("I1") || text.contains("ACK"))
            && (text.contains("A1") || text.contains("wstrb") || text.contains("AXI4-Lite")),
        "risk record must nail concrete per-class acceptance bullets"
    );

    // Forbidden: deepen only one class and claim FR98 full green without cut-down contract
    assert!(
        (text.contains("一类") || text.contains("单类") || text.contains("只加深"))
            && (text.contains("FR98") || text.contains("全绿"))
            && (text.contains("不得") || text.contains("禁止"))
            && (text.contains("裁剪") || text.contains("合同")),
        "risk record must forbid claiming FR98 full green from one-class deepen unless explicit cut-down contract"
    );

    // Forbidden: claim VIP without ATDD
    assert!(
        text.contains("ATDD")
            && (text.contains("VIP") || text.contains("近 VIP") || text.contains("全协议"))
            && (text.contains("不得") || text.contains("禁止")),
        "risk record must forbid claiming VIP without ATDD"
    );

    // Gate: 43.2–43.5 must not be ready without this record
    assert!(
        text.contains("43.2")
            && text.contains("43.5")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 43.2–43.5 from ready without this record"
    );

    assert!(text.contains("FR98"), "risk record must name FR98");
    assert!(
        text.contains("Epic 43") || text.contains("Epic43"),
        "risk record must name Epic 43"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR40") && text.contains("负责人"),
        "risk record must assign NFR40 ownership alongside NFR14"
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
    assert!(
        text.contains("Epic 40") || text.contains("FR94"),
        "risk record must acknowledge Epic 40 / FR94 gate already closed"
    );
    assert!(
        text.contains("FR82") && (text.contains("FR89") || text.contains("UartTx")),
        "risk record must contrast FR82/FR89 baselines with FR98"
    );
}
