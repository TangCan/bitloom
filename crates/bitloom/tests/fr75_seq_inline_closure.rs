//! ATDD: FR75 sequential inline synthesizable closures (Story 28.3 / Cap-R-56 / Cap-R-70).
//!
//! Recipe (also covered by `just test` / `cargo test --workspace`):
//! ```text
//! cargo test -p bitloom --test fr75_seq_inline_closure
//! ```
//!
//! Design surface: `bitloom-prelude` / `ElaborateSession::inline_seq_fn`.
//! Closures dissolve before freeze; backends see only ordinary Reg.d assigns (NFR36).

use bitloom_builder::{
    CombInline, ElaborateSession, GroundType, LegalSimpleClosure, SeqInline, SeqOwnershipViolation,
    Span, SynthesizableClosureViolation,
};
use bitloom_hir::{AssignExpr, AssignTarget, PortValues, ProcessKind, Stmt};
use bitloom_prelude::{
    SeqInline as PreludeSeqInline, SeqOwnershipViolation as PreludeSeqOwnershipViolation,
};
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

/// Accumulator: `q.d = load ? data : (q + data)` — both paths write Reg.d mux
/// (sim evaluates sequential before combinational, so a comb `next` wire would lag).
fn acc_hir(name: &str, via_inline: bool) -> bitloom_hir::FrozenHir {
    let mut s = ElaborateSession::new(name);
    s.begin_module(name, Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("data", GroundType::UInt { width: 8 }, Span::default());
    s.add_input("load", GroundType::Bool, Span::default());
    s.add_output("q_out", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("q", GroundType::UInt { width: 8 }, Span::default());
    s.declare_wire("sum", GroundType::UInt { width: 8 }, Span::default());

    s.begin_combinational(Span::default());
    s.assign_add("sum", "q", "data", Span::default());
    s.assign_net("q_out", "q", Span::default());
    s.end_process();
    s.begin_sequential(Span::default());
    if via_inline {
        s.inline_seq_fn_marker(
            "q",
            &["load", "data", "sum"],
            &LegalSimpleClosure,
            &[],
            Span::default(),
            |args| {
                CombInline::Mux {
                    sel: args[0].into(),
                    t: args[1].into(),
                    f: args[2].into(),
                }
                .into()
            },
        );
    } else {
        s.assign_reg_d_mux("q", "load", "data", "sum", Span::default());
    }
    s.end_process();

    s.end_module();
    s.finish().expect("elaborate")
}

fn counter_hir(name: &str, via_inline: bool) -> bitloom_hir::FrozenHir {
    let mut s = ElaborateSession::new(name);
    s.begin_module(name, Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_output("count_out", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
    s.begin_sequential(Span::default());
    if via_inline {
        s.inline_seq_fn("count", &[], &[], &[], Span::default(), |_args| {
            SeqInline::Inc
        });
    } else {
        s.assign_reg_d_inc("count", Span::default());
    }
    s.end_process();
    s.begin_combinational(Span::default());
    s.assign_net("count_out", "count", Span::default());
    s.end_process();
    s.end_module();
    s.finish().expect("elaborate")
}

fn seq_reg_d_assigns(hir: &bitloom_hir::FrozenHir) -> Vec<(String, AssignExpr)> {
    hir.circuit().modules[0]
        .body
        .iter()
        .find_map(|st| match st {
            Stmt::Process(p) if p.kind == ProcessKind::Sequential => Some(
                p.assigns
                    .iter()
                    .map(|a| {
                        let name = match &a.target {
                            AssignTarget::RegD(n) => n.clone(),
                            other => panic!("unexpected target {other:?}"),
                        };
                        (name, a.expr.clone())
                    })
                    .collect(),
            ),
            _ => None,
        })
        .expect("sequential process")
}

fn tick_q(sim: &mut Sim, data: u64, load: u64) -> u64 {
    let mut pv = PortValues::default();
    pv.set("rst", 0);
    pv.set("data", data);
    pv.set("load", load);
    sim.set_inputs(pv);
    sim.tick();
    sim.ports().get("q_out").expect("q_out")
}

fn tick_count(sim: &mut Sim) -> u64 {
    let mut pv = PortValues::default();
    pv.set("rst", 0);
    sim.set_inputs(pv);
    sim.tick();
    sim.ports().get("count_out").expect("count_out")
}

#[test]
fn fr75_seq_inline_counter_matches_handwritten_tick() {
    let via_fn = counter_hir("CntGen", true);
    let via_hand = counter_hir("CntHand", false);

    assert_eq!(seq_reg_d_assigns(&via_fn), seq_reg_d_assigns(&via_hand));

    let v_fn = bitloom_vlog::emit(&via_fn).files[0].contents.clone();
    let v_hand = bitloom_vlog::emit(&via_hand).files[0].contents.clone();
    assert_no_closure_ir("verilog/fn", &v_fn);
    assert_no_closure_ir("verilog/hand", &v_hand);

    let fir_fn = rhdl_firrtl::emit(&via_fn).files[0].contents.clone();
    assert_no_closure_ir("firrtl/fn", &fir_fn);

    let mut sim_fn = Sim::new(via_fn);
    let mut sim_hand = Sim::new(via_hand);
    for _ in 0..4 {
        let c_fn = tick_count(&mut sim_fn);
        let c_hand = tick_count(&mut sim_hand);
        assert_eq!(c_fn, c_hand);
    }
}

#[test]
fn fr75_seq_inline_acc_matches_handwritten_tick() {
    let via_fn = acc_hir("AccGen", true);
    let via_hand = acc_hir("AccHand", false);

    assert_eq!(seq_reg_d_assigns(&via_fn), seq_reg_d_assigns(&via_hand));

    let mut sim_fn = Sim::new(via_fn);
    let mut sim_hand = Sim::new(via_hand);
    // Stay within u8 wrapping_add range used by sim.
    for (data, load) in [(1u64, 1), (2, 0), (3, 0), (5, 1), (1, 0)] {
        let q_fn = tick_q(&mut sim_fn, data, load);
        let q_hand = tick_q(&mut sim_hand, data, load);
        assert_eq!(q_fn, q_hand, "tick mismatch data={data} load={load}");
    }

    let v_fn = bitloom_vlog::emit(&acc_hir("AccEmitFn", true)).files[0]
        .contents
        .clone();
    assert_no_closure_ir("verilog/acc", &v_fn);
}

#[test]
fn fr75_seq_cap_r70_illegal_mutable_borrow() {
    let mut s = ElaborateSession::new("BadOwn");
    s.begin_module("BadOwn", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("q", GroundType::UInt { width: 8 }, Span::default());
    s.begin_sequential(Span::default());
    s.assign_reg_d_inc("q", Span::default());
    s.inline_seq_fn("q", &[], &[], &[], Span::default(), |_args| SeqInline::Inc);
    s.end_process();
    s.begin_combinational(Span::default());
    s.assign_net("y", "q", Span::default());
    s.end_process();
    s.end_module();
    let err = s.finish().expect_err("Cap-R-70 must fire");
    assert!(
        err.0.iter().any(|d| d.code == "rhdl::E0146"),
        "expected E0146, got {err}"
    );
}

#[test]
fn fr75_seq_cap_r70_tokenized() {
    let mut s = ElaborateSession::new("TokOwn");
    s.begin_module("TokOwn", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("q", GroundType::UInt { width: 8 }, Span::default());
    s.begin_sequential(Span::default());
    s.inline_seq_fn(
        "q",
        &[],
        &[],
        &[SeqOwnershipViolation::illegal_mutable_borrow(
            "&mut q in closure body",
        )],
        Span::default(),
        |_args| SeqInline::Inc,
    );
    s.end_process();
    s.begin_combinational(Span::default());
    s.assign_net("y", "q", Span::default());
    s.end_process();
    s.end_module();
    let err = s.finish().expect_err("token Cap-R-70");
    assert!(err.0.iter().any(|d| d.code == "rhdl::E0146"));
}

#[test]
fn fr75_seq_synth_violation_blocks_expand() {
    let mut s = ElaborateSession::new("BadSynth");
    s.begin_module("BadSynth", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("q", GroundType::UInt { width: 8 }, Span::default());
    s.begin_sequential(Span::default());
    s.inline_seq_fn(
        "q",
        &[],
        &[SynthesizableClosureViolation::heap("Box in seq transform")],
        &[],
        Span::default(),
        |_args| SeqInline::Inc,
    );
    s.end_process();
    s.begin_combinational(Span::default());
    s.assign_net("y", "q", Span::default());
    s.end_process();
    s.end_module();
    let err = s.finish().expect_err("Cap-R-60 must fail");
    assert!(
        err.0.iter().any(|d| d.code == "rhdl::E0143"),
        "expected E0143, got {err}"
    );
}

#[test]
fn fr75_seq_multi_drive_still_e0140() {
    let mut s = ElaborateSession::new("Multi");
    s.begin_module("Multi", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("data", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("q", GroundType::UInt { width: 8 }, Span::default());
    s.begin_sequential(Span::default());
    s.inline_seq_fn("q", &[], &[], &[], Span::default(), |_args| SeqInline::Inc);
    s.end_process();
    s.begin_sequential(Span::default());
    s.assign_reg_d_from("q", "data", Span::default());
    s.end_process();
    s.begin_combinational(Span::default());
    s.assign_net("y", "q", Span::default());
    s.end_process();
    s.end_module();
    let err = s.finish().expect_err("AD-4 multi-drive");
    assert!(
        err.0.iter().any(|d| d.code == "rhdl::E0140"),
        "expected E0140, got {err}"
    );
}

#[test]
fn fr75_prelude_seq_inline_reexport() {
    let _ = PreludeSeqInline::Inc;
    let _ = PreludeSeqOwnershipViolation::illegal_mutable_borrow("x");
}
