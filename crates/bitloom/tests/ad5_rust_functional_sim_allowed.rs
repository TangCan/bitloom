//! ATDD / guardrail: AD-5 allows generated Rust functional sim (Story 19.2 / FR47).
//! Locks the architecture contract; does not implement the FR47 generator.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn ad5_section(spine: &str) -> &str {
    spine
        .split("### AD-5")
        .nth(1)
        .and_then(|rest| rest.split("### AD-6").next())
        .expect("AD-5 section present")
}

#[test]
fn ad5_allows_generated_rust_functional_sim_and_cites_fr47() {
    let spine_path = workspace_root().join(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    let spine = fs::read_to_string(&spine_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", spine_path.display()));
    let ad5 = ad5_section(&spine);

    // Allow generated Rust functional-sim crate
    assert!(
        ad5.contains("生成的 Rust 功能模拟器") || ad5.contains("生成 Rust 功能模拟器"),
        "AD-5 must allow toolchain-generated Rust functional-sim crates"
    );

    // Cite FR47 in Rule and/or Revised
    assert!(ad5.contains("FR47"), "AD-5 must cite FR47");

    // Explicitly not SystemC TLM-2.0 contract
    assert!(
        ad5.contains("SystemC TLM-2.0")
            && (ad5.contains("不承诺")
                || ad5.contains("不要求")
                || ad5.contains("不强制")
                || ad5.contains("形态不强制")),
        "AD-5 must state SystemC TLM-2.0 is not contracted / not required"
    );

    // Cycle-accurate only from FrozenHir tick
    assert!(
        ad5.contains("FrozenHir")
            && ad5.contains("tick")
            && (ad5.contains("周期精确") || ad5.contains("周期精确仿真")),
        "AD-5 must bind cycle-accurate sim to FrozenHir tick"
    );

    // Revised must cite PRD FR47 and 推翻表
    let revised = ad5
        .lines()
        .find(|l| l.contains("**Revised:**") || l.contains("Revised:"))
        .expect("AD-5 Revised line present");
    assert!(revised.contains("FR47"), "AD-5 Revised must cite PRD FR47");
    assert!(
        revised.contains("推翻表"),
        "AD-5 Revised must cite the PRD 推翻表"
    );

    // Current Rule must not restate the old ban as a blocking Rule
    // (do not false-positive on 「不得进入 HIR」 or 「不要求从 HIR 降低 SystemC TLM-2.0」)
    let rule = ad5
        .lines()
        .find(|l| l.contains("**Rule:**") || l.trim_start().starts_with("- **Rule:**"))
        .expect("AD-5 Rule line present");
    let old_ban_phrases = [
        "禁止从 HIR 生成 TLM",
        "禁止从 HIR 降低 TLM",
        "禁止一切 HIR→功能模拟器生成",
        "禁止 HIR→功能模拟器生成",
        "禁止 HIR→TLM",
        "无 HIR→TLM",
    ];
    for phrase in old_ban_phrases {
        assert!(
            !rule.contains(phrase),
            "AD-5 Rule must not restate old ban phrase that blocks FR47: {phrase}"
        );
    }

    // AGENTS.md: no old ban; positive Brand-lock pointer
    let agents_path = workspace_root().join("AGENTS.md");
    let agents = fs::read_to_string(&agents_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", agents_path.display()));
    let agents_has_old_ban = (agents.contains("禁止") || agents.contains("不得"))
        && (agents.contains("HIR→功能模拟")
            || agents.contains("禁止 HIR→功能模拟器生成")
            || agents.contains("禁止从 HIR 生成 TLM"));
    assert!(
        !agents_has_old_ban,
        "AGENTS.md must not restate the old HIR→functional-sim / TLM generation ban"
    );
    assert!(
        agents.contains("AD-5")
            && agents.contains("FR47")
            && (agents.contains("Rust") || agents.contains("功能模拟")),
        "AGENTS.md Brand lock must positively point at AD-5 / FR47 generated Rust functional sim"
    );
    assert!(
        (agents.contains("SystemC TLM-2.0") || agents.contains("TLM-2.0"))
            && (agents.contains("not contracted")
                || agents.contains("不承诺")
                || agents.contains("不要求")
                || agents.contains("不强制")),
        "AGENTS.md must note SystemC TLM-2.0 is not contracted (not merely mention TLM)"
    );
}
