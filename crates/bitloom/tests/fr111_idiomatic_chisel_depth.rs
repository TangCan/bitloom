//! ATDD (red→green): Story 53.2 / FR111 — idiomatic Chisel deepen (D1+D3).
//!
//! Beyond FR97 MVP: multi-module per-module markers + FR111 header.
//!
//! ```text
//! cargo test -p bitloom --test fr111_idiomatic_chisel_depth
//! ```

use std::fs;
use std::path::PathBuf;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use rhdl_firrtl::{
    check_idiomatic_chisel, check_idiomatic_chisel_fr111, emit_chisel, emit_chisel_idiomatic,
    emit_chisel_idiomatic_fr111,
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
fn fr111_docs_contract() {
    let text = read("docs/fr111-idiomatic-chisel-depth.md");
    assert!(text.contains("FR111") && text.contains("Bitloom"));
    assert!(text.contains("D1") || text.contains("multi-module") || text.contains("多模块"));
    assert!(text.contains("D3") || text.contains("FR111 per-module"));
    assert!(
        (text.contains("FR97") || text.contains("MVP") || text.contains("机械"))
            && (text.contains("≠") || text.contains("alone") || text.contains("不得")),
        "must contrast FR97/mechanical alone ≠ FR111"
    );
    assert!(text.contains("Parser") && (text.contains("not") || text.contains("不得")));
}

#[test]
fn fr111_deepen_emit_passes_on_hierarchy() {
    let hir = hierarchy_fixture();
    assert!(hir.circuit().modules.len() >= 2);
    let art = emit_chisel_idiomatic_fr111(&hir).expect("FR111 emit");
    let scala = &art.files[0].contents;
    check_idiomatic_chisel_fr111(scala, &hir).expect("FR111 check");
    assert!(scala.contains("FR111 deepen") || scala.contains("FR111"));
    assert!(scala.matches("--- FR111 per-module ---").count() >= 2);
    assert!(scala.contains("class Child") && scala.contains("class Top"));
}

#[test]
fn fr111_rejects_fr97_mvp_alone_not_silent() {
    let hir = hierarchy_fixture();
    let scala = emit_chisel_idiomatic(&hir).expect("FR97 emit").files[0]
        .contents
        .clone();
    check_idiomatic_chisel(&scala, &hir).expect("FR97 still green");
    let err = check_idiomatic_chisel_fr111(&scala, &hir).expect_err("FR97 alone ≠ FR111");
    let msg = err.to_string();
    assert!(
        msg.contains("FR111")
            && (msg.contains("deepen") || msg.contains("MVP") || msg.contains("缺少")),
        "readable FR111 fail: {msg}"
    );
}

#[test]
fn fr111_rejects_mechanical_emit() {
    let hir = hierarchy_fixture();
    let scala = emit_chisel(&hir).expect("mechanical").files[0]
        .contents
        .clone();
    let err = check_idiomatic_chisel_fr111(&scala, &hir).expect_err("mechanical ≠ FR111");
    assert!(
        err.to_string().contains("FR97")
            || err.to_string().contains("FR111")
            || err.to_string().contains("idiomatic")
    );
}

#[test]
fn fr111_d1_fails_when_per_module_marker_stripped() {
    let hir = hierarchy_fixture();
    let mut scala = emit_chisel_idiomatic_fr111(&hir).expect("emit").files[0]
        .contents
        .clone();
    // Strip only Child's FR111 marker (keep Top) → D3/D1 scoped fail
    scala = scala.replacen("  // --- FR111 per-module ---\n", "", 1);
    let err = check_idiomatic_chisel_fr111(&scala, &hir).expect_err("missing Child marker");
    let msg = err.to_string();
    assert!(
        msg.contains("FR111")
            && (msg.contains("Child") || msg.contains("per-module") || msg.contains("D3")),
        "{msg}"
    );
}

#[test]
fn fr111_nfr44_fr97_docs_still_present() {
    let text = read("docs/fr97-idiomatic-chisel.md");
    assert!(text.contains("FR97") && text.contains("idiomatic"));
}
