//! Guardrail: Story 36.3 / FR93 historical lock — rewritten under Story 40.4.
//!
//! Phase 11 locked five permanent non-goals behind "须新 PRD". Phase 12 / FR94
//! overturned that lock. These tests now assert the **public surface documents
//! the overturn** and maps the five items to Phase 12 FRs — **not** that an
//! active "须新 PRD" lock remains.
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

fn public_fr93_surface() -> String {
    let (readme, deferred) = readme_and_deferred();
    format!("{readme}\n{deferred}")
}

#[test]
fn fr93_public_section_exists_as_historical_or_remapped() {
    let text = public_fr93_surface();
    assert!(
        (text.contains("永久非目标（FR93）")
            || text.contains("永久非目标 (FR93)")
            || text.contains("## 永久非目标")
            || text.contains("### 永久非目标"))
            && text.contains("FR93"),
        "README and/or deferred-work must retain an explicit FR93 section (historical remapping OK)"
    );
    assert!(
        text.contains("推翻") || text.contains("Phase 12") || text.contains("FR94"),
        "FR93 section must state the Phase 12 / FR94 overturn"
    );
}

#[test]
fn fr93_lists_in_tree_hls_scheduler_as_fr95() {
    let text = public_fr93_surface();
    assert!(
        (text.contains("树内") || text.contains("自研") || text.contains("in-tree"))
            && text.contains("HLS")
            && (text.contains("调度") || text.contains("调度器"))
            && text.contains("FR95"),
        "FR93 remapping must list in-tree / self-built HLS scheduler → FR95"
    );
}

#[test]
fn fr93_lists_firrtl_to_idiomatic_scala_as_fr97() {
    let text = public_fr93_surface();
    assert!(
        (text.contains("idiomatic Scala")
            || text.contains("惯用 Scala")
            || text.contains("idiomatic Chisel")
            || (text.contains("FIRRTL") && text.contains("idiomatic") && text.contains("Scala")))
            && text.contains("FR97"),
        "FR93 remapping must list FIRRTL→idiomatic Scala → FR97"
    );
}

#[test]
fn fr93_lists_tlm_ca_formal_proof_as_fr100_or_fr101() {
    let text = public_fr93_surface();
    assert!(
        (text.contains("TLM≡CA")
            || text.contains("TLM ≡ CA")
            || text.contains("TLM==CA")
            || (text.contains("TLM")
                && text.contains("CA")
                && (text.contains("形式证明") || text.contains("形式化证明"))))
            && (text.contains("FR100") || text.contains("FR101")),
        "FR93 remapping must list TLM≡CA / TLM product → FR100/FR101"
    );
}

#[test]
fn fr93_lists_vip_full_protocol_ip_as_fr98() {
    let text = public_fr93_surface();
    assert!(
        (text.contains("VIP 级全协议")
            || text.contains("VIP级全协议")
            || text.contains("VIP 全协议 IP")
            || text.contains("VIP-level full-protocol")
            || (text.contains("VIP") && text.contains("全协议 IP")))
            && text.contains("FR98"),
        "FR93 remapping must list VIP-level full-protocol IP → FR98"
    );
}

#[test]
fn fr93_lists_keystroke_full_elaboration_netlist_lsp_as_fr99() {
    let text = public_fr93_surface();
    assert!(
        text.contains("LSP")
            && (text.contains("按键全设计")
                || text.contains("按键全 elaborate")
                || text.contains("全设计 elaborate")
                || text.contains("keystroke full")
                || (text.contains("按键")
                    && text.contains("elaborate")
                    && (text.contains("netlist") || text.contains("全设计"))))
            && text.contains("FR99"),
        "FR93 remapping must list keystroke full-elaboration netlist LSP → FR99"
    );
}

#[test]
fn fr93_active_new_prd_lock_is_not_current_requirement() {
    let (readme, deferred) = readme_and_deferred();
    // Historical mention of the old gate is OK if framed as overturned.
    for (label, text) in [("README", &readme), ("deferred-work", &deferred)] {
        if text.contains("须新 PRD") {
            assert!(
                text.contains("历史")
                    || text.contains("曾")
                    || text.contains("已推翻")
                    || text.contains("已被")
                    || text.contains("已批准推翻")
                    || text.contains("原「须新 PRD"),
                "{label}: if '须新 PRD' appears it must be historical / overturn framing, not an active lock"
            );
        }
    }
    // README FR93 section must not present active lock without overturn.
    let fr93_start = readme
        .find("永久非目标")
        .expect("README must have 永久非目标 section");
    let block = &readme[fr93_start..];
    let end = block
        .find("\n## ")
        .or_else(|| block.find("\n详见"))
        .unwrap_or(block.len().min(3000));
    let section = &block[..end];
    assert!(
        section.contains("已推翻")
            || section.contains("已被")
            || section.contains("已批准推翻")
            || section.contains("Phase 12")
            || section.contains("FR94"),
        "README FR93 section must frame overturn / Phase 12 delivery"
    );
    assert!(
        !section.contains("须新 PRD 才能推翻")
            || section.contains("曾")
            || section.contains("历史")
            || section.contains("并写「须新 PRD"),
        "README must not keep '须新 PRD 才能推翻' as an active current lock"
    );
}

#[test]
fn fr93_addendum_retains_historical_pointer_and_phase12_overturn() {
    let root = workspace_root();
    let addendum = fs::read_to_string(
        root.join("_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md"),
    )
    .expect("addendum.md");
    assert!(
        addendum.contains("永久非目标") && addendum.contains("FR93"),
        "PRD addendum must retain FR93 permanent-non-goals historical pointer"
    );
    assert!(
        addendum.contains("Phase 12")
            && (addendum.contains("推翻 FR93")
                || addendum.contains("推翻") && addendum.contains("FR93")),
        "addendum Phase 12 must document FR93 overturn"
    );
    assert!(
        addendum.contains("README")
            || addendum.contains("deferred-work")
            || addendum.contains("状态与 deferred"),
        "addendum FR93 pointer must cite README and/or deferred-work"
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
