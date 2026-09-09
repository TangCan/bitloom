---
title: '43.3 SPI 全协议 / 近 VIP（FR98）'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: 'fbdf2b9'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic43-vip-full-protocol-ip.md'
  - '{project-root}/_agile-output/implementation-artifacts/43-1-epic-43-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/43-2-uart-全协议-近-vip-fr98.md'
  - '{project-root}/docs/ip/README.md'
  - '{project-root}/crates/bitloom-prelude/src/ip.rs'
  - '{project-root}/crates/bitloom/tests/fr82_spi_i2c_axi_baseline.rs'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
warnings: []
deferred:
  - 'I2C / AXI 近 VIP → Stories 43.4–43.5'
  - 'Epic 43 / FR98 关闭勾选 → Story 43.5'
  - 'DMA / 多 CS 阵列 / slave / LSB-first / 可变字长非 8×N / 商业 VIP 对拍 → 明确非目标（S4）'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Epic 43 / FR98 要求 SPI 达到 NFR14 风险记录钉死的「全协议 / 近 VIP」必选条 S1–S4。现状仍是 FR82：**单一 Mode-0-ish** 单字节 toy shifter（1 bit/clk、`sclk` 忙时恒高）；无 CPOL/CPHA 配置、无多字节帧、`cs_n` 仅单字节边界。不得用 FR82 最小子集冒充 FR98 SPI 面。

**Approach:** 加深 `bitloom_prelude::ip::SpiMaster` 至近 VIP：**可配置 CPOL/CPHA（四模式）**；**Master 多字节传输**且 **`cs_n` 在整帧内保持有效、帧结束释放**；半周期 `sclk` 边沿语义（相对 FR82 Mode-0-ish 对照）；导出可读 RX 字节；至少一 ATDD 夹具 elaborate → emit → tick。更新 `docs/ip/README.md` 写明已交付模式/位序 vs 明确非目标。**不**实现 I2C/AXI/GPIO；**不**勾选 Epic 43 / FR98 关闭；**不得**宣称四类 FR98 全绿。

## Boundaries & Constraints

**Always:** S1–S4；CPOL/CPHA 可配（四模式）；Master 多字节 + `cs_n` 帧边界；elaborate/emit/tick + ATDD；文档边界；品牌 Bitloom；设计只依赖 `bitloom-prelude`；既有 FR82 夹具在默认 `cpol=0,cpha=0,byte_count≤1` 路径仍证明非 stub（时序可升为真 Mode-0 半周期，须同步更新 FR82 断言）；无捕获闭包进入 tick（AD-18）。

**Ask First:** 若裁剪为「仅文档钉死 ≥2 模式子集」而非四模式 — 须在风险记录写明；本故事默认交付 **四模式**。

