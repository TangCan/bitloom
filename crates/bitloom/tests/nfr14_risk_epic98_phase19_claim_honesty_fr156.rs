//! ATDD / guardrail: Epic 98 NFR14 risk record for FR156
//! Phase 19 claim honesty (Story 98.1 / AD-28 / NFR68–72).

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
fn nfr14_risk_epic98_phase19_claim_honesty_fr156_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic98-phase19-claim-honesty-fr156.md",
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
        && !text.contains("closed — Story 98.3");
    let status_closed = text.contains("closed — Story 98.3")
        || (text.contains("closed") && text.contains("Epic 98") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let epic98_done = sprint.contains("epic-98: done") || sprint.contains("epic-98:done");
    if !epic98_done {
        assert!(
            status_open,
            "before epic-98 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("FR156")
            && (text.contains("宣称") || text.contains("诚实") || text.contains("claim")),
        "must cover FR156 claim honesty"
    );
    assert!(
        text.contains("NFR72")
            && (text.contains("Phase 18") || text.contains("冒充") || text.contains("FR155")),
        "must forbid Phase 18 alone / require FR pointers"
    );
    assert!(
        text.contains("FR155")
            && text.contains("FR157")
            && text.contains("FR165")
            && text.contains("FR154"),
        "must reference FR154–165 claim map"
    );
    assert!(
        (text.contains("FR142") || text.contains("静默扩大"))
            && (text.contains("不得") || text.contains("禁止")),
        "must forbid silent FR142 expand"
    );
    assert!(
        text.contains("NFR71")
            && (text.contains("超出") || text.contains("子集") || text.contains("新合同")),
        "must keep NFR71 honesty"
    );
    assert!(
        text.contains("NFR68")
            && (text.contains("仍有效") || text.contains("不得") || text.contains("改写")),
        "must state NFR68 isolation"
    );
    assert!(
        (text.contains("未关") || text.contains("已交付"))
            && (text.contains("不得") || text.contains("禁止")),
        "must forbid claiming undelivered FRs"
    );
    assert!(
        text.contains("98.2")
            && text.contains("98.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "must gate 98.2–98.3 behind this NFR14"
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(text.contains("bitloom-prelude") || text.contains("AD-6"));

    let s98_1 = yaml_value_for_key(&sprint, "98-1-epic-98-nfr14-风险记录").expect("98-1");
    let later = [
        "98-2-readme-deferred-状态页诚实更新-fr156",
        "98-3-fr156-收口与-phase-19-故事清单指针",
    ];
    if s98_1 != "done" {
        for key in later {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 98-1 done (got {status})"
            );
        }
    }
    assert!(
        sprint.contains("epic-87: done") || sprint.contains("epic-87:done"),
        "Epic 87 must be done before Epic 98 NFR14"
    );
}
