# Episode II 五级流水核（Story 17.4–17.5）

公开品牌 **Bitloom**。与 `samitbasu/rhdl` 无关。设计依赖仅 `bitloom-prelude`。

包：`examples/rv32_pipe`（`EpisodeIIPipe`）。Episode I 单周期绿测仍在 `examples/rv32_core`，本包**不**改写该核。

## 取指合同（FR69 / 17.2）— 继承 (b)

与 [`../rv32_core/SUBSET.md`](../rv32_core/SUBSET.md) 一致：**harness `instr` 口**。禁止片上 SyncReadMem I-fetch。黄金 / 教程按当前 IF 所见 PC（`pc_out`）驱动指令字。

## 本包交付

| 项 | 说明 |
|----|------|
| 五级 | IF/ID/EX/MEM/WB 级间 Reg |
| 转发 | EX/MEM→EX 优先（且 **非** LW），其次 MEM/WB→EX；`rd∈{1..4}` |
| Load-use（17.5） | ID/EX 为 LW 且 IF/ID 源匹配 → 冻结 PC/IF-ID、向 ID/EX 插 bubble；随后 MEM/WB→EX 转发 |
| 分支 | predict-not-taken；taken 时 flush IF/ID 与 ID/EX（NOP/bubble）并 redirect PC |
| 子集 | 同 Episode I（ADDI/ADD/BEQ/LW/SW，x1–x4） |
| 非目标 | **无** CSR/trap（可选 Zicsr/M-trap：**延期**；见教程 [Ch.06](../../docs/tutorials/rv32-episode-ii/06-csr-m-trap-deferred.md)；NFR32 — 不阻塞 Epic 17） |

## 仿真时序（`bitloom-sim`）

1. 每拍 **先 sequential 再 combinational**；级间 next 线对 `instr` 的采样滞后一拍。
2. 级间赋值必须 **下游 Reg 先于上游**（WB←MEM←EX←ID←IF，然后 `pc_f`←`pc`←`next_pc`），因 `RegD` 就地更新。
3. **`pc_f`**：在更新 `pc` 之前锁存当前 PC，供 comb 将 harness `instr` 与取指 PC 对齐。
4. **复位后 arming：** 复位边沿后 comb 仍可能见 `rst=1`（`next_pc` 保持 0）。黄金先 `tick_with` 首条指令，再按 `pc_out` 查 ROM（`rom_tick`）排空至 WB。
5. Stall 用 **mux hold**（非模块级 `en`）：`do_stall` 时 PC/IF-ID 保持，ID/EX 进 bubble；EX/MEM/WB 继续前进。`rs2` 匹配仅对 ADD/BEQ/SW 门控（避免 I-type `imm[4:0]` 伪冲突）。
6. **勿**对 `load_q` 做 MEM→EX 旁路：async DMEM + seq→comb 下同拍可读到 `load_q`，旁路会让无 stall 的 load-use 仍绿，破坏 ATDD。

## 验证阶梯

见 [`../rv32_core/COMPLIANCE.md`](../rv32_core/COMPLIANCE.md)：定向 tick →（可选、已延期）`riscv-tests` `rv32ui`；**不是**完整 DV，**不得**宣称 arch-test 等价。

```bash
cargo test -p rv32_pipe
cargo test -p rv32_core
cargo bitloom build --package rv32_pipe
```

黄金：`tick_clean_path_addi_add_golden`、`tick_alu_alu_raw_forward_golden`、`tick_addi_negative_imm_pipe_golden`、`tick_beq_taken_flush_wrong_path_not_committed`、`tick_load_use_stall_atdd_golden`。
