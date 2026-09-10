//! ATDD Story 45.4 — FR103 / Epic 45 closeout.
//!
//! Locks NFR14 close checkboxes, README/deferred honesty (default TLM≡CA /
//! automatic formal equiv is not a permanent non-goal; Epic 45 / FR100+FR103
//! closed), docs/fr103, and sprint `epic-45: done`. Epic 46 may leave backlog only
//! after Story 46.1 NFR14; Epic 47 must remain backlog.
//!
//! ```text
//! cargo test -p bitloom --test fr103_epic45_closeout
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
fn fr103_nfr14_epic45_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic45-formal-equiv-dual-model.md");
    for needle in [
        "- [x] **45.2 / FR100：**",
        "- [x] **45.3 / FR102：**",
        "- [x] **45.4 / FR103：**",
        "- [x] **文档 / deferred：**",
        "- [x] **禁止事项未触发：**",
        "- [x] **品牌 / 依赖：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 45 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 45.4") || text.contains("45.4")),
        "NFR14 Epic 45 record must be closed with Story 45.4 pointer"
    );
}

#[test]
fn fr103_deferred_readme_epic45_closed() {
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
        section.contains("FR100")
            && (section.contains("已关闭")
                || section.contains("已交付")
                || section.contains("Epic 45 已")
                || section.contains("Epic 45 closed")),
        "README FR93#3 / Epic 45 mapping must note FR100 / Epic 45 closed"
    );
    assert!(
        (section.contains("TLM≡CA") || section.contains("形式")) && section.contains("FR100"),
        "README FR93#3 must still map TLM≡CA / formal → FR100"
    );

    assert!(
        deferred.contains("FR100")
            && (deferred.contains("Epic 45") || deferred.contains("epic-45"))
            && (deferred.contains("已关闭")
                || deferred.contains("已交付")
                || deferred.contains("closed")),
        "deferred-work must note Epic 45 / FR100 closed"
    );
    assert!(
        deferred.contains("FR103") || deferred.contains("双模型") || deferred.contains("FR102"),
        "deferred must mention dual-model / FR103 close context"
    );
}

#[test]
fn fr103_sprint_epic45_done_epic46_gated_epic47_gated() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-45: done") || sprint.contains("epic-45:done"),
        "sprint must mark epic-45 done"
    );
    assert!(
        sprint.contains("45-4-一级-ip-双模型齐全-epic-45-收口-fr103: done")
            || sprint.contains("45-4-一级-ip-双模型齐全-epic-45-收口-fr103:done")
            || sprint.contains("45-4-一级-ip-双模型齐全-epic45-收口-fr103: done")
            || sprint.contains("45-4-一级-ip-双模型齐全-epic45-收口-fr103:done"),
        "sprint must mark story 45-4 done"
    );
    // Epic 46: backlog, or in-progress/done only after 46.1 NFR14 gate
    assert!(
        sprint.contains("epic-46: backlog")
            || sprint.contains("epic-46: in-progress")
            || sprint.contains("epic-46: done"),
        "sprint-status must list epic-46 as backlog, in-progress, or done"
    );
    if !sprint.contains("epic-46: backlog") {
        assert!(
            sprint.contains("46-1-epic-46-nfr14-风险记录: done"),
            "leaving epic-46 backlog requires Story 46.1 NFR14 done (gate)"
        );
    }
    // Epic 47: backlog, or in-progress/done only after 47.1 NFR14 gate
    assert!(
        sprint.contains("epic-47: backlog")
            || sprint.contains("epic-47: in-progress")
            || sprint.contains("epic-47: done"),
        "sprint-status must list epic-47 as backlog, in-progress, or done"
    );
    if !sprint.contains("epic-47: backlog") {
        assert!(
            sprint.contains("47-1-epic-47-nfr14-风险记录: done"),
            "leaving epic-47 backlog requires Story 47.1 NFR14 done (gate)"
        );
    }
}

#[test]
fn fr103_docs_ip_and_fr103_cross_links() {
    let fr103 = read("docs/fr103-ip-dual-model.md");
    assert!(
        fr103.contains("45.4")
            && (fr103.contains("closed")
                || fr103.contains("关闭")
                || fr103.contains("Epic 45")
                || fr103.contains("完成面")),
        "fr103 doc must record Story 45.4 / Epic 45 completion"
    );
    let ip = read("docs/ip/README.md");
    assert!(
        ip.contains("fr103") || ip.contains("FR103"),
        "docs/ip must cross-link FR103 dual-model"
    );
}
