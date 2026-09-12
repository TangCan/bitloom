//! ATDD / guardrail: Epic 110 NFR14 risk record for FR177
//! Phase 21 claim honesty (Story 110.1).

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
fn nfr14_risk_epic110_phase21_claim_honesty_fr177_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic110-phase21-claim-honesty-fr177.md",
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
        && !text.contains("closed — Story 110.3");
    let status_closed = text.contains("closed — Story 110.3")
        || (text.contains("closed") && text.contains("Epic 110") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-110: done") || sprint.contains("epic-110:done")) {
        assert!(
            status_open,
            "before epic-110 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("FR177")
            && (text.contains("宣称") || text.contains("诚实") || text.contains("claim")),
        "must cover FR177 claim honesty"
    );
    assert!(
        text.contains("NFR82")
            && (text.contains("Phase 20") || text.contains("冒充") || text.contains("FR173")),
        "must forbid Phase 20 alone / require FR pointers"
    );
    assert!(
        text.contains("FR172")
            && text.contains("FR173")
            && text.contains("FR174")
            && text.contains("FR175")
            && text.contains("FR176"),
        "must reference FR172–176 claim map"
    );
    assert!(
        (text.contains("FR142") || text.contains("静默扩大"))
            && (text.contains("不得") || text.contains("禁止")),
        "must forbid silent FR142 expand"
    );
    assert!(
        text.contains("NFR76")
            && (text.contains("账本已空") || text.contains("已空"))
            && (text.contains("不得") || text.contains("禁止")),
        "must forbid claiming NFR76 ledger empty"
    );
    assert!(
        text.contains("NFR81")
            && (text.contains("新合同") || text.contains("子集") || text.contains("静默")),
        "must keep NFR81 honesty"
    );
    assert!(
        text.contains("git push") && (text.contains("不是") || text.contains("不得")),
        "must state git push is not an FR"
    );
    assert!(
        text.contains("110.2")
            && text.contains("110.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 110.2–110.3"
    );
    assert!(text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"));
}

#[test]
fn nfr14_epic110_gates_110_2_until_110_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let s1 = yaml_value_for_key(&sprint, "110-1-epic-110-nfr14-风险记录").expect("110-1");
    let s2 = yaml_value_for_key(&sprint, "110-2-readme-deferred-状态页诚实更新-fr177");
    if s1 == "backlog" || s1 == "ready-for-dev" || s1 == "in-progress" {
        if let Some(s2) = s2 {
            assert!(
                s2 == "backlog",
                "110.2 must stay backlog until 110.1 done; got {s2}"
            );
        }
    }
}

#[test]
fn nfr14_epic110_requires_epic105_and_deepens_closed() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    assert!(
        sprint.contains("epic-105: done") || sprint.contains("epic-105:done"),
        "Epic 110 assumes Epic 105 closed"
    );
    for epic in ["epic-106", "epic-107", "epic-108", "epic-109"] {
        assert!(
            sprint.contains(&format!("{epic}: done")) || sprint.contains(&format!("{epic}:done")),
            "{epic} must be done before Epic 110 soft-order claim honesty"
        );
    }
}
