# FemtoRV / Bitloom Episode I 对照（Ch.4–6）

| 阶段 | FemtoRV `FROM_BLINKER_TO_RISCV` | Bitloom Episode I |
|------|--------------------------------|-------------------|
| ALU / 立即数 | 逐步加算术与 I-type | Ch.4：`ADDI`/`ADD`；`tick_addi_then_add_golden` |
| 分支 | 引入条件跳转与 PC | Ch.5：`BEQ` + B-imm 位域；`tick_beq_taken_jumps_plus8` |
| 访存 / 外设 | load/store 与 LED/MMIO | Ch.6：`LW`/`SW` + LED@`0x100`；`tick_sw_mmio_led_golden` |
| 波形 | 板级 / 仿真器波形 | 可选：`Sim` VCD（`bitloom-sim`）；本目录不强制贴图 |

FemtoRV 原文：<https://github.com/BrunoLevy/learn-fpga/blob/master/FemtoRV/TUTORIALS/FROM_BLINKER_TO_RISCV.md>

Bitloom 主路径仍是 `cargo test` / `cargo bitloom build`，不以 Make/SBT 为必装。
