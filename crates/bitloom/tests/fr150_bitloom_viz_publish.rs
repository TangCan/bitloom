//! ATDD Story 85.3 / FR150: bitloom-viz is the publishable package name.

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
fn fr150_bitloom_viz_package_identity() {
    let root = workspace_root();
    let toml = fs::read_to_string(root.join("crates/rhdl-viz/Cargo.toml")).expect("viz toml");
    assert!(
        toml.contains("name = \"bitloom-viz\""),
        "package name must be bitloom-viz:\n{toml}"
    );
    assert!(
        !toml.contains("name = \"rhdl-viz\""),
        "must not publish under rhdl-viz"
    );
    assert!(
        toml.contains("publish = true"),
        "bitloom-viz must be publishable:\n{toml}"
    );

    let ws = fs::read_to_string(root.join("Cargo.toml")).expect("workspace toml");
    assert!(
        ws.contains("bitloom-viz")
            && ws.contains("crates/rhdl-viz")
            && ws.contains("version = \"1.0.0\""),
        "workspace must map bitloom-viz path+version"
    );
    assert!(
        !ws.lines().any(|l| l.trim().starts_with("rhdl-viz =")),
        "workspace must not keep rhdl-viz dependency key"
    );

    let bitloom = fs::read_to_string(root.join("crates/bitloom/Cargo.toml")).expect("cli toml");
    assert!(
        bitloom.contains("bitloom-viz"),
        "CLI must depend on bitloom-viz"
    );
    assert!(
        !bitloom.contains("rhdl-viz"),
        "CLI must not depend on rhdl-viz package name"
    );

    let doc = fs::read_to_string(root.join("docs/fr150-bitloom-viz-publish.md")).expect("fr150");
    assert!(
        doc.contains("bitloom-viz") && doc.contains("FR150"),
        "docs/fr150 must lock publish identity"
    );
}

#[test]
fn fr150_bitloom_viz_dry_run_publish() {
    let root = workspace_root();
    let out = Command::new("cargo")
        .args(["publish", "-p", "bitloom-viz", "--dry-run", "--allow-dirty"])
        .current_dir(&root)
        .output()
        .expect("spawn cargo publish dry-run");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "cargo publish -p bitloom-viz --dry-run must succeed\nstdout={stdout}\nstderr={stderr}"
    );
}
