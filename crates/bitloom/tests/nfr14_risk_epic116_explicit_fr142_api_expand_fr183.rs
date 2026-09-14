//! ATDD / guardrail: Epic 116 NFR14 risk record for FR183
//! explicit FR142 public API surface expand (Story 116.1).

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
fn nfr14_risk_epic116_explicit_fr142_api_expand_fr183_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic116-explicit-fr142-api-expand-fr183.md",
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
        && !text.contains("closed — Story 116.3");
    let status_closed = text.contains("closed — Story 116.3")
        || (text.contains("closed") && text.contains("Epic 116") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-116: done") || sprint.contains("epic-116:done")) {
        assert!(
            status_open,
            "before epic-116 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR83")
            && text.contains("FR142")
            && (text.contains("不得") || text.contains("仍有效") || text.contains("alone")),
        "must state NFR83 / FR142 isolation"
    );
    assert!(
        text.contains("FR183")
            && (text.contains("表面") || text.contains("surface") || text.contains("FR142")),
        "must cover FR183 explicit surface expand"
    );
    assert!(
        text.contains("public-api-1-0-surface")
            && (text.contains("NFR85") || text.contains("修订")),
        "must plan public-api surface doc revise (NFR85)"
    );
    assert!(
        text.contains("SemVer")
            || text.contains("semver")
            || text.contains("FR143")
            || text.contains("minor"),
        "must pin SemVer / release honesty"
    );
    assert!(
        (text.contains("静默") || text.contains("silent"))
            && (text.contains("扩大") || text.contains("expand") || text.contains("扩")),
        "must forbid silent expand narrative"
    );
    assert!(
        text.contains("bitloom-firrtl")
            && (text.contains("emit_chisel") || text.contains("BitloomFirrtlParser")),
        "must pin proposed surface entries (S1)"
    );
    assert!(
        text.contains("AD-6") && (text.contains("bitloom-prelude") || text.contains("prelude")),
        "must keep AD-6 design-crate → prelude boundary"
    );
    assert!(
        text.contains("116.2")
            && text.contains("116.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 116.2–116.3"
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
fn nfr14_epic116_gates_116_2_until_116_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let s116_1 = yaml_value_for_key(&sprint, "116-1-epic-116-nfr14-风险记录").expect("116-1 key");
    if s116_1 != "done" {
        for key in [
            "116-2-显式扩大-fr142-公开-api-表面实现与验收-fr183",
            "116-3-fr183-收口与文档指针",
        ] {
            let st = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                st, "backlog",
                "{key} must stay backlog until 116-1 done (got {st})"
            );
        }
    }
    assert!(
        sprint.contains("epic-111: done") || sprint.contains("epic-111:done"),
        "Epic 111 must be done before Epic 116 NFR14"
    );
}
