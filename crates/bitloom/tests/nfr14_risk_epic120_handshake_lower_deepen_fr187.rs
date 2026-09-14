//! ATDD / guardrail: Epic 120 NFR14 risk record for FR187
//! Handshake lower deepen beyond FR180 fork+join (Story 120.1).

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
fn nfr14_risk_epic120_handshake_lower_deepen_fr187_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic120-handshake-lower-deepen-fr187.md",
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
        && !text.contains("closed — Story 120.3");
    let status_closed = text.contains("closed — Story 120.3")
        || (text.contains("closed") && text.contains("Epic 120") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-120: done") || sprint.contains("epic-120:done")) {
        assert!(
            status_open,
            "before epic-120 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR88")
            && text.contains("FR180")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR88 / FR180 isolation"
    );
    assert!(
        text.contains("FR187")
            && (text.contains("Handshake") || text.contains("handshake") || text.contains("lower")),
        "must cover FR187 Handshake lower deepen"
    );
    assert!(
        text.contains("handshake.branch") && text.contains("handshake.merge"),
        "must pin branch+merge deepen subset beyond FR180"
    );
    assert!(
        text.contains("handshake.fork") && text.contains("handshake.join"),
        "must contrast FR180 fork+join baseline"
    );
    assert!(
        (text.contains("FR180") || text.contains("FR129"))
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid FR180/FR129 alone closing FR187"
    );
    assert!(
        text.contains("AD-25") && (text.contains("NFR90") || text.contains("修订")),
        "must require AD-25 revise (NFR90)"
    );
    assert!(
        text.contains("120.2")
            && text.contains("120.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 120.2–120.3"
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
fn nfr14_epic120_gates_120_2_until_120_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    assert!(
        sprint.contains("epic-118: done") || sprint.contains("epic-118:done"),
        "Epic 118 must be done before Epic 120 NFR14"
    );
    let s120_1 = yaml_value_for_key(&sprint, "120-1-epic-120-nfr14-风险记录").expect("120-1 key");
    if s120_1 != "done" {
        for key in [
            "120-2-handshake-lower-加深实现与验收-fr187",
            "120-3-fr187-收口与文档指针",
        ] {
            let st = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                st, "backlog",
                "{key} must stay backlog until 120-1 done (got {st})"
            );
        }
    }
}
