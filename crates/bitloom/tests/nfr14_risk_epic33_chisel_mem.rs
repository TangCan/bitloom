//! ATDD / guardrail: Epic 33 NFR14 risk record for Mem→Chisel depth
//! (Story 33.1 / AD-28 / AD-27 / FR81 / NFR12 / FR71). Red if file missing
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
fn nfr14_risk_epic33_chisel_mem_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic33-chisel-mem.md");
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

    // NFR12 pinned Chisel/firtool pair
    assert!(
        text.contains("NFR12")
            && (text.contains("7.14.0") || text.contains("钉死"))
            && (text.contains("1.155.0") || text.contains("firtool") || text.contains("Chisel")),
        "risk record must document NFR12 pinned Chisel/firtool pair"
    );

    // E0901 status quo on Mem / emit_chisel
    assert!(
        text.contains("E0901")
            && (text.contains("Mem") || text.contains("MemDecl") || text.contains("emit_chisel")),
        "risk record must document E0901 status quo on Mem/emit_chisel"
    );

    // Tradeoffs: supported Mem subset vs permanent non-goal + alt acceptance
    assert!(
        (text.contains("支持") && text.contains("子集"))
            && (text.contains("永久非目标") || text.contains("非目标"))
            && (text.contains("利") || text.contains("弊") || text.contains("路径")),
        "risk record must cover tradeoffs of supported Mem subset vs permanent non-goal"
    );
    assert!(
        text.contains("替代验收") || text.contains("替代"),
        "risk record must mention alternative acceptance for non-goal path"
    );

    // Forbidden: must not delete E0901 to fake support without decision
    assert!(
        text.contains("E0901")
            && (text.contains("删除") || text.contains("删") || text.contains("绕过"))
            && (text.contains("不得") || text.contains("禁止"))
            && (text.contains("决策") || text.contains("未做") || text.contains("冒充")),
        "risk record must forbid deleting E0901 to fake support without decision"
    );

    // Forbidden: must not break FR71
    assert!(
        text.contains("FR71")
            && (text.contains("不得") || text.contains("禁止") || text.contains("破坏"))
            && (text.contains("JVM")
                || text.contains("chisel-fr28-jvm")
                || text.contains("fr28-chisel-jvm")
                || text.contains("破坏")),
        "risk record must forbid breaking FR71 JVM gate"
    );

    // Gate: 33.2–33.4 must not be ready without this record
    assert!(
        text.contains("33.2")
            && text.contains("33.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 33.2–33.4 from ready without this record"
    );

    assert!(text.contains("FR81"), "risk record must cover FR81");
    assert!(
        text.contains("AD-27"),
        "risk record must cite AD-27 Chisel product interop"
    );
    assert!(
        text.contains("NFR37"),
        "risk record must cite NFR37 (planning done ≠ depth done)"
    );
    assert!(
        text.contains("Epic 33") || text.contains("Epic33"),
        "risk record must name Epic 33"
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
