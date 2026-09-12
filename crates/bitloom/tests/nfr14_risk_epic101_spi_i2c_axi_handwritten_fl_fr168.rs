//! ATDD / guardrail: Epic 101 NFR14 risk record for FR168
//! SPI+I2C+AXI handwritten FL (Story 101.1 / AD-28 / NFR73–77).

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
fn nfr14_risk_epic101_spi_i2c_axi_handwritten_fl_fr168_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic101-spi-i2c-axi-handwritten-fl-fr168.md",
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
        && !text.contains("closed — Story 101.3");
    let status_closed = text.contains("closed — Story 101.3")
        || (text.contains("closed") && text.contains("Epic 101") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let epic101_done = sprint.contains("epic-101: done") || sprint.contains("epic-101:done");
    if !epic101_done {
        assert!(
            status_open,
            "before epic-101 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR73")
            && (text.contains("FR163") || text.contains("FR135") || text.contains("FR126"))
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR73 / prior FL isolation"
    );
    assert!(
        text.contains("FR168")
            && text.contains("SpiMaster")
            && text.contains("I2cMaster")
            && text.contains("Axi4LiteSlave"),
        "must nail all three SPI/I2C/AXI targets"
    );
    assert!(
        text.contains("三者") || text.contains("皆须") || text.contains("皆交付"),
        "must require all three SPI+I2C+AXI"
    );
    assert!(
        text.contains("禁止") && text.contains("至少一项"),
        "must explicitly forbid至少一项交差"
    );
    assert!(
        (text.contains("FR163") || text.contains("UartRx"))
            && (text.contains("alone") || text.contains("不得") || text.contains("勾选")),
        "must forbid FR163-alone closing FR168"
    );
    assert!(
        text.contains("NFR76")
            && (text.contains("静默") || text.contains("扩大") || text.contains("子集")),
        "must forbid silent subset expand"
    );
    assert!(
        text.contains("101.2")
            && text.contains("101.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "must gate 101.2–101.3"
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

    let s101_1 = yaml_value_for_key(&sprint, "101-1-epic-101-nfr14-风险记录").expect("101-1 key");
    if s101_1 != "done" {
        for key in [
            "101-2-spi-i2c-axi-手写-fl-实现与验收-fr168",
            "101-3-fr168-收口与文档指针",
        ] {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 101-1 done (got {status})"
            );
        }
    }
    assert!(
        sprint.contains("epic-99: done") || sprint.contains("epic-99:done"),
        "Epic 99 must be done before Epic 101 NFR14"
    );
}
