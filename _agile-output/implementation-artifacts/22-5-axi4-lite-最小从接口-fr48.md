---
title: '22.5 AXI4-Lite 最小从接口（FR48）'
type: 'feature'
created: '2026-08-21'
status: 'done'
baseline_commit: '945c55f31ad38ddc430cbba92742921fefc0827c'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/epic-22-context.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-first-class-ip.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR48 AXI 类须按 Open Q7 = AXI4-Lite 最小从接口达标。

**Approach:** `bitloom_prelude::ip::Axi4LiteSlave`（ADDR=8 / DATA=32 通道端口 + 数据保持寄存器）；文档声明非 Full AXI；smoke elaborate→emit→tick。

## Boundaries & Constraints

**Always:** AXI4-Lite 最小从；文档声明非 Full AXI / 非互联；经 prelude；品牌 Bitloom。

**Ask First:** 无。

**Never:** Full AXI；完整互联；静默改 AXI 范围为他物。

## I/O & Edge-Case Matrix

| Scenario | Input | Expected | Error Handling |
|----------|-------|----------|----------------|
| write+read smoke | wdata + valids | rdata 捕获；awready；bresp=OKAY | panic |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip.rs` — `Axi4LiteSlave`

## Tasks & Acceptance

**Execution:**
- [x] Axi4LiteSlave + smoke + 文档 -- FR48 AXI
- [x] code-review / sprint-status -- 收口

**Acceptance Criteria:**
- Given AXI=AXI4-Lite 最小从已锁定，when 交付，then elaborate→emit→tick 通过
- Given 文档，when 阅读，then 声明非 Full AXI / 非完整互联；ADDR/DATA 宽度钉死

## Spec Change Log

## Design Notes

最小从握手 stub + 单寄存器回读；可选 UART/FIFO 连接夹具未做（AC 非必须）。

## Verification

**Commands:**
- `cargo test -p bitloom-prelude --lib axi4_lite` -- expected: pass
- `cargo fmt --all && just test` -- expected: pass

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- `Axi4LiteSlave` ADDR=8 DATA=32；审查 accept

### File List

- `crates/bitloom-prelude/src/ip.rs`
- `_agile-output/implementation-artifacts/22-5-axi4-lite-最小从接口-fr48.md`
- `_agile-output/implementation-artifacts/22-5-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-08-21: FR48 AXI4-Lite 最小从 smoke（Story 22.5）
