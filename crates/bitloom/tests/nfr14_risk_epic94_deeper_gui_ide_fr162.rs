//! ATDD / guardrail: Epic 94 NFR14 risk record for FR162
//! deeper GUI/IDE subset (Story 94.1 / AD-28 / NFR68–72).

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
fn nfr14_risk_epic94_deeper_gui_ide_fr162_has_required_fields() {
    let path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-epic94-deeper-gui-ide-fr162.md");
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
        && !text.contains("closed — Story 94.3");
    let status_closed = text.contains("closed — Story 94.3")
        || (text.contains("closed") && text.contains("Epic 94") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let epic94_done = sprint.contains("epic-94: done") || sprint.contains("epic-94:done");
    if !epic94_done {
        assert!(
            status_open,
            "before epic-94 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR68")
            && text.contains("FR134")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR68 / FR134 isolation"
    );
    assert!(
        text.contains("FR162")
            && (text.contains("GUI") || text.contains("IDE") || text.contains("波形")),
        "must cover FR162 deeper GUI/IDE"
    );
    assert!(
        text.contains("typed-wave")
            || text.contains("VCD")
            || text.contains("唯一")
            || text.contains("默认"),
        "must nail default wave-surface selection"
    );
    assert!(
        text.contains("ChiselSim")
            && (text.contains("不做") || text.contains("新合同") || text.contains("NFR71")),
        "must defer ChiselSim explicitly"
    );
    assert!(
        (text.contains("商店") || text.contains("marketplace") || text.contains("Open VSX"))
            && (text.contains("不做") || text.contains("新合同") || text.contains("NFR71")),
        "must defer extra IDE-store multi-target"
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
        (text.contains("FR134") || text.contains("alone") || text.contains("G1"))
            && (text.contains("不得") || text.contains("冒充") || text.contains("勾选")),
        "must forbid FR134-alone closing FR162"
    );
    assert!(
        text.contains("94.2")
            && text.contains("94.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "must gate 94.2–94.3 behind this NFR14"
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(text.contains("bitloom-prelude") || text.contains("AD-6"));

    let s94_1 = yaml_value_for_key(&sprint, "94-1-epic-94-nfr14-风险记录").expect("94-1");
    let later = [
        "94-2-更深-gui-ide-实现与验收-fr162",
        "94-3-fr162-收口与文档指针",
    ];
    if s94_1 != "done" {
        for key in later {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 94-1 done (got {status})"
            );
        }
    }
    assert!(
        sprint.contains("epic-87: done") || sprint.contains("epic-87:done"),
        "Epic 87 must be done before Epic 94 NFR14"
    );
}
