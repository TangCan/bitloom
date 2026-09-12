//! ATDD Story 97.2 / FR165 — Style Guide / linter deepen (≠ FR138 / FR130 alone).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_firrtl::{
    check_chisel_style_guide_fr130, check_chisel_style_guide_fr165, emit_chisel_style_guide_fr130,
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

#[test]
fn fr165_docs_contract() {
    let docs =
        fs::read_to_string(root().join("docs/fr165-deeper-chisel-parser-ecosystem.md")).unwrap();
    assert!(docs.contains("FR165") && docs.contains("Bitloom"));
    assert!(docs.contains("FR138") && docs.contains("FR130"));
    assert!(docs.contains("emit_chisel_style_guide_fr165"));
    assert!(docs.contains("chisel-style-lint-check"));
    assert!(
        docs.contains("HEAD") && (docs.contains("deferred") || docs.contains("NFR71")),
        "must defer Chisel HEAD Parser"
    );
    assert!(
        docs.contains("AD-27") && (docs.contains("not revised") || docs.contains("NFR70")),
        "must state AD-27 not revised"
    );
}

#[test]
fn fr165_emit_and_check() {
    let hir = hierarchy_fixture();
    let art = emit_chisel_style_guide_fr165(&hir).expect("FR165 emit");
    let scala = &art.files[0].contents;
    assert!(scala.contains("FR165 Style Guide / linter"));
    assert!(scala.contains("chisel-lint-rules") && scala.contains("scalafmt.conf"));
    assert!(scala.contains("import-hygiene-lint") && scala.contains("no-Chisel-HEAD-Parser"));
    assert!(scala.contains("--- FR165 style-lint ---"));
    assert!(scala.contains("FR130 Style Guide") && scala.contains("scalafmt-style"));
    check_chisel_style_guide_fr165(scala, &hir).expect("FR165 check");
    check_chisel_style_guide_fr130(scala, &hir).expect("FR165 is FR130 superset");
}

#[test]
fn fr165_fr130_alone_fails_linter_check() {
    let hir = hierarchy_fixture();
    let art = emit_chisel_style_guide_fr130(&hir).unwrap();
    check_chisel_style_guide_fr130(&art.files[0].contents, &hir).unwrap();
    let err = check_chisel_style_guide_fr165(&art.files[0].contents, &hir).expect_err("not FR165");
    let msg = format!("{err}");
    assert!(msg.contains("FR165"));
}

#[test]
fn fr165_just_and_script_exist() {
    let just = fs::read_to_string(root().join("Justfile")).unwrap();
    assert!(just.contains("chisel-style-lint-check"));
    assert!(just.contains("chisel-style-lint-check.sh"));
    let script = root().join("scripts/chisel-style-lint-check.sh");
    assert!(script.is_file(), "missing {}", script.display());
}

#[test]
fn fr165_force_missing_nonzero() {
    let out = Command::new("bash")
        .arg(root().join("scripts/chisel-style-lint-check.sh"))
        .env("BITLOOM_STYLE_LINT_FORCE_MISSING", "1")
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
fn fr165_nfr14_gates_implementation() {
    let risk = fs::read_to_string(
        root().join(
            "_agile-output/implementation-artifacts/nfr14-risk-epic97-deeper-chisel-parser-ecosystem-fr165.md",
        ),
    )
    .unwrap();
    assert!(risk.contains("Style") || risk.contains("linter"));
    assert!(risk.contains("FR138") && risk.contains("FR165"));
}
