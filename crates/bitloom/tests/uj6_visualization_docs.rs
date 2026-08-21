//! ATDD Story 23.4: unified docs + UJ-6 visualization half.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn uj6_visualization_doc_is_followable() {
    let root = workspace_root();
    let uj6 = fs::read_to_string(root.join("docs/tutorials/uj6-visualization.md"))
        .expect("uj6-visualization.md");
    assert!(
        uj6.contains("Bitloom")
            && uj6.contains("cargo bitloom visualize")
            && uj6.contains("cargo bitloom wave")
            && uj6.contains("external_hierarchy.fir")
            && uj6.contains("hierarchy.html")
            && uj6.contains("timing.html"),
        "UJ-6 doc must name Bitloom, both CLI entries, fixture, and outputs"
    );
    assert!(
        uj6.contains("LSP")
            && (uj6.contains("deferred") || uj6.contains("Deferred") || uj6.contains("not")),
        "UJ-6 doc must state LSP is not an Epic 23 done criterion"
    );
    assert!(
        uj6.contains("fr31-optional-fst") || uj6.contains("FR31"),
        "UJ-6 doc must cross-link optional FST / FR31"
    );

    let readme = fs::read_to_string(root.join("README.md")).expect("README");
    assert!(
        readme.contains("visualize")
            && readme.contains("wave")
            && readme.contains("uj6-visualization"),
        "README must document visualize/wave and link UJ-6 tutorial"
    );
    assert!(
        !readme.contains("`visualize` / `wave` / `doc`")
            || !readme.contains("部分 CLI 动词（`check` / `visualize`"),
        "README must not still list visualize/wave/doc as deferred verbs"
    );
}
