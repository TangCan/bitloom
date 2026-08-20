# 子集合规门禁说明（Story 15.4 / NFR26；Episode II FR66 / NFR31）

本仓库对 Episode I/II 子集使用**最小过滤器**与**定向黄金**，**不是**完整设计验证（DV）。

## 验证阶梯（directed → 可选 rv32ui）

| 阶 | 内容 | 状态 |
|----|------|------|
| 1. 定向 | 仓库内黄金 `tick` / `elaborate` / `cargo bitloom build` | **启用** |
| 2. 可选 `rv32ui` | 接入 `riscv-tests` 的 `rv32ui-*` 子集冒烟 | **延期**（不阻塞 Epic 17 Done） |
| 3. 未启用 | RISCOF / arch-test CI、完整 DV | **未启用** |

**不得**在 README 或本文件宣称已通过 arch-test，或把 arch-test 绿当作流水/hazard 正确性证明。

## 启用的门禁 (a)

- 文档化子集：`examples/rv32_core/SUBSET.md`（含 Story 17.2 取指合同：**harness `instr`**，不得静默混用片上 SyncReadMem I-fetch；Story 17.3 已冻结 I/S/B/U/J 符号扩展与负向 BEQ；LB/LH 书面延期）
- Episode I：`cargo test -p rv32_core`（含 ALU/负向立即数/正负 BEQ/MMIO tick）与 `cargo bitloom build --package rv32_core`
- 测试名 `subset_minimal_filter_program` 将 ADDI→SW→LED 串成一条可回归路径
- Episode II 流水（17.4–17.5）另包 `examples/rv32_pipe`：五级 + 转发 + load-use stall + 分支 flush；`cargo test -p rv32_pipe` / `cargo bitloom build --package rv32_pipe`；见该包 `PIPE.md`（取指仍为 harness `instr`；无 CSR）
- Load-use 独立 ATDD：`tick_load_use_stall_atdd_golden`（无停顿会失败）

## 未启用 (b)

未接入 `riscv-tests` / RISCOF / arch-test CI。

## 可选 `rv32ui`（延期，如何日后接入）

若后续故事启用：在 CI 或本地脚本中检出 `riscv-tests`，仅跑与本子集重叠的 `rv32ui-*`（ADDI/ADD/BEQ/LW/SW 等），用 harness 喂 `instr` 或另立取指故事；**通过也不等于** arch-test / 完整 DV。当前标为可选/延期，**不影响** Story 17.5 / Epic 17 Done。

## 可选进阶（非必做）

RVFI / `riscv-formal` 若出现仅为进阶选项，不构成本子集「完整」的必要条件。
