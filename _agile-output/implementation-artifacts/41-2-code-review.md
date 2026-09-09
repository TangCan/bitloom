# Code Review: Story 41.2 树内 #[hls] 调度 MVP（FR95）

**Verdict:** Approve

**Scope:** `bitloom::hls` in-tree schedule API + CLI `--in-tree` + `docs/fr35-hls.md` + `fr95_in_tree_hls_schedule` ATDD

## Findings

1. **无阻塞缺陷。** `schedule_in_tree` / `emit_in_tree_schedule` 对 `LoopUnroll` 产出含 `fr95` / `loop-unroll` / `trip_count` / stages 的 schedule IR 与诚实 `in-tree-mvp` RTL stub；**不**调用 `resolve_bambu` / spawn Bambu。`trip_count=0` 可读失败。
2. **AC 诚实：** 文档化子集为 loop-unroll（另有 Pipeline 枚举）；文档明确树内 = FR95 完成面、外挂不得单独满足；品牌 Bitloom。外挂 `emit_c_stub` / `run_hls*` 保留。
3. **边界：** 未实现 FR96（41.3）；未勾选 Epic 41 关闭（41.4）；未引入 Handshake/动态数据流默认。非商业 HLS——产物标注诚实。
4. **轻微非阻塞：** Wave 0 `closure-decision-table` / Epic 24 `nfr14-risk-hls` 历史「禁树内」文案仍在——故意保留历史门禁 ATDD；FR95 由新文档节 + 新 ATDD 证明。可在 41.4 收口时交叉更新指针。

## AC Trace

| AC | Result |
| ---- | ------ |
| 树内 loop-unroll/pipeline 子集 + 可检查 IR；不调用 Bambu | pass |
| 文档：树内 FR95；外挂可选且不得单独满足 | pass |
| 品牌 Bitloom | pass |

## Decision

**Approve** — 可标 done；sprint `41-2: done`；`41-3`/`41-4` 仍 backlog。
