//! ATDD / guardrail: Epic 115 NFR14 risk record for FR182
//! unpaired firtool product-pin bump (Story 115.1).

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
fn nfr14_risk_epic115_unpaired_firtool_product_pin_fr182_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic115-unpaired-firtool-product-pin-fr182.md",
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
        && !text.contains("closed — Story 115.3");
    let status_closed = text.contains("closed — Story 115.3")
        || (text.contains("closed") && text.contains("Epic 115") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    if !(sprint.contains("epic-115: done") || sprint.contains("epic-115:done")) {
        assert!(
            status_open,
            "before epic-115 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR83")
            && text.contains("FR173")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR83 / FR173 isolation"
    );
    assert!(
        text.contains("FR182")
            && (text.contains("产品钉")
                || text.contains("product-pin")
                || text.contains("product pin")),
        "must cover FR182 unpaired product-pin"
    );
    assert!(
        text.contains("1.159.0")
            && (text.contains("无") || text.contains("unpaired") || text.contains("例外")),
        "must pin target product version 1.159.0 as unpaired"
    );
    assert!(
        text.contains("AD-9")
            && (text.contains("unpaired product-pin")
                || text.contains("unpaired product")
                || text.contains("例外"))
            && (text.contains("NFR85") || text.contains("修订")),
        "must require AD-9 unpaired product-pin exception (NFR85)"
    );
    assert!(
        text.contains("FR174")
            && text.contains("FR179")
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid FR174/FR179 alone closing FR182"
    );
    assert!(
        text.contains("115.2")
            && text.contains("115.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 115.2–115.3"
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
fn nfr14_epic115_gates_115_2_until_115_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let s115_1 = yaml_value_for_key(&sprint, "115-1-epic-115-nfr14-风险记录").expect("115-1 key");
    if s115_1 != "done" {
        for key in [
            "115-2-unpaired-firtool-产品钉再升钉实现与验收-fr182",
            "115-3-fr182-收口与文档指针",
        ] {
            let st = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                st, "backlog",
                "{key} must stay backlog until 115-1 done (got {st})"
            );
        }
    }
    assert!(
        sprint.contains("epic-111: done") || sprint.contains("epic-111:done"),
        "Epic 111 must be done before Epic 115 NFR14"
    );
    assert!(
        sprint.contains("epic-112: done") || sprint.contains("epic-112:done"),
        "Epic 112 soft-serial before Epic 115 AD-9 touch"
    );
}
