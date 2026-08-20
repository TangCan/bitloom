//! Episode II 五级流水教学核（Story 17.4 / FR64）。
//!
//! IF/ID/EX/MEM/WB + EX/MEM→EX 与 MEM/WB→EX 转发 + predict-not-taken 分支 flush。
//! 取指合同 (b) harness `instr`；无 load-use stall；无 CSR。
//! 设计依赖仅 `bitloom-prelude`。公开品牌 Bitloom；与 `samitbasu/rhdl` 无关。

use bitloom_prelude::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// 经典五级流水 + ALU 转发 + 分支冲刷。
pub struct EpisodeIIPipe;

impl Elaboratable for EpisodeIIPipe {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("EpisodeIIPipe");
        s.begin_module("EpisodeIIPipe", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("instr", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("pc_out", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("x1_out", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("x2_out", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("x3_out", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("x4_out", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("led_out", GroundType::UInt { width: 32 }, Span::default());

        for n in ["pc", "pc_f", "x1", "x2", "x3", "x4", "led", "load_q"] {
            s.declare_reg(n, GroundType::UInt { width: 32 }, Span::default());
        }
        s.declare_mem("dmem", 16, 32, Span::default());

        for n in ["if_id_pc", "if_id_instr"] {
            s.declare_reg(n, GroundType::UInt { width: 32 }, Span::default());
        }
        for n in [
            "id_ex_pc",
            "id_ex_rs1_data",
            "id_ex_rs2_data",
            "id_ex_imm",
            "id_ex_rd",
            "id_ex_rs1",
            "id_ex_rs2",
        ] {
            s.declare_reg(n, GroundType::UInt { width: 32 }, Span::default());
        }
        for n in [
            "id_ex_is_addi",
            "id_ex_is_add",
            "id_ex_is_beq",
            "id_ex_is_lw",
            "id_ex_is_sw",
        ] {
            s.declare_reg(n, GroundType::Bool, Span::default());
        }
        for n in [
            "ex_mem_alu",
            "ex_mem_rs2_data",
            "ex_mem_rd",
            "ex_mem_ea",
            "ex_mem_dmem_idx",
        ] {
            s.declare_reg(n, GroundType::UInt { width: 32 }, Span::default());
        }
        for n in ["ex_mem_we", "ex_mem_is_lw", "ex_mem_is_sw"] {
            s.declare_reg(n, GroundType::Bool, Span::default());
        }
        for n in ["mem_wb_data", "mem_wb_rd"] {
            s.declare_reg(n, GroundType::UInt { width: 32 }, Span::default());
        }
        s.declare_reg("mem_wb_we", GroundType::Bool, Span::default());

        for (n, w) in [
            ("c0", 32u32),
            ("c1", 32),
            ("c2", 32),
            ("c3", 32),
            ("c4", 32),
            ("c5", 32),
            ("c7", 32),
            ("c8", 32),
            ("c11", 32),
            ("c12", 32),
            ("c15", 32),
            ("c20", 32),
            ("c21", 32),
            ("c25", 32),
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
            "is_add_pre",
            "rs1_data",
            "rs2_data",
            "t_rs1_1",
            "t_rs1_2",
            "t_rs1_3",
            "t_rs2_1",
            "t_rs2_2",
            "t_rs2_3",
            "pc_plus4_raw",
            "pc_plus4",
            "branch_tgt_raw",
            "branch_tgt",
            "next_pc_br",
            "next_pc",
            "if_id_pc_n",
            "if_id_instr_n",
            "id_ex_pc_n",
            "id_ex_rs1_data_n",
            "id_ex_rs2_data_n",
            "id_ex_imm_n",
            "id_ex_rd_n",
            "id_ex_rs1_n",
            "id_ex_rs2_n",
            "fwd_rs1_mw",
            "fwd_rs1",
            "fwd_rs2_mw",
            "fwd_rs2",
            "alu_add_raw",
            "alu_add",
            "alu_addi_raw",
            "alu_addi",
            "ex_alu",
            "imm_off",
            "ea_raw",
            "ea",
            "dmem_idx",
            "ex_mem_alu_n",
            "ex_mem_rs2_data_n",
            "ex_mem_rd_n",
            "ex_mem_ea_n",
            "ex_mem_dmem_idx_n",
            "mem_wb_data_n",
            "mem_wb_rd_n",
            "wb_data",
            "next_x1",
            "next_x2",
            "next_x3",
            "next_x4",
            "next_led",
            "next_led_mmio",
        ] {
            s.declare_wire(n, GroundType::UInt { width: 32 }, Span::default());
        }

        for n in [
            "sign31",
            "is_addi",
            "is_add",
            "is_beq",
            "is_op",
            "is_lw",
            "is_sw",
            "is_lui",
            "is_auipc",
            "is_jal",
            "is_u",
            "f3_add",
            "f7_add",
            "rs1_1",
            "rs1_2",
            "rs1_3",
            "rs1_4",
            "rs2_1",
            "rs2_2",
            "rs2_3",
            "rs2_4",
            "bfalse",
            "do_flush",
            "id_ex_is_addi_n",
            "id_ex_is_add_n",
            "id_ex_is_beq_n",
            "id_ex_is_lw_n",
            "id_ex_is_sw_n",
            "em_rd1",
            "em_rd2",
            "em_rd3",
            "em_rd4",
            "em_rd_nz",
            "em_rd_nz_a",
            "em_rd_nz_b",
            "mw_rd1",
            "mw_rd2",
            "mw_rd3",
            "mw_rd4",
            "mw_rd_nz",
            "mw_rd_nz_a",
            "mw_rd_nz_b",
            "em_rs1_match",
            "em_rs2_match",
            "mw_rs1_match",
            "mw_rs2_match",
            "fwd_em_rs1_a",
            "fwd_em_rs1_b",
            "fwd_em_rs1",
            "fwd_em_rs2_a",
            "fwd_em_rs2_b",
            "fwd_em_rs2",
            "fwd_mw_rs1_a",
            "fwd_mw_rs1",
            "fwd_mw_rs2_a",
            "fwd_mw_rs2",
            "em_not_lw",
            "eq_rs",
            "take_br",
            "we_alu_ex",
            "we_ex",
            "ex_mem_we_n",
            "ex_mem_is_lw_n",
            "ex_mem_is_sw_n",
            "mem_wb_we_n",
            "is_mmio",
            "rd1",
            "rd2",
            "rd3",
            "rd4",
            "we1",
            "we2",
            "we3",
            "we4",
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
        s.assign_lit("c11", 11, Span::default());
        s.assign_lit("c12", 12, Span::default());
        s.assign_lit("c15", 15, Span::default());
        s.assign_lit("c20", 20, Span::default());
        s.assign_lit("c21", 21, Span::default());
        s.assign_lit("c25", 25, Span::default());
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
        s.assign_eq("bfalse", "c0", "c1", Span::default());

        // ID decode from IF/ID.instr
        s.assign_and("opcode", "if_id_instr", "mask7", Span::default());
        s.assign_shr("sh7", "if_id_instr", "c7", Span::default());
        s.assign_and("rd", "sh7", "mask5", Span::default());
        s.assign_shr("sh12", "if_id_instr", "c12", Span::default());
        s.assign_and("funct3", "sh12", "mask3", Span::default());
        s.assign_shr("sh15", "if_id_instr", "c15", Span::default());
        s.assign_and("rs1", "sh15", "mask5", Span::default());
        s.assign_shr("sh20", "if_id_instr", "c20", Span::default());
        s.assign_and("rs2", "sh20", "mask5", Span::default());
        s.assign_and("imm_i_raw", "sh20", "mask12", Span::default());
        s.assign_shr("sh25", "if_id_instr", "c25", Span::default());
        s.assign_and("funct7", "sh25", "mask7", Span::default());

        s.assign_shr("b_s31", "if_id_instr", "c31", Span::default());
        s.assign_and("sign_bit", "b_s31", "c1", Span::default());
        s.assign_eq("sign31", "sign_bit", "c1", Span::default());

        s.assign_mux("imm_i_sext", "sign31", "sext12", "c0", Span::default());
        s.assign_or("imm_i", "imm_i_raw", "imm_i_sext", Span::default());

        s.assign_and("imm_s_hi", "sh25", "mask7", Span::default());
        s.assign_and("imm_s_lo", "sh7", "mask5", Span::default());
        s.assign_shl("imm_s_hi_s", "imm_s_hi", "c5", Span::default());
        s.assign_or("imm_s_raw", "imm_s_hi_s", "imm_s_lo", Span::default());
        s.assign_mux("imm_s_sext", "sign31", "sext12", "c0", Span::default());
        s.assign_or("imm_s", "imm_s_raw", "imm_s_sext", Span::default());

        s.assign_and("imm_u", "if_id_instr", "mask_uimm", Span::default());

        s.assign_and("imm_b12", "sign_bit", "c1", Span::default());
        s.assign_shl("imm_b12_s", "imm_b12", "c12", Span::default());
        s.assign_and("imm_b11", "sh7", "c1", Span::default());
        s.assign_shl("imm_b11_s", "imm_b11", "c11", Span::default());
        s.assign_and("imm_b10_5", "sh25", "mask6", Span::default());
        s.assign_shl("imm_b10_5_s", "imm_b10_5", "c5", Span::default());
        s.assign_shr("sh8", "if_id_instr", "c8", Span::default());
        s.assign_and("imm_b4_1", "sh8", "mask4", Span::default());
        s.assign_shl("imm_b4_1_s", "imm_b4_1", "c1", Span::default());
        s.assign_or("imm_b_a", "imm_b12_s", "imm_b11_s", Span::default());
        s.assign_or("imm_b_b", "imm_b_a", "imm_b10_5_s", Span::default());
        s.assign_or("imm_b_raw", "imm_b_b", "imm_b4_1_s", Span::default());
        s.assign_mux("imm_b_sext", "sign31", "sext13", "c0", Span::default());
        s.assign_or("imm_b", "imm_b_raw", "imm_b_sext", Span::default());

        s.assign_and("j_20", "sign_bit", "c1", Span::default());
        s.assign_shl("j_20_s", "j_20", "c20", Span::default());
        s.assign_and("j_19_12", "sh12", "mask8", Span::default());
        s.assign_shl("j_19_12_s", "j_19_12", "c12", Span::default());
        s.assign_and("j_11", "sh20", "c1", Span::default());
        s.assign_shl("j_11_s", "j_11", "c11", Span::default());
        s.assign_shr("sh21", "if_id_instr", "c21", Span::default());
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

        // EX forwarding: MEM/WB then EX/MEM (outer = higher priority)
        s.assign_eq("em_rd1", "ex_mem_rd", "c1", Span::default());
        s.assign_eq("em_rd2", "ex_mem_rd", "c2", Span::default());
        s.assign_eq("em_rd3", "ex_mem_rd", "c3", Span::default());
        s.assign_eq("em_rd4", "ex_mem_rd", "c4", Span::default());
        s.assign_or("em_rd_nz_a", "em_rd1", "em_rd2", Span::default());
        s.assign_or("em_rd_nz_b", "em_rd3", "em_rd4", Span::default());
        s.assign_or("em_rd_nz", "em_rd_nz_a", "em_rd_nz_b", Span::default());
        s.assign_eq("mw_rd1", "mem_wb_rd", "c1", Span::default());
        s.assign_eq("mw_rd2", "mem_wb_rd", "c2", Span::default());
        s.assign_eq("mw_rd3", "mem_wb_rd", "c3", Span::default());
        s.assign_eq("mw_rd4", "mem_wb_rd", "c4", Span::default());
        s.assign_or("mw_rd_nz_a", "mw_rd1", "mw_rd2", Span::default());
        s.assign_or("mw_rd_nz_b", "mw_rd3", "mw_rd4", Span::default());
        s.assign_or("mw_rd_nz", "mw_rd_nz_a", "mw_rd_nz_b", Span::default());

        s.assign_eq("em_rs1_match", "ex_mem_rd", "id_ex_rs1", Span::default());
        s.assign_eq("em_rs2_match", "ex_mem_rd", "id_ex_rs2", Span::default());
        s.assign_eq("mw_rs1_match", "mem_wb_rd", "id_ex_rs1", Span::default());
        s.assign_eq("mw_rs2_match", "mem_wb_rd", "id_ex_rs2", Span::default());

        s.assign_and("fwd_em_rs1_a", "ex_mem_we", "em_rs1_match", Span::default());
        s.assign_and("fwd_em_rs1_b", "fwd_em_rs1_a", "em_rd_nz", Span::default());
        // em_not_lw := (ex_mem_is_lw == false) — do not forward EA as if it were load data.
        s.assign_eq("em_not_lw", "ex_mem_is_lw", "bfalse", Span::default());
        s.assign_and("fwd_em_rs1", "fwd_em_rs1_b", "em_not_lw", Span::default());
        s.assign_and("fwd_em_rs2_a", "ex_mem_we", "em_rs2_match", Span::default());
        s.assign_and("fwd_em_rs2_b", "fwd_em_rs2_a", "em_rd_nz", Span::default());
        s.assign_and("fwd_em_rs2", "fwd_em_rs2_b", "em_not_lw", Span::default());
        s.assign_and("fwd_mw_rs1_a", "mem_wb_we", "mw_rs1_match", Span::default());
        s.assign_and("fwd_mw_rs1", "fwd_mw_rs1_a", "mw_rd_nz", Span::default());
        s.assign_and("fwd_mw_rs2_a", "mem_wb_we", "mw_rs2_match", Span::default());
        s.assign_and("fwd_mw_rs2", "fwd_mw_rs2_a", "mw_rd_nz", Span::default());

        s.assign_mux(
            "fwd_rs1_mw",
            "fwd_mw_rs1",
            "mem_wb_data",
            "id_ex_rs1_data",
            Span::default(),
        );
        s.assign_mux(
            "fwd_rs1",
            "fwd_em_rs1",
            "ex_mem_alu",
            "fwd_rs1_mw",
            Span::default(),
        );
        s.assign_mux(
            "fwd_rs2_mw",
            "fwd_mw_rs2",
            "mem_wb_data",
            "id_ex_rs2_data",
            Span::default(),
        );
        s.assign_mux(
            "fwd_rs2",
            "fwd_em_rs2",
            "ex_mem_alu",
            "fwd_rs2_mw",
            Span::default(),
        );

        s.assign_add("alu_add_raw", "fwd_rs1", "fwd_rs2", Span::default());
        s.assign_and("alu_add", "alu_add_raw", "mask32", Span::default());
        s.assign_add("alu_addi_raw", "fwd_rs1", "id_ex_imm", Span::default());
        s.assign_and("alu_addi", "alu_addi_raw", "mask32", Span::default());
        s.assign_mux(
            "ex_alu",
            "id_ex_is_addi",
            "alu_addi",
            "alu_add",
            Span::default(),
        );

        s.assign_mux(
            "imm_off",
            "id_ex_is_sw",
            "id_ex_imm",
            "id_ex_imm",
            Span::default(),
        );
        s.assign_add("ea_raw", "fwd_rs1", "imm_off", Span::default());
        s.assign_and("ea", "ea_raw", "mask32", Span::default());
        s.assign_and("dmem_idx", "ea", "mask4", Span::default());

        s.assign_or(
            "we_alu_ex",
            "id_ex_is_addi",
            "id_ex_is_add",
            Span::default(),
        );
        s.assign_or("we_ex", "we_alu_ex", "id_ex_is_lw", Span::default());

        s.assign_eq("eq_rs", "fwd_rs1", "fwd_rs2", Span::default());
        s.assign_and("take_br", "id_ex_is_beq", "eq_rs", Span::default());
        s.assign_net("do_flush", "take_br", Span::default());

        s.assign_add("branch_tgt_raw", "id_ex_pc", "id_ex_imm", Span::default());
        s.assign_and("branch_tgt", "branch_tgt_raw", "mask32", Span::default());
        s.assign_add("pc_plus4_raw", "pc", "c4", Span::default());
        s.assign_and("pc_plus4", "pc_plus4_raw", "mask32", Span::default());
        s.assign_mux(
            "next_pc_br",
            "take_br",
            "branch_tgt",
            "pc_plus4",
            Span::default(),
        );
        // Hold PC at 0 while rst is observed in comb (helps post-reset arming).
        s.assign_mux("next_pc", "rst", "c0", "next_pc_br", Span::default());

        // IF/ID next (bubble on flush)
        s.assign_mux("if_id_pc_n", "do_flush", "c0", "pc_f", Span::default());
        s.assign_mux("if_id_instr_n", "do_flush", "c0", "instr", Span::default());

        // ID/EX next (bubble on flush)
        s.assign_mux("id_ex_pc_n", "do_flush", "c0", "if_id_pc", Span::default());
        s.assign_mux(
            "id_ex_rs1_data_n",
            "do_flush",
            "c0",
            "rs1_data",
            Span::default(),
        );
        s.assign_mux(
            "id_ex_rs2_data_n",
            "do_flush",
            "c0",
            "rs2_data",
            Span::default(),
        );
        s.assign_mux("id_ex_imm_n", "do_flush", "c0", "imm", Span::default());
        s.assign_mux("id_ex_rd_n", "do_flush", "c0", "rd", Span::default());
        s.assign_mux("id_ex_rs1_n", "do_flush", "c0", "rs1", Span::default());
        s.assign_mux("id_ex_rs2_n", "do_flush", "c0", "rs2", Span::default());
        s.assign_mux(
            "id_ex_is_addi_n",
            "do_flush",
            "bfalse",
            "is_addi",
            Span::default(),
        );
        s.assign_mux(
            "id_ex_is_add_n",
            "do_flush",
            "bfalse",
            "is_add",
            Span::default(),
        );
        s.assign_mux(
            "id_ex_is_beq_n",
            "do_flush",
            "bfalse",
            "is_beq",
            Span::default(),
        );
        s.assign_mux(
            "id_ex_is_lw_n",
            "do_flush",
            "bfalse",
            "is_lw",
            Span::default(),
        );
        s.assign_mux(
            "id_ex_is_sw_n",
            "do_flush",
            "bfalse",
            "is_sw",
            Span::default(),
        );

        // EX/MEM next
        s.assign_net("ex_mem_alu_n", "ex_alu", Span::default());
        s.assign_net("ex_mem_rs2_data_n", "fwd_rs2", Span::default());
        s.assign_net("ex_mem_rd_n", "id_ex_rd", Span::default());
        s.assign_net("ex_mem_ea_n", "ea", Span::default());
        s.assign_net("ex_mem_dmem_idx_n", "dmem_idx", Span::default());
        s.assign_net("ex_mem_we_n", "we_ex", Span::default());
        s.assign_net("ex_mem_is_lw_n", "id_ex_is_lw", Span::default());
        s.assign_net("ex_mem_is_sw_n", "id_ex_is_sw", Span::default());

        // MEM/WB next (LW uses load_q from prior MEM read — Episode I style; golden 17.4 不测 load)
        s.assign_mux(
            "mem_wb_data_n",
            "ex_mem_is_lw",
            "load_q",
            "ex_mem_alu",
            Span::default(),
        );
        s.assign_net("mem_wb_rd_n", "ex_mem_rd", Span::default());
        s.assign_net("mem_wb_we_n", "ex_mem_we", Span::default());

        // WB → RF
        s.assign_net("wb_data", "mem_wb_data", Span::default());
        s.assign_eq("rd1", "mem_wb_rd", "c1", Span::default());
        s.assign_eq("rd2", "mem_wb_rd", "c2", Span::default());
        s.assign_eq("rd3", "mem_wb_rd", "c3", Span::default());
        s.assign_eq("rd4", "mem_wb_rd", "c4", Span::default());
        s.assign_and("we1", "mem_wb_we", "rd1", Span::default());
        s.assign_and("we2", "mem_wb_we", "rd2", Span::default());
        s.assign_and("we3", "mem_wb_we", "rd3", Span::default());
        s.assign_and("we4", "mem_wb_we", "rd4", Span::default());
        s.assign_mux("next_x1", "we1", "wb_data", "x1", Span::default());
        s.assign_mux("next_x2", "we2", "wb_data", "x2", Span::default());
        s.assign_mux("next_x3", "we3", "wb_data", "x3", Span::default());
        s.assign_mux("next_x4", "we4", "wb_data", "x4", Span::default());

        s.assign_eq("is_mmio", "ex_mem_ea", "mmio_led", Span::default());
        s.assign_mux(
            "next_led_mmio",
            "is_mmio",
            "ex_mem_rs2_data",
            "led",
            Span::default(),
        );
        s.assign_mux(
            "next_led",
            "ex_mem_is_sw",
            "next_led_mmio",
            "led",
            Span::default(),
        );

        s.assign_net("pc_out", "pc", Span::default());
        s.assign_net("x1_out", "x1", Span::default());
        s.assign_net("x2_out", "x2", Span::default());
        s.assign_net("x3_out", "x3", Span::default());
        s.assign_net("x4_out", "x4", Span::default());
        s.assign_net("led_out", "led", Span::default());
        s.end_process();

        // Seq: 下游 Reg 先于上游（WB ← MEM ← EX ← ID ← IF ← pc_f ← PC）
        s.begin_sequential(Span::default());
        s.assign_reg_d_from("x1", "next_x1", Span::default());
        s.assign_reg_d_from("x2", "next_x2", Span::default());
        s.assign_reg_d_from("x3", "next_x3", Span::default());
        s.assign_reg_d_from("x4", "next_x4", Span::default());
        s.assign_reg_d_from("led", "next_led", Span::default());
        s.assign_mem_write_en(
            "dmem",
            "ex_mem_dmem_idx",
            "ex_mem_rs2_data",
            "ex_mem_is_sw",
            Span::default(),
        );
        s.assign_reg_d_mem_read("load_q", "dmem", "ex_mem_dmem_idx", Span::default());

        s.assign_reg_d_from("mem_wb_data", "mem_wb_data_n", Span::default());
        s.assign_reg_d_from("mem_wb_rd", "mem_wb_rd_n", Span::default());
        s.assign_reg_d_from("mem_wb_we", "mem_wb_we_n", Span::default());

        s.assign_reg_d_from("ex_mem_alu", "ex_mem_alu_n", Span::default());
        s.assign_reg_d_from("ex_mem_rs2_data", "ex_mem_rs2_data_n", Span::default());
        s.assign_reg_d_from("ex_mem_rd", "ex_mem_rd_n", Span::default());
        s.assign_reg_d_from("ex_mem_ea", "ex_mem_ea_n", Span::default());
        s.assign_reg_d_from("ex_mem_dmem_idx", "ex_mem_dmem_idx_n", Span::default());
        s.assign_reg_d_from("ex_mem_we", "ex_mem_we_n", Span::default());
        s.assign_reg_d_from("ex_mem_is_lw", "ex_mem_is_lw_n", Span::default());
        s.assign_reg_d_from("ex_mem_is_sw", "ex_mem_is_sw_n", Span::default());

        s.assign_reg_d_from("id_ex_pc", "id_ex_pc_n", Span::default());
        s.assign_reg_d_from("id_ex_rs1_data", "id_ex_rs1_data_n", Span::default());
        s.assign_reg_d_from("id_ex_rs2_data", "id_ex_rs2_data_n", Span::default());
        s.assign_reg_d_from("id_ex_imm", "id_ex_imm_n", Span::default());
        s.assign_reg_d_from("id_ex_rd", "id_ex_rd_n", Span::default());
        s.assign_reg_d_from("id_ex_rs1", "id_ex_rs1_n", Span::default());
        s.assign_reg_d_from("id_ex_rs2", "id_ex_rs2_n", Span::default());
        s.assign_reg_d_from("id_ex_is_addi", "id_ex_is_addi_n", Span::default());
        s.assign_reg_d_from("id_ex_is_add", "id_ex_is_add_n", Span::default());
        s.assign_reg_d_from("id_ex_is_beq", "id_ex_is_beq_n", Span::default());
        s.assign_reg_d_from("id_ex_is_lw", "id_ex_is_lw_n", Span::default());
        s.assign_reg_d_from("id_ex_is_sw", "id_ex_is_sw_n", Span::default());

        s.assign_reg_d_from("if_id_pc", "if_id_pc_n", Span::default());
        s.assign_reg_d_from("if_id_instr", "if_id_instr_n", Span::default());

        // pc_f mirrors pre-update PC so comb can pair instr with fetch PC (seq-then-comb).
        s.assign_reg_d_from("pc_f", "pc", Span::default());
        s.assign_reg_d_from("pc", "next_pc", Span::default());
        s.end_process();

        s.end_module();
        s.finish()
    }
}

pub fn rhdl_elaborate() -> Result<FrozenHir, Diagnostics> {
    EpisodeIIPipe::elaborate()
}

/// Encode `ADDI rd, rs1, imm`（12-bit signed imm）。
pub fn enc_addi(rd: u32, rs1: u32, imm: i32) -> u64 {
    let imm = (imm as u32) & 0xfff;
    u64::from((imm << 20) | (rs1 << 15) | (rd << 7) | 0b0010011)
}

/// Encode `ADD rd, rs1, rs2`。
pub fn enc_add(rd: u32, rs1: u32, rs2: u32) -> u64 {
    u64::from((rs2 << 20) | (rs1 << 15) | (rd << 7) | 0b0110011)
}

/// Encode `BEQ rs1, rs2, offset`（字节偏移，偶地址）。
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bitloom_hir::PortValues;
    use bitloom_prelude::Elaboratable;
    use bitloom_sim::Sim;
    use bitloom_vlog::emit;

    use super::*;

    /// `bitloom-sim` 每拍先 seq 再 comb：级间 `*_n` 与对 `instr` 的采样滞后一拍。
    /// 测试按「当前 `pc_out`」查 ROM 驱动 `instr`，并在复位后多拍排空/提交。
    fn reset_sim() -> Sim {
        let mut sim = Sim::new(EpisodeIIPipe::elaborate().unwrap());
        let mut pv = PortValues::default();
        pv.set("rst", 1);
        pv.set("instr", 0);
        sim.set_inputs(pv);
        sim.tick();
        sim
    }

    fn tick_with(sim: &mut Sim, instr: u64) {
        let mut pv = PortValues::default();
        pv.set("rst", 0);
        pv.set("instr", instr);
        sim.set_inputs(pv);
        sim.tick();
    }

    fn rom_tick(sim: &mut Sim, rom: &HashMap<u64, u64>) {
        let pc = sim.ports().get("pc_out").unwrap_or(0);
        let instr = *rom.get(&pc).unwrap_or(&0);
        tick_with(sim, instr);
    }

    #[test]
    fn elaborate_ok() {
        let f = EpisodeIIPipe::elaborate().unwrap();
        assert_eq!(f.abi_name, "EpisodeIIPipe");
    }

    /// Clean path：指令间距足够，不依赖转发/分支；五级后 x 寄存器正确。
    #[test]
    fn tick_clean_path_addi_add_golden() {
        let mut sim = reset_sim();
        let mut rom = HashMap::new();
        // 0: ADDI x1; +3 NOP 后 16: ADDI x2; +3 NOP 后 32: ADD（RF 已提交，无 RAW hazard）
        rom.insert(0, enc_addi(1, 0, 5));
        rom.insert(4, enc_addi(0, 0, 0));
        rom.insert(8, enc_addi(0, 0, 0));
        rom.insert(12, enc_addi(0, 0, 0));
        rom.insert(16, enc_addi(2, 0, 7));
        rom.insert(20, enc_addi(0, 0, 0));
        rom.insert(24, enc_addi(0, 0, 0));
        rom.insert(28, enc_addi(0, 0, 0));
        rom.insert(32, enc_add(3, 1, 2));

        // 复位后 comb 仍见 rst=1→next_pc=0；再 arm instr@0，随后按 pc_out 查 ROM。
        tick_with(&mut sim, enc_addi(1, 0, 5));
        for _ in 0..24 {
            rom_tick(&mut sim, &rom);
        }
        assert_eq!(sim.ports().get("x1_out"), Some(5));
        assert_eq!(sim.ports().get("x2_out"), Some(7));
        assert_eq!(sim.ports().get("x3_out"), Some(12));
    }

    /// ALU→ALU RAW：紧邻 ADDI 后 ADD/ADDI 用同一 rd，靠转发得正确结果（无 load-use stall）。
    #[test]
    fn tick_alu_alu_raw_forward_golden() {
        let mut sim = reset_sim();
        let mut rom = HashMap::new();
        // ADDI x1,x0,5; ADDI x1,x1,3 → x1=8（MEM/WB 与 EX/MEM 转发）
        rom.insert(0, enc_addi(1, 0, 5));
        rom.insert(4, enc_addi(1, 1, 3));
        // ADD x2,x1,x1 → 16（再测 EX/MEM 转发）
        rom.insert(8, enc_add(2, 1, 1));

        tick_with(&mut sim, enc_addi(1, 0, 5));
        for _ in 0..14 {
            rom_tick(&mut sim, &rom);
        }
        assert_eq!(sim.ports().get("x1_out"), Some(8));
        assert_eq!(sim.ports().get("x2_out"), Some(16));
    }

    /// 负向 ADDI：保留 17.3 signed I-imm（pipe 路径独立黄金）。
    #[test]
    fn tick_addi_negative_imm_pipe_golden() {
        let mut sim = reset_sim();
        let mut rom = HashMap::new();
        rom.insert(0, enc_addi(1, 0, -3));
        rom.insert(4, enc_addi(2, 1, -1));

        tick_with(&mut sim, enc_addi(1, 0, -3));
        for _ in 0..14 {
            rom_tick(&mut sim, &rom);
        }
        assert_eq!(sim.ports().get("x1_out"), Some((-3i32) as u32 as u64));
        assert_eq!(sim.ports().get("x2_out"), Some((-4i32) as u32 as u64));
    }

    /// Taken BEQ：比较数经 EX 转发；错误路径写 RF 不得提交；PC redirect。
    #[test]
    fn tick_beq_taken_flush_wrong_path_not_committed() {
        let mut sim = reset_sim();
        let mut rom = HashMap::new();
        // 紧邻生产者 → BEQ：ID 锁存时 RF 仍旧，EX 靠转发比较。
        // 0: ADDI x1; 4: ADDI x2; 8: BEQ +8 → 16; 12: wrong-path; 16: taken-path.
        rom.insert(0, enc_addi(1, 0, 1));
        rom.insert(4, enc_addi(2, 0, 1));
        rom.insert(8, enc_beq(1, 2, 8));
        rom.insert(12, enc_addi(3, 0, 0xff));
        rom.insert(16, enc_addi(4, 0, 0x11));

        tick_with(&mut sim, enc_addi(1, 0, 1));
        for _ in 0..28 {
            rom_tick(&mut sim, &rom);
        }
        assert_eq!(sim.ports().get("x1_out"), Some(1));
        assert_eq!(sim.ports().get("x2_out"), Some(1));
        assert_eq!(
            sim.ports().get("x3_out"),
            Some(0),
            "wrong-path ADDI x3 must not commit"
        );
        assert_eq!(sim.ports().get("x4_out"), Some(0x11));
        // Taken-path ADDI@16 已提交即可；其后空 ROM 上 PC 继续 +4。
        let pc = sim.ports().get("pc_out").unwrap();
        assert!(
            pc >= 16,
            "PC should have redirected to taken-path (>=16), got {pc}"
        );
    }

    #[test]
    fn emit_verilog_smoke() {
        let hir = EpisodeIIPipe::elaborate().unwrap();
        let art = emit(&hir);
        assert!(!art.files.is_empty());
        let v = &art.files[0].contents;
        assert!(v.contains("module EpisodeIIPipe"));
        assert!(v.contains("always @(posedge clk)"));
    }
}
