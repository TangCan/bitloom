//! FR176 — Combined Chisel ecosystem deepen gate (beyond FR165 + FR170 alone).
//!
//! Invoked by `scripts/chisel-ecosystem-deepen-check.sh`. Proves emit + check of
//! the FR176 ecosystem pack; ≠ FR165 style-lint alone; ≠ FR170 parser alone.

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_firrtl::{check_chisel_ecosystem_fr176, emit_chisel_ecosystem_fr176};
use std::process::ExitCode;

fn fixture() -> bitloom_hir::FrozenHir {
    let mut s = ElaborateSession::new("fr176");
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
    s.finish().expect("elaborate")
}

fn main() -> ExitCode {
    let hir = fixture();
    let art = match emit_chisel_ecosystem_fr176(&hir) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: FR176 ecosystem gate: emit failed: {e}");
            return ExitCode::FAILURE;
        }
    };
    let scala = &art.files[0].contents;
    if let Err(e) = check_chisel_ecosystem_fr176(scala, &hir) {
        eprintln!("error: FR176 ecosystem gate: check failed: {e}");
        return ExitCode::FAILURE;
    }
    println!("fr176_ecosystem_gate: OK combined ecosystem pack (≠ FR165/FR170 alone)");
    ExitCode::SUCCESS
}
