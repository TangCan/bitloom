//! ATDD Story 116.2 / FR183 — explicit FR142 public API surface expand.
//!
//! ```text
//! cargo test -p bitloom --test fr183_explicit_fr142_api_expand
//! ```

use std::fs;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr183_docs_contract_forbid_silent_expand() {
    let docs = read("docs/fr183-explicit-fr142-api-expand.md");
    assert!(docs.contains("FR183") && docs.contains("Bitloom"));
    assert!(
        docs.contains("emit_chisel")
            && docs.contains("BitloomFirrtlParser")
            && (docs.contains("CHISEL_TARGET") || docs.contains("FIRTOOL_TARGET")),
        "must name promoted interop entries"
    );
    assert!(
        docs.contains("FR142")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("仍")),
        "must keep FR142 distinct"
    );
    assert!(
        (docs.contains("静默") || docs.contains("silent"))
            && (docs.contains("扩大") || docs.contains("expand")),
        "must forbid silent expand"
    );
    assert!(
        docs.contains("AD-6") && (docs.contains("prelude") || docs.contains("bitloom-prelude")),
        "must keep AD-6 design-crate boundary"
    );
    assert!(
        docs.contains("SemVer") || docs.contains("minor") || docs.contains("FR143"),
        "must state SemVer honesty"
    );
    assert!(
        docs.contains("NFR86") || docs.contains("Further"),
        "must leave further expands as NFR86"
    );
}

#[test]
fn fr183_surface_doc_lists_expand() {
    let surface = read("docs/public-api-1-0-surface.md");
    assert!(
        surface.contains("FR183")
            && surface.contains("bitloom-firrtl")
            && surface.contains("emit_chisel")
            && surface.contains("BitloomFirrtlParser"),
        "public-api surface must list FR183 firrtl interop promote"
    );
    assert!(
        surface.contains("CHISEL_TARGET") && surface.contains("FIRTOOL_TARGET"),
        "surface must list pin constants"
    );
    assert!(
        surface.contains("parseUpdateMainline"),
        "surface must list update-mainline Parser entry"
    );
    assert!(
        surface.contains("AD-6")
            && surface.contains("bitloom-firrtl")
            && (surface.contains("must **not** depend")
                || surface.contains("must not depend")
                || surface.contains("不得")),
        "surface must forbid design-crate dependency on bitloom-firrtl"
    );
    assert!(
        surface.contains("minor") && (surface.contains("additive") || surface.contains("Additive")),
        "surface must document additive → minor SemVer honesty"
    );
    assert!(
        surface.contains("undocumented") && surface.contains("FR183"),
        "must carve undocumented firrtl pub as still out-of-surface under FR183"
    );
}

#[test]
fn fr183_ad6_prelude_does_not_depend_on_firrtl() {
    let prelude = read("crates/bitloom-prelude/Cargo.toml");
    assert!(
        !prelude.contains("bitloom-firrtl") && !prelude.contains("rhdl-firrtl"),
        "bitloom-prelude must not depend on bitloom-firrtl (AD-6)"
    );
}

#[test]
fn fr183_nfr14_selected_entries_still_documented() {
    let nfr14 = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic116-explicit-fr142-api-expand-fr183.md",
    );
    assert!(
        nfr14.contains("emit_chisel")
            && nfr14.contains("BitloomFirrtlParser")
            && nfr14.contains("public-api-1-0-surface"),
        "NFR14 selected S1 must remain the authority for this expand"
    );
}

#[test]
fn fr183_spine_or_semver_policy_honesty() {
    let semver = read("docs/semver-1-0-policy.md");
    assert!(
        semver.contains("Expanding in-surface")
            || semver.contains("additive")
            || semver.contains("minor"),
        "semver policy must allow documented additive expands as minor"
    );
}
