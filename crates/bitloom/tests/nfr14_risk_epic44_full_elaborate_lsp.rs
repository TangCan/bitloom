//! ATDD / guardrail: Epic 44 NFR14 risk record for keystroke full-elaborate
//! Bitloom LSP (Story 44.1 / AD-28 / FR99). Red if file missing or required
//! sections absent.
//!
//! ```text
//! cargo test -p bitloom --test nfr14_risk_epic44_full_elaborate_lsp
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
fn nfr14_risk_epic44_full_elaborate_lsp_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic44-full-elaborate-lsp.md");
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

    // Keystroke full-elaborate performance / scope bounds
    assert!(
        (text.contains("按键")
            || text.contains("全 elaborate")
            || text.contains("全设计 elaborate"))
            && (text.contains("性能")
                || text.contains("范围")
                || text.contains("边界")
                || text.contains("预算")),
        "risk record must nail 按键全 elaborate performance/scope bounds"
    );
    assert!(
        text.contains("P1")
            && text.contains("P3")
            && (text.contains("超时") || text.contains("2s") || text.contains("预算")),
        "risk record must define concrete performance bound markers (P1–P6 style)"
    );

    // Division of labor with host rust-analyzer
    assert!(
        (text.contains("rust-analyzer") || text.contains("FR90"))
            && (text.contains("分工") || text.contains("宿主")),
        "risk record must define division of labor with host rust-analyzer / FR90"
    );

    // Forbidden: half-baked language-server binary as done
    assert!(
        (text.contains("半成品") || text.contains("half"))
            && (text.contains("language-server")
                || text.contains("language server")
                || text.contains("二进制"))
            && (text.contains("不得") || text.contains("禁止")),
        "risk record must forbid shipping a half-baked language-server binary as done"
    );

    // Forbidden: HTML viz counts as LSP
    assert!(
        (text.contains("HTML") || text.contains("可视化"))
            && (text.contains("LSP") || text.contains("计入"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("≠")),
        "risk record must forbid counting HTML visualization as LSP"
    );

    // Forbidden: close FR99 on shallow diagnostics only
    assert!(
        (text.contains("浅层") || text.contains("shallow"))
            && (text.contains("FR99") || text.contains("诊断"))
            && (text.contains("不得") || text.contains("禁止")),
        "risk record must forbid closing FR99 on shallow diagnostics only"
    );

    // Gate: 44.2–44.4 must not be ready without this record
    assert!(
        text.contains("44.2")
            && text.contains("44.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 44.2–44.4 from ready without this record"
    );

    assert!(text.contains("FR99"), "risk record must name FR99");
    assert!(
        text.contains("Epic 44") || text.contains("Epic44"),
        "risk record must name Epic 44"
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
        text.contains("FR91")
            && (text.contains("Path B") || text.contains("分支 B") || text.contains("defer")),
        "risk record must contrast FR91 Path B defer with FR99"
    );
}