**Never:** 实现 I2C/AXI/GPIO（→ 43.4–43.5）；勾选 Epic 43 / FR98 关闭（→ 43.5）；宣称 FR98 全绿（仅 SPI 一类）；静默扩大到 DMA/多 CS/slave/未列模式；引入生成器闭包定制 SPI API；破坏设计 crate 仅依赖 `bitloom-prelude`。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| S1 Mode-0 | `cpol=0,cpha=0` | idle `sclk=0`；leading 采样 / trailing 改 MOSI；对照 FR82 Mode-0-ish | ATDD |
| S1 四模式 | `cpol/cpha` 00/01/10/11 | idle=`cpol`；CPHA 决定采样/改沿；emit 含端口 | ATDD ≥2 + 文档四模式 |
| S2 多字节 | `byte_count≥2` + 连续字节 | 整帧 `cs_n=0`；帧结束 `cs_n=1` | ATDD |
| S2 单字节 | `byte_count=0` 或 `1` | 1 字节帧；默认兼容 FR82 调用面 | 回归 |
| S3 夹具 | elaborate→emit→tick | 至少一 FR98 SPI ATDD 绿 | panic on fail |
| S4 文档 | `docs/ip/README.md` | 已交付 vs 非目标（DMA/多 CS/slave 等） | 审查 |
| 范围 | I2C/AXI | **不**触达实现 | Never |
| 闭包 | generator Fn | **禁止** | AD-18 |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip.rs` — **UPDATE** `SpiMaster`（CPOL/CPHA、`byte_count`、半周期 sclk、多字节 `cs_n`、RX 导出）
- `docs/ip/README.md` / language-surface / deferred-work — SPI 近 VIP 边界
- `crates/bitloom/tests/fr98_spi_near_vip.rs` — ATDD S1–S4
- `crates/bitloom/tests/fr82_spi_i2c_axi_baseline.rs` + prelude `spi_master_*` — 同步 Mode-0 半周期断言（仍非 stub）
- `examples/ip_box/` — 若再导出 SPI 端口则轻触

## Story

As a IP 集成者,
I want SPI 达到风险记录验收条,
So that 一级 SPI 不再是薄 stub/最小子集交差。

## Acceptance Criteria

1. Given Story 43.1 与风险记录 S1–S4，when 实现 SPI 清单项（可配置 CPOL/CPHA；Master 多字节 + `cs_n` 帧边界）, then 可 elaborate/emit/tick + ATDD 证明（FR98 SPI 面）
2. Given S1，when 配置 `cpol`/`cpha`, then 四模式可达（或若 Ask First 裁剪则文档钉死 ≥2 且含 Mode-0 对照）；默认交付四模式
3. Given S2，when `byte_count≥2`, then 整帧内 `cs_n` 保持有效，帧结束释放；单字节默认路径仍可用
4. Given 文档，when 阅读 `docs/ip/README.md`, then 写明已交付模式/位序 vs **明确非目标**；公开品牌 Bitloom；**未**静默扩大未列模式
5. Given 范围纪律，when 交付本故事, then **未**实现 I2C/AXI；**未**勾选 Epic 43 / FR98 全绿关闭；设计 crate 仍只依赖 `bitloom-prelude`

## Tasks / Subtasks

- [x] T1: 加深 `SpiMaster`：`cpol`/`cpha`/`byte_count`；半周期边沿；多字节 `cs_n` 帧；RX 可读（AC: 1–3）
- [x] T2: 更新 prelude 单元测试 + 同步 FR82 SPI 断言至 Mode-0 半周期（仍非 stub）（AC: 1, 3）
- [x] T3: FR98 ATDD `fr98_spi_near_vip`（S1–S4；多模式 + 多字节帧 + docs）（AC: 1–4）
- [x] T4: 更新 `docs/ip/README.md`（及 language-surface / deferred 轻触）（AC: 4）
- [x] T5: 回归 fr82；sprint `43-3: done`；`43-4`–`43-5` 仍 backlog；`epic-43` 保持 in-progress；NFR14 关闭清单仅勾 SPI 行（若有）但不勾 Epic 全关（AC: 5）

## Dev Notes

### 当前状态（须读后改）

- `SpiMaster`（FR82）：单字节；忙时 `sclk=1` 恒高；无 `cpol`/`cpha`/`byte_count`；RX 未导出（`mosi_byte`=TX latch）
- FR82 ATDD / prelude 单测硬编码「首拍 `sclk=1`」— 升半周期 Mode-0 后 **必须**改断言，禁止留红或假绿

### 建议交付语义

- 端口新增：`cpol`、`cpha`（1b）、`byte_count`（建议 3b；`0`≡`1` 字节）
- 保留：`start`/`tx_data`/`miso`/`mosi_byte`/`busy`/`cs_n`/`sclk`/`mosi`
- 新增输出：`rx_data`（末字节或当前字节）、建议 `rx_valid` 每字节一拍
- 多字节：accept 锁存 `n=max(byte_count,1)`；字节间采样当前 `tx_data`；`cs_n` 直至末字节结束
- MSB-first；1 半周期/边沿（2 clk/bit）
- idle：`cs_n=1`，`sclk=cpol`

### 从 43.2 继承

- 故事体例 / ATDD 文件布局 / docs+deferred+language-surface 三联更新
- 单类加深 **≠** FR98 全绿；sprint 只标本故事 `done`
- 禁止闭包定制 IP API

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 43 / Story 43.3]
- [Source: `_agile-output/implementation-artifacts/nfr14-risk-epic43-vip-full-protocol-ip.md` — S1–S4]
- [Source: `crates/bitloom-prelude/src/ip.rs` — 现有 `SpiMaster`]
- [Source: `docs/ip/README.md` — SPI FR82 边界]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

### Completion Notes List

- `SpiMaster` near-VIP: CPOL/CPHA 四模式、`byte_count` 多字节、`cs_n` 帧、`rx_data`/`rx_valid`；半周期 sclk
- ATDD `fr98_spi_near_vip` 绿；FR82 SPI 断言同步 Mode-0 半周期
- docs/ip + language-surface + deferred；NFR14 仅勾 SPI 行；未关 Epic 43
- 审查 Approve；automate 认定 ATDD 足够
- sprint：`43-3: done`；`43-4`–`43-5` backlog；`epic-43` in-progress

### File List

- `crates/bitloom-prelude/src/ip.rs`
- `crates/bitloom/tests/fr98_spi_near_vip.rs`
- `crates/bitloom/tests/fr82_spi_i2c_axi_baseline.rs`
- `docs/ip/README.md`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic43-vip-full-protocol-ip.md`
- `_agile-output/implementation-artifacts/43-3-spi-全协议-近-vip-fr98.md`
- `_agile-output/implementation-artifacts/atdd-checklist-43-3-spi-全协议-近-vip-fr98.md`
- `_agile-output/implementation-artifacts/43-3-code-review.md`
- `_agile-output/implementation-artifacts/43-3-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Story context created (ready-for-dev) — SPI near-VIP S1–S4 / FR98
- 2026-09-09: Implement SPI near-VIP SpiMaster + ATDD（Story 43.3）

## Suggested Review Order

**SpiMaster CPOL/CPHA + 半周期** → **多字节 cs_n 帧** → **FR98 ATDD** → **FR82 断言同步** → **docs/ip 边界** → **sprint（43.4–43.5 backlog；epic-43 in-progress）**
