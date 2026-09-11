//! ATDD Story 85.5 / FR151: bitloom CLI is publishable with versioned deps.

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
fn fr151_bitloom_cli_versioned_and_documented() {
    let root = workspace_root();
    let toml = fs::read_to_string(root.join("crates/bitloom/Cargo.toml")).expect("cli toml");
    assert!(toml.contains("name = \"bitloom\""));
    assert!(toml.contains("publish = true"));
    assert!(toml.contains("bitloom-firrtl") && toml.contains("bitloom-viz"));
    assert!(!toml.contains("rhdl-firrtl") && !toml.contains("rhdl-viz"));
    assert!(
        !toml.contains("bitloom-lsp"),
        "FR152(b): no lsp path dep on CLI package"
    );

    let doc = fs::read_to_string(root.join("docs/fr151-bitloom-cli-publish.md")).expect("fr151");
    assert!(doc.contains("FR151") && doc.contains("cargo install bitloom"));
    assert!(doc.contains("cargo-bitloom") || doc.contains("cargo bitloom"));
    assert!(
        doc.contains("FR142")
            && (doc.contains("不") || doc.contains("not") || doc.contains("does **not**")),
        "must not expand FR142 surface"
    );
    assert!(doc.contains("Bitloom") || doc.contains("bitloom"));

    // Live publish evidence (Story 85.5): checklist marks upload.
    assert!(
        doc.contains("Live `cargo publish -p bitloom`")
            || doc.contains("Published bitloom")
            || doc.contains("uploaded"),
        "FR151 doc must track live publish status"
    );
}

#[test]
fn fr151_bitloom_cli_dry_run_publish() {
    let root = workspace_root();
    let out = Command::new("cargo")
        .args(["publish", "-p", "bitloom", "--dry-run", "--allow-dirty"])
        .current_dir(&root)
        .output()
        .expect("spawn dry-run");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "cargo publish -p bitloom --dry-run must succeed\nstdout={stdout}\nstderr={stderr}"
    );
}
