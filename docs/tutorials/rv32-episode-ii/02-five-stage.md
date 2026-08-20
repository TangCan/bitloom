# Chapter 2 — 五级流水插入

一步一变：把单周期 datapath 拆成 IF → ID → EX → MEM → WB，级间用 Reg；**先不**讲转发 / stall / flush。

## 引入

包：`examples/rv32_pipe`（`EpisodeIIPipe`）。

要点（详 `PIPE.md`）：

- 级间 IF/ID、ID/EX、EX/MEM、MEM/WB
- `bitloom-sim`：**下游 Reg 先于上游**赋值（WB←MEM←EX←ID←IF）
- `pc_f` 对齐 harness `instr` 与取指 PC
- 取指合同仍是 (b) harness `instr`

本集「干净路径」黄金故意拉开指令间距，使 RF 已提交后再用，避免依赖转发。

## 验收

```bash
cargo test -p rv32_pipe elaborate_ok
cargo test -p rv32_pipe tick_clean_path_addi_add_golden
```

类型：`elaborate` + `tick`。
