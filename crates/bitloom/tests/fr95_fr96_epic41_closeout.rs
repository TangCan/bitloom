//! ATDD Story 41.4 — FR95/FR96 Epic 41 closeout.
//!
//! Locks NFR14 close checkboxes, README/deferred honesty (in-tree HLS is not a
//! permanent non-goal; external Bambu is not the sole FR95 definition), and
//! sprint `epic-41: done` without starting Epic 42+.
//!
//! ```text
//! cargo test -p bitloom --test fr95_fr96_epic41_closeout
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
fn fr95_fr96_nfr14_epic41_close_conditions_checked() {
    let text = read("_agile-output/implementation-artifacts/nfr14-risk-epic41-in-tree-hls.md");
    for needle in [
        "- [x] **FR95：**",
        "- [x] **FR96：**",
        "- [x] **回归 / 文档：**",
        "- [x] **禁止事项未触发：**",
        "- [x] **品牌 / 依赖：**",
        "- [x] **NFR41：**",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 41 close condition missing checked item: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 41.4") || text.contains("41.4")),
        "NFR14 Epic 41 record must be closed with Story 41.4 pointer"
    );
}

#[test]
fn fr95_fr96_readme_in_tree_not_forbidden() {
    let readme = read("README.md");
    assert!(
        !readme.contains("不实现树内调度器"),
        "README must not claim Bitloom does not implement an in-tree scheduler"
    );
    assert!(
        !readme.contains("**不**实现树内调度器"),
        "README must not keep bold 'does not implement in-tree scheduler' framing"
    );
    // HLS product surface must cite FR95 and in-tree completion face.
    let hls_idx = readme
        .find("## HLS")
        .expect("README must have ## HLS section");
    let hls = &readme[hls_idx..];
    let hls_end = hls.find("\n## ").unwrap_or(hls.len().min(2500));
    let section = &hls[..hls_end];
    assert!(
        section.contains("FR95")
            && (section.contains("树内")
                || section.contains("in-tree")
                || section.contains("--in-tree")),
        "README HLS section must document FR95 in-tree path"
    );
    assert!(
        section.contains("FR96") || readme.contains("FR96"),
        "README must mention FR96 after Epic 41 closeout"
    );
}

#[test]
fn fr95_fr96_external_not_sole_fr95_definition() {
    let readme = read("README.md");
    let fr35 = read("docs/fr35-hls.md");
    let combined = format!("{readme}\n{fr35}");
    assert!(
        (combined.contains("不得单独满足") || combined.contains("不得单独关闭"))
            && combined.contains("FR95"),
        "docs must state external path must not alone satisfy/close FR95"
    );
    assert!(
        fr35.contains("FR95")
            && (fr35.contains("完成面") || fr35.contains("树内"))
            && (fr35.contains("外挂") || fr35.contains("Bambu")),
        "fr35 must keep in-tree FR95 completion face vs external honesty"
    );
}

#[test]
fn fr95_fr96_deferred_readme_epic41_closed() {
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    let surface = format!("{readme}\n{deferred}");

    // Must not frame in-tree HLS as an active permanent non-goal.
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
        (section.contains("FR95") || section.contains("FR96"))
            && (section.contains("已关闭")
                || section.contains("已交付")
                || section.contains("Epic 41 已")
                || section.contains("Epic 41 closed")),
        "README FR93#1 / Epic 41 mapping must note FR95/FR96 / Epic 41 closed"
    );

    assert!(
        (deferred.contains("FR95") || deferred.contains("FR96"))
            && (deferred.contains("Epic 41") || deferred.contains("epic-41"))
            && (deferred.contains("已关闭")
                || deferred.contains("已交付")
                || deferred.contains("closed")),
        "deferred-work must note Epic 41 / FR95+FR96 closed"
    );

    // Sanity: still not "须新 PRD" as active lock for in-tree HLS.
    assert!(
        !surface.contains("树内")
            || !surface.contains("须新 PRD 才能推翻")
            || surface.contains("历史"),
        "in-tree HLS must not be under an active 须新 PRD permanent-non-goal lock"
    );
}

#[test]
fn fr95_fr96_sprint_epic41_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-41: done") || sprint.contains("epic-41:done"),
        "sprint-status must mark epic-41: done"
    );
    assert!(
        sprint.contains("41-4-fr95-fr96-收口与回归: done"),
        "sprint-status must mark 41-4 done"
    );
    // Epic 42 may remain backlog after 41.4, or advance after Story 42.1 NFR14.
    assert!(
        sprint.contains("epic-42: backlog")
            || sprint.contains("epic-42: in-progress")
            || sprint.contains("epic-42: done"),
        "sprint-status must list epic-42 as backlog, in-progress, or done"
    );
    if !sprint.contains("epic-42: backlog") {
        assert!(
            sprint.contains("42-1-epic-42-nfr14-风险记录: done"),
            "leaving epic-42 backlog requires Story 42.1 NFR14 done (gate)"
        );
    }
}

#[test]
fn fr95_fr96_closeout_brand_bitloom() {
    let readme = read("README.md");
    assert!(
        readme.contains("Bitloom") && (readme.contains("bitloom") || readme.contains("`bitloom")),
        "README must keep Bitloom / bitloom branding"
    );
}
