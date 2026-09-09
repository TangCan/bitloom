# Code Review: Story 31.2 DoubleFlop 可综合同步器 RTL

**Verdict:** Approve

**Scope:** prelude `DoubleFlop` Elaboratable、builder `declare_double_flop_stages` / CDC on `assign_reg_d_from`、sim RegD NBA、`examples/doubleflop_skel`、`fr79_doubleflop_rtl`、language-surface / `docs/fr79-doubleflop-cdc.md`

## Findings

1. **无阻塞缺陷。** prelude → HIR → `.v` 含 `sync_ff0`/`sync_ff1` + `always @(posedge`；黄金延迟 2 目的域 tick；E0220 负例保留；非空 ZST 网表（NFR37）。
2. **AD-29 / NFR14 边界对齐：** 级数=2、延迟合同、非 MTBF 写清；SyncFIFO 真 RTL 明确留给 31.3。
3. **sim NBA：** RegD 先算后提交，双 FF 延迟不折叠；pipeline / FIFO 回归绿。
4. **轻量建议（不挡合入）：** Story 31.4 收口时可勾选 NFR14 Epic 31 DoubleFlop 关闭项，并统一 `clockdomain_skel` 文案指向 FR79 夹具。

## AC Trace

| AC | Result |
| ---- | ------ |
| DoubleFlop prelude→HIR→emit 两级 sync reg；夹具 elaborate/emit/按域 tick 黄金延迟 | pass |
| 非法未同步跨域仍 E0220 | pass |
| 不得仅空 ZST 无网表（NFR37） | pass |

## Decision

**Approve** — 可标 `31-2: done`；保持 `epic-31: in-progress`。
