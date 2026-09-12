//! ATDD / guardrail: Epic 106 NFR14 risk record for FR173
//! firtool bump beyond AD-9 with Chisel pairing (Story 106.1).

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
fn nfr14_risk_epic106_firtool_bump_ad9_fr173_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic106-firtool-bump-ad9-fr173.md",
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
        && !text.contains("closed — Story 106.3");
    let status_closed = text.contains("closed — Story 106.3")
        || (text.contains("closed") && text.contains("Epic 106") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-106: done") || sprint.contains("epic-106:done")) {
        assert!(
            status_open,
            "before epic-106 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR78")
            && text.contains("FR169")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR78 / FR169 isolation"
    );
    assert!(
        text.contains("FR173")
            && (text.contains("1.158.0") || text.contains("firtool-1.158"))
            && (text.contains("7.15.0") || text.contains("Chisel 7.15")),
        "must pin Chisel 7.15.0 ↔ firtool-1.158.0"
    );
    assert!(
        text.contains("AD-9")
            && (text.contains("配对") || text.contains("NFR80") || text.contains("NFR12")),
        "must require AD-9 / pairing for bumps"
    );
    assert!(
        (text.contains("PATH") || text.contains("随机") || text.contains("HEAD"))
            && (text.contains("不得") || text.contains("禁止")),
        "must forbid PATH-random / unpaired HEAD fake bumps"
    );
    assert!(
        (text.contains("FR169") || text.contains("FR164") || text.contains("FR137"))
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid FR169/FR164/FR137-alone closing FR173"
    );
    assert!(
        text.contains("NFR81")
            && (text.contains("静默") || text.contains("扩大") || text.contains("子集")),
        "must forbid silent subset expand"
    );
    assert!(
        text.contains("106.2")
            && text.contains("106.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "must gate 106.2–106.3"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "must name NFR14 owner"
    );
    assert!(
        (text.contains("git push") || text.contains("`git push`"))
            && (text.contains("不是") || text.contains("非") || text.contains("不得")),
        "must state git push is not an FR"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom-prelude") || text.contains("设计 crate"),
        "must keep Bitloom / design-crate boundary"
    );
}

#[test]
fn nfr14_epic106_gates_106_2_until_106_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let s106_1 = yaml_value_for_key(&sprint, "106-1-epic-106-nfr14-风险记录");
    let s106_2 = yaml_value_for_key(&sprint, "106-2-firtool-升钉实现与验收-fr173");
    let s106_1 = s106_1.expect("106-1 key");
    if s106_1 == "backlog" || s106_1 == "ready-for-dev" || s106_1 == "in-progress" {
        if let Some(s2) = s106_2 {
            assert!(
                s2 == "backlog",
                "106.2 must stay backlog until 106.1 done; got {s2}"
            );
        }
    }
}

#[test]
fn nfr14_epic106_requires_epic105_closed() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    assert!(
        sprint.contains("epic-105: done") || sprint.contains("epic-105:done"),
        "Epic 106 NFR14 assumes Epic 105 gate closed"
    );
}
