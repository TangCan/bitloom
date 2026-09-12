//! ATDD / guardrail: Epic 100 NFR14 risk record for FR167
//! full ChiselSim + multi IDE stores (Story 100.1 / AD-28 / NFR73–77).

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
fn nfr14_risk_epic100_chiselsim_ide_stores_fr167_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic100-chiselsim-ide-stores-fr167.md",
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
        && !text.contains("closed — Story 100.3");
    let status_closed = text.contains("closed — Story 100.3")
        || (text.contains("closed") && text.contains("Epic 100") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let epic100_done = sprint.contains("epic-100: done") || sprint.contains("epic-100:done");
    if !epic100_done {
        assert!(
            status_open,
            "before epic-100 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR73")
            && (text.contains("FR162") || text.contains("FR134"))
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR73 / FR162·FR134 isolation"
    );
    assert!(
        text.contains("FR167") && text.contains("ChiselSim"),
        "must cover FR167 ChiselSim"
    );
    assert!(
        (text.contains("Open VSX") || text.contains("JetBrains") || text.contains("商店"))
            && (text.contains("多端") || text.contains("marketplace")),
        "must nail multi IDE-store targets"
    );
    assert!(
        (text.contains("(a)") || text.contains("（a）"))
            && (text.contains("(b)") || text.contains("（b）"))
            && (text.contains("皆") || text.contains("二者") || text.contains("必须")),
        "must require both (a) and (b)"
    );
    assert!(
        (text.contains("silent") || text.contains("silent-Ok") || text.contains("令牌"))
            && (text.contains("不得") || text.contains("非零")),
        "must forbid silent-Ok missing token"
    );
    assert!(
        text.contains("NFR75") || text.contains("商店"),
        "must cite NFR75 / store honesty"
    );
    assert!(
        text.contains("NFR76")
            && (text.contains("静默") || text.contains("扩大") || text.contains("子集")),
        "must forbid silent subset expand"
    );
    assert!(
        (text.contains("FR162") || text.contains("FR134"))
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid FR162/FR134-alone closing FR167"
    );
    assert!(
        text.contains("100.2")
            && text.contains("100.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "must gate 100.2–100.3"
    );
    assert!(
        text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"),
        "must name owner"
    );
    assert!(
        text.contains("NFR14-crates"),
        "must disambiguate NFR14-crates"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom-prelude"),
        "must cite Bitloom / prelude"
    );

    let s100_1 = yaml_value_for_key(&sprint, "100-1-epic-100-nfr14-风险记录").expect("100-1 key");
    if s100_1 != "done" {
        for key in [
            "100-2-chiselsim-与多端-ide-商店实现与验收-fr167",
            "100-3-fr167-收口与文档指针",
        ] {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 100-1 done (got {status})"
            );
        }
    }
    assert!(
        sprint.contains("epic-99: done") || sprint.contains("epic-99:done"),
        "Epic 99 must be done before Epic 100 NFR14"
    );
}
