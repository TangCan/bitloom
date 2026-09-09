# Code Review: Story 40.4 修订 AD + 撤销 FR93 锁定（FR94 / NFR41）

**Date:** 2026-09-09  
**Reviewer:** Composer (pipeline)  
**Scope:** ARCHITECTURE-SPINE AD-5/25/27 + Deferred；README/deferred FR93 重接线；NFR14 Epic 40 关闭；`fr94_ad_fr93_unlock` + `fr93_permanent_non_goals` 历史化；sprint `epic-40: done`

## Findings

1. **AC 满足：** AD-5 允许 FR101 TLM 产品路径；AD-25 允许 FR95 树内调度（外挂可选）；AD-27 增加 FR97 idiomatic 面且保留机械 FR28/46；Deferred 指向 FR94–105。
2. **FR93 解锁正确：** README/deferred 不再以「须新 PRD」为当前锁；五条→Phase 12 FR 映射；历史可追溯保留。
3. **边界正确：** Epic 41–47 仍 `backlog`；未开工实现；未冒充字面 HLS/LSP/Chisel/TLM 已交付。
4. **NFR14：** Epic 40 关闭条件全勾；状态 `closed`；NFR41 写进修订 AD。
5. **回归：** `fr94_prd_path_b_gate`、`fr94_doc19_literal_green`、改写后 `fr93_*`、新 `fr94_ad_fr93_unlock` 绿。

## Verdict

**Approve** — 可标 done；sprint `epic-40: done`，`40-4: done`；Epic 41–47 仍 backlog。
