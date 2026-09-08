---
title: '34.4 IP 基线 ATDD + Epic 29 衔接文档'
type: 'feature'
created: '2026-09-08'
status: 'done'
baseline_commit: '0a21facd1406659fe1c092da4ef6c0cf0a16be1a'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic34-ip-baseline.md'
  - '{project-root}/_agile-output/implementation-artifacts/34-2-fifo-uart-可综合基线.md'
  - '{project-root}/_agile-output/implementation-artifacts/34-3-spi-i2c-axi-基线-或文档最小子集.md'
  - '{project-root}/docs/ip/README.md'
  - '{project-root}/crates/bitloom/tests/fr82_fifo_uart_baseline.rs'
  - '{project-root}/crates/bitloom/tests/fr82_spi_i2c_axi_baseline.rs'
warnings: []
deferred:
  - 'IP generator closures → Epic 29 / Story 29.3'
  - 'full-protocol / VIP / Full AXI deepen → explicit future contract'
  - 'AXI↔UART/FIFO interconnect fixture → deferred-work item-50'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** After 34.2–34.3 deliver all five FR82 IP classes, QA needs consolidated ATDD plus clear user/IP docs on no-closure baseline vs Epic 29 closure overlay and vs Epic 22 stub history (NFR37), and NFR14 must check off Epic 34 close conditions.

**Approach:** Add `fr82_ip_baseline_matrix` ATDD that references sibling `fr82_*_baseline` fixtures, asserts five-class elaborate→emit + no-Fn API, and locks Epic 29 handoff / NFR37 wording in `docs/ip/README.md`. Check Epic 34 close conditions on the NFR14 record. No new RTL; no closure APIs.

## Boundaries & Constraints

**Always:** FR82 ATDD consolidation; documented recipe stable; Epic 34 = no closures; Epic 29 = closure overlay; NFR37 vs stub; NFR14 close checklist; Bitloom / `bitloom-prelude`.

**Ask First:** 无。

**Never:** cargo clean / full `just test` as sole gate in this story run (targeted recipe OK); invent generator-closure IP API; claim full-protocol VIP; commit unless asked.

## I/O & Edge-Case Matrix

| Scenario | Expected |
|----------|----------|
| Sibling fixtures present | `fr82_fifo_uart_baseline` + `fr82_spi_i2c_axi_baseline` |
| Five classes elaborate→emit | SyncFifo / UartTx / Spi / I2c / Axi modules |
| Generator closures | no `Fn(` / `dyn Fn` in five elaborate impls |
| Docs handoff | Epic 34 no-closure; Epic 29 overlay; stub/NFR37 |
| NFR14 close | checklist `[x]` for Epic 34 关闭条件 |

</frozen-after-approval>

## Code Map

- `crates/bitloom/tests/fr82_ip_baseline_matrix.rs` — consolidating ATDD
- `crates/bitloom/tests/fr82_fifo_uart_baseline.rs` / `fr82_spi_i2c_axi_baseline.rs` — deep rows
- `docs/ip/README.md` — Epic 29 handoff + recipe
- `nfr14-risk-epic34-ip-baseline.md` — close conditions checked
- `docs/fr37-ip-box.md` / `language-surface.md` — handoff pointers

## Story

As a 质量负责人,
I want IP 基线自动化与「无闭包/有闭包」边界说明,
So that FR82 可回归且 Epic 29 可叠加。

## Acceptance Criteria

1. Given Story 34.2–34.3, when 增加 ATDD 覆盖已交付 IP；更新用户/IP 文档, then `just test`（或文档化配方）稳定通过（FR82）
2. Given 文档, when 阅读 IP 索引, then 声明：本 epic 无闭包；Epic 29 为闭包定制叠加点；与 stub 历史区别（NFR37）
3. Given NFR14 记录, when Story 34.4 收口, then 勾选 Epic 34 关闭条件

## Tasks / Subtasks

- [x] T1: ATDD `fr82_ip_baseline_matrix` (+ refs to sibling fr82 fixtures)（AC: 1）
- [x] T2: docs/ip Epic 29 handoff + recipe；language-surface / fr37-ip-box（AC: 1–2）
- [x] T3: NFR14 Epic 34 关闭条件勾选；Epic 29 记录注记基线已交付（AC: 3）
- [x] T4: story / code-review Approve / sprint `34-4` + `epic-34: done`

## Dev Notes

- 不实现新 RTL；五类已由 34.2–34.3 交付。
- 配方优先于本会话跑全量 `just test`（用户约束）。
- bmad-build `render_skill.py` HALT（ambiguous `implementation_artifacts`）；按 34.2/34.3 产物管道收口。

### Project Structure Notes

- IP 仍住在 `bitloom-prelude::ip`；ATDD 住在 `crates/bitloom/tests/`

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- Matrix consolidates five-class elaborate→emit + no-closure scan + docs/NFR14 assertions; deep goldens remain in sibling fr82 fixtures
- `docs/ip/README.md` Epic 29 handoff table（34 无闭包 / 29 overlay / 22 stub·NFR37）+ 文档化配方
- NFR14 Epic 34 关闭条件全勾；Epic 29 风险记录注明 FR82 基线已交付
- sprint：`34-4: done`，`epic-34: done`

### File List

- `crates/bitloom/tests/fr82_ip_baseline_matrix.rs`
- `docs/ip/README.md`
- `docs/fr37-ip-box.md`
- `_agile-output/specs/spec-rhdl/language-surface.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic34-ip-baseline.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic29-hls-ip-closures.md`
- `_agile-output/implementation-artifacts/34-4-ip-基线-atdd-epic-29-衔接文档.md`
- `_agile-output/implementation-artifacts/34-4-code-review.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-09-08: FR82 IP baseline ATDD matrix + Epic 29 handoff docs（Story 34.4）

## Suggested Review Order

**matrix ATDD** → **docs/ip handoff** → **NFR14 close checklist** → **sprint 键**
