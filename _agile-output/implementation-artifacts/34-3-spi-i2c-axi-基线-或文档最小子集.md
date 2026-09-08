---
title: '34.3 SPI / I2C / AXI 基线（或文档最小子集）'
type: 'feature'
created: '2026-09-08'
status: 'done'
baseline_commit: 'f87701223953868c22c9f3040f6b908872df0030'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic34-ip-baseline.md'
  - '{project-root}/_agile-output/implementation-artifacts/34-2-fifo-uart-可综合基线.md'
  - '{project-root}/docs/ip/README.md'
  - '{project-root}/crates/bitloom-prelude/src/ip.rs'
warnings: []
deferred:
  - 'IP generator closures → Epic 29'
  - 'SPI CPOL/CPHA modes / multi-CS / DMA / slave → explicit future contract'
  - 'I2C ACK/NACK / clock stretch / multi-master / 10-bit addr / slave → explicit future contract'
  - 'Full AXI / interconnect / VIP / multi-slave decode → out of FR82 (Open Q7 = AXI4-Lite min slave)'
  - 'AXI↔UART/FIFO interconnect fixture → deferred-work item-50'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Epic 22 `SpiMaster` / `I2cMaster` / `Axi4LiteSlave` remain port-semantic stubs after 34.2; FR82 needs non-stub elaborate→emit→tick for the remaining three classes (or a documented minimal subset with NFR37 honesty), without generator closures.

**Approach:** Upgrade all three in `bitloom_prelude::ip` to real synthesizable baselines: SPI Mode-0-ish MSB byte shifter; I2C START+8data+STOP bit-bang; AXI4-Lite single-register write/read handshake toy (ADDR=8, DATA=32). ATDD + docs; no closure APIs (Epic 29). No silent deferral — all three delivered as documented subsets.

## Boundaries & Constraints

**Always:** FR82 SPI+I2C+AXI non-stub (or written subset); elaborate→emit `.v`→tick fixtures; Bitloom / `bitloom-prelude` only; NFR37 honesty vs Epic 22 stub.

**Ask First:** 无。

**Never:** generator-closure IP API; rename-only stub claim; claim full SPI/I2C protocol or Full AXI; silently drop a class without decision table.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| SpiMaster byte | start + tx_data | cs_n low, mosi MSB-first 8×, busy gate | assert |
| SpiMaster busy | start while busy | payload not replaced | assert |
| I2cMaster byte | start + tx_data | START→8 data→STOP on scl/sda_out | assert |
| I2cMaster busy | start while busy | hold intact | assert |
| AXI write | awvalid+wvalid+wdata | store reg; bvalid until bready | assert |
| AXI read | arvalid after write | rvalid + rdata = stored | assert |
| Closures | Spi/I2c/Axi API | no Fn / dyn Fn params | ATDD source scan |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip.rs` — SpiMaster / I2cMaster / Axi4LiteSlave FR82 RTL
- `crates/bitloom/tests/fr82_spi_i2c_axi_baseline.rs` — ATDD
- `docs/ip/README.md` / `docs/fr37-ip-box.md` / `language-surface.md` / `deferred-work.md`

## Story

As a IP 集成者,
I want 其余一级 IP 达到合同化最小可综合基线,
So that FR48 五类深度可勾选。

## Acceptance Criteria

1. Given Story 34.2, when 交付 SPI、I2C、AXI4-Lite Slave（或文档钉死的等价最小子集）可 elaborate/emit/tick（FR82）, then 每类有自动化夹具，或决策表明确「本 epic 交付子集」且仍满足 FR82 关闭条
2. Given 未在本 epic 交付的类型, when 文档化, then 须书面降级（不得静默声称五类全完成）（NFR37）
3. Given 本故事 API, when 例化, then 无闭包定制参数

## Tasks / Subtasks

- [x] T1: SpiMaster Mode-0-ish byte shifter + tests（AC: 1）
- [x] T2: I2cMaster START+8data+STOP bit-bang + tests（AC: 1）
- [x] T3: Axi4LiteSlave single-reg handshake toy + tests（AC: 1）
- [x] T4: docs/ip + language-surface + deferred-work；ATDD `fr82_spi_i2c_axi_baseline`（AC: 1–3）
- [x] T5: story / code-review Approve / sprint `34-3: done`

## Dev Notes

- 「非 stub」= 可演示最小真实硬件语义（NFR14 Epic 34），非全协议 / VIP。
- 本故事**不**书面降级任一类——三类别均交付文档化最小子集。
- 同周期门控：`set_inputs` → `settle` → `tick`。
- AXI = AXI4-Lite 最小从（Open Q7）；非 Full AXI。
- SHL 后须 `& 0xFF`（sim 不自动截断位宽）。

### Project Structure Notes

- IP 仍住在 `bitloom-prelude::ip`（无独立 crate 本故事）

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- SpiMaster: Mode-0-ish MSB byte shifter（cs_n/sclk/mosi + busy 门控；替换端口 stub）
- I2cMaster: START+8data+STOP bit-bang（scl 恒高教学玩具；busy 门控）
- Axi4LiteSlave: 单寄存器 write/read 握手（aw/w → bvalid；ar → rvalid/rdata）
- docs/ip 决策表：三类均交付最小子集，无静默降级；ATDD `fr82_spi_i2c_axi_baseline`
- bmad-build render HALT（ambiguous implementation_artifacts）；按 34.2 产物管道收口

### File List

- `crates/bitloom-prelude/src/ip.rs`
- `crates/bitloom/tests/fr82_spi_i2c_axi_baseline.rs`
- `docs/ip/README.md`
- `docs/fr37-ip-box.md`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `_agile-output/implementation-artifacts/34-3-spi-i2c-axi-基线-或文档最小子集.md`
- `_agile-output/implementation-artifacts/34-3-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-08: FR82 SpiMaster + I2cMaster + Axi4LiteSlave non-stub baselines（Story 34.3）

## Suggested Review Order

**ip.rs Spi/I2c/Axi** → **ATDD fr82** → **docs/ip + decision table** → **sprint 键**
