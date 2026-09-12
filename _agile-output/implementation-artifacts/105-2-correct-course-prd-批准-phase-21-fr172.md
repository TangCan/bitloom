---
title: '105.2 Correct Course + PRD 批准 Phase 21（FR172）'
type: 'chore'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: '33aa4c2 Story 105.1: Epic 105 NFR14 risk record for Phase 21 NFR76 leftovers (FR172).'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase21-nfr76-leftovers.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic105-phase21-nfr76-leftovers.md'
  - '{project-root}/_agile-output/implementation-artifacts/105-1-epic-105-nfr14-风险记录.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR172 要求 Correct Course + PRD/addendum 明确批准「NFR76 leftovers 升格」；Correct Course finalize 已落地正文与 `correctCoursePhase21Approved`，但缺少 **Story 105.2 验收闸门**（ATDD）。

**Approach:** 以 Story 105.1 为前提，**验证** PRD/addendum Phase 21 合同戳：Phase 21 = FR172–177；Phase 12–20 关闭仍有效（NFR78）；Q1–Q5；品牌 Bitloom；Epic 105–110 映射；`correctCoursePhase21Approved` 可验证；`git push` 非 FR。**不**同步 README（→ 105.3）、**不**收口 AD（→ 105.4）。

## Boundaries & Constraints

**Always:** Correct Course `status: approved`；addendum Phase 21；prd amendment；Bitloom；Epic 105–110 / FR 映射；`correctCoursePhase21Approved`；105.1 NFR14 done；ATDD 绿。

**Never:** 同步 README/deferred（→ 105.3）；修订 ARCHITECTURE-SPINE 收口（→ 105.4）；勾选 Epic 105 关闭；将 epic-106..110 标 ready；改写 Phase 12–20 FR 为失败；firtool live 升钉 / HEAD 二进制（→ Epic 106/107）。

</frozen-after-approval>

## Code Map

- `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase21-nfr76-leftovers.md` — Correct Course **approved**
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` — Phase 21 段落
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md` — `phase21-nfr76-leftovers` amendment
- `_agile-output/planning-artifacts/epics.md` — `correctCoursePhase21Approved: 2026-09-12`
- `crates/bitloom/tests/fr172_prd_phase21_gate.rs` — 本故事 ATDD

## Story

As a 产品负责人,
I want PRD/addendum 经 Correct Course 明确批准「NFR76 leftovers 升格」,
So that Phase 21 具备合同授权。

## Acceptance Criteria

1. Correct Course approved + addendum Phase 21：FR172–177；NFR78；Q1–Q5
2. 公开品牌仍为 Bitloom / `bitloom-*`
3. Epic 105–110 / FR 映射指针
4. `correctCoursePhase21Approved` 戳可验证
5. Story 105.1 NFR14 已 done

## Tasks / Subtasks

- [x] T1: 核对合同戳（AC: 1–5）— 已落地于 Correct Course finalize；本故事验收不重写
- [x] T2: ATDD `fr172_prd_phase21_gate.rs`
- [x] T3: sprint `105-2` done；epic-105 保持 in-progress；不标 106–110 ready；105.3 → ready-for-dev
- [x] T4: code-review Approve
- [x] T5: automation-summary

## Testing

- `cargo test -p bitloom --test fr172_prd_phase21_gate`
- 回归：`cargo fmt --all && just test`

## Dev Agent Record

### Completion Notes List

- 验收 Correct Course / PRD Phase 21 戳（FR172–177 / NFR78–82 / Q1–Q5）
- ATDD 绿；sprint 105-2 done；105.3 ready-for-dev；106–110 仍 backlog

### File List

- `_agile-output/implementation-artifacts/105-2-correct-course-prd-批准-phase-21-fr172.md`
- `_agile-output/implementation-artifacts/105-2-code-review.md`
- `_agile-output/implementation-artifacts/105-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `crates/bitloom/tests/fr172_prd_phase21_gate.rs`
