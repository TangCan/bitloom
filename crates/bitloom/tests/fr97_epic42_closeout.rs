//! ATDD Story 42.3 — FR97 Epic 42 closeout.
//!
//! Locks NFR14 close checkboxes, README/deferred honesty (FIRRTL→idiomatic
//! Scala is not a permanent non-goal; Epic 42 / FR97 closed), fr28 mechanical
//! honesty retained, and sprint `epic-42: done` without starting Epic 43+.
//!
//! ```text
//! cargo test -p bitloom --test fr97_epic42_closeout
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
fn fr97_nfr14_epic42_close_conditions_checked() {
    let text = read("_agile-output/implementation-artifacts/nfr14-risk-epic42-idiomatic-chisel.md");
    for needle in [
        "- [x] **FR97：**",
        "- [x] **文档：**",
        "- [x] **回归：**",
        "- [x] **禁止事项未触发：**",
        "- [x] **品牌 / 依赖：**",
        "- [x] **NFR41：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 42 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 42.3") || text.contains("42.3")),
        "NFR14 Epic 42 record must be closed with Story 42.3 pointer"
    );
}

#[test]
fn fr97_deferred_readme_epic42_closed() {
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    let surface = format!("{readme}\n{deferred}");

    let fr93_start = readme
        .find("永久非目标")
        .expect("README must retain historical 永久非目标 section");
    let block = &readme[fr93_start..];
    let end = block
        .find("\n## ")
        .or_else(|| block.find("\n详见"))
        .unwrap_or(block.len().min(3000));
    let section = &block[..end];
    assert!(
        section.contains("已推翻")
            || section.contains("已被")
            || section.contains("Phase 12")
            || section.contains("FR94"),
        "FR93 section must remain overturn / Phase 12 framed"
    );
    assert!(
        section.contains("FR97")
            && (section.contains("已关闭")
                || section.contains("已交付")
                || section.contains("Epic 42 已")
                || section.contains("Epic 42 closed")),
        "README FR93#2 / Epic 42 mapping must note FR97 / Epic 42 closed"
    );
    assert!(
        (section.contains("idiomatic") || section.contains("FIRRTL")) && section.contains("FR97"),
        "README FR93#2 must still map FIRRTL→idiomatic / idiomatic Chisel → FR97"
    );

    assert!(
        deferred.contains("FR97")
            && (deferred.contains("Epic 42") || deferred.contains("epic-42"))
            && (deferred.contains("已关闭")
                || deferred.contains("已交付")
                || deferred.contains("closed")),
        "deferred-work must note Epic 42 / FR97 closed"
    );

    // Must not frame idiomatic as an active permanent non-goal lock.
    assert!(
        !surface.contains("须新 PRD 才能推翻")
            || surface.contains("历史")
            || surface.contains("曾")
            || surface.contains("已推翻"),
        "idiomatic Chisel must not be under an active 须新 PRD permanent-non-goal lock"
    );
}

#[test]
fn fr97_fr28_mechanical_honesty_retained() {
    let fr28 = read("docs/fr28-chisel-compilable.md");
    assert!(
        fr28.contains("可编译 ≠ idiomatic")
            || fr28.contains("≠ idiomatic")
            || fr28.contains("compilable ≠ idiomatic"),
        "fr28 must retain mechanical ≠ idiomatic honesty after Epic 42 closeout"
    );
    assert!(
        fr28.contains("FR97") || fr28.contains("fr97"),
        "fr28 must cross-link FR97 idiomatic face"
    );
    assert!(
        fr28.contains("Epic 42") || fr28.contains("已关闭") || fr28.contains("FR97"),
        "fr28 closeout should acknowledge FR97 / Epic 42 delivery context"
    );
}

#[test]
fn fr97_doc_closeout_no_pending_42_3() {
    let fr97 = read("docs/fr97-idiomatic-chisel.md");
    assert!(
        fr97.contains("FR97") && (fr97.contains("idiomatic") || fr97.contains("可维护")),
        "fr97 doc must describe idiomatic/maintainable face"
    );
    assert!(
        fr97.contains("AD-27") || fr97.contains("AD27"),
        "fr97 doc must cite revised AD-27"
    );
    assert!(
        fr97.to_lowercase().contains("bitloom"),
        "fr97 doc must keep Bitloom brand"
    );
    assert!(
        !fr97.contains("不在本故事") && !fr97.contains("不在本页"),
        "fr97 doc must not leave Story 42.3 as pending 'not in this story'"
    );
    assert!(
        fr97.contains("已关闭")
            || fr97.contains("已交付")
            || fr97.contains("Epic 42 closed")
            || (fr97.contains("Epic 42") && fr97.contains("关闭")),
        "fr97 doc must note Epic 42 / FR97 closeout"
    );
}

#[test]
fn fr97_sprint_epic42_done_epic43_untouched() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-42: done") || sprint.contains("epic-42:done"),
        "sprint-status must mark epic-42: done"
    );
    assert!(
        sprint.contains("42-3-fr97-收口与回归: done"),
        "sprint-status must mark 42-3 done"
    );
    assert!(
        sprint.contains("epic-43: backlog") || sprint.contains("epic-43:backlog"),
        "sprint-status must keep epic-43: backlog (do not start Epic 43+)"
    );
    assert!(
        sprint.contains("43-1-epic-43-nfr14-风险记录: backlog"),
        "sprint-status must keep 43.1 backlog"
    );
}

#[test]
fn fr97_closeout_brand_bitloom() {
    let readme = read("README.md");
    assert!(
        readme.contains("Bitloom") && (readme.contains("bitloom") || readme.contains("`bitloom")),
        "README must keep Bitloom / bitloom branding"
    );
}
