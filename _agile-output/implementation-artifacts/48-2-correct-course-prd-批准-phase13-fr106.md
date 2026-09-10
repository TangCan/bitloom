---
title: '48.2 Correct Course + PRD 批准 Phase 13（FR106）'
type: 'chore'
created: '2026-09-10'
status: 'done'
baseline_commit: '0acaa0e'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/sprint-change-proposal-2026-09-10-phase13-mvp-commercial-deepen.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/48-1-epic-48-nfr14-风险记录.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic48-mvp-commercial-deepen.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR106 要求 Correct Course + PRD/addendum 明确批准「MVP→商业加深」；`501f3be` 已落地正文，但缺少 **Story 48.2 验收闸门**（ATDD），易把「提案已合入」误判为 FR106 PRD 门关闭，或漏掉 NFR44 / Bitloom / Epic 48–56 指针。

**Approach:** 以 Story 48.1 完成与已批准 Correct Course 为前提，**验证** PRD/addendum Phase 13 合同戳：Phase 13 = FR106–115 加深；Phase 12 MVP 关闭仍有效（NFR44）；新宣称须引 FR106–114（FR115）；品牌 Bitloom/`bitloom-*`；Epic 48–56 映射指针。用 ATDD 锁住提案 `status: approved`、addendum Phase 13 节、prd amendment。本故事**不**同步 README/deferred（→ 48.3）、**不**修订 AD 指针收口（→ 48.4）、**不**开闸 Epic 49–56。

## Boundaries & Constraints

**Always:** Correct Course approved；addendum「Phase 13」；NFR44；FR106–115；Bitloom；48.1 NFR14 门已满足。

**Ask First:** 若产品撤回 Phase 13 — 须新 Correct Course。

**Never:** README/deferred 正文同步（→ 48.3）；ARCHITECTURE-SPINE Deferred 收口（→ 48.4）；勾选 Epic 48 关闭；将 Epic 49–56 标 ready；实现 FR107+ 代码。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 合同戳齐全 | Correct Course `approved` + addendum Phase 13 + prd amendment + Bitloom | ATDD 绿 | N/A |
| 缺 Phase 13 节 / 缺 NFR44 | addendum 无加深边界 | ATDD 红 | 补齐后再绿 |
| 越界改 README / AD | 触及 48.3/48.4 | 拒绝 | 审查拦截 |

</frozen-after-approval>

## Code Map

- `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-10-phase13-mvp-commercial-deepen.md`
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` / `prd.md`
- `_agile-output/planning-artifacts/epics.md`
- `crates/bitloom/tests/fr94_prd_path_b_gate.rs` — 样板
- `crates/bitloom/tests/fr106_prd_phase13_gate.rs` — 本故事 ATDD

## Story

As a 产品负责人,
I want PRD/addendum 经 Correct Course 明确批准「MVP→商业加深」,
So that Phase 13 具备合同授权。

## Acceptance Criteria

1. Given Story 48.1；现 addendum 已含 Phase 13, when 验收 sprint-change-proposal + PRD/addendum Phase 13 段落, then 写明：Phase 13 = FR106–115 加深；Phase 12 MVP 关闭仍有效；新宣称须引 FR106–114（FR115 / NFR44）
2. And 公开品牌仍为 Bitloom / `bitloom-*`
3. And 列出 Epic 48–56 与 FR 映射指针（可指向 `epics.md`）

## Tasks / Subtasks

- [x] T1: 核对 Correct Course `approved` + addendum/prd Phase 13 合同戳（AC: 1–3）
- [x] T2: ATDD `fr106_prd_phase13_gate.rs`（AC: 1–3）
- [x] T3: sprint → `48-2` done；**不**开工 48.3+ / 49–56
- [x] T4: code-review 记录 Approve

## Dev Notes

- **已落地（501f3be）：** Correct Course 全文；addendum Phase 13；prd amendment；epics 戳记；sprint Epic 48–56 播种。本故事是 **验收/ATDD**，不是重写提案。
- **仍待 48.3：** README / deferred 诚实面。
- **仍待 48.4：** AD 指针 + Epic 48 关闭勾选。

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- ATDD `fr106_prd_phase13_gate` 验收已落地合同戳；审查 Approve
- sprint：`48-2: done`；`48-3`/`48-4` 仍 backlog

### File List

- `_agile-output/implementation-artifacts/48-2-correct-course-prd-批准-phase13-fr106.md`
- `_agile-output/implementation-artifacts/48-2-code-review.md`
- `_agile-output/implementation-artifacts/48-2-automation-summary.md`
- `_agile-output/implementation-artifacts/atdd-checklist-48-2-correct-course-prd-批准-phase13-fr106.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `crates/bitloom/tests/fr106_prd_phase13_gate.rs`

## Change Log

- 2026-09-10: Story 48.2 FR106 PRD/Correct Course 验收闸门 + ATDD
