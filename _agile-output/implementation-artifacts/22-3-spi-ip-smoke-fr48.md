---
title: '22.3 SPI IP smoke（FR48）'
type: 'feature'
created: '2026-08-21'
status: 'done'
baseline_commit: 'd898971f0f064660416f2a0c9deb780126b1aa36'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-22-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/22-2-树内-fifo-uart-黑盒-fr37.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR48 五类中的 SPI 尚无可例化 smoke。

**Approach:** 在 `bitloom_prelude::ip` 增加 `SpiMaster`（主设备、最小字节缓冲端口），文档钉死角色与非目标；smoke elaborate→emit→tick。

## Boundaries & Constraints

**Always:** 经 `bitloom-prelude`；非空壳；文档说明限制；品牌 Bitloom。

**Ask First:** 无。

**Never:** 全协议栈 SPI；I2C/AXI（22.4/22.5）。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected | Error Handling |
|----------|--------------|----------|----------------|
| smoke | start+tx_data | mosi_byte/busy | panic |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip.rs` — `SpiMaster`

## Tasks & Acceptance

**Execution:**
- [x] `SpiMaster` + smoke + 文档限制 -- FR48 SPI
- [x] code-review / sprint-status / 故事文件 -- 收口

**Acceptance Criteria:**
- Given 22.2 模式，when 交付 SPI，then elaborate→emit→tick 通过
- Given 文档，when 阅读，then 钉死 master 角色与非全栈限制

## Spec Change Log

## Design Notes

Master-only byte buffer；非 CPOL/CPHA/多 CS。

## Verification

**Commands:**
- `cargo test -p bitloom-prelude --lib spi_master` -- expected: pass
- `cargo fmt --all && just test` -- expected: pass

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- `SpiMaster` + smoke；对抗审查 accept

### File List

- `crates/bitloom-prelude/src/ip.rs`
- `_agile-output/implementation-artifacts/22-3-spi-ip-smoke-fr48.md`
- `_agile-output/implementation-artifacts/22-3-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-08-21: FR48 SPI master stub smoke（Story 22.3）
