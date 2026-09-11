//! ATDD: FR145 hygiene skip documentation.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn fr145_skip_doc_exists() {
    let path = workspace_root().join("docs/fr145-pre-1-0-hygiene-skip.md");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    assert!(text.contains("FR145"), "must cite FR145");
    assert!(
        text.contains("skip") || text.contains("Skip") || text.contains("FR145-skip"),
        "must state skip"
    );
    assert!(
        text.contains("1.0.0") || text.contains("major"),
        "must relate skip to 1.0 major absorption"
    );
    assert!(
        text.contains("NFR59")
            && (text.contains("not") || text.contains("不") || text.contains("不得")),
        "must not claim NFR59 closed"
    );
    assert!(text.contains("Bitloom") || text.contains("bitloom"));
    assert!(
        text.contains("PortField") || text.contains("prelude") || text.contains("semver-check"),
        "must cite evidence for skip"
    );
}
