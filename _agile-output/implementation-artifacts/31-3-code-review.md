# Code Review: Story 31.3 SyncFIFO（或等价）可综合跨域 FIFO

**Verdict:** Approve

**Scope:** prelude `SyncFIFO::<4,8>` Elaboratable（灰码指针 + DoubleFlop 跨域同步 + mem/full/empty）、`examples/syncfifo_skel`、`fr79_syncfifo_rtl`、`docs/fr79-syncfifo-cdc.md`、language-surface / IP README 消歧；FR82 `ip::SyncFifo` 未改行为

## Findings

1. **无阻塞缺陷。** prelude → HIR → `.v` 含 `ram` + `w2r_ff*`/`r2w_ff*` + `full`/`empty`；跨域写/读黄金与满语义 ATDD 绿；E0220 负例保留；非空 ZST（NFR37）。
2. **AD-29 / NFR14 边界对齐：** DEPTH/WIDTH/指针同步延迟文档化；单物理 clk + phantom 双域 MVP 写清；≠ `ip::SyncFifo`；非 MTBF。
3. **FR82 未破：** `fr82_fifo_uart_baseline` + prelude `sync_fifo` 仍绿；模块名 `SyncFIFO` ≠ `SyncFifo`。
4. **轻量建议（不挡合入）：** Story 31.4 收口时勾选 NFR14 Epic 31 SyncFIFO 关闭项，并刷新风险记录「crates 现状」段（今日已非双侧 ZST）。

## AC Trace

| AC | Result |
| ---- | ------ |
| SyncFIFO elaborate/emit/tick；深度/宽度文档化；跨域写/读 + 满/空（最小子集） | pass |
| 未标记非法跨域仍 E0220 | pass |
| Verilog 抽检非空同步/FIFO 结构 | pass |
| 不破坏 `ip::SyncFifo` FR82 基线 | pass |

## Decision

**Approve** — 可标 `31-3: done`；保持 `epic-31: in-progress`。
