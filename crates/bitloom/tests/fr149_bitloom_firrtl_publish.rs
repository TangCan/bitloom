//! ATDD Story 85.2 / FR149: bitloom-firrtl is the publishable package name.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn fr149_bitloom_firrtl_package_identity() {
    let root = workspace_root();
    let toml = fs::read_to_string(root.join("crates/rhdl-firrtl/Cargo.toml")).expect("firrtl toml");
    assert!(
        toml.contains("name = \"bitloom-firrtl\""),
        "package name must be bitloom-firrtl:\n{toml}"
    );
    assert!(
        !toml.contains("name = \"rhdl-firrtl\""),
        "must not publish under rhdl-firrtl"
    );
    assert!(
        toml.contains("publish = true"),
        "bitloom-firrtl must be publishable:\n{toml}"
    );

    let ws = fs::read_to_string(root.join("Cargo.toml")).expect("workspace toml");
    assert!(
        ws.contains("bitloom-firrtl")
            && ws.contains("crates/rhdl-firrtl")
            && ws.contains("version = \"1.0.0\""),
        "workspace must map bitloom-firrtl path+version:\n{ws}"
    );
    assert!(
        !ws.lines().any(|l| l.trim().starts_with("rhdl-firrtl =")),
        "workspace must not keep rhdl-firrtl dependency key"
    );

    let bitloom = fs::read_to_string(root.join("crates/bitloom/Cargo.toml")).expect("cli toml");
    assert!(
        bitloom.contains("bitloom-firrtl"),
        "CLI must depend on bitloom-firrtl"
    );
    assert!(
        !bitloom.contains("rhdl-firrtl"),
        "CLI must not depend on rhdl-firrtl package name"
    );

    let doc = fs::read_to_string(root.join("docs/fr149-bitloom-firrtl-publish.md")).expect("fr149");
    assert!(
        doc.contains("bitloom-firrtl")
            && doc.contains("FR149")
            && (doc.contains("rhdl-firrtl") || doc.contains("never")),
        "docs/fr149 must lock publish identity"
    );
    assert!(
        doc.contains("Bitloom") || doc.contains("bitloom"),
        "FR149 doc must cite Bitloom brand"
    );
}

#[test]
fn fr149_bitloom_firrtl_dry_run_publish() {
    let root = workspace_root();
    let out = Command::new("cargo")
        .args([
            "publish",
            "-p",
            "bitloom-firrtl",
            "--dry-run",
            "--allow-dirty",
        ])
        .current_dir(&root)
        .output()
        .expect("spawn cargo publish dry-run");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "cargo publish -p bitloom-firrtl --dry-run must succeed\nstdout={stdout}\nstderr={stderr}"
    );
}
