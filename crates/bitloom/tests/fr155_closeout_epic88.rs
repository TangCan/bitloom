//! ATDD / guardrail: Story 88.4 / FR155 — Epic 88 closeout + docs pointers.
//!
//! ```text
//! cargo test -p bitloom --test fr155_closeout_epic88
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
    let path = workspace_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn fr155_readme_marks_fr155_closed() {
    let readme = read("README.md");
    assert!(
        readme.contains("FR155")
            && (readme.contains("已关闭") || readme.contains("Epic 88"))
            && (readme.contains("bitloom-lsp") || readme.contains("cargo install bitloom-lsp")),
        "README must mark FR155 / Epic 88 closed with install path"
    );
    assert!(
        readme.contains("FR154") && readme.contains("已关闭"),
        "README must keep FR154 gate closed"
    );
    assert!(
        readme.contains("FR157")
            && (readme.contains("未关闭前不得宣称") || readme.contains("不得宣称")),
        "README must keep NFR59 deepen rows unclaimed"
    );
    assert!(
        readme.contains("FR156") && (readme.contains("98") || readme.contains("宣称")),
        "README must keep FR156 claim gate pointing at Epic 98"
    );
    assert!(readme.contains("Bitloom") && readme.contains("bitloom"));
}

#[test]
fn fr155_deferred_marks_epic88_closed() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Epic 88")
            && deferred.contains("FR155")
            && (deferred.contains("已关闭") || deferred.contains("Story 88.4")),
        "deferred Phase 19 pointer must mark Epic 88 / FR155 closed"
    );
    assert!(
        deferred.contains("FR157") && deferred.contains("Epic 89"),
        "deferred must point remaining NFR59 to Epic 89+"
    );
    assert!(
        deferred.contains("FR156") && deferred.contains("98"),
        "deferred must keep FR156 / Epic 98 claim discipline"
    );
}

#[test]
fn fr155_agents_and_spine_fr155_closed() {
    let agents = read("AGENTS.md");
    assert!(
        agents.contains("Epic 88")
            && agents.contains("FR155")
            && (agents.contains("closed") || agents.contains("已关闭")),
        "AGENTS must note Epic 88 / FR155 closed"
    );
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("Epic 88")
            && (spine.contains("已关闭") || spine.contains("88.4"))
            && spine.contains("bitloom-lsp"),
        "spine must note Epic 88 / FR155 closed"
    );
    assert!(
        spine.contains("NFR68") && (spine.contains("仍有效") || spine.contains("不得")),
        "spine must keep NFR68 isolation"
    );
}

#[test]
fn fr155_nfr14_epic88_close_conditions_checked() {
    let text = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic88-bitloom-lsp-fr152a-fr155.md",
    );
    for needle in [
        "- [x] **FR155 可发布化 + 政策 (a)**",
        "- [x] **live `cargo publish -p bitloom-lsp`**",
        "- [x] **文档 / deferred 收口**",
        "- [x] **NFR68/70/72：**",
        "- [x] **品牌 / AD-2：**",
        "- [x] **Epic 89–98：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 88 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed — Story 88.4"),
        "NFR14 risk record must be closed after Story 88.4"
    );
}

#[test]
fn fr155_sprint_epic88_done_89_ready() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-88: done") || sprint.contains("epic-88:done"),
        "epic-88 must be done after Story 88.4"
    );
    assert!(
        sprint.contains("88-4-fr155-收口与文档指针: done")
            || sprint.contains("88-4-fr155-收口与文档指针:done"),
        "88-4 must be done"
    );
    assert!(
        sprint.contains("89-1-epic-89-nfr14-风险记录: ready-for-dev")
            || sprint.contains("89-1-epic-89-nfr14-风险记录:ready-for-dev")
            || sprint.contains("89-1-epic-89-nfr14-风险记录: done")
            || sprint.contains("89-1-epic-89-nfr14-风险记录: in-progress"),
        "89-1 must be ready-for-dev (or further) after Epic 88 closes"
    );
}

#[test]
fn fr155_epics_phase19_epic88_complete() {
    let epics = read("_agile-output/planning-artifacts/epics.md");
    assert!(
        epics.contains("phase19Epic88Status: complete")
            || epics.contains("phase19Epic88Status:complete"),
        "epics.md must stamp phase19Epic88Status complete"
    );
}

#[test]
fn fr155_policy_and_publish_docs_present() {
    let policy = read("docs/fr152-bitloom-lsp-publish-policy.md");
    assert!(
        policy.contains("(a)") && (policy.contains("Selected") || policy.contains("**Selected**")),
        "fr152 policy must remain (a)"
    );
    let fr155 = read("docs/fr155-bitloom-lsp-publish.md");
    assert!(
        fr155.contains("Published bitloom-lsp")
            || fr155.contains("Live `cargo publish -p bitloom-lsp`"),
        "fr155 doc must retain live publish evidence"
    );
}
