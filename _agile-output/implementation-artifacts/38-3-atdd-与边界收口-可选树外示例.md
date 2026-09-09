---
title: '38.3 ATDD + 边界收口（可选树外示例）'
type: 'docs'
created: '2026-09-09'
status: 'done'
baseline_commit: '293f4ea'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic38-uarttx-deepen.md'
  - '{project-root}/_agile-output/implementation-artifacts/38-2-uarttx-显式加深子集实现-fr89.md'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/docs/ip/README.md'
  - '{project-root}/crates/bitloom/tests/fr89_uarttx_programmable_baud.rs'
  - '{project-root}/examples/ip_box/src/lib.rs'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
  - '{project-root}/_agile-output/implementation-artifacts/process-one-story-one-commit.md'
warnings: []
deferred:
  - '最小 RX（分支 B）→ 本 epic 明确不交付；须新合同'
  - '全协议 / VIP / 小数分频 / 波特率表全家桶 → standing deferred；须新合同'
  - 'Epic 39 宿主 IDE / 多视图 → 本故事不开工'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Story 38.2 已交付 UartTx 可编程 `baud_div`（分支 A）与 FR89 加深夹具，但 NFR14 Epic 38 **关闭条件未勾选**；deferred-work「全协议仍须新合同」交叉引用仍指向「→ Story 38.3」未收口；Epic 38 仍 `in-progress`。缺少稳定的**收口 ATDD**易让 FR89 关闭条件不可重复验证。

**Approach:** 以文档/契约收口为主（对齐 37.3 体例）：（1）加深路径 ATDD 稳定（既有 `fr89_uarttx_programmable_baud` + 新建收口 ATDD）；（2）更新 deferred-work 交叉引用，钉死「全协议仍须新合同 / FR89 子集 ≠ 全家桶」；（3）勾选 NFR14 Epic 38 关闭条件并将记录标 closed；（4）**可选**在 `examples/ip_box` 演示 `baud_div` 加深 API（不做不阻塞）；（5）sprint：`38-3` + `epic-38` → done。**不**交付 RX；**不**开工 Epic 39。

## Boundaries & Constraints

**Always:** Gate 38.1+38.2 done；加深分支仍为 **A**；FR89 加深 ATDD 保持绿；deferred-work 交叉引用收口；勾选 NFR14 Epic 38 关闭条件；文档边界诚实（非 VIP/全协议/RX/全双工）；NFR39；品牌 Bitloom；AD-6；one-story-one-commit；全部 38 故事 done 后 `epic-38: done`。

**Ask First:** 无（可选树外示例默认做轻量 `ip_box` 加深演示；若跳过须在 Completion Notes 写明「可选未做不阻塞」）。

**Never:** 实现最小 RX / 全双工 / VIP / 全协议；静默扩 SPI/I2C/AXI；开工 Epic 39；把 FR89 子集写成「UART 全协议已交付」；把本收口冒充 NFR14-crates；引入生成器闭包定制 API。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 加深 ATDD 稳定 | `fr89_uarttx_programmable_baud` | 仍绿（baud_div>0 + baud_div=0 回归 + docs 边界） | 回归红则先修 |
| 收口 ATDD | 新建 `fr89_epic38_boundary_closeout`（或等价） | 锁 NFR14 勾选 + deferred 交叉引用 + 非 RX | 红直至落盘 |
| deferred 收口 | epic-22-retro-item-49（及 item-83 注） | 交叉引用写明 Story 38.3 已收口；全协议仍须新合同 | ATDD 锁关键词 |
| NFR14 关闭 | `nfr14-risk-epic38-uarttx-deepen.md` | 关闭条件全 `[x]`；status → closed | ATDD |
| 可选示例 | `examples/ip_box` | 演示 `baud_div` 加深（可选） | 不做不阻塞 |
| 范围越界 | Epic 39 / UartRx | 拒收 | Never |

</frozen-after-approval>

## Code Map

- `crates/bitloom/tests/fr89_uarttx_programmable_baud.rs` — **核对**（38.2 加深路径；本故事保持稳定）
- `crates/bitloom/tests/fr89_epic38_boundary_closeout.rs` — **NEW** 收口 ATDD（NFR14 勾选 / deferred 交叉引用 / 非 RX / epic 关闭叙事）
- `_agile-output/implementation-artifacts/nfr14-risk-epic38-uarttx-deepen.md` — **UPDATE** 勾选 Epic 38 关闭条件；状态 → closed
- `_agile-output/implementation-artifacts/deferred-work.md` — **UPDATE** item-49（及必要时 item-83）交叉引用收口：Story 38.3；全协议仍须新合同
- `docs/ip/README.md` — **核对**（38.2 已写边界；必要时轻触指向收口）
- `examples/ip_box/src/lib.rs` — **可选 UPDATE**：UartTx 路径演示 `baud_div>0`
- `_agile-output/implementation-artifacts/sprint-status.yaml` — `38-3` → done；`epic-38` → done；**不**改 epic-39 为 in-progress

## Story

