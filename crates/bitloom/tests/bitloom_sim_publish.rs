//! ATDD Story 14.1: package identity is bitloom-sim; design deps stay prelude-only.

use std::fs;
use std::path::PathBuf;

#[test]
fn bitloom_sim_package_publishable_name() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace");
    let toml = fs::read_to_string(root.join("crates/rhdl-sim/Cargo.toml")).expect("sim toml");
    assert!(
        toml.contains("name = \"bitloom-sim\""),
        "publish name must be bitloom-sim:\n{toml}"
    );
    assert!(
        toml.contains("publish = true"),
        "bitloom-sim must be publishable:\n{toml}"
    );
    assert!(
        toml.contains("bitloom-hir"),
        "must depend on bitloom-hir:\n{toml}"
    );
}

#[test]
fn ad6_allows_bitloom_sim_only_as_dev_dep() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace");
    let spine = fs::read_to_string(
        root.join(
            "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
        ),
    )
    .expect("spine");
    assert!(
        spine.contains("bitloom-sim")
            && spine.contains("[dev-dependencies]")
            && spine.contains("bitloom-prelude"),
        "AD-6 must document bitloom-sim as optional dev-dep"
    );
    let readme = fs::read_to_string(root.join("README.md")).expect("readme");
    assert!(
        readme.contains("cargo add bitloom-sim --dev"),
        "README must document optional independent sim"
    );
}
