//! ATDD / guardrail: Epic 91 NFR14 risk record for FR159
//! MemRead stub→full emit (Story 91.1 / AD-28 / NFR68–72).

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
fn nfr14_risk_epic91_memread_full_emit_fr159_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic91-memread-full-emit-fr159.md",
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
        && !text.contains("closed — Story 91.3");
    let status_closed = text.contains("closed — Story 91.3")
        || (text.contains("closed") && text.contains("Epic 91") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let epic91_done = sprint.contains("epic-91: done") || sprint.contains("epic-91:done");
    if !epic91_done {
        assert!(
            status_open,
            "before epic-91 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR68")
            && text.contains("FR112")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR68 / FR112 isolation"
    );
    assert!(
        text.contains("FR159")
            && (text.contains("MemRead") || text.contains("mem"))
            && (text.contains("stub") || text.contains("Stub") || text.contains("0")),
        "must cover FR159 MemRead stub gap"
    );
    assert!(
        text.contains("generate_functional_sim") || text.contains("缺口") || text.contains("G1"),
        "must nail gap vs generate_functional_sim stub"
    );
    assert!(
        text.contains("latency") || text.contains("SyncReadMem") || text.contains("latency-1"),
        "must nail SyncReadMem / latency-1 target"
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
        (text.contains("stub") || text.contains("alone") || text.contains("FR112"))
            && (text.contains("不得") || text.contains("冒充") || text.contains("勾选")),
        "must forbid stub/FR112-alone closing FR159"
    );
    assert!(
        text.contains("91.2")
            && text.contains("91.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "must gate 91.2–91.3 behind this NFR14"
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(text.contains("bitloom-prelude") || text.contains("AD-6"));

    let s91_1 = yaml_value_for_key(&sprint, "91-1-epic-91-nfr14-风险记录").expect("91-1");
    let later = [
        "91-2-memread-完整生成实现与验收-fr159",
        "91-3-fr159-收口与文档指针",
    ];
    if s91_1 != "done" {
        for key in later {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 91-1 done (got {status})"
            );
        }
    }
    assert!(
        sprint.contains("epic-87: done") || sprint.contains("epic-87:done"),
        "Epic 87 must be done before Epic 91 NFR14"
    );
}
