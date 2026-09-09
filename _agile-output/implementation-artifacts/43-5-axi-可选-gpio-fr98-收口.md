---
title: '43.5 AXI（及可选 GPIO）+ FR98 收口'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: '6467d60'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic43-vip-full-protocol-ip.md'
  - '{project-root}/_agile-output/implementation-artifacts/43-1-epic-43-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/43-4-i2c-全协议-近-vip-fr98.md'
  - '{project-root}/docs/ip/README.md'
  - '{project-root}/crates/bitloom-prelude/src/ip.rs'
  - '{project-root}/crates/bitloom/tests/fr82_spi_i2c_axi_baseline.rs'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/README.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
warnings: []
deferred:
  - 'GPIO VIP（G0）— 风险记录默认不纳入 FR98 关闭必选（G1）；未实现 ≠ 失败；不得口头宣称 GPIO VIP'
  - 'Full AXI / burst / ID / QoS / 互联 / 商业 VIP 对拍 — A4 明确非目标'
  - '可选 AXI↔UART/FIFO 互联夹具 — deferred-work epic-22-retro-item-50（仍可选）'
  - 'Epic 44+ — 本故事不得开工'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Epic 43 / FR98 要求 AXI 达到 NFR14 风险记录钉死的近 VIP 必选条 A1–A4，并收口四类面。现状仍是 FR82：**单寄存器**握手玩具，**忽略 addr/wstrb**。UART/SPI/I2C（43.2–43.4）已近 VIP，但不得单类宣称 FR98 全绿；Epic 43 / NFR14 关闭勾选仍开；README/deferred 仍把「VIP 级全协议 IP → FR98（Epic 43）」写成待交付而非已关闭。

**Approach:** 加深 `bitloom_prelude::ip::Axi4LiteSlave` 至 **AXI4-Lite 近 VIP（默认子集）**：正确 **AW/W/B** 与 **AR/R** 握手；**addr 译码** + **wstrb** 字节写；至少 **多寄存器**可寻址从窗口；elaborate→emit→tick + ATDD。**GPIO 不实现**（风险记录 G0 可选 / G1：未实现不构成关闭失败）。更新 `docs/ip/`、deferred、README；勾选 NFR14 Epic 43 / FR98 关闭条件；sprint `43-5: done` + **`epic-43: done`**。停止把 VIP 级全协议 IP 列为永久非目标（标 Epic 43 / FR98 已关闭）。**不**开工 Epic 44+。

## Boundaries & Constraints

**Always:** A1–A4；AXI4-Lite 近 VIP（非 Full）；多寄存器 + addr 译码 + wstrb；elaborate/emit/tick + ATDD；文档 Lite vs Full / 明确非目标；GPIO 诚实（未交付 / 非关闭失败）；勾选 NFR14 Epic 43 关闭；README/deferred FR93#4 标已关闭；品牌 Bitloom；设计只依赖 `bitloom-prelude`；FR82 AXI 夹具在加深后仍非 stub（须同步断言）；无捕获闭包进入 tick（AD-18）；`epic-43: done`；Epic 44+ 保持 backlog。

**Ask First:** 若产品决定把 GPIO G0 纳入关闭必选 — 须先改风险记录再实现；本故事默认 **不**交付 GPIO。若改钉 Full AXI / 互联 — Correct Course，非本故事。

