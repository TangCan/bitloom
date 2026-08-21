# Chapter 6 — 可选 Zicsr + M-mode trap（已实现 · 教学最小集）

**状态：可选已实现。** 本页对应 FR65 教学最小集，**不是** Privileged / `riscv-arch-test` M-mode 合规合同。

公开品牌 **Bitloom**。与 `samitbasu/rhdl` 无关。设计依赖仅 `bitloom-prelude`。

## 为何独立包（NFR32）

Epic 17 五级流水（`examples/rv32_pipe`）的 DoD **不依赖** CSR。  
可选特权落在 **`examples/rv32_priv`**（边沿提交 + SYSTEM），避免把 CSR 半成品塞进流水主路径。

## 实现范围（FR65）

| 项 | 本仓库交付 |
|----|------------|
| CSR | `mstatus` / `mtvec` / `mepc` / `mcause` / `mscratch` / `mie`（`mtval` 未做） |
| 指令 | `CSRRW` / `CSRRS` + `ECALL`（trap 入口）+ `MRET` |
| Trap | 写 `mepc←pc+4`、`mcause←11`、更新 `mstatus` IE 栈，PC←`mtvec` |
| 串行化 | 边沿提交：写 `mstatus`/`mie` 在提交拍生效，下一指令前可观测（无流水 skid）；**若并入五级流水，须像 taken branch 一样 flush** |
| 禁止 | **不以** PicoRV32 自定义 IRQ 为标准模板 |
| 取指 | harness `instr`（与 Episode I/II 合同 (b) 一致） |

## DoD（必跑）

```bash
cargo test -p rv32_priv
```

| 测试 | 覆盖 |
|------|------|
| `tick_mtvec_ecall_mret_golden` | 写 `mtvec` → `ECALL` → handler → `MRET` 回 `mepc` |
| `tick_csr_rmw_and_ie_serialize_golden` | `mscratch` RMW + 写 `mstatus` 后立即可见（IE 串行化） |
| `elaborate_ok` / `emit_verilog_smoke` | elaborate + Verilog 冒烟 |

回归（确认未伤主路径）：

```bash
cargo test -p rv32_core
cargo test -p rv32_pipe
```

## 非目标

- 完整异常/中断优先级、PLIC/CLINT、`mtval`、用户态特权级
- Privileged 规范全文或 arch-test M-mode 绿
- 宣称与 FemtoRV Episode III（常标 WIP）等价

对照阅读（外部，非本仓库 DoD）：RISC-V Privileged M-mode / Zicsr；FemtoRV 中断教程仅作概念参考。
