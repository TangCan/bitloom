//! FR188 — community Style Guide pack gate (beyond FR181 alone).
//!
//! Invoked by `scripts/chisel-style-guide-pack-check.sh`.

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_firrtl::{check_chisel_style_guide_pack_fr188, emit_chisel_style_guide_pack_fr188};
use std::process::ExitCode;

fn fixture() -> bitloom_hir::FrozenHir {
    let mut s = ElaborateSession::new("fr188");
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
    let art = match emit_chisel_style_guide_pack_fr188(&hir) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: FR188 style-guide-pack gate: emit failed: {e}");
            return ExitCode::FAILURE;
        }
    };
    let scala = &art.files[0].contents;
    if let Err(e) = check_chisel_style_guide_pack_fr188(scala, &hir) {
        eprintln!("error: FR188 style-guide-pack gate: check failed: {e}");
        return ExitCode::FAILURE;
    }
    println!("fr188_style_guide_pack_gate: OK community Style Guide pack (≠ FR181 alone)");
    ExitCode::SUCCESS
}
