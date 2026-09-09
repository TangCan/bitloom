# Code Review: Story 35.1 Epic 35 NFR14 风险记录

**Verdict:** Approve

**Scope:** `nfr14-risk-epic35-residual-partials.md` + `nfr14_risk_epic35_residual_partials.rs` + story/sprint keys

## Findings

1. **无阻塞缺陷。** 字段 (a)–(d)、Partial 现状（Counter-only / SoftF16 host-only / `check_sva_text` toy / LSP deferred）、FR83/84/85 各自「实现 vs 显式 defer」选项表、LSP 非本 epic、禁止 toy 关 FR85、禁止 defer 时声称 SoftF16 可综合、负责人（NFR14/NFR37）、门禁 35.2–35.4 均有正文与 ATDD 覆盖。
2. **禁静默降级写进 (c)：** 「不得用 toy check 关闭 FR85」与「若选 defer 不得声称 SoftF16 可综合」满足 Story 35.1 AC；NFR37 / NFR14-crates 消歧齐全。
3. **轻量建议（不挡合入）：** 35.2/35.3 选型落地后可在选项表旁加「已选 A/B」一行；非本故事范围。

## AC Trace

| AC | Result |
| ---- | ------ |
| FR83/84/85 各实现 vs 显式 defer + LSP 非本 epic + 禁 toy 关 FR85 + 禁 defer 称 SoftF16 可综合 + 负责人（NFR14/NFR37） | pass |
| 无记录不得标 35.2–35.4 ready | pass |

## Decision

**Approve** — 可标 done；sprint `epic-35: in-progress`，`35-1: done`。
