# 子集合规门禁说明（Story 15.4 / NFR26）

本仓库对 Episode I 子集使用**最小过滤器**，**不是**完整设计验证（DV）。

## 启用的门禁 (a)

- 文档化子集：`examples/rv32_core/SUBSET.md`（含 Story 17.2 取指合同：**harness `instr`**，不得静默混用片上 SyncReadMem I-fetch）
- 仓库内黄金程序 + `cargo test -p rv32_core`（含 ALU/分支/MMIO tick）与 `cargo bitloom build --package rv32_core`
- 测试名 `subset_minimal_filter_program` 将 ADDI→SW→LED 串成一条可回归路径

## 未启用 (b)

未接入 `riscv-tests` / RISCOF / arch-test CI。**不得**在 README 宣称已通过 arch-test。

## 可选进阶（非必做）

RVFI / `riscv-formal` 若出现仅为进阶选项，不构成本子集「完整」的必要条件。
