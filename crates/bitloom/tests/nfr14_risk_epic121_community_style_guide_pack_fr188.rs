//! ATDD / guardrail: Epic 121 NFR14 risk record for FR188
//! Community Style Guide 全家桶 beyond FR181 wartremover deepen (Story 121.1).

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
fn nfr14_risk_epic121_community_style_guide_pack_fr188_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic121-community-style-guide-pack-fr188.md",
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
        && !text.contains("closed — Story 121.3");
    let status_closed = text.contains("closed — Story 121.3")
        || (text.contains("closed") && text.contains("Epic 121") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-121: done") || sprint.contains("epic-121:done")) {
        assert!(
            status_open,
            "before epic-121 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR88")
            && text.contains("FR181")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR88 / FR181 isolation"
    );
    assert!(
        text.contains("FR188")
            && (text.contains("Style") || text.contains("style") || text.contains("全家桶")),
        "must cover FR188 Style Guide 全家桶"
    );
    assert!(
        text.contains("chisel-community-style-guide") && text.contains("scalafmt-community"),
        "must pin community style-guide + scalafmt pack beyond FR181"
    );
    assert!(
        text.contains("wartremover") || text.contains("fatal-warnings"),
        "must contrast FR181 wartremover/fatal-warnings baseline"
    );
    assert!(
        (text.contains("FR181") || text.contains("FR176"))
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid FR181/FR176 alone closing FR188"
    );
    assert!(
        text.contains("AD-27") && (text.contains("NFR90") || text.contains("修订")),
        "must require AD-27 revise (NFR90)"
    );
    assert!(
        text.contains("121.2")
            && text.contains("121.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 121.2–121.3"
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
fn nfr14_epic121_gates_121_2_until_121_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    assert!(
        sprint.contains("epic-118: done") || sprint.contains("epic-118:done"),
        "Epic 118 must be done before Epic 121 NFR14"
    );
    let s121_1 = yaml_value_for_key(&sprint, "121-1-epic-121-nfr14-风险记录").expect("121-1 key");
    if s121_1 != "done" {
        for key in [
            "121-2-社区-style-guide-全家桶实现与验收-fr188",
            "121-3-fr188-收口与文档指针",
        ] {
            let st = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                st, "backlog",
                "{key} must stay backlog until 121-1 done (got {st})"
            );
        }
    }
}
