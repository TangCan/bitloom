//! ATDD Story 21.5: FR30 equivalence on the FR47 generated path (P3 acceptance).

use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn fr30_docs_p3_generated_path_contract() {
    let doc = fs::read_to_string(workspace_root().join("docs/fr30-dual-view-equiv.md")).unwrap();
    assert!(
        doc.contains("check_functional_equiv_generated")
            || doc.contains("generated path")
            || doc.contains("生成路径"),
        "FR30 doc must name the generated-path equiv entry"
    );
    assert!(
        doc.contains("P3")
            && (doc.contains("acceptance")
                || doc.contains("验收")
                || doc.contains("gate")
                || doc.contains("Acceptance")),
        "FR30 doc must state P3 acceptance is the generated path"
    );
    assert!(
        (doc.contains("coexist") || doc.contains("并存") || doc.contains("Handwritten"))
            && (doc.contains("not") || doc.contains("不") || doc.contains("may")),
        "FR30 doc must allow handwritten coexistence without making it the P3 gate"
    );

    let sim = fs::read_to_string(workspace_root().join("crates/bitloom-sim/src/equiv.rs")).unwrap();
    assert!(
        sim.contains("check_functional_equiv_generated"),
        "equiv.rs must export check_functional_equiv_generated"
    );
}

#[test]
fn fr30_generated_path_pass_and_fail() {
    use bitloom_builder::{ElaborateSession, GroundType, Span};
    use bitloom_hir::PortValues;
    use bitloom_sim::{
        AbstractionView, check_functional_equiv_generated, check_generated_bridge_with,
        reset_then_run,
    };

    let mut s = ElaborateSession::new("t");
    s.begin_module("Counter", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("data_in", GroundType::UInt { width: 8 }, Span::default());
    s.add_output("data_out", GroundType::UInt { width: 8 }, Span::default());
    s.declare_reg("count", GroundType::UInt { width: 8 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_net("data_out", "count", Span::default());
    s.end_process();
    s.begin_sequential(Span::default());
    s.assign_reg_d_inc("count", Span::default());
    s.end_process();
    s.end_module();
    let hir = s.finish().unwrap();

    // Pass: generated functional vs tick
    assert!(
        check_functional_equiv_generated(hir.clone(), reset_then_run(8)).is_pass(),
        "generated-path FR30 must pass on matching views"
    );

    // Fail: deliberate mismatch (SM-7)
    struct Wrong;
    impl AbstractionView for Wrong {
        fn cycle(&mut self, inputs: &PortValues) -> PortValues {
            let mut o = inputs.clone();
            o.set("data_out", 123);
            o
        }
    }
    let mut w = Wrong;
    assert!(
        !check_generated_bridge_with(hir, &mut w, reset_then_run(2)).is_pass(),
        "generated-path FR30 must fail on deliberate mismatch"
    );
}
