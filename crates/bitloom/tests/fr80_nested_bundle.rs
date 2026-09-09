//! ATDD / guardrail: FR80 one-level nested Bundle + derive (Stories 32.2–32.3).
//! Locks language-surface contract so nested/derive is not closed only by OUT OF SCOPE.

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
