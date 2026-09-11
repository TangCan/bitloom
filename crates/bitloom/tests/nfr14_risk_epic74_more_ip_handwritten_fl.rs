//! ATDD / guardrail: Epic 74 NFR14 risk record for FR135
//! more IP handwritten FL beyond Gpio (Story 74.1 / NFR56–59).
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
fn nfr14_risk_epic74_more_ip_handwritten_fl_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic74-more-ip-handwritten-fl.md");
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

    // FR135 / Epic 74 identity
    assert!(text.contains("FR135"), "risk record must cite FR135");
    assert!(
        text.contains("Epic 74") || text.contains("Epic74"),
        "risk record must name Epic 74"
    );

    // Protocol list beyond Gpio (≥1 acceptable)
    assert!(
        text.contains("Gpio")
            && (text.contains("超") || text.contains("beyond") || text.contains("之外")),
        "risk record must frame protocols beyond Gpio"
    );
    assert!(
        text.contains("协议")
            && (text.contains("清单")
                || text.contains("选定")
                || text.contains("Uart")
                || text.contains("UART")),
        "risk record must nail added protocol list (≥1)"
    );
    assert!(
        text.contains("UartTx") || text.contains("UART") || text.contains("Uart"),
        "risk record must name ≥1 acceptable protocol beyond Gpio"
    );

    // FL≡tick (or equiv) acceptance predicates
    assert!(
        (text.contains("FL") || text.contains("手写"))
            && (text.contains("tick") || text.contains("≡") || text.contains("等价")),
        "risk record must nail FL≡tick (or equiv) acceptance predicates"
    );
    assert!(
        text.contains("F1") && text.contains("F2") && text.contains("F3"),
        "risk record must nail F1–F3 acceptance predicates"
    );

    // GeneratedFunctional / handwritten boundary
    assert!(
        text.contains("GeneratedFunctional")
            && (text.contains("手写") || text.contains("handwritten") || text.contains("边界")),
        "risk record must nail GeneratedFunctional/handwritten boundary"
    );

    // Forbid FR92 alone
    assert!(
        text.contains("FR92")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must forbid closing on FR92 alone"
    );

    // Forbid FR100 F1-(i) alone
    assert!(
        text.contains("FR100")
            && (text.contains("F1-(i)") || text.contains("F1-(i)") || text.contains("F1"))
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must forbid closing on FR100 F1-(i) alone"
    );

    // Forbid FR103/112/119 alone
    assert!(
        text.contains("FR103") && text.contains("FR112") && text.contains("FR119"),
        "risk record must cite FR103/112/119 bans"
    );

    // Forbid FR126 Gpio alone
    assert!(
        text.contains("FR126")
            && text.contains("Gpio")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must forbid closing on FR126 Gpio alone"
    );

    // Forbid docs-only
    assert!(
        text.contains("docs-only") || (text.contains("docs") && text.contains("不得")),
        "risk record must forbid docs-only close"
    );

    // Gate: 74.2–74.3 must not be ready without this record
    assert!(
        text.contains("74.2")
            && text.contains("74.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 74.2–74.3 from ready without this record"
    );

    // Soft order vs Epic 75/78 ip/
    assert!(
        (text.contains("75") || text.contains("78"))
            && text.contains("软序")
            && (text.contains("ip/") || text.contains("`ip/`") || text.contains("ip")),
        "risk record must document soft-order notes vs Epic 75/78 ip/"
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
