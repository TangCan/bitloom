//! Episode I/II teaching RV32 core (Stories 15.2–15.3 / 17.3 / FR56 / FR63).
//!
//! Subset: `ADDI`, `ADD`, `BEQ`, `LW`, `SW` over `x0`/`x1`–`x4`.
//! DMEM(16)×32 + LED MMIO at byte address `0x100`. Harness presents `instr`.
//! Decode rebuilds signed I/S/B/U/J immediates (sign from instr bit31).
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
            ("c21", 32),
            ("c25", 32),
            ("c11", 32),
            ("c31", 32),
            ("mask7", 32),
            ("mask5", 32),
            ("mask3", 32),
            ("mask4", 32),
            ("mask6", 32),
            ("mask8", 32),
            ("mask10", 32),
            ("mask12", 32),
            ("mask32", 32),
            ("sext12", 32),
            ("sext13", 32),
            ("sext21", 32),
            ("mask_uimm", 32),
            ("op_addi", 32),
            ("op_op", 32),
            ("op_beq", 32),
            ("op_load", 32),
            ("op_store", 32),
            ("op_lui", 32),
            ("op_auipc", 32),
            ("op_jal", 32),
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
            "imm_i_raw",
            "imm_i_sext",
            "imm_i",
            "sh7",
            "sh8",
            "sh12",
            "sh15",
            "sh20",
            "sh21",
            "sh25",
            "rs1_data",
            "rs2_data",
            "alu_add_raw",
            "alu_add",
            "alu_addi_raw",
            "alu_addi",
            "wb_alu",
            "wb_data",
            "pc_plus4",
            "branch_tgt_raw",
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
            "ea_raw",
            "ea",
            "dmem_idx",
            "b_s31",
            "sign_bit",
            "imm_b12",
            "imm_b12_s",
            "imm_b11",
            "imm_b11_s",
            "imm_b10_5",
            "imm_b10_5_s",
            "imm_b4_1",
            "imm_b4_1_s",
            "imm_b_a",
            "imm_b_b",
            "imm_b_raw",
            "imm_b_sext",
            "imm_b",
            "imm_s_hi",
            "imm_s_lo",
            "imm_s_hi_s",
            "imm_s_raw",
            "imm_s_sext",
            "imm_s",
            "imm_u",
            "j_20",
            "j_20_s",
            "j_19_12",
            "j_19_12_s",
            "j_11",
            "j_11_s",
            "j_10_1",
            "j_10_1_s",
            "imm_j_a",
            "imm_j_b",
            "imm_j_raw",
            "imm_j_sext",
            "imm_j",
            "imm_t0",
            "imm_t1",
            "imm_t2",
            "imm",
        ] {
            s.declare_wire(n, GroundType::UInt { width: 32 }, Span::default());
        }

        for n in [
            "is_addi", "is_add", "is_beq", "is_op", "is_lw", "is_sw", "is_lui", "is_auipc",
            "is_jal", "is_u", "f3_add", "f7_add", "eq_rs", "take_br", "we_alu", "we", "we1", "we2",
            "we3", "we4", "rd1", "rd2", "rd3", "rd4", "rs1_1", "rs1_2", "rs1_3", "rs1_4", "rs2_1",
            "rs2_2", "rs2_3", "rs2_4", "is_mmio", "sign31",
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
        s.assign_lit("c21", 21, Span::default());
        s.assign_lit("c25", 25, Span::default());
        s.assign_lit("c11", 11, Span::default());
        s.assign_lit("c31", 31, Span::default());
        s.assign_lit("mask7", 0x7f, Span::default());
        s.assign_lit("mask5", 0x1f, Span::default());
        s.assign_lit("mask3", 0x7, Span::default());
        s.assign_lit("mask4", 0xf, Span::default());
        s.assign_lit("mask6", 0x3f, Span::default());
        s.assign_lit("mask8", 0xff, Span::default());
        s.assign_lit("mask10", 0x3ff, Span::default());
        s.assign_lit("mask12", 0xfff, Span::default());
        s.assign_lit("mask32", 0xffff_ffff, Span::default());
        s.assign_lit("sext12", 0xffff_f000, Span::default());
        s.assign_lit("sext13", 0xffff_e000, Span::default());
        s.assign_lit("sext21", 0xffe0_0000, Span::default());
        s.assign_lit("mask_uimm", 0xffff_f000, Span::default());
        s.assign_lit("op_addi", 0b0010011, Span::default());
        s.assign_lit("op_op", 0b0110011, Span::default());
        s.assign_lit("op_beq", 0b1100011, Span::default());
        s.assign_lit("op_load", 0b0000011, Span::default());
        s.assign_lit("op_store", 0b0100011, Span::default());
        s.assign_lit("op_lui", 0b0110111, Span::default());
        s.assign_lit("op_auipc", 0b0010111, Span::default());
        s.assign_lit("op_jal", 0b1101111, Span::default());
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
        s.assign_and("imm_i_raw", "sh20", "mask12", Span::default());
        s.assign_shr("sh25", "instr", "c25", Span::default());
        s.assign_and("funct7", "sh25", "mask7", Span::default());

        // Sign bit always instr[31] (shared by I/S/B/J).
        s.assign_shr("b_s31", "instr", "c31", Span::default());
        s.assign_and("sign_bit", "b_s31", "c1", Span::default());
        s.assign_eq("sign31", "sign_bit", "c1", Span::default());

        // I-imm: instr[31:20], sign-extend from bit11 (= instr[31]).
        s.assign_mux("imm_i_sext", "sign31", "sext12", "c0", Span::default());
        s.assign_or("imm_i", "imm_i_raw", "imm_i_sext", Span::default());

        // S-imm: {instr[31:25], instr[11:7]}, same 12-bit sext.
        s.assign_and("imm_s_hi", "sh25", "mask7", Span::default());
        s.assign_and("imm_s_lo", "sh7", "mask5", Span::default());
        s.assign_shl("imm_s_hi_s", "imm_s_hi", "c5", Span::default());
        s.assign_or("imm_s_raw", "imm_s_hi_s", "imm_s_lo", Span::default());
        s.assign_mux("imm_s_sext", "sign31", "sext12", "c0", Span::default());
        s.assign_or("imm_s", "imm_s_raw", "imm_s_sext", Span::default());

        // U-imm: {instr[31:12], 12'b0} — high bits already in place.
        s.assign_and("imm_u", "instr", "mask_uimm", Span::default());

        // B-imm: {instr[31], instr[7], instr[30:25], instr[11:8], 0} then sext from bit12.
        // Only shift the [4:1] nibble into bit positions — never <<1 the whole packed imm.
        s.assign_and("imm_b12", "sign_bit", "c1", Span::default());
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
        s.assign_or("imm_b_raw", "imm_b_b", "imm_b4_1_s", Span::default());
        s.assign_mux("imm_b_sext", "sign31", "sext13", "c0", Span::default());
        s.assign_or("imm_b", "imm_b_raw", "imm_b_sext", Span::default());

        // J-imm: {instr[31], instr[19:12], instr[20], instr[30:21], 0} then sext from bit20.
        s.assign_and("j_20", "sign_bit", "c1", Span::default());
        s.assign_shl("j_20_s", "j_20", "c20", Span::default());
        s.assign_and("j_19_12", "sh12", "mask8", Span::default());
        s.assign_shl("j_19_12_s", "j_19_12", "c12", Span::default());
        s.assign_and("j_11", "sh20", "c1", Span::default());
        s.assign_shl("j_11_s", "j_11", "c11", Span::default());
        s.assign_shr("sh21", "instr", "c21", Span::default());
        s.assign_and("j_10_1", "sh21", "mask10", Span::default());
        s.assign_shl("j_10_1_s", "j_10_1", "c1", Span::default());
        s.assign_or("imm_j_a", "j_20_s", "j_19_12_s", Span::default());
        s.assign_or("imm_j_b", "imm_j_a", "j_11_s", Span::default());
        s.assign_or("imm_j_raw", "imm_j_b", "j_10_1_s", Span::default());
        s.assign_mux("imm_j_sext", "sign31", "sext21", "c0", Span::default());
        s.assign_or("imm_j", "imm_j_raw", "imm_j_sext", Span::default());

        s.assign_eq("is_addi", "opcode", "op_addi", Span::default());
        s.assign_eq("is_op", "opcode", "op_op", Span::default());
        s.assign_eq("is_beq", "opcode", "op_beq", Span::default());
        s.assign_eq("is_lw", "opcode", "op_load", Span::default());
        s.assign_eq("is_sw", "opcode", "op_store", Span::default());
        s.assign_eq("is_lui", "opcode", "op_lui", Span::default());
        s.assign_eq("is_auipc", "opcode", "op_auipc", Span::default());
        s.assign_eq("is_jal", "opcode", "op_jal", Span::default());
        s.assign_or("is_u", "is_lui", "is_auipc", Span::default());
        s.assign_eq("f3_add", "funct3", "funct3_add", Span::default());
        s.assign_eq("f7_add", "funct7", "funct7_add", Span::default());
        s.assign_and("is_add_pre", "is_op", "f3_add", Span::default());
        s.assign_and("is_add", "is_add_pre", "f7_add", Span::default());

        // Unified imm bus by format (I default; S/B/U/J override).
        s.assign_mux("imm_t0", "is_sw", "imm_s", "imm_i", Span::default());
        s.assign_mux("imm_t1", "is_beq", "imm_b", "imm_t0", Span::default());
        s.assign_mux("imm_t2", "is_u", "imm_u", "imm_t1", Span::default());
        s.assign_mux("imm", "is_jal", "imm_j", "imm_t2", Span::default());

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

        // Mask add results to 32 bits (sim evaluates Add on u64 without wire-width truncate).
        s.assign_add("alu_add_raw", "rs1_data", "rs2_data", Span::default());
        s.assign_and("alu_add", "alu_add_raw", "mask32", Span::default());
        s.assign_add("alu_addi_raw", "rs1_data", "imm_i", Span::default());
        s.assign_and("alu_addi", "alu_addi_raw", "mask32", Span::default());
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
        s.assign_add("ea_raw", "rs1_data", "imm_off", Span::default());
        s.assign_and("ea", "ea_raw", "mask32", Span::default());
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
        s.assign_add("branch_tgt_raw", "pc", "imm_b", Span::default());
        s.assign_and("branch_tgt", "branch_tgt_raw", "mask32", Span::default());
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

/// Encode `ADDI rd, rs1, imm` (12-bit signed imm, hardware sign-extends).
pub fn enc_addi(rd: u32, rs1: u32, imm: i32) -> u64 {
    let imm = (imm as u32) & 0xfff;
    u64::from((imm << 20) | (rs1 << 15) | (rd << 7) | 0b0010011)
}

/// Encode `ADD rd, rs1, rs2`.
pub fn enc_add(rd: u32, rs1: u32, rs2: u32) -> u64 {
    u64::from((rs2 << 20) | (rs1 << 15) | (rd << 7) | 0b0110011)
}

/// Encode `BEQ rs1, rs2, offset` (byte offset, even; B-imm scrambled fields).
pub fn enc_beq(rs1: u32, rs2: u32, offset: i32) -> u64 {
    let imm = offset as u32;
    let b12 = (imm >> 12) & 1;
    let b11 = (imm >> 11) & 1;
    let b10_5 = (imm >> 5) & 0x3f;
    let b4_1 = (imm >> 1) & 0xf;
    u64::from(
        (b12 << 31)
            | (b10_5 << 25)
            | (rs2 << 20)
            | (rs1 << 15)
            | (b4_1 << 8)
            | (b11 << 7)
            | 0b1100011,
    )
}

/// Encode `BEQ` with teaching +8 offset (compat wrapper).
pub fn enc_beq_plus8(rs1: u32, rs2: u32) -> u64 {
    enc_beq(rs1, rs2, 8)
}

/// Encode `SW rs2, imm(rs1)` (S-type, 12-bit signed imm).
pub fn enc_sw(rs1: u32, rs2: u32, imm: i32) -> u64 {
    let imm = (imm as u32) & 0xfff;
    let imm11_5 = (imm >> 5) & 0x7f;
    let imm4_0 = imm & 0x1f;
    u64::from(
        (imm11_5 << 25) | (rs2 << 20) | (rs1 << 15) | (0b010 << 12) | (imm4_0 << 7) | 0b0100011,
    )
}

/// Encode `LW rd, imm(rs1)` (I-type load, 12-bit signed imm).
pub fn enc_lw(rd: u32, rs1: u32, imm: i32) -> u64 {
    let imm = (imm as u32) & 0xfff;
    u64::from((imm << 20) | (rs1 << 15) | (0b010 << 12) | (rd << 7) | 0b0000011)
}

/// Software mirror of decode imm rebuild (for U/J contract tests).
pub fn reconstruct_imm_i(instr: u32) -> u32 {
    let raw = (instr >> 20) & 0xfff;
    if (instr >> 31) & 1 == 1 {
        raw | 0xffff_f000
    } else {
        raw
    }
}

pub fn reconstruct_imm_s(instr: u32) -> u32 {
    let raw = (((instr >> 25) & 0x7f) << 5) | ((instr >> 7) & 0x1f);
    if (instr >> 31) & 1 == 1 {
        raw | 0xffff_f000
    } else {
        raw
    }
}

pub fn reconstruct_imm_b(instr: u32) -> u32 {
    let raw = (((instr >> 31) & 1) << 12)
        | (((instr >> 7) & 1) << 11)
        | (((instr >> 25) & 0x3f) << 5)
        | (((instr >> 8) & 0xf) << 1);
    if (instr >> 31) & 1 == 1 {
        raw | 0xffff_e000
    } else {
        raw
    }
}

pub fn reconstruct_imm_u(instr: u32) -> u32 {
    instr & 0xffff_f000
}

pub fn reconstruct_imm_j(instr: u32) -> u32 {
    let raw = (((instr >> 31) & 1) << 20)
        | (((instr >> 12) & 0xff) << 12)
        | (((instr >> 20) & 1) << 11)
        | (((instr >> 21) & 0x3ff) << 1);
    if (instr >> 31) & 1 == 1 {
        raw | 0xffe0_0000
    } else {
        raw
    }
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

    fn reset_sim() -> Sim {
        let mut sim = Sim::new(EpisodeICore::elaborate().unwrap());
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("instr", 0);
        sim.set_inputs(pv);
        sim.tick();
        sim
    }

    #[test]
    fn elaborate_ok() {
        let f = EpisodeICore::elaborate().unwrap();
        assert_eq!(f.abi_name, "EpisodeICore");
    }

    #[test]
    fn tick_addi_then_add_golden() {
        let mut sim = reset_sim();

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
    fn tick_addi_negative_imm_golden() {
        let mut sim = reset_sim();

        tick_instr(&mut sim, enc_addi(1, 0, -1));
        tick_instr(&mut sim, enc_addi(1, 0, -1));
        assert_eq!(sim.ports().get("x1_out"), Some(0xffff_ffff));

        tick_instr(&mut sim, enc_addi(2, 0, 5));
        tick_instr(&mut sim, enc_addi(2, 0, 5));
        tick_instr(&mut sim, enc_addi(2, 2, -2));
        tick_instr(&mut sim, enc_addi(2, 2, -2));
        assert_eq!(sim.ports().get("x2_out"), Some(3));
    }

    #[test]
    fn tick_beq_taken_jumps_plus8() {
        let mut sim = reset_sim();

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
    fn tick_beq_taken_jumps_minus8() {
        let mut sim = reset_sim();

        tick_instr(&mut sim, enc_addi(1, 0, 1));
        tick_instr(&mut sim, enc_addi(1, 0, 1));
        tick_instr(&mut sim, enc_addi(2, 0, 1));
        tick_instr(&mut sim, enc_addi(2, 0, 1));
        // PC is 16 after four ADDI ticks; one more BEQ tick commits PC+=-8.
        tick_instr(&mut sim, enc_beq(1, 2, -8));
        let pc_before = sim.ports().get("pc_out").unwrap();
        tick_instr(&mut sim, enc_beq(1, 2, -8));
        let pc_after = sim.ports().get("pc_out").unwrap();
        assert_eq!(pc_after, pc_before.wrapping_sub(8));
        assert!(pc_after < pc_before);
    }

    #[test]
    fn tick_sw_mmio_led_golden() {
        let mut sim = reset_sim();

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
    fn tick_sw_negative_imm_mmio_golden() {
        let mut sim = reset_sim();

        // x1 = 0x108; SW with imm=-8 → ea=0x100 LED
        tick_instr(&mut sim, enc_addi(1, 0, 0x108));
        tick_instr(&mut sim, enc_addi(1, 0, 0x108));
        tick_instr(&mut sim, enc_addi(2, 0, 0x5A));
        tick_instr(&mut sim, enc_addi(2, 0, 0x5A));
        tick_instr(&mut sim, enc_sw(1, 2, -8));
        tick_instr(&mut sim, enc_sw(1, 2, -8));
        assert_eq!(sim.ports().get("led_out"), Some(0x5A));
    }

    #[test]
    fn imm_reconstruct_i_s_b_u_j_contract() {
        // I: ADDI x0,x0,-1
        let addi_m1 = enc_addi(0, 0, -1) as u32;
        assert_eq!(reconstruct_imm_i(addi_m1), 0xffff_ffff);

        // S: SW offset -8
        let sw_m8 = enc_sw(0, 0, -8) as u32;
        assert_eq!(reconstruct_imm_s(sw_m8), 0xffff_fff8);

        // B: +8 and -8
        let beq_p8 = enc_beq(1, 2, 8) as u32;
        let beq_m8 = enc_beq(1, 2, -8) as u32;
        assert_eq!(reconstruct_imm_b(beq_p8), 8);
        assert_eq!(reconstruct_imm_b(beq_m8), 0xffff_fff8);

        // U: LUI-shaped word with upper imm 0xABCDE000
        let lui = 0xabcde000u32 | 0b0110111;
        assert_eq!(reconstruct_imm_u(lui), 0xabcde000);

        // J: negative offset -8 → scrambled fields then sext
        // offset -8 = 0x1FF_FF8 in 21-bit; build instr manually
        let j_off = (-8i32) as u32;
        let j20 = (j_off >> 20) & 1;
        let j19_12 = (j_off >> 12) & 0xff;
        let j11 = (j_off >> 11) & 1;
        let j10_1 = (j_off >> 1) & 0x3ff;
        let jal = (j20 << 31) | (j19_12 << 12) | (j11 << 20) | (j10_1 << 21) | 0b1101111;
        assert_eq!(reconstruct_imm_j(jal), 0xffff_fff8);
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
        let mut sim = reset_sim();

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
