//! ATDD / guardrail: Epic 114 NFR14 risk record for FR181
//! deeper Style Guide / linter beyond FR176 (Story 114.1).

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
fn nfr14_risk_epic114_deeper_style_guide_linter_fr181_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic114-deeper-style-guide-linter-fr181.md",
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
        && !text.contains("closed — Story 114.3");
    let status_closed = text.contains("closed — Story 114.3")
        || (text.contains("closed") && text.contains("Epic 114") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-114: done") || sprint.contains("epic-114:done")) {
        assert!(
            status_open,
            "before epic-114 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR83")
            && text.contains("FR176")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR83 / FR176 isolation"
    );
    assert!(
        text.contains("FR181")
            && (text.contains("Style") || text.contains("linter") || text.contains("style-linter")),
        "must cover FR181 Style/linter deepen"
    );
    assert!(
        text.contains("emit_chisel_style_linter_fr181")
            || text.contains("chisel-style-linter-deepen")
            || text.contains("FR181 style-linter"),
        "must pin deepen API / gate / marker"
    );
    assert!(
        text.contains("FR165")
            && text.contains("FR130")
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must contrast FR165/FR130 alone ≠ FR181"
    );
    assert!(
        text.contains("AD-27") && (text.contains("NFR85") || text.contains("修订")),
        "must require AD-27 revise (NFR85)"
    );
    assert!(
        (text.contains("FR176") || text.contains("FR138"))
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid prior-FR-alone closing FR181"
    );
    assert!(
        text.contains("114.2")
            && text.contains("114.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 114.2–114.3"
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
fn nfr14_epic114_gates_114_2_until_114_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let s114_1 = yaml_value_for_key(&sprint, "114-1-epic-114-nfr14-风险记录").expect("114-1 key");
    if s114_1 != "done" {
        for key in [
            "114-2-更深-style-guide-linter-实现与验收-fr181",
            "114-3-fr181-收口与文档指针",
        ] {
            let st = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                st, "backlog",
                "{key} must stay backlog until 114-1 done (got {st})"
            );
        }
    }
    assert!(
        sprint.contains("epic-111: done") || sprint.contains("epic-111:done"),
        "Epic 111 must be done before Epic 114 NFR14"
    );
}
