//! ATDD / guardrail: Epic 93 NFR14 risk record for FR161
//! formal-sby image hygiene (Story 93.1 / AD-28 / NFR68–72).

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
fn nfr14_risk_epic93_formal_sby_image_hygiene_fr161_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic93-formal-sby-image-hygiene-fr161.md",
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
        && !text.contains("closed — Story 93.3");
    let status_closed = text.contains("closed — Story 93.3")
        || (text.contains("closed") && text.contains("Epic 93") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let epic93_done = sprint.contains("epic-93: done") || sprint.contains("epic-93:done");
    if !epic93_done {
        assert!(
            status_open,
            "before epic-93 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR68")
            && text.contains("FR127")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR68 / FR127 isolation"
    );
    assert!(
        text.contains("FR161")
            && (text.contains("formal-sby") || text.contains("sby") || text.contains("镜像")),
        "must cover FR161 formal-sby hygiene"
    );
    assert!(
        text.contains("pin")
            || text.contains("钉死")
            || text.contains("标签")
            || text.contains("SHA")
            || text.contains("卫生"),
        "must nail pin / tag / hygiene strategy"
    );
    assert!(
        (text.contains("缺") || text.contains("失败") || text.contains("过期"))
            && (text.contains("可读") || text.contains("非零") || text.contains("silent")),
        "must nail missing/expired failure semantics"
    );
    assert!(
        (text.contains("验收") || text.contains("谓词") || text.contains("ATDD"))
            && (text.contains("失败") || text.contains("负向") || text.contains("不得")),
        "must nail acceptance / failure semantics"
    );
    assert!(
        text.contains("NFR71")
            && (text.contains("静默") || text.contains("扩大") || text.contains("子集")),
        "must forbid silent subset expand"
    );
    assert!(
        (text.contains("FR127") || text.contains("alone") || text.contains("FR119"))
            && (text.contains("不得") || text.contains("冒充") || text.contains("勾选")),
        "must forbid FR127/FR119-alone closing FR161"
    );
    assert!(
        text.contains("93.2")
            && text.contains("93.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "must gate 93.2–93.3 behind this NFR14"
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(text.contains("bitloom-prelude") || text.contains("AD-6"));

    let s93_1 = yaml_value_for_key(&sprint, "93-1-epic-93-nfr14-风险记录").expect("93-1");
    let later = [
        "93-2-formal-sby-镜像卫生实现与验收-fr161",
        "93-3-fr161-收口与文档指针",
    ];
    if s93_1 != "done" {
        for key in later {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 93-1 done (got {status})"
            );
        }
    }
    assert!(
        sprint.contains("epic-87: done") || sprint.contains("epic-87:done"),
        "Epic 87 must be done before Epic 93 NFR14"
    );
}
