//! Story 12.1 maturity contract files must exist.

use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .unwrap()
}

#[test]
fn maturity_contract_artifacts_exist() {
    let root = root();
    for rel in [
        "SECURITY.md",
        "CHANGELOG.md",
        "docs/semver-0x-policy.md",
        "docs/crates-io-publish-bitloom.md",
        ".github/workflows/ci.yml",
        "README.md",
    ] {
        let p = root.join(rel);
        assert!(p.is_file(), "missing maturity artifact: {}", p.display());
    }
    let readme = std::fs::read_to_string(root.join("README.md")).unwrap();
    assert!(
        readme.contains("deferred") || readme.contains("Deferred"),
        "README must disclose deferred/non-goals"
    );
    assert!(readme.contains("0.x") || readme.contains("**0.x**"));

    let manifest = std::fs::read_to_string(root.join("crates/bitloom/Cargo.toml")).unwrap();
    assert!(
        manifest.contains("repository = \"https://github.com/TangCan/bitloom\""),
        "bitloom must declare GitHub repository metadata"
    );
    assert!(
        manifest.contains("homepage = \"https://github.com/TangCan/bitloom\""),
        "bitloom must declare homepage metadata"
    );
}
