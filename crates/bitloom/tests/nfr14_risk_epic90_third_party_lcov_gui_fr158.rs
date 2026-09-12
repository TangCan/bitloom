//! ATDD / guardrail: Epic 90 NFR14 risk record for FR158
//! third-party LCOV GUI (Story 90.1 / AD-28 / NFR68–72).

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
fn nfr14_risk_epic90_third_party_lcov_gui_fr158_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic90-third-party-lcov-gui-fr158.md",
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
        && !text.contains("closed — Story 90.3");
    let status_closed = text.contains("closed — Story 90.3")
        || (text.contains("closed") && text.contains("Epic 90") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let epic90_done = sprint.contains("epic-90: done") || sprint.contains("epic-90:done");
    if !epic90_done {
        assert!(
            status_open,
            "before epic-90 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR68")
            && (text.contains("FR114") || text.contains("Phase 12"))
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR68 / FR114 isolation"
    );
    assert!(
        text.contains("FR158") && (text.contains("genhtml") || text.contains("lcov")),
        "must nail FR158 third-party tool (genhtml/lcov)"
    );
    assert!(
        text.contains("coverage.lcov")
            && (text.contains("输入") || text.contains("产物") || text.contains("Input")),
        "must nail input artifact coverage.lcov"
    );
    assert!(
        text.contains("FR114")
            && (text.contains("树内") || text.contains("coverage.html") || text.contains("alone")),
        "must bound against FR114 in-tree GUI"
    );
    assert!(
        (text.contains("缺") || text.contains("PATH") || text.contains("失败"))
            && (text.contains("genhtml") || text.contains("工具")),
        "must nail missing-tool failure semantics"
    );
    assert!(
        text.contains("NFR71")
            && (text.contains("静默") || text.contains("扩大") || text.contains("子集")),
        "must forbid silent subset expand"
    );
    assert!(
        (text.contains("仅导出") || text.contains("alone") || text.contains("LCOV 文件"))
            && (text.contains("不得") || text.contains("冒充")),
        "must forbid LCOV-export-alone closing FR158"
    );
    assert!(
        text.contains("90.2")
            && text.contains("90.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "must gate 90.2–90.3 behind this NFR14"
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(text.contains("bitloom-prelude") || text.contains("AD-6"));

    let s90_1 = yaml_value_for_key(&sprint, "90-1-epic-90-nfr14-风险记录").expect("90-1");
    let later = [
        "90-2-lcov-gui-一等集成实现与验收-fr158",
        "90-3-fr158-收口与文档指针",
    ];
    if s90_1 != "done" {
        for key in later {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 90-1 done (got {status})"
            );
        }
    }
    assert!(
        sprint.contains("epic-87: done") || sprint.contains("epic-87:done"),
        "Epic 87 must be done before Epic 90 NFR14"
    );
}
