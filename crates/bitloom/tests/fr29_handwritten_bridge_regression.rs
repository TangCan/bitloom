//! ATDD / guardrail: FR29 handwritten bridge/abstraction/both regression (Story 21.2).
//! Locks docs + fixture presence; does not implement the FR47 generator.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn fr29_docs_and_mixed_both_fixture_regression_contract() {
    let root = workspace_root();

    let doc_path = root.join("docs/fr29-bridge-abstraction-both.md");
    let doc = fs::read_to_string(&doc_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", doc_path.display()));

    // Handwritten surface still documented
    assert!(
        doc.contains("bridge") && doc.contains("abstraction") && doc.contains("both"),
        "FR29 doc must name bridge / abstraction / both"
    );
    assert!(
        doc.contains("PortValues"),
        "FR29 doc must bind comparison to PortValues"
    );

    // Relationship: handwritten now + generated FR47 upcoming; generation does not replace handwritten
    assert!(
        doc.contains("FR47")
            && (doc.contains("生成") || doc.contains("[Gg]enerat") || doc.contains("Generated")),
        "FR29 doc must relate handwritten path to upcoming FR47 generation"
    );
    assert!(
        (doc.contains("不") && (doc.contains("取代") || doc.contains("替换")))
            || doc.contains("does **not** remove")
            || doc.contains("does not remove")
            || doc.contains("does **not** replace")
            || doc.contains("does not replace"),
        "FR29 doc must state generation does not replace handwritten annotation capability"
    );

    // Bitloom package names (not legacy rhdl-prelude / rhdl-sim as the only names)
    assert!(
        doc.contains("bitloom-prelude") && doc.contains("bitloom_sim"),
        "FR29 doc must cite bitloom-prelude and bitloom_sim"
    );

    // SystemC not contracted
    assert!(
        doc.contains("SystemC")
            && (doc.contains("not") || doc.contains("不") || doc.contains("Non-goals")),
        "FR29 doc must keep SystemC TLM out of contract"
    );

    // Mixed fixture source present with pass + deliberate mismatch
    let fixture = root.join("examples/mixed_both/src/lib.rs");
    let src =
        fs::read_to_string(&fixture).unwrap_or_else(|e| panic!("read {}: {e}", fixture.display()));
    assert!(
        src.contains("#[rhdl::bridge]")
            && src.contains("#[rhdl::abstraction]")
            && src.contains("#[rhdl::both]"),
        "mixed_both fixture must use bridge / abstraction / both attributes"
    );
    assert!(
        src.contains("both_fixture_matches_tick") && src.contains("mismatch_fails"),
        "mixed_both must keep match + deliberate-mismatch regression tests"
    );
    assert!(
        src.contains("check_mixed_both"),
        "mixed_both must call check_mixed_both for PortValues compare"
    );

    // Sim API still exposes mixed-both helper (handwritten path)
    let sim = fs::read_to_string(root.join("crates/bitloom-sim/src/lib.rs"))
        .unwrap_or_else(|e| panic!("read bitloom-sim lib: {e}"));
    assert!(
        sim.contains("fn check_mixed_both"),
        "bitloom-sim must export check_mixed_both"
    );
    // FR47 generator must not appear yet in CLI/sim as a completed product surface
    assert!(
        !sim.contains("generate_functional_sim") && !sim.contains("emit_functional_crate"),
        "Story 21.2 must not ship FR47 generator entry points"
    );
}
