# Episode II 大纲

本页**不是**核实现合同；指向已交付教程与示例包。

## 状态（Epic 17 + 18）

| 主题 | 状态 | 入口 |
|------|------|------|
| 用户态立即数 / 符号扩展 | **已实现**（17.3） | `examples/rv32_core` + [Episode II Ch.01](../rv32-episode-ii/01-isa-and-imm.md) |
| 经典 5 级 + 转发 + load-use + 分支 flush | **已实现**（17.4–17.5） | `examples/rv32_pipe` + [Episode II 教程](../rv32-episode-ii/README.md) |
| 可选 Zicsr + M-mode trap | **可选已实现**（FR65） | `examples/rv32_priv` + [Ch.06](../rv32-episode-ii/06-csr-m-trap.md) |

逐步章节：[`docs/tutorials/rv32-episode-ii/`](../rv32-episode-ii/README.md)。

## 非目标（仍适用）

- 无 cache / MMU / Linux Softcore
- 无动态 BTB；VexRiscv **仅对照**
- 不以 PicoRV32 自定义 IRQ 为特权模板
- 定向黄金 ≠ arch-test / 完整 DV（见 `COMPLIANCE.md`）

## 外部阅读

- Harris DDCA RISC-V Ch.7（流水章节）
- FemtoRV 教程后半（流水 / hazard）

Episode I 以 `examples/rv32_core` + `rv32-episode-i` 00–07 为完成定义；缺少 CSR **不**表示 Episode I 或 Epic 17 未完成。
