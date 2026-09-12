//! ATDD / guardrail: Epic 107 NFR14 risk record for FR174
//! unpaired CIRCT/Chisel HEAD product path (Story 107.1).

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
fn nfr14_risk_epic107_unpaired_head_fr174_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic107-unpaired-head-fr174.md");
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
        && !text.contains("closed — Story 107.3");
    let status_closed = text.contains("closed — Story 107.3")
        || (text.contains("closed") && text.contains("Epic 107") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-107: done") || sprint.contains("epic-107:done")) {
        assert!(
            status_open,
            "before epic-107 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR78")
            && text.contains("FR170")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR78 / FR170 isolation"
    );
    assert!(
        text.contains("FR174")
            && (text.contains("HEAD") || text.contains("unpaired") || text.contains("未配对")),
        "must cover FR174 unpaired HEAD"
    );
    assert!(
        (text.contains("AD-9") || text.contains("AD-27"))
            && (text.contains("NFR80") || text.contains("修订")),
        "must require AD-9 and/or AD-27 revise (NFR80)"
    );
    assert!(
        (text.contains("PATH") || text.contains("随机"))
            && (text.contains("不得") || text.contains("禁止")),
        "must forbid PATH-random fake HEAD"
    );
    assert!(
        (text.contains("FR170") || text.contains("FR138") || text.contains("FR165"))
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid prior-FR-alone closing FR174"
    );
    assert!(
        text.contains("107.2")
            && text.contains("107.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 107.2–107.3"
    );
    assert!(text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"));
    assert!(
        (text.contains("git push") || text.contains("`git push`"))
            && (text.contains("不是") || text.contains("非") || text.contains("不得"))
    );
}

#[test]
fn nfr14_epic107_gates_107_2_until_107_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let s1 = yaml_value_for_key(&sprint, "107-1-epic-107-nfr14-风险记录").expect("107-1");
    let s2 = yaml_value_for_key(&sprint, "107-2-unpaired-head-产品路径实现与验收-fr174");
    if s1 == "backlog" || s1 == "ready-for-dev" || s1 == "in-progress" {
        if let Some(s2) = s2 {
            assert!(
                s2 == "backlog",
                "107.2 must stay backlog until 107.1 done; got {s2}"
            );
        }
    }
}

#[test]
fn nfr14_epic107_requires_epic105_and_soft_106() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    assert!(
        sprint.contains("epic-105: done") || sprint.contains("epic-105:done"),
        "Epic 107 assumes Epic 105 closed"
    );
    assert!(
        sprint.contains("epic-106: done") || sprint.contains("epic-106:done"),
        "soft order: Epic 106 should be done before Epic 107 NFR14 proceeds"
    );
}
