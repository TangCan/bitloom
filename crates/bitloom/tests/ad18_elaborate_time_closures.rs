//! ATDD / guardrail: AD-18 distinguishes capturing vs elaborate-time closures
//! (Story 26.3 / FR72 / NFR35). Locks the architecture contract; does not
//! implement positive closure APIs (Epic 27+).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn ad18_section(spine: &str) -> &str {
    spine
        .split("### AD-18")
        .nth(1)
        .and_then(|rest| rest.split("### AD-19").next())
        .expect("AD-18 section present")
}

#[test]
fn ad18_keeps_capturing_ban_and_allows_elaborate_time_non_capturing() {
    let spine_path = workspace_root().join(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    let spine = fs::read_to_string(&spine_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", spine_path.display()));
    let ad18 = ad18_section(&spine);

    assert!(
        ad18.contains("[ADOPTED]") || spine.contains("### AD-18 — 阶段一语言表面 [ADOPTED]"),
        "AD-18 must remain ADOPTED"
    );

    // Ban: cycle-accurate path rejects capturing closures (+ heap/dyn/etc.)
    assert!(
        (ad18.contains("周期精确") || ad18.contains("周期精确路径"))
            && ad18.contains("捕获闭包")
            && (ad18.contains("拒绝") || ad18.contains("禁止")),
        "AD-18 must keep cycle-accurate ban on capturing closures"
    );
    assert!(
        ad18.contains("Vec")
            && ad18.contains("dyn Trait")
            && (ad18.contains("堆") || ad18.contains("Box")),
        "AD-18 must keep heap/dyn (etc.) bans on cycle-accurate path"
    );

    // Allow: elaborate-time non-capturing Fn dissolve before freeze
    assert!(
        (ad18.contains("elaborate-time") || ad18.contains("elaborate"))
            && ad18.contains("非捕获")
            && (ad18.contains("`Fn`") || ad18.contains("Fn"))
            && (ad18.contains("允许") || ad18.contains("**允许**")),
        "AD-18 must allow elaborate-time non-capturing Fn (or documented equiv.)"
    );
    assert!(
        (ad18.contains("freeze") || ad18.contains("`freeze`"))
            && (ad18.contains("消解") || ad18.contains("溶解")),
        "AD-18 must require dissolve to HIR before freeze"
    );
    assert!(
        ad18.contains("tick")
            && (ad18.contains("不得") || ad18.contains("禁止"))
            && (ad18.contains("闭包对象") || ad18.contains("Rust 闭包")),
        "AD-18 must forbid entering tick as a Rust closure object"
    );

    // AD-1 + AD-7/13 via ElaborateSession
    assert!(
        ad18.contains("AD-1") && (ad18.contains("rustc") || ad18.contains("抽网表")),
        "AD-18 must obey AD-1 (no rustc-time netlist extract)"
    );
    assert!(
        ad18.contains("ElaborateSession")
            && (ad18.contains("AD-7") || ad18.contains("AD-13"))
            && ad18.contains("AD-13"),
        "AD-18 must route via ElaborateSession (AD-7/13)"
    );

    // Revised date
    let revised = ad18
        .lines()
        .find(|l| l.contains("**Revised:**") || l.contains("Revised:"))
        .expect("AD-18 Revised line present");
    assert!(
        revised.contains("2026-09-08"),
        "AD-18 Revised must date 2026-09-08"
    );
    assert!(
        revised.contains("捕获")
            && (revised.contains("elaborate-time") || revised.contains("非捕获")),
        "AD-18 Revised must name capturing ban vs elaborate-time allowance"
    );

    // Decision table D1–D3 cross-ref
    assert!(
        ad18.contains("closure-decision-table-2026-09-08")
            && (ad18.contains("D1") && ad18.contains("D3") || ad18.contains("D1–D3")),
        "AD-18 must link decision table D1–D3"
    );
}

#[test]
fn agents_md_clarifies_ad18_nfr35_capture_vs_elaborate_time() {
    let agents_path = workspace_root().join("AGENTS.md");
    let agents = fs::read_to_string(&agents_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", agents_path.display()));

    // Must not state a blanket "all closures banned" without the elaborate-time carve-out
    let blanket_ban = agents.contains("拒绝闭包")
        || agents.contains("禁止闭包")
        || ((agents.contains("AD-18") && agents.contains("禁止") && agents.contains("闭包"))
            && !agents.contains("非捕获")
            && !agents.contains("non-capturing")
            && !agents.contains("capturing"));
    assert!(
        !blanket_ban,
        "AGENTS.md must not restate a blanket AD-18 closure ban without NFR35 carve-out"
    );

    assert!(
        agents.contains("AD-18")
            && (agents.contains("NFR35")
                || agents.contains("capturing")
                || agents.contains("捕获"))
            && (agents.contains("elaborate-time")
                || agents.contains("non-capturing")
                || agents.contains("非捕获"))
            && (agents.contains("freeze")
                || agents.contains("dissolved")
                || agents.contains("消解")),
        "AGENTS.md Brand lock must clarify AD-18 / NFR35 capturing vs elaborate-time"
    );
}

#[test]
fn decision_table_d1_d3_still_align_with_ad18_revision() {
    let path = workspace_root().join(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/closure-decision-table-2026-09-08.md",
    );
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));

    assert!(
        text.contains("D1") && text.contains("D2") && text.contains("D3"),
        "decision table must retain D1–D3 for AD-18 cross-ref"
    );
    assert!(
        (text.contains("freeze") || text.contains("消解"))
            && (text.contains("生成器")
                || text.contains("elaborate-time")
                || text.contains("FR73")),
        "D3 dual-track dissolve-before-freeze must remain"
    );
    assert!(
        text.contains("AD-18") && (text.contains("26.3") || text.contains("2026-09-08")),
        "decision table must cite AD-18 revision / Story 26.3"
    );
}
