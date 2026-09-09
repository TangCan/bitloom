//! ATDD / guardrail: Story 36.3 / FR93 — permanent non-goals lock + new-PRD gate.
//!
//! Red until README and/or deferred-work contain an explicit FR93 lock listing
//! all five permanent non-goals, state that a new PRD is required to overturn,
//! and PRD addendum points at that public list (not merely FR87–FR93 inventory).
//!
//! ```text
//! cargo test -p bitloom --test fr93_permanent_non_goals
//! ```

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn readme_and_deferred() -> (String, String) {
    let root = workspace_root();
    let readme = fs::read_to_string(root.join("README.md")).expect("README.md");
    let deferred =
        fs::read_to_string(root.join("_agile-output/implementation-artifacts/deferred-work.md"))
            .expect("deferred-work.md");
    (readme, deferred)
}

/// Public contract surface: README and/or deferred-work must lock FR93.
fn public_fr93_surface() -> String {
    let (readme, deferred) = readme_and_deferred();
    format!("{readme}\n{deferred}")
}

fn has_fr93_lock_heading(text: &str) -> bool {
    // Require an explicit lock heading — not only the 36.2 "→ Story 36.3" pointer.
    let headed = text.contains("永久非目标（FR93）")
        || text.contains("永久非目标 (FR93)")
        || text.contains("## 永久非目标")
        || text.contains("### 永久非目标")
        || text.contains("FR93 永久非目标");
    headed && (text.contains("须新 PRD") || text.contains("须**新 PRD**"))
}

#[test]
fn fr93_public_lock_heading_exists() {
    let text = public_fr93_surface();
    assert!(
        has_fr93_lock_heading(&text)
            || ((text.contains("永久非目标") && text.contains("FR93"))
                && (text.contains("须新 PRD") || text.contains("须**新 PRD**"))),
        "README and/or deferred-work must publish an explicit FR93 permanent-non-goals lock \
         (not only a pointer to Story 36.3)"
    );
}

#[test]
fn fr93_lists_in_tree_hls_scheduler() {
    let text = public_fr93_surface();
    assert!(
        (text.contains("树内") || text.contains("自研") || text.contains("in-tree"))
            && text.contains("HLS")
            && text.contains("调度器"),
        "FR93 lock must list in-tree / self-built HLS scheduler"
    );
    assert!(
        text.contains("永久非目标") && text.contains("FR93"),
        "HLS scheduler non-goal must sit under FR93 permanent-non-goals framing"
    );
}

#[test]
fn fr93_lists_firrtl_to_idiomatic_scala() {
    let text = public_fr93_surface();
    assert!(
        (text.contains("idiomatic Scala")
            || text.contains("惯用 Scala")
            || text.contains("idiomatic Chisel")
            || (text.contains("FIRRTL") && text.contains("idiomatic") && text.contains("Scala"))),
        "FR93 lock must list FIRRTL→idiomatic Scala as permanent non-goal"
    );
}

#[test]
fn fr93_lists_tlm_ca_formal_proof() {
    let text = public_fr93_surface();
    assert!(
        (text.contains("TLM≡CA")
            || text.contains("TLM ≡ CA")
            || text.contains("TLM==CA")
            || (text.contains("TLM")
                && text.contains("CA")
                && (text.contains("形式证明") || text.contains("形式化证明"))))
            && (text.contains("形式证明")
                || text.contains("形式化证明")
                || text.contains("formal proof")
                || text.contains("≡")),
        "FR93 lock must list default TLM≡CA formal proof as permanent non-goal"
    );
}

#[test]
fn fr93_lists_vip_full_protocol_ip() {
    let text = public_fr93_surface();
    assert!(
        (text.contains("VIP 级全协议")
            || text.contains("VIP级全协议")
            || text.contains("VIP 全协议 IP")
            || text.contains("VIP-level full-protocol")
            || (text.contains("VIP") && text.contains("全协议 IP"))),
        "FR93 lock must list VIP-level full-protocol IP with distinctive phrasing"
    );
}

#[test]
fn fr93_lists_keystroke_full_elaboration_netlist_lsp() {
    let text = public_fr93_surface();
    assert!(
        text.contains("LSP")
            && (text.contains("按键全设计")
                || text.contains("按键全 elaborate")
                || text.contains("全设计 elaborate")
                || text.contains("keystroke full")
                || (text.contains("按键")
                    && text.contains("elaborate")
                    && (text.contains("netlist") || text.contains("全设计")))),
        "FR93 lock must list keystroke full-elaboration netlist LSP"
    );
}

#[test]
fn fr93_requires_new_prd_to_overturn() {
    let text = public_fr93_surface();
    assert!(
        (text.contains("须新 PRD")
            || text.contains("须**新 PRD**")
            || text.contains("必须新 PRD")
            || text.contains("requires a new PRD")
            || text.contains("new PRD required"))
            && (text.contains("推翻") || text.contains("overturn") || text.contains("撤销")),
        "public FR93 lock must state a new PRD is required to overturn the non-goals"
    );
}

#[test]
fn fr93_addendum_points_to_permanent_non_goals() {
    let root = workspace_root();
    let addendum = fs::read_to_string(
        root.join("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md"),
    )
    .expect("addendum.md");
    // Inventory alone (FR87–FR93) is insufficient — need an explicit FR93 pointer block.
    assert!(
        addendum.contains("永久非目标")
            && addendum.contains("FR93")
            && (addendum.contains("须新 PRD")
                || addendum.contains("新 PRD")
                || addendum.contains("除非新 PRD")),
        "PRD addendum must have an explicit FR93 permanent-non-goals pointer (incl. new-PRD gate)"
    );
    assert!(
        addendum.contains("README")
            || addendum.contains("deferred-work")
            || addendum.contains("状态与 deferred"),
        "addendum FR93 pointer must cite README and/or deferred-work as the public list"
    );
}

#[test]
fn fr93_brand_remains_bitloom() {
    let (readme, _) = readme_and_deferred();
    assert!(
        readme.contains("Bitloom") && (readme.contains("bitloom") || readme.contains("`bitloom")),
        "README must keep Bitloom / bitloom-* branding"
    );
}
