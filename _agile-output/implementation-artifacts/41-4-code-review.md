# Code Review — Story 41.4 (FR95/FR96 Epic 41 closeout)

**Reviewer:** Composer (adversarial pass; no subagents — parent pipeline)  
**Baseline:** `bb8b256`  
**Date:** 2026-09-09

## Findings

1. **无阻塞缺陷。** README 去掉「不实现树内调度器」；双路径（树内 FR95/FR96 + 外挂可选不得单独满足）落盘；deferred/FR93#1 标 Epic 41 已关；NFR14 关闭条件全勾 + `closed — Story 41.4`；sprint `epic-41: done` / `41-4: done`；`epic-42` 仍 backlog。
2. **外挂诚实路径未回退：** fr35 / README 仍保留 Bambu stub / `BITLOOM_HLS_USE_REAL`；明确 ≠ FR95。
3. **边界：** 未开工 Epic 42+；未扩大树内调度子集；未把 stub 标成 FR95 done。
4. **轻微非阻塞：** 历史风险记录（Epic 24/29/37）仍写「HLS 仅外挂」——故意保留历史门禁；公开 README/AGENTS 已对齐修订 AD-25。

## AC Trace

| AC | Result |
| ---- | ---- |
| 回归 + 文档；NFR14 勾选 Epic 41 关闭 | pass |
| README/deferred 树内非永久非目标；无「不实现树内」 | pass |
| `epic-41: done`；未开工 42+ | pass |

**Approve** — 可标 done；sprint 已 `epic-41: done`。
