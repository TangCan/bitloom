//! ATDD / guardrail: Epic 117 NFR14 risk record for FR184
//! Phase 22 claim honesty (Story 117.1).

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
fn nfr14_risk_epic117_phase22_claim_honesty_fr184_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic117-phase22-claim-honesty-fr184.md",
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
        && !text.contains("closed — Story 117.3");
    let status_closed = text.contains("closed — Story 117.3")
        || (text.contains("closed") && text.contains("Epic 117") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-117: done") || sprint.contains("epic-117:done")) {
        assert!(
            status_open,
            "before epic-117 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("FR184")
            && (text.contains("宣称") || text.contains("诚实") || text.contains("claim")),
        "must cover FR184 claim honesty"
    );
    assert!(
        text.contains("NFR87")
            && (text.contains("Phase 21") || text.contains("冒充") || text.contains("FR179")),
        "must forbid Phase 21 alone / require FR pointers"
    );
    for fr in ["FR178", "FR179", "FR180", "FR181", "FR182", "FR183"] {
        assert!(text.contains(fr), "must reference {fr} in claim matrix");
    }
    assert!(
        text.contains("矩阵") || text.contains("matrix") || text.contains("宣称矩阵"),
        "must pin closed/open claim matrix"
    );
    assert!(
        text.contains("NFR81")
            && (text.contains("账本已空") || text.contains("已空"))
            && (text.contains("不得") || text.contains("禁止")),
        "must forbid claiming NFR81 ledger empty"
    );
    assert!(
        text.contains("NFR86")
            && (text.contains("新合同") || text.contains("子集") || text.contains("静默")),
        "must keep NFR86 honesty"
    );
    assert!(
        text.contains("NFR83") && (text.contains("仍有效") || text.contains("不得改写")),
        "must keep NFR83 prior-close honesty"
    );
    assert!(
        text.contains("git push") && (text.contains("不是") || text.contains("不得")),
        "must state git push is not an FR"
    );
    assert!(
        text.contains("117.2")
            && text.contains("117.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 117.2–117.3"
    );
    assert!(text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"));
}

#[test]
fn nfr14_epic117_gates_117_2_until_117_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let s117_1 = yaml_value_for_key(&sprint, "117-1-epic-117-nfr14-风险记录").expect("117-1");
    if s117_1 != "done" {
        for key in [
            "117-2-readme-deferred-状态页诚实更新-fr184",
            "117-3-fr184-收口与-phase-22-故事清单指针",
        ] {
            let st = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                st, "backlog",
                "{key} must stay backlog until 117-1 done (got {st})"
            );
        }
    }
}

#[test]
fn nfr14_epic117_requires_epic111_and_deepens_closed() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    assert!(
        sprint.contains("epic-111: done") || sprint.contains("epic-111:done"),
        "Epic 117 assumes Epic 111 closed"
    );
    for epic in ["epic-112", "epic-113", "epic-114", "epic-115", "epic-116"] {
        assert!(
            sprint.contains(&format!("{epic}: done")) || sprint.contains(&format!("{epic}:done")),
            "{epic} must be done before Epic 117 soft-order claim honesty"
        );
    }
}
