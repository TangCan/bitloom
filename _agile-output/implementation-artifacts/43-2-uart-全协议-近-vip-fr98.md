---
title: '43.2 UART 全协议 / 近 VIP（FR98）'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: '91308c0'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic43-vip-full-protocol-ip.md'
  - '{project-root}/_agile-output/implementation-artifacts/43-1-epic-43-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/38-2-uarttx-显式加深子集实现-fr89.md'
  - '{project-root}/docs/ip/README.md'
  - '{project-root}/crates/bitloom-prelude/src/ip.rs'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
warnings: []
deferred:
  - 'SPI / I2C / AXI 近 VIP → Stories 43.3–43.5'
  - 'Epic 43 / FR98 关闭勾选 → Story 43.5'
  - '小数分频 / 流控 / IrDA / parity / FIFO\'d UART / 商业 VIP 对拍 → 明确非目标（U5）'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Epic 43 / FR98 要求 UART 达到 NFR14 风险记录钉死的「全协议 / 近 VIP」必选条 U1–U5。现状仍是 FR82+FR89：**仅 UartTx**（8N1 + 可编程 `baud_div`）；RX / 全双工仍为明确非目标。不得用 FR89 子集加深冒充 FR98 UART 面。

**Approach:** 在 `bitloom_prelude::ip` 交付 **TX + RX** 可达的 UART 近 VIP 面：保留并复用 `UartTx`（含 `baud_div`）；新增 **`UartRx`**（8N1 RX，同一 `baud_div` 语义驱动位时序）；全双工合同 = **同设计可例化 UartTx + UartRx**（独立串行线；不强制片上半双工切换）。至少一 ATDD 夹具 elaborate → emit → tick 覆盖 TX **与** RX 路径（推荐 loopback：TX `tx` → RX `rx`）。更新 `docs/ip/README.md` 写明已交付协议面 vs 明确非目标。**不**实现 SPI/I2C/AXI；**不**勾选 Epic 43 / FR98 关闭条件。

## Boundaries & Constraints

**Always:** U1–U5；TX+RX；可编程波特率驱动两侧；8N1 帧；elaborate/emit/tick + ATDD；文档边界；品牌 Bitloom；设计只依赖 `bitloom-prelude`；既有 FR82/FR89 夹具在默认路径仍绿；无捕获闭包进入 tick（AD-18）。

**Ask First:** 若改为「仅文档化半双工切换、不做独立 RX 线」— 须在风险记录写明例外（本记录默认 TX+RX 全双工例化合同，不采用单工例外）。

