# FemtoRV / Harris / Bitloom Episode II 对照

对照阅读用；**不**要求板级截图或二进制波形资产。主路径仍是 `cargo test -p rv32_pipe`。

| 主题 | Harris DDCA (RISC-V) Ch.7 | FemtoRV Episode II（流水后半） | Bitloom Episode II |
|------|---------------------------|-------------------------------|--------------------|
| 五级插入 | 级间寄存器 / 左→右 datapath | 流水寄存器与时序 | [Ch.02](./02-five-stage.md)；`tick_clean_path_addi_add_golden` |
| ALU 转发 | RAW；EX/MEM、MEM/WB 旁路 | 转发 mux | [Ch.03](./03-forwarding.md)；`tick_alu_alu_raw_forward_golden` |
| Load-use | hazard 检测 + stall + 再转发 | stall / bubble | [Ch.04](./04-load-use.md)；`tick_load_use_stall_atdd_golden`、`tick_load_use_rs2_consumer_atdd_golden` |
| 分支 | predict-not-taken；flush 错路径 | 冲刷 / redirect | [Ch.05](./05-branch-flush.md)；`tick_beq_taken_flush_wrong_path_not_committed` |
| CSR / trap | Privileged（书后半） | 视教程阶段 | [Ch.06](./06-csr-m-trap.md) **可选已实现**（`rv32_priv`；不阻塞流水 DoD；非合规） |

## 可选：从 `bitloom-sim` dump VCD

教学黄金不强制波形。若本地对照 GTKWave：

```rust
use bitloom_sim::Sim;
// ...
let mut sim = Sim::new(EpisodeIIPipe::elaborate().unwrap());
sim.enable_vcd("episode-ii.vcd")?;
// ... tick ...
sim.finish_waves()?; // 若启用了可选 FST；纯 VCD 也可在 drop 前保持文件打开
```

细节见仓库 [`docs/fr31-optional-fst.md`](../../fr31-optional-fst.md)（默认 VCD；FST 为 `vcd2fst` 可选转换）。本目录**不**附截图。

## 链接

- FemtoRV：<https://github.com/BrunoLevy/learn-fpga/blob/master/FemtoRV/TUTORIALS/FROM_BLINKER_TO_RISCV.md>
- Episode I 对照：[`../rv32-episode-i/femtorv-compare.md`](../rv32-episode-i/femtorv-compare.md)
- 核合同：[`examples/rv32_pipe/PIPE.md`](../../../examples/rv32_pipe/PIPE.md)
