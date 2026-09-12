//! ATDD / guardrail: Epic 95 NFR14 risk record for FR163
//! unlisted-protocol handwritten FL (Story 95.1 / AD-28 / NFR68–72).

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
fn nfr14_risk_epic95_unlisted_protocol_fl_fr163_has_required_fields() {
    let path = workspace_root().join(
        "_agile-output/implementation-artifacts/nfr14-risk-epic95-unlisted-protocol-handwritten-fl-fr163.md",
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
        && !text.contains("closed — Story 95.3");
    let status_closed = text.contains("closed — Story 95.3")
        || (text.contains("closed") && text.contains("Epic 95") && text.contains("可宣称"));
    assert!(status_open || status_closed);

    let sprint = fs::read_to_string(
        workspace_root().join("_agile-output/implementation-artifacts/sprint-status.yaml"),
    )
    .expect("sprint");
    let epic95_done = sprint.contains("epic-95: done") || sprint.contains("epic-95:done");
    if !epic95_done {
        assert!(
            status_open,
            "before epic-95 done, risk record must stay open"
        );
    }

    assert!(
        text.contains("NFR68")
            && text.contains("FR135")
            && (text.contains("不得") || text.contains("仍有效")),
        "must state NFR68 / FR135 isolation"
    );
    assert!(
        text.contains("FR163") && (text.contains("UartRx") || text.contains("协议")),
        "must cover FR163 and selected protocol"
    );
    assert!(
        text.contains("UartRx")
            && (text.contains("选定") || text.contains("MVP") || text.contains("手写")),
        "must nail UartRx as selected MVP"
    );
    assert!(
        (text.contains("Spi") || text.contains("SPI"))
            && (text.contains("不做") || text.contains("新合同") || text.contains("NFR71")),
        "must defer SPI explicitly"
    );
    assert!(
        (text.contains("I2c") || text.contains("I2C"))
            && (text.contains("不做") || text.contains("新合同") || text.contains("NFR71")),
        "must defer I2C explicitly"
    );
    assert!(
        (text.contains("Axi") || text.contains("AXI"))
            && (text.contains("不做") || text.contains("新合同") || text.contains("NFR71")),
        "must defer AXI explicitly"
    );
    assert!(
        (text.contains("验收") || text.contains("谓词") || text.contains("ATDD"))
            && (text.contains("tick") || text.contains("FL") || text.contains("失败")),
        "must nail acceptance / FL≡tick semantics"
    );
    assert!(
        text.contains("bitloom-prelude") || text.contains("AD-6"),
        "must nail design-crate prelude boundary"
    );
    assert!(
        text.contains("bitloom-sim") || text.contains("IpDualModel"),
        "must place handwritten FL in sim side"
    );
    assert!(
        text.contains("NFR71")
            && (text.contains("静默") || text.contains("扩大") || text.contains("子集")),
        "must forbid silent subset expand"
    );
    assert!(
        (text.contains("FR135") || text.contains("UartTx") || text.contains("alone"))
            && (text.contains("不得") || text.contains("冒充") || text.contains("勾选")),
        "must forbid FR135-alone closing FR163"
    );
    assert!(
        text.contains("95.2")
            && text.contains("95.3")
            && (text.contains("ready") || text.contains("`ready`"))
            && (text.contains("不得") || text.contains("缺")),
        "must gate 95.2–95.3 behind this NFR14"
    );
    assert!(
        text.contains("ip/") || text.contains("`ip/`") || text.contains("软序"),
        "must nail ip/ soft-order / layout boundary"
    );
    assert!(text.contains("Richard") && text.contains("NFR14"));
    assert!(text.contains("NFR14-crates"));
    assert!(text.contains("Bitloom") || text.contains("bitloom"));

    let s95_1 = yaml_value_for_key(&sprint, "95-1-epic-95-nfr14-风险记录").expect("95-1");
    let later = [
        "95-2-未列协议手写-fl-实现与验收-fr163",
        "95-3-fr163-收口与文档指针",
    ];
    if s95_1 != "done" {
        for key in later {
            let status = yaml_value_for_key(&sprint, key).unwrap_or_default();
            assert_eq!(
                status, "backlog",
                "{key} must stay backlog until 95-1 done (got {status})"
            );
        }
    }
    assert!(
        sprint.contains("epic-87: done") || sprint.contains("epic-87:done"),
        "Epic 87 must be done before Epic 95 NFR14"
    );
}
