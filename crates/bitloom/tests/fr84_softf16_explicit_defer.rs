//! ATDD — Story 35.3 / FR84: SoftF16 synthesizable path **or** explicit defer.
//! Chosen close path: **Option B (explicit defer)**. Red if docs/PRD/NFR14
//! claim synthesizable SoftF16 delivered, omit Path B contract, or leave
//! FR84 unticked / dual-path.

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
fn fr84_user_docs_explicit_defer_and_forbid_synthesizable_claim() {
    let text = read("docs/fr36-rhdl-float.md");

    assert!(
        text.contains("FR84")
            && (text.contains("Option B")
                || text.contains("选项 B")
                || text.contains("Path B")
                || text.contains("显式 defer")
                || text.contains("explicit defer")
                || text.contains("deferred")),
        "fr36 docs must document FR84 Option B / explicit defer"
    );
    assert!(
        text.contains("host-only")
            || text.contains("host only")
            || text.contains("仅 host")
            || text.contains("Host-only"),
        "fr36 docs must state SoftF16 is host-only"
    );
    assert!(
        (text.contains("不得") || text.contains("must not") || text.contains("MUST NOT"))
            && (text.contains("可综合") || text.contains("synthesiz")),
        "fr36 docs must forbid claiming synthesizable SoftF16 / float delivered"
    );
    // Must not market Bits<16> emit as synthesizable float operators
    assert!(
        text.contains("Bits<16>") || text.contains("`Bits<16>`") || text.contains("bitvector"),
        "fr36 docs must situate Bits<16> / bitvector emit surface (not float ops)"
    );
}

#[test]
fn fr84_prd_addendum_option_b_contract() {
    let text = read("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md");

    assert!(
        text.contains("FR84")
            && (text.contains("Option B")
                || text.contains("选项 B")
                || text.contains("Path B")
                || text.contains("显式 defer")
                || text.contains("explicit defer")),
        "PRD addendum must contain FR84 Option B / explicit defer contract"
    );
    assert!(
        (text.contains("不得") || text.contains("must not") || text.contains("MUST NOT"))
            && (text.contains("SoftF16")
                || text.contains("可综合浮点")
                || text.contains("synthesiz")),
        "addendum must forbid claiming synthesizable SoftF16 / float delivered"
    );
    assert!(
        text.contains("NFR37") || text.contains("host-only") || text.contains("FR36"),
        "addendum must situate FR84 relative to FR36 / NFR37 / host-only"
    );
}

#[test]
fn fr84_nfr14_marks_option_b_and_close_checkbox() {
    let text =
        read("_agile-output/implementation-artifacts/nfr14-risk-epic35-residual-partials.md");

    assert!(
        text.contains("FR84")
            && (text.contains("已选 B")
                || text.contains("已选 **B**")
                || text.contains("选项 B") && text.contains("已选")
                || text.contains("Option B")
                    && (text.contains("已选") || text.contains("selected"))),
        "NFR14 record must mark FR84 as Option B selected"
    );
    // Close checklist FR84 ticked (markdown [x])
    assert!(
        text.contains("[x]")
            && text.contains("FR84")
            && (text.contains("选项 B")
                || text.contains("Option B")
                || text.contains("显式 defer")),
        "NFR14 Epic 35 close checklist must tick FR84 (Option B / explicit defer)"
    );
}

#[test]
fn fr84_crate_docs_are_host_only_not_synthesizable_float_ops() {
    let text = read("crates/rhdl-float/src/lib.rs");

    assert!(
        text.contains("SoftF16")
            && (text.contains("host-only")
                || text.contains("Host-only")
                || text.contains("host only")
                || text.contains("host golden")),
        "rhdl-float must document SoftF16 as host-only / host golden"
    );

    let crate_doc: String = text
        .lines()
        .take_while(|l| l.starts_with("//!") || l.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    let lower = crate_doc.to_lowercase();

    // Positive marketing of synthesizable float delivery is forbidden in crate docs.
    assert!(
        !crate_doc.contains("for synthesizable designs")
            && !lower.contains("provides synthesizable")
            && !lower.contains("synthesizable floating-point operator library delivered"),
        "crate root docs must not market SoftF16 as synthesizable designs / delivered float ops"
    );
    assert!(
        lower.contains("defer")
            || lower.contains("not") && lower.contains("synthesiz")
            || crate_doc.contains("不得")
            || crate_doc.contains("FR84"),
        "crate root docs must explicitly defer / deny synthesizable SoftF16 delivery"
    );
    assert!(
        text.contains("Bits<16>") || text.contains("bitvector") || text.contains("bit vector"),
        "implementation comments should situate emit as Bits<16> / bitvector surface"
    );
}
