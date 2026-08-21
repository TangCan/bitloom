---
title: '22.4 I2C IP smoke（FR48）'
type: 'feature'
created: '2026-08-21'
status: 'done'
baseline_commit: '895bd5e1f6c53d9b238df7c9135bff4fb2f23fe7'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-22-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/22-3-spi-ip-smoke-fr48.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR48 I2C 类尚无可例化 smoke。

**Approach:** `bitloom_prelude::ip::I2cMaster`（主设备、最小字节缓冲）；文档钉死范围与非目标；smoke elaborate→emit→tick。

## Boundaries & Constraints

**Always:** 经 prelude；文档说明范围；品牌 Bitloom。

**Ask First:** 无。

**Never:** 全协议栈；AXI（22.5）。

## I/O & Edge-Case Matrix

| Scenario | Input | Expected | Error Handling |
|----------|-------|----------|----------------|
| smoke | start+tx_data | tx_byte/busy | panic |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip.rs` — `I2cMaster`

## Tasks & Acceptance

**Execution:**
- [x] I2cMaster + smoke + 文档 -- FR48 I2C
- [x] code-review / sprint-status -- 收口

**Acceptance Criteria:**
- Given SPI 模式可复用，when 交付 I2C，then elaborate→emit→tick 通过
- Given 文档，when 阅读，then 说明范围与非目标

## Spec Change Log

## Design Notes

Master-only；非多主仲裁 / clock stretch / 从模式。

## Verification

**Commands:**
- `cargo test -p bitloom-prelude --lib i2c_master` -- expected: pass
- `cargo fmt --all && just test` -- expected: pass

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- `I2cMaster` + smoke；审查 accept

### File List

- `crates/bitloom-prelude/src/ip.rs`
- `_agile-output/implementation-artifacts/22-4-i2c-ip-smoke-fr48.md`
- `_agile-output/implementation-artifacts/22-4-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-08-21: FR48 I2C master stub smoke（Story 22.4）
