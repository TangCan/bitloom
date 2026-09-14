//! ATDD / guardrail: Epic 119 NFR14 risk record for FR186
//! unbounded CIRCT tip beyond FR179 (Story 119.1).

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
fn nfr14_risk_epic119_unbounded_circt_tip_fr186_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic119-unbounded-circt-tip-fr186.md",
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
        && !text.contains("closed — Story 119.3");
    let status_closed = text.contains("closed — Story 119.3")
        || (text.contains("closed") && text.contains("Epic 119") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-119: done") || sprint.contains("epic-119:done")) {
        assert!(
            status_open,
            "before epic-119 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR88")
            && text.contains("FR179")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR88 / FR179 isolation"
    );
    assert!(
        text.contains("FR186")
            && (text.contains("无界") || text.contains("tip") || text.contains("live")),
        "must cover FR186 unbounded tip"
    );
    assert!(
        text.contains("AD-9") && (text.contains("NFR90") || text.contains("修订")),
        "must require AD-9 revise (NFR90)"
    );
    assert!(
        (text.contains("PATH") || text.contains("随机") || text.contains("silent"))
            && (text.contains("不得") || text.contains("禁止")),
        "must forbid PATH-random / silent-Ok fake tip"
    );
    assert!(
        (text.contains("FR179") || text.contains("FR174") || text.contains("FR182"))
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid prior-FR-alone closing FR186"
    );
    assert!(
        text.contains("非")
            && (text.contains("产品默认") || text.contains("默认钉") || text.contains("FR179")),
        "must require honesty that tip is not product default / not FR179 alone"
    );
    assert!(
        text.contains("119.2")
            && text.contains("119.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 119.2–119.3"
    );
    assert!(text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"));
    assert!(
        (text.contains("git push") || text.contains("`git push`"))
            && (text.contains("不是") || text.contains("非") || text.contains("不得"))
    );
    assert!(
        text.contains("NFR91")
            && (text.contains("子集") || text.contains("另开") || text.contains("静默")),
        "must cite NFR91"
    );
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(text.contains("bitloom-prelude") || text.contains("bitloom_prelude"));
}

#[test]
fn nfr14_epic119_gates_119_2_until_119_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    assert!(
        sprint.contains("epic-118: done") || sprint.contains("epic-118:done"),
        "Epic 118 must be done before Epic 119 NFR14"
    );
    let s119_1 = yaml_value_for_key(&sprint, "119-1-epic-119-nfr14-风险记录").expect("119-1 key");
    if s119_1 != "done" {
        for key in [
            "119-2-无界-circt-tip-实现与验收-fr186",
            "119-3-fr186-收口与文档指针",
        ] {
            let st = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                st, "backlog",
                "{key} must stay backlog until 119-1 done (got {st})"
            );
        }
    }
}
