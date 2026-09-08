//! ATDD: FR74 SynthesizableClosure constraints + Cap-R-60 check hook (Story 28.1).
//!
//! Violations → stable `rhdl::E0143` / `E0144` / `E0145`.
//! Legal empty/simple markers pass. No new closure IR in FrozenHir (NFR36).

use bitloom_builder::{
    ElaborateSession, GroundType, LegalEmptyClosure, LegalSimpleClosure, Span,
    SynthesizableClosureViolation, diagnose_synthesizable_closure_violations,
};
use bitloom_prelude::{
    LegalEmptyClosure as PreludeEmpty, SynthesizableClosureViolation as PreludeViolation,
};

fn minimal_module(s: &mut ElaborateSession, name: &str) {
    s.begin_module(name, Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
}

#[test]
fn heap_violation_stable_e0143() {
    let mut s = ElaborateSession::new("HeapBad");
    minimal_module(&mut s, "HeapBad");
    s.reject_unsynthesizable_closure(
        &SynthesizableClosureViolation::heap("Vec<u8> / Box in body"),
        Span::default(),
    );
    s.begin_combinational(Span::default());
    s.assign_net("data_out", "data_in", Span::default());
    s.end_process();
    s.end_module();
    let err = s.finish().expect_err("heap must fail");
    assert!(
        err.0.iter().any(|d| d.code == "rhdl::E0143"),
        "expected E0143, got: {err}"
    );
    assert!(
        err.0.iter().any(|d| d.en.contains("heap")),
        "diagnostic should mention heap: {err}"
    );
}

#[test]
fn runtime_capture_state_stable_e0144() {
    let mut s = ElaborateSession::new("CapState");
    minimal_module(&mut s, "CapState");
    s.check_synthesizable_closure(
        &[SynthesizableClosureViolation::runtime_capture_state(
            "captures runtime threshold",
        )],
        Span::default(),
    );
    s.begin_combinational(Span::default());
    s.assign_net("data_out", "data_in", Span::default());
    s.end_process();
    s.end_module();
    let err = s.finish().expect_err("capture state must fail");
    assert!(
        err.0.iter().any(|d| d.code == "rhdl::E0144"),
        "expected E0144, got: {err}"
    );
}

#[test]
fn impure_violation_stable_e0145() {
    let mut s = ElaborateSession::new("Impure");
    minimal_module(&mut s, "Impure");
    s.reject_unsynthesizable_closure(
        &SynthesizableClosureViolation::impure("stdio side effect"),
        Span::default(),
    );
    s.begin_combinational(Span::default());
    s.assign_net("data_out", "data_in", Span::default());
    s.end_process();
    s.end_module();
    let err = s.finish().expect_err("impure must fail");
    assert!(
        err.0.iter().any(|d| d.code == "rhdl::E0145"),
        "expected E0145, got: {err}"
    );
}

#[test]
fn legal_empty_and_simple_pass_check() {
    let mut s = ElaborateSession::new("Ok");
    minimal_module(&mut s, "Ok");
    // Cap-R-60: empty violation list + marker stand-ins.
    s.check_synthesizable_closure(&[], Span::default());
    s.check_synthesizable_closure_marker(&LegalEmptyClosure, Span::default());
    s.check_synthesizable_closure_marker(&LegalSimpleClosure, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("data_out", "data_in", Span::default());
    s.end_process();
    s.end_module();
    let frozen = s.finish().expect("legal empty/simple must pass");
    // NFR36: check hook must not inject closure IR into FrozenHir.
    let dbg = format!("{frozen:?}");
    for bad in ["closure", "callback", "Fn(", "Fn (", "||"] {
        assert!(
            !dbg.contains(bad),
            "FrozenHir must not gain closure IR from check hook (NFR36): found {bad:?} in {dbg}"
        );
    }
}

#[test]
fn prelude_reexports_cap_r60_surface() {
    let a = SynthesizableClosureViolation::heap("x");
    let b = PreludeViolation::heap("x");
    assert_eq!(a, b);
    assert_eq!(a.code(), "rhdl::E0143");
    let _ = PreludeEmpty;
    let diags = diagnose_synthesizable_closure_violations(
        &[SynthesizableClosureViolation::runtime_capture_state("y")],
        Span::default(),
    );
    assert!(diags.0.iter().any(|d| d.code == "rhdl::E0144"));
}

#[test]
fn fr16_e0141_and_e0142_remain_distinct() {
    let mut s = ElaborateSession::new("Distinct");
    minimal_module(&mut s, "Distinct");
    s.reject_unsynthesizable("capturing closure", Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("data_out", "data_in", Span::default());
    s.end_process();
    s.end_module();
    let err = s.finish().expect_err("FR16 still red");
    assert!(err.0.iter().any(|d| d.code == "rhdl::E0141"));
    assert!(!err.0.iter().any(|d| {
        matches!(
            d.code.as_str(),
            "rhdl::E0143" | "rhdl::E0144" | "rhdl::E0145"
        )
    }));
}
