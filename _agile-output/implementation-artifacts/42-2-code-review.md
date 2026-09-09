# Code Review: Story 42.2 Idiomatic Chisel 发射与验收（FR97）

**Verdict:** Approve

**Scope:** `emit_chisel_idiomatic` / `check_idiomatic_chisel` + `docs/fr97-idiomatic-chisel.md` + fr28 交叉 + ATDD

## Findings

1. **无阻塞缺陷（修补后）。** 正例层次夹具满足命名/结构/HIR 驱动分节；机械 `emit_chisel` 与剥除分节均以 `rhdl::E0904` 可读失败；机械头不再嵌入 `FR97`。
2. **文档区分到位：** `docs/fr97-idiomatic-chisel.md` vs `fr28`「可编译 ≠ idiomatic」+ FR97 交叉链；未勾选 NFR14 Epic 42 关闭、未改 README/deferred 永久非目标列表（留给 42.3）。
3. **Deferred（非阻断）：** 按模块块作用域硬化端口校验；空电路拒绝策略。

## AC Trace

| AC | Result |
| ---- | ------ |
| 至少一夹具满足 idiomatic 验收条 + 自动化断言 | pass |
| 文档区分机械 vs FR97 | pass |
| 负向/边界可读失败或显式降级 | pass |
| 品牌 Bitloom | pass |

## Decision

**Approve** — 可标 done；sprint `epic-42: in-progress`，`42-2: done`；`42-3` 仍 backlog。
