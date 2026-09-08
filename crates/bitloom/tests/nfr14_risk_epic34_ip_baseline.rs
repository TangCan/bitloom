//! ATDD / guardrail: Epic 34 NFR14 risk record for first-class IP synthesizable
//! baseline without closures (Story 34.1 / AD-28 / FR82). Red if file missing or
//! required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic34_ip_baseline_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic34-ip-baseline.md");
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

    // Five IP classes in scope
    for class in ["UART", "SPI", "I2C", "FIFO", "AXI"] {
        assert!(
            text.contains(class),
            "risk record must name IP class {class}"
        );
    }

    // Difference from historical Epic 22 minimal / stub contract (NFR37)
    assert!(
        (text.contains("Epic 22") || text.contains("历史"))
            && (text.contains("stub") || text.contains("Stub") || text.contains("最小合同"))
            && (text.contains("非 stub") || text.contains("深度") || text.contains("FR82")),
        "risk record must contrast Epic 34 depth vs historical Epic 22 stub/minimal contract"
    );
    assert!(
        text.contains("NFR37"),
        "risk record must cite NFR37 (planning done ≠ depth done)"
    );

    // Dependency order vs Epic 29 (34 before 29.3)
    assert!(
        text.contains("Epic 29")
            && text.contains("29.3")
            && (text.contains("先于") || text.contains("优先") || text.contains("基线")),
        "risk record must note Epic 34 baseline precedes Epic 29.3"
    );

    // Forbidden: must not only rename stubs
    assert!(
        (text.contains("重命名") || text.contains("改名") || text.contains("rename"))
            && (text.contains("stub") || text.contains("Stub"))
            && (text.contains("不得") || text.contains("禁止")),
        "risk record must forbid claiming FR82 done by only renaming stubs"
    );

    // Gate: 34.2–34.4 must not be ready without this record
    assert!(
        text.contains("34.2")
            && text.contains("34.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 34.2–34.4 from ready without this record"
    );

    assert!(text.contains("FR82"), "risk record must cover FR82");
    assert!(
        text.contains("FR37") && text.contains("FR48"),
        "risk record must cite inherited FR37/FR48"
    );
    assert!(
        text.contains("Epic 34") || text.contains("Epic34"),
        "risk record must name Epic 34"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("AD-28") || text.contains("NFR14"),
        "risk record must cite NFR14/AD-28 gate"
    );
    assert!(
        text.contains("NFR14-crates"),
        "risk record must disambiguate NFR14-crates from NFR14 gate"
    );
}
