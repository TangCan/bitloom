use std::process::Command;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::PortValues;
use bitloom_sim::{
    AbstractionView, CycleAccurateSim, GeneratedFunctional, generate_cycle_accurate_sim,
    generate_functional_sim,
};

fn hir() -> bitloom_hir::FrozenHir {
    let mut s = ElaborateSession::new("rv32_alu");
    s.begin_module("Rv32Alu", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("ua", GroundType::UInt { width: 32 }, Span::default());
    s.add_input("ub", GroundType::UInt { width: 32 }, Span::default());
    s.add_input("sa", GroundType::SInt { width: 32 }, Span::default());
    s.add_input("sb", GroundType::SInt { width: 32 }, Span::default());
    s.add_input("shamt", GroundType::UInt { width: 32 }, Span::default());
    s.add_output("sltu", GroundType::Bool, Span::default());
    s.add_output("slt", GroundType::Bool, Span::default());
    s.add_output("sra", GroundType::SInt { width: 32 }, Span::default());
    s.add_output("srl", GroundType::UInt { width: 32 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_ult("sltu", "ua", "ub", Span::default());
    s.assign_slt("slt", "sa", "sb", Span::default());
    s.assign_sar("sra", "sa", "shamt", Span::default());
    s.assign_shr("srl", "ua", "shamt", Span::default());
    s.end_process();
    s.end_module();
    s.finish().expect("valid typed RV32 ALU")
}

fn cycle(
    hir: bitloom_hir::FrozenHir,
    ua: u64,
    ub: u64,
    sa: u64,
    sb: u64,
    shamt: u64,
) -> PortValues {
    let mut sim = bitloom_sim::Sim::new(hir);
    let mut input = PortValues::default();
    input.set("clk", 0);
    input.set("rst", 0);
    input.set("ua", ua);
    input.set("ub", ub);
    input.set("sa", sa);
    input.set("sb", sb);
    input.set("shamt", shamt);
    sim.set_inputs(input);
    sim.tick();
    sim.ports().clone()
}

#[test]
fn rv32_signed_arithmetic_is_consistent_across_consumers() {
    let h = hir();
    let out = cycle(h.clone(), u32::MAX as u64, 0, u32::MAX as u64, 0, 31);
    assert_eq!(out.get("sltu"), Some(0));
    assert_eq!(out.get("slt"), Some(1));
    assert_eq!(out.get("sra"), Some(u32::MAX as u64));
    assert_eq!(out.get("srl"), Some(1));
    let out = cycle(
        h.clone(),
        0,
        u32::MAX as u64,
        i32::MIN as u32 as u64,
        i32::MAX as u64,
        63,
    );
    assert_eq!(out.get("sltu"), Some(1));
    assert_eq!(out.get("slt"), Some(1));
    assert_eq!(out.get("sra"), Some(u32::MAX as u64));

    let mut input = PortValues::default();
    input.set("sa", u32::MAX as u64);
    input.set("shamt", 31);
    let mut functional = GeneratedFunctional::from_hir(&h);
    assert_eq!(functional.cycle(&input).get("sra"), Some(u32::MAX as u64));
    let mut generated_cycle = CycleAccurateSim::from_hir(h.clone());
    assert_eq!(
        generated_cycle.tick_with(input.clone()).get("sra"),
        Some(u32::MAX as u64)
    );

    let verilog = bitloom_vlog::emit(&h).files.remove(0).contents;
    assert!(verilog.contains("$signed(sa) >>> shamt"));
    assert!(verilog.contains("$signed(sa) < $signed(sb)"));
    assert!(
        bitloom_firrtl::emit(&h).files[0]
            .contents
            .contains("dshr(asSInt(sa), shamt)")
    );
    assert!(
        bitloom_firrtl::emit_chisel(&h).unwrap().files[0]
            .contents
            .contains("io.sa >> io.shamt")
    );

    let root = std::env::temp_dir().join("bitloom-fr177-cpu-arithmetic");
    let _ = std::fs::remove_dir_all(&root);
    let functional_dir = root.join("functional");
    let cycle_dir = root.join("cycle");
    generate_functional_sim(&h, &functional_dir).unwrap();
    generate_cycle_accurate_sim(&h, &cycle_dir).unwrap();
    for dir in [&functional_dir, &cycle_dir] {
        assert!(
            Command::new("cargo")
                .args(["+1.97.1", "test", "--manifest-path"])
                .arg(dir.join("Cargo.toml"))
                .arg("--quiet")
                .status()
                .unwrap()
                .success()
        );
    }
}

#[test]
fn invalid_signedness_and_destinations_are_diagnosed() {
    let mut s = ElaborateSession::new("invalid");
    s.begin_module("Invalid", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("u", GroundType::UInt { width: 32 }, Span::default());
    s.add_input("short", GroundType::UInt { width: 5 }, Span::default());
    s.add_output("not_bool", GroundType::UInt { width: 1 }, Span::default());
    s.add_output("bad_sar", GroundType::UInt { width: 32 }, Span::default());
    s.begin_combinational(Span::default());
    s.assign_ult("not_bool", "u", "short", Span::default());
    s.assign_ult("not_bool", "u", "u", Span::default());
    s.assign_slt("not_bool", "u", "u", Span::default());
    s.assign_sar("bad_sar", "u", "short", Span::default());
    s.end_process();
    s.end_module();
    let diagnostics = s.finish().expect_err("invalid typed arithmetic");
    assert!(diagnostics.0.iter().any(|d| d.code == "rhdl::E0137"));
    assert!(diagnostics.0.iter().any(|d| d.code == "rhdl::E0138"));
    assert!(diagnostics.0.iter().any(|d| d.code == "rhdl::E0139"));
    assert!(!diagnostics.0.iter().any(|d| d.code == "rhdl::E0130"));
}
