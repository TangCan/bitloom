//! ATDD / guardrail: Epic 113 NFR14 risk record for FR180
//! Handshake dialect deepen beyond FR129 / FR175 (Story 113.1).

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
fn nfr14_risk_epic113_handshake_dialect_deepen_fr180_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic113-handshake-dialect-deepen-fr180.md",
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
        && !text.contains("closed — Story 113.3");
    let status_closed = text.contains("closed — Story 113.3")
        || (text.contains("closed") && text.contains("Epic 113") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-113: done") || sprint.contains("epic-113:done")) {
        assert!(
            status_open,
            "before epic-113 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR83")
            && text.contains("FR129")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR83 / FR129 isolation"
    );
    assert!(
        text.contains("FR180")
            && (text.contains("Handshake") || text.contains("handshake") || text.contains("方言")),
        "must cover FR180 Handshake deepen"
    );
    assert!(
        text.contains("handshake.fork") && text.contains("handshake.join"),
        "must pin fork+join deepen subset"
    );
    assert!(
        text.contains("FR175")
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must contrast FR175 alone ≠ FR180"
    );
    assert!(
        text.contains("AD-25") && (text.contains("NFR85") || text.contains("修订")),
        "must require AD-25 revise (NFR85)"
    );
    assert!(
        (text.contains("FR129") || text.contains("FR121") || text.contains("FR175"))
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid prior-FR-alone closing FR180"
    );
    assert!(
        text.contains("113.2")
            && text.contains("113.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 113.2–113.3"
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
fn nfr14_epic113_gates_113_2_until_113_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let s113_1 = yaml_value_for_key(&sprint, "113-1-epic-113-nfr14-风险记录").expect("113-1 key");
    if s113_1 != "done" {
        for key in [
            "113-2-handshake-dialect-加深实现与验收-fr180",
            "113-3-fr180-收口与文档指针",
        ] {
            let st = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                st, "backlog",
                "{key} must stay backlog until 113-1 done (got {st})"
            );
        }
    }
    assert!(
        sprint.contains("epic-111: done") || sprint.contains("epic-111:done"),
        "Epic 111 must be done before Epic 113 NFR14"
    );
}
