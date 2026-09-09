//! ATDD / guardrail: FR80 one-level nested Bundle + derive + Story 32.4 docs close-out.
//! Locks language-surface / tutorial / NFR14 so nested/derive is not closed only by OUT OF SCOPE.
//!
//! ```text
//! cargo test -p bitloom --test fr80_nested_bundle
//! ```

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

#[test]
fn language_surface_documents_one_level_nested_bundle_fr80() {
    let path = repo_root().join("_agile-output/specs/spec-rhdl/language-surface.md");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    assert!(
        text.contains("FR80") && text.contains("nested_bundles"),
        "language-surface must document FR80 nested_bundles"
    );
    assert!(
        text.contains("一层") || text.contains("one-level") || text.contains("一层嵌套"),
        "language-surface must document at least one nesting level"
    );
    // Must not close nested Bundle solely with blanket OUT OF SCOPE (NFR37).
    let composite = text
        .split("## Composite types")
        .nth(1)
        .and_then(|s| s.split("## ").next())
        .expect("Composite types section");
    let nested_only_oos = composite.contains("嵌套 `Bundle` 成员与 `HwVec<Bundle,_>`")
        && composite.contains("OUT OF SCOPE")
        && !composite.contains("FR80");
    assert!(
        !nested_only_oos,
        "nested Bundle must not be closed only by OUT OF SCOPE without FR80 path"
    );
    assert!(
        composite.contains("HwVec<Bundle")
            && (composite.contains("OUT OF SCOPE") || composite.contains("仍 OUT OF SCOPE")),
        "HwVec<Bundle,_> may remain OUT OF SCOPE"
    );
    assert!(
        composite.contains("#[derive(Bundle)]")
            && (composite.contains("经 `bitloom-prelude`")
                || composite.contains("bitloom-prelude")
                || composite.contains("可用")),
        "language-surface must document #[derive(Bundle)] as available via prelude"
    );
    assert!(
        !composite.contains("不可用（documented defer"),
        "must not keep derive unavailable defer wording"
    );
    assert!(
        composite.contains("32.4")
            || composite.contains("nested-bundle.md")
            || composite.contains("fr80_nested_bundle"),
        "language-surface must mark Story 32.4 docs close-out"
    );
}

#[test]
fn prelude_bundle_trait_exposes_nested_bundles() {
    let path = repo_root().join("crates/bitloom-prelude/src/lib.rs");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    assert!(
        text.contains("fn nested_bundles") && text.contains("FR80"),
        "prelude Bundle must expose nested_bundles under FR80"
    );
    assert!(
        !text.contains("**OUT OF SCOPE (MVP):** nested `Bundle` members and `HwVec<Bundle, _>`"),
        "must not keep blanket nested+HwVec OUT OF SCOPE MVP phrase"
    );
    assert!(
        text.contains("pub use bitloom_macro::Bundle")
            || (text.contains("Derive (FR80)") && text.contains("#[derive(Bundle)] is available")),
        "prelude must re-export or document #[derive(Bundle)]"
    );
    assert!(
        !text.contains("**`#[derive(Bundle)]` is not available**"),
        "must not claim derive unavailable"
    );
}

#[test]
fn design_crate_bundle_skel_depends_only_on_prelude() {
    let path = repo_root().join("examples/bundle_vec_skel/Cargo.toml");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    let deps = text
        .split("[dependencies]")
        .nth(1)
        .and_then(|s| s.split('[').next())
        .expect("[dependencies]");
    assert!(
        deps.contains("bitloom-prelude"),
        "skel must depend on bitloom-prelude"
    );
    assert!(
        !deps.contains("bitloom-macro") && !deps.contains("bitloom ="),
        "design crate must not depend on bitloom-macro or CLI bitloom (AD-6)"
    );
}

