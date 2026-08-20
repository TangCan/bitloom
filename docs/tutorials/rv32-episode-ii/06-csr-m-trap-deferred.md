# Chapter 6 — 可选 Zicsr + M-mode trap（延期 stub）

**状态：延期（deferred）。** 本页是教学大纲占位，**不是**可 `tick` 的 CSR RTL 合同。

## 为何延期（NFR32）

Epic 17 已交付五级 + 转发 + load-use + 分支 flush（`examples/rv32_pipe`）。  
**CSR/trap 不得阻塞**流水 DoD；本会话优先交付教程主线，避免半成品特权路径。

Epic 17 / 18.2 **仍视为完成**，不因本章延期回溯判定失败。

## 若日后实现：最小教学集（FR65）

| 项 | 说明 |
|----|------|
| CSR | `mstatus` / `mtvec` / `mepc` / `mcause` / `mscratch`（开中断再加 `mie`/`mip`；`mtval` 可后补） |
| 指令 | CSR 读写 + `mret`；trap 写 mepc/mcause/mstatus 并跳 `mtvec` |
| Flush | 写影响中断使能的 CSR 后必须 flush / 串行化（防 interrupt skid） |
| 禁止 | **不以** PicoRV32 自定义 IRQ 为标准模板 |
| 合规 | 目标「能教 / 能跑 mret」；**除非另文声明**不得宣称 Privileged / arch-test M-mode 合规 |
| 依赖 | 设计仅 `bitloom-prelude` |

## 当前仓库标记

- `examples/rv32_pipe/PIPE.md` — 非目标含 **无 CSR/trap**；可选延期见本页
- `examples/rv32_core/SUBSET.md` — No CSR/ECALL/…；可选 Zicsr 标延期
- `examples/rv32_core/COMPLIANCE.md` — 定向黄金；无 CSR 门禁

## 验收（本 stub）

阅读本页 + `PIPE.md` / `SUBSET.md` 延期注记。**无**必跑 CSR 测试。

对照阅读（外部）：RISC-V Privileged 规范 M-mode / Zicsr；FemtoRV Episode III（常标 WIP）——仅参考，非本仓库 DoD。
