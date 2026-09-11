//! ATDD (red→green): Story 63.2 / FR122 — official-style Chisel full pack (O1–O4).
//!
//! Beyond FR111 D1+D3: package + FR122 claim, ordered sections, per-module FR122 markers.
//!
//! ```text
//! cargo test -p bitloom --test fr122_official_style_chisel
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_firrtl::{
    check_idiomatic_chisel, check_idiomatic_chisel_fr111, check_idiomatic_chisel_fr122,
    emit_chisel, emit_chisel_idiomatic, emit_chisel_idiomatic_fr111, emit_chisel_idiomatic_fr122,
};

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(rel: &str) -> String {
    fs::read_to_string(workspace_root().join(rel)).unwrap_or_else(|e| panic!("read {rel}: {e}"))
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
fn fr122_docs_contract() {
    let text = read("docs/fr122-official-style-chisel.md");
    assert!(text.contains("FR122") && text.contains("Bitloom"));
    assert!(
        text.contains("O1") && text.contains("O2") && text.contains("O3") && text.contains("O4")
    );
    assert!(text.contains("package bitloom.generated") || text.contains("bitloom.generated"));
    assert!(
        (text.contains("FR97") || text.contains("FR111") || text.contains("机械"))
            && (text.contains("≠") || text.contains("alone") || text.contains("不得")),
        "must contrast FR97/FR111/mechanical alone ≠ FR122"
    );
    assert!(text.contains("Parser") && (text.contains("not") || text.contains("不得")));
}

#[test]
fn fr122_official_emit_passes_on_hierarchy() {
    let hir = hierarchy_fixture();
    assert!(hir.circuit().modules.len() >= 2);
    let art = emit_chisel_idiomatic_fr122(&hir).expect("FR122 emit");
    let scala = &art.files[0].contents;
    check_idiomatic_chisel_fr122(scala, &hir).expect("FR122 check");
    assert!(scala.contains("FR122 official-style") || scala.contains("FR122"));
    assert!(scala.contains("package bitloom.generated"));
    assert!(scala.matches("--- FR122 official ---").count() >= 2);
    assert!(scala.contains("class Child") && scala.contains("class Top"));
    // Superset: FR111 still green
    check_idiomatic_chisel_fr111(scala, &hir).expect("FR122 is FR111 subset");
    check_idiomatic_chisel(scala, &hir).expect("FR122 is FR97 subset");
}

#[test]
fn fr122_rejects_fr111_alone_not_silent() {
    let hir = hierarchy_fixture();
    let scala = emit_chisel_idiomatic_fr111(&hir).expect("FR111 emit").files[0]
        .contents
        .clone();
    check_idiomatic_chisel_fr111(&scala, &hir).expect("FR111 still green");
    let err = check_idiomatic_chisel_fr122(&scala, &hir).expect_err("FR111 alone ≠ FR122");
    let msg = err.to_string();
    assert!(
        msg.contains("FR122")
            && (msg.contains("official") || msg.contains("FR111") || msg.contains("缺少")),
        "readable FR122 fail: {msg}"
    );
}

#[test]
fn fr122_rejects_fr97_mvp_alone() {
    let hir = hierarchy_fixture();
    let scala = emit_chisel_idiomatic(&hir).expect("FR97 emit").files[0]
        .contents
        .clone();
    check_idiomatic_chisel(&scala, &hir).expect("FR97 still green");
    let err = check_idiomatic_chisel_fr122(&scala, &hir).expect_err("FR97 alone ≠ FR122");
    assert!(err.to_string().contains("FR111") || err.to_string().contains("FR122"));
}

#[test]
fn fr122_rejects_mechanical_emit() {
    let hir = hierarchy_fixture();
    let scala = emit_chisel(&hir).expect("mechanical").files[0]
        .contents
        .clone();
    let err = check_idiomatic_chisel_fr122(&scala, &hir).expect_err("mechanical ≠ FR122");
    assert!(
        err.to_string().contains("FR97")
            || err.to_string().contains("FR111")
            || err.to_string().contains("FR122")
            || err.to_string().contains("idiomatic")
    );
}

#[test]
fn fr122_o3_fails_when_per_module_marker_stripped() {
    let hir = hierarchy_fixture();
    let mut scala = emit_chisel_idiomatic_fr122(&hir).expect("emit").files[0]
        .contents
        .clone();
    scala = scala.replacen("  // --- FR122 official ---\n", "", 1);
    let err = check_idiomatic_chisel_fr122(&scala, &hir).expect_err("missing Child FR122 marker");
    let msg = err.to_string();
    assert!(
        msg.contains("FR122")
            && (msg.contains("Child") || msg.contains("official") || msg.contains("O3")),
        "{msg}"
    );
}

#[test]
fn fr122_o1_fails_when_package_stripped() {
    let hir = hierarchy_fixture();
    let mut scala = emit_chisel_idiomatic_fr122(&hir).expect("emit").files[0]
        .contents
        .clone();
    scala = scala.replace("package bitloom.generated\n", "");
    let err = check_idiomatic_chisel_fr122(&scala, &hir).expect_err("missing package");
    let msg = err.to_string();
    assert!(
        msg.contains("FR122") && (msg.contains("package") || msg.contains("O1")),
        "{msg}"
    );
}

#[test]
fn fr122_o2_fails_when_section_order_swapped() {
    let hir = hierarchy_fixture();
    let mut scala = emit_chisel_idiomatic_fr122(&hir).expect("emit").files[0]
        .contents
        .clone();
    // Swap first registers/logic markers so logic appears before registers (O2 fail).
    assert!(
        scala.contains("// --- registers ---") && scala.contains("// --- logic ---"),
        "fixture must emit both sections for O2 negative"
    );
    scala = scala.replacen("// --- registers ---", "// --- __SWAP_REG__ ---", 1);
    scala = scala.replacen("// --- logic ---", "// --- registers ---", 1);
    scala = scala.replacen("// --- __SWAP_REG__ ---", "// --- logic ---", 1);
    let err = check_idiomatic_chisel_fr122(&scala, &hir).expect_err("swapped section order");
    let msg = err.to_string();
    assert!(
        msg.contains("FR122")
            && (msg.contains("O2") || msg.contains("order") || msg.contains("顺序")),
        "{msg}"
    );
}

#[test]
fn fr122_nfr48_fr97_fr111_docs_still_present() {
    let fr97 = read("docs/fr97-idiomatic-chisel.md");
    let fr111 = read("docs/fr111-idiomatic-chisel-depth.md");
    assert!(fr97.contains("FR97") && fr97.contains("idiomatic"));
    assert!(fr111.contains("FR111") && (fr111.contains("D1") || fr111.contains("deepen")));
}

#[test]
fn fr122_ad27_revised_for_fr122() {
    let spine = read(
        "_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md",
    );
    assert!(spine.contains("FR122"));
    assert!(
        spine.contains("AD-27")
            && (spine.contains("Revised") || spine.contains("修订"))
            && spine.contains("2026-09-10"),
        "AD-27 must carry FR122 revise stamp"
    );
    assert!(
        spine.contains("Parser")
            && (spine.contains("不") || spine.contains("禁止") || spine.contains("not")),
        "must keep Parser forbid"
    );
}
