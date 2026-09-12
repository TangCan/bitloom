//! ATDD Story 88.3 / FR155: bitloom-lsp live publish evidence + install docs.
//! Does not perform live upload in CI (credentials / index); locks documentation.

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
fn fr155_bitloom_lsp_live_publish_documented() {
    let root = workspace_root();
    let toml = fs::read_to_string(root.join("crates/bitloom-lsp/Cargo.toml")).expect("lsp toml");
    assert!(toml.contains("name = \"bitloom-lsp\""));
    assert!(toml.contains("publish = true"));
    assert!(!toml.contains("publish = false"));
    assert!(toml.contains("bitloom-builder") && toml.contains("bitloom-hir"));

    let doc = fs::read_to_string(root.join("docs/fr155-bitloom-lsp-publish.md")).expect("fr155");
    assert!(doc.contains("FR155") && doc.contains("bitloom-lsp"));
    assert!(doc.contains("1.0.0"));
    assert!(
        doc.contains("cargo install bitloom-lsp"),
        "must document install path"
    );
    assert!(
        doc.contains("Live `cargo publish -p bitloom-lsp`")
            || doc.contains("Published bitloom-lsp")
            || doc.contains("uploaded"),
        "FR155 doc must track live publish status"
    );
    assert!(
        (doc.contains("≠") || doc.contains("not") || doc.contains("不是"))
            && (doc.contains("deeper") || doc.contains("加深") || doc.contains("feature")),
        "must state (a) ≠ deeper LSP product"
    );
    assert!(
        doc.contains("FR151")
            && (doc.contains("independent") || doc.contains("不得") || doc.contains("remain")),
        "must protect FR151 boundary"
    );
    assert!(doc.contains("Bitloom") || doc.contains("bitloom"));
    assert!(
        !doc.contains("rhdl-lsp") && !doc.contains("publish `rhdl`"),
        "must not use rhdl publish name"
    );

    let policy = fs::read_to_string(root.join("docs/fr152-bitloom-lsp-publish-policy.md"))
        .expect("fr152 policy");
    assert!(
        policy.contains("(a)") && (policy.contains("Selected") || policy.contains("**Selected**")),
        "policy must remain (a)"
    );

    let cli = fs::read_to_string(root.join("crates/bitloom/Cargo.toml")).expect("cli");
    assert!(
        !cli.contains("bitloom-lsp"),
        "FR151: bitloom must not gain bitloom-lsp dep"
    );
}

#[test]
fn fr155_bitloom_lsp_dry_run_still_green() {
    let root = workspace_root();
    let out = Command::new("cargo")
        .args(["publish", "-p", "bitloom-lsp", "--dry-run", "--allow-dirty"])
        .current_dir(&root)
        .output()
        .expect("spawn dry-run");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "cargo publish -p bitloom-lsp --dry-run must succeed\nstdout={stdout}\nstderr={stderr}"
    );
}
