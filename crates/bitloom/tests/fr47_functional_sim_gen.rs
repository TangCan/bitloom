//! ATDD Story 21.3: FR47 leg 1 — generate Rust functional-sim artifact + gold PortValues.

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
fn fr47_docs_and_generator_surface() {
    let root = workspace_root();
    let doc = fs::read_to_string(root.join("docs/fr47-dual-sim-generation.md"))
        .expect("docs/fr47-dual-sim-generation.md");
    assert!(
        doc.contains("generate_functional_sim") && doc.contains("FR47"),
        "FR47 doc must name generate_functional_sim"
    );
    assert!(
        doc.contains("bitloom-prelude")
            && (doc.contains("toolchain") || doc.contains("工具链") || doc.contains("CLI")),
        "FR47 doc must keep design crates on bitloom-prelude; generator in toolchain"
    );
    assert!(
        doc.contains("SystemC")
            && (doc.contains("Not") || doc.contains("not") || doc.contains("Non-goals")),
        "FR47 doc must exclude SystemC"
    );

    let sim = fs::read_to_string(root.join("crates/bitloom-sim/src/lib.rs")).expect("sim lib");
    assert!(
        sim.contains("generate_functional_sim") || sim.contains("mod generate"),
        "bitloom-sim must expose FR47 generator module"
    );
    let main = fs::read_to_string(root.join("crates/bitloom/src/main.rs")).expect("cli");
    assert!(
        main.contains("GenFunc") || main.contains("gen-func"),
        "CLI must expose gen-func"
    );
}

#[test]
fn fr47_gold_port_values_and_emitted_crate_compiles() {
    use bitloom_builder::{ElaborateSession, GroundType, Span};
    use bitloom_hir::PortValues;
    use bitloom_sim::{GeneratedFunctional, Sim, check_mixed_both, generate_functional_sim};

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

    // Gold: in-process generated functional vs tick → data_out == 3
    let mut sim = Sim::new(hir.clone());
    let mut abs = GeneratedFunctional::from_hir(&hir);
    let mut pv = PortValues::default();
    pv.set("rst", 1);
    check_mixed_both(&mut sim, &mut abs, pv.clone()).unwrap();
    pv.set("rst", 0);
    for _ in 0..3 {
        check_mixed_both(&mut sim, &mut abs, pv.clone()).unwrap();
    }
    assert_eq!(sim.ports().get("data_out"), Some(3));

    // Emit crate and `cargo test` it (documents cargo test path)
    let out = workspace_root().join("target/bitloom-func-sim-atdd");
    let _ = fs::remove_dir_all(&out);
    generate_functional_sim(&hir, &out).expect("generate");
    assert!(out.join("src/lib.rs").is_file());
    assert!(out.join("src/main.rs").is_file());
    let status = Command::new("cargo")
        .arg("+1.97.1")
        .arg("test")
        .arg("--manifest-path")
        .arg(out.join("Cargo.toml"))
        .arg("--quiet")
        .status()
        .expect("spawn cargo test on generated crate");
    assert!(
        status.success(),
        "generated functional-sim crate cargo test failed"
    );
}
