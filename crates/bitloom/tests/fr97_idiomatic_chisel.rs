//! ATDD Story 42.2 / FR97 — idiomatic / maintainable Chisel emit + acceptance.
//!
//! Mechanical `emit_chisel` (FR28/FR46) must **not** alone satisfy FR97.
//! Positive path: `emit_chisel_idiomatic` + `check_idiomatic_chisel`.
//! Negative: mechanical output fails idiomatic check readably.

use std::fs;
use std::path::PathBuf;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use rhdl_firrtl::{
    CHISEL_TARGET, FIRTOOL_TARGET, check_idiomatic_chisel, emit_chisel, emit_chisel_idiomatic,
};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
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
fn fr97_idiomatic_emit_passes_check_on_hierarchy() {
    let hir = hierarchy_fixture();
    let art = emit_chisel_idiomatic(&hir).expect("idiomatic emit must succeed");
    let scala = &art.files[0].contents;

    check_idiomatic_chisel(scala, &hir).expect("idiomatic check must accept FR97 emit");

    // Naming: public HIR names preserved
    assert!(scala.contains("class Child extends Module"), "{scala}");
    assert!(scala.contains("class Top extends Module"), "{scala}");
    assert!(
        scala.contains("val x = Input(") || scala.contains("val x ="),
        "{scala}"
    );
    assert!(scala.contains("val u0 = Module(new Child)"), "{scala}");

    // Structure
    assert!(scala.contains("IO(new Bundle"), "{scala}");
    assert!(scala.contains("Module(new Child)"), "{scala}");

    // Pin + brand
    assert!(scala.contains(CHISEL_TARGET), "{scala}");
    assert!(scala.contains(FIRTOOL_TARGET), "{scala}");
    assert!(
        scala.to_lowercase().contains("bitloom"),
        "idiomatic emit must cite Bitloom: {scala}"
    );
}

#[test]
fn fr97_idiomatic_header_and_sections() {
    let hir = hierarchy_fixture();
    let scala = emit_chisel_idiomatic(&hir).expect("idiomatic emit").files[0]
        .contents
        .clone();

    assert!(
        scala.contains("FR97") && (scala.contains("idiomatic") || scala.contains("Idiomatic")),
        "header must claim FR97 idiomatic: {scala}"
    );
    assert!(
        scala.contains("AD-27") || scala.contains("AD27"),
        "header should cite revised AD-27: {scala}"
    );
    // Readability: section markers for HIR constructs (not merely always-on IO)
    assert!(
        scala.to_lowercase().contains("--- io ---"),
        "must include IO section: {scala}"
    );
    assert!(
        scala.to_lowercase().contains("--- registers ---"),
        "hierarchy fixture has RegDecl; must include registers section: {scala}"
    );
    assert!(
        scala.to_lowercase().contains("--- instances ---"),
        "hierarchy fixture has Instance; must include instances section: {scala}"
    );
    assert!(
        scala.to_lowercase().contains("--- logic ---"),
        "hierarchy fixture has Process; must include logic section: {scala}"
    );
    assert!(
        scala.lines().count() > 8,
        "idiomatic output must not be a single-line dump"
    );
}

#[test]
fn fr97_mutilated_idiomatic_fails_section_check() {
    let hir = hierarchy_fixture();
    let mut scala = emit_chisel_idiomatic(&hir).expect("idiomatic emit").files[0]
        .contents
        .clone();
    // Strip body section markers while keeping IO + FR97 header — must fail E0904
    for marker in [
        "// --- registers ---",
        "// --- instances ---",
        "// --- logic ---",
    ] {
        scala = scala.replace(marker, "");
    }
    let err = check_idiomatic_chisel(&scala, &hir)
        .expect_err("stripped body sections must fail idiomatic check");
    assert_eq!(err.code, "rhdl::E0904");
    let msg = format!("{err}");
    assert!(
        msg.contains("section") || msg.contains("分节") || msg.contains("registers"),
        "failure must mention missing sections: {msg}"
    );
}

#[test]
fn fr97_mechanical_emit_fails_idiomatic_check() {
    let hir = hierarchy_fixture();
    let mechanical = emit_chisel(&hir).expect("mechanical emit").files[0]
        .contents
        .clone();

    // Explicit downgrade path: FR28 mechanical must not silently pass FR97 check
    let err = check_idiomatic_chisel(&mechanical, &hir)
        .expect_err("mechanical FR28 output must fail idiomatic check");
    let msg = format!("{err}");
    assert!(
        msg.contains("FR97")
            || msg.to_lowercase().contains("idiomatic")
            || msg.to_lowercase().contains("mechanical")
            || err.code.contains("E0904"),
        "failure must be readable about idiomatic/FR97/mechanical: {msg}"
    );
    assert!(
        !mechanical.contains("FR97"),
        "mechanical emit must not mention FR97 (downgrade stays on emit_chisel_idiomatic): {mechanical}"
    );
}

#[test]
fn fr97_docs_distinguish_mechanical_vs_idiomatic() {
    let root = workspace_root();
    let fr97 = fs::read_to_string(root.join("docs/fr97-idiomatic-chisel.md"))
        .expect("docs/fr97-idiomatic-chisel.md must exist");
    assert!(
        fr97.contains("FR97") && (fr97.contains("idiomatic") || fr97.contains("可维护")),
        "fr97 doc must describe idiomatic/maintainable face"
    );
    assert!(
        fr97.contains("FR28") || fr97.contains("机械") || fr97.contains("mechanical"),
        "fr97 doc must contrast mechanical FR28 path"
    );
    assert!(
        fr97.contains("AD-27") || fr97.contains("AD27"),
        "fr97 doc must cite revised AD-27"
    );
    assert!(
        fr97.to_lowercase().contains("bitloom"),
        "fr97 doc must keep Bitloom brand"
    );
    assert!(
        !fr97.contains("Parser.parse")
            || fr97.contains("不要求")
            || fr97.contains("不得")
            || fr97.contains("废弃"),
        "fr97 must not require restoring Parser.parse without alternative"
    );

    let fr28 = fs::read_to_string(root.join("docs/fr28-chisel-compilable.md")).expect("fr28 doc");
    assert!(
        fr28.contains("可编译 ≠ idiomatic")
            || fr28.contains("≠ idiomatic")
            || fr28.contains("compilable ≠ idiomatic"),
        "fr28 must retain mechanical ≠ idiomatic honesty"
    );
    assert!(
        fr28.contains("fr97") || fr28.contains("FR97"),
        "fr28 must cross-link FR97 idiomatic face"
    );
}
