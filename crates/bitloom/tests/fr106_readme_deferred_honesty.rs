//! ATDD / guardrail: Story 48.3 / FR106+FR115 — README / deferred /
//! doc-19 honesty surface for Phase 13 commercial deepen.
//!
//! ```text
//! cargo test -p bitloom --test fr106_readme_deferred_honesty
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
fn fr106_readme_distinguishes_phase12_mvp_vs_phase13() {
    let readme = read("README.md");
    assert!(
        readme.contains("Phase 12")
            && (readme.contains("FR94") || readme.contains("字面绿"))
            && (readme.contains("已关闭") || readme.contains("MVP")),
        "README must keep Phase 12 MVP closed framing"
    );
    assert!(
        readme.contains("Phase 13") && (readme.contains("商业加深") || readme.contains("FR106")),
        "README must declare Phase 13 commercial deepen"
    );
    assert!(
        readme.contains("FR115")
            || (readme.contains("FR106") && readme.contains("FR114") && readme.contains("宣称")),
        "README must cite FR115 / FR106–114 claim discipline"
    );
    assert!(
        readme.contains("NFR44")
            || (readme.contains("不得")
                && readme.contains("失败")
                && (readme.contains("Phase 12") || readme.contains("FR94"))),
        "README must keep NFR44 Phase 12 isolation honesty"
    );
}

#[test]
fn fr106_readme_lists_deepen_fr_table() {
    let readme = read("README.md");
    for fr in [
        "FR107", "FR108", "FR109", "FR110", "FR111", "FR112", "FR113", "FR114",
    ] {
        assert!(
            readme.contains(fr),
            "README Phase 13 deepen table must list {fr}"
        );
    }
}

#[test]
fn fr106_deferred_phase13_pointer_and_optional_upgrade() {
    let deferred = read("_agile-output/implementation-artifacts/deferred-work.md");
    assert!(
        deferred.contains("Phase 13")
            && (deferred.contains("FR106") || deferred.contains("商业加深")),
        "deferred-work must have Phase 13 pointer"
    );
    assert!(
        deferred.contains("FR107") && deferred.contains("FR113") && deferred.contains("FR114"),
        "deferred optional items must map to Phase 13 FRs"
    );
    assert!(
        !deferred.contains("explicit new contract required")
            || deferred.contains("Phase 13 contract approved"),
        "optional product must no longer claim no contract exists without Phase 13 approval note"
    );
    assert!(
        deferred.contains("NFR44") || deferred.contains("仍有效"),
        "deferred must keep Phase 12 MVP closes valid"
    );
}

#[test]
fn fr106_doc19_cross_link_phase13_without_redefining_mvp() {
    let doc19 = read("docs/requirements/19. 实施路线图.md");
    assert!(
        doc19.contains("Phase 13") || doc19.contains("FR106"),
        "doc-19 should cross-link Phase 13 deepen contract"
    );
    assert!(
        doc19.contains("字面绿完成定义") && doc19.contains("FR94"),
        "doc-19 must retain Phase 12 literal-green MVP definitions"
    );
}
