//! ATDD / guardrail: Epic 122 NFR14 risk record for FR189
//! further firtool product-pin beyond FR182 1.159.0 (Story 122.1).

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
fn nfr14_risk_epic122_further_firtool_product_pin_fr189_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic122-further-firtool-product-pin-fr189.md",
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
        && (text.contains("open")
            || text.contains("in-progress")
            || text.contains("进行中")
            || text.contains("deferred")
            || text.contains("未交付")
            || text.contains("parked"))
        && !text.contains("closed — Story 122.3");
    let status_closed = text.contains("closed — Story 122.3")
        || (text.contains("closed")
            && text.contains("Epic 122")
            && text.contains("可宣称")
            && text.contains("已交付"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    // `deferred` is NOT delivery-done; risk record must stay non-delivered.
    if sprint.contains("epic-122: done") || sprint.contains("epic-122:done") {
        assert!(
            status_closed,
            "epic-122 done must mean delivered close (Story 122.3)"
        );
    } else {
        assert!(
            status_open,
            "before epic-122 delivery-done, risk record must stay open/deferred/not-delivered"
        );
    }

    assert!(
        text.contains("NFR88")
            && text.contains("FR182")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR88 / FR182 isolation"
    );
    assert!(
        text.contains("FR189")
            && (text.contains("firtool") || text.contains("产品钉") || text.contains("product")),
        "must cover FR189 firtool product-pin"
    );
    assert!(
        text.contains("1.159.0")
            && (text.contains(">")
                || text.contains("大于")
                || text.contains("再升")
                || text.contains("超")),
        "must pin target beyond FR182 1.159.0"
    );
    assert!(
        (text.contains("FR182") || text.contains("FR186"))
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid FR182/FR186 alone closing FR189"
    );
    assert!(
        text.contains("AD-9") && (text.contains("NFR90") || text.contains("修订")),
        "must require AD-9 revise (NFR90)"
    );
    assert!(
        text.contains("122.2")
            && text.contains("122.3")
            && (text.contains("ready") || text.contains("`ready`")),
        "must gate 122.2–122.3"
    );
    assert!(text.contains("负责人") && text.contains("Richard") && text.contains("NFR14"));
    assert!(
        (text.contains("git push") || text.contains("`git push`"))
            && (text.contains("不是") || text.contains("非") || text.contains("不得"))
    );
    assert!(
        text.contains("NFR91")
            && (text.contains("子集") || text.contains("另开") || text.contains("静默")),
        "must cite NFR91"
    );
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(text.contains("bitloom-prelude") || text.contains("bitloom_prelude"));
}

#[test]
fn nfr14_epic122_gates_122_2_until_122_1_done() {
    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    assert!(
        sprint.contains("epic-118: done") || sprint.contains("epic-118:done"),
        "Epic 118 must be done before Epic 122 NFR14"
    );
    let s122_1 = yaml_value_for_key(&sprint, "122-1-epic-122-nfr14-风险记录").expect("122-1 key");
    if s122_1 != "done" {
        for key in [
            "122-2-继续-firtool-产品钉升钉实现与验收-fr189",
            "122-3-fr189-收口与文档指针",
        ] {
            let st = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                st, "backlog",
                "{key} must stay backlog until 122-1 done (got {st})"
            );
        }
    }
}
