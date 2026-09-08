//! ATDD matrix: FR74/FR75 synthesizable closures coexist with FR16 capture bans
//! (Story 28.4 / NFR35 / NFR36).
//!
//! Recipe (also covered by `just test` / `cargo test --workspace`):
//! ```text
//! cargo test -p bitloom --test fr74_fr75_fr16_coexist_matrix
//! ```
//!
//! Deep fixtures (rows consolidated here; see those files for fuller goldens):
//! - `fr74_synthesizable_closure_check` — Cap-R-60 E0143–E0145 + legal markers
//! - `fr75_comb_inline_closure` — Cap-R-55 comb inline tick/emit
//! - `fr75_seq_inline_closure` — Cap-R-56/70 seq inline + E0146
//! - `fr73_hw_capture_diag` — Wire/Reg E0142 + FR16 E0141 regression

use bitloom_builder::{
    CombInline, ElaborateSession, GroundType, HwCaptureRef, SeqInline, SeqOwnershipViolation, Span,
    SynthesizableClosureViolation,
};
use bitloom_hir::PortValues;
use bitloom_sim::Sim;
use std::path::PathBuf;

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

fn finish_code(err: &bitloom_hir::Diagnostics) -> Vec<String> {
    err.0.iter().map(|d| d.code.clone()).collect()
}

#[test]
fn matrix_sibling_fixtures_present() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
    for name in [
        "fr74_synthesizable_closure_check.rs",
        "fr75_comb_inline_closure.rs",
        "fr75_seq_inline_closure.rs",
        "fr73_hw_capture_diag.rs",
    ] {
        assert!(
            root.join(name).is_file(),
            "matrix consolidates sibling fixture {name}"
        );
    }
}

#[test]
fn matrix_positive_comb_and_seq_inline_pass_with_nfr36_emit() {
    // Comb: y = a + b via Cap-R-55
    let mut comb = ElaborateSession::new("MtxComb");
    comb.begin_module("MtxComb", Span::default());
    comb.add_input("clk", GroundType::Clock, Span::default());
    comb.add_input("rst", GroundType::Reset, Span::default());
    comb.add_input("a", GroundType::UInt { width: 8 }, Span::default());
    comb.add_input("b", GroundType::UInt { width: 8 }, Span::default());
    comb.add_output("y", GroundType::UInt { width: 8 }, Span::default());
    comb.begin_combinational(Span::default());
    comb.check_synthesizable_closure(&[], Span::default());
    comb.inline_comb_fn("y", &["a", "b"], &[], Span::default(), |args| {
        CombInline::Add(args[0].into(), args[1].into())
    });
    comb.end_process();
    comb.end_module();
    let comb_hir = comb.finish().expect("legal comb inline must pass");

    let v = bitloom_vlog::emit(&comb_hir).files[0].contents.clone();
    let fir = rhdl_firrtl::emit(&comb_hir).files[0].contents.clone();
    assert_no_closure_ir("matrix/comb/verilog", &v);
    assert_no_closure_ir("matrix/comb/firrtl", &fir);

    let mut sim = Sim::new(comb_hir);
    let mut pv = PortValues::default();
    pv.set("rst", 0);
    pv.set("a", 3);
    pv.set("b", 4);
    sim.set_inputs(pv);
    sim.tick();
    assert_eq!(sim.ports().get("y").expect("y"), 7);

    // Seq: count.d = count + 1 via Cap-R-56
    let mut seq = ElaborateSession::new("MtxSeq");
    seq.begin_module("MtxSeq", Span::default());
    seq.add_input("clk", GroundType::Clock, Span::default());
    seq.add_input("rst", GroundType::Reset, Span::default());
    seq.add_output("count_out", GroundType::UInt { width: 8 }, Span::default());
    seq.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
    seq.begin_sequential(Span::default());
    seq.inline_seq_fn("count", &[], &[], &[], Span::default(), |_args| {
        SeqInline::Inc
    });
    seq.end_process();
    seq.begin_combinational(Span::default());
    seq.assign_net("count_out", "count", Span::default());
    seq.end_process();
    seq.end_module();
    let seq_hir = seq.finish().expect("legal seq inline must pass");

    let v_seq = bitloom_vlog::emit(&seq_hir).files[0].contents.clone();
    assert_no_closure_ir("matrix/seq/verilog", &v_seq);

    let mut sim_seq = Sim::new(seq_hir);
    for expect in 1u64..=3 {
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        sim_seq.set_inputs(pv);
        sim_seq.tick();
        assert_eq!(sim_seq.ports().get("count_out").expect("count_out"), expect);
    }
}

