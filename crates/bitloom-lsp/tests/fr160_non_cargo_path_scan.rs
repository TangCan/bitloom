//! ATDD: FR160 — non-Cargo monorepo path scan (Story 92.2).
//!
//! cargo test -p bitloom-lsp --test fr160_non_cargo_path_scan

use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use bitloom_lsp::{discover_design_roots, discover_design_roots_under};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

fn fixture_non_cargo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/fr160_non_cargo")
}

#[test]
fn fr160_docs_contract_and_forbid_fr118_alone() {
    let text = read("docs/fr160-non-cargo-path-scan.md");
    assert!(text.contains("FR160") && text.contains("Bitloom"));
    assert!(
        text.contains("discover_design_roots_under")
            && (text.contains("非 Cargo")
                || text.contains("non-Cargo")
                || text.contains("Cargo.toml")),
        "must name under-path API / non-Cargo face"
    );
    assert!(
        text.contains("FR118")
            && (text.contains("alone") || text.contains("≠") || text.contains("members")),
        "must keep FR118 distinct / alone ban"
    );
    assert!(
        text.contains("path-not-found")
            || text.contains("PermissionDenied")
            || text.contains("permission"),
        "must document failure semantics"
    );
    let fr118 = read("docs/fr118-syn-scan-design-root-discovery.md");
    assert!(
        fr118.contains("fr160") || fr118.contains("FR160"),
        "FR118 must cross-link FR160"
    );
}

#[test]
fn fr160_fixture_has_no_cargo_toml_and_discovers_top() {
    let root = fixture_non_cargo();
    assert!(
        !root.join("Cargo.toml").is_file(),
        "FR160 fixture must not rely on Cargo.toml"
    );
    let design = root.join("design");
    let roots = discover_design_roots_under(&[design.as_path()]).expect("FR160 discover");
    assert!(
        roots.iter().any(|r| r.root_id == "Fr160BareTop"),
        "must find Fr160BareTop: {roots:?}"
    );
}

#[test]
fn fr160_missing_path_fails_readable() {
    let missing = workspace_root().join("target/bitloom-fr160-does-not-exist");
    let _ = fs::remove_dir_all(&missing);
    let err = discover_design_roots_under(&[missing.as_path()]).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::NotFound);
    let msg = err.to_string();
    assert!(
        msg.contains("path-not-found") && msg.contains("FR160"),
        "missing path must be readable FR160 fail: {msg}"
    );
}

#[test]
fn fr160_fr118_members_discover_still_green() {
    let cargo_fix = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/fr118_syn_ok");
    let roots = discover_design_roots(&cargo_fix).expect("FR118 discover");
    assert_eq!(roots.len(), 1);
    assert_eq!(roots[0].root_id, "Fr118OkCounter");
}

#[cfg(unix)]
#[test]
fn fr160_permission_denied_fails_readable() {
    use std::os::unix::fs::PermissionsExt;

    let dir = workspace_root().join("target/bitloom-fr160-perm");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let mut perms = fs::metadata(&dir).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&dir, perms).unwrap();

    let result = discover_design_roots_under(&[dir.as_path()]);

    // Restore before assert so cleanup always works.
    let mut perms = fs::metadata(&dir).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&dir, perms).unwrap();
    let _ = fs::remove_dir_all(&dir);

    let err = result.expect_err("chmod 000 dir must fail");
    assert_eq!(err.kind(), ErrorKind::PermissionDenied);
    let msg = err.to_string();
    assert!(
        msg.contains("path-permission-denied") && msg.contains("FR160"),
        "permission deny must be readable FR160 fail: {msg}"
    );
}
