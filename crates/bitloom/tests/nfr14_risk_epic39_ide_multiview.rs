//! ATDD / guardrail: Epic 39 NFR14 risk record for host IDE / multiview
//! (Story 39.1 / AD-28 / FR90–92 / FR91 Path B / NFR39). Red if file missing,
//! branch B unset, or required sections absent.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic39_ide_multiview_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic39-ide-multiview.md");
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

    // FR91 Path B selected (explicit defer); not A as delivered
    assert!(
        (text.contains("显式 defer") || text.contains("Path B") || text.contains("分支 B"))
            && (text.contains("选用")
                || text.contains("选定：分支 B")
                || text.contains("钉死为 B")
                || text.contains("选定：分支 B")),
        "risk record must explicitly select FR91 branch B (explicit defer Bitloom LSP)"
    );
    assert!(
        text.contains("浅层")
            && (text.contains("不选用")
                || text.contains("未选")
                || text.contains("不交付")
                || text.contains("不得")),
        "risk record must state branch A (shallow LSP MVP) is not delivered this epic"
    );

    // Forbidden: SystemC TLM-2.0
    assert!(
        (text.contains("不得") || text.contains("禁止"))
            && (text.contains("SystemC") || text.contains("TLM-2.0") || text.contains("TLM")),
        "risk record must forbid SystemC TLM-2.0 product claims"
    );

    // Forbidden: automatic FL≡RTL formal proof
    assert!(
        (text.contains("不得") || text.contains("禁止"))
            && (text.contains("FL≡RTL")
                || text.contains("形式证明")
                || text.contains("形式等价")
                || text.contains("TLM≡CA")),
        "risk record must forbid automatic FL≡RTL / formal-equivalence claims"
    );

    // Forbidden: hierarchy HTML counting as LSP done
    assert!(
        (text.contains("不得") || text.contains("禁止") || text.contains("不计入"))
            && (text.contains("HTML") || text.contains("层次"))
            && (text.contains("LSP") || text.contains("冒充")),
        "risk record must forbid treating hierarchy HTML as LSP completion"
    );

    // Branch B: no undocumented half-baked LSP binary
    assert!(
        (text.contains("半成品") || text.contains("二进制"))
            && (text.contains("不得") || text.contains("禁止"))
            && text.contains("LSP"),
        "risk record must forbid undocumented half-baked LSP binary under branch B"
    );

    // Gate: 39.2–39.4 must not be ready without this record
    assert!(
        text.contains("39.2")
            && text.contains("39.3")
            && text.contains("39.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 39.2–39.4 from ready without this record"
    );

    assert!(text.contains("FR90"), "risk record must cite FR90");
    assert!(text.contains("FR91"), "risk record must cite FR91");
    assert!(text.contains("FR92"), "risk record must cite FR92");
    assert!(text.contains("NFR39"), "risk record must cite NFR39");
    assert!(
        text.contains("AD-5") || text.contains("AD-5"),
        "risk record must cite AD-5 (dual-model / no SystemC TLM contract)"
    );
    assert!(
        text.contains("Epic 39") || text.contains("Epic39"),
        "risk record must name Epic 39"
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
    assert!(
        text.contains("rust-analyzer") || text.contains("宿主"),
        "risk record must mention host IDE / rust-analyzer path (FR90)"
    );
}
