//! ATDD / guardrail: Epic 124 NFR14 risk record for FR191
//! Phase 23 claim honesty gate (Story 124.1) — must list FR189 unclosed.

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
fn nfr14_risk_epic124_phase23_claim_honesty_fr191_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic124-phase23-claim-honesty-fr191.md",
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
        && !text.contains("closed — Story 124.3");
    let status_closed = text.contains("closed — Story 124.3")
        || (text.contains("closed") && text.contains("Epic 124") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-124: done") || sprint.contains("epic-124:done")) {
        assert!(
            status_open,
            "before epic-124 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("FR191")
            && (text.contains("宣称") || text.contains("诚实") || text.contains("honesty")),
        "must cover FR191 claim honesty"
    );
    assert!(
        text.contains("FR185")
            && text.contains("FR186")
            && text.contains("FR187")
            && text.contains("FR188")
            && text.contains("FR189")
            && text.contains("FR190"),
        "must include FR185–190 claim matrix"
    );
    assert!(
        text.contains("FR189")
            && (text.contains("blocked")
                || text.contains("未关")
                || text.contains("未关闭")
                || text.contains("不得宣称")),
        "must honestly list FR189 unclosed / blocked"
    );
    assert!(
        (text.contains("Phase 22") || text.contains("结项"))
            && (text.contains("alone") || text.contains("冒充") || text.contains("不得")),
        "must forbid Phase 22 / closeout alone impersonation"
    );
    assert!(
        text.contains("NFR86")
            && (text.contains("账本") || text.contains("空") || text.contains("empty")),
        "must forbid claiming NFR86 ledger empty"
    );
    assert!(
        (text.contains("git push") || text.contains("`git push`"))
            && (text.contains("不是") || text.contains("非") || text.contains("不得"))
    );
    assert!(
        text.contains("124.2")
            && text.contains("124.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 124.2–124.3"
    );
    assert!(text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"));
    assert!(
        text.contains("NFR91")
            && (text.contains("子集") || text.contains("另开") || text.contains("静默")),
        "must cite NFR91"
    );
    assert!(text.contains("NFR92") || text.contains("NFR88"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(text.contains("bitloom-prelude") || text.contains("bitloom_prelude"));
}

#[test]
fn nfr14_epic124_gates_124_2_until_124_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    assert!(
        sprint.contains("epic-118: done") || sprint.contains("epic-118:done"),
        "Epic 118 must be done before Epic 124 NFR14"
    );
    let s124_1 = yaml_value_for_key(&sprint, "124-1-epic-124-nfr14-风险记录").expect("124-1 key");
    if s124_1 != "done" {
        for key in [
            "124-2-readme-deferred-状态页诚实更新-fr191",
            "124-3-fr191-收口与-phase-23-故事清单指针",
        ] {
            let st = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                st, "backlog",
                "{key} must stay backlog until 124-1 done (got {st})"
            );
        }
    }
}
