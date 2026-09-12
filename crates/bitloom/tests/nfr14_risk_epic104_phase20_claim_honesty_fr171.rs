//! ATDD / guardrail: Epic 104 NFR14 risk record for FR171
//! Phase 20 claim honesty (Story 104.1 / AD-28 / NFR73–77).

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
fn nfr14_risk_epic104_phase20_claim_honesty_fr171_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic104-phase20-claim-honesty-fr171.md",
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
        && !text.contains("closed — Story 104.3");
    let status_closed = text.contains("closed — Story 104.3")
        || (text.contains("closed") && text.contains("Epic 104") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let epic104_done = sprint.contains("epic-104: done") || sprint.contains("epic-104:done");
    if !epic104_done {
        assert!(
            status_open,
            "before epic-104 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("FR171")
            && (text.contains("宣称") || text.contains("诚实") || text.contains("claim")),
        "must cover FR171 claim honesty"
    );
    assert!(
        text.contains("NFR77")
            && (text.contains("Phase 19") || text.contains("冒充") || text.contains("FR167")),
        "must forbid Phase 19 alone / require FR pointers"
    );
    assert!(
        text.contains("FR166")
            && text.contains("FR167")
            && text.contains("FR168")
            && text.contains("FR169")
            && text.contains("FR170"),
        "must reference FR166–170 claim map"
    );
    assert!(
        (text.contains("FR142") || text.contains("静默扩大"))
            && (text.contains("不得") || text.contains("禁止")),
        "must forbid silent FR142 expand"
    );
    assert!(
        text.contains("NFR71")
            && (text.contains("账本已空") || text.contains("已空"))
            && (text.contains("不得") || text.contains("禁止")),
        "must forbid claiming NFR71 ledger empty"
    );
    assert!(
        text.contains("NFR76")
            && (text.contains("新合同") || text.contains("子集") || text.contains("静默")),
        "must keep NFR76 honesty"
    );
    assert!(
        text.contains("git push") && (text.contains("不是") || text.contains("不得")),
        "must state git push is not an FR"
    );
    assert!(
        text.contains("104.2")
            && text.contains("104.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "must gate 104.2–104.3"
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
    assert!(
        text.contains("NFR73") && (text.contains("仍有效") || text.contains("不得改写")),
        "must state NFR73 isolation"
    );

    let s104_1 = yaml_value_for_key(&sprint, "104-1-epic-104-nfr14-风险记录").expect("104-1 key");
    if s104_1 != "done" {
        for key in [
            "104-2-readme-deferred-状态页诚实更新-fr171",
            "104-3-fr171-收口与-phase-20-故事清单指针",
        ] {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 104-1 done (got {status})"
            );
        }
    }
    assert!(
        sprint.contains("epic-99: done") || sprint.contains("epic-99:done"),
        "Epic 99 must be done before Epic 104 NFR14"
    );
    assert!(
        (sprint.contains("epic-100: done") || sprint.contains("epic-100:done"))
            && (sprint.contains("epic-101: done") || sprint.contains("epic-101:done"))
            && (sprint.contains("epic-102: done") || sprint.contains("epic-102:done"))
            && (sprint.contains("epic-103: done") || sprint.contains("epic-103:done")),
        "Epic 100–103 should be done (soft order) before FR171 honesty"
    );
}
