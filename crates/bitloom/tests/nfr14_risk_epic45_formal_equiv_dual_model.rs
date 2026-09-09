//! ATDD / guardrail: Epic 45 NFR14 risk record for formal equivalence +
//! dual-model completeness (Story 45.1 / AD-28 / FR100, FR102, FR103).
//! Red if file missing or required sections absent.
//!
//! ```text
//! cargo test -p bitloom --test nfr14_risk_epic45_formal_equiv_dual_model
//! ```

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic45_formal_equiv_dual_model_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic45-formal-equiv-dual-model.md",
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

    // Formal-equivalence product bounds: tool / proof obligation / fixture
    assert!(
        (text.contains("形式等价") || text.contains("FL≡RTL") || text.contains("FL≡"))
            && (text.contains("工具") || text.contains("F1"))
            && (text.contains("证明") || text.contains("义务") || text.contains("F2"))
            && (text.contains("夹具") || text.contains("F3")),
        "risk record must nail formal-equivalence product bounds (tool/proof/fixture)"
    );
    assert!(
        text.contains("F1") && text.contains("F2") && text.contains("F3"),
        "risk record must define concrete formal bound markers (F1–F3 style)"
    );

    // Property-macro matrix inventory
    assert!(
        (text.contains("属性") || text.contains("宏") || text.contains("矩阵"))
            && text.contains("functional_model")
            && text.contains("abstraction")
            && text.contains("functional_state"),
        "risk record must inventory property-macro matrix (functional_model/abstraction/functional_state)"
    );

    // First-class IP set covered by dual-model completeness
    assert!(
        (text.contains("双模型") || text.contains("齐全"))
            && text.contains("UART")
            && text.contains("SPI")
            && text.contains("I2C")
            && (text.contains("AXI") || text.contains("AXI4"))
            && (text.contains("FIFO") || text.contains("SyncFifo")),
        "risk record must name first-class IP set for dual-model completeness"
    );

    // Forbidden: random co-sim scoreboard alone as formal-equivalence product
    assert!(
        (text.contains("随机")
            || text.contains("记分板")
            || text.contains("scoreboard")
            || text.contains("共测"))
            && (text.contains("形式等价") || text.contains("FR100"))
            && (text.contains("不得")
                || text.contains("禁止")
                || text.contains("≠")
                || text.contains("alone")
                || text.contains("单独")),
        "risk record must forbid labeling random co-sim scoreboard alone as formal-equivalence product"
    );

    // Forbidden: close FR102/103 on template adapters only
    assert!(
        (text.contains("模板") || text.contains("adapter") || text.contains("Adapter"))
            && (text.contains("FR102") || text.contains("FR103"))
            && (text.contains("不得")
                || text.contains("禁止")
                || text.contains("不足")
                || text.contains("仅")),
        "risk record must forbid closing FR102/103 on template adapters only"
    );

    // Gate: 45.2–45.4 must not be ready without this record
    assert!(
        text.contains("45.2")
            && text.contains("45.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 45.2–45.4 from ready without this record"
    );

    assert!(
        text.contains("FR100") && text.contains("FR102") && text.contains("FR103"),
        "risk record must name FR100, FR102, and FR103"
    );
    assert!(
        text.contains("Epic 45") || text.contains("Epic45"),
        "risk record must name Epic 45"
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
        text.contains("FR92")
            && (text.contains("SharedStimulus")
                || text.contains("记分板")
                || text.contains("scoreboard")
                || text.contains("同刺激")),
        "risk record must contrast FR92 shared-stimulus scoreboard with FR100"
    );
}
