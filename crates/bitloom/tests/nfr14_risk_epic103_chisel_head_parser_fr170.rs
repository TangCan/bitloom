//! ATDD / guardrail: Epic 103 NFR14 risk record for FR170
//! Chisel HEAD Parser migration (Story 103.1 / AD-27).

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
fn nfr14_risk_epic103_chisel_head_parser_fr170_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic103-chisel-head-parser-fr170.md",
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
        && !text.contains("closed — Story 103.3");
    let status_closed = text.contains("closed — Story 103.3")
        || (text.contains("closed") && text.contains("Epic 103") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-103: done") || sprint.contains("epic-103:done")) {
        assert!(
            status_open,
            "before epic-103 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR73")
            && text.contains("FR138")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR73 / FR138 isolation"
    );
    assert!(
        text.contains("FR170")
            && (text.contains("HEAD") || text.contains("主线"))
            && (text.contains("Parser") || text.contains("parse")),
        "must cover FR170 HEAD Parser migration"
    );
    assert!(
        text.contains("AD-27") && (text.contains("修订") || text.contains("NFR75")),
        "must require AD-27 revision"
    );
    assert!(
        (text.contains("FR138") || text.contains("FR165") || text.contains("FR130"))
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid FR138/FR165/FR130-alone closing FR170"
    );
    assert!(
        text.contains("NFR76")
            && (text.contains("静默") || text.contains("扩大") || text.contains("子集")),
        "must forbid silent subset expand"
    );
    assert!(
        text.contains("103.2")
            && text.contains("103.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "must gate 103.2–103.3"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "must name owner"
    );
    assert!(
        text.contains("NFR14-crates"),
        "must disambiguate NFR14-crates"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom-prelude"),
        "must cite Bitloom / prelude"
    );

    let s103_1 = yaml_value_for_key(&sprint, "103-1-epic-103-nfr14-风险记录").expect("103-1 key");
    if s103_1 != "done" {
        for key in [
            "103-2-chisel-head-parser-回迁实现与验收-fr170",
            "103-3-fr170-收口与文档指针",
        ] {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 103-1 done (got {status})"
            );
        }
    }
    assert!(
        sprint.contains("epic-99: done") || sprint.contains("epic-99:done"),
        "Epic 99 must be done before Epic 103 NFR14"
    );
}
