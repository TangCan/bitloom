//! ATDD / guardrail: Epic 84 NFR14 risk record for Phase 18
//! CLI crates.io publishability / FR148 (Story 84.1 / AD-28 / NFR64–67).
//! Red if file missing, epic already closed, or 84.2+ ready before 84.1 done.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn yaml_value_for_key(sprint: &str, key: &str) -> Option<String> {
    let needle = format!("{key}:");
    for line in sprint.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(&needle) {
            return Some(rest.trim().to_string());
        }
    }
    None
}

#[test]
fn nfr14_risk_epic84_phase18_cli_crates_io_publish_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic84-phase18-cli-crates-io-publish.md",
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

    // Story 84.1: risk record open until Epic 84 closeout; after 84.4 may be closed.
    let status_open = text.contains("状态")
        && (text.contains("open") || text.contains("in-progress") || text.contains("进行中"))
        && !text.contains("closed — Story 84.4")
        && !text.contains("closed — Epic 84");
    let status_closed = text.contains("closed — Story 84.4")
        || (text.contains("closed") && text.contains("Epic 84") && text.contains("闸门已开"));
    assert!(
        status_open || status_closed,
        "Story 84.1 risk record status must be open/in-progress or closed after Story 84.4"
    );

    let sprint_early = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint-status.yaml");
    let epic84_done =
        sprint_early.contains("epic-84: done") || sprint_early.contains("epic-84:done");
    if !epic84_done {
        assert!(
            status_open,
            "before epic-84 done, risk record must remain open/in-progress"
        );
    }

    // NFR64: Phase 17 closed faces must not be rewritten
    assert!(
        text.contains("NFR64")
            && (text.contains("Phase 17") || text.contains("FR141") || text.contains("FR147"))
            && (text.contains("不得") || text.contains("保留") || text.contains("仍有效")),
        "risk record must state NFR64 Phase 17 isolation"
    );
    assert!(
        text.contains("已关闭")
            && (text.contains("FR141") || text.contains("FR147") || text.contains("Phase 17"))
            && (text.contains("失败") || text.contains("改写") || text.contains("不得")),
        "risk record must forbid rewriting FR141–147 closed as failed"
    );

    // FR149–153 scope summary
    assert!(text.contains("FR149"), "risk record must summarize FR149");
    assert!(text.contains("FR150"), "risk record must summarize FR150");
    assert!(text.contains("FR151"), "risk record must summarize FR151");
    assert!(text.contains("FR152"), "risk record must summarize FR152");
    assert!(text.contains("FR153"), "risk record must summarize FR153");

    // Q1–Q5 approval defaults
    assert!(
        text.contains("Q1")
            && (text.contains("bitloom-firrtl") || text.contains("firrtl"))
            && (text.contains("rename") || text.contains("可发布")),
        "risk record must document Q1 rename → bitloom-firrtl"
    );
    assert!(
        text.contains("Q2")
            && (text.contains("bitloom-viz") || text.contains("viz"))
            && (text.contains("rename") || text.contains("可发布")),
        "risk record must document Q2 rename → bitloom-viz"
    );
    assert!(
        text.contains("Q3")
            && (text.contains("FR152") || text.contains("lsp"))
            && (text.contains("(b)") || text.contains("publish=false") || text.contains("不挡")),
        "risk record must document Q3 FR152(b) lsp default"
    );
    assert!(
        text.contains("Q4") && text.contains("NFR59"),
        "risk record must document Q4 no NFR59 prerequisite"
    );
    assert!(
        text.contains("Q5")
            && (text.contains("FR148") || text.contains("MSRV") || text.contains("1.0.0")),
        "risk record must document Q5 MSRV / FR148 gate default"
    );

    // Forbidden: open 85–86 before FR148 / Epic 84 gate
    assert!(
        (text.contains("85") || text.contains("85–86") || text.contains("85-86"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("缺"))
            && (text.contains("FR148") || text.contains("Epic 84") || text.contains("ready")),
        "risk record must forbid opening Epic 85–86 before FR148 / Epic 84 gate"
    );

    // Forbidden: publish rhdl / rhdl-bits
    assert!(
        (text.contains("rhdl-bits") || text.contains("`rhdl`"))
            && (text.contains("不得") || text.contains("禁止"))
            && (text.contains("publish") || text.contains("Publish") || text.contains("发布")),
        "risk record must forbid publishing rhdl / rhdl-bits"
    );

    // Forbidden: silent swallow NFR59; LSP deepen as required
    assert!(
        text.contains("NFR59")
            && text.contains("NFR67")
            && (text.contains("静默") || text.contains("吞并")),
        "risk record must forbid silent swallow of NFR59 (NFR67)"
    );
    assert!(
        (text.contains("LSP") || text.contains("lsp"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("冒充"))
            && (text.contains("加深") || text.contains("必做") || text.contains("一等")),
        "risk record must forbid treating LSP deepen as required this batch"
    );

    // Gate: 84.2–84.4 must not be ready without this record
    assert!(
        text.contains("84.2")
            && text.contains("84.3")
            && text.contains("84.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 84.2–84.4 from ready without this record"
    );

    assert!(text.contains("FR148"), "risk record must cite FR148");
    assert!(
        text.contains("Epic 84") || text.contains("Epic84"),
        "risk record must name Epic 84"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR64") && text.contains("负责人"),
        "risk record must assign NFR64 ownership alongside NFR14"
    );
    assert!(
        (text.contains("NFR65") || text.contains("NFR66") || text.contains("NFR67"))
            && text.contains("负责人"),
        "risk record must assign NFR64–67 ownership (at least one of NFR65–67 named with owner)"
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
        text.contains("Bitloom") || text.contains("bitloom"),
        "risk record must cite Bitloom brand"
    );
    assert!(
        text.contains("84")
            && text.contains("85")
            && text.contains("86")
            && (text.contains("软序") || text.contains("→")),
        "risk record must document soft order 84 → 85 → 86"
    );

    // Sprint gate: 84.2+ must not be ready before 84.1 is done
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint-status.yaml");
    let s84_1 = yaml_value_for_key(&sprint, "84-1-epic-84-nfr14-风险记录")
        .expect("84-1 key in sprint-status");
    let later = [
        "84-2-correct-course-prd-批准-phase-18-fr148",
        "84-3-同步-readme-deferred-路线图指针-fr148",
        "84-4-ad-指针与-epic-84-收口-fr148-nfr66",
    ];
    if s84_1 != "done" {
        for key in later {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 84-1 is done (got {status})"
            );
        }
    }

    assert!(
        sprint.contains("epic-83: done") || sprint.contains("epic-83:done"),
        "Phase 17 Epic 83 must be done before Epic 84 NFR14"
    );
    assert!(
        yaml_value_for_key(&sprint, "epic-85").as_deref() == Some("backlog")
            && yaml_value_for_key(&sprint, "epic-86").as_deref() == Some("backlog"),
        "Epic 85–86 epic keys remain backlog until their NFR14 stories advance (85.1 may be ready-for-dev)"
    );
    let epic84_done_gate = sprint.contains("epic-84: done") || sprint.contains("epic-84:done");
    if !epic84_done_gate {
        for line in sprint.lines() {
            let t = line.trim();
            if (t.starts_with("85-") || t.starts_with("86-")) && t.contains("ready-for-dev") {
                panic!("Epic 85–86 stories must not be ready-for-dev before epic-84 done: {t}");
            }
        }
    }
}
