//! ATDD Story 13.6: README leads with true-standalone path.

use std::fs;
use std::path::PathBuf;

#[test]
fn readme_leads_with_install_new_build() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace");
    let readme = fs::read_to_string(root.join("README.md")).expect("README");
    assert!(
        readme.contains("cargo install bitloom"),
        "README must show cargo install bitloom"
    );
    assert!(
        readme.contains("cargo bitloom new"),
        "README must show cargo bitloom new"
    );
    assert!(
        readme.contains("cargo bitloom build"),
        "README must show cargo bitloom build"
    );
    assert!(
        readme.contains("bitloom-prelude"),
        "README must name bitloom-prelude as design dep"
    );
    assert!(
        !readme.contains("应只依赖 `rhdl-prelude`"),
        "must not still prescribe unpublished rhdl-prelude"
    );
    let publish = fs::read_to_string(root.join("docs/crates-io-publish-bitloom.md"))
        .expect("publish runbook");
    assert!(
        publish.contains("bitloom-prelude") && publish.contains("Trusted Publishing"),
        "publish runbook must cover bitloom-* + Trusted Publishing"
    );
}
