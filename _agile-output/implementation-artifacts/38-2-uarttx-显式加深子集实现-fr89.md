---
title: '38.2 UartTx 显式加深子集实现（FR89）'
type: 'feature'
created: '2026-09-09'
status: 'done'
baseline_commit: 'd8985b3'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic38-uarttx-deepen.md'
  - '{project-root}/_agile-output/implementation-artifacts/38-1-epic-38-nfr14-风险记录.md'
  - '{project-root}/docs/ip/README.md'
  - '{project-root}/crates/bitloom-prelude/src/ip.rs'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md'
warnings: []
deferred:
  - 'Epic 38 关闭条件勾选 / 加深路径 ATDD 收口 / deferred-work 交叉引用 → Story 38.3'
  - '可选树外 examples 包演示加深 API → Story 38.3（不做不阻塞）'
  - '最小 RX（分支 B）→ 本 epic 明确不交付；须新合同'
  - '全协议 / VIP / 小数分频 / 波特率表全家桶 → standing deferred；须新合同'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR82 `UartTx` 仍为 **baud = clk**（1 bit/clk）。Epic 38 / FR89 要求在 NFR14 已钉死的加深分支上交付**一类**可验收深度；38.1 已选定 **分支 A（可编程波特率）**，尚未实现。

**Approach:** 加深 `bitloom_prelude::ip::UartTx`：在既有 8N1 TX 移位/帧状态机上叠加**可编程分频/波特计数**（输入端口驱动位时序）；保持 `baud_div=0` 时与 FR82 行为一致以便回归；至少一夹具 elaborate → emit `.v` → tick 证明加深语义；更新 `docs/ip/README.md` 写明本 epic 交付子集与明确非目标。**不**交付 RX / 全双工 / VIP；**不**勾选 Epic 38 关闭条件（→ 38.3）。

## Boundaries & Constraints

**Always:** 分支 A 可编程波特率；仍 8N1 TX；elaborate → emit → tick 证据；文档边界 + 明确非目标；NFR39（不得声称分支 B）；设计 crate 只依赖 `bitloom-prelude`（AD-6）；品牌 Bitloom；既有 FR82 / fr78 等 baud=clk 夹具在默认 `baud_div=0` 下仍绿。

**Ask First:** 无（分支已由 38.1 钉死为 A）。

**Never:** 实现最小 RX / 全双工 / VIP / 全协议；静默扩 SPI/I2C/AXI；勾选 NFR14 Epic 38 关闭条件（→ 38.3）；开工 38.3 树外示例 / deferred-work 收口故事本体；引入生成器闭包定制 API（Epic 29）；把 FR82 基线话术改成「早已含可编程 baud」而不交付分频语义。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| FR82 兼容 | `baud_div=0`（或未驱动→0）+ 写字节 | 1 clk/bit；既有帧时序不变 | 回归失败则修 |
| 加深分频 | `baud_div=N`（N>0）+ 写字节 | 每位 `N+1` clk；start/data/stop 仍 8N1 | ATDD assert |
| busy 门控 | busy 期间 wr_en | 载荷不被替换 | 同 FR82 |
| 文档边界 | README | 写明可编程波特率子集；非目标含 RX/VIP/全双工 | 审查 |
| 分支 B | 文档/测试标题 | **不得**声称 RX 已交付 | NFR39 |
| emit | elaborate | `.v` 含 `baud_div`（或文档等价名）与 `tx` | assert |

</frozen-after-approval>

## Code Map

- `crates/bitloom-prelude/src/ip.rs` — **UPDATE** `UartTx`：加 `baud_div` 输入 + `baud_cnt`（或等价）分频；文档注释从「non-goals: programmable baud」改为「FR89 加深：可编程分频；仍非 RX/VIP」
- `crates/bitloom-prelude/src/ip.rs` — **UPDATE** `uart_tx_*` 单元测试：覆盖 `baud_div=0` 回归 + `baud_div>0` 加深语义
- `docs/ip/README.md` — **UPDATE** UART 行：FR89 / Epic 38 可编程波特率子集；明确非目标
- `_agile-output/specs/spec-rhdl/language-surface.md` — **UPDATE**（若表面描述仍写死 baud=clk only）
- `crates/bitloom/tests/fr82_fifo_uart_baseline.rs` / `fr78_bridge_*` / `fr82_ip_baseline_matrix.rs` — **核对**：默认 0 分频须仍绿；仅在破坏 ABI 时最小修补
- `examples/ip_box/` — **核对**（若驱动 UartTx）；本故事不强制新树外包（→ 38.3 可选）
- ATDD（本管道 `bmad-testarch-atdd`）：新建 `crates/bitloom/tests/fr89_uarttx_programmable_baud.rs`（或等价名）锁 deepen
- `_agile-output/implementation-artifacts/nfr14-risk-epic38-uarttx-deepen.md` — **勿**勾选关闭条件（38.3）
- `_agile-output/implementation-artifacts/deferred-work.md` — 本故事可轻触叙事（可编程 baud **子集**已合同化）；完整交叉引用收口 → 38.3

## Story

As a IP 集成者,
I want UartTx 在文档化加深子集上可 elaborate/emit/tick,
So that FR89 有一类可验收深度。

