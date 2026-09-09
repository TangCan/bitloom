//! ATDD Story 43.5 — FR98 Epic 43 closeout.
//!
//! Locks NFR14 close checkboxes, README/deferred honesty (VIP-level full-protocol
//! IP is not a permanent non-goal; Epic 43 / FR98 closed), docs/ip boundaries,
//! and sprint `epic-43: done`. Epic 45–47 must remain backlog.
//! Epic 44 may be `in-progress` only after Story 44.1 NFR14 is `done`.
//!
//! ```text
//! cargo test -p bitloom --test fr98_epic43_closeout
//! ```

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr98_nfr14_epic43_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic43-vip-full-protocol-ip.md");
    for needle in [
        "- [x] **UART：**",
        "- [x] **SPI：**",
        "- [x] **I2C：**",
        "- [x] **AXI：**",
        "- [x] **文档 / deferred：**",
        "- [x] **禁止事项未触发：**",
        "- [x] **品牌 / 依赖：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 43 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 43.5") || text.contains("43.5")),
        "NFR14 Epic 43 record must be closed with Story 43.5 pointer"
    );
}

#[test]
fn fr98_deferred_readme_epic43_closed() {
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");

    let fr93_start = readme
        .find("永久非目标")
        .expect("README must retain historical 永久非目标 section");
    let block = &readme[fr93_start..];
    let end = block
        .find("\n## ")
        .or_else(|| block.find("\n详见"))
        .unwrap_or(block.len().min(3500));
    let section = &block[..end];
    assert!(
        section.contains("已推翻")
            || section.contains("已被")
            || section.contains("Phase 12")
            || section.contains("FR94"),
        "FR93 section must remain overturn / Phase 12 framed"
    );
    assert!(
        section.contains("FR98")
            && (section.contains("已关闭")
                || section.contains("已交付")
                || section.contains("Epic 43 已")
                || section.contains("Epic 43 closed")),
        "README FR93#4 / Epic 43 mapping must note FR98 / Epic 43 closed"
    );
    assert!(
        (section.contains("VIP") || section.contains("全协议")) && section.contains("FR98"),
        "README FR93#4 must still map VIP / full-protocol IP → FR98"
    );

    assert!(
        deferred.contains("FR98")
            && (deferred.contains("Epic 43") || deferred.contains("epic-43"))
            && (deferred.contains("已关闭")
                || deferred.contains("已交付")
                || deferred.contains("closed")),
        "deferred-work must note Epic 43 / FR98 closed"
    );
}

#[test]
fn fr98_sprint_epic43_done_epic44_gated() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-43: done") || sprint.contains("epic-43:done"),
        "sprint must mark epic-43 done"
    );
    assert!(
        sprint.contains("43-5-axi-可选-gpio-fr98-收口: done")
            || sprint.contains("43-5-axi-可选-gpio-fr98-收口:done"),
        "sprint must mark story 43-5 done"
    );
    // Epic 44: backlog, or in-progress/done only after 44.1 NFR14 gate
    assert!(
        sprint.contains("epic-44: backlog")
            || sprint.contains("epic-44: in-progress")
            || sprint.contains("epic-44: done"),
        "sprint-status must list epic-44 as backlog, in-progress, or done"
    );
    if !sprint.contains("epic-44: backlog") {
        assert!(
            sprint.contains("44-1-epic-44-nfr14-风险记录: done"),
            "leaving epic-44 backlog requires Story 44.1 NFR14 done (gate)"
        );
    }
    // Must not have started Epic 45+
    for key in ["epic-45: backlog", "epic-46: backlog", "epic-47: backlog"] {
        assert!(
            sprint.contains(key),
            "closeout must leave {key} (do not start Epic 45+)"
        );
    }
}

#[test]
fn fr98_docs_ip_four_class_near_vip_closed() {
    let ip = read("docs/ip/README.md");
    assert!(
        ip.contains("UartRx") && ip.contains("SpiMaster") && ip.contains("I2cMaster"),
        "docs/ip must still list UART/SPI/I2C near-VIP types"
    );
    assert!(
        ip.contains("Axi4LiteSlave") && (ip.contains("FR98") || ip.contains("近 VIP")),
        "docs/ip must list AXI FR98 near-VIP"
    );
    // Should not still say four-class not green / wait for 43.5 on AXI row as open work
    let axi_blob = ip
        .split("**AXI**")
        .nth(1)
        .unwrap_or("")
        .split('\n')
        .next()
        .unwrap_or("");
    assert!(
        !axi_blob.contains("→ 43.5") && !axi_blob.contains("仍属 43.5"),
        "AXI docs row must not still defer near-VIP to open Story 43.5"
    );
}
