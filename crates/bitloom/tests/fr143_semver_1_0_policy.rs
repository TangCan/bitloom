//! ATDD: FR143 SemVer 1.0 policy document.
//!
//! ```text
//! cargo test -p bitloom --test fr143_semver_1_0_policy
//! ```

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    let path = workspace_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn fr143_semver_1_0_policy_exists_with_required_sections() {
    let text = read("docs/semver-1-0-policy.md");
    assert!(text.contains("FR143"), "must cite FR143");
    assert!(
        text.contains("public-api-1-0-surface") || text.contains("FR142"),
        "must cross-link surface / FR142"
    );
    assert!(
        text.contains("semver-0x-policy") || text.contains("NFR15"),
        "must cross-link 0.x policy / NFR15"
    );
    assert!(
        text.contains("breaking") || text.contains("破坏"),
        "must discuss breaking changes"
    );
    assert!(
        text.contains("major")
            && (text.contains("in-surface") || text.contains("表面") || text.contains("FR142")),
        "must state in-surface breaking → major"
    );
    assert!(
        text.contains("sprint") || text.contains("backlog") || text.contains("自动"),
        "must state closing sprint ≠ automatic major"
    );
    assert!(
        text.contains("MSRV") || text.contains("1.97"),
        "must document MSRV / Q5"
    );
    assert!(
        text.contains("deprecat") || text.contains("弃用"),
        "must document deprecation window"
    );
    assert!(
        text.contains("semver-check") || text.contains("FR144") || text.contains("cargo-semver"),
        "must point at FR144 CI gate"
    );
    assert!(
        text.contains("Bitloom") || text.contains("bitloom"),
        "must cite Bitloom brand"
    );
    assert!(
        text.contains("authorizes") || text.contains("授权") || text.contains("1.0.0"),
        "must authorize promoting surface to 1.0"
    );
}

#[test]
fn fr143_semver_0x_policy_cross_links_1_0() {
    let text = read("docs/semver-0x-policy.md");
    assert!(
        text.contains("semver-1-0-policy") || text.contains("1.0 policy") || text.contains("FR143"),
        "0.x policy must point forward to 1.0 / FR143"
    );
}

#[test]
fn fr143_surface_doc_points_at_1_0_policy() {
    let text = read("docs/public-api-1-0-surface.md");
    assert!(
        text.contains("semver-1-0-policy"),
        "surface doc must link semver-1-0-policy.md (not only future tense)"
    );
}
