//! ATDD / guardrail: AD-27 FIRRTL/FrozenHir → compilable Chisel (Story 20.2 / FR28).
//! Locks the architecture contract; does not implement the FR28 generator.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn ad27_section(spine: &str) -> &str {
    spine
        .split("### AD-27")
        .nth(1)
        .and_then(|rest| rest.split("### AD-28").next())
        .expect("AD-27 section present")
}

#[test]
fn ad27_requires_compilable_chisel_cites_fr28_fr46_overturns_nfr9() {
    let spine_path = workspace_root().join(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    let spine = fs::read_to_string(&spine_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", spine_path.display()));
    let ad27 = ad27_section(&spine);

    // Compilable Chisel Scala from FrozenHir / .fir
    assert!(
        ad27.contains("可编译")
            && ad27.contains("Chisel")
            && (ad27.contains("FrozenHir") || ad27.contains(".fir")),
        "AD-27 must require compilable Chisel Scala from FrozenHir/.fir"
    );

    // Acceptance: compile under pinned stack + port/hierarchy round-trip
    assert!(
        ad27.contains("编译通过")
            && (ad27.contains("端口") || ad27.contains("宽/向"))
            && (ad27.contains("层次") || ad27.contains("实例层次"))
            && (ad27.contains("往返") || ad27.contains("谓词")),
        "AD-27 must bind acceptance to compile + port/hierarchy predicates"
    );

    // Mechanical style OK; Open Q5 closed
    assert!(
        ad27.contains("机械")
            && (ad27.contains("Open Q5") || ad27.contains("Q5"))
            && (ad27.contains("已关闭") || ad27.contains("关闭")),
        "AD-27 must allow mechanical style and note Open Q5 closed"
    );

    // No Scala Parser.parse / firrtl.Parser restoration required
    assert!(
        (ad27.contains("Parser.parse") || ad27.contains("firrtl.Parser"))
            && (ad27.contains("不要求")
                || ad27.contains("不得要求")
                || ad27.contains("**不**要求")),
        "AD-27 must state Scala Parser.parse / firrtl.Parser is not required"
    );

    // Cite FR28 and FR46
    assert!(ad27.contains("FR28"), "AD-27 must cite FR28");
    assert!(ad27.contains("FR46"), "AD-27 must cite FR46");

    // Overturn historical NFR9
    assert!(
        ad27.contains("NFR9")
            && (ad27.contains("推翻")
                || ad27.contains("已被推翻")
                || ad27.contains("不再作为阻断")),
        "AD-27 must record overturn of historical NFR9"
    );

    // Revised cites FR28/FR46 and NFR9 / 推翻表
    let revised = ad27
        .lines()
        .find(|l| l.contains("**Revised:**") || l.contains("Revised:"))
        .expect("AD-27 Revised line present");
    assert!(
        revised.contains("FR28") && revised.contains("FR46"),
        "AD-27 Revised must cite FR28 and FR46"
    );
    assert!(
        revised.contains("NFR9") || revised.contains("推翻"),
        "AD-27 Revised must cite NFR9 overturn / 推翻表"
    );

    // Rule must not restate NFR9 ban as blocking current contract
    let rule = ad27
        .lines()
        .find(|l| l.contains("**Rule:**") || l.trim_start().starts_with("- **Rule:**"))
        .expect("AD-27 Rule line present");
    let old_ban_phrases = [
        "不承诺可维护 Chisel",
        "不以可维护 Chisel Scala 为互转契约",
        "FR28 仅尽力",
        "结构化尽力失败即完成",
    ];
    for phrase in old_ban_phrases {
        assert!(
            !rule.contains(phrase) || rule.contains("推翻") || rule.contains("不再作为阻断"),
            "AD-27 Rule must not leave historical NFR9/尽力 ban as the binding contract: {phrase}"
        );
    }
    // Stronger: Rule must positively overturn NFR9
    assert!(
        rule.contains("NFR9")
            && (rule.contains("推翻")
                || rule.contains("已被推翻")
                || rule.contains("不再作为阻断")),
        "AD-27 Rule must positively overturn NFR9"
    );

    // AGENTS.md Brand lock pointer; NFR9 only as overturned
    let agents_path = workspace_root().join("AGENTS.md");
    let agents = fs::read_to_string(&agents_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", agents_path.display()));
    assert!(
        agents.contains("AD-27")
            && agents.contains("FR28")
            && (agents.contains("compilable")
                || agents.contains("可编译")
                || agents.contains("Chisel")),
        "AGENTS.md Brand lock must positively point at AD-27 / FR28 compilable Chisel"
    );
    if agents.contains("NFR9") {
        assert!(
            agents.contains("overturn") || agents.contains("推翻"),
            "AGENTS.md may mention NFR9 only as overturned"
        );
    }
    // Story 20.1 risk record must still exist (Given)
    let risk_path = workspace_root()
        .join("_agile-output/implementation-artifacts/nfr14-risk-chisel-bidirectional.md");
    assert!(
        risk_path.is_file(),
        "Story 20.1 NFR14 risk record must exist before AD-27 work: {}",
        risk_path.display()
    );
}
