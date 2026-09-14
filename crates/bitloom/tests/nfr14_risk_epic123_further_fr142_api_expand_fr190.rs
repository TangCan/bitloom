//! ATDD / guardrail: Epic 123 NFR14 risk record for FR190
//! further explicit FR142 surface expand beyond FR183 (Story 123.1).

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
fn nfr14_risk_epic123_further_fr142_api_expand_fr190_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic123-further-fr142-api-expand-fr190.md",
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
        && !text.contains("closed — Story 123.3");
    let status_closed = text.contains("closed — Story 123.3")
        || (text.contains("closed") && text.contains("Epic 123") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-123: done") || sprint.contains("epic-123:done")) {
        assert!(
            status_open,
            "before epic-123 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR88")
            && text.contains("FR183")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR88 / FR183 isolation"
    );
    assert!(
        text.contains("FR190")
            && (text.contains("FR142") || text.contains("表面") || text.contains("surface")),
        "must cover FR190 further FR142 expand"
    );
    assert!(
        text.contains("public-api-1-0-surface")
            && (text.contains("emit") || text.contains("import") || text.contains("check_")),
        "must pin surface doc + emit/import/check expand subset"
    );
    assert!(
        (text.contains("FR183") || text.contains("FR142"))
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid FR183/FR142 alone closing FR190"
    );
    assert!(
        text.contains("AD-6") && (text.contains("prelude") || text.contains("bitloom-prelude")),
        "must keep AD-6 prelude-only"
    );
    assert!(
        (text.contains("NFR90") || text.contains("修订")) && text.contains("public-api"),
        "must require surface doc revise (NFR90)"
    );
    assert!(
        text.contains("123.2")
            && text.contains("123.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 123.2–123.3"
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
}

#[test]
fn nfr14_epic123_gates_123_2_until_123_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    assert!(
        sprint.contains("epic-118: done") || sprint.contains("epic-118:done"),
        "Epic 118 must be done before Epic 123 NFR14"
    );
    let s123_1 = yaml_value_for_key(&sprint, "123-1-epic-123-nfr14-风险记录").expect("123-1 key");
    if s123_1 != "done" {
        for key in [
            "123-2-继续显式扩大-fr142-表面实现与验收-fr190",
            "123-3-fr190-收口与文档指针",
        ] {
            let st = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                st, "backlog",
                "{key} must stay backlog until 123-1 done (got {st})"
            );
        }
    }
}