**Never:** 实现 SPI/I2C/AXI/GPIO（→ 43.3–43.5）；勾选 Epic 43 / FR98 关闭（→ 43.5）；宣称 FR98 全绿（仅 UART 一类）；把 FR89 重标为 FR98 UART 完成面而不交付 RX；引入生成器闭包定制 UART API；破坏 `baud_div=0` ≡ FR82 1 clk/bit。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| U1 TX+RX | 例化 `UartTx` + `UartRx` | 两侧均可 elaborate；文档钉全双工合同 | 缺 RX → ATDD 红 |
| U2 baud | 两侧 `baud_div=N` | TX/RX 位时序均为 N+1 clk/bit | assert |
| U3 8N1 | 发送/接收一字节 | start+8 data LSB-first+stop；文档写明帧面 | ATDD |
| U4 夹具 | loopback 或分路径 tick | TX 与 RX 路径均有 tick 证据 | panic on fail |
| U5 文档 | `docs/ip/README.md` | 已交付 vs 非目标（小数分频/流控/IrDA/parity/VIP 对拍等） | 审查 |
| FR89 回归 | `baud_div=0` UartTx | 仍 1 clk/bit；既有 fr82/fr89 绿 | 回归失败则修 |
| 范围 | SPI/I2C/AXI | **不**触达实现 | Never |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip.rs` — **UPDATE** `UartTx` 注释；**NEW** `UartRx`
- `docs/ip/README.md` / language-surface / deferred-work — UART 近 VIP 边界
- `crates/bitloom/tests/fr98_uart_near_vip.rs` — ATDD U1–U5
- FR89 ATDD — 历史 Epic 38 合同与 FR98 并存
- `examples/ip_box/` — 再导出 `UartRx` smoke

## Story

As a IP 集成者,
I want UART 达到风险记录中的全协议/近 VIP 验收条,
So that 超出 FR89 可编程波特率子集。

## Acceptance Criteria

1. Given Story 43.1 与风险记录 U1–U5，when 实现 UART 清单项（含 TX/RX；全双工=可同设计例化两侧）, then 可 elaborate/emit/tick + ATDD 证明 TX 与 RX 路径（FR98 UART 面）
2. Given U2/U3，when 配置 `baud_div` 并收发, then 两侧位时序由可编程分频驱动；帧格式至少 8N1
3. Given 文档，when 阅读 `docs/ip/README.md`, then 写明已交付协议面 vs **明确非目标**；公开品牌 Bitloom
4. Given 范围纪律，when 交付本故事, then **未**实现 SPI/I2C/AXI；**未**勾选 Epic 43 / FR98 全绿关闭；设计 crate 仍只依赖 `bitloom-prelude`

## Tasks / Subtasks

- [x] T1: 实现 `UartRx`（8N1 + `baud_div`；start 检测；采样/组装）（AC: 1–2）
- [x] T2: 更新 `UartTx`/ip 文档注释；保留 FR82/FR89 兼容（AC: 2, 4）
- [x] T3: prelude 单元测试 + FR98 ATDD（TX+RX elaborate→emit→tick；loopback）（AC: 1–2）
- [x] T4: 更新 `docs/ip/README.md`（及 language-surface / deferred 轻触）（AC: 3）
- [x] T5: 回归 fr82/fr89；sprint `43-2: done`；`43-3`–`43-5` 仍 backlog；`epic-43` 保持 in-progress（AC: 4）

## Dev Notes

### 交付语义

- `UartRx`：`clk/rst/rx/baud_div` → `rd_data/rd_valid/rx_busy`；下降沿启动；bit_idx 从 1 起采 8 数据位（期末采样）+ stop；`rd_valid` 一周期脉冲
- 全双工：`UartTx` + `UartRx` 双例化
- 未勾选 NFR14 Epic 43 关闭清单

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 43 / Story 43.2]
- [Source: `_agile-output/implementation-artifacts/nfr14-risk-epic43-vip-full-protocol-ip.md` — U1–U5]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

### Completion Notes List

- `UartRx` 8N1 + `baud_div`；loopback ATDD `fr98_uart_near_vip` 绿
- docs/ip + language-surface + deferred；FR89 历史 ATDD 与 FR98 并存
- 审查 Approve；automate 认定 ATDD 足够
- sprint：`43-2: done`；`43-3`–`43-5` backlog；`epic-43` in-progress；未勾选 FR98 全关

### File List

- `crates/bitloom-prelude/src/ip.rs`
- `crates/bitloom/tests/fr98_uart_near_vip.rs`
- `crates/bitloom/tests/fr89_uarttx_programmable_baud.rs`
- `crates/bitloom/tests/fr89_epic38_boundary_closeout.rs`
- `crates/bitloom/tests/fr82_ip_baseline_matrix.rs`
- `docs/ip/README.md`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `examples/ip_box/src/lib.rs`
- `_agile-output/implementation-artifacts/43-2-uart-全协议-近-vip-fr98.md`
- `_agile-output/implementation-artifacts/atdd-checklist-43-2-uart-全协议-近-vip-fr98.md`
- `_agile-output/implementation-artifacts/43-2-code-review.md`
- `_agile-output/implementation-artifacts/43-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Story context created (ready-for-dev) — UART near-VIP U1–U5 / FR98
- 2026-09-09: Implement UART near-VIP UartRx + ATDD（Story 43.2）

## Suggested Review Order

**UartRx + baud 对齐** → **FR98 ATDD（TX+RX / loopback）** → **docs/ip 边界** → **sprint（43.3–43.5 backlog；epic-43 in-progress）**
