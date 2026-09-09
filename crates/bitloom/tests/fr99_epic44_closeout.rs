//! ATDD Story 44.4 — FR99 Epic 44 closeout.
//!
//! Locks NFR14 close checkboxes, README/deferred honesty (keystroke full-elaborate
//! LSP is not a permanent non-goal / Path B completion bar; Epic 44 / FR99 closed),
//! FR90 still available but not a substitute, and sprint `epic-44: done`.
//! Epic 45–47 must remain backlog.
//!
//! ```text
//! cargo test -p bitloom --test fr99_epic44_closeout
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
fn fr99_nfr14_epic44_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic44-full-elaborate-lsp.md");
    for needle in [
        "- [x] **44.2：**",
        "- [x] **44.3：**",
        "- [x] **文档 / deferred：**",
        "- [x] **禁止事项未触发：**",
        "- [x] **品牌 / 依赖：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 44 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 44.4") || text.contains("44.4")),
        "NFR14 Epic 44 record must be closed with Story 44.4 pointer"
    );
}

#[test]
fn fr99_deferred_readme_epic44_closed() {
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
        section.contains("FR99")
            && (section.contains("已关闭")
                || section.contains("已交付")
                || section.contains("Epic 44 已")
                || section.contains("Epic 44 closed")),
        "README FR93#5 / Epic 44 mapping must note FR99 / Epic 44 closed"
    );
    assert!(
        (section.contains("LSP") || section.contains("elaborate")) && section.contains("FR99"),
        "README FR93#5 must still map keystroke full-elaborate LSP → FR99"
    );

    assert!(
        deferred.contains("FR99")
            && (deferred.contains("Epic 44") || deferred.contains("epic-44"))
            && (deferred.contains("已关闭")
                || deferred.contains("已交付")
                || deferred.contains("closed")),
        "deferred-work must note Epic 44 / FR99 closed"
    );
    // Path B defer must not remain the Phase 12 completion bar
    let item52 = deferred
        .split("epic-23-retro-item-52")
        .nth(1)
        .unwrap_or("")
        .split("- source_spec:")
        .next()
        .unwrap_or("");
    assert!(
        item52.contains("closed")
            || item52.contains("已关闭")
            || item52.contains("Story 44.4")
            || item52.contains("44.4"),
        "deferred epic-23-retro-item-52 must close FR99 / Epic 44 (not leave Path B as completion bar)"
    );
}

#[test]
fn fr99_docs_revoke_path_b_completion_narrative() {
    let fr99 = read("docs/fr99-bitloom-lsp.md");
    assert!(
        fr99.contains("44.4")
            && (fr99.contains("closed")
                || fr99.contains("已关闭")
                || fr99.contains("done")
                || fr99.contains("Epic 44")),
        "fr99 doc must record Story 44.4 / Epic 44 closeout"
    );
    assert!(
        !fr99.contains("**not yet**"),
        "fr99 must not leave 44.4 as open 'not yet'"
    );

    let fr38 = read("docs/fr38-viz-lsp.md");
    assert!(
        (fr38.contains("FR99") || fr38.contains("bitloom-lsp"))
            && (fr38.contains("closed")
                || fr38.contains("已关闭")
                || fr38.contains("44.4")
                || fr38.contains("Epic 44")),
        "fr38 must note FR99 / Epic 44 closed (not pending 44.4 only)"
    );
    assert!(
        fr38.contains("HTML")
            && (fr38.contains("≠") || fr38.contains("not") || fr38.contains("不是")),
        "fr38 must keep HTML ≠ LSP honesty"
    );

    let fr90 = read("docs/fr90-host-ide-rust-analyzer.md");
    assert!(
        fr90.contains("FR99")
            && (fr90.contains("不替代")
                || fr90.contains("不 替代")
                || fr90.contains("not a substitute")
                || fr90.contains("does not substitute")
                || fr90.contains("≠")
                || fr90.contains("alone")),
        "fr90 must state host path does not substitute for FR99"
    );
    assert!(
        fr90.contains("bitloom-lsp") || fr90.contains("FR99"),
        "fr90 must cross-link delivered Bitloom LSP / FR99"
    );
}

#[test]
fn fr99_sprint_epic44_done_epic45_plus_backlog() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-44: done") || sprint.contains("epic-44:done"),
        "sprint must mark epic-44 done"
    );
    assert!(
        sprint.contains("44-4-fr99-收口与撤销-lsp-非目标: done")
            || sprint.contains("44-4-fr99-收口与撤销-lsp-非目标:done"),
        "sprint must mark story 44-4 done"
    );
    for key in ["epic-45: backlog", "epic-46: backlog", "epic-47: backlog"] {
        assert!(
            sprint.contains(key),
            "closeout must leave {key} (do not start Epic 45+)"
        );
    }
}

#[test]
fn fr99_closeout_brand_and_prelude_only() {
    let fr99 = read("docs/fr99-bitloom-lsp.md");
    assert!(
        fr99.contains("Bitloom") && fr99.contains("bitloom-prelude"),
        "fr99 must keep Bitloom brand and prelude-only design deps"
    );
    let nfr14 =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic44-full-elaborate-lsp.md");
    assert!(
        nfr14.contains("bitloom-prelude") && nfr14.contains("Bitloom"),
        "NFR14 must retain Bitloom / prelude-only brand+dep close condition"
    );
}
