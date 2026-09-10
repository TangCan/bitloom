//! ATDD Story 70.2 / FR130 — Style Guide pack (no Parser).

use std::fs;
use std::path::PathBuf;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use rhdl_firrtl::{
    check_chisel_style_guide_fr130, check_idiomatic_chisel_fr122, emit_chisel_idiomatic_fr122,
    emit_chisel_style_guide_fr130,
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
fn fr130_ad27_revised() {
    let spine = fs::read_to_string(
        root().join(
            "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
        ),
    )
    .unwrap();
    assert!(spine.contains("FR130") && spine.contains("AD-27"));
    assert!(
        spine.contains("Phase 15 / **FR130**")
            || (spine.contains("FR130") && spine.contains("NFR54") && spine.contains("2026-09-10"))
    );
    assert!(spine.contains("不恢复") || spine.contains("not restored") || spine.contains("仍禁止"));
}

#[test]
fn fr130_docs_contract() {
    let docs = fs::read_to_string(root().join("docs/fr130-style-guide.md")).unwrap();
    assert!(docs.contains("FR130") && docs.contains("Bitloom"));
    assert!(docs.contains("S1") && docs.contains("S4"));
    assert!(docs.contains("FR122") && docs.contains("Parser"));
    assert!(docs.contains("emit_chisel_style_guide_fr130"));
}

#[test]
fn fr130_emit_and_check() {
    let hir = hierarchy_fixture();
    let art = emit_chisel_style_guide_fr130(&hir).expect("FR130 emit");
    let scala = &art.files[0].contents;
    assert!(scala.contains("FR130 Style Guide"));
    assert!(scala.contains("scalafmt-style") && scala.contains("withClockAndReset"));
    assert!(scala.contains("--- FR130 style-guide ---"));
    assert!(scala.contains("Parser.parse not required"));
    check_chisel_style_guide_fr130(scala, &hir).expect("FR130 check");
}

#[test]
fn fr130_fr122_alone_fails_style_guide_check() {
    let hir = hierarchy_fixture();
    let art = emit_chisel_idiomatic_fr122(&hir).unwrap();
    check_idiomatic_chisel_fr122(&art.files[0].contents, &hir).unwrap();
    let err = check_chisel_style_guide_fr130(&art.files[0].contents, &hir).expect_err("not FR130");
    let msg = format!("{err}");
    assert!(msg.contains("FR130"));
}
