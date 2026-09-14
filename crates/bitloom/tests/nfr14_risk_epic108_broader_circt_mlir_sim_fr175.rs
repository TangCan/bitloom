//! ATDD / guardrail: Epic 108 NFR14 risk record for FR175
//! broader CIRCT/MLIR/sim beyond FR169 (Story 108.1).

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
fn nfr14_risk_epic108_broader_circt_mlir_sim_fr175_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic108-broader-circt-mlir-sim-fr175.md",
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
        && !text.contains("closed — Story 108.3");
    let status_closed = text.contains("closed — Story 108.3")
        || (text.contains("closed") && text.contains("Epic 108") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-108: done") || sprint.contains("epic-108:done")) {
        assert!(
            status_open,
            "before epic-108 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR78")
            && text.contains("FR169")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR78 / FR169 isolation"
    );
    assert!(
        text.contains("FR175")
            && (text.contains("--ir-sv") || text.contains("ir-sv") || text.contains("SV"))
            && (text.contains("ir-verilog") || text.contains("--ir-verilog")),
        "must pin SV / ir-verilog deepen beyond FR169"
    );
    assert!(
        text.contains("1.158.0")
            || text.contains("firtool-1.158")
            || text.contains("1.159.0")
            || text.contains("firtool-1.159"),
        "must use AD-9 product pin"
    );
    assert!(
        (text.contains("FR169") || text.contains("FR164") || text.contains("FR137"))
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid prior-FR-alone closing FR175"
    );
    assert!(
        text.contains("108.2")
            && text.contains("108.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 108.2–108.3"
    );
    assert!(text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"));
    assert!(
        (text.contains("git push") || text.contains("`git push`"))
            && (text.contains("不是") || text.contains("非") || text.contains("不得"))
    );
}

#[test]
fn nfr14_epic108_gates_108_2_until_108_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let s1 = yaml_value_for_key(&sprint, "108-1-epic-108-nfr14-风险记录").expect("108-1");
    let s2 = yaml_value_for_key(&sprint, "108-2-更广-circt-mlir-sim-实现与验收-fr175");
    if s1 == "backlog" || s1 == "ready-for-dev" || s1 == "in-progress" {
        if let Some(s2) = s2 {
            assert!(
                s2 == "backlog",
                "108.2 must stay backlog until 108.1 done; got {s2}"
            );
        }
    }
}

#[test]
fn nfr14_epic108_requires_epic105_closed() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    assert!(
        sprint.contains("epic-105: done") || sprint.contains("epic-105:done"),
        "Epic 108 assumes Epic 105 closed"
    );
}
