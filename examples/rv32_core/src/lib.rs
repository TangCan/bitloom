//! Episode I teaching RV32 core (Story 15.2 / FR56).
//!
//! Subset: `ADDI`, `ADD`, `BEQ` over architectural `x0`/`x1`–`x4` (x0 hardwired 0).
//! No CSR/ECALL/EBREAK/FENCE/MMU/Linux/pipeline. `lw`/`sw` deferred to Story 15.3.
//! Instruction memory is the `instr` input (harness-presented); design deps: prelude only.

use bitloom_prelude::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// Single-cycle (edge-commit) teaching core.
pub struct EpisodeICore;

impl Elaboratable for EpisodeICore {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("EpisodeICore");
        s.begin_module("EpisodeICore", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("instr", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("pc_out", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("x1_out", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("x2_out", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("x3_out", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("x4_out", GroundType::UInt { width: 32 }, Span::default());

        s.declare_reg("pc", GroundType::UInt { width: 32 }, Span::default());
        s.declare_reg("x1", GroundType::UInt { width: 32 }, Span::default());
        s.declare_reg("x2", GroundType::UInt { width: 32 }, Span::default());
        s.declare_reg("x3", GroundType::UInt { width: 32 }, Span::default());
        s.declare_reg("x4", GroundType::UInt { width: 32 }, Span::default());

        for (n, w) in [
            ("c0", 32u32),
            ("c1", 32),
            ("c2", 32),
            ("c3", 32),
            ("c4", 32),
            ("c7", 32),
            ("c8", 32),
            ("c12", 32),
            ("c15", 32),
            ("c20", 32),
            ("c25", 32),
            ("mask7", 32),
            ("mask5", 32),
            ("mask3", 32),
            ("mask12", 32),
            ("op_addi", 32),
            ("op_op", 32),
            ("op_beq", 32),
            ("funct3_add", 32),
            ("funct7_add", 32),
        ] {
            s.declare_wire(n, GroundType::UInt { width: w }, Span::default());
        }

        for n in [
            "opcode",
            "rd",
            "funct3",
            "rs1",
            "rs2",
            "funct7",
            "imm_i",
            "sh7",
            "sh12",
            "sh15",
            "sh20",
            "sh25",
            "rs1_data",
            "rs2_data",
            "alu_add",
            "alu_addi",
            "wb_data",
            "pc_plus4",
            "branch_tgt",
            "next_pc",
            "next_x1",
            "next_x2",
            "next_x3",
            "next_x4",
            "t_rs1_1",
            "t_rs1_2",
            "t_rs1_3",
            "t_rs2_1",
            "t_rs2_2",
            "t_rs2_3",
            "is_add_pre",
        ] {
            s.declare_wire(n, GroundType::UInt { width: 32 }, Span::default());
        }

        for n in [
            "is_addi", "is_add", "is_beq", "is_op", "f3_add", "f7_add", "eq_rs", "take_br", "we",
            "we1", "we2", "we3", "we4", "rd1", "rd2", "rd3", "rd4", "rs1_1", "rs1_2", "rs1_3",
            "rs1_4", "rs2_1", "rs2_2", "rs2_3", "rs2_4",
        ] {
            s.declare_wire(n, GroundType::Bool, Span::default());
        }

        s.begin_combinational(Span::default());
        s.assign_lit("c0", 0, Span::default());
        s.assign_lit("c1", 1, Span::default());
        s.assign_lit("c2", 2, Span::default());
        s.assign_lit("c3", 3, Span::default());
        s.assign_lit("c4", 4, Span::default());
        s.assign_lit("c7", 7, Span::default());
        s.assign_lit("c8", 8, Span::default());
        s.assign_lit("c12", 12, Span::default());
        s.assign_lit("c15", 15, Span::default());
        s.assign_lit("c20", 20, Span::default());
        s.assign_lit("c25", 25, Span::default());
        s.assign_lit("mask7", 0x7f, Span::default());
        s.assign_lit("mask5", 0x1f, Span::default());
        s.assign_lit("mask3", 0x7, Span::default());
        s.assign_lit("mask12", 0xfff, Span::default());
        s.assign_lit("op_addi", 0b0010011, Span::default());
        s.assign_lit("op_op", 0b0110011, Span::default());
        s.assign_lit("op_beq", 0b1100011, Span::default());
        s.assign_lit("funct3_add", 0, Span::default());
        s.assign_lit("funct7_add", 0, Span::default());

        s.assign_and("opcode", "instr", "mask7", Span::default());
        s.assign_shr("sh7", "instr", "c7", Span::default());
        s.assign_and("rd", "sh7", "mask5", Span::default());
        s.assign_shr("sh12", "instr", "c12", Span::default());
        s.assign_and("funct3", "sh12", "mask3", Span::default());
        s.assign_shr("sh15", "instr", "c15", Span::default());
        s.assign_and("rs1", "sh15", "mask5", Span::default());
        s.assign_shr("sh20", "instr", "c20", Span::default());
        s.assign_and("rs2", "sh20", "mask5", Span::default());
        s.assign_and("imm_i", "sh20", "mask12", Span::default());
        s.assign_shr("sh25", "instr", "c25", Span::default());
        s.assign_and("funct7", "sh25", "mask7", Span::default());

        s.assign_eq("is_addi", "opcode", "op_addi", Span::default());
        s.assign_eq("is_op", "opcode", "op_op", Span::default());
        s.assign_eq("is_beq", "opcode", "op_beq", Span::default());
        s.assign_eq("f3_add", "funct3", "funct3_add", Span::default());
        s.assign_eq("f7_add", "funct7", "funct7_add", Span::default());
        s.assign_and("is_add_pre", "is_op", "f3_add", Span::default());
        s.assign_and("is_add", "is_add_pre", "f7_add", Span::default());

        s.assign_eq("rs1_1", "rs1", "c1", Span::default());
        s.assign_eq("rs1_2", "rs1", "c2", Span::default());
        s.assign_eq("rs1_3", "rs1", "c3", Span::default());
        s.assign_eq("rs1_4", "rs1", "c4", Span::default());
        s.assign_mux("t_rs1_1", "rs1_1", "x1", "c0", Span::default());
        s.assign_mux("t_rs1_2", "rs1_2", "x2", "t_rs1_1", Span::default());
        s.assign_mux("t_rs1_3", "rs1_3", "x3", "t_rs1_2", Span::default());
        s.assign_mux("rs1_data", "rs1_4", "x4", "t_rs1_3", Span::default());

        s.assign_eq("rs2_1", "rs2", "c1", Span::default());
        s.assign_eq("rs2_2", "rs2", "c2", Span::default());
        s.assign_eq("rs2_3", "rs2", "c3", Span::default());
        s.assign_eq("rs2_4", "rs2", "c4", Span::default());
        s.assign_mux("t_rs2_1", "rs2_1", "x1", "c0", Span::default());
        s.assign_mux("t_rs2_2", "rs2_2", "x2", "t_rs2_1", Span::default());
        s.assign_mux("t_rs2_3", "rs2_3", "x3", "t_rs2_2", Span::default());
        s.assign_mux("rs2_data", "rs2_4", "x4", "t_rs2_3", Span::default());

        s.assign_add("alu_add", "rs1_data", "rs2_data", Span::default());
        s.assign_add("alu_addi", "rs1_data", "imm_i", Span::default());
        s.assign_mux("wb_data", "is_addi", "alu_addi", "alu_add", Span::default());

        s.assign_or("we", "is_addi", "is_add", Span::default());
        s.assign_eq("rd1", "rd", "c1", Span::default());
        s.assign_eq("rd2", "rd", "c2", Span::default());
        s.assign_eq("rd3", "rd", "c3", Span::default());
        s.assign_eq("rd4", "rd", "c4", Span::default());
        s.assign_and("we1", "we", "rd1", Span::default());
        s.assign_and("we2", "we", "rd2", Span::default());
        s.assign_and("we3", "we", "rd3", Span::default());
        s.assign_and("we4", "we", "rd4", Span::default());
        s.assign_mux("next_x1", "we1", "wb_data", "x1", Span::default());
        s.assign_mux("next_x2", "we2", "wb_data", "x2", Span::default());
        s.assign_mux("next_x3", "we3", "wb_data", "x3", Span::default());
        s.assign_mux("next_x4", "we4", "wb_data", "x4", Span::default());

        // BEQ taken → PC+8 (teaching offset; full B-imm in later stories)
        s.assign_add("pc_plus4", "pc", "c4", Span::default());
        s.assign_add("branch_tgt", "pc", "c8", Span::default());
        s.assign_eq("eq_rs", "rs1_data", "rs2_data", Span::default());
        s.assign_and("take_br", "is_beq", "eq_rs", Span::default());
        s.assign_mux(
            "next_pc",
            "take_br",
            "branch_tgt",
            "pc_plus4",
            Span::default(),
        );

        s.assign_net("pc_out", "pc", Span::default());
        s.assign_net("x1_out", "x1", Span::default());
        s.assign_net("x2_out", "x2", Span::default());
        s.assign_net("x3_out", "x3", Span::default());
        s.assign_net("x4_out", "x4", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_from("pc", "next_pc", Span::default());
        s.assign_reg_d_from("x1", "next_x1", Span::default());
        s.assign_reg_d_from("x2", "next_x2", Span::default());
        s.assign_reg_d_from("x3", "next_x3", Span::default());
        s.assign_reg_d_from("x4", "next_x4", Span::default());
        s.end_process();

        s.end_module();
        s.finish()
    }
}

pub fn rhdl_elaborate() -> Result<FrozenHir, Diagnostics> {
    EpisodeICore::elaborate()
}

/// Encode `ADDI rd, rs1, imm` (imm zero-extended 12-bit; tests use non-negative).
pub fn enc_addi(rd: u32, rs1: u32, imm: u32) -> u64 {
    let imm = imm & 0xfff;
    u64::from((imm << 20) | (rs1 << 15) | (rd << 7) | 0b0010011)
}

/// Encode `ADD rd, rs1, rs2`.
pub fn enc_add(rd: u32, rs1: u32, rs2: u32) -> u64 {
    u64::from((rs2 << 20) | (rs1 << 15) | (rd << 7) | 0b0110011)
}

/// Encode `BEQ rs1, rs2` (Episode I teaching core jumps +8 when taken).
pub fn enc_beq_plus8(rs1: u32, rs2: u32) -> u64 {
    let imm11 = 0u32;
    let imm4_1 = 4u32;
    u64::from((imm11 << 7) | (imm4_1 << 8) | (rs1 << 15) | (rs2 << 20) | 0b1100011)
}

#[cfg(test)]
mod tests {
    use bitloom_hir::PortValues;
    use bitloom_prelude::Elaboratable;
    use bitloom_sim::Sim;

    use super::*;

    fn tick_instr(sim: &mut Sim, instr: u64) {
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        pv.set("instr", instr);
        sim.set_inputs(pv);
        sim.tick();
    }

    #[test]
    fn elaborate_ok() {
        let f = EpisodeICore::elaborate().unwrap();
        assert_eq!(f.abi_name, "EpisodeICore");
    }

    #[test]
    fn tick_addi_then_add_golden() {
        let mut sim = Sim::new(EpisodeICore::elaborate().unwrap());
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("instr", 0);
        sim.set_inputs(pv);
        sim.tick();

        // Each instr: tick to compute comb, tick to commit regs (or change instr on commit tick).
        tick_instr(&mut sim, enc_addi(1, 0, 5));
        tick_instr(&mut sim, enc_addi(1, 0, 5));
        assert_eq!(sim.ports().get("x1_out"), Some(5));

        tick_instr(&mut sim, enc_addi(2, 0, 7));
        tick_instr(&mut sim, enc_addi(2, 0, 7));
        assert_eq!(sim.ports().get("x2_out"), Some(7));

        tick_instr(&mut sim, enc_add(3, 1, 2));
        tick_instr(&mut sim, enc_add(3, 1, 2));
        assert_eq!(sim.ports().get("x3_out"), Some(12));
    }

    #[test]
    fn tick_beq_taken_jumps_plus8() {
        let mut sim = Sim::new(EpisodeICore::elaborate().unwrap());
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("instr", 0);
        sim.set_inputs(pv);
        sim.tick();

        tick_instr(&mut sim, enc_addi(1, 0, 1));
        tick_instr(&mut sim, enc_addi(1, 0, 1));
        tick_instr(&mut sim, enc_addi(2, 0, 1));
        tick_instr(&mut sim, enc_addi(2, 0, 1));
        // Drain prior ADDI PC+4 commit while arming BEQ comb.
        tick_instr(&mut sim, enc_beq_plus8(1, 2));
        let pc_before = sim.ports().get("pc_out").unwrap();
        tick_instr(&mut sim, enc_beq_plus8(1, 2));
        let pc_after = sim.ports().get("pc_out").unwrap();
        assert_eq!(pc_after, pc_before + 8);
    }
}
