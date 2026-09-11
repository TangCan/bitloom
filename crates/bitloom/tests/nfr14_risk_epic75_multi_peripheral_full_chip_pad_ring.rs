//! ATDD / guardrail: Epic 75 NFR14 risk record for FR136
//! multi-peripheral / full-chip pad ring beyond GpioSocPad D1–D4 (Story 75.1 / NFR56–59).
//! Red if file missing or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic75_multi_peripheral_full_chip_pad_ring_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic75-multi-peripheral-full-chip-pad-ring.md",
    );
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

    // FR136 / Epic 75 identity
    assert!(text.contains("FR136"), "risk record must cite FR136");
    assert!(
        text.contains("Epic 75") || text.contains("Epic75"),
        "risk record must name Epic 75"
    );

    // Pad-ring scope: multi-peripheral set and/or full-chip ring shape
    assert!(
        (text.contains("多外设")
            || text.contains("multi-peripheral")
            || text.contains("MultiPeripheral"))
            && (text.contains("外设")
                || text.contains("UART")
                || text.contains("Uart")
                || text.contains("GPIO")),
        "risk record must nail multi-peripheral pad-ring scope"
    );
    assert!(
        (text.contains("全芯片") || text.contains("full-chip") || text.contains("FullChip"))
            && (text.contains("环")
                || text.contains("ring")
                || text.contains("bank")
                || text.contains("Bank")),
        "risk record must nail full-chip ring shape"
    );

    // Assertion / dual-check depth
    assert!(
        (text.contains("对拍")
            || text.contains("记分板")
            || text.contains("scoreboard")
            || text.contains("dual"))
            && (text.contains("断言") || text.contains("深度") || text.contains("R3")),
        "risk record must nail assertion/dual-check depth"
    );

    // Acceptance predicates R1–R4
    assert!(
        text.contains("R1") && text.contains("R2") && text.contains("R3") && text.contains("R4"),
        "risk record must nail R1–R4 acceptance predicates"
    );

    // FR128 D1–D4 boundary
    assert!(
        text.contains("FR128")
            && text.contains("D1")
            && text.contains("D4")
            && (text.contains("GpioSocPad") || text.contains("边界")),
        "risk record must nail FR128 D1–D4 boundary vs FR136"
    );

    // Forbid FR108 alone
    assert!(
        text.contains("FR108")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must forbid closing on FR108 alone"
    );

    // Forbid FR120 C1–C4 alone
    assert!(
        text.contains("FR120")
            && text.contains("C1")
            && text.contains("C4")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must forbid closing on FR120 C1–C4 alone"
    );

    // Forbid FR128 D1–D4 alone
    assert!(
        text.contains("FR128")
            && text.contains("D1")
            && text.contains("D4")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must forbid closing on FR128 D1–D4 alone"
    );

    // Forbid docs-only
    assert!(
        text.contains("docs-only") || (text.contains("docs") && text.contains("不得")),
        "risk record must forbid docs-only close"
    );

    // Gate: 75.2–75.3 must not be ready without this record
    assert!(
        text.contains("75.2")
            && text.contains("75.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 75.2–75.3 from ready without this record"
    );

    // Soft order vs Epic 74/78 ip/
    assert!(
        (text.contains("74") || text.contains("78"))
            && text.contains("软序")
            && (text.contains("ip/") || text.contains("`ip/`") || text.contains("ip")),
        "risk record must document soft-order notes vs Epic 74/78 ip/"
    );

    // Owner
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR56") && text.contains("NFR59"),
        "risk record must assign NFR56/NFR59 ownership"
    );
    assert!(
        text.contains("NFR14-crates"),
        "risk record must disambiguate NFR14-crates from NFR14 gate"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom"),
        "risk record must cite Bitloom brand"
    );
}
