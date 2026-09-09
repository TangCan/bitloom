# Code Review: Story 33.2 Mem→Chisel 合同决策（FR81）

**Verdict:** Approve

**Scope:** `fr81-mem-chisel-contract-decision-2026-09-09.md` + `fr81_mem_chisel_contract_decision.rs` + addendum/epics/NFR14 回链 + story/sprint keys

## Findings

1. **无阻塞缺陷。** 唯一路径 **Path A** 明确采纳并拒绝 Path B；支持形态（单时钟 `Mem`/`SyncReadMem` + 可选 init）、NFR12 钉死对（7.14.0 ↔ 1.155.0）、NFR37、子集外保留 E0901、禁破坏 FR71 均有正文与 ATDD。
2. **AD-27 / AD-21 / AD-9 对齐：** Path A 服务可编译 Chisel 产品腿；子集压在既有 HIR `MemDecl` 表面；版本不漂移。本故事正确**未**实现降级或删除 E0901。
3. **轻量建议（不挡合入）：** 33.3 实现时可在 `language-surface.md` Mem 节把「仍可 E0901」改为「子集内可编译 / 子集外 E0901」；非本故事范围。

## AC Trace

| AC | Result |
| ---- | ------ |
| 决策写入注记并链接 epic；选定唯一路径 | pass（Path A；epics/addendum/NFR14 回链） |
| 若非目标：替代验收 | N/A（未选 Path B） |
| 若支持子集：形态 + NFR12 | pass |
| 显式引用 NFR37 | pass |

## Decision

**Accept** — 可标 done；`epic-33` 保持 in-progress；提交由父代理执行。