As a 质量负责人,
I want 加深路径有稳定 ATDD，并可选树外示例,
So that FR89 关闭条件可重复验证。

## Acceptance Criteria

1. Given Story 38.2，when 增加自动化黄金/ATDD 覆盖选定加深分支（分支 A），并更新 deferred-work 中「全协议仍须新合同」条目交叉引用，then 测试稳定通过
2. And （可选）增加树外/`examples` 包演示加深 API——不做不阻塞关闭
3. And NFR14 记录勾选 Epic 38 关闭条件
4. And 全部 38 故事 done 后 sprint `epic-38: done`；**不**开工 Epic 39

## Tasks / Subtasks

- [x] T1: 收口 ATDD（NFR14 关闭勾选 + deferred 交叉引用 + 非 RX 边界）（AC: 1, 3）
- [x] T2: 勾选 `nfr14-risk-epic38-uarttx-deepen.md` Epic 38 关闭条件；status closed（AC: 3）
- [x] T3: `deferred-work.md` 交叉引用收口（Story 38.3；全协议仍须新合同）（AC: 1）
- [x] T4: （可选）`examples/ip_box` 演示 `baud_div` 加深 API（AC: 2）
- [x] T5: 核对 `fr89_uarttx_programmable_baud` 仍绿；sprint `38-3` + `epic-38` → done；**不**开工 epic-39（AC: 1, 4）

## Dev Notes

### Previous story intelligence（38.2 / commit 293f4ea）

- UartTx：`baud_div`（clk/bit−1）+ `baud_cnt`；0≡FR82；帧推进仅 baud tick。
- ATDD `fr89_uarttx_programmable_baud` 已覆盖 elaborate→emit→tick、baud_div=0 回归、docs 非目标、无 UartRx。
- docs/ip + language-surface 已写 FR89 边界；**未**勾选 NFR14 关闭条件；deferred 仍写「→ Story 38.3」。
- 分支 B（RX）明确未交付。

### 收口 ATDD 体例（对齐 37.3）

- 读文件 assert 关键词（NFR14 `- [x]`、deferred「Story 38.3」收口、全协议仍须新合同）。
- **不要**重写 38.2 加深行为测试；收口测试与加深测试分工。

### 可选示例裁决

| 选项 | 现状 | 建议 |
|------|------|------|
| 新树外包 | 无 FR89 专用包 | 不做（成本高） |
| `examples/ip_box` 轻触 | 已有 UartTx FR82 烟测，未驱动 `baud_div` | **默认做**：设 `baud_div` 并 assert 位保持 |

### 架构合规

- AD-6 / NFR39 / AD-28；关闭条件见 NFR14 记录。
- FR89 一类加深即可关闭 Epic 38；不扩类。

### Project Structure Notes

- 一故事一提交（`process-one-story-one-commit.md`）。
- 公开品牌 Bitloom。

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 38 / Story 38.3 / FR89]
- [Source: `_agile-output/implementation-artifacts/nfr14-risk-epic38-uarttx-deepen.md`]
- [Source: `_agile-output/implementation-artifacts/38-2-uarttx-显式加深子集实现-fr89.md`]
- [Source: `_agile-output/implementation-artifacts/37-3-可选夜间真机-bambu-或显式保持-stub-fr88.md` — 收口体例]
- [Source: `_agile-output/implementation-artifacts/process-one-story-one-commit.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

- ATDD red→green: `cargo test -p bitloom --test fr89_epic38_boundary_closeout` — 5 passed（红相 2 失败：NFR14 未勾选 / deferred 开放指针）
- `cargo test -p ip_box` — ok（baud_div demo）

### Completion Notes List

- NFR14 Epic 38 关闭条件全勾；status closed — Story 38.3
- deferred item-49 / item-83 交叉引用收口；全协议仍须新合同
- 收口 ATDD `fr89_epic38_boundary_closeout`；加深 ATDD 仍绿
- 可选 `ip_box` 演示 `baud_div=1` 位保持
- sprint：`38-3: done`；`epic-38: done`；未开工 epic-39

### File List

- `crates/bitloom/tests/fr89_epic38_boundary_closeout.rs`
- `examples/ip_box/src/lib.rs`
- `_agile-output/implementation-artifacts/nfr14-risk-epic38-uarttx-deepen.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `_agile-output/implementation-artifacts/38-3-atdd-与边界收口-可选树外示例.md`
- `_agile-output/implementation-artifacts/atdd-checklist-38-3-atdd-与边界收口-可选树外示例.md`
- `_agile-output/implementation-artifacts/38-3-code-review.md`
- `_agile-output/implementation-artifacts/38-3-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Story context created (ready-for-dev) — Epic 38 FR89 ATDD/boundary closeout
- 2026-09-09: Epic 38 closeout ATDD + NFR14/deferred/ip_box；epic-38 done（Story 38.3）

## Suggested Review Order

**NFR14 关闭勾选** → **deferred 交叉引用** → **收口 ATDD** → **可选 ip_box** → **sprint epic-38 done（未开工 39）**
