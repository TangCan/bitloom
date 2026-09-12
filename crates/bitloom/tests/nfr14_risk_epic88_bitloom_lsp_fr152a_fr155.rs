//! ATDD / guardrail: Epic 88 NFR14 risk record for FR152(a)/FR155
//! (Story 88.1 / AD-28 / NFR68–72).

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
fn nfr14_risk_epic88_bitloom_lsp_fr152a_fr155_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic88-bitloom-lsp-fr152a-fr155.md",
    );
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(text.contains("上游约束") && (text.contains("(a)") || text.contains("（a）")));
    assert!(text.contains("粗工期带") && (text.contains("(b)") || text.contains("（b）")));
    assert!(
        (text.contains("禁止的静默降级") || text.contains("禁止静默降级"))
            && (text.contains("(c)") || text.contains("（c）"))
    );
    assert!(text.contains("负责人") && (text.contains("(d)") || text.contains("（d）")));

    let status_open = text.contains("状态")
        && (text.contains("open") || text.contains("in-progress") || text.contains("进行中"))
        && !text.contains("closed — Story 88.4");
    let status_closed = text.contains("closed — Story 88.4")
        || (text.contains("closed") && text.contains("Epic 88") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let epic88_done = sprint.contains("epic-88: done") || sprint.contains("epic-88:done");
    if !epic88_done {
        assert!(
            status_open,
            "before epic-88 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR68")
            && (text.contains("Phase 12") || text.contains("Phase 18") || text.contains("FR153"))
            && (text.contains("不得") || text.contains("保留") || text.contains("仍有效")),
        "risk record must state NFR68 Phase isolation"
    );
    assert!(
        text.contains("FR155") && text.contains("FR152") && text.contains("(a)"),
        "risk record must cover FR155 / FR152(a)"
    );
    assert!(
        text.contains("live") && text.contains("cargo publish"),
        "risk record must require live cargo publish"
    );
    assert!(
        text.contains("FR151") && (text.contains("不得破坏") || text.contains("不得")),
        "risk record must protect FR151 CLI path"
    );
    assert!(
        text.contains("88.2")
            && text.contains("88.3")
            && text.contains("88.4")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "risk record must gate 88.2–88.4 behind this NFR14"
    );
    assert!(
        text.contains("FR142") && (text.contains("扩大") || text.contains("静默")),
        "risk record must forbid silent FR142 expand"
    );
    assert!(
        (text.contains("rhdl-bits") || text.contains("`rhdl`"))
            && (text.contains("禁止") || text.contains("不得")),
        "risk record must forbid rhdl / rhdl-bits publish"
    );
    assert!(
        text.contains("Richard") && text.contains("NFR14"),
        "risk record must name NFR14 owner"
    );
    assert!(
        text.contains("NFR14-crates"),
        "risk record must not impersonate NFR14-crates FCFS"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom"),
        "risk record must keep Bitloom brand"
    );
    assert!(
        text.contains("bitloom-prelude") || text.contains("AD-6") || text.contains("AD-2"),
        "risk record must cite AD-2/AD-6 or prelude boundary"
    );

    let s88_1 = yaml_value_for_key(&sprint, "88-1-epic-88-nfr14-风险记录").expect("88-1");
    let later = [
        "88-2-bitloom-lsp-可发布化与政策-a-fr155",
        "88-3-live-发布-bitloom-lsp-fr155",
        "88-4-fr155-收口与文档指针",
    ];
    if s88_1 != "done" {
        for key in later {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 88-1 done (got {status})"
            );
        }
    }
    assert!(
        sprint.contains("epic-87: done") || sprint.contains("epic-87:done"),
        "Epic 87 must be done before Epic 88 NFR14"
    );
}
