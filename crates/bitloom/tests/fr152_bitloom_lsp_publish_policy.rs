//! ATDD Story 88.2 / FR155 / FR152(a): lsp is publishable; policy (a); dry-run green.
//! Does not claim live crates.io installability (→ Story 88.3 / NFR72).

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
fn fr152_lsp_policy_a_documented_and_dry_run() {
    let root = workspace_root();
    let doc = fs::read_to_string(root.join("docs/fr152-bitloom-lsp-publish-policy.md"))
        .expect("fr152 doc");
    assert!(
        doc.contains("FR152") && (doc.contains("(a)") || doc.contains("策略 (a)")),
        "must document FR152(a)"
    );
    assert!(
        doc.contains("Selected") || doc.contains("选定") || doc.contains("**Selected**"),
        "must mark (a) as selected"
    );
    assert!(
        (doc.contains("publish = true") || doc.contains("publish=true"))
            && doc.contains("bitloom-lsp"),
        "must document lsp publish=true"
    );
    assert!(
        doc.contains("Superseded")
            || doc.contains("升格")
            || doc.contains("(b)") && (doc.contains("仍有效") || doc.contains("closed")),
        "must honesty-note (b) superseded / still historically closed"
    );
    assert!(
        doc.contains("≠")
            || doc.contains("not")
            || doc.contains("不是")
            || doc.contains("≠ shipping"),
        "must state (a) ≠ deeper LSP product"
    );
    assert!(
        (doc.contains("88.3") || doc.contains("live") || doc.contains("NFR72"))
            && (doc.contains("不得")
                || doc.contains("do **not**")
                || doc.contains("Do not")
                || doc.contains("until")),
        "must defer crates.io install claim until live publish"
    );
    assert!(doc.contains("Bitloom") || doc.contains("bitloom"));
    assert!(
        doc.contains("FR151")
            && (doc.contains("independent")
                || doc.contains("不得")
                || doc.contains("must **not**")
                || doc.contains("remain")),
        "must protect FR151 CLI path"
    );

    let lsp = fs::read_to_string(root.join("crates/bitloom-lsp/Cargo.toml")).expect("lsp toml");
    assert!(
        lsp.contains("publish = true"),
        "bitloom-lsp must be publish=true under (a):\n{lsp}"
    );
    assert!(
        !lsp.contains("publish = false"),
        "bitloom-lsp must not keep publish=false:\n{lsp}"
    );
    assert!(
        lsp.contains("repository") && lsp.contains("documentation"),
        "packaging metadata required for crates.io:\n{lsp}"
    );

    let cli = fs::read_to_string(root.join("crates/bitloom/Cargo.toml")).expect("cli toml");
    assert!(
        !cli.contains("bitloom-lsp"),
        "bitloom must not declare bitloom-lsp path dep:\n{cli}"
    );

    let lsp_dry = Command::new("cargo")
        .args(["publish", "-p", "bitloom-lsp", "--dry-run", "--allow-dirty"])
        .current_dir(&root)
        .output()
        .expect("spawn lsp dry-run");
    let lsp_err = String::from_utf8_lossy(&lsp_dry.stderr);
    let lsp_out = String::from_utf8_lossy(&lsp_dry.stdout);
    assert!(
        lsp_dry.status.success(),
        "cargo publish -p bitloom-lsp --dry-run must succeed\nstdout={lsp_out}\nstderr={lsp_err}"
    );

    let cli_dry = Command::new("cargo")
        .args(["publish", "-p", "bitloom", "--dry-run", "--allow-dirty"])
        .current_dir(&root)
        .output()
        .expect("spawn bitloom dry-run");
    let cli_err = String::from_utf8_lossy(&cli_dry.stderr);
    let cli_out = String::from_utf8_lossy(&cli_dry.stdout);
    assert!(
        cli_dry.status.success(),
        "cargo publish -p bitloom --dry-run must remain green (FR151)\nstdout={cli_out}\nstderr={cli_err}"
    );
}
