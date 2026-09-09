# Code review — Story 38.2

**Scope:** `UartTx` programmable `baud_div` + FR89 ATDD + docs/IP surface  
**Baseline:** `d8985b3` (Story 38.1)

## Findings

1. **无阻塞缺陷。** 分支 A 语义清晰：`baud_div` = clk/bit−1，`0` 保持 FR82 1 bit/clk；`baud_cnt`/`baud_tick` 门控帧推进与移位；emit 含 `baud_div`；ATDD + prelude 单元测绿。
2. **边界诚实：** docs/ip + rustdoc 写明非目标（RX / 全双工 / VIP / 全协议）；无 `UartRx`；未勾选 NFR14 Epic 38 关闭条件；38.3 仍 backlog。
3. **回归：** `fr82_fifo_uart_baseline` / `fr78_bridge_*` / `fr82_ip_baseline_matrix` / `nfr14_risk_epic38_*` / `ip_box` 在默认 `baud_div=0` 下通过。
4. **非问题：** ABI 增端口 `baud_div` 属合同加深；未驱动输入 sim lookup→0，与 FR82 夹具兼容。

| AC | Result |
| --- | --- |
| elaborate→emit→tick 加深语义 | pass |
| docs 交付子集 + 明确非目标 | pass |
| 不得声称分支 B（RX） | pass |
| 设计只依赖 bitloom-prelude | pass |
| 未开工 38.3 / 未勾关闭条件 | pass |

**Approve** — 可标 done；sprint `38-2: done`；`38-3` 仍 backlog。
