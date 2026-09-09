//! ATDD / guardrail: Story 40.4 / FR94 / NFR41 — revise AD-5/25/27 + unlock
//! FR93 permanent non-goals (point to Phase 12 FRs).
//!
//! Red until ARCHITECTURE-SPINE AD-5/25/27 + Deferred, README, and
//! deferred-work align with Path B (no current "须新 PRD" lock).
//!
//! ```text
//! cargo test -p bitloom --test fr94_ad_fr93_unlock
//! ```

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read_spine() -> String {
    let path = workspace_root().join(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn readme_and_deferred() -> (String, String) {
    let root = workspace_root();
    let readme = fs::read_to_string(root.join("README.md")).expect("README.md");
    let deferred =
        fs::read_to_string(root.join("_agile-output/implementation-artifacts/deferred-work.md"))
            .expect("deferred-work.md");
    (readme, deferred)
}

fn public_surface() -> String {
    let (readme, deferred) = readme_and_deferred();
    format!("{readme}\n{deferred}")
}

/// Extract AD-5 section (until next ### AD-).
fn ad_section(spine: &str, heading_substr: &str) -> String {
    let start = spine
        .find(heading_substr)
        .unwrap_or_else(|| panic!("missing heading containing {heading_substr}"));
    let rest = &spine[start..];
    let end = rest[3..]
        .find("\n### AD-")
        .map(|i| i + 3)
        .unwrap_or(rest.len());
    rest[..end].to_string()
}

#[test]
fn fr94_ad5_allows_tlm_product_path() {
    let ad5 = ad_section(&read_spine(), "### AD-5");
    assert!(
        ad5.contains("FR101")
            && (ad5.contains("SystemC") || ad5.contains("TLM"))
            && (ad5.contains("允许")
                || ad5.contains("产品路径")
                || ad5.contains("Phase 12")
                || ad5.contains("Path B")),
        "AD-5 must allow SystemC TLM-2.0 product path (FR101) under Path B"
    );
    assert!(
        ad5.contains("2026-09-09") || ad5.contains("FR94") || ad5.contains("NFR41"),
        "AD-5 must record Phase 12 / FR94 / NFR41 revision"
    );
    // Must not retain absolute "不承诺 / 不要求 … SystemC TLM-2.0" as the sole rule.
    let forbids_only = ad5.contains("**不**承诺 / **不**要求从 HIR 降低 **SystemC TLM-2.0**")
        && !ad5.contains("FR101");
    assert!(
        !forbids_only,
        "AD-5 must not leave the Phase-11 absolute TLM non-contract as the only rule"
    );
}

#[test]
fn fr94_ad25_allows_in_tree_hls() {
    let ad25 = ad_section(&read_spine(), "### AD-25");
    assert!(
        ad25.contains("FR95")
            && (ad25.contains("树内") || ad25.contains("自研") || ad25.contains("in-tree"))
            && (ad25.contains("调度")
                || ad25.contains("scheduling")
                || ad25.contains("allocation")),
        "AD-25 must allow in-tree HLS scheduling (FR95)"
    );
    assert!(
        !(ad25.contains("禁止 bitloom/rhdl crate 实现 scheduling/allocation")
            && !ad25.contains("FR95")),
        "AD-25 must not keep absolute ban on in-tree scheduling without FR95 Path B carve-out"
    );
    assert!(
        ad25.contains("2026-09-09") || ad25.contains("FR94") || ad25.contains("Path B"),
        "AD-25 must record Phase 12 Path B revision"
    );
}

#[test]
fn fr94_ad27_adds_idiomatic_acceptance() {
    let ad27 = ad_section(&read_spine(), "### AD-27");
    assert!(
        ad27.contains("FR97")
            && (ad27.contains("idiomatic") || ad27.contains("惯用") || ad27.contains("可维护")),
        "AD-27 must add idiomatic / maintainable acceptance face (FR97)"
    );
    assert!(
        ad27.contains("FR28") || ad27.contains("FR46") || ad27.contains("可编译"),
        "AD-27 must retain mechanical compilable path (FR28/FR46)"
    );
    assert!(
        ad27.contains("2026-09-09") || ad27.contains("FR94") || ad27.contains("Path B"),
        "AD-27 must record Phase 12 Path B revision"
    );
}

#[test]
fn fr94_spine_deferred_path_b_pointer() {
    let spine = read_spine();
    let deferred_idx = spine
        .find("## Deferred")
        .expect("ARCHITECTURE-SPINE must have ## Deferred");
    let deferred = &spine[deferred_idx..];
    assert!(
        deferred.contains("FR94")
            && (deferred.contains("FR95")
                || deferred.contains("FR105")
                || deferred.contains("Phase 12")),
        "Deferred section must point at Phase 12 FR94–105 completion contract"
    );
    assert!(
        !(deferred.contains("永久非目标")
            && deferred.contains("FR93")
            && (deferred.contains("以 PRD/epics 为准") || deferred.contains("须新 PRD"))
            && !deferred.contains("推翻")),
        "Deferred must not keep FR93 permanent-non-goals as the active completion lock"
    );
}

#[test]
fn fr94_readme_deferred_fr93_unlocked() {
    let text = public_surface();
    // Five items mapped to Phase 12 FRs
    assert!(
        text.contains("FR95")
            && text.contains("FR97")
            && text.contains("FR98")
            && text.contains("FR99"),
        "public surface must map overturned FR93 items to Phase 12 FRs (at least FR95/97/98/99)"
    );
    assert!(
        (text.contains("FR100") || text.contains("FR101"))
            && (text.contains("TLM") || text.contains("形式")),
        "public surface must map TLM/formal items to FR100/FR101"
    );
    assert!(
        text.contains("推翻") || text.contains("Phase 12") || text.contains("FR94"),
        "public surface must state FR93 lock overturned / Phase 12"
    );
    // Current lock phrase must not remain as active requirement on README FR93 section.
    let readme = readme_and_deferred().0;
    let fr93_start = readme
        .find("永久非目标")
        .expect("README must still have a 永久非目标 section (historical or remapped)");
    let fr93_block = &readme[fr93_start..];
    let fr93_end = fr93_block
        .find("\n## ")
        .or_else(|| fr93_block.find("\n详见"))
        .unwrap_or(fr93_block.len().min(2500));
    let section = &fr93_block[..fr93_end];
    assert!(
        !(section.contains("须新 PRD 才能推翻")
            && !section.contains("历史")
            && !section.contains("已推翻")
            && !section.contains("已被")),
        "README FR93 section must not keep active '须新 PRD 才能推翻' lock without historical/overturn framing"
    );
    // Stronger: after unlock, active current-lock wording should be gone or clearly historical.
    assert!(
        section.contains("已推翻")
            || section.contains("已被")
            || section.contains("Phase 12")
            || section.contains("FR94")
            || section.contains("交付"),
        "README FR93 section must rewire to Phase 12 delivery / overturn framing"
    );
}

#[test]
fn fr94_nfr41_ad_gate_stated() {
    let spine = read_spine();
    let nfr14 = fs::read_to_string(
        workspace_root()
            .join("_agile-output/implementation-artifacts/nfr14-risk-epic40-literal-path-b.md"),
    )
    .expect("nfr14-risk-epic40");
    let combined = format!("{spine}\n{nfr14}");
    assert!(
        combined.contains("NFR41")
            && (combined.contains("已修订 AD")
                || combined.contains("修订后 AD")
                || combined.contains("引用已修订")
                || combined.contains("须引用")
                || combined.contains("实现 epic")),
        "NFR41 gate (implement epics must cite revised ADs) must be stated in spine and/or NFR14 record"
    );
}

#[test]
fn fr94_ad_unlock_brand_bitloom() {
    let (readme, _) = readme_and_deferred();
    assert!(
        readme.contains("Bitloom") && (readme.contains("bitloom") || readme.contains("`bitloom")),
        "README must keep Bitloom / bitloom-* branding"
    );
}
