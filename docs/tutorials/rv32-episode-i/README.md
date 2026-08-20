# Bitloom Episode I：教学 RV32 教程

公开产品名 **Bitloom**（`bitloom`）。与 [samitbasu/rhdl](https://github.com/samitbasu/rhdl) 无关。

## 范围与非目标

- **范围：** 裁剪 RV32I 教学核（`examples/rv32_core`）+ 本目录逐步教程。
- **非目标：** 无 SoC / MMU / Linux；流水线 / 可选 CSR 见 [Episode II](../rv32-episode-ii/README.md)（大纲：[99](./99-episode-ii-outline.md)）。

## 章节索引

| 章 | 主题 | DoD |
|----|------|-----|
| [00](./00-getting-started.md) | 工具链与真独立跟练 | 能 `cargo bitloom` / `bitloom-sim` |
| [01](./01-blink-and-rom.md) | blink / ROM 概念 | 理解 `instr` 输入 |
| [02](./02-decode.md) | 译码 | `elaborate_ok` |
| [03](./03-regs-control.md) | 寄存器与控制 | tick 见 `x*_out` |
| [04](./04-alu.md) | ALU | `tick_addi_then_add_golden` |
| [05](./05-branch.md) | 分支 | `tick_beq_taken_jumps_plus8` |
| [06](./06-load-store-mmio.md) | 访存与 MMIO | `tick_sw_mmio_led_golden` |
| [对照](./femtorv-compare.md) | FemtoRV Ch.4–6 对照表 | 阅读 |
| [07](./07-asm-and-c.md) | 手写 asm / C（简化） | 编码助手 + 外部汇编对照 |
| [99](./99-episode-ii-outline.md) | Episode II 入口 | 指向已实现教程 / 延期 CSR |

## 延伸阅读

- Harris & Harris, *Digital Design and Computer Architecture: RISC-V Edition*, Ch.7
- FemtoRV [`FROM_BLINKER_TO_RISCV`](https://github.com/BrunoLevy/learn-fpga/blob/master/FemtoRV/TUTORIALS/FROM_BLINKER_TO_RISCV.md)
- 对照：PicoRV32、SERV（bit-serial）
- **不要**把 VexRiscv / Linux Softcore 教程当作第一路径

子集与合规：`examples/rv32_core/SUBSET.md`、`COMPLIANCE.md`（最小过滤器，非完整 DV）。
