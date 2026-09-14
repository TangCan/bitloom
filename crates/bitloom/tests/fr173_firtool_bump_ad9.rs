//! ATDD Story 106.2 / FR173 — firtool bump beyond AD-9 with Chisel pairing.
//!
//! Close evidence remains **1.158.0 ↔ 7.15.0**. Live AD-9 product pin later moved
//! to **1.159.0** via FR182 (unpaired); those live assertions live in `fr182_*`.
//!
//! ```text
//! cargo test -p bitloom --test fr173_firtool_bump_ad9
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
fn fr173_docs_and_ad9_pin_chisel_715_firtool_1158() {
    let docs = read("docs/fr173-firtool-bump-ad9.md");
    assert!(docs.contains("FR173") && docs.contains("Bitloom"));
    assert!(
        docs.contains("7.15.0") && (docs.contains("1.158.0") || docs.contains("firtool-1.158")),
        "docs must pin Chisel 7.15.0 ↔ firtool-1.158.0"
    );
    assert!(
        docs.contains("FR169")
            && (docs.contains("alone") || docs.contains("≠") || docs.contains("仍")),
        "must keep FR169 distinct"
    );
    assert!(
        (docs.contains("PATH") || docs.contains("HEAD"))
            && (docs.contains("Forbidden") || docs.contains("禁止") || docs.contains("≠")),
        "must forbid PATH-random / unpaired HEAD"
    );

    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("firtool-1.158.0") && spine.contains("7.15.0"),
        "AD-9 / Stack must retain FR173 firtool-1.158.0 ↔ Chisel 7.15.0 close evidence"
    );
    assert!(
        spine.contains("FR173") && (spine.contains("Revised") || spine.contains("修订")),
        "spine must record FR173 AD-9 revise"
    );
    assert!(
        spine.contains("NFR78")
            || (spine.contains("1.155.0") && (spine.contains("仍") || spine.contains("历史"))),
        "spine must keep NFR78 historical pin honesty"
    );
}

#[test]
fn fr173_close_docs_not_rewritten_by_fr182() {
    let docs = read("docs/fr173-firtool-bump-ad9.md");
    assert!(
        docs.contains("1.158.0")
            && (docs.contains("closed") || docs.contains("已关闭") || docs.contains("106.3")),
        "FR173 close @ 1.158.0 must remain"
    );
    assert!(
        docs.contains("FR182") || docs.contains("1.159.0"),
        "FR173 docs should acknowledge live pin moved via FR182"
    );
}

#[test]
fn fr173_nfr78_prior_closes_still_documented() {
    let fr169 = read("docs/fr169-circt-mlir-allocation.md");
    assert!(
        fr169.contains("closed") || fr169.contains("已关闭") || fr169.contains("102.3"),
        "FR169 close evidence must remain"
    );
    assert!(
        fr169.contains("1.155.0"),
        "FR169 historical close pin 1.155.0 must remain documented"
    );
}
