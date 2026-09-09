# Code Review: Story 39.3 浅层 Bitloom LSP 或显式 defer（FR91）

**Verdict:** Approve  
**Date:** 2026-09-09  
**Baseline:** `c6ac3f0` (39.2)

## Findings

1. **无阻塞缺陷。** Path B 合同落在 `docs/fr38-viz-lsp.md` + README；NFR14 `- [x] **FR91：**`；FR92 / 整 epic 未关；无 `language-server` / `bitloom-lsp` crate；HTML ≠ LSP 明确。
2. **边界诚实：** 宿主 FR90 ≠ 自研 LSP；层次/时序 HTML 不计入 FR91；半成品 LSP 二进制有 ATDD 守卫。
3. **范围：** `39-4` 仍 backlog；未开工 FR92；fr90 范围守卫已改为只锁 39.4/FR92。
4. **ATDD：** `fr91_bitloom_lsp_explicit_defer` 6 绿；`fr90_host_ide_rust_analyzer` 6 绿；`nfr14_risk_epic39` 仍绿。

## AC Trace

| AC | Result |
| ---- | ---- |
| Path B：README / fr38 显式 deferred；不得声称 LSP 已交付；FR91 可勾选 | pass |
| 无半成品 language-server 二进制 | pass |
| 层次/时序 HTML 不计入 LSP | pass |
| 不开工 39.4；不关整 epic；不交付分支 A | pass |

**Approve** — 可标 done；`39-4` 仍 backlog。
