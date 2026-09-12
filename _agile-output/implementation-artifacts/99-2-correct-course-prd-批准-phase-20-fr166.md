---
title: '99.2 Correct Course + PRD 批准 Phase 20（FR166）'
type: 'chore'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: '938521a Story 99.1: Epic 99 NFR14 risk record for Phase 20 gate (FR166).'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase20-nfr71-four-leftovers.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic99-phase20-nfr71-four-leftovers.md'
  - '{project-root}/_agile-output/implementation-artifacts/99-1-epic-99-nfr14-风险记录.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR166 要求 Correct Course + PRD/addendum 明确批准「NFR71 四条升格」；Correct Course finalize 已落地正文与 `correctCoursePhase20Approved`，但缺少 **Story 99.2 验收闸门**（ATDD）。

**Approach:** 以 Story 99.1 为前提，**验证** PRD/addendum Phase 20 合同戳：Phase 20 = FR166–171；Phase 12–19 关闭仍有效（NFR73）；Q1–Q5（四条全做；FR168=三者皆交付；不回滚；不扩 FR142；MSRV/AD）；品牌 Bitloom；Epic 99–104 映射；`correctCoursePhase20Approved` 可验证；`git push` 非 FR。**不**同步 README（→ 99.3）、**不**收口 AD（→ 99.4）。

## Boundaries & Constraints

**Always:** Correct Course `status: approved`；addendum Phase 20；prd amendment；Bitloom；Epic 99–104 / FR 映射；`correctCoursePhase20Approved`；99.1 NFR14 done；ATDD 绿。

**Never:** 同步 README/deferred（→ 99.3）；修订 ARCHITECTURE-SPINE 收口（→ 99.4）；勾选 Epic 99 关闭；将 epic-100..104 标 ready；改写 Phase 12–19 FR 为失败；商店 live publish / firtool 升钉（→ Epic 100/102）。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 戳齐全 | proposal approved + addendum Phase 20 + epics stamp | ATDD 绿 | N/A |
| 缺 approved / Phase 20 / stamp | 任一缺失 | ATDD 红 | 补齐合同戳 |
| 99.1 未 done | sprint 缺 99-1 done | ATDD 红 | 先完成 99.1 |

</frozen-after-approval>

## Code Map

- `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase20-nfr71-four-leftovers.md` — Correct Course **approved**
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` — Phase 20 段落
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md` — `phase20-nfr71-four-leftovers` amendment
- `_agile-output/planning-artifacts/epics.md` — `correctCoursePhase20Approved: 2026-09-12`
- `crates/bitloom/tests/fr166_prd_phase20_gate.rs` — 本故事 ATDD

## Story

As a 产品负责人,
I want PRD/addendum 经 Correct Course 明确批准「NFR71 四条升格」,
So that Phase 20 具备合同授权。

## Acceptance Criteria

1. Correct Course approved + addendum Phase 20：FR166–171；NFR73；Q1–Q5
2. 公开品牌仍为 Bitloom / `bitloom-*`
3. Epic 99–104 / FR 映射指针
4. `correctCoursePhase20Approved` 戳可验证
5. Story 99.1 NFR14 已 done

## Tasks / Subtasks

- [x] T1: 核对合同戳（AC: 1–5）— 已落地于 Correct Course finalize；本故事验收不重写
- [x] T2: ATDD `fr166_prd_phase20_gate.rs`
- [x] T3: sprint `99-2` done；epic-99 保持 in-progress；不标 100–104 ready；99.3 → ready-for-dev
- [x] T4: code-review Approve
- [x] T5: automation-summary

## Testing

- `cargo test -p bitloom --test fr166_prd_phase20_gate`
- 回归：`cargo fmt --all && just test`

## Dev Agent Record

### Completion Notes List

- 验收 Correct Course / PRD Phase 20 戳（FR166–171 / NFR73–77 / Q1–Q5）
- ATDD 绿；sprint 99-2 done；99.3 ready-for-dev；100–104 仍 backlog

### File List

- `_agile-output/implementation-artifacts/99-2-correct-course-prd-批准-phase-20-fr166.md`
- `_agile-output/implementation-artifacts/99-2-code-review.md`
- `_agile-output/implementation-artifacts/99-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `crates/bitloom/tests/fr166_prd_phase20_gate.rs`
