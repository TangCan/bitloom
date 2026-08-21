//! ATDD Story 21.4: FR47 leg 2 — cycle-accurate artifact + bridge/compare.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

#[test]
fn fr47_leg2_docs_and_cli_surface() {
    let root = workspace_root();
    let doc = fs::read_to_string(root.join("docs/fr47-dual-sim-generation.md")).unwrap();
    assert!(
        doc.contains("generate_cycle_accurate_sim") && doc.contains("check_generated_bridge"),
        "FR47 doc must cover cycle-accurate generate + bridge"
    );
    assert!(
        doc.contains("gen-cycle") || doc.contains("GenCycle"),
        "FR47 doc must mention gen-cycle CLI"
    );
    let main = fs::read_to_string(root.join("crates/bitloom/src/main.rs")).unwrap();
    assert!(main.contains("GenCycle"));
    let sim = fs::read_to_string(root.join("crates/bitloom-sim/src/lib.rs")).unwrap();
    assert!(sim.contains("mod cycle") || sim.contains("check_generated_bridge"));
}

#[test]
fn fr47_bridge_pass_fail_and_cycle_crate_tests() {
    use bitloom_builder::{ElaborateSession, GroundType, Span};
    use bitloom_hir::PortValues;
    use bitloom_sim::{
        AbstractionView, check_generated_bridge, check_generated_bridge_with,
        generate_cycle_accurate_sim, reset_then_run,
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

    assert!(
        check_generated_bridge(hir.clone(), reset_then_run(3)).is_pass(),
        "matching generated functional vs tick must pass"
    );

    struct Wrong;
    impl AbstractionView for Wrong {
        fn cycle(&mut self, inputs: &PortValues) -> PortValues {
            let mut o = inputs.clone();
            o.set("data_out", 42);
            o
        }
    }
    let mut w = Wrong;
    assert!(
        !check_generated_bridge_with(hir.clone(), &mut w, reset_then_run(1)).is_pass(),
        "deliberate mismatch must fail (prep 21.5)"
    );

    let out = workspace_root().join("target/bitloom-cycle-sim-atdd");
    let _ = fs::remove_dir_all(&out);
    generate_cycle_accurate_sim(&hir, &out).expect("generate cycle crate");
    let status = Command::new("cargo")
        .arg("+1.97.1")
        .arg("test")
        .arg("--manifest-path")
        .arg(out.join("Cargo.toml"))
        .arg("--quiet")
        .status()
        .expect("spawn cargo test");
    assert!(
        status.success(),
        "generated cycle-accurate crate cargo test failed"
    );
}
