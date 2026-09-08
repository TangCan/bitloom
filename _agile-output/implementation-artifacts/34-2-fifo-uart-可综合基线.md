---
title: '34.2 FIFO + UART 可综合基线'
type: 'feature'
created: '2026-09-08'
status: 'done'
baseline_commit: 'd80bb446c82ce72f193fd9f5a2726de2d00c7025'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic34-ip-baseline.md'
  - '{project-root}/_agile-output/implementation-artifacts/34-1-epic-34-nfr14-风险记录.md'
  - '{project-root}/docs/ip/README.md'
  - '{project-root}/crates/bitloom-prelude/src/ip.rs'
warnings: []
deferred:
  - 'SPI / I2C / AXI non-stub deepen → Story 34.3'
  - 'IP generator closures → Epic 29'
  - 'programmable UART baud / RX / full-duplex → explicit future contract'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Epic 22 `SyncFifo` / `UartTx` are port-semantic stubs; FR82 needs first non-stub elaborate→emit→tick evidence for FIFO + UART without claiming full protocol stacks or introducing generator closures.

**Approach:** Harden `bitloom_prelude::ip::{SyncFifo,UartTx}` into real synthesizable baselines (depth-4 sync FIFO with full/empty; 8N1 UART TX bit-bang at baud=`clk`); ATDD + docs; retain opaque `ExtBlackBox` with documented boundaries; no closure APIs (Epic 29).

## Boundaries & Constraints

**Always:** FR82 FIFO+UART non-stub; elaborate→emit `.v`→tick fixtures; Bitloom / `bitloom-prelude` only for design crates; document black-box; NFR37 honesty vs Epic 22 stub.

**Ask First:** 无。

**Never:** generator-closure IP API; rename-only stub claim; full UART protocol / async FIFO; deepen SPI/I2C/AXI in this story.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| SyncFifo push/pop | wr_en/rd_en + data | full/empty + data_out order | panic on ATDD fail |
| SyncFifo full | 4 pushes then wr_en | write ignored; head intact | assert |
| UartTx frame | wr_en + byte | start+8data+stop on `tx`; busy gate | assert |
| UartTx busy write | wr_en while busy | payload not replaced | assert |
| Black-box | elaborate | empty body + vendor `.v` string | assert |
| Closures | SyncFifo/UartTx API | no `Fn` / dyn Fn params | ATDD source scan |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip.rs` — SyncFifo / UartTx FR82 RTL
- `crates/bitloom-sim/src/lib.rs` — `Sim::settle` for same-cycle enables
- `examples/ip_box/` — joint fixture
- `crates/bitloom/tests/fr82_fifo_uart_baseline.rs` — ATDD
- `docs/ip/README.md` / `docs/fr37-ip-box.md` / `language-surface.md`

## Story

As a IP 集成者,
I want FIFO 与 UART 达到可 elaborate/emit/tick,
So that FR82 有首批非 stub 证据。

## Acceptance Criteria

1. Given Story 34.1, when 实现/加固 SyncFifo 与 UartTx 为非 stub 硬件路径（FR82）, then 各至少一夹具 elaborate → emit `.v` → tick 通过
2. Given 本故事 API, when 例化, then **不**接受生成器闭包（留给 Epic 29）
3. Given 黑盒 wrapper（若保留）, when 文档化, then 行为与边界写清

## Tasks / Subtasks

- [x] T1: SyncFifo depth-4 + full/empty + tests（AC: 1）
- [x] T2: UartTx 8N1 bit-bang + busy gate + tests（AC: 1–2）
- [x] T3: ExtBlackBox 边界文档 + `Sim::settle`（AC: 3）
- [x] T4: docs/ip + language-surface + deferred-work；ATDD `fr82_fifo_uart_baseline`
- [x] T5: story / code-review Approve / sprint `34-2: done`

## Dev Notes

- 「非 stub」= 可演示最小真实硬件语义（NFR14 Epic 34），非全协议。
- 同周期门控：`set_inputs` → `settle` → `tick`。
- CDC `SyncFIFO` marker ≠ 一级 `SyncFifo` IP。

### Project Structure Notes

- IP 仍住在 `bitloom-prelude::ip`（无独立 `bitloom-fifo` crate 本故事）

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- SyncFifo: Mem depth-4 + ptr/count + full/empty（替换 depth-1 skid stub）
- UartTx: 8N1 serial `tx` + accept-only-when-idle（替换 hold-only stub）
- `Sim::settle`；docs + FR82 ATDD；black-box 边界保留并文档化
- bmad-build render HALT（ambiguous implementation_artifacts）；按 34.1 产物管道收口

### File List

- `crates/bitloom-prelude/src/ip.rs`
- `crates/bitloom-sim/src/lib.rs`
- `examples/ip_box/src/lib.rs`
- `crates/bitloom/tests/fr82_fifo_uart_baseline.rs`
- `docs/ip/README.md`
- `docs/fr37-ip-box.md`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `_agile-output/implementation-artifacts/34-2-fifo-uart-可综合基线.md`
- `_agile-output/implementation-artifacts/34-2-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-08: FR82 SyncFifo + UartTx non-stub baselines（Story 34.2）

## Suggested Review Order

**ip.rs SyncFifo/UartTx** → **ATDD fr82** → **docs/ip + black-box** → **sprint 键**
