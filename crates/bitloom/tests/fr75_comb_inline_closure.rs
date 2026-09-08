//! ATDD: FR75 comb inline synthesizable closures (Story 28.2 / Cap-R-55).
//!
//! Recipe (also covered by `just test` / `cargo test --workspace`):
//! ```text
//! cargo test -p bitloom --test fr75_comb_inline_closure
//! ```
//!
//! Design surface: `bitloom-prelude` / `ElaborateSession::inline_comb_fn`.
//! Closures dissolve before freeze; backends see only ordinary assigns (NFR36).

use bitloom_builder::{
    CombInline, ElaborateSession, GroundType, LegalSimpleClosure, Span,
    SynthesizableClosureViolation,
};
use bitloom_hir::{AssignExpr, AssignTarget, PortValues, Stmt};
use bitloom_prelude::CombInline as PreludeCombInline;
use bitloom_sim::Sim;

/// NFR36 spot-check: backends must not emit closure/callback IR tokens.
fn assert_no_closure_ir(label: &str, text: &str) {
    let lower = text.to_lowercase();
    let has_fn_ir = ["Fn(", "Fn (", "FnOnce", "FnMut", "dyn Fn"]
        .iter()
        .any(|needle| {
            let mut start = 0;
            while let Some(rel) = text[start..].find(needle) {
                let abs = start + rel;
                let prev_ok = abs == 0 || !text.as_bytes()[abs - 1].is_ascii_alphanumeric();
                if prev_ok {
                    return true;
                }
                start = abs + 1;
            }
            false
        });
    let bad =
        lower.contains("closure") || lower.contains("callback") || has_fn_ir || text.contains("||");
    assert!(
        !bad,
        "{label}: emit must not contain closure/callback IR (NFR36):\n{text}"
    );
}

/// `y = sel ? (a + b) : a` — either via `inline_comb_fn` or handwritten `assign_*`.
fn mux_add_hir(name: &str, via_inline: bool) -> bitloom_hir::FrozenHir {
    let mut s = ElaborateSession::new(name);
    s.begin_module(name, Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
    s.add_input("b", GroundType::UInt { width: 8 }, Span::default());
    s.add_input("sel", GroundType::Bool, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.declare_wire("sum", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    if via_inline {
        s.inline_comb_fn("sum", &["a", "b"], &[], Span::default(), |args| {
            CombInline::Add(args[0].into(), args[1].into())
        });
        s.inline_comb_fn_marker(
            "y",
            &["sel", "sum", "a"],
            &LegalSimpleClosure,
            Span::default(),
            |args| CombInline::Mux {
                sel: args[0].into(),
                t: args[1].into(),
                f: args[2].into(),
            },
        );
    } else {
        s.assign_add("sum", "a", "b", Span::default());
        s.assign_mux("y", "sel", "sum", "a", Span::default());
    }
    s.end_process();
    s.end_module();
    s.finish().expect("elaborate")
}

fn comb_assigns(hir: &bitloom_hir::FrozenHir) -> Vec<(String, AssignExpr)> {
    hir.circuit().modules[0]
        .body
        .iter()
        .find_map(|st| match st {
            Stmt::Process(p) => Some(
                p.assigns
                    .iter()
                    .map(|a| {
                        let name = match &a.target {
                            AssignTarget::Net(n) => n.clone(),
                            other => panic!("unexpected target {other:?}"),
                        };
                        (name, a.expr.clone())
                    })
                    .collect(),
            ),
            _ => None,
        })
        .expect("combinational process")
}

fn tick_y(sim: &mut Sim, a: u64, b: u64, sel: u64) -> u64 {
    let mut pv = PortValues::default();
    pv.set("rst", 0);
    pv.set("a", a);
    pv.set("b", b);
    pv.set("sel", sel);
    sim.set_inputs(pv);
    sim.tick();
    sim.ports().get("y").expect("y")
}

#[test]
fn fr75_comb_inline_matches_handwritten_emit_and_tick() {
    let via_fn = mux_add_hir("MuxAddGen", true);
    let via_hand = mux_add_hir("MuxAddHand", false);

    assert_eq!(comb_assigns(&via_fn), comb_assigns(&via_hand));

    let v_fn = bitloom_vlog::emit(&via_fn).files[0].contents.clone();
    let v_hand = bitloom_vlog::emit(&via_hand).files[0].contents.clone();
    // Strip module name differences for body compare.
    let body = |v: &str| {
        let start = v.find("assign ").expect("assign");
        v[start..].to_string()
    };
    assert_eq!(
        body(&v_fn),
        body(&v_hand),
        "Verilog assign bodies must match\nfn:\n{v_fn}\nhand:\n{v_hand}"
    );

    assert_no_closure_ir("verilog/fn", &v_fn);
    assert_no_closure_ir("verilog/hand", &v_hand);

    let fir_fn = rhdl_firrtl::emit(&via_fn).files[0].contents.clone();
    let fir_hand = rhdl_firrtl::emit(&via_hand).files[0].contents.clone();
    assert_no_closure_ir("firrtl/fn", &fir_fn);
    assert_no_closure_ir("firrtl/hand", &fir_hand);

    let mut sim_fn = Sim::new(via_fn);
    let mut sim_hand = Sim::new(via_hand);
    // Values stay within u8 add range so tick golden matches sim wrapping_add semantics.
    for (a, b, sel) in [(1u64, 2, 1), (0x10, 0x20, 0), (0x7F, 1, 1), (7, 8, 0)] {
        let y_fn = tick_y(&mut sim_fn, a, b, sel);
        let y_hand = tick_y(&mut sim_hand, a, b, sel);
        let golden = if sel != 0 { a.wrapping_add(b) } else { a };
        assert_eq!(y_fn, y_hand, "tick mismatch a={a} b={b} sel={sel}");
        assert_eq!(y_fn, golden, "golden mismatch a={a} b={b} sel={sel}");
    }
}

#[test]
fn fr75_violation_blocks_inline_expand() {
    let mut s = ElaborateSession::new("BadInline");
    s.begin_module("BadInline", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.inline_comb_fn(
        "y",
        &["a"],
        &[SynthesizableClosureViolation::impure(
            "println in transform",
        )],
        Span::default(),
        |args| CombInline::Ref(args[0].into()),
    );
    s.end_process();
    s.end_module();
    let err = s.finish().expect_err("impure must fail Cap-R-60");
    assert!(
        err.0.iter().any(|d| d.code == "rhdl::E0145"),
        "expected E0145, got {err}"
    );
}

#[test]
fn fr75_incomplete_assign_still_e0110() {
    let mut s = ElaborateSession::new("LatchInline");
    s.begin_module("LatchInline", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.begin_then(Span::default());
    s.inline_comb_fn("y", &["a"], &[], Span::default(), |args| {
        CombInline::Ref(args[0].into())
    });
    s.begin_else(Span::default());
    s.end_if(Span::default());
    s.end_process();
    s.end_module();
    let err = s.finish().expect_err("incomplete assign must fail");
    assert!(
        err.0.iter().any(|d| d.code == "rhdl::E0110"),
        "expected E0110 latch, got {err}"
    );
}

#[test]
fn fr75_prelude_comb_inline_reexport() {
    let _ = PreludeCombInline::Lit(0);
}
