//! ATDD / guardrail: Epic 89 NFR14 risk record for FR157
//! automatic FSM label extraction (Story 89.1 / AD-28 / NFR68–72).

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
fn nfr14_risk_epic89_auto_fsm_labels_fr157_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic89-auto-fsm-labels-fr157.md");
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
        && !text.contains("closed — Story 89.3");
    let status_closed = text.contains("closed — Story 89.3")
        || (text.contains("closed") && text.contains("Epic 89") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let epic89_done = sprint.contains("epic-89: done") || sprint.contains("epic-89:done");
    if !epic89_done {
        assert!(
            status_open,
            "before epic-89 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR68")
            && (text.contains("Phase 12") || text.contains("FR109") || text.contains("FR153"))
            && (text.contains("不得") || text.contains("仍有效") || text.contains("保留")),
        "must state NFR68 isolation vs prior closes"
    );
    assert!(
        text.contains("FR157") && (text.contains("FSM") || text.contains("标签")),
        "must cover FR157 FSM labels"
    );
    assert!(
        text.contains("输入面")
            && text.contains("输出形态")
            && (text.contains("验收谓词") || text.contains("失败语义")),
        "must nail input / output / acceptance / failure"
    );
    assert!(
        text.contains("FR109")
            && (text.contains("register_fsm_states")
                || text.contains("state-visit")
                || text.contains("visit")),
        "must bound against FR109 explicit registration"
    );
    assert!(
        text.contains("NFR71")
            && (text.contains("静默") || text.contains("扩大") || text.contains("子集")),
        "must forbid silent subset expand"
    );
    assert!(
        (text.contains("波形")
            || text.contains("无标注")
            || text.contains("LCOV")
            || text.contains("GUI"))
            && (text.contains("不得") || text.contains("不在") || text.contains("禁止")),
        "must exclude waveform / unlabeled / GUI overreach"
    );
    assert!(
        text.contains("89.2")
            && text.contains("89.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "must gate 89.2–89.3 behind this NFR14"
    );
    assert!(
        text.contains("Richard") && text.contains("NFR14"),
        "must name NFR14 owner"
    );
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(
        text.contains("bitloom-prelude") || text.contains("AD-6"),
        "must cite prelude / AD-6"
    );

    let s89_1 = yaml_value_for_key(&sprint, "89-1-epic-89-nfr14-风险记录").expect("89-1");
    let later = [
        "89-2-自动-fsm-标签提取实现与验收-fr157",
        "89-3-fr157-收口与文档指针",
    ];
    if s89_1 != "done" {
        for key in later {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 89-1 done (got {status})"
            );
        }
    }
    assert!(
        sprint.contains("epic-87: done") || sprint.contains("epic-87:done"),
        "Epic 87 must be done before Epic 89 NFR14"
    );
}
