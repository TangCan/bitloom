---
title: '79.2 Correct Course + PRD 批准 Phase 17（FR141）'
type: 'chore'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: '6772f89 Story 79.1: Epic 79 NFR14 risk record for Phase 17 API stability gate.'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/sprint-change-proposal-2026-09-11-phase17-api-stability-1-0.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic79-phase17-api-stability-1-0.md'
  - '{project-root}/_agile-output/implementation-artifacts/79-1-epic-79-nfr14-风险记录.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR141 要求 Correct Course + PRD/addendum 明确批准「公开 API 稳定门 / Bitloom 1.0」；`03eb42d` 已落地正文与 `correctCoursePhase17Approved`，但缺少 **Story 79.2 验收闸门**（ATDD）。

**Approach:** 以 Story 79.1 为前提，**验证** PRD/addendum Phase 17 合同戳：Phase 17 = FR141–147；Phase 12–16 关闭仍有效（NFR60）；新宣称须引 FR141–146（FR147）；1.0 ≠ 清空 NFR59（NFR63）；Q1–Q5 默认；品牌 Bitloom；Epic 79–83 映射指针；`correctCoursePhase17Approved` 可验证。**不**同步 README（→ 79.3）、**不**收口 AD（→ 79.4）。**不**改写 Phase 1–16。

## Boundaries & Constraints

**Always:** Correct Course `status: approved`；addendum Phase 17；prd amendment；Bitloom；Epic 79–83 / FR 映射；`correctCoursePhase17Approved`；79.1 NFR14 done；ATDD 绿；Q1–Q5。

**Ask First:** 若撤回 Phase 17 合同 — 须改 Correct Course / PRD 与本故事。

**Never:** 同步 README/deferred（→ 79.3）；修订 ARCHITECTURE-SPINE Deferred 收口（→ 79.4）；勾选 Epic 79 关闭；将 epic-80..83 标 ready；改写 Phase 1–16 FR 为失败；执行 `cargo publish` / 升 1.0.0（→ Epic 83）。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 戳齐全 | proposal approved + addendum Phase 17 + epics stamp | ATDD 绿 | N/A |
| 缺 approved / Phase 17 / stamp | 任一缺失 | ATDD 红 | 补齐合同戳（勿重写 Phase 1–16） |
| 79.1 未 done | sprint 缺 79-1 done | ATDD 红 | 先完成 79.1 |

</frozen-after-approval>

## Code Map

- `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-11-phase17-api-stability-1-0.md` — Correct Course **approved**
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` — Phase 17 段落
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md` — `phase17-api-stability-1-0` amendment
- `_agile-output/planning-artifacts/epics.md` — `correctCoursePhase17Approved: 2026-09-11`；Epic 79–83 Inventory
- `crates/bitloom/tests/fr133_prd_phase16_gate.rs` — ATDD 样板
- `_agile-output/implementation-artifacts/nfr14-risk-epic79-phase17-api-stability-1-0.md` — 79.1 门禁
- `_agile-output/implementation-artifacts/sprint-status.yaml` — 79-2 键

## Story

As a 产品负责人,
I want PRD/addendum 经 Correct Course 明确批准「公开 API 稳定门 / Bitloom 1.0」,
So that Phase 17 具备合同授权。

## Acceptance Criteria

1. Correct Course approved + addendum Phase 17：FR141–147；NFR60；宣称须引 FR141–146（FR147）；1.0 ≠ 清空 NFR59；Q1–Q5
2. 公开品牌仍为 Bitloom / `bitloom-*`
3. Epic 79–83 / FR 映射指针（`epics.md`）
4. `correctCoursePhase17Approved` 戳可验证
5. Story 79.1 NFR14 已 done（软闸门）

## Tasks / Subtasks

- [x] T1: 核对合同戳（AC: 1–5）— 已落地于 `03eb42d`；本故事验收不重写
- [x] T2: ATDD `fr141_prd_phase17_gate.rs`
- [x] T3: sprint `79-2` done；epic-79 保持 in-progress；不标 80–83 ready
- [x] T4: code-review Approve
- [x] T5: automation-summary

## Dev Notes

- **上下文：** Correct Course / PRD / addendum / `correctCoursePhase17Approved` **已批准落地**（`03eb42d`）。本故事 = FR141 **验收闸门**（ATDD + 故事记录），非重写合同正文。
- **不**做 79.3 README/deferred、**不**做 79.4 AD 指针收口。
- 镜像 Story 72.2 / `fr133_prd_phase16_gate.rs`。
- 品牌 Bitloom；设计 crate → `bitloom-prelude` only。

## Testing

- `cargo test -p bitloom --test fr141_prd_phase17_gate`
- 回归：`cargo clean && cargo fmt --all && just test`

## Dev Agent Record

### Completion Notes List

- 验收已落地 Correct Course + addendum/prd Phase 17；`correctCoursePhase17Approved` 可验证；ATDD 11/11 绿
- sprint：79-2 done；epic-79 in-progress；80–83 仍 backlog
- code-review Approve；automation-summary 已写

### File List

- `crates/bitloom/tests/fr141_prd_phase17_gate.rs`
- `_agile-output/implementation-artifacts/79-2-correct-course-prd-批准-phase-17-fr141.md`
- `_agile-output/implementation-artifacts/79-2-code-review.md`
- `_agile-output/implementation-artifacts/79-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