## Acceptance Criteria

1. Given Story 38.1 已选定分支 **A（可编程波特率）**，when 实现该分支的 UartTx 加深行为（prelude → HIR → emit `.v`），then 至少一夹具 elaborate → emit → tick（或文档等价）证明加深语义（FR89）
2. Given 文档，when 阅读 `docs/ip/README.md`（或等价），then 写明本 epic 交付子集与**明确非目标**（全协议/VIP/全双工/RX 等）
3. Given NFR39，when 交付本故事，then 未选中的分支 B（最小 RX）**不得**声称已交付
4. Given AD-6，when 设计 crate 依赖，then 仍只依赖 `bitloom-prelude`

## Tasks / Subtasks

- [x] T1: `UartTx` 加可编程波特分频（`baud_div` = clocks/bit−1；0≡FR82）（AC: 1）
- [x] T2: prelude 单元测试 + FR89 ATDD 夹具 elaborate→emit→tick（AC: 1）
- [x] T3: 更新 `docs/ip/README.md`（及必要 language-surface）边界文案（AC: 2–3）
- [x] T4: 回归 FR82/fr78/ip_box；确认无闭包 API、无 RX 宣称（AC: 3–4）
- [x] T5: sprint → `38-2: done`（经 review）；**不**开工 38.3 / **不**勾选 Epic 38 关闭条件

## Dev Notes

### 推荐语义（最小可测）

- 新增输入 **`baud_div`**：`UInt` 宽建议 8；语义 = **每位时钟数 − 1**。
  - `0` → 1 clk/bit（与 FR82 / 现有未驱动端口 `lookup→0` 兼容）。
  - `N>0` → 每位 `N+1` 个 `clk` 后才推进 `bit_idx` / 移位。
- 新增寄存器 **`baud_cnt`**：busy 期间计数；`baud_cnt == baud_div` 时产生 baud tick 并清零，否则 `+1`；accept 时清零。
- **仅在 baud tick** 上推进帧状态（start→data→stop→idle）；`tx` 线在整个位期内保持稳定。
- **不要**改模块名 `UartTx`；**不要**拆新 crate。
- Accept / busy 门控逻辑保持 FR82：`accept = wr_en && !busy`。

### 当前状态（UPDATE 前必读）

- `UartTx::elaborate`：端口 `clk/rst/wr_en/wr_data` → `tx/tx_byte/tx_busy`；regs `hold/shift_reg/busy/bit_idx`；每位一拍。
- 注释仍列 `programmable baud divider` 为 non-goal — **本故事推翻该 non-goal（仅分频子集）**，保留 RX/parity/FIFO'd TX/closures 为 non-goal。
- 测试驱动（`uart_drive` / fr82 / fr78）**不**设 `baud_div` → 必须保持 0≡1 clk/bit。

### 架构合规

- AD-6：设计只依赖 prelude。
- AD-28 / NFR14：门禁已由 38.1 打开；本故事实现加深，不重开分支选择。
- NFR39：禁止静默扩大；文档必须写明非目标。
- AD-18：无捕获闭包进入 tick。

### Project Structure Notes

- IP 仍住 `bitloom-prelude::ip`（与 34.2 一致）。
- 新 ATDD 放 `crates/bitloom/tests/`，命名对齐 `fr89_*`。
- 一故事一提交（`process-one-story-one-commit.md`）。

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 38 / Story 38.2]
- [Source: `_agile-output/implementation-artifacts/nfr14-risk-epic38-uarttx-deepen.md` — 分支 A]
- [Source: `crates/bitloom-prelude/src/ip.rs` — 当前 UartTx]
- [Source: `docs/ip/README.md` — FR82 UART 限制]
- [Source: `_agile-output/implementation-artifacts/34-2-fifo-uart-可综合基线.md` — 基线模式]
- [Source: `_agile-output/implementation-artifacts/process-one-story-one-commit.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

### Completion Notes List

- UartTx：`baud_div`（clk/bit−1）+ `baud_cnt`；0≡FR82；帧推进仅 baud tick
- ATDD `fr89_uarttx_programmable_baud` + prelude 单元测绿；审查 Approve；automate 认定 ATDD 足够
- docs/ip + language-surface + deferred-work 轻触；未勾选 Epic 38 关闭条件；未开工 38.3
- sprint：`38-2: done`；`38-3` 仍 backlog

### File List

- `crates/bitloom-prelude/src/ip.rs`
- `crates/bitloom/tests/fr89_uarttx_programmable_baud.rs`
- `docs/ip/README.md`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `_agile-output/implementation-artifacts/38-2-uarttx-显式加深子集实现-fr89.md`
- `_agile-output/implementation-artifacts/38-2-code-review.md`
- `_agile-output/implementation-artifacts/38-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-09: Story context created (ready-for-dev) — UartTx FR89 branch A programmable baud
- 2026-09-09: Implement FR89 programmable baud deepen（Story 38.2）

## Suggested Review Order

**ip.rs UartTx 分频** → **FR89 ATDD + baud_div=0 回归** → **docs/ip 边界** → **sprint 键（38.3 仍 backlog）**
