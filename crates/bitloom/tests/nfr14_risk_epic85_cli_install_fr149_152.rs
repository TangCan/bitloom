//! ATDD / guardrail: Epic 85 NFR14 risk record for CLI install path
//! FR149–152 (Story 85.1 / AD-28 / NFR64–67).
//! Red if file missing, epic already closed before 85.6, or 85.2+ ready before 85.1 done.

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
fn nfr14_risk_epic85_cli_install_fr149_152_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic85-cli-install-fr149-152.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

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

    let status_open = text.contains("状态")
        && (text.contains("open") || text.contains("in-progress") || text.contains("进行中"))
        && !text.contains("closed — Story 85.6")
        && !text.contains("closed — Epic 85");
    let status_closed = text.contains("closed — Story 85.6")
        || (text.contains("closed") && text.contains("Epic 85") && text.contains("FR149"));
    assert!(
        status_open || status_closed,
        "Story 85.1 risk record status must be open/in-progress or closed after Story 85.6"
    );

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint-status.yaml");
    let epic85_done = sprint.contains("epic-85: done") || sprint.contains("epic-85:done");
    if !epic85_done {
        assert!(
            status_open,
            "before epic-85 done, risk record must remain open/in-progress"
        );
    }

    assert!(text.contains("FR149"), "risk record must summarize FR149");
    assert!(text.contains("FR150"), "risk record must summarize FR150");
    assert!(text.contains("FR151"), "risk record must summarize FR151");
    assert!(text.contains("FR152"), "risk record must summarize FR152");

    assert!(
        text.contains("bitloom-firrtl")
            && (text.contains("rename") || text.contains("可发布") || text.contains("publish")),
        "risk record must cover bitloom-firrtl publishability"
    );
    assert!(
        text.contains("bitloom-viz")
            && (text.contains("rename") || text.contains("可发布") || text.contains("publish")),
        "risk record must cover bitloom-viz publishability"
    );
    assert!(
        (text.contains("FR152") || text.contains("lsp"))
            && (text.contains("(b)") || text.contains("publish=false") || text.contains("不挡")),
        "risk record must document FR152(b) lsp default"
    );

    assert!(
        (text.contains("rhdl-bits") || text.contains("`rhdl`"))
            && (text.contains("不得") || text.contains("禁止"))
            && (text.contains("publish") || text.contains("Publish") || text.contains("发布")),
        "risk record must forbid publishing rhdl / rhdl-bits"
    );
    assert!(
        text.contains("bitloom-prelude")
            && (text.contains("不得") || text.contains("禁止") || text.contains("AD-6")),
        "risk record must forbid rewriting design→prelude boundary"
    );
    assert!(
        text.contains("FR142")
            && (text.contains("不得") || text.contains("禁止") || text.contains("扩大")),
        "risk record must forbid silently expanding FR142"
    );
    assert!(
        (text.contains("path-only")
            || text.contains("path only")
            || text.contains("path-only 依赖"))
            && (text.contains("不得") || text.contains("禁止") || text.contains("挡")),
        "risk record must forbid leaving path-only deps that block bitloom publish"
    );

    assert!(
        text.contains("85.2")
            && text.contains("85.3")
            && text.contains("85.6")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate stories 85.2–85.6 from ready without this record"
    );

    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "risk record must name an owner under NFR14"
    );
    assert!(
        text.contains("NFR64") && text.contains("NFR66") && text.contains("NFR67"),
        "risk record must cite NFR64–67"
    );
    assert!(
        text.contains("NFR14-crates"),
        "risk record must disambiguate NFR14-crates from NFR14 gate"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom"),
        "risk record must cite Bitloom brand"
    );
    assert!(
        (text.contains("firrtl") && text.contains("viz") && text.contains("lsp"))
            && (text.contains("软序") || text.contains("→")),
        "risk record must document soft order firrtl → viz → lsp → CLI"
    );

    let s85_1 = yaml_value_for_key(&sprint, "85-1-epic-85-nfr14-风险记录")
        .expect("85-1 key in sprint-status");
    let later = [
        "85-2-bitloom-firrtl-可发布-fr149",
        "85-3-bitloom-viz-可发布-fr150",
        "85-4-bitloom-lsp-发布策略-b-fr152",
        "85-5-发布-bitloom-cli-1-0-0-fr151",
        "85-6-fr149-152-收口与文档指针",
    ];
    if s85_1 != "done" {
        for key in later {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 85-1 is done (got {status})"
            );
        }
    }

    assert!(
        sprint.contains("epic-84: done") || sprint.contains("epic-84:done"),
        "Epic 84 must be done before Epic 85 NFR14"
    );
    let epic85 = yaml_value_for_key(&sprint, "epic-85").unwrap_or_default();
    assert!(
        epic85 == "backlog" || epic85 == "in-progress" || epic85 == "done",
        "epic-85 status must be backlog|in-progress|done (got {epic85})"
    );
    if !epic85_done {
        assert_eq!(
            yaml_value_for_key(&sprint, "epic-86").as_deref(),
            Some("backlog"),
            "epic-86 stays backlog until Epic 85 closes"
        );
    }
}
