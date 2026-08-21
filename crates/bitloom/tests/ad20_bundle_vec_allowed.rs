//! ATDD / guardrail: AD-20 allows synthesizable Bundle/Vec (Story 19.3 / FR51).
//! Locks the architecture contract; does not implement the FR51 language surface.

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn ad20_section(spine: &str) -> &str {
    spine
        .split("### AD-20")
        .nth(1)
        .and_then(|rest| rest.split("### AD-21").next())
        .expect("AD-20 section present")
}

#[test]
fn ad20_allows_bundle_vec_cites_fr51_and_fr22_boundary() {
    let spine_path = workspace_root().join(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    let spine = fs::read_to_string(&spine_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", spine_path.display()));
    let ad20 = ad20_section(&spine);

    // Allow documented Bundle / Vec on synthesizable path
    assert!(
        ad20.contains("Bundle")
            && (ad20.contains("Vec<T,N>") || ad20.contains("`Vec`"))
            && (ad20.contains("允许进入可综合") || ad20.contains("允许进入可综合路径")),
        "AD-20 must allow Bundle/Vec on the synthesizable path"
    );

    // Width/dir fail before emit; no silent use without checks
    assert!(
        ad20.contains("位宽")
            && (ad20.contains("方向") || ad20.contains("宽/向"))
            && ad20.contains("emit")
            && (ad20.contains("失败") || ad20.contains("必须在 emit 前失败")),
        "AD-20 must require width/dir mismatches to fail before emit"
    );
    assert!(
        ad20.contains("silently")
            || ad20.contains("不得 silently")
            || (ad20.contains("无检查") && ad20.contains("不得")),
        "AD-20 must forbid silent Bundle/Vec use without width/dir checks"
    );

    // Cite FR51
    assert!(ad20.contains("FR51"), "AD-20 must cite FR51");

    // FR22 boundary: composites via FR51, not FR22 construct bar
    assert!(
        ad20.contains("FR22")
            && (ad20.contains("本 FR 非目标")
                || ad20.contains("不算进 FR22")
                || (ad20.contains("非目标") && ad20.contains("FR51"))),
        "AD-20 must clarify FR22 boundary (composites via FR51)"
    );

    // Revised must cite PRD FR51 and FR22 boundary
    let revised = ad20
        .lines()
        .find(|l| l.contains("**Revised:**") || l.contains("Revised:"))
        .expect("AD-20 Revised line present");
    assert!(
        revised.contains("FR51") && (revised.contains("PRD") || revised.contains("**PRD")),
        "AD-20 Revised must cite PRD FR51"
    );
    assert!(
        revised.contains("FR22")
            && (revised.contains("本 FR 非目标") || revised.contains("非目标")),
        "AD-20 Revised must cite FR22「本 FR 非目标」boundary"
    );

    // Current Rule must positively allow; must not restate bare old ban as blocking
    let rule = ad20
        .lines()
        .find(|l| l.contains("**Rule:**") || l.trim_start().starts_with("- **Rule:**"))
        .expect("AD-20 Rule line present");
    assert!(
        rule.contains("允许进入可综合路径") || rule.contains("允许进入可综合"),
        "AD-20 Rule must positively allow synthesizable Bundle/Vec"
    );
    assert!(
        rule.contains("不再作为阻断"),
        "AD-20 Rule must state historical Bundle/Vec ban no longer blocks FR51"
    );
    let old_ban_phrases = [
        "Bundle/Vec 禁止进入",
        "禁止 Bundle/Vec 进入可综合",
        "Bundle 与 Vec 不得进入可综合路径",
        "`Bundle`/`Vec` 仍 Deferred",
        "Bundle/Vec 仍 Deferred",
    ];
    for phrase in old_ban_phrases {
        assert!(
            !rule.contains(phrase),
            "AD-20 Rule must not restate old ban phrase that blocks FR51: {phrase}"
        );
    }

    // AGENTS.md: no old ban; positive Brand-lock pointer
    let agents_path = workspace_root().join("AGENTS.md");
    let agents = fs::read_to_string(&agents_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", agents_path.display()));
    let agents_has_old_ban = (agents.contains("禁止") || agents.contains("不得"))
        && (agents.contains("禁止 Bundle")
            || agents.contains("Bundle/Vec 禁止")
            || agents.contains("不得进入可综合") && agents.contains("Bundle"));
    assert!(
        !agents_has_old_ban,
        "AGENTS.md must not restate the old Bundle/Vec synthesizable ban"
    );
    assert!(
        agents.contains("AD-20") && agents.contains("FR51") && agents.contains("Bundle"),
        "AGENTS.md Brand lock must positively point at AD-20 / FR51 Bundle/Vec"
    );

    // language-surface: FR22 boundary present
    let surface_path = workspace_root().join("_agile-output/specs/spec-rhdl/language-surface.md");
    let surface = fs::read_to_string(&surface_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", surface_path.display()));
    assert!(
        surface.contains("FR51")
            && surface.contains("Bundle")
            && (surface.contains("FR22")
                && (surface.contains("不含")
                    || surface.contains("非目标")
                    || surface.contains("边界"))),
        "language-surface.md must document Bundle/Vec under FR51 with FR22 boundary"
    );
}
