//! ATDD Story 123.2 / FR190 — further explicit FR142 surface expand beyond FR183.
//!
//! ```text
//! cargo test -p bitloom --test fr190_further_fr142_api_expand
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
fn fr190_docs_contract_forbid_fr183_alone() {
    let docs = read("docs/fr190-further-fr142-api-expand.md");
    assert!(docs.contains("FR190") && docs.contains("Bitloom"));
    assert!(
        docs.contains("emit")
            && docs.contains("import")
            && (docs.contains("check_") || docs.contains("check_idiomatic")),
        "must name promoted emit/import/check entries"
    );
    assert!(
        docs.contains("FR183")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("仍")),
        "must keep FR183 distinct"
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
        docs.contains("NFR91") || docs.contains("Further") || docs.contains("超子集"),
        "must leave further expands as NFR91"
    );
}

#[test]
fn fr190_surface_doc_lists_expand() {
    let surface = read("docs/public-api-1-0-surface.md");
    assert!(
        surface.contains("FR190")
            && surface.contains("bitloom-firrtl")
            && surface.contains("ports_roundtrip_ok")
            && surface.contains("instance_graph_roundtrip_ok"),
        "public-api surface must list FR190 FIRRTL text/roundtrip promote"
    );
    assert!(
        surface.contains("check_idiomatic_chisel")
            && surface.contains("check_chisel_style_guide_pack_fr188"),
        "surface must list documented check_* family"
    );
    assert!(
        surface.contains("`emit`") || surface.contains("| `emit`"),
        "surface must list FIRRTL emit"
    );
    assert!(
        surface.contains("`import`") || surface.contains("| `import`"),
        "surface must list FIRRTL import"
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
        surface.contains("FR183") && surface.contains("FR190"),
        "must keep FR183 baseline and FR190 expand both visible"
    );
}

#[test]
fn fr190_ad6_prelude_does_not_depend_on_firrtl() {
    let prelude = read("crates/bitloom-prelude/Cargo.toml");
    assert!(
        !prelude.contains("bitloom-firrtl") && !prelude.contains("rhdl-firrtl"),
        "bitloom-prelude must not depend on bitloom-firrtl (AD-6)"
    );
}

#[test]
fn fr190_nfr14_selected_entries_still_documented() {
    let nfr14 = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic123-further-fr142-api-expand-fr190.md",
    );
    assert!(
        nfr14.contains("emit")
            && nfr14.contains("import")
            && nfr14.contains("check_")
            && nfr14.contains("public-api-1-0-surface"),
        "NFR14 selected S1–S2 must remain the authority for this expand"
    );
}

#[test]
fn fr190_fr183_alone_not_sufficient() {
    let docs = read("docs/fr183-explicit-fr142-api-expand.md");
    assert!(
        docs.contains("FR190") || docs.contains("NFR91"),
        "FR183 docs must point further expands to FR190/NFR91"
    );
}
