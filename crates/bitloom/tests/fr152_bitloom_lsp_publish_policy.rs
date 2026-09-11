//! ATDD Story 85.4 / FR152(b): lsp stays unpublished and must not block bitloom publish.

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
fn fr152_lsp_policy_b_documented_and_unblocks_cli() {
    let root = workspace_root();
    let doc = fs::read_to_string(root.join("docs/fr152-bitloom-lsp-publish-policy.md"))
        .expect("fr152 doc");
    assert!(doc.contains("FR152") && (doc.contains("(b)") || doc.contains("策略 (b)")));
    assert!(
        doc.contains("publish = false") || doc.contains("publish=false"),
        "must document lsp publish=false default"
    );
    assert!(
        doc.contains("不挡")
            || doc.contains("does **not** block")
            || doc.contains("must not block")
            || doc.contains("without"),
        "must document non-blocking publish"
    );
    assert!(
        doc.contains("(a)")
            && (doc.contains("新合同") || doc.contains("Deferred") || doc.contains("deferred")),
        "must note (a) needs a new contract"
    );
    assert!(doc.contains("Bitloom") || doc.contains("bitloom"));

    let lsp = fs::read_to_string(root.join("crates/bitloom-lsp/Cargo.toml")).expect("lsp toml");
    assert!(
        lsp.contains("publish = false"),
        "bitloom-lsp must remain publish=false under (b):\n{lsp}"
    );

    let cli = fs::read_to_string(root.join("crates/bitloom/Cargo.toml")).expect("cli toml");
    assert!(
        !cli.contains("bitloom-lsp"),
        "bitloom must not declare bitloom-lsp path dep that could affect packaging:\n{cli}"
    );

    // Packaged crate must not require bitloom-lsp from the registry.
    let out = Command::new("cargo")
        .args(["publish", "-p", "bitloom", "--dry-run", "--allow-dirty"])
        .current_dir(&root)
        .output()
        .expect("spawn dry-run");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        out.status.success(),
        "cargo publish -p bitloom --dry-run must succeed without bitloom-lsp on registry\nstdout={stdout}\nstderr={stderr}"
    );
    assert!(
        !stderr.contains("bitloom-lsp")
            || !stderr.to_lowercase().contains("does not specify a version"),
        "dry-run must not fail on bitloom-lsp version requirement:\n{stderr}"
    );
}
