# Story 15.2: Episode I 核 — 译码/ALU/寄存器与 tick

Status: done

## Story

As a 学习者/维护者,
I want 一个裁剪 RV32I 的 Episode I 核能执行 ALU/分支类指令并在 `cargo test` 里 tick 对齐黄金值,
So that 教学核在仿真路径上「真的在跑」。

## Acceptance Criteria

见 `epics.md` Story 15.2（FR56/FR58/NFR24）。

## Tasks / Subtasks

- [x] 扩展 HIR/`tick`/emit：`Sub`/`And`/`Or`/`Xor`/`Shl`/`Shr`（译码位域）
- [x] `examples/rv32_core`：ADDI/ADD/BEQ，x0–x4，`elaborate` + tick 黄金
- [x] `SUBSET.md` 声明子集与非目标
- [x] ATDD = 包内测试；code-review 笔记

## Dev Notes

- 设计依赖仅 `bitloom-prelude`；`bitloom-sim` 仅 dev-dep。
- `instr` 由测试夹具驱动；访存/MMIO → 15.3。
- BEQ 教学偏移固定 +8（完整 B-imm 可后置）。
