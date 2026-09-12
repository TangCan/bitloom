//! ATDD Story 109.2 / FR176 — Combined Parser/Chisel ecosystem (≠ FR165 / FR170 alone).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_firrtl::{
    check_chisel_ecosystem_fr176, check_chisel_style_guide_fr165, emit_chisel_ecosystem_fr176,
    emit_chisel_style_guide_fr165,
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
    s.declare_reg("r", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("y", "x", Span::default());
    s.end_process();
    s.begin_sequential(Span::default());
    s.assign_reg_d_inc("r", Span::default());
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
fn fr176_docs_contract() {
    let docs = read("docs/fr176-deeper-parser-chisel-ecosystem.md");
    assert!(docs.contains("FR176") && docs.contains("Bitloom"));
    assert!(docs.contains("FR165") && docs.contains("FR170"));
    assert!(docs.contains("emit_chisel_ecosystem_fr176"));
    assert!(docs.contains("chisel-ecosystem-deepen-check"));
    assert!(
        docs.contains("AD-27") && (docs.contains("revised") || docs.contains("NFR80")),
        "must state AD-27 revised for FR176"
    );
    assert!(
        (docs.contains("alone") || docs.contains("≠"))
            && docs.contains("FR170")
            && docs.contains("FR165"),
        "must forbid FR165/FR170 alone"
    );
}

#[test]
fn fr176_emit_and_check() {
    let hir = hierarchy_fixture();
    let art = emit_chisel_ecosystem_fr176(&hir).expect("FR176 emit");
    let scala = &art.files[0].contents;
    assert!(scala.contains("FR176 Chisel ecosystem"));
    assert!(scala.contains("chisel-ecosystem-pack") && scala.contains("parser-mainline-bridge"));
    assert!(scala.contains("chisel-official-style-pack"));
    assert!(scala.contains("--- FR176 ecosystem ---"));
    assert!(scala.contains("FR165 Style Guide / linter"));
    check_chisel_ecosystem_fr176(scala, &hir).expect("FR176 check");
    check_chisel_style_guide_fr165(scala, &hir).expect("FR176 is FR165 superset");
}

#[test]
fn fr176_fr165_alone_fails_ecosystem_check() {
    let hir = hierarchy_fixture();
    let art = emit_chisel_style_guide_fr165(&hir).unwrap();
    check_chisel_style_guide_fr165(&art.files[0].contents, &hir).unwrap();
    let err = check_chisel_ecosystem_fr176(&art.files[0].contents, &hir).expect_err("not FR176");
    let msg = format!("{err}");
    assert!(msg.contains("FR176"));
}

#[test]
fn fr176_just_script_and_ci() {
    let just = read("Justfile");
    assert!(just.contains("chisel-ecosystem-deepen-check"));
    assert!(just.contains("chisel-ecosystem-deepen-check.sh"));
    let script = root().join("scripts/chisel-ecosystem-deepen-check.sh");
    assert!(script.is_file(), "missing {}", script.display());
    let script_txt = read("scripts/chisel-ecosystem-deepen-check.sh");
    assert!(script_txt.contains("chisel-style-lint-check.sh"));
    assert!(script_txt.contains("parser-head-migration-check.sh"));
    assert!(script_txt.contains("fr176_ecosystem_gate"));
    assert!(script_txt.contains("BITLOOM_ECOSYSTEM_FORCE_MISSING"));
    let ci = read(".github/workflows/ci.yml");
    assert!(
        ci.contains("chisel-ecosystem-deepen:"),
        "must define required chisel-ecosystem-deepen job"
    );
    let idx = ci.find("chisel-ecosystem-deepen:").expect("job");
    let block = &ci[idx..idx.saturating_add(800).min(ci.len())];
    assert!(
        block.contains("chisel-ecosystem-deepen-check"),
        "CI must run chisel-ecosystem-deepen-check"
    );
}

#[test]
fn fr176_force_missing_nonzero() {
    let out = Command::new("bash")
        .arg(root().join("scripts/chisel-ecosystem-deepen-check.sh"))
        .env("BITLOOM_ECOSYSTEM_FORCE_MISSING", "1")
        .current_dir(root())
        .output()
        .expect("run force-missing");
    assert!(!out.status.success(), "force-missing must be non-zero");
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("FORCE_MISSING") || err.contains("unavailable") || err.contains("refusing"),
        "stderr must be readable: {err}"
    );
}

#[test]
fn fr176_nfr14_and_ad27_revised() {
    let risk = read(
        "_agile-output/implementation-artifacts/nfr14-risk-epic109-deeper-parser-chisel-ecosystem-fr176.md",
    );
    assert!(risk.contains("FR176") && (risk.contains("组合") || risk.contains("ecosystem")));
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    let ad27 = spine
        .split("### AD-27")
        .nth(1)
        .expect("AD-27")
        .split("### AD-28")
        .next()
        .expect("AD-28");
    assert!(
        ad27.contains("FR176") && ad27.contains("2026-09-12"),
        "AD-27 must be revised for FR176"
    );
}