#[test]
fn matrix_negatives_heap_capture_impure_fr16_wire_ownership() {
    // Row: heap → E0143 (FR74)
    {
        let mut s = ElaborateSession::new("MtxHeap");
        s.begin_module("MtxHeap", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.inline_comb_fn(
            "y",
            &["a"],
            &[SynthesizableClosureViolation::heap("Vec in body")],
            Span::default(),
            |args| CombInline::Ref(args[0].into()),
        );
        s.end_process();
        s.end_module();
        let err = s.finish().expect_err("heap");
        assert!(
            finish_code(&err).iter().any(|c| c == "rhdl::E0143"),
            "E0143: {err}"
        );
    }

    // Row: runtime capture state → E0144 (FR74)
    {
        let mut s = ElaborateSession::new("MtxCapState");
        s.begin_module("MtxCapState", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.check_synthesizable_closure(
            &[SynthesizableClosureViolation::runtime_capture_state(
                "threshold",
            )],
            Span::default(),
        );
        s.begin_combinational(Span::default());
        s.assign_net("y", "a", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().expect_err("capture state");
        assert!(
            finish_code(&err).iter().any(|c| c == "rhdl::E0144"),
            "E0144: {err}"
        );
    }

    // Row: impure / illegal IO → E0145 (FR74)
    {
        let mut s = ElaborateSession::new("MtxImpure");
        s.begin_module("MtxImpure", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.begin_combinational(Span::default());
        s.inline_comb_fn(
            "y",
            &["a"],
            &[SynthesizableClosureViolation::impure("stdio")],
            Span::default(),
            |args| CombInline::Ref(args[0].into()),
        );
        s.end_process();
        s.end_module();
        let err = s.finish().expect_err("impure");
        assert!(
            finish_code(&err).iter().any(|c| c == "rhdl::E0145"),
            "E0145: {err}"
        );
    }

    // Row: capture Wire → E0142 (FR73 / NFR35); do not capture Wire
    {
        let mut s = ElaborateSession::new("MtxWire");
        s.begin_module("MtxWire", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.declare_wire("w", GroundType::UInt { width: 8 }, Span::default());
        s.reject_hw_capture(&HwCaptureRef::wire("w"), Span::default());
        s.begin_combinational(Span::default());
        s.assign_lit("y", 0, Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().expect_err("Wire capture");
        assert!(
            finish_code(&err).iter().any(|c| c == "rhdl::E0142"),
            "E0142: {err}"
        );
    }

    // Row: FR16 capturing closure still E0141 (not reclassified)
    {
        let mut s = ElaborateSession::new("MtxFr16");
        s.begin_module("MtxFr16", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("a", GroundType::UInt { width: 8 }, Span::default());
        s.add_output("y", GroundType::UInt { width: 8 }, Span::default());
        s.reject_unsynthesizable("capturing closure", Span::default());
        s.begin_combinational(Span::default());
        s.assign_net("y", "a", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().expect_err("FR16");
        let codes = finish_code(&err);
        assert!(codes.iter().any(|c| c == "rhdl::E0141"), "E0141: {err}");
        assert!(
            !codes.iter().any(|c| c == "rhdl::E0142"),
            "FR16 must stay E0141, not E0142: {err}"
        );
    }

    // Row: Cap-R-70 illegal mutable borrow → E0146 (FR75)
    {
        let mut s = ElaborateSession::new("MtxOwn");
        s.begin_module("MtxOwn", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_output("q_out", GroundType::UInt { width: 8 }, Span::default());
        s.declare_reg("q", GroundType::UInt { width: 8 }, Span::default());
        s.begin_sequential(Span::default());
        s.inline_seq_fn(
            "q",
            &[],
            &[],
            &[SeqOwnershipViolation::illegal_mutable_borrow(
                "second Reg.d",
            )],
            Span::default(),
            |_args| SeqInline::Inc,
        );
        s.end_process();
        s.begin_combinational(Span::default());
        s.assign_net("q_out", "q", Span::default());
        s.end_process();
        s.end_module();
        let err = s.finish().expect_err("ownership");
        assert!(
            finish_code(&err).iter().any(|c| c == "rhdl::E0146"),
            "E0146: {err}"
        );
    }
}
