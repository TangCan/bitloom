//! ATDD / guardrail: Epic 97 NFR14 risk record for FR165
//! deeper Chisel/Parser ecosystem (Story 97.1 / AD-28 / NFR68–72).

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
fn nfr14_risk_epic97_deeper_chisel_parser_fr165_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic97-deeper-chisel-parser-ecosystem-fr165.md",
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
        && !text.contains("closed — Story 97.3");
    let status_closed = text.contains("closed — Story 97.3")
        || (text.contains("closed") && text.contains("Epic 97") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let epic97_done = sprint.contains("epic-97: done") || sprint.contains("epic-97:done");
    if !epic97_done {
        assert!(
            status_open,
            "before epic-97 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR68")
            && text.contains("FR138")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR68 / FR138 isolation"
    );
    assert!(
        text.contains("FR165")
            && (text.contains("Style") || text.contains("linter") || text.contains("Parser")),
        "must cover FR165 Chisel/Parser ecosystem"
    );
    assert!(
        (text.contains("Style Guide") || text.contains("linter") || text.contains("lint"))
            && (text.contains("选定") || text.contains("MVP")),
        "must nail Style Guide/linter as selected MVP"
    );
    assert!(
        (text.contains("HEAD") || text.contains("Chisel HEAD") || text.contains("回迁"))
            && (text.contains("不做") || text.contains("新合同") || text.contains("NFR71")),
        "must defer Chisel HEAD Parser migration"
    );
    assert!(
        text.contains("AD-27")
            && (text.contains("不修订") || text.contains("NFR70") || text.contains("修订")),
        "must address AD-27 / NFR70"
    );
    assert!(
        text.contains("FR130")
            && (text.contains("仍有效") || text.contains("alone") || text.contains("≠")),
        "must keep FR130 boundary"
    );
    assert!(
        (text.contains("验收") || text.contains("谓词") || text.contains("ATDD"))
            && (text.contains("失败") || text.contains("不得") || text.contains("Fail")),
        "must nail acceptance / failure semantics"
    );
    assert!(
        text.contains("NFR71")
            && (text.contains("静默") || text.contains("扩大") || text.contains("子集")),
        "must forbid silent subset expand"
    );
    assert!(
        (text.contains("FR138") || text.contains("P1") || text.contains("alone"))
            && (text.contains("不得") || text.contains("冒充") || text.contains("勾选")),
        "must forbid FR138-alone closing FR165"
    );
    assert!(
        text.contains("97.2")
            && text.contains("97.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "must gate 97.2–97.3 behind this NFR14"
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(text.contains("bitloom-prelude") || text.contains("AD-6"));

    let s97_1 = yaml_value_for_key(&sprint, "97-1-epic-97-nfr14-风险记录").expect("97-1");
    let later = [
        "97-2-更深-chisel-parser-生态实现与验收-fr165",
        "97-3-fr165-收口与文档指针",
    ];
    if s97_1 != "done" {
        for key in later {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 97-1 done (got {status})"
            );
        }
    }
    assert!(
        sprint.contains("epic-87: done") || sprint.contains("epic-87:done"),
        "Epic 87 must be done before Epic 97 NFR14"
    );
}
