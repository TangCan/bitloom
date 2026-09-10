# Code Review: Story 58.2 Typed IDE 波形路径实现与验收（FR117）

**Verdict:** Approve

**Scope:** `typed-wave.html` / `wave.typed.json` + `rhdl-viz` typed API + `cargo bitloom wave` + `docs/fr117-*` + ATDD

**Review mode:** adversarial self-review（嵌套 subagent 不可用；对齐 Epic 47.2 / 56.2 产品路径审查口径）

## Findings

1. **无阻塞缺陷。** 子集 B 产品路径可复现：`typed_signals_from_hir` → `typed-wave.html`（`data-bitloom-typed-wave`）+ `wave.typed.json`；CLI 一等写出；缺 typed 元数据失败码 `bitloom.typed-wave-empty`。
2. **超出 I1–I3：** 类型/kind/module 层级 + 选中元数据面板，明确 ≠ FR104 名字符串 timeline；文档与 ATDD 守卫齐全。
3. **NFR48：** 同次 `wave` 仍写 VCD / `interactive.html` / `timing.html`；coverage CLI 未改。
4. **NFR51：** Tywaves (A) 在 fr117/fr114/NFR14 仍标 deferred；未 silent 宣称。
5. **范围诚实：** 未勾选 Epic 58 / FR117 关闭；`58-3` 保持 backlog。
6. **已知 MVP 边角（不阻塞）：** 多模块同名端口在 typed 列表中可能重复；VCD 仍扁平 dump 第一模块——符合风险记录「结构化类型可观察」MVP，非实例路径展开。

## AC Trace

| AC | Result |
| ---- | ------ |
| 子集 B 产品路径 + typed ≠ I1–I3 | pass |
| Bitloom + VCD / interactive / FR114 LCOV 仍可用 | pass |
| A deferred；不关 58.3 | pass |

## Decision

**Approve** — 可标 done；sprint `58-2: done`；`58-3` backlog；`epic-58` in-progress。
