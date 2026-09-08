# Code Review: Story 34.2 FIFO + UART 可综合基线

**Verdict:** Approve

**Scope:** `bitloom_prelude::ip::{SyncFifo,UartTx}` FR82 deepen + `Sim::settle` + `fr82_fifo_uart_baseline` ATDD + `docs/ip` / language-surface / deferred-work + `examples/ip_box`

## Findings

1. **非 stub 证据充分。** SyncFifo 为 depth-4 + `wr_en`/`rd_en`/`full`/`empty`（非 depth-1 skid）；UartTx 为 8N1 `tx` 帧 + busy 门控写（非纯保持寄存器）。均有 elaborate→emit `.v`→tick 夹具。
2. **无生成器闭包 API。** `Elaboratable::elaborate()` 无 `Fn`；ATDD 扫描 SyncFifo/UartTx 实现段。闭包留给 Epic 29。
3. **黑盒边界文档化。** `ExtBlackBox` 空 body + `vendor_blackbox_v`；`docs/ip/README.md` 与 language-surface 写明「不内联 vendor HIR」。
4. **轻量建议（不挡合入）：** 全局 `tick` 仍为 seq→comb；同周期门控依赖 `settle`——已在 IP 文档与夹具写清；未来可考虑 comb-seq-comb（非本故事）。

## AC Trace

| AC | Result |
| ---- | ------ |
| SyncFifo + UartTx 非 stub；各至少一 elaborate→emit→tick | pass |
| API 不接受生成器闭包 | pass |
| 黑盒 wrapper 行为/边界文档化 | pass |

## Decision

**Approve** — 可标 done；sprint `34-2-fifo-uart-可综合基线: done`。
