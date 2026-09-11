//! ATDD / guardrail: Epic 76 NFR14 risk record for FR137
//! full external CIRCT compile and/or sim gate beyond FR129 C1–C4 (Story 76.1 / NFR56–59).
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
fn nfr14_risk_epic76_external_circt_compile_sim_gate_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic76-external-circt-compile-sim-gate.md",
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

    // FR137 / Epic 76 identity
    assert!(text.contains("FR137"), "risk record must cite FR137");
    assert!(
        text.contains("Epic 76") || text.contains("Epic76"),
        "risk record must name Epic 76"
    );

    // External CIRCT tool version / release channel
    assert!(
        text.contains("CIRCT")
            && (text.contains("firtool") || text.contains("firtool-"))
            && (text.contains("1.155") || text.contains("版本") || text.contains("渠道")),
        "risk record must nail external CIRCT tool version/release channel"
    );

    // Compile and/or sim gate shape: CI required job and/or documented pinned path
    assert!(
        (text.contains("编译")
            || text.contains("compile")
            || text.contains("仿真")
            || text.contains("sim"))
            && (text.contains("CI")
                || text.contains("required")
                || text.contains("路径")
                || text.contains("job")),
        "risk record must nail compile and/or sim gate shape (CI required and/or pinned path)"
    );

    // Missing-tool non-zero readable failure semantics
    assert!(
        (text.contains("缺工具") || text.contains("missing") || text.contains("缺"))
            && (text.contains("非零") || text.contains("non-zero") || text.contains("nonzero"))
            && (text.contains("可读") || text.contains("readable") || text.contains("失败")),
        "risk record must nail missing-tool non-zero readable failure semantics"
    );

    // Acceptance predicates E1–E4
    assert!(
        text.contains("E1") && text.contains("E2") && text.contains("E3") && text.contains("E4"),
        "risk record must nail E1–E4 acceptance predicates"
    );

    // FR129 C1–C4 boundary
    assert!(
        text.contains("FR129")
            && text.contains("C1")
            && text.contains("C4")
            && (text.contains("边界") || text.contains("alone") || text.contains("Handshake")),
        "risk record must nail FR129 C1–C4 boundary vs FR137"
    );

    // Forbid FR95/96 alone
    assert!(
        text.contains("FR95")
            && text.contains("FR96")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must forbid closing on FR95/96 alone"
    );

    // Forbid FR110 alone
    assert!(
        text.contains("FR110")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must forbid closing on FR110 alone"
    );

    // Forbid FR121 ready/valid alone
    assert!(
        text.contains("FR121")
            && (text.contains("ready") || text.contains("valid") || text.contains("Handshake"))
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must forbid closing on FR121 ready/valid alone"
    );

    // Forbid FR129 C1–C4 alone
    assert!(
        text.contains("FR129")
            && text.contains("C1")
            && text.contains("C4")
            && (text.contains("alone") || text.contains("仅") || text.contains("不得")),
        "risk record must forbid closing on FR129 C1–C4 alone"
    );

    // Forbid docs-only
    assert!(
        text.contains("docs-only") || (text.contains("docs") && text.contains("不得")),
        "risk record must forbid docs-only close"
    );

    // Forbid continue-on-error silent skip
    assert!(
        text.contains("continue-on-error")
            || (text.contains("静默") && (text.contains("跳过") || text.contains("skip"))),
        "risk record must forbid continue-on-error silent skip"
    );

    // Gate: 76.2–76.3 must not be ready without this record
    assert!(
        text.contains("76.2")
            && text.contains("76.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 76.2–76.3 from ready without this record"
    );

    // Owner including NFR58 ops sync
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR58")
            && (text.contains("运维") || text.contains("同步") || text.contains("ops")),
        "risk record must assign NFR58 ops-sync ownership"
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