#[test]
fn fr80_nested_bundle_tutorial_names_examples_limits_and_recipe() {
    let root = repo_root();
    let tutorial = std::fs::read_to_string(root.join("docs/tutorials/nested-bundle.md"))
        .expect("docs/tutorials/nested-bundle.md must exist for UJ nested Bundle");

    assert!(
        tutorial.contains("嵌套 Bundle") || tutorial.contains("FR80"),
        "tutorial must identify nested Bundle / FR80"
    );
    assert!(
        tutorial.contains("nested_bundles") || tutorial.contains("Packet"),
        "tutorial must show nested Bundle surface"
    );
    assert!(
        tutorial.contains("#[derive(Bundle)]"),
        "tutorial must include derive minimal example"
    );
    assert!(
        tutorial.contains("Limits table") || tutorial.contains("限制"),
        "tutorial must include limits table"
    );
    assert!(
        tutorial.contains("E0131") || tutorial.contains("rhdl::E0131"),
        "tutorial limits must name width mismatch diagnostic"
    );
    assert!(
        tutorial.contains("E0112") || tutorial.contains("rhdl::E0112"),
        "tutorial limits must name dir mismatch diagnostic"
    );
    assert!(
        tutorial.contains("E0180") || tutorial.contains("rhdl::E0180"),
        "tutorial limits must name derive reject diagnostic"
    );
    assert!(
        (tutorial.contains("≥2") || tutorial.contains("非目标") || tutorial.contains("Non-goal"))
            && (tutorial.contains("HwVec<Bundle") || tutorial.contains("OUT OF SCOPE")),
        "tutorial must document ≥2 non-goal and HwVec<Bundle,_> OOS"
    );
    assert!(
        tutorial.contains("bundle_vec_skel") && tutorial.contains("fr80_nested_bundle"),
        "tutorial must point at skel goldens and this ATDD"
    );
    assert!(
        tutorial.contains("just test") || tutorial.contains("cargo test --workspace"),
        "tutorial must document contributor recipe (just test / workspace)"
    );
    assert!(
        tutorial.contains("NFR37") && (tutorial.contains("FR51") || tutorial.contains("最小")),
        "tutorial must contrast FR80 depth vs FR51 minimal (NFR37)"
    );

    assert!(
        root.join("examples/bundle_vec_skel/src/lib.rs").is_file(),
        "bundle_vec_skel fixture must exist on disk"
    );
    assert!(
        root.join("docs/fr80-nested-bundle.md").is_file(),
        "FR80 product doc must exist on disk"
    );
}

#[test]
fn fr80_docs_and_readme_index_nested_bundle_depth() {
    let root = repo_root();

    let product = std::fs::read_to_string(root.join("docs/fr80-nested-bundle.md")).expect("fr80");
    assert!(
        product.contains("NFR37") && product.contains("FR51") && product.contains("FR80"),
        "FR80 product doc must state depth vs FR51 / NFR37"
    );
    assert!(
        product.contains("tutorials/nested-bundle.md") || product.contains("嵌套 Bundle"),
        "FR80 product doc must link nested-bundle follow-along"
    );

    let readme = std::fs::read_to_string(root.join("README.md")).expect("README");
    assert!(
        readme.contains("FR80")
            && (readme.contains("nested-bundle.md") || readme.contains("bundle_vec_skel")),
        "README must index FR80 nested Bundle (tutorial and/or skel)"
    );
}

#[test]
fn fr80_nfr14_epic32_close_checklist_ticked() {
    let root = repo_root();
    let nfr14 = std::fs::read_to_string(
        root.join("_agile-output/implementation-artifacts/nfr14-risk-epic32-nested-bundle.md"),
    )
    .expect("nfr14 epic32");

    let close = nfr14
        .split("### Epic 32 关闭条件")
        .nth(1)
        .expect("Epic 32 close section");
    for needle in [
        "FR80 嵌套",
        "FR80 derive",
        "宽/向负例",
        "NFR37",
        "ATDD",
        "禁止事项未触发",
    ] {
        assert!(
            close.contains(needle),
            "close checklist must mention {needle}"
        );
    }
    let unchecked = close
        .lines()
        .filter(|l| l.trim_start().starts_with("- [ ]"))
        .count();
    assert_eq!(
        unchecked, 0,
        "Epic 32 close conditions must all be checked [x]; found {unchecked} unchecked"
    );
    assert!(
        close
            .lines()
            .filter(|l| l.trim_start().starts_with("- [x]"))
            .count()
            >= 6,
        "expected at least 6 ticked close conditions"
    );
}

#[test]
fn fr80_skel_source_names_nested_emit_tick_and_width_dir_negatives() {
    let path = repo_root().join("examples/bundle_vec_skel/src/lib.rs");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    assert!(
        text.contains("elaborate_emit_tick_one_level_nested_bundle")
            && text.contains("elaborate_emit_tick_derived_nested_bundle"),
        "skel must keep nested hand-written + derived emit/tick goldens"
    );
    assert!(
        text.contains("nested_width_mismatch_fails_before_emit")
            && text.contains("nested_dir_mismatch_fails_before_emit"),
        "skel must keep nested width/dir negatives before emit"
    );
    assert!(
        text.contains("rhdl::E0131") && text.contains("rhdl::E0112"),
        "skel negatives must assert E0131 / E0112"
    );
}
