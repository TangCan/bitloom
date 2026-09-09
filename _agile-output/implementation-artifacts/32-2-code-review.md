# Code Review: Story 32.2 嵌套 Bundle 可综合路径

**Verdict:** Approve

**Scope:** `Bundle::nested_bundles` flatten、夹具嵌套 emit/tick、宽/向负例、language-surface / prelude 文档、FR80 ATDD

## Findings

1. **无阻塞缺陷。** 一层嵌套经 `nested_bundles` → `{field}_{nested}_{leaf}` 标量 HIR；夹具 elaborate/emit/tick；嵌套叶 E0131/E0112 emit 前失败；文档不再仅以 nested OUT OF SCOPE 交差；`HwVec<Bundle,_>` / derive 仍诚实 OOS。
2. **AD-20 对齐：** 未扩展公开 HIR Bundle 节点；沿用 FR51 flatten；宽/向门禁复用。
3. **轻量建议（不挡合入）：** ≥2 层递归 flatten、用户文档限制表收口留给 32.4；derive 留给 32.3。

## AC Trace

| AC | Result |
| ---- | ------ |
| 至少一层嵌套 Bundle → HIR → emit `.v`；夹具 elaborate/emit/tick | pass |
| 嵌套字段宽/向不匹配 emit 前失败 | pass |
| nested 不再仅以 OUT OF SCOPE 交差（NFR37） | pass |

## Decision

**Approve** — 可标 done；sprint 保持 `epic-32: in-progress`，`32-2: done`。
