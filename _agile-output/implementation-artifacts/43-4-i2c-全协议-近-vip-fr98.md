---
title: '43.4 I2C 全协议 / 近 VIP（FR98）'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: '5302b84'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic43-vip-full-protocol-ip.md'
  - '{project-root}/_agile-output/implementation-artifacts/43-1-epic-43-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/43-3-spi-全协议-近-vip-fr98.md'
  - '{project-root}/docs/ip/README.md'
  - '{project-root}/crates/bitloom-prelude/src/ip.rs'
  - '{project-root}/crates/bitloom/tests/fr82_spi_i2c_axi_baseline.rs'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
warnings: []
deferred:
  - 'AXI / 可选 GPIO / FR98 收口 → Story 43.5'
  - 'Epic 43 / FR98 关闭勾选 → Story 43.5'
  - 'clock stretch / 多主 / 10-bit / slave / SMBus PEC / 商业 VIP 对拍 → 明确非目标（I4）'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Epic 43 / FR98 要求 I2C 达到 NFR14 风险记录钉死的「全协议 / 近 VIP」必选条 I1–I4。现状仍是 FR82：**教学玩具** — START + 8 data + STOP；**SCL 恒高**；`sda_in` 仅 sticky 采样、**无 ACK/NACK 驱动路径**；无 7-bit 地址相位。不得用 FR82 最小子集冒充 FR98 I2C 面。

**Approach:** 加深 `bitloom_prelude::ip::I2cMaster` 至近 VIP：**ACK/NACK 驱动**的 Master **写**路径，以及文档要求的 **读**路径；**START / 7-bit 地址 / 数据 / STOP**；**SCL 为真实半周期时钟边沿**（idle 高）；至少一 ATDD 夹具 elaborate → emit → tick。更新 `docs/ip/README.md` 写明已交付 vs 明确非目标。**不**实现 AXI/GPIO；**不**勾选 Epic 43 / FR98 关闭；**不得**宣称四类 FR98 全绿。

## Boundaries & Constraints

**Always:** I1–I4；Master 写 + 读路径；START/7-bit addr/data/STOP；SCL 半周期边沿；elaborate/emit/tick + ATDD；文档边界；品牌 Bitloom；设计只依赖 `bitloom-prelude`；既有 FR82 夹具在加深后仍证明非 stub（时序/端口变化须同步更新 FR82 断言）；无捕获闭包进入 tick（AD-18）。

**Ask First:** 若砍掉读路径仅保留写 — 须改风险记录；本故事默认 **写 + 读**（I1「及文档要求的读」）。

