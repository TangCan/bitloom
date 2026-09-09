//! ATDD / guardrail: Epic 35 NFR14 risk record for residual Partial depth
//! (Story 35.1 / AD-28 / FR83 / FR84 / FR85 / NFR37). Red if file missing
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
fn nfr14_risk_epic35_residual_partials_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic35-residual-partials.md");
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

    // Status quo Partial facts
    assert!(
        text.contains("Counter")
            && (text.contains("Counter-only")
                || text.contains("仅 Counter")
                || text.contains("硬编码 Counter")),
        "risk record must document C ABI Counter-only status quo"
    );
    assert!(
        text.contains("SoftF16")
            && (text.contains("host-only")
                || text.contains("host only")
                || text.contains("仅 host")),
        "risk record must document SoftF16 host-only status quo"
    );
    assert!(
        text.contains("check_sva_text")
            || (text.contains("toy") && (text.contains("SVA") || text.contains("formal"))),
        "risk record must document SVA toy-check status quo"
    );

    // FR83/84/85 each: implement vs explicit defer options
    for fr in ["FR83", "FR84", "FR85"] {
        assert!(text.contains(fr), "risk record must cover {fr}");
    }
    assert!(
        (text.contains("实现") || text.contains("implement"))
            && (text.contains("defer") || text.contains("Defer") || text.contains("显式")),
        "risk record must present implement vs explicit defer options"
    );
    // Per-FR option tables / rows (not a single shared blob)
    assert!(
        text.contains("FR83")
            && text.contains("FR84")
            && text.contains("FR85")
            && (text.matches("defer").count() >= 2
                || text.matches("Defer").count() >= 1
                || text.matches("显式").count() >= 2),
        "risk record must give implement-vs-defer options across FR83/84/85"
    );

    // LSP not this epic
    assert!(
        text.contains("LSP")
            && (text.contains("非本 epic")
                || text.contains("本 epic") && text.contains("deferred")
                || text.contains("继续 deferred")
                || text.contains("仍 deferred")),
        "risk record must state LSP is not in scope for this epic / remains deferred"
    );

    // Forbidden: must not close FR85 with toy check
    assert!(
        text.contains("FR85")
            && (text.contains("toy") || text.contains("check_sva_text") || text.contains("玩具"))
            && (text.contains("不得") || text.contains("禁止"))
            && (text.contains("关闭")
                || text.contains("关")
                || text.contains("交差")
                || text.contains("完成")),
        "risk record must forbid closing FR85 with toy check / check_sva_text"
    );

    // Forbidden: must not claim SoftF16 synthesizable if defer chosen
    assert!(
        text.contains("SoftF16")
            && (text.contains("可综合") || text.contains("synthesiz"))
            && (text.contains("不得") || text.contains("禁止"))
            && (text.contains("defer") || text.contains("Defer") || text.contains("显式")),
        "risk record must forbid claiming SoftF16 synthesizable when defer is chosen"
    );

    // Gate: 35.2–35.4 must not be ready without this record
    assert!(
        text.contains("35.2")
            && text.contains("35.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 35.2–35.4 from ready without this record"
    );

    assert!(
        text.contains("FR33") && text.contains("FR36") && text.contains("FR39"),
        "risk record must cite inherited FR33/FR36/FR39"
    );
    assert!(
        text.contains("NFR37"),
        "risk record must cite NFR37 (planning done ≠ depth done)"
    );
    assert!(
        text.contains("Epic 35") || text.contains("Epic35"),
        "risk record must name Epic 35"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR37") && text.contains("负责人"),
        "risk record must assign NFR37 ownership alongside NFR14"
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
