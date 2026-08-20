//! Episode I teaching RV32 core (Stories 15.2–15.3 / FR56).
//!
//! Subset: `ADDI`, `ADD`, `BEQ`, `LW`, `SW` over `x0`/`x1`–`x4`.
//! DMEM(16)×32 + LED MMIO at byte address `0x100`. Harness presents `instr`.
//! No CSR/ECALL/EBREAK/FENCE/MMU/Linux/pipeline. Design deps: `bitloom-prelude` only.

use bitloom_prelude::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// Edge-commit teaching core with DMEM + LED MMIO.
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
        s.add_output("led_out", GroundType::UInt { width: 32 }, Span::default());

        s.declare_reg("pc", GroundType::UInt { width: 32 }, Span::default());
        s.declare_reg("x1", GroundType::UInt { width: 32 }, Span::default());
        s.declare_reg("x2", GroundType::UInt { width: 32 }, Span::default());
        s.declare_reg("x3", GroundType::UInt { width: 32 }, Span::default());
        s.declare_reg("x4", GroundType::UInt { width: 32 }, Span::default());
        s.declare_reg("led", GroundType::UInt { width: 32 }, Span::default());
        s.declare_reg("load_q", GroundType::UInt { width: 32 }, Span::default());
        s.declare_mem("dmem", 16, 32, Span::default());

        for (n, w) in [
            ("c0", 32u32),
            ("c1", 32),
            ("c2", 32),
            ("c3", 32),
            ("c4", 32),
            ("c5", 32),
            ("c7", 32),
            ("c8", 32),
            ("c12", 32),
            ("c15", 32),
            ("c20", 32),
            ("c25", 32),
            ("c11", 32),
            ("c31", 32),
            ("mask7", 32),
            ("mask5", 32),
            ("mask3", 32),
            ("mask4", 32),
            ("mask6", 32),
            ("mask12", 32),
            ("op_addi", 32),
            ("op_op", 32),
            ("op_beq", 32),
            ("op_load", 32),
            ("op_store", 32),
            ("funct3_add", 32),
            ("funct7_add", 32),
            ("mmio_led", 32),
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
            "wb_alu",
            "wb_data",
            "pc_plus4",
            "branch_tgt",
            "next_pc",
            "next_x1",
            "next_x2",
            "next_x3",
            "next_x4",
            "next_led",
            "next_led_mmio",
            "t_rs1_1",
            "t_rs1_2",
            "t_rs1_3",
            "t_rs2_1",
            "t_rs2_2",
            "t_rs2_3",
            "is_add_pre",
            "imm_off",
            "ea",
            "dmem_idx",
            "b_s31",
            "imm_b12",
            "imm_b12_s",
            "imm_b11",
            "imm_b11_s",
            "imm_b10_5",
            "imm_b10_5_s",
            "sh8",
            "imm_b4_1",
            "imm_b4_1_s",
            "imm_b_a",
            "imm_b_b",
            "imm_b",
            "imm_s_hi",
            "imm_s_lo",
            "imm_s_hi_s",
            "imm_s",
        ] {
            s.declare_wire(n, GroundType::UInt { width: 32 }, Span::default());
        }

        for n in [
            "is_addi", "is_add", "is_beq", "is_op", "is_lw", "is_sw", "f3_add", "f7_add", "eq_rs",
            "take_br", "we_alu", "we", "we1", "we2", "we3", "we4", "rd1", "rd2", "rd3", "rd4",
            "rs1_1", "rs1_2", "rs1_3", "rs1_4", "rs2_1", "rs2_2", "rs2_3", "rs2_4", "is_mmio",
        ] {
            s.declare_wire(n, GroundType::Bool, Span::default());
        }

        s.begin_combinational(Span::default());
        s.assign_lit("c0", 0, Span::default());
        s.assign_lit("c1", 1, Span::default());
        s.assign_lit("c2", 2, Span::default());
        s.assign_lit("c3", 3, Span::default());
        s.assign_lit("c4", 4, Span::default());
        s.assign_lit("c5", 5, Span::default());
        s.assign_lit("c7", 7, Span::default());
        s.assign_lit("c8", 8, Span::default());
        s.assign_lit("c12", 12, Span::default());
        s.assign_lit("c15", 15, Span::default());
        s.assign_lit("c20", 20, Span::default());
        s.assign_lit("c25", 25, Span::default());
        s.assign_lit("c11", 11, Span::default());
        s.assign_lit("c31", 31, Span::default());
        s.assign_lit("mask7", 0x7f, Span::default());
        s.assign_lit("mask5", 0x1f, Span::default());
        s.assign_lit("mask3", 0x7, Span::default());
        s.assign_lit("mask4", 0xf, Span::default());
        s.assign_lit("mask6", 0x3f, Span::default());
        s.assign_lit("mask12", 0xfff, Span::default());
        s.assign_lit("op_addi", 0b0010011, Span::default());
        s.assign_lit("op_op", 0b0110011, Span::default());
        s.assign_lit("op_beq", 0b1100011, Span::default());
        s.assign_lit("op_load", 0b0000011, Span::default());
        s.assign_lit("op_store", 0b0100011, Span::default());
        s.assign_lit("funct3_add", 0, Span::default());
        s.assign_lit("funct7_add", 0, Span::default());
        s.assign_lit("mmio_led", 0x100, Span::default());

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

        s.assign_and("imm_s_hi", "sh25", "mask7", Span::default());
        s.assign_and("imm_s_lo", "sh7", "mask5", Span::default());
        s.assign_shl("imm_s_hi_s", "imm_s_hi", "c5", Span::default());
        s.assign_or("imm_s", "imm_s_hi_s", "imm_s_lo", Span::default());

        s.assign_eq("is_addi", "opcode", "op_addi", Span::default());
        s.assign_eq("is_op", "opcode", "op_op", Span::default());
        s.assign_eq("is_beq", "opcode", "op_beq", Span::default());
        s.assign_eq("is_lw", "opcode", "op_load", Span::default());
        s.assign_eq("is_sw", "opcode", "op_store", Span::default());
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
        s.assign_mux("wb_alu", "is_addi", "alu_addi", "alu_add", Span::default());
        s.assign_mux("wb_data", "is_lw", "load_q", "wb_alu", Span::default());

        s.assign_or("we_alu", "is_addi", "is_add", Span::default());
        s.assign_or("we", "we_alu", "is_lw", Span::default());
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

        s.assign_mux("imm_off", "is_sw", "imm_s", "imm_i", Span::default());
        s.assign_add("ea", "rs1_data", "imm_off", Span::default());
        s.assign_and("dmem_idx", "ea", "mask4", Span::default());
        s.assign_eq("is_mmio", "ea", "mmio_led", Span::default());
        s.assign_mux(
            "next_led_mmio",
            "is_mmio",
            "rs2_data",
            "led",
            Span::default(),
        );
        s.assign_mux("next_led", "is_sw", "next_led_mmio", "led", Span::default());

        s.assign_add("pc_plus4", "pc", "c4", Span::default());
        // B-type imm (positive offsets): {instr[31],instr[7],instr[30:25],instr[11:8],0}
        s.assign_shr("b_s31", "instr", "c31", Span::default());
        s.assign_and("imm_b12", "b_s31", "c1", Span::default());
        s.assign_shl("imm_b12_s", "imm_b12", "c12", Span::default());
        s.assign_and("imm_b11", "sh7", "c1", Span::default());
        s.assign_shl("imm_b11_s", "imm_b11", "c11", Span::default());
        s.assign_and("imm_b10_5", "sh25", "mask6", Span::default());
        s.assign_shl("imm_b10_5_s", "imm_b10_5", "c5", Span::default());
        s.assign_shr("sh8", "instr", "c8", Span::default());
        s.assign_and("imm_b4_1", "sh8", "mask4", Span::default());
        s.assign_shl("imm_b4_1_s", "imm_b4_1", "c1", Span::default());
        s.assign_or("imm_b_a", "imm_b12_s", "imm_b11_s", Span::default());
        s.assign_or("imm_b_b", "imm_b_a", "imm_b10_5_s", Span::default());
        s.assign_or("imm_b", "imm_b_b", "imm_b4_1_s", Span::default());
        s.assign_add("branch_tgt", "pc", "imm_b", Span::default());
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
        s.assign_net("led_out", "led", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_from("pc", "next_pc", Span::default());
        s.assign_reg_d_from("x1", "next_x1", Span::default());
        s.assign_reg_d_from("x2", "next_x2", Span::default());
        s.assign_reg_d_from("x3", "next_x3", Span::default());
        s.assign_reg_d_from("x4", "next_x4", Span::default());
        s.assign_reg_d_from("led", "next_led", Span::default());
        s.assign_mem_write_en("dmem", "dmem_idx", "rs2_data", "is_sw", Span::default());
        s.assign_reg_d_mem_read("load_q", "dmem", "dmem_idx", Span::default());
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

/// Encode `SW rs2, imm(rs1)` (S-type).
pub fn enc_sw(rs1: u32, rs2: u32, imm: u32) -> u64 {
    let imm = imm & 0xfff;
    let imm11_5 = (imm >> 5) & 0x7f;
    let imm4_0 = imm & 0x1f;
    u64::from(
        (imm11_5 << 25) | (rs2 << 20) | (rs1 << 15) | (0b010 << 12) | (imm4_0 << 7) | 0b0100011,
    )
}

/// Encode `LW rd, imm(rs1)`.
pub fn enc_lw(rd: u32, rs1: u32, imm: u32) -> u64 {
    let imm = imm & 0xfff;
    u64::from((imm << 20) | (rs1 << 15) | (0b010 << 12) | (rd << 7) | 0b0000011)
}

#[cfg(test)]
mod tests {
    use bitloom_hir::PortValues;
    use bitloom_prelude::Elaboratable;
    use bitloom_sim::Sim;
    use bitloom_vlog::emit;

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
        tick_instr(&mut sim, enc_beq_plus8(1, 2));
        let pc_before = sim.ports().get("pc_out").unwrap();
        tick_instr(&mut sim, enc_beq_plus8(1, 2));
        let pc_after = sim.ports().get("pc_out").unwrap();
        assert_eq!(pc_after, pc_before + 8);
    }

    #[test]
    fn tick_sw_mmio_led_golden() {
        let mut sim = Sim::new(EpisodeICore::elaborate().unwrap());
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("instr", 0);
        sim.set_inputs(pv);
        sim.tick();

        // x1 = 0x100 (LED MMIO base)
        tick_instr(&mut sim, enc_addi(1, 0, 0x100));
        tick_instr(&mut sim, enc_addi(1, 0, 0x100));
        // x2 = 0xA5
        tick_instr(&mut sim, enc_addi(2, 0, 0xA5));
        tick_instr(&mut sim, enc_addi(2, 0, 0xA5));
        // SW x2, 0(x1)
        tick_instr(&mut sim, enc_sw(1, 2, 0));
        tick_instr(&mut sim, enc_sw(1, 2, 0));
        assert_eq!(sim.ports().get("led_out"), Some(0xA5));
    }

    #[test]
    fn emit_yosys_friendly_v() {
        let hir = EpisodeICore::elaborate().unwrap();
        let art = emit(&hir);
        assert!(!art.files.is_empty());
        let v = &art.files[0].contents;
        assert!(v.contains("module EpisodeICore"));
        assert!(v.contains("always @(posedge clk)"));
        assert!(!v.trim().is_empty());
    }

    /// Story 15.4 (a): minimal subset filter — not full DV (see COMPLIANCE.md).
    #[test]
    fn subset_minimal_filter_program() {
        let mut sim = Sim::new(EpisodeICore::elaborate().unwrap());
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("instr", 0);
        sim.set_inputs(pv);
        sim.tick();

        tick_instr(&mut sim, enc_addi(1, 0, 0x100));
        tick_instr(&mut sim, enc_addi(1, 0, 0x100));
        tick_instr(&mut sim, enc_addi(2, 0, 0x3C));
        tick_instr(&mut sim, enc_addi(2, 0, 0x3C));
        tick_instr(&mut sim, enc_sw(1, 2, 0));
        tick_instr(&mut sim, enc_sw(1, 2, 0));
        assert_eq!(sim.ports().get("led_out"), Some(0x3C));
        assert_eq!(sim.ports().get("x1_out"), Some(0x100));
    }
}
