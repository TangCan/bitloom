# Code Review: Story 41.1 Epic 41 NFR14 风险记录

**Verdict:** Approve

**Scope:** `nfr14-risk-epic41-in-tree-hls.md` + `nfr14_risk_epic41_in_tree_hls.rs` + story/sprint keys

## Findings

1. **无阻塞缺陷。** 字段 (a)–(d)、树内↔外挂共存、scheduling/allocation crate 范围、FR96↔AD-18 溶解、禁止仅文档假交付、禁止 stub/`BITLOOM_HLS_USE_REAL` 冒充 FR95、禁止 silent 动态数据流默认、负责人（NFR14 / NFR41）、门禁 41.2–41.4 均有正文与 ATDD 覆盖。
2. **与 Epic 40 / 外挂 HLS 体例一致：** 元数据 / 共存表 / FR96·AD-18 / 故事分工 / 关闭条件（留给 41.4）形状正确；未越界实现 41.2–41.4。
3. **前置诚实：** Epic 40 closed + 修订 AD-25 写在上游约束；外挂 FR35/FR86 明确为可选对照而非 FR95 完成面。

## AC Trace

| AC | Result |
| ---- | ------ |
| 共存策略；crate 范围；FR96↔AD-18；禁文档假交付 / 外挂冒充 FR95 / 动态数据流默认；负责人（NFR14 / NFR41） | pass |
| 无记录不得标 41.2–41.4 ready | pass |

## Decision

**Approve** — 可标 done；sprint `epic-41: in-progress`，`41-1: done`；`41-2`/`41-3`/`41-4` 仍 backlog。
