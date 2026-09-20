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
        ws.contains("bitloom-viz") && ws.contains("crates/rhdl-viz"),
        "workspace must map bitloom-viz path+version"
    );
    assert!(
        !ws.lines().any(|l| l.trim().starts_with("rhdl-viz =")),
        "workspace must not keep rhdl-viz dependency key"
    );

    // Cargo resolves the current path+registry requirement; historical 1.0.0
    // identity must not pin every subsequent release to its original version.
    let metadata = Command::new("cargo")
        .args([
            "metadata",
            "--no-deps",
            "--offline",
            "--format-version",
            "1",
        ])
        .current_dir(&root)
        .output()
        .expect("cargo metadata");
    assert!(
        metadata.status.success(),
        "{}",
        String::from_utf8_lossy(&metadata.stderr)
    );
    let metadata: serde_json::Value = serde_json::from_slice(&metadata.stdout).unwrap();
    let cli = metadata["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["name"] == "bitloom")
        .unwrap();
    let dependency = cli["dependencies"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["name"] == "bitloom-viz")
        .unwrap();
    assert_ne!(
        dependency["req"], "*",
        "published dependency needs a version requirement"
    );
    assert_eq!(
        dependency["path"].as_str(),
        root.join("crates/rhdl-viz").to_str()
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
