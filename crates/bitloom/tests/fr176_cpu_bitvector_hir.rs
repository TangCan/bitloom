use std::process::Command;

use bitloom_builder::{ElaborateSession, GroundType, Span};
use bitloom_hir::PortValues;
use bitloom_sim::{
    AbstractionView, CycleAccurateSim, GeneratedFunctional, generate_cycle_accurate_sim,
    generate_functional_sim,
};

fn hir() -> bitloom_hir::FrozenHir {
    let mut s = ElaborateSession::new("rv32_decode");
    s.begin_module("Rv32Decode", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("instr", GroundType::UInt { width: 32 }, Span::default());
    s.add_input("rs1_addr", GroundType::UInt { width: 5 }, Span::default());
    s.add_output("opcode", GroundType::UInt { width: 7 }, Span::default());
    s.add_output("rd", GroundType::UInt { width: 5 }, Span::default());
    s.add_output("rs1", GroundType::UInt { width: 5 }, Span::default());
    s.add_output("rs2", GroundType::UInt { width: 5 }, Span::default());
    s.add_output("imm", GroundType::UInt { width: 32 }, Span::default());
    s.add_output("gpr_data", GroundType::UInt { width: 32 }, Span::default());
    s.add_output("joined", GroundType::UInt { width: 12 }, Span::default());
    s.declare_wire("imm12", GroundType::UInt { width: 12 }, Span::default());
    s.declare_wire("top7", GroundType::UInt { width: 7 }, Span::default());
    s.declare_wire("low5", GroundType::UInt { width: 5 }, Span::default());
    s.declare_mem_with_init(
        "gpr",
        32,
        32,
        (0..32).map(|i| i as u64 * 3).collect(),
        Span::default(),
    );
    s.begin_combinational(Span::default());
    s.assign_slice("opcode", "instr", 0, 7, Span::default());
    s.assign_slice("rd", "instr", 7, 5, Span::default());
    s.assign_slice("rs1", "instr", 15, 5, Span::default());
    s.assign_slice("rs2", "instr", 20, 5, Span::default());
    s.assign_slice("imm12", "instr", 20, 12, Span::default());
    s.assign_sign_extend("imm", "imm12", 32, Span::default());
    s.assign_slice("top7", "instr", 25, 7, Span::default());
    s.assign_slice("low5", "instr", 20, 5, Span::default());
    s.assign_concat("joined", "top7", "low5", Span::default());
    s.assign_mem_read("gpr_data", "gpr", "rs1_addr", Span::default());
    s.end_process();
    s.end_module();
    s.finish().expect("valid rv32 decode fixture")
}

#[test]
fn rv32_decode_bitvectors_and_async_gpr_are_consistent() {
    let h = hir();
    let mut inputs = PortValues::default();
    inputs.set("clk", 0);
    inputs.set("rst", 0);
    inputs.set("instr", 0xfff0_8093); // addi x1, x1, -1
    inputs.set("rs1_addr", 31);
    let mut sim = bitloom_sim::Sim::new(h.clone());
    sim.set_inputs(inputs.clone());
    sim.tick();
    assert_eq!(sim.ports().get("opcode"), Some(0x13));
    assert_eq!(sim.ports().get("rd"), Some(1));
    assert_eq!(sim.ports().get("rs1"), Some(1));
    assert_eq!(sim.ports().get("rs2"), Some(31));
    assert_eq!(sim.ports().get("imm"), Some(0xffff_ffff));
    assert_eq!(sim.ports().get("joined"), Some(0xfff));
    assert_eq!(sim.ports().get("gpr_data"), Some(93));
    inputs.set("rs1_addr", 32);
    sim.set_inputs(inputs.clone());
    sim.tick();
    assert_eq!(sim.ports().get("gpr_data"), Some(0));
    inputs.set("rs1_addr", 31);
    inputs.set("instr", u32::MAX as u64);
    sim.set_inputs(inputs.clone());
    sim.tick();
    assert_eq!(sim.ports().get("opcode"), Some(0x7f));
    assert_eq!(sim.ports().get("rd"), Some(0x1f));
    inputs.set("instr", 0xfff0_8093);

    let mut generated = GeneratedFunctional::from_hir(&h);
    assert_eq!(generated.cycle(&inputs).get("imm"), Some(0xffff_ffff));
    let mut cycle = CycleAccurateSim::from_hir(h.clone());
    assert_eq!(cycle.tick_with(inputs.clone()).get("gpr_data"), Some(93));

    let vlog = bitloom_vlog::emit(&h).files.remove(0).contents;
    assert!(vlog.contains("instr[6:0]"));
    assert!(vlog.contains("gpr[rs1_addr]"));

    let root = std::env::temp_dir().join("bitloom-fr176-cpu-bitvectors");
    let _ = std::fs::remove_dir_all(&root);
    let functional = root.join("functional");
    let cycle = root.join("cycle");
    generate_functional_sim(&h, &functional).unwrap();
    generate_cycle_accurate_sim(&h, &cycle).unwrap();
    for dir in [&functional, &cycle] {
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
fn invalid_bitvectors_and_sync_memory_reads_are_diagnosed() {
    let mut s = ElaborateSession::new("invalid");
    s.begin_module("Invalid", Span::default());
    s.add_input("clk", GroundType::Clock, Span::default());
    s.add_input("rst", GroundType::Reset, Span::default());
    s.add_input("addr", GroundType::UInt { width: 4 }, Span::default());
    s.declare_wire("src", GroundType::UInt { width: 8 }, Span::default());
    s.declare_wire("bad", GroundType::UInt { width: 4 }, Span::default());
    s.declare_sync_read_mem("sync", 16, 8, Span::default());
    s.begin_combinational(Span::default());
    s.assign_slice("bad", "src", 6, 4, Span::default());
    s.assign_mem_read("bad", "sync", "addr", Span::default());
    s.end_process();
    s.end_module();
    let diagnostics = s.finish().expect_err("invalid operations must diagnose");
    assert!(diagnostics.0.iter().any(|d| d.code == "rhdl::E0133"));
    assert!(diagnostics.0.iter().any(|d| d.code == "rhdl::E0214"));
}
