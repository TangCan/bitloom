//! ATDD Story 46.3 — FR101 / Epic 46 closeout.
//!
//! Locks NFR14 close checkboxes, README/deferred honesty (「不承诺 SystemC TLM」
//! is not a product completion exclusion; Epic 46 / FR101 closed — LT-only MVP;
//! AT deferred), docs/fr101, and sprint `epic-46: done`. Epic 47 must remain backlog.
//!
//! ```text
//! cargo test -p bitloom --test fr101_epic46_closeout
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
fn fr101_nfr14_epic46_close_conditions_checked() {
    let text = read("_agile-output/implementation-artifacts/nfr14-risk-epic46-systemc-tlm.md");
    for needle in [
        "- [x] **46.2 / FR101：**",
        "- [x] **文档 / deferred / doc-19：**",
        "- [x] **禁止事项未触发：**",
        "- [x] **修订后 AD-5 / NFR41：**",
        "- [x] **品牌 / 依赖：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 46 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 46.3") || text.contains("46.3")),
        "NFR14 Epic 46 record must be closed with Story 46.3 pointer"
    );
}

#[test]
fn fr101_deferred_readme_epic46_closed() {
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    let agents = read("AGENTS.md");

    let fr93_start = readme
        .find("永久非目标")
        .expect("README must retain historical 永久非目标 section");
    let block = &readme[fr93_start..];
    let end = block
        .find("\n## ")
        .or_else(|| block.find("\n详见"))
        .unwrap_or(block.len().min(4000));
    let section = &block[..end];
    assert!(
        section.contains("已推翻")
            || section.contains("已被")
            || section.contains("Phase 12")
            || section.contains("FR94"),
        "FR93 section must remain overturn / Phase 12 framed"
    );
    assert!(
        section.contains("FR101")
            && (section.contains("已关闭")
                || section.contains("已交付")
                || section.contains("Epic 46 已")
                || section.contains("Epic 46 closed")),
        "README FR93 / Epic 46 mapping must note FR101 / Epic 46 closed"
    );

    assert!(
        deferred.contains("FR101")
            && (deferred.contains("Epic 46") || deferred.contains("epic-46"))
            && (deferred.contains("已关闭")
                || deferred.contains("已交付")
                || deferred.contains("closed")),
        "deferred-work must note Epic 46 / FR101 closed"
    );
    assert!(
        agents.contains("FR101")
            && (agents.contains("Epic 46 closed")
                || agents.contains("Epic 46 **closed**")
                || agents.to_lowercase().contains("epic 46 closed")),
        "AGENTS.md must note Epic 46 / FR101 closed"
    );

    // Must not leave 「关闭前不得宣称」tied to FR101 without a closed note nearby
    for (name, text) in [("README", &readme), ("deferred", &deferred)] {
        if text.contains("关闭前不得宣称") && text.contains("FR101") {
            let idx = text.find("FR101").expect("FR101");
            let window = &text[idx.saturating_sub(200)..(idx + 400).min(text.len())];
            assert!(
                window.contains("已关闭")
                    || window.contains("已交付")
                    || window.contains("closed")
                    || window.contains("Epic 46 已"),
                "{name}: FR101 near 「关闭前不得宣称」must also note closed"
            );
        }
    }
}

#[test]
fn fr101_docs_epic46_closed_lt_only() {
    let fr101 = read("docs/fr101-systemc-tlm.md");
    assert!(
        fr101.contains("FR101") && fr101.contains("Bitloom"),
        "fr101 doc must name FR101 + Bitloom"
    );
    assert!(
        (fr101.contains("closed") || fr101.contains("关闭") || fr101.contains("已关闭"))
            && (fr101.contains("46.3") || fr101.contains("Epic 46")),
        "fr101 doc must record Story 46.3 / Epic 46 completion"
    );
    assert!(
        fr101.contains("LT-only") || fr101.contains("LT only") || fr101.contains("loosely-timed"),
        "must keep LT-only honesty"
    );
    assert!(
        fr101.contains("FR47")
            && (fr101.contains("≠")
                || fr101.contains("Not")
                || fr101.contains("not")
                || fr101.contains("不是")),
        "must contrast FR47 ≠ SystemC TLM"
    );
    // AT must remain deferred / out of MVP — do not claim AT delivered as closeout
    let lower = fr101.to_lowercase();
    if lower.contains("nb_transport") {
        assert!(
            fr101.contains("Out of MVP")
                || fr101.contains("deferred")
                || fr101.contains("非目标")
                || fr101.contains("不交付"),
            "if nb_transport is mentioned, AT must stay deferred / out of MVP"
        );
    }
}

#[test]
fn fr101_sprint_epic46_done_epic47_backlog() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-46: done") || sprint.contains("epic-46:done"),
        "sprint must mark epic-46 done"
    );
    assert!(
        sprint.contains("46-3-fr101-收口与撤销-不承诺-tlm: done")
            || sprint.contains("46-3-fr101-收口与撤销-不承诺-tlm:done"),
        "sprint must mark story 46-3 done"
    );
    assert!(
        sprint.contains("46-2-systemc-tlm-2-0-产品面-fr101: done")
            || sprint.contains("46-2-systemc-tlm-2-0-产品面-fr101:done"),
        "sprint must keep 46-2 done"
    );
    assert!(
        sprint.contains("epic-47: backlog"),
        "closeout must leave epic-47: backlog (do not start Epic 47)"
    );
}