**Never:** 实现 AXI/GPIO（→ 43.5）；勾选 Epic 43 / FR98 关闭（→ 43.5）；宣称 FR98 全绿（仅 I2C 一类）；静默扩大到 clock stretch/多主/10-bit/slave；引入生成器闭包定制 I2C API；破坏设计 crate 仅依赖 `bitloom-prelude`。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| I2 START+addr | `start` + `addr` + `rw=0` | START 条件；随后 7-bit addr + W；SCL 高低半周期 | ATDD |
| I1 ACK | `sda_in=0` 于 ACK 槽 | 继续数据相位；`ack_error=0` | ATDD |
| I1 NACK | `sda_in=1` 于 ACK 槽 | 进入 STOP；`ack_error=1`；busy 清 | ATDD |
| I1 写数据 | ACK 后 `tx_data` | 8 位 MSB-first + ACK 槽 | ATDD |
| I1 读路径 | `rw=1` | 释放 SDA 采 8 位 → `rx_data`/`rx_valid`；末字节 NACK 或文档钉死的 ACK 合同 | ATDD |
| I2 SCL | busy 位相位 | idle `scl=1`；位内半周期 `0→1`（非恒高玩具） | ATDD |
| I3 夹具 | elaborate→emit→tick | 至少一 FR98 I2C ATDD 绿 | panic on fail |
| I4 文档 | `docs/ip/README.md` | 已交付 vs 非目标（stretch/多主/10-bit/slave…） | 审查 |
| 范围 | AXI/GPIO | **不**触达实现 | Never |
| 闭包 | generator Fn | **禁止** | AD-18 |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip.rs` — **UPDATE** `I2cMaster`（addr/rw、半周期 SCL、ACK/NACK、读路径、`rx_data`/`ack_error`）
- `docs/ip/README.md` / language-surface / deferred-work — I2C 近 VIP 边界
- `crates/bitloom/tests/fr98_i2c_near_vip.rs` — ATDD I1–I4
- `crates/bitloom/tests/fr82_spi_i2c_axi_baseline.rs` + prelude `i2c_master_*` — 同步新时序断言（仍非 stub）
- `examples/ip_box/` — 若再导出 I2C 端口则轻触

## Story

As a IP 集成者,
I want I2C 达到风险记录验收条,
So that 一级 I2C 达到字面绿深度。

## Acceptance Criteria

1. Given Story 43.1 与风险记录 I1–I4，when 实现 I2C 清单项（ACK/NACK 驱动写 + 文档化读；START/7-bit addr/data/STOP；SCL 真实边沿）, then 可 elaborate/emit/tick + ATDD 证明（FR98 I2C 面）
2. Given I1，when Master 写并在 ACK 槽采 `sda_in`, then ACK 继续 / NACK 停并置错；读路径导出 `rx_data`（或等价）
3. Given I2，when 传输进行中, then SCL 非恒高玩具（半周期边沿）；完整 START → 地址 → 数据 → STOP
4. Given 文档，when 阅读 `docs/ip/README.md`, then 写明已交付 vs **明确非目标**；公开品牌 Bitloom；**未**静默扩大未列模式
5. Given 范围纪律，when 交付本故事, then **未**实现 AXI/GPIO；**未**勾选 Epic 43 / FR98 全绿关闭；设计 crate 仍只依赖 `bitloom-prelude`

## Tasks / Subtasks

- [x] T1: 加深 `I2cMaster`：`addr`/`rw`；半周期 SCL；ACK/NACK；写+读；`rx_data`/`rx_valid`/`ack_error`（AC: 1–3）
- [x] T2: 更新 prelude 单元测试 + 同步 FR82 I2C 断言至近 VIP 时序（仍非 stub）（AC: 1, 3）
- [x] T3: FR98 ATDD `fr98_i2c_near_vip`（I1–I4；写 ACK/NACK + 读 + docs）（AC: 1–4）
- [x] T4: 更新 `docs/ip/README.md`（及 language-surface / deferred 轻触）（AC: 4）
- [x] T5: 回归 fr82；sprint `43-4: done`；`43-5` 仍 backlog；`epic-43` 保持 in-progress；NFR14 关闭清单仅勾 I2C 行但不勾 Epic 全关（AC: 5）

## Dev Notes

### 当前状态（须读后改）

- `I2cMaster`（FR82）：START + 8 data + STOP；**`scl` 恒 1**；无 `addr`/`rw`；`ack_sample` 未导出；无读路径
- FR82 ATDD / prelude 单测硬编码「START 后下一拍即 data MSB、scl≡1」— 升半周期 + 地址相位后 **必须**改断言，禁止留红或假绿

### 建议交付语义

- 端口新增：`addr`（7b）、`rw`（1b）；保留 `start`/`tx_data`/`sda_in`/`tx_byte`/`busy`/`scl`/`sda_out`
- 新增输出：`rx_data`（8b）、`rx_valid`（1b 脉冲）、`ack_error`（NACK 所见 sticky 或帧末）
- 可选：`byte_count`（3b；`0`≡1）— 若实现成本过高，单字节写/读即可满足 I1–I2（多字节非 I 必选条）
- 事务：START → `{addr[6:0],rw}` ×8 → ACK 槽 →（写：`tx_data`×8→ACK）|（读：采 8 位→主驱动 ACK/NACK）→ STOP
- SCL：idle=1；每位 half0=`scl=0`（改 SDA）/ half1=`scl=1`（采样 ACK / 读位）
- open-drain 模型：主释放 = `sda_out=1`（外部上拉）；从 ACK = `sda_in=0`
- MSB-first；禁止闭包定制 API

### 从 43.3 继承

- 故事体例 / ATDD 文件布局 / docs+deferred+language-surface 三联更新
- 单类加深 **≠** FR98 全绿；sprint 只标本故事 `done`
- 禁止闭包定制 IP API；半周期边沿模式可对照 `SpiMaster`

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 43 / Story 43.4]
- [Source: `_agile-output/implementation-artifacts/nfr14-risk-epic43-vip-full-protocol-ip.md` — I1–I4]
- [Source: `crates/bitloom-prelude/src/ip.rs` — 现有 `I2cMaster`]
- [Source: `docs/ip/README.md` — I2C FR82 边界]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

### Completion Notes List

- `I2cMaster` near-VIP: `addr`/`rw`、半周期 SCL、ACK/NACK 写+读、`rx_data`/`rx_valid`/`ack_error`
- ATDD `fr98_i2c_near_vip` 绿；FR82 I2C 断言同步新时序
- docs/ip + language-surface + deferred；NFR14 仅勾 I2C 行；未关 Epic 43
- 审查 Approve；automate 认定 ATDD 足够
- sprint：`43-4: done`；`43-5` backlog；`epic-43` in-progress

### File List

- `crates/bitloom-prelude/src/ip.rs`
- `crates/bitloom/tests/fr98_i2c_near_vip.rs`
- `crates/bitloom/tests/fr82_spi_i2c_axi_baseline.rs`
- `docs/ip/README.md`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic43-vip-full-protocol-ip.md`
- `_agile-output/implementation-artifacts/43-4-i2c-全协议-近-vip-fr98.md`
- `_agile-output/implementation-artifacts/atdd-checklist-43-4-i2c-全协议-近-vip-fr98.md`
- `_agile-output/implementation-artifacts/43-4-code-review.md`
- `_agile-output/implementation-artifacts/43-4-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Story context created (ready-for-dev) — I2C near-VIP I1–I4 / FR98
- 2026-09-09: Implement I2C near-VIP I2cMaster + ATDD（Story 43.4）

## Suggested Review Order

**I2cMaster addr/rw + 半周期 SCL** → **ACK/NACK 写** → **读路径** → **FR98 ATDD** → **FR82 断言同步** → **docs/ip 边界** → **sprint（43.5 backlog；epic-43 in-progress）**
