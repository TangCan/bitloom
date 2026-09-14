---
title: '111.2 Correct Course + PRD 批准 Phase 22（FR178）'
type: 'chore'
created: '2026-09-14'
status: 'done'
route: 'oneshot'
baseline_commit: '34a1262 Story 111.1: Epic 111 NFR14 risk record for Phase 22 NFR81 leftovers (FR178).'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase22-nfr81-leftovers.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic111-phase22-nfr81-leftovers.md'
  - '{project-root}/_agile-output/implementation-artifacts/111-1-epic-111-nfr14-风险记录.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR178 要求 Correct Course + PRD/addendum 明确批准「NFR81 leftovers 升格」；Correct Course finalize 已落地正文与 `correctCoursePhase22Approved`，但缺少 **Story 111.2 验收闸门**（ATDD）。

**Approach:** 以 Story 111.1 为前提，**验证** PRD/addendum Phase 22 合同戳：Phase 22 = FR178–184；Phase 12–21 关闭仍有效（NFR83）；Q1–Q7；品牌 Bitloom；Epic 111–117 映射；`correctCoursePhase22Approved` 可验证；`git push` 非 FR。**不**同步 README（→ 111.3）、**不**收口 AD（→ 111.4）。

## Boundaries & Constraints

**Always:** Correct Course `status: approved`；addendum Phase 22；prd amendment；Bitloom；Epic 111–117 / FR 映射；`correctCoursePhase22Approved`；111.1 NFR14 done；ATDD 绿。

**Never:** 同步 README/deferred（→ 111.3）；修订 ARCHITECTURE-SPINE 收口（→ 111.4）；勾选 Epic 111 关闭；将 epic-112..117 标 ready；改写 Phase 12–21 FR 为失败；浮动 HEAD live / unpaired 产品钉 / 破坏性 crates.io 发版（→ Epic 112–116）。

</frozen-after-approval>

## Code Map

- `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-12-phase22-nfr81-leftovers.md` — Correct Course **approved**
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` — Phase 22 段落
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md` — `phase22-nfr81-leftovers` amendment
- `_agile-output/planning-artifacts/epics.md` — `correctCoursePhase22Approved: 2026-09-12`
- `crates/bitloom/tests/fr178_prd_phase22_gate.rs` — 本故事 ATDD

## Story

As a 产品负责人,
I want PRD/addendum 经 Correct Course 明确批准「NFR81 leftovers 升格」,
So that Phase 22 具备合同授权。

## Acceptance Criteria

1. Correct Course approved + addendum Phase 22：FR178–184；NFR83；Q1–Q7
2. 公开品牌仍为 Bitloom / `bitloom-*`
3. Epic 111–117 / FR 映射指针
4. `correctCoursePhase22Approved` 戳可验证
5. Story 111.1 NFR14 已 done

## Tasks / Subtasks

- [x] T1: 核对合同戳（AC: 1–5）— 已落地于 Correct Course finalize；本故事验收不重写
- [x] T2: ATDD `fr178_prd_phase22_gate.rs`
- [x] T3: sprint `111-2` done；epic-111 保持 in-progress；不标 112–117 ready；111.3 → ready-for-dev
- [x] T4: code-review Approve
- [x] T5: automation-summary

## Testing

- `cargo test -p bitloom --test fr178_prd_phase22_gate`
- 回归：`cargo fmt --all && just test`

## Dev Agent Record

### Completion Notes List

- 验收 Correct Course / PRD Phase 22 戳（FR178–184 / NFR83–87 / Q1–Q7）
- ATDD 绿；sprint 111-2 done；111.3 ready-for-dev；112–117 仍 backlog

### File List

- `_agile-output/implementation-artifacts/111-2-correct-course-prd-批准-phase-22-fr178.md`
- `_agile-output/implementation-artifacts/111-2-code-review.md`
- `_agile-output/implementation-artifacts/111-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `crates/bitloom/tests/fr178_prd_phase22_gate.rs`
