# Code Review: Story 37.1 Epic 37 NFR14 风险记录

**Verdict:** Approve

**Scope:** `nfr14-risk-epic37-interop-hls.md` + `nfr14_risk_epic37_interop_hls.rs` + story/sprint keys

## Findings

1. **无阻塞缺陷。** 字段 (a)–(d)、firtool/Chisel 钉死对漂移、机械≠idiomatic 误读、夜间真机 vs stub、禁止私自升 firtool、禁止 stub 写成 HLS 质量已验、禁止 `continue-on-error`、负责人（NFR14/NFR12/NFR39）、门禁 37.2–37.3 均有正文与 ATDD 覆盖。
2. **与 Epic 36 体例一致：** 元数据 / 主题对照表 / 故事分工 / 关闭条件（留给 37.3）形状正确；未越界实现 37.2–37.3。
3. **轻量建议（不挡合入）：** Story 37.3 勾选关闭条件时可同步把元数据状态改为 `closed`；非本故事范围。

## AC Trace

| AC | Result |
| ---- | ------ |
| firtool/Chisel 漂移；机械误读 idiomatic；夜间真机 vs stub；禁私自升 firtool；禁 stub 冒充 HLS 质量；禁 `continue-on-error`；负责人（NFR14/NFR12/NFR39） | pass |
| 无记录不得标 37.2–37.3 ready | pass |

## Decision

**Approve** — 可标 done；sprint `epic-37: in-progress`，`37-1: done`。
