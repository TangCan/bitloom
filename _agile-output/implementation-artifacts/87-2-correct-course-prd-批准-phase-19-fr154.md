---
title: '87.2 Correct Course + PRD 批准 Phase 19（FR154）'
type: 'chore'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: 'd4df19d Story 87.1: Epic 87 NFR14 risk record for Phase 19 gate (FR154).'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase19-nfr59-fr152a.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic87-phase19-nfr59-fr152a.md'
  - '{project-root}/_agile-output/implementation-artifacts/87-1-epic-87-nfr14-风险记录.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR154 要求 Correct Course + PRD/addendum 明确批准「NFR59 全子集 + FR152(a)」；Correct Course finalize 已落地正文与 `correctCoursePhase19Approved`，但缺少 **Story 87.2 验收闸门**（ATDD）。

**Approach:** 以 Story 87.1 为前提，**验证** PRD/addendum Phase 19 合同戳：Phase 19 = FR154–165；Phase 12–18 关闭仍有效（NFR68）；Q1–Q5（NFR59 全做；FR152(a) live；不回滚；不扩 FR142；MSRV）；品牌 Bitloom；Epic 87–98 映射；`correctCoursePhase19Approved` 可验证；`git push` 非 FR。**不**同步 README（→ 87.3）、**不**收口 AD（→ 87.4）。

## Boundaries & Constraints

**Always:** Correct Course `status: approved`；addendum Phase 19；prd amendment；Bitloom；Epic 87–98 / FR 映射；`correctCoursePhase19Approved`；87.1 NFR14 done；ATDD 绿。

**Never:** 同步 README/deferred（→ 87.3）；修订 ARCHITECTURE-SPINE 收口（→ 87.4）；勾选 Epic 87 关闭；将 epic-88..98 标 ready；改写 Phase 12–18 FR 为失败；live publish（→ Epic 88）。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 戳齐全 | proposal approved + addendum Phase 19 + epics stamp | ATDD 绿 | N/A |
| 缺 approved / Phase 19 / stamp | 任一缺失 | ATDD 红 | 补齐合同戳 |
| 87.1 未 done | sprint 缺 87-1 done | ATDD 红 | 先完成 87.1 |

</frozen-after-approval>

## Code Map

- `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase19-nfr59-fr152a.md` — Correct Course **approved**
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` — Phase 19 段落
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md` — `phase19-nfr59-fr152a` amendment
- `_agile-output/planning-artifacts/epics.md` — `correctCoursePhase19Approved: 2026-09-12`
- `crates/bitloom/tests/fr154_prd_phase19_gate.rs` — 本故事 ATDD

## Story

As a 产品负责人,
I want PRD/addendum 经 Correct Course 明确批准「NFR59 全子集升格 + FR152(a)」,
So that Phase 19 具备合同授权。

## Acceptance Criteria

1. Correct Course approved + addendum Phase 19：FR154–165；NFR68；Q1–Q5
2. 公开品牌仍为 Bitloom / `bitloom-*`
3. Epic 87–98 / FR 映射指针
4. `correctCoursePhase19Approved` 戳可验证
5. Story 87.1 NFR14 已 done

## Tasks / Subtasks

- [x] T1: 核对合同戳（AC: 1–5）— 已落地于 Correct Course finalize；本故事验收不重写
- [x] T2: ATDD `fr154_prd_phase19_gate.rs`
- [x] T3: sprint `87-2` done；epic-87 保持 in-progress；不标 88–98 ready
- [x] T4: code-review Approve
- [x] T5: automation-summary

## Testing

- `cargo test -p bitloom --test fr154_prd_phase19_gate`
- 回归：`cargo fmt --all && just test`

## Dev Agent Record

### Completion Notes List

- 验收已落地 Correct Course + addendum/prd Phase 19；`correctCoursePhase19Approved` 可验证；ATDD 绿
- sprint：87-2 done；epic-87 in-progress；88–98 仍 backlog
- code-review Approve

### File List

- `crates/bitloom/tests/fr154_prd_phase19_gate.rs`
- `_agile-output/implementation-artifacts/87-2-correct-course-prd-批准-phase-19-fr154.md`
- `_agile-output/implementation-artifacts/87-2-code-review.md`
- `_agile-output/implementation-artifacts/87-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
