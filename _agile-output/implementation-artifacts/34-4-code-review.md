# Code Review: Story 34.4 IP 基线 ATDD + Epic 29 衔接文档

**Verdict:** Approve

**Scope:** `fr82_ip_baseline_matrix` consolidating ATDD + `docs/ip` Epic 29 handoff + NFR14 Epic 34 close checklist；no new RTL

## Findings

1. **ATDD 汇总充分。** 矩阵断言 sibling `fr82_*_baseline` 存在、五类 elaborate→emit、五类 elaborate 段无 `Fn`/`dyn Fn`，并锁住 docs/NFR14 措辞；深度 tick 金标仍在 34.2/34.3 夹具。
2. **Epic 29 handoff 清晰。** `docs/ip/README.md` 表格钉死：Epic 34 = 无闭包基线；Epic 29 = 闭包定制 overlay（29.3）；Epic 22 stub vs NFR37。
3. **NFR14 关闭条件已勾选。** `nfr14-risk-epic34-ip-baseline.md` 含「Epic 34 关闭条件」清单且全部 `[x]`；Epic 29 风险记录注明 FR82 基线已交付。
4. **轻量建议（不挡合入）：** 全量 `just test` 未在本会话跑（用户约束）；文档化配方与矩阵已覆盖 FR82 回归面。

## AC Trace

| AC | Result |
| ---- | ------ |
| ATDD 覆盖已交付 IP；配方/`just test` 路径稳定（FR82） | pass（matrix + siblings；配方见 docs/ip） |
| 文档：本 epic 无闭包；Epic 29 叠加；vs stub（NFR37） | pass |
| NFR14 勾选 Epic 34 关闭条件 | pass |

## Decision

**Approve** — 可标 done；sprint `34-4-ip-基线-atdd-epic-29-衔接文档: done`，`epic-34: done`。

## Verified recipe

```text
cargo test -p bitloom --test fr82_ip_baseline_matrix \
  --test fr82_fifo_uart_baseline --test fr82_spi_i2c_axi_baseline
```
