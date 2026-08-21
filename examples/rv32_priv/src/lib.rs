//! Episode II 可选特权教学核（FR65 / epic-18-retro-item-35）。
//!
//! 边沿提交 + harness `instr`：ADDI + CSRRW/CSRRS + ECALL trap + MRET。
//! CSR：mstatus / mtvec / mepc / mcause / mscratch / mie。
//! 目标「能教 / 能跑 mret」——**不**宣称 Privileged / arch-test 合规。
//! 写 mstatus/mie 后边沿提交天然串行化（下一拍取指前可见）；并入流水时须 flush。
//! 设计依赖仅 `bitloom-prelude`。公开品牌 Bitloom；与 `samitbasu/rhdl` 无关。

use bitloom_prelude::{Diagnostics, Elaboratable, ElaborateSession, FrozenHir, GroundType, Span};

/// 教学向 M-mode CSR + trap 最小核。
pub struct EpisodeIIPriv;

impl Elaboratable for EpisodeIIPriv {
    fn elaborate() -> Result<FrozenHir, Diagnostics> {
        let mut s = ElaborateSession::new("EpisodeIIPriv");
        s.begin_module("EpisodeIIPriv", Span::default());
        s.add_input("clk", GroundType::Clock, Span::default());
        s.add_input("rst", GroundType::Reset, Span::default());
        s.add_input("instr", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("pc_out", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("x1_out", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("x2_out", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("x3_out", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("x4_out", GroundType::UInt { width: 32 }, Span::default());
        s.add_output(
            "mstatus_out",
            GroundType::UInt { width: 32 },
            Span::default(),
        );
        s.add_output("mtvec_out", GroundType::UInt { width: 32 }, Span::default());
        s.add_output("mepc_out", GroundType::UInt { width: 32 }, Span::default());
        s.add_output(
            "mcause_out",
            GroundType::UInt { width: 32 },
            Span::default(),
        );
        s.add_output(
            "mscratch_out",
            GroundType::UInt { width: 32 },
            Span::default(),
        );
        s.add_output("mie_out", GroundType::UInt { width: 32 }, Span::default());

        for n in [
            "pc", "x1", "x2", "x3", "x4", "mstatus", "mtvec", "mepc", "mcause", "mscratch", "mie",
        ] {
            s.declare_reg(n, GroundType::UInt { width: 32 }, Span::default());
        }

        for (n, w) in [
            ("c0", 32u32),
            ("c1", 32),
            ("c2", 32),
            ("c3", 32),
            ("c4", 32),
            ("c7", 32),
            ("c12", 32),
            ("c15", 32),
            ("c20", 32),
            ("mask3", 32),
            ("mask5", 32),
            ("mask7", 32),
            ("mask12", 32),
            ("mask32", 32),
            ("op_addi", 32),
            ("op_system", 32),
            ("f3_csrrw", 32),
            ("f3_csrrs", 32),
            ("csr_mstatus", 32),
            ("csr_mie", 32),
            ("csr_mtvec", 32),
            ("csr_mscratch", 32),
            ("csr_mepc", 32),
            ("csr_mcause", 32),
            ("imm_mret", 32),
            ("cause_ecall", 32),
            ("bit_mie", 32),
            ("bit_mpie", 32),
            ("mask_not_mie", 32),
            ("mask_not_mpie", 32),
        ] {
            s.declare_wire(n, GroundType::UInt { width: w }, Span::default());
        }

        for n in [
            "opcode",
            "rd",
            "funct3",
            "rs1",
            "imm_i_raw",
            "sh7",
            "sh12",
            "sh15",
            "sh20",
            "is_addi",
            "is_system",
            "f3_is_csrrw",
            "f3_is_csrrs",
            "is_csr_f3",
            "is_csrrw",
            "is_csrrs",
            "is_csr",
            "f3_zero",
            "is_priv",
            "imm_is_0",
            "imm_is_mret",
            "is_ecall",
            "is_mret",
            "rs1_is_0",
            "rs1_nz",
            "csrrs_write",
            "do_csr_write",
            "sel_mstatus",
            "sel_mie",
            "sel_mtvec",
            "sel_mscratch",
            "sel_mepc",
            "sel_mcause",
            "csr_r0",
            "csr_r1",
            "csr_r2",
            "csr_r3",
            "csr_r4",
            "csr_rdata",
            "csr_or",
            "csr_wdata",
            "wr_mstatus",
            "wr_mie",
            "wr_mtvec",
            "wr_mscratch",
            "wr_mepc",
            "wr_mcause",
            "rs1_1",
            "rs1_2",
            "rs1_3",
            "rs1_4",
            "t_rs1_1",
            "t_rs1_2",
            "t_rs1_3",
            "rs1_data",
            "rd1",
            "rd2",
            "rd3",
            "rd4",
            "we",
            "we1",
            "we2",
            "we3",
            "we4",
            "wb_data",
            "next_x1",
            "next_x2",
            "next_x3",
            "next_x4",
            "pc_plus4",
            "alu_addi_raw",
            "alu_addi",
            "mie_bit",
            "mpie_from_mie",
            "mstatus_clr_ie",
            "mstatus_clr_mpie",
            "mstatus_trap",
            "mpie_bit",
            "mie_from_mpie",
            "mstatus_clr_mie2",
            "mstatus_set_mie",
            "mstatus_mret_clr",
            "mstatus_mret",
            "next_mstatus_w",
            "next_mstatus_t",
            "next_mstatus_m",
            "next_mstatus",
            "next_mie_w",
            "next_mie",
            "next_mtvec_w",
            "next_mtvec",
            "next_mscratch_w",
            "next_mscratch",
            "next_mepc_w",
            "next_mepc_t",
            "next_mepc",
            "next_mcause_w",
            "next_mcause_t",
            "next_mcause",
            "next_pc_trap",
            "next_pc_mret",
            "next_pc_pre",
            "next_pc",
            "ie_sel",
            "ie_csr_write",
        ] {
            s.declare_wire(n, GroundType::UInt { width: 32 }, Span::default());
        }

        s.begin_combinational(Span::default());
        s.assign_lit("c0", 0, Span::default());
        s.assign_lit("c1", 1, Span::default());
        s.assign_lit("c2", 2, Span::default());
        s.assign_lit("c3", 3, Span::default());
        s.assign_lit("c4", 4, Span::default());
        s.assign_lit("c7", 7, Span::default());
        s.assign_lit("c12", 12, Span::default());
        s.assign_lit("c15", 15, Span::default());
        s.assign_lit("c20", 20, Span::default());
        s.assign_lit("mask3", 0x7, Span::default());
        s.assign_lit("mask5", 0x1f, Span::default());
        s.assign_lit("mask7", 0x7f, Span::default());
        s.assign_lit("mask12", 0xfff, Span::default());
        s.assign_lit("mask32", 0xffff_ffff, Span::default());
        s.assign_lit("op_addi", 0b0010011, Span::default());
        s.assign_lit("op_system", 0b1110011, Span::default());
        s.assign_lit("f3_csrrw", 1, Span::default());
        s.assign_lit("f3_csrrs", 2, Span::default());
        s.assign_lit("csr_mstatus", 0x300, Span::default());
        s.assign_lit("csr_mie", 0x304, Span::default());
        s.assign_lit("csr_mtvec", 0x305, Span::default());
        s.assign_lit("csr_mscratch", 0x340, Span::default());
        s.assign_lit("csr_mepc", 0x341, Span::default());
        s.assign_lit("csr_mcause", 0x342, Span::default());
        s.assign_lit("imm_mret", 0x302, Span::default());
        s.assign_lit("cause_ecall", 11, Span::default());
        s.assign_lit("bit_mie", 0x8, Span::default());
        s.assign_lit("bit_mpie", 0x80, Span::default());
        s.assign_lit("mask_not_mie", 0xffff_fff7, Span::default());
        s.assign_lit("mask_not_mpie", 0xffff_ff7f, Span::default());

        s.assign_and("opcode", "instr", "mask7", Span::default());
        s.assign_shr("sh7", "instr", "c7", Span::default());
        s.assign_and("rd", "sh7", "mask5", Span::default());
        s.assign_shr("sh12", "instr", "c12", Span::default());
        s.assign_and("funct3", "sh12", "mask3", Span::default());
        s.assign_shr("sh15", "instr", "c15", Span::default());
        s.assign_and("rs1", "sh15", "mask5", Span::default());
        s.assign_shr("sh20", "instr", "c20", Span::default());
        s.assign_and("imm_i_raw", "sh20", "mask12", Span::default());

        s.assign_eq("is_addi", "opcode", "op_addi", Span::default());
        s.assign_eq("is_system", "opcode", "op_system", Span::default());
        s.assign_eq("f3_is_csrrw", "funct3", "f3_csrrw", Span::default());
        s.assign_eq("f3_is_csrrs", "funct3", "f3_csrrs", Span::default());
        s.assign_or("is_csr_f3", "f3_is_csrrw", "f3_is_csrrs", Span::default());
        s.assign_and("is_csrrw", "is_system", "f3_is_csrrw", Span::default());
        s.assign_and("is_csrrs", "is_system", "f3_is_csrrs", Span::default());
        s.assign_and("is_csr", "is_system", "is_csr_f3", Span::default());
        s.assign_eq("f3_zero", "funct3", "c0", Span::default());
        s.assign_and("is_priv", "is_system", "f3_zero", Span::default());
        s.assign_eq("imm_is_0", "imm_i_raw", "c0", Span::default());
        s.assign_eq("imm_is_mret", "imm_i_raw", "imm_mret", Span::default());
        s.assign_and("is_ecall", "is_priv", "imm_is_0", Span::default());
        s.assign_and("is_mret", "is_priv", "imm_is_mret", Span::default());

        s.assign_eq("sel_mstatus", "imm_i_raw", "csr_mstatus", Span::default());
        s.assign_eq("sel_mie", "imm_i_raw", "csr_mie", Span::default());
        s.assign_eq("sel_mtvec", "imm_i_raw", "csr_mtvec", Span::default());
        s.assign_eq("sel_mscratch", "imm_i_raw", "csr_mscratch", Span::default());
        s.assign_eq("sel_mepc", "imm_i_raw", "csr_mepc", Span::default());
        s.assign_eq("sel_mcause", "imm_i_raw", "csr_mcause", Span::default());

        s.assign_mux("csr_r0", "sel_mstatus", "mstatus", "c0", Span::default());
        s.assign_mux("csr_r1", "sel_mie", "mie", "csr_r0", Span::default());
        s.assign_mux("csr_r2", "sel_mtvec", "mtvec", "csr_r1", Span::default());
        s.assign_mux(
            "csr_r3",
            "sel_mscratch",
            "mscratch",
            "csr_r2",
            Span::default(),
        );
        s.assign_mux("csr_r4", "sel_mepc", "mepc", "csr_r3", Span::default());
        s.assign_mux(
            "csr_rdata",
            "sel_mcause",
            "mcause",
            "csr_r4",
            Span::default(),
        );

        s.assign_eq("rs1_1", "rs1", "c1", Span::default());
        s.assign_eq("rs1_2", "rs1", "c2", Span::default());
        s.assign_eq("rs1_3", "rs1", "c3", Span::default());
        s.assign_eq("rs1_4", "rs1", "c4", Span::default());
        s.assign_mux("t_rs1_1", "rs1_1", "x1", "c0", Span::default());
        s.assign_mux("t_rs1_2", "rs1_2", "x2", "t_rs1_1", Span::default());
        s.assign_mux("t_rs1_3", "rs1_3", "x3", "t_rs1_2", Span::default());
        s.assign_mux("rs1_data", "rs1_4", "x4", "t_rs1_3", Span::default());

        s.assign_eq("rs1_is_0", "rs1", "c0", Span::default());
        // rs1_nz = !rs1_is_0 → mux(rs1_is_0, 0, 1)
        s.assign_mux("rs1_nz", "rs1_is_0", "c0", "c1", Span::default());
        s.assign_and("csrrs_write", "is_csrrs", "rs1_nz", Span::default());
        s.assign_or("do_csr_write", "is_csrrw", "csrrs_write", Span::default());

        s.assign_or("csr_or", "csr_rdata", "rs1_data", Span::default());
        s.assign_mux(
            "csr_wdata",
            "is_csrrw",
            "rs1_data",
            "csr_or",
            Span::default(),
        );

        s.assign_and("wr_mstatus", "do_csr_write", "sel_mstatus", Span::default());
        s.assign_and("wr_mie", "do_csr_write", "sel_mie", Span::default());
        s.assign_and("wr_mtvec", "do_csr_write", "sel_mtvec", Span::default());
        s.assign_and(
            "wr_mscratch",
            "do_csr_write",
            "sel_mscratch",
            Span::default(),
        );
        s.assign_and("wr_mepc", "do_csr_write", "sel_mepc", Span::default());
        s.assign_and("wr_mcause", "do_csr_write", "sel_mcause", Span::default());

        // ADDI：教学路径只用非负小立即数（无符号扩展）；handler/常量 < 2048。
        s.assign_add("alu_addi_raw", "rs1_data", "imm_i_raw", Span::default());
        s.assign_and("alu_addi", "alu_addi_raw", "mask32", Span::default());
        s.assign_mux(
            "wb_data",
            "is_csr",
            "csr_rdata",
            "alu_addi",
            Span::default(),
        );

        s.assign_eq("rd1", "rd", "c1", Span::default());
        s.assign_eq("rd2", "rd", "c2", Span::default());
        s.assign_eq("rd3", "rd", "c3", Span::default());
        s.assign_eq("rd4", "rd", "c4", Span::default());
        s.assign_or("we", "is_addi", "is_csr", Span::default());
        s.assign_and("we1", "we", "rd1", Span::default());
        s.assign_and("we2", "we", "rd2", Span::default());
        s.assign_and("we3", "we", "rd3", Span::default());
        s.assign_and("we4", "we", "rd4", Span::default());
        s.assign_mux("next_x1", "we1", "wb_data", "x1", Span::default());
        s.assign_mux("next_x2", "we2", "wb_data", "x2", Span::default());
        s.assign_mux("next_x3", "we3", "wb_data", "x3", Span::default());
        s.assign_mux("next_x4", "we4", "wb_data", "x4", Span::default());

        s.assign_add("pc_plus4", "pc", "c4", Span::default());

        // mstatus：trap 时 MPIE←MIE、MIE←0；mret 时 MIE←MPIE、MPIE←1
        s.assign_and("mie_bit", "mstatus", "bit_mie", Span::default());
        s.assign_shl("mpie_from_mie", "mie_bit", "c4", Span::default());
        s.assign_and("mstatus_clr_ie", "mstatus", "mask_not_mie", Span::default());
        s.assign_and(
            "mstatus_clr_mpie",
            "mstatus_clr_ie",
            "mask_not_mpie",
            Span::default(),
        );
        s.assign_or(
            "mstatus_trap",
            "mstatus_clr_mpie",
            "mpie_from_mie",
            Span::default(),
        );

        s.assign_and("mpie_bit", "mstatus", "bit_mpie", Span::default());
        s.assign_shr("mie_from_mpie", "mpie_bit", "c4", Span::default());
        s.assign_and(
            "mstatus_clr_mie2",
            "mstatus",
            "mask_not_mie",
            Span::default(),
        );
        s.assign_or(
            "mstatus_set_mie",
            "mstatus_clr_mie2",
            "mie_from_mpie",
            Span::default(),
        );
        s.assign_and(
            "mstatus_mret_clr",
            "mstatus_set_mie",
            "mask_not_mpie",
            Span::default(),
        );
        s.assign_or(
            "mstatus_mret",
            "mstatus_mret_clr",
            "bit_mpie",
            Span::default(),
        );

        s.assign_mux(
            "next_mstatus_w",
            "wr_mstatus",
            "csr_wdata",
            "mstatus",
            Span::default(),
        );
        s.assign_mux(
            "next_mstatus_t",
            "is_ecall",
            "mstatus_trap",
            "next_mstatus_w",
            Span::default(),
        );
        s.assign_mux(
            "next_mstatus_m",
            "is_mret",
            "mstatus_mret",
            "next_mstatus_t",
            Span::default(),
        );
        s.assign_net("next_mstatus", "next_mstatus_m", Span::default());

        s.assign_mux("next_mie_w", "wr_mie", "csr_wdata", "mie", Span::default());
        s.assign_net("next_mie", "next_mie_w", Span::default());

        s.assign_mux(
            "next_mtvec_w",
            "wr_mtvec",
            "csr_wdata",
            "mtvec",
            Span::default(),
        );
        s.assign_net("next_mtvec", "next_mtvec_w", Span::default());

        s.assign_mux(
            "next_mscratch_w",
            "wr_mscratch",
            "csr_wdata",
            "mscratch",
            Span::default(),
        );
        s.assign_net("next_mscratch", "next_mscratch_w", Span::default());

        s.assign_mux(
            "next_mepc_w",
            "wr_mepc",
            "csr_wdata",
            "mepc",
            Span::default(),
        );
        s.assign_mux(
            "next_mepc_t",
            "is_ecall",
            "pc_plus4",
            "next_mepc_w",
            Span::default(),
        );
        s.assign_net("next_mepc", "next_mepc_t", Span::default());

        s.assign_mux(
            "next_mcause_w",
            "wr_mcause",
            "csr_wdata",
            "mcause",
            Span::default(),
        );
        s.assign_mux(
            "next_mcause_t",
            "is_ecall",
            "cause_ecall",
            "next_mcause_w",
            Span::default(),
        );
        s.assign_net("next_mcause", "next_mcause_t", Span::default());

        s.assign_mux(
            "next_pc_trap",
            "is_ecall",
            "mtvec",
            "pc_plus4",
            Span::default(),
        );
        s.assign_mux(
            "next_pc_mret",
            "is_mret",
            "mepc",
            "next_pc_trap",
            Span::default(),
        );
        // 复位拍保持 next_pc=0，避免 rst 后首条指令被错误 +4 跳过。
        s.assign_mux("next_pc_pre", "rst", "c0", "next_pc_mret", Span::default());
        s.assign_net("next_pc", "next_pc_pre", Span::default());

        // 观测：写影响 IE 的 CSR（文档 + 测试用）
        s.assign_or("ie_sel", "sel_mstatus", "sel_mie", Span::default());
        s.assign_and("ie_csr_write", "do_csr_write", "ie_sel", Span::default());

        s.assign_net("pc_out", "pc", Span::default());
        s.assign_net("x1_out", "x1", Span::default());
        s.assign_net("x2_out", "x2", Span::default());
        s.assign_net("x3_out", "x3", Span::default());
        s.assign_net("x4_out", "x4", Span::default());
        s.assign_net("mstatus_out", "mstatus", Span::default());
        s.assign_net("mtvec_out", "mtvec", Span::default());
        s.assign_net("mepc_out", "mepc", Span::default());
        s.assign_net("mcause_out", "mcause", Span::default());
        s.assign_net("mscratch_out", "mscratch", Span::default());
        s.assign_net("mie_out", "mie", Span::default());
        s.end_process();

        s.begin_sequential(Span::default());
        s.assign_reg_d_from("pc", "next_pc", Span::default());
        s.assign_reg_d_from("x1", "next_x1", Span::default());
        s.assign_reg_d_from("x2", "next_x2", Span::default());
        s.assign_reg_d_from("x3", "next_x3", Span::default());
        s.assign_reg_d_from("x4", "next_x4", Span::default());
        s.assign_reg_d_from("mstatus", "next_mstatus", Span::default());
        s.assign_reg_d_from("mie", "next_mie", Span::default());
        s.assign_reg_d_from("mtvec", "next_mtvec", Span::default());
        s.assign_reg_d_from("mscratch", "next_mscratch", Span::default());
        s.assign_reg_d_from("mepc", "next_mepc", Span::default());
        s.assign_reg_d_from("mcause", "next_mcause", Span::default());
        s.end_process();

        s.end_module();
        s.finish()
    }
}

pub fn rhdl_elaborate() -> Result<FrozenHir, Diagnostics> {
    EpisodeIIPriv::elaborate()
}

/// Encode `ADDI rd, rs1, imm`（本核立即数按无符号 12-bit 相加；测试用非负小常量）。
pub fn enc_addi(rd: u32, rs1: u32, imm: u32) -> u64 {
    let imm = imm & 0xfff;
    u64::from((imm << 20) | (rs1 << 15) | (rd << 7) | 0b0010011)
}

/// Encode `CSRRW rd, csr, rs1`。
pub fn enc_csrrw(rd: u32, csr: u32, rs1: u32) -> u64 {
    u64::from(((csr & 0xfff) << 20) | (rs1 << 15) | (0b001 << 12) | (rd << 7) | 0b1110011)
}

/// Encode `CSRRS rd, csr, rs1`。
pub fn enc_csrrs(rd: u32, csr: u32, rs1: u32) -> u64 {
    u64::from(((csr & 0xfff) << 20) | (rs1 << 15) | (0b010 << 12) | (rd << 7) | 0b1110011)
}

/// Encode `ECALL`。
pub fn enc_ecall() -> u64 {
    0x0000_0073
}

/// Encode `MRET`。
pub fn enc_mret() -> u64 {
    0x3020_0073
}

pub const CSR_MSTATUS: u32 = 0x300;
pub const CSR_MIE: u32 = 0x304;
pub const CSR_MTVEC: u32 = 0x305;
pub const CSR_MSCRATCH: u32 = 0x340;
pub const CSR_MEPC: u32 = 0x341;
pub const CSR_MCAUSE: u32 = 0x342;

#[cfg(test)]
mod tests {
    use bitloom_hir::PortValues;
    use bitloom_prelude::Elaboratable;
    use bitloom_sim::Sim;
    use bitloom_vlog::emit;

    use super::*;

    fn reset_sim() -> Sim {
        let mut sim = Sim::new(EpisodeIIPriv::elaborate().unwrap());
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

    /// 边沿提交 + seq→comb：本拍 seq 提交上一拍 comb，再算当前 `instr`。
    /// 灌指序列中最后一条需多喂一拍（可用 ADDI x0 或重复）才能提交。
    fn step(sim: &mut Sim, instr: u64) {
        tick_with(sim, instr);
    }

    #[test]
    fn elaborate_ok() {
        let f = EpisodeIIPriv::elaborate().unwrap();
        assert_eq!(f.abi_name, "EpisodeIIPriv");
    }

    /// CSR RMW：写 mscratch，再 CSRRS 读回；IE CSR（mstatus）写后下一提交可见。
    #[test]
    fn tick_csr_rmw_and_ie_serialize_golden() {
        let mut sim = reset_sim();
        let nop = enc_addi(0, 0, 0);

        step(&mut sim, enc_addi(1, 0, 0x11)); // arm
        step(&mut sim, enc_csrrw(0, CSR_MSCRATCH, 1)); // commit addi
        assert_eq!(sim.ports().get("x1_out"), Some(0x11));

        step(&mut sim, enc_csrrs(2, CSR_MSCRATCH, 0)); // commit csrrw
        assert_eq!(sim.ports().get("mscratch_out"), Some(0x11));

        step(&mut sim, enc_addi(1, 0, 0x8)); // commit csrrs → x2
        assert_eq!(sim.ports().get("x2_out"), Some(0x11));

        step(&mut sim, enc_csrrw(0, CSR_MSTATUS, 1)); // commit addi x1=8
        assert_eq!(sim.ports().get("x1_out"), Some(0x8));

        step(&mut sim, nop); // commit mstatus write
        assert_eq!(
            sim.ports().get("mstatus_out"),
            Some(0x8),
            "IE-affecting CSR write must be visible before younger instr side-effects"
        );
    }

    /// 写 mtvec → ECALL trap → handler → MRET 回到 mepc。
    #[test]
    fn tick_mtvec_ecall_mret_golden() {
        let mut sim = reset_sim();
        let nop = enc_addi(0, 0, 0);

        step(&mut sim, enc_addi(1, 0, 0x20)); // arm
        step(&mut sim, enc_csrrw(0, CSR_MTVEC, 1)); // commit addi
        step(&mut sim, enc_ecall()); // commit mtvec
        assert_eq!(sim.ports().get("mtvec_out"), Some(0x20));

        step(&mut sim, enc_addi(2, 0, 0x55)); // commit ecall (trap)
        assert_eq!(sim.ports().get("mcause_out"), Some(11));
        let mepc = sim.ports().get("mepc_out").expect("mepc");
        assert!(mepc >= 4, "mepc should be pc+4 of ecall site, got {mepc}");
        assert_eq!(
            sim.ports().get("pc_out"),
            Some(0x20),
            "trap must jump to mtvec"
        );

        step(&mut sim, enc_mret()); // commit handler addi
        assert_eq!(sim.ports().get("x2_out"), Some(0x55), "handler must run");

        step(&mut sim, enc_addi(3, 0, 0x66)); // commit mret
        assert_eq!(
            sim.ports().get("pc_out"),
            Some(mepc),
            "mret must restore mepc"
        );

        step(&mut sim, nop); // commit return-path addi
        assert_eq!(
            sim.ports().get("x3_out"),
            Some(0x66),
            "execution continues after mret"
        );
    }

    #[test]
    fn emit_verilog_smoke() {
        let hir = EpisodeIIPriv::elaborate().unwrap();
        let art = emit(&hir);
        assert!(!art.files.is_empty());
        let v = &art.files[0].contents;
        assert!(v.contains("module EpisodeIIPriv"));
        assert!(v.contains("always @(posedge clk)"));
    }
}
