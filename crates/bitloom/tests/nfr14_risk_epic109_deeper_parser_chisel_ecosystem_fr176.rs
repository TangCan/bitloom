//! ATDD / guardrail: Epic 109 NFR14 risk record for FR176
//! deeper Parser/Chisel ecosystem beyond FR170 (Story 109.1).

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
fn nfr14_risk_epic109_deeper_parser_chisel_ecosystem_fr176_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic109-deeper-parser-chisel-ecosystem-fr176.md",
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
        && !text.contains("closed — Story 109.3");
    let status_closed = text.contains("closed — Story 109.3")
        || (text.contains("closed") && text.contains("Epic 109") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-109: done") || sprint.contains("epic-109:done")) {
        assert!(
            status_open,
            "before epic-109 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR78")
            && text.contains("FR170")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR78 / FR170 isolation"
    );
    assert!(
        text.contains("FR176")
            && (text.contains("组合") || text.contains("ecosystem") || text.contains("生态"))
            && text.contains("FR165"),
        "must pin combined ecosystem deepen with FR165"
    );
    assert!(
        text.contains("AD-27") && (text.contains("NFR80") || text.contains("修订")),
        "must require AD-27 revise when touching"
    );
    assert!(
        (text.contains("FR170") || text.contains("FR165") || text.contains("FR138"))
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid prior-FR-alone closing FR176"
    );
    assert!(
        text.contains("109.2")
            && text.contains("109.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 109.2–109.3"
    );
    assert!(text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"));
    assert!(
        (text.contains("git push") || text.contains("`git push`"))
            && (text.contains("不是") || text.contains("非") || text.contains("不得"))
    );
}

#[test]
fn nfr14_epic109_gates_109_2_until_109_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let s1 = yaml_value_for_key(&sprint, "109-1-epic-109-nfr14-风险记录").expect("109-1");
    let s2 = yaml_value_for_key(&sprint, "109-2-更深-parser-chisel-生态实现与验收-fr176");
    if s1 == "backlog" || s1 == "ready-for-dev" || s1 == "in-progress" {
        if let Some(s2) = s2 {
            assert!(
                s2 == "backlog",
                "109.2 must stay backlog until 109.1 done; got {s2}"
            );
        }
    }
}

#[test]
fn nfr14_epic109_requires_epic105_closed() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    assert!(
        sprint.contains("epic-105: done") || sprint.contains("epic-105:done"),
        "Epic 109 assumes Epic 105 closed"
    );
}
