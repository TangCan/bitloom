//! ATDD Story 63.3 — FR122 / Epic 63 closeout + FR97/FR111 cross-link honesty.
//!
//! ```text
//! cargo test -p bitloom --test fr122_epic63_closeout
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
fn fr122_nfr14_epic63_close_conditions_checked() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic63-official-style-chisel.md");
    for needle in [
        "- [x] **63.2 / FR122：",
        "- [x] **文档 / deferred / README / AD-27 修订戳",
        "- [x] **禁止事项未触发",
        "- [x] **品牌 / 依赖：",
        "- [x] **FR97 / FR111 关闭仍有效",
    ] {
        assert!(
            text.contains(needle),
            "NFR14 Epic 63 close condition missing: {needle}"
        );
    }
    assert!(
        text.contains("closed") && (text.contains("Story 63.3") || text.contains("63.3")),
        "NFR14 must be closed with Story 63.3 pointer"
    );
}

#[test]
fn fr122_docs_readme_deferred_closed() {
    let fr122 = read("docs/fr122-official-style-chisel.md");
    let fr97 = read("docs/fr97-idiomatic-chisel.md");
    let fr111 = read("docs/fr111-idiomatic-chisel-depth.md");
    let readme = read("README.md");
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    let epics = read("_agile-output/planning-artifacts/epics.md");
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        (fr122.contains("closed") || fr122.contains("已关闭"))
            && (fr122.contains("Epic 63") || fr122.contains("63.3")),
        "fr122 doc must declare Epic 63 / FR122 closed"
    );
    assert!(
        fr97.contains("FR122") || fr111.contains("FR122"),
        "fr97/fr111 must cross-link FR122"
    );
    assert!(
        readme.contains("FR122")
            && (readme.contains("Epic 63 已关闭") || readme.contains("FR122 / Epic 63 已关闭")),
        "README must note FR122 / Epic 63 closed"
    );
    assert!(
        deferred.contains("FR122")
            && (deferred.contains("已关闭") || deferred.contains("closed"))
            && (deferred.contains("63.3") || deferred.contains("Epic 63")),
        "deferred must close FR122 item"
    );
    assert!(
        deferred.contains("Phase 14 规划/实现故事已齐")
            || deferred.contains("Epic 57–63")
            || readme.contains("规划/实现故事已齐"),
        "must declare Phase 14 planning/implementation stories complete"
    );
    assert!(
        epics.contains("phase14Epic63Status: complete"),
        "epics.md must stamp Epic 63 complete"
    );
    assert!(
        spine.contains("FR122")
            && spine.contains("AD-27")
            && (spine.contains("Revised") || spine.contains("修订")),
        "spine must retain AD-27 FR122 revise stamp"
    );
    // Honest uncovered / NFR51
    assert!(
        (fr122.contains("Style Guide")
            || fr122.contains("Parser")
            || fr122.contains("NFR51")
            || fr122.contains("linter"))
            && (fr122.contains("deferred") || fr122.contains("Non-goals")),
        "must honestly disclose NFR51 deferred non-goals"
    );
}

#[test]
fn fr122_fr97_fr111_mvp_still_valid() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("epic-42: done") || sprint.contains("epic-42:done"),
        "FR97 / Epic 42 must remain closed (NFR48)"
    );
    assert!(
        sprint.contains("epic-53: done") || sprint.contains("epic-53:done"),
        "FR111 / Epic 53 must remain closed (NFR48)"
    );
    let fr97 = read("docs/fr97-idiomatic-chisel.md");
    let fr111 = read("docs/fr111-idiomatic-chisel-depth.md");
    assert!(
        fr97.contains("FR97") && (fr97.contains("idiomatic") || fr97.contains("可维护")),
        "fr97 must keep FR97 closed surface"
    );
    assert!(
        fr111.contains("FR111")
            && (fr111.contains("closed") || fr111.contains("已关闭") || fr111.contains("53.3")),
        "fr111 must keep FR111 closed surface"
    );
}

#[test]
fn fr122_sprint_epic63_done() {
    let sprint = read("_agile-output/implementation-artifacts/sprint-status.yaml");
    assert!(
        sprint.contains("63-3-fr122-收口与-phase-14-故事清单指针: done")
            || sprint.contains("63-3-fr122-收口与-phase-14-故事清单指针:done")
    );
    assert!(sprint.contains("epic-63: done") || sprint.contains("epic-63:done"));
    assert!(
        sprint.contains("63-2-ad-27-修订-若需-官方风格全家桶路径-fr122: done")
            || sprint.contains("63-2-ad-27-修订-若需-官方风格全家桶路径-fr122:done")
    );
    assert!(
        sprint.contains("63-1-epic-63-nfr14-风险记录: done")
            || sprint.contains("63-1-epic-63-nfr14-风险记录:done")
    );
}
