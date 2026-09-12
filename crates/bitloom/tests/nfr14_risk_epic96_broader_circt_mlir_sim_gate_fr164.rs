//! ATDD / guardrail: Epic 96 NFR14 risk record for FR164
//! broader CIRCT/MLIR / sim-gate deepen (Story 96.1 / AD-28 / NFR68–72).

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
fn nfr14_risk_epic96_broader_circt_sim_gate_fr164_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic96-broader-circt-mlir-sim-gate-fr164.md",
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
        && !text.contains("closed — Story 96.3");
    let status_closed = text.contains("closed — Story 96.3")
        || (text.contains("closed") && text.contains("Epic 96") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let epic96_done = sprint.contains("epic-96: done") || sprint.contains("epic-96:done");
    if !epic96_done {
        assert!(
            status_open,
            "before epic-96 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR68")
            && text.contains("FR137")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR68 / FR137 isolation"
    );
    assert!(
        text.contains("FR164")
            && (text.contains("仿真") || text.contains("sim") || text.contains("CIRCT")),
        "must cover FR164 sim / CIRCT deepen"
    );
    assert!(
        (text.contains("仿真") || text.contains("sim"))
            && (text.contains("选定") || text.contains("MVP")),
        "must nail sim-gate as selected MVP"
    );
    assert!(
        text.contains("firtool-1.155.0")
            || text.contains("1.155.0")
            || text.contains("1.158.0")
            || text.contains("firtool-1.158.0"),
        "must pin firtool version"
    );
    assert!(
        (text.contains("MLIR") || text.contains("allocation") || text.contains("全家桶"))
            && (text.contains("不做") || text.contains("新合同") || text.contains("NFR71")),
        "must defer broader MLIR lower explicitly"
    );
    assert!(
        text.contains("AD-25")
            && (text.contains("不修订") || text.contains("NFR70") || text.contains("修订")),
        "must address AD-25 / NFR70"
    );
    assert!(
        (text.contains("验收") || text.contains("谓词") || text.contains("ATDD"))
            && (text.contains("失败") || text.contains("非零") || text.contains("不得")),
        "must nail acceptance / failure semantics"
    );
    assert!(
        text.contains("NFR71")
            && (text.contains("静默") || text.contains("扩大") || text.contains("子集")),
        "must forbid silent subset expand"
    );
    assert!(
        (text.contains("FR137") || text.contains("compile") || text.contains("编译"))
            && (text.contains("不得") || text.contains("冒充") || text.contains("勾选")),
        "must forbid FR137-alone closing FR164"
    );
    assert!(
        text.contains("96.2")
            && text.contains("96.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "must gate 96.2–96.3 behind this NFR14"
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(text.contains("bitloom-prelude") || text.contains("AD-6"));

    let s96_1 = yaml_value_for_key(&sprint, "96-1-epic-96-nfr14-风险记录").expect("96-1");
    let later = [
        "96-2-circt-mlir-仿真门禁加深实现与验收-fr164",
        "96-3-fr164-收口与文档指针",
    ];
    if s96_1 != "done" {
        for key in later {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 96-1 done (got {status})"
            );
        }
    }
    assert!(
        sprint.contains("epic-87: done") || sprint.contains("epic-87:done"),
        "Epic 87 must be done before Epic 96 NFR14"
    );
}
