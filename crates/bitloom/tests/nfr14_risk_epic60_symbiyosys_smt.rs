//! ATDD: Epic 60 NFR14 for FR119 SymbiYosys/SMT formal path (Story 60.1).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn nfr14_risk_epic60_symbiyosys_smt_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic60-symbiyosys-smt.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && text.contains("(a)"));
    assert!(text.contains("粗工期带") && text.contains("(b)"));
    assert!(text.contains("禁止的静默降级") && text.contains("(c)"));
    assert!(text.contains("负责人") && text.contains("(d)"));
    assert!(text.contains("FR119"));

    // Selected binding: SymbiYosys / sby (at least class A)
    assert!(
        text.contains("SymbiYosys") && (text.contains("sby") || text.contains("`sby`")),
        "must nail SymbiYosys / sby binding"
    );
    assert!(
        text.contains("选定") && (text.contains("(A)") || text.contains("绑定")),
        "must select binding class"
    );

    // Tool version / detect obligations
    assert!(
        text.contains("版本") || text.contains("command -v") || text.contains("sby --version"),
        "must document tool version / detection obligations"
    );

    // assume / assert obligations
    assert!(
        text.contains("assume") && text.contains("assert"),
        "must nail assume/assert obligations"
    );

    // Fixture + repro
    assert!(
        text.contains("夹具")
            && (text.contains("可复现") || text.contains("CI") || text.contains("本地")),
        "must nail fixtures and CI/local repro steps"
    );

    // Forbidden closes
    assert!(
        text.contains("FR92")
            && (text.contains("不得") || text.contains("alone") || text.contains("≠")),
        "must ban FR92 scoreboard alone"
    );
    assert!(
        text.contains("FR100")
            && (text.contains("F1-(i)") || text.contains("有界"))
            && (text.contains("不得") || text.contains("alone") || text.contains("≠")),
        "must ban FR100 F1-(i) alone"
    );
    assert!(
        (text.contains("FR112") || text.contains("MemRead"))
            && (text.contains("不得") || text.contains("alone") || text.contains("≠")),
        "must ban FR112-B MemRead≡tick alone"
    );
    assert!(
        text.contains("docs-only") || text.contains("仅改文档"),
        "must ban docs-only close explicitly"
    );
    assert!(
        text.contains("FR107")
            && (text.contains("不得") || text.contains("≠") || text.contains("冒充")),
        "must forbid FR107 SystemC AT confusion"
    );
    assert!(
        (text.contains("FR85") || text.contains("Verilator") || text.contains("check_sva_text"))
            && (text.contains("不得") || text.contains("≠") || text.contains("alone")),
        "must ban FR85 Verilator/toy alone as FR119 close"
    );
    assert!(
        text.contains("Fork")
            || text.contains("formal-sby-check")
            || (text.contains("formal-sva-check") && text.contains("FR85")),
        "must relate FR119 product entry to FR85 (fork, not reuse as close)"
    );

    // Independent SMT (B) not a silent alternate close
    assert!(
        text.contains("未选") || text.contains("独立") || text.contains("(B)"),
        "must document independent SMT (B) not selected as equal close"
    );

    // Gate + owners
    assert!(
        text.contains("60.2")
            && text.contains("60.3")
            && text.contains("ready")
            && text.contains("不得")
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR50"));
    assert!(text.contains("NFR51"));
    assert!(text.contains("NFR48"));
    assert!(text.contains("NFR14-crates"));

    // Missing tool must not silent-succeed
    assert!(
        (text.contains("silent") || text.contains("缺失"))
            && (text.contains("不得") || text.contains("可读")),
        "must forbid silent success when tool missing"
    );

    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(
        text.contains("bitloom-prelude") || text.contains("bitloom_prelude"),
        "must cite bitloom-prelude design dependency boundary"
    );

    // Branch C stays deferred (honesty)
    assert!(
        text.contains("分支 C") || text.contains("手写 FL"),
        "must acknowledge branch C / more IP FL deferred boundary"
    );
}
