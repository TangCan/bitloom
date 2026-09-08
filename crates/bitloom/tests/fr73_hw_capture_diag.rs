//! ATDD: FR73 / NFR35 capturing Wire/Reg diagnostics + FR16 regression (Story 27.3).
//!
//! Illegal hardware-ref capture → `rhdl::E0142` (not silent success).
//! Cycle-accurate capturing-closure ban remains `rhdl::E0141` (FR16).

use bitloom_builder::{ElaborateSession, GroundType, HwCaptureKind, HwCaptureRef, Span};
use bitloom_prelude::HwCaptureRef as PreludeCapture;

#[test]
fn capturing_wire_into_generator_context_fails_e0142() {
    let mut s = ElaborateSession::new("CapWire");
    s.begin_module("CapWire", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.declare_wire("w", GroundType::UInt { width: 8 }, Span::default());

    // Documented illegal capture: Wire mapped to HwCaptureRef (Option C).
    let illegal = HwCaptureRef::wire("w");
    assert_eq!(illegal.kind, HwCaptureKind::Wire);
    s.assert_no_hw_capture(&[illegal], Span::default());

    // Even a non-capturing init body must not mask the capture diagnostic.
    s.declare_mem_with_init_fn("rom", 4, 8, |i| i as u64, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("y", "rom", Span::default());
    s.end_process();
    s.end_module();

    let err = s.finish().expect_err("capture must fail elaborate");
    assert!(
        err.0.iter().any(|d| d.code == "rhdl::E0142"),
        "expected stable E0142, got: {err}"
    );
    assert!(
        err.0
            .iter()
            .any(|d| d.en.contains("Wire") && d.en.contains('w')),
        "diagnostic should name Wire 'w': {err}"
    );
}

#[test]
fn capturing_reg_into_factory_context_fails_e0142() {
    let mut s = ElaborateSession::new("CapReg");
    s.begin_module("Lane", Span::default());
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
    s.add_input("x0", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("y0", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("hold", GroundType::UInt { width: 8 }, Span::default());

    // Illegal: Reg capture token present while running factory (FR73).
    s.reject_hw_capture(&HwCaptureRef::reg("hold"), Span::default());
    s.generate_instances(1, |i, sess| {
        sess.add_instance(
            format!("u{i}"),
            "Lane",
            vec![
                ("clk".into(), "clk".into()),
                ("rst".into(), "rst".into()),
                ("x".into(), format!("x{i}")),
                ("y".into(), format!("y{i}")),
            ],
            vec![],
            Span::default(),
        );
    });
    s.end_module();

    let err = s.finish().expect_err("Reg capture must fail");
    assert!(
        err.0.iter().any(|d| d.code == "rhdl::E0142"),
        "expected E0142, got: {err}"
    );
    assert!(
        err.0
            .iter()
            .any(|d| d.en.contains("Reg") && d.en.contains("hold")),
        "diagnostic should name Reg 'hold': {err}"
    );
}

#[test]
fn prelude_hw_capture_ref_reexport_matches_builder() {
    let a = HwCaptureRef::signal("data_in");
    let b = PreludeCapture::signal("data_in");
    assert_eq!(a, b);
    assert_eq!(a.kind, HwCaptureKind::Signal);
}

#[test]
fn empty_assert_keeps_positive_generator_green() {
    let mut s = ElaborateSession::new("Ok");
    s.begin_module("Ok", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.assert_no_hw_capture(&[], Span::default());
    s.declare_mem_with_init_fn("rom", 4, 8, |i| ((i * i) & 0xff) as u64, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("y", "rom", Span::default());
    s.end_process();
    s.end_module();
    assert!(s.finish().is_ok(), "non-capturing path must stay green");
}

#[test]
fn fr16_capturing_closure_negative_still_e0141() {
    // FR16 / AD-18 cycle-accurate path: capturing closure remains rejected.
    let mut s = ElaborateSession::new("Fr16");
    s.begin_module("Fr16", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
    s.reject_unsynthesizable("capturing closure", Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("data_out", "data_in", Span::default());
    s.end_process();
    s.end_module();
    let err = s
        .finish()
        .expect_err("FR16 capturing closure must stay red");
    assert!(
        err.0.iter().any(|d| d.code == "rhdl::E0141"),
        "FR16 regression expects E0141, got: {err}"
    );
    assert!(
        !err.0.iter().any(|d| d.code == "rhdl::E0142"),
        "FR16 capturing closure must not be reclassified as E0142"
    );
}
