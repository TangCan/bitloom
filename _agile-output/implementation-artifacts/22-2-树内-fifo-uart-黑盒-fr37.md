---
title: '22.2 树内 FIFO + UART + 黑盒（FR37）'
type: 'feature'
created: '2026-08-21'
status: 'done'
baseline_commit: '80cead2b1e01d2bba0be7dcb77ad71bec1c0e34c'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-22-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-first-class-ip.md'
  - '{project-root}/_agile-output/implementation-artifacts/22-1-nfr14-风险记录-一级-ip.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR37 要求树内至少 FIFO + UART 与一黑盒 wrapper 可 elaborate/emit/tick；既有 `fifo_skel`/`ip_box` 未形成经 `bitloom-prelude` 的一级表面。

**Approach:** 在 `bitloom_prelude::ip` 交付 `SyncFifo`、`UartTx`、`ExtBlackBox` 最小可综合 stub；各 smoke elaborate→emit→tick；`examples/ip_box` 再导出并联验。

## Boundaries & Constraints

**Always:** 设计侧仅 `bitloom-prelude`；三者均有 smoke；黑盒 body 空（不内联）；品牌 Bitloom。

**Ask First:** 无。

**Never:** 交付 SPI/I2C/AXI（留给 22.3–22.5）；全协议栈 UART；静默砍掉黑盒。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| SyncFifo smoke | elaborate→emit→tick + 向量 | data_out 捕获 data_in | panic on fail |
| UartTx smoke | wr_en/wr_data | tx_byte/tx_busy | panic on fail |
| ExtBlackBox | elaborate | body 空 + vendor .v 旁路 | panic on fail |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip.rs` — SyncFifo / UartTx / ExtBlackBox
- `examples/ip_box/` — 再导出与联验
- `examples/fifo_skel/` — 既有 FR22 夹具（保留）

## Tasks & Acceptance

**Execution:**
- [x] `crates/bitloom-prelude/src/ip.rs` -- FIFO + UART + 黑盒 + 单元 smoke -- FR37
- [x] `examples/ip_box` -- 经 prelude::ip 再导出与联验 -- 演示
- [x] 故事文件 / code-review / sprint-status -- 收口

**Acceptance Criteria:**
- Given 22.1 风险记录，when 交付 FIFO/UART/黑盒，then 均可 elaborate→emit→tick（黑盒不透明）
- Given 设计侧，when 依赖，then 仅为 `bitloom-prelude`
- Given 各模块，when 跑测试，then 至少各一 smoke

## Spec Change Log

## Design Notes

UART 为字节保持寄存器 stub（非波特率移位）；FIFO 为 depth-1 skid。黑盒仅端口 + 旁路 vendor Verilog 字符串。

## Verification

**Commands:**
- `cargo test -p bitloom-prelude --lib` -- expected: pass
- `cargo test -p ip_box` -- expected: pass
- `cargo fmt --all && just test` -- expected: pass

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- `bitloom_prelude::ip::{SyncFifo,UartTx,ExtBlackBox}` + smokes
- `ip_box` 改为 prelude 再导出
- 对抗审查 accept

### File List

- `crates/bitloom-prelude/src/ip.rs`
- `crates/bitloom-prelude/src/lib.rs`
- `crates/bitloom-prelude/Cargo.toml`
- `examples/ip_box/src/lib.rs`
- `examples/ip_box/Cargo.toml`
- `_agile-output/implementation-artifacts/22-2-树内-fifo-uart-黑盒-fr37.md`
- `_agile-output/implementation-artifacts/22-2-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-08-21: FR37 FIFO + UART + 黑盒经 bitloom-prelude::ip（Story 22.2）
