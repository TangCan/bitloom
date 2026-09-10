//! ATDD Story 61.3 — FR120 / Epic 61 closeout + FR98/FR108 cross-link honesty.
//!
//! ```text
//! cargo test -p bitloom --test fr120_epic61_closeout
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
fn fr120_nfr14_epic61_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic61-commercial-vip-gpio.md");
    for needle in [
        "- [x] **61.2 / FR120：",
        "- [x] **文档 / deferred / IP README / 未覆盖协议诚实披露",
        "- [x] **禁止事项未触发",
        "- [x] **品牌 / 依赖：",
        "- [x] **FR98 / FR108 关闭仍有效",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 61 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 61.3") || text.contains("61.3")),
        "NFR14 must be closed with Story 61.3 pointer"
    );
}

#[test]
fn fr120_docs_readme_deferred_closed() {
    let fr120 = read("docs/fr120-commercial-vip-gpio.md");
    let ip = read("docs/ip/README.md");
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        (fr120.contains("closed") || fr120.contains("已关闭"))
            && (fr120.contains("Epic 61") || fr120.contains("61.3")),
        "fr120 doc must declare Epic 61 / FR120 closed"
    );
    assert!(
        ip.contains("FR120")
            && (ip.contains("已关闭")
                || ip.contains("61.3")
                || ip.contains("Epic 61")
                || ip.contains("GpioVip")),
        "docs/ip must declare FR120 / GpioVip"
    );
    assert!(
        readme.contains("FR120")
            && (readme.contains("Epic 61 已关闭") || readme.contains("FR120 / Epic 61 已关闭")),
        "README must note FR120 / Epic 61 closed"
    );
    assert!(
        deferred.contains("FR120")
            && (deferred.contains("已关闭") || deferred.contains("closed"))
            && (deferred.contains("61.3") || deferred.contains("Epic 61")),
        "deferred must close FR120 item"
    );
    assert!(
        epics.contains("phase14Epic61Status: complete"),
        "epics.md must stamp Epic 61 complete"
    );
    // Honest uncovered protocols
    assert!(
        (fr120.contains("全 SoC")
            || fr120.contains("SoC pad")
            || fr120.contains("debounce")
            || fr120.contains("NFR51"))
            && fr120.contains("deferred"),
        "must honestly disclose uncovered protocols as deferred"
    );
    assert!(
        deferred.contains("FR122")
            && (deferred.contains("已关闭")
                || deferred.contains("closed")
                || deferred.contains("Epic 63")
                || deferred.contains("仍 deferred")),
        "deferred must still track FR122 (closed or deferred)"
    );
}

#[test]
fn fr120_fr98_fr108_mvp_still_valid() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-43: done") || sprint.contains("epic-43:done"),
        "FR98 / Epic 43 must remain closed (NFR48)"
    );
    assert!(
        sprint.contains("epic-50: done") || sprint.contains("epic-50:done"),
        "FR108 / Epic 50 must remain closed (NFR48)"
    );
    let ip = read("docs/ip/README.md");
    assert!(
        ip.contains("FR108") && ip.contains("FR98"),
        "docs/ip must keep FR98/FR108 cross-links"
    );
}

#[test]
fn fr120_sprint_epic61_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("61-3-fr120-收口与文档指针: done")
            || sprint.contains("61-3-fr120-收口与文档指针:done")
    );
    assert!(sprint.contains("epic-61: done") || sprint.contains("epic-61:done"));
    assert!(
        sprint.contains("61-2-商业-vip-gpio-实现与验收-fr120: done")
            || sprint.contains("61-2-商业-vip-gpio-实现与验收-fr120:done")
    );
    assert!(
        sprint.contains("61-1-epic-61-nfr14-风险记录: done")
            || sprint.contains("61-1-epic-61-nfr14-风险记录:done")
    );
    assert!(
        sprint.contains("epic-62: backlog")
            || sprint.contains("epic-62:backlog")
            || sprint.contains("epic-62: in-progress")
            || sprint.contains("epic-62:in-progress")
            || sprint.contains("epic-62: done")
            || sprint.contains("epic-62:done"),
        "epic-62 must be backlog, in-progress, or done"
    );
    if !sprint.contains("epic-62: backlog") && !sprint.contains("epic-62:backlog") {
        assert!(
            sprint.contains("62-1-epic-62-nfr14-风险记录: done")
                || sprint.contains("62-1-epic-62-nfr14-风险记录:done"),
            "leaving epic-62 backlog requires Story 62.1 NFR14 done (gate)"
        );
    }
    assert!(
        sprint.contains("epic-63: backlog")
            || sprint.contains("epic-63:backlog")
            || sprint.contains("epic-63: in-progress")
            || sprint.contains("epic-63:in-progress")
            || sprint.contains("epic-63: done")
            || sprint.contains("epic-63:done"),
        "epic-63 must be backlog, in-progress, or done"
    );
    if !sprint.contains("epic-63: backlog") && !sprint.contains("epic-63:backlog") {
        assert!(
            sprint.contains("63-1-epic-63-nfr14-风险记录: done")
                || sprint.contains("63-1-epic-63-nfr14-风险记录:done"),
            "leaving epic-63 backlog requires Story 63.1 NFR14 done (gate)"
        );
    }
}
