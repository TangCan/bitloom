# Code review — Story 17.4

**Verdict: approve**

## Findings

1. **(pass)** 新包 `examples/rv32_pipe`：`EpisodeIIPipe` 含 IF/ID/EX/MEM/WB 级间 Reg；设计依赖仅 `bitloom-prelude`。
2. **(pass)** 取指 (b) harness `instr`；无 SyncReadMem I-fetch；`PIPE.md` / `SUBSET.md` / `COMPLIANCE.md` 交叉说明。
3. **(pass)** EX/MEM→EX 优先（且非 LW）、MEM/WB→EX 次之；`rd∈{1..4}`；seq 下游先于上游。
4. **(pass)** predict-not-taken；taken 时 flush IF/ID 与 ID/EX 并 redirect；错误路径 ADDI 不提交。
5. **(pass)** 黄金：`tick_clean_path_addi_add_golden`、`tick_alu_alu_raw_forward_golden`、`tick_addi_negative_imm_pipe_golden`、`tick_beq_taken_flush_wrong_path_not_committed`。
6. **(pass)** `cargo test -p rv32_pipe`、`cargo test -p rv32_core`、`cargo bitloom build --package rv32_pipe` 绿。
7. **(info)** `pc_f` 在更新 `pc` 前锁存取指 PC，避免 seq→comb 下 `if_id_pc` 与 `instr` 错位。
8. **(info)** BEQ 紧邻生产者：比较走 `fwd_rs*`；clean path 用 NOP 间距避免依赖转发。
9. **(info)** LW/SW 接线但本故事不测 load；load-use → 17.5；EX/MEM 不转发 LW 的 EA。

## Review-loop patches applied

- 门控 EX/MEM 转发：`!ex_mem_is_lw`
- `pc_plus4` 经 `mask32`
- clean path 间距；负向 ADDI 黄金；BEQ 测例改走转发比较
- `PIPE.md` 补充 arming / LW 转发合同

No blocking defects.
