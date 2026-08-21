# Episode II 可选特权核（FR65）

公开品牌 **Bitloom**。与 `samitbasu/rhdl` 无关。设计依赖仅 `bitloom-prelude`。

包：`examples/rv32_priv`（`EpisodeIIPriv`）。**不**修改 `rv32_pipe` 流水 RTL；Epic 17 DoD 不依赖本包（NFR32）。

## 取指合同

harness `instr` 口（与 [`../rv32_core/SUBSET.md`](../rv32_core/SUBSET.md) 合同 (b) 一致）。

## 交付

| 项 | 说明 |
|----|------|
| 模型 | 边沿提交（非五级） |
| 用户指令 | `ADDI`（教学路径：非负小立即数，无符号扩展） |
| Zicsr | `CSRRW` / `CSRRS` |
| Trap | `ECALL` → 写 mepc/mcause/mstatus，PC←mtvec；`MRET` 返回 |
| CSR | mstatus, mtvec, mepc, mcause, mscratch, mie |
| 串行化 | 写 mstatus/mie 后下一提交前可观测；并入流水时须 flush |
| 非目标 | Privileged/arch-test 合规；PicoRV32 IRQ 模板；`mtval` |

## 验证

```bash
cargo test -p rv32_priv
```

黄金：`tick_mtvec_ecall_mret_golden`、`tick_csr_rmw_and_ie_serialize_golden`。

教程：[Ch.06](../../docs/tutorials/rv32-episode-ii/06-csr-m-trap.md)。
