# 子集合规门禁说明（Story 15.4 / NFR26；Episode II FR66 / NFR31）

本仓库对 Episode I/II 子集使用**最小过滤器**与**定向黄金**，**不是**完整设计验证（DV）。

## 验证阶梯（directed → 可选 rv32ui）

| 阶 | 内容 | 状态 |
|----|------|------|
| 1. 定向 | 仓库内黄金 `tick` / `elaborate` / `cargo bitloom build` | **启用** |
| 2. 可选 `rv32ui` | 接入 `riscv-tests` 的 `rv32ui-*` 子集冒烟 | **可选 / 延期**（文档化跑法如下；**未**进 CI） |
| 3. 未启用 | RISCOF / arch-test CI、完整 DV | **未启用** |

**不得**在 README 或本文件宣称已通过 arch-test，或把 arch-test 绿当作流水/hazard 正确性证明。

## 启用的门禁 (a)

- 文档化子集：`examples/rv32_core/SUBSET.md`（含 Story 17.2 取指合同：**harness `instr`**，不得静默混用片上 SyncReadMem I-fetch；Story 17.3 已冻结 I/S/B/U/J 符号扩展与负向 BEQ；LB/LH 书面延期）
- Episode I：`cargo test -p rv32_core`（含 ALU/负向立即数/正负 BEQ/MMIO tick；SW@`0x100` 排除 DMEM 旁路写）与 `cargo bitloom build --package rv32_core`
- 测试名 `subset_minimal_filter_program` 将 ADDI→SW→LED 串成一条可回归路径
- Episode II 流水（17.4–17.5）另包 `examples/rv32_pipe`：五级 + 转发 + load-use stall + 分支 flush；`cargo test -p rv32_pipe` / `cargo bitloom build --package rv32_pipe`；见该包 `PIPE.md`（取指仍为 harness `instr`；无 CSR）
- Load-use 独立 ATDD：`tick_load_use_stall_atdd_golden`（rs1 / ADDI）、`tick_load_use_rs2_consumer_atdd_golden`（rs2 / ADD）；无停顿会失败
- 可选 Zicsr/M-trap（FR65 / NFR32）：**可选已实现**于 `examples/rv32_priv`；教程 [`docs/tutorials/rv32-episode-ii/06-csr-m-trap.md`](../../docs/tutorials/rv32-episode-ii/06-csr-m-trap.md)；`cargo test -p rv32_priv`；**不**宣称 Privileged/arch-test 合规；不回溯 Epic 17

## 未启用 (b)

未接入 `riscv-tests` / RISCOF / arch-test **CI**。本文件只说明日后如何**可选**本地跑 `rv32ui`；通过也不构成完整 DV。

## 可选 `rv32ui`（延期 — 如何日后本地跑，非 CI）

**地位：** optional / deferred。教学核仍以定向黄金为准；**不要**把 `rv32ui` 绿当成 arch-test 或完整 DV。

若后续故事启用本地冒烟（仍可不进 CI）：

1. 检出上游 [riscv-tests](https://github.com/riscv-software-src/riscv-tests)（不必 vendor 进本仓，除非故事明确要求）。
2. 用 RISC-V GNU toolchain 构建与本子集重叠的 `rv32ui-*`（至少 `addi` / `add` / `beq` / `lw` / `sw` 等；跳过依赖 LB/LH、完整 privilege、或超出 x1–x4 教学 RF 的用例）。
3. 把机器码按 **harness `instr` 合同**喂给 `EpisodeICore` / `EpisodeIIPipe`（按当前 `pc_out` 查 ROM），或另立取指故事后再改驱动方式。
4. 通过判据：定向观察 `tohost` / 约定寄存器（自行约定）；**绿 ≠** RISCOF/arch-test 等价。

当前**不**启用 CI job、**不**宣称完整指令集符合性。不影响 Epic 17/18 流水 DoD。

## 可选进阶（非必做）

RVFI / `riscv-formal` 若出现仅为进阶选项，不构成本子集「完整」的必要条件。
