---
title: '57.2 Correct Course + PRD 批准 Phase 14（FR116）'
type: 'chore'
created: '2026-09-10'
status: 'done'
baseline_commit: '3222e34'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/sprint-change-proposal-2026-09-10-phase14-nfr47-deferred-deepen.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/57-1-epic-57-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic57-phase14-nfr47-deferred-deepen.md'
  - '{project-root}/_agile-output/implementation-artifacts/48-2-correct-course-prd-批准-phase13-fr106.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR116 要求 Correct Course + PRD/addendum 明确批准「NFR47 未选加深升格」；`6781d53` 已落地正文，但缺少 **Story 57.2 验收闸门**（ATDD），易把「提案已合入」误判为 FR116 PRD 门关闭，或漏掉 NFR48 / Bitloom / Epic 57–63 指针。

**Approach:** 以 Story 57.1 完成与已批准 Correct Course 为前提，**验证** PRD/addendum Phase 14 合同戳：Phase 14 = FR116–123 加深；Phase 12/13 关闭仍有效（NFR48）；新宣称须引 FR116–122（FR123）；品牌 Bitloom/`bitloom-*`；Epic 57–63 映射指针。用 ATDD 锁住提案 `status: approved`、addendum Phase 14 节、prd amendment。本故事**不**同步 README/deferred（→ 57.3）、**不**修订 AD 指针收口（→ 57.4）、**不**开闸 Epic 58–63。

## Boundaries & Constraints

**Always:** Correct Course approved；addendum「Phase 14」；NFR48；FR116–123；Bitloom；57.1 NFR14 门已满足。

**Ask First:** 若产品撤回 Phase 14 — 须新 Correct Course。

**Never:** README/deferred 正文同步（→ 57.3）；ARCHITECTURE-SPINE Deferred 收口（→ 57.4）；勾选 Epic 57 关闭；将 Epic 58–63 标 ready；实现 FR117+ 代码。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 合同戳齐全 | Correct Course `approved` + addendum Phase 14 + prd amendment + Bitloom | ATDD 绿 | N/A |
| 缺 Phase 14 节 / 缺 NFR48 | addendum 无加深边界 | ATDD 红 | 补齐后再绿 |
| 越界改 README / AD | 触及 57.3/57.4 | 拒绝 | 审查拦截 |

</frozen-after-approval>

## Code Map

- `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-10-phase14-nfr47-deferred-deepen.md`
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` / `prd.md`
- `_agile-output/planning-artifacts/epics.md`
- `crates/bitloom/tests/fr106_prd_phase13_gate.rs` — 样板
- `crates/bitloom/tests/fr116_prd_phase14_gate.rs` — 本故事 ATDD

## Story

As a 产品负责人,
I want PRD/addendum 经 Correct Course 明确批准「NFR47 未选加深升格」,
So that Phase 14 具备合同授权。

## Acceptance Criteria

1. Given Story 57.1；现 addendum 已含 Phase 14, when 验收 sprint-change-proposal + PRD/addendum Phase 14 段落, then 写明：Phase 14 = FR116–123 加深；Phase 12/13 关闭仍有效；新宣称须引 FR116–122（FR123 / NFR48）
2. And 公开品牌仍为 Bitloom / `bitloom-*`
3. And 列出 Epic 57–63 与 FR 映射指针（可指向 `epics.md`）

## Tasks / Subtasks

- [x] T1: 核对 Correct Course `approved` + addendum/prd Phase 14 合同戳（AC: 1–3）
- [x] T2: ATDD `fr116_prd_phase14_gate.rs`（AC: 1–3）
- [x] T3: sprint → `57-2` done；**不**开工 57.3+ / 58–63
- [x] T4: code-review 记录 Approve

## Dev Notes

- **已落地（6781d53）：** Correct Course 全文；addendum Phase 14；prd amendment；epics 戳记；sprint Epic 57–63 播种。本故事是 **验收/ATDD**，不是重写提案。
- **仍待 57.3：** README / deferred 诚实面。
- **仍待 57.4：** AD 指针 + Epic 57 关闭勾选。
- **Known issue：** 修复 `fr114_epic56_closeout` 对重命名 sprint 键的断言。

### Project Structure Notes

- 测试落在 `crates/bitloom/tests/` 与 `fr106_prd_phase13_gate.rs` 同形
- 不改 README / deferred / ARCHITECTURE-SPINE

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Epic 57 / Story 57.2]
- [Source: `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-10-phase14-nfr47-deferred-deepen.md`]
- [Source: `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` — Phase 14]
- [Source: `_agile-output/implementation-artifacts/48-2-correct-course-prd-批准-phase13-fr106.md`]
- [Source: `_agile-output/implementation-artifacts/process-one-story-one-commit.md`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- 核对 Correct Course / addendum / prd Phase 14 合同戳完整，无缺口
- ATDD `fr116_prd_phase14_gate` 7 用例绿
- code-review Approve；sprint `57-2: done`；`57-3`/`57-4` 仍 backlog；Epic 58–63 仍 backlog
- 顺带修复多处过时 sprint 键断言（Phase 14 tracking 重命名）

### File List

- `_agile-output/implementation-artifacts/57-2-correct-course-prd-批准-phase-14-fr116.md`
- `_agile-output/implementation-artifacts/57-2-code-review.md`
- `_agile-output/implementation-artifacts/57-2-automation-summary.md`
- `_agile-output/implementation-artifacts/atdd-checklist-57-2-correct-course-prd-批准-phase-14-fr116.md`
- `_agile-output/implementation-artifacts/nfr14-risk-epic57-phase14-nfr47-deferred-deepen.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `crates/bitloom/tests/fr116_prd_phase14_gate.rs`
- `crates/bitloom/tests/fr91_bitloom_lsp_explicit_defer.rs`
- `crates/bitloom/tests/fr92_shared_stimulus_adapter.rs`
- `crates/bitloom/tests/fr98_epic43_closeout.rs`
- `crates/bitloom/tests/fr99_bitloom_lsp_full_elaborate.rs`
- `crates/bitloom/tests/fr99_bitloom_lsp_server_mvp.rs`
- `crates/bitloom/tests/fr101_epic46_closeout.rs`
- `crates/bitloom/tests/fr101_systemc_tlm_product.rs`
- `crates/bitloom/tests/fr102_multiview_attribute_matrix.rs`
- `crates/bitloom/tests/fr103_epic45_closeout.rs`
- `crates/bitloom/tests/fr104_interactive_wave.rs`
- `crates/bitloom/tests/fr105_epic47_closeout.rs`
- `crates/bitloom/tests/fr106_ad_pointer_epic48_close.rs`
- `crates/bitloom/tests/fr109_epic51_closeout.rs`
- `crates/bitloom/tests/fr110_epic52_closeout.rs`
- `crates/bitloom/tests/fr111_epic53_closeout.rs`
- `crates/bitloom/tests/fr112_epic54_closeout.rs`
- `crates/bitloom/tests/fr113_epic55_closeout.rs`
- `crates/bitloom/tests/fr114_epic56_closeout.rs`

## Change Log

- 2026-09-10: Story 57.2 FR116 PRD/Correct Course 验收闸门 + ATDD
