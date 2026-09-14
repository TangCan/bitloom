//! ATDD / guardrail: Epic 112 NFR14 risk record for FR179
//! floating CIRCT git HEAD beyond FR174 (Story 112.1).

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
fn nfr14_risk_epic112_floating_circt_git_head_fr179_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic112-floating-circt-git-head-fr179.md",
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
        && !text.contains("closed — Story 112.3");
    let status_closed = text.contains("closed — Story 112.3")
        || (text.contains("closed") && text.contains("Epic 112") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-112: done") || sprint.contains("epic-112:done")) {
        assert!(
            status_open,
            "before epic-112 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR83")
            && text.contains("FR174")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR83 / FR174 isolation"
    );
    assert!(
        text.contains("FR179")
            && (text.contains("浮动") || text.contains("git HEAD") || text.contains("HEAD")),
        "must cover FR179 floating HEAD"
    );
    assert!(
        text.contains("1.156.0") || text.contains("FR174"),
        "must contrast FR174 document-pinned 1.156.0 subset"
    );
    assert!(
        text.contains("AD-9") && (text.contains("NFR85") || text.contains("修订")),
        "must require AD-9 revise (NFR85)"
    );
    assert!(
        (text.contains("PATH") || text.contains("随机") || text.contains("silent"))
            && (text.contains("不得") || text.contains("禁止")),
        "must forbid PATH-random / silent-Ok fake HEAD"
    );
    assert!(
        (text.contains("FR174") || text.contains("FR173") || text.contains("FR182"))
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid prior-FR-alone closing FR179"
    );
    assert!(
        text.contains("112.2")
            && text.contains("112.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 112.2–112.3"
    );
    assert!(text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"));
    assert!(
        (text.contains("git push") || text.contains("`git push`"))
            && (text.contains("不是") || text.contains("非") || text.contains("不得"))
    );
    assert!(
        text.contains("NFR86")
            && (text.contains("子集") || text.contains("另开") || text.contains("静默")),
        "must cite NFR86"
    );
}

#[test]
fn nfr14_epic112_gates_112_2_until_112_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let s112_1 = yaml_value_for_key(&sprint, "112-1-epic-112-nfr14-风险记录").expect("112-1 key");
    if s112_1 != "done" {
        for key in [
            "112-2-浮动-circt-git-head-实现与验收-fr179",
            "112-3-fr179-收口与文档指针",
        ] {
            let st = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                st, "backlog",
                "{key} must stay backlog until 112-1 done (got {st})"
            );
        }
    }
    assert!(
        sprint.contains("epic-111: done") || sprint.contains("epic-111:done"),
        "Epic 111 must be done before Epic 112 NFR14"
    );
}