**Never:** 用单寄存器忽略 addr/wstrb 冒充近 VIP；无合同宣称 Full AXI / 互联完成；把未交付 GPIO 说成 FR98 失败或口头宣称 GPIO VIP；只改 sprint 勾选而无 AXI ATDD/文档；开工 Epic 44+ 或把 44.x 标 ready；破坏设计 crate 仅依赖 `bitloom-prelude`；引入生成器闭包定制 AXI API。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| A1 AW/W/B | awvalid+wvalid 同拍 | awready/wready；写提交；bvalid 直至 bready | ATDD |
| A1 AR/R | arvalid | arready；rvalid+rdata；rready 清 | ATDD |
| A1 addr 译码 | awaddr/araddr ∈ 从窗口 | 命中对应寄存器；窗外写忽略/读 0（文档钉死） | ATDD |
| A1 wstrb | wstrb 部分字节 | 仅掩码字节更新；其余保留 | ATDD |
| A2 多寄存器 | ≥2 字地址（建议 0x00/0x04/0x08/0x0C） | 独立读写互不覆盖（除非同址） | ATDD |
| A3 夹具 | elaborate→emit→tick | FR98 AXI ATDD 绿 | panic on fail |
| A4 文档 | `docs/ip/README.md` | Lite 近 VIP 已交付 vs Full/互联/VIP 商业对拍非目标；GPIO 可选未交付 | 审查 |
| FR98 收口 | UART+SPI+I2C+AXI | NFR14 全勾；不再永久非目标话术 | 收口 ATDD |
| GPIO | G0 未实现 | **不**失败；**不得**宣称 GPIO VIP | G1 |
| sprint | epic-43 | `43-5: done`；`epic-43: done`；44+ backlog | Never 开工 |
| 闭包 | generator Fn | **禁止** | AD-18 |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip.rs` — **UPDATE** `Axi4LiteSlave`（多寄存器窗口、addr 译码、wstrb、rdata 锁存；文档注释 A1–A4）
- `docs/ip/README.md` — AXI 近 VIP 边界；UART/SPI/I2C 去掉「不得仅因本类宣称全绿」待 43.5 措辞；GPIO 可选未交付
- `_agile-output/specs/spec-rhdl/language-surface.md` — FR98 AXI 面
- `_agile-output/implementation-artifacts/deferred-work.md` — FR93#4 / VIP 全协议 → Epic 43 **已关闭**；epic-22 AXI 条目同步
- `README.md` — FR93#4 标 FR98 / Epic 43 **已关闭**
- `_agile-output/implementation-artifacts/nfr14-risk-epic43-vip-full-protocol-ip.md` — 勾选 AXI + 文档 + 禁止事项 + 品牌；状态 closed
- `AGENTS.md` — 轻量：Epic 43 / FR98 closed（若 brand-lock 有对应条）
- `crates/bitloom/tests/fr98_axi_near_vip.rs` — ATDD A1–A4
- `crates/bitloom/tests/fr98_epic43_closeout.rs` — FR98 / Epic 43 收口 ATDD
- `crates/bitloom/tests/fr82_spi_i2c_axi_baseline.rs` + prelude `axi4_lite_*` — 同步多寄存器/wstrb（addr0 全 strb 仍绿）
- `_agile-output/implementation-artifacts/sprint-status.yaml` — `43-5: done`；`epic-43: done`；**勿**动 44+

## Story

As a 系统集成者,
I want AXI4-Lite 或记录选定的 AXI 子集达到近 VIP 验收，并收口 FR98,
So that 全协议一级 IP 面可检查关闭。

## Acceptance Criteria

1. Given Story 43.2–43.4 与风险记录 A1–A4，when 实现 AXI4-Lite 近 VIP（AW/W/B + AR/R；addr 译码；wstrb；多寄存器）+ ATDD, then 可 elaborate/emit/tick 证明（FR98 AXI 面）
2. Given 可选 GPIO，when 交付本故事, then **未**实现 G0（默认）；文档写明可选未交付；**不**因此判 FR98 失败；**不**宣称 GPIO VIP
3. Given 文档 / deferred / README，when 阅读公开面, then 写明 Lite vs Full / 已交付 vs 非目标；**不再**把「VIP 级全协议 IP」列为永久非目标（标 Epic 43 / FR98 已关闭）
4. Given NFR14 关闭条件，when 勾选 AXI + 文档 + 禁止事项 + 品牌, then FR98 / Epic 43 关闭条件可检查；状态 closed — Story 43.5
5. Given sprint 纪律，when 交付, then `43-5: done`；`epic-43: done`；Epic 44+ **仍** backlog；设计 crate 仍只依赖 `bitloom-prelude`

## Tasks / Subtasks

- [x] T1: 加深 `Axi4LiteSlave`：≥4 字从窗口；addr 译码；wstrb 字节合并；握手保持；rdata 锁存（AC: 1）
- [x] T2: 更新 prelude 单元测试 + 同步 FR82 AXI 断言（仍非 stub）（AC: 1）
- [x] T3: FR98 ATDD `fr98_axi_near_vip`（A1–A4）（AC: 1, 3）
- [x] T4: 更新 `docs/ip/README.md` + language-surface + deferred + README；GPIO G1 诚实（AC: 2–3）
- [x] T5: 勾选 NFR14 Epic 43 关闭；收口 ATDD `fr98_epic43_closeout`；sprint `43-5` + `epic-43: done`；**不**开工 44+（AC: 4–5）

## Dev Notes

### 当前状态（须读后改）

- `Axi4LiteSlave`（FR82）：单 `data_r`；写捕获全 `wdata`；**忽略** `awaddr`/`araddr`/`wstrb`；`*ready` 在对应响应通道 idle 时组合为 1
- FR82 / prelude 单测：addr=0、wstrb=0xF 写读回环 — 加深后 **必须**保持 addr0 全 strb 路径绿，并扩展多址/部分 strb

### 建议交付语义（A1–A4）

- 宽度不变：**ADDR=8**，**DATA=32**，`wstrb` 4-bit
- 从窗口：字对齐 **`0x00` / `0x04` / `0x08` / `0x0C`** → `data0..data3`（A2）
- 写：`awvalid && wvalid && !bvalid` → 按 `awaddr` 译码；`next = (wdata & mask(wstrb)) | (old & ~mask)`；未映射地址写忽略（旧值不变）；`bresp=OKAY`
- 读：`arvalid && !rvalid && !bvalid` → 锁存 `rdata`（映射命中对应 reg，否则 0）；`rresp=OKAY`
- 握手：保持现有 AW/W 同拍、写优先于读、b/r 直到 ready（A1）
- **明确非目标（A4）：** Full AXI（burst/ID/QoS）、互联、多从 decode 阵列、商业 VIP 对拍、GPIO VIP、生成器闭包
- **GPIO：** 不新增类型；docs 写「可选未交付（G1）」

### 从 43.4 / 42.3 继承

- 故事体例 / ATDD 文件布局 / docs+deferred+language-surface+README 收口
- 关闭 epic：对照 `42-3-fr97-收口与回归`（`epic-*: done` + 收口 ATDD）
- 禁止闭包定制 IP API；one-story-one-commit

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 43 / Story 43.5]
- [Source: `_agile-output/implementation-artifacts/nfr14-risk-epic43-vip-full-protocol-ip.md` — A1–A4 / G0–G1 / 关闭条件]
- [Source: `crates/bitloom-prelude/src/ip.rs` — 现有 `Axi4LiteSlave`]
- [Source: `docs/ip/README.md` — AXI FR82 边界]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

### Completion Notes List

- `Axi4LiteSlave` near-VIP: 4-word window, addr decode, wstrb merge, rdata latch
- ATDD `fr98_axi_near_vip` + closeout `fr98_epic43_closeout` 绿；FR82 AXI 回归绿
- docs/ip + language-surface + deferred + README；NFR14 Epic 43 全勾 closed
- GPIO 可选未交付（G1）；未开工 Epic 44+
- 审查 Approve；automate 认定 ATDD 足够
- sprint：`43-5: done`；`epic-43: done`

### File List

- `crates/bitloom-prelude/src/ip.rs`
- `crates/bitloom/tests/fr98_axi_near_vip.rs`
- `crates/bitloom/tests/fr98_epic43_closeout.rs`
- `docs/ip/README.md`
- `README.md`
- `AGENTS.md`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic43-vip-full-protocol-ip.md`
- `_agile-output/implementation-artifacts/43-5-axi-可选-gpio-fr98-收口.md`
- `_agile-output/implementation-artifacts/atdd-checklist-43-5-axi-可选-gpio-fr98-收口.md`
- `_agile-output/implementation-artifacts/43-5-code-review.md`
- `_agile-output/implementation-artifacts/43-5-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Story context created (ready-for-dev) — AXI near-VIP A1–A4 + FR98 / Epic 43 closeout
- 2026-09-09: Implement AXI near-VIP Axi4LiteSlave + FR98/Epic 43 closeout（Story 43.5）

## Suggested Review Order

**Axi4LiteSlave 多寄存器 + addr/wstrb** → **FR98 AXI ATDD** → **FR82 同步** → **docs/ip + README/deferred** → **NFR14 勾选 + 收口 ATDD** → **sprint epic-43 done（44+ backlog）**
