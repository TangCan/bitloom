//! ATDD Story 114.2 / FR181 — deeper Style Guide / linter beyond FR176.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_firrtl::{
    check_chisel_ecosystem_fr176, check_chisel_style_linter_fr181, emit_chisel_ecosystem_fr176,
    emit_chisel_style_linter_fr181,
};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn hierarchy_fixture() -> bitloom_hir::FrozenHir {
    let mut s = ElaborateSession::new("t");
    s.begin_module("Child", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("y", "x", Span::default());
    s.end_process();
    s.end_module();

    s.begin_module("Top", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("x", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.add_instance(
        "u0",
        "Child",
        vec![
            ("clk".into(), "clk".into()),
            ("rst".into(), "rst".into()),
            ("x".into(), "x".into()),
            ("y".into(), "y".into()),
        ],
        vec![],
        Span::default(),
    );
    s.end_module();
    s.finish().unwrap()
}

fn read(rel: &str) -> String {
    fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
}

#[test]
fn fr181_docs_contract() {
    let docs = read("docs/fr181-deeper-style-guide-linter.md");
    assert!(docs.contains("FR181") && docs.contains("Bitloom"));
    assert!(docs.contains("FR176"));
    assert!(docs.contains("emit_chisel_style_linter_fr181"));
    assert!(docs.contains("chisel-style-linter-deepen-check"));
    assert!(
        docs.contains("AD-27") && (docs.contains("revised") || docs.contains("NFR85")),
        "must state AD-27 revised for FR181"
    );
    assert!(
        (docs.contains("alone") || docs.contains("≠")) && docs.contains("FR176"),
        "must forbid FR176 alone"
    );
    assert!(
        docs.contains("NFR86") || docs.contains("全家桶"),
        "must leave fuller suite as NFR86"
    );
}

#[test]
fn fr181_spine_ad27_revised() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(
        spine.contains("FR181")
            && spine.contains("chisel-wartremover-rules")
            && spine.contains("2026-09-14"),
        "AD-27 must document FR181 deepen"
    );
    assert!(
        spine.contains("NFR85") || spine.contains("FR181"),
        "must cite NFR85 / FR181 revise"
    );
}

#[test]
fn fr181_emit_and_check() {
    let hir = hierarchy_fixture();
    let art = emit_chisel_style_linter_fr181(&hir).expect("FR181 emit");
    let scala = &art.files[0].contents;
    assert!(scala.contains("FR181 Style Guide / linter"));
    assert!(scala.contains("chisel-wartremover-rules") && scala.contains("fatal-warnings-lint"));
    assert!(scala.contains("--- FR181 style-linter ---"));
    assert!(scala.contains("FR176 Chisel ecosystem"));
    check_chisel_style_linter_fr181(scala, &hir).expect("FR181 check");
    check_chisel_ecosystem_fr176(scala, &hir).expect("FR181 is FR176 superset");
}

#[test]
fn fr181_fr176_alone_fails_style_linter_check() {
    let hir = hierarchy_fixture();
    let art = emit_chisel_ecosystem_fr176(&hir).unwrap();
    check_chisel_ecosystem_fr176(&art.files[0].contents, &hir).unwrap();
    let err = check_chisel_style_linter_fr181(&art.files[0].contents, &hir).expect_err("not FR181");
    let msg = format!("{err}");
    assert!(msg.contains("FR181"));
}

#[test]
fn fr181_script_just_ci() {
    let just = read("Justfile");
    assert!(just.contains("chisel-style-linter-deepen-check"));
    assert!(just.contains("chisel-style-linter-deepen-check.sh"));
    let script = root().join("scripts/chisel-style-linter-deepen-check.sh");
    assert!(script.is_file());
    let script_txt = read("scripts/chisel-style-linter-deepen-check.sh");
    assert!(script_txt.contains("BITLOOM_STYLE_LINTER_DEEPEN_FORCE_MISSING"));
    assert!(script_txt.contains("chisel-ecosystem-deepen-check"));
    let ci = read(".github/workflows/ci.yml");
    assert!(
        ci.contains("chisel-style-linter-deepen:"),
        "must define required chisel-style-linter-deepen job"
    );
    let idx = ci.find("chisel-style-linter-deepen:").expect("job");
    let block = &ci[idx..idx.saturating_add(600).min(ci.len())];
    assert!(
        block.contains("chisel-style-linter-deepen-check"),
        "CI must run chisel-style-linter-deepen-check"
    );
    assert!(!block.contains("continue-on-error"));
}

#[test]
fn fr181_force_missing_nonzero() {
    let out = Command::new("bash")
        .arg(root().join("scripts/chisel-style-linter-deepen-check.sh"))
        .current_dir(root())
        .env("BITLOOM_STYLE_LINTER_DEEPEN_FORCE_MISSING", "1")
        .output()
        .expect("run force-missing");
    assert!(
        !out.status.success(),
        "FORCE_MISSING must be non-zero; stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn fr181_live_gate() {
    let out = Command::new("bash")
        .arg(root().join("scripts/chisel-style-linter-deepen-check.sh"))
        .current_dir(root())
        .env_remove("BITLOOM_STYLE_LINTER_DEEPEN_FORCE_MISSING")
        .output()
        .expect("run live gate");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        out.status.success(),
        "live FR181 gate must pass; stdout={stdout} stderr={stderr}"
    );
}
