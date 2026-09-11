---
title: '84.2 Correct Course + PRD 批准 Phase 18（FR148）'
type: 'chore'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: 'aa38b39 Story 84.1: Epic 84 NFR14 risk record for Phase 18 CLI crates.io publishability.'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/sprint-change-proposal-2026-09-11-phase18-cli-crates-io-publish.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md'
  - '{project-root}/_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic84-phase18-cli-crates-io-publish.md'
  - '{project-root}/_agile-output/implementation-artifacts/84-1-epic-84-nfr14-风险记录.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR148 要求 Correct Course + PRD/addendum 明确批准「CLI / 依赖 crate crates.io 可发布」；Correct Course finalize 已落地正文与 `correctCoursePhase18Approved`，但缺少 **Story 84.2 验收闸门**（ATDD）。

**Approach:** 以 Story 84.1 为前提，**验证** PRD/addendum Phase 18 合同戳：Phase 18 = FR148–153；Phase 17 关闭仍有效（NFR64）；CLI 上架 ≠ 清空 NFR59（NFR67）；Q 默认（rename bitloom-firrtl/viz；FR152(b)）；品牌 Bitloom；Epic 84–86 映射指针；`correctCoursePhase18Approved` 可验证。**不**同步 README（→ 84.3）、**不**收口 AD（→ 84.4）。**不**改写 Phase 1–17。

## Boundaries & Constraints

**Always:** Correct Course `status: approved`；addendum Phase 18；prd amendment；Bitloom；Epic 84–86 / FR 映射；`correctCoursePhase18Approved`；84.1 NFR14 done；ATDD 绿；Q rename + FR152(b)。

**Ask First:** 若撤回 Phase 18 合同 — 须改 Correct Course / PRD 与本故事。

**Never:** 同步 README/deferred（→ 84.3）；修订 ARCHITECTURE-SPINE Deferred 收口（→ 84.4）；勾选 Epic 84 关闭；将 epic-85..86 标 ready；改写 Phase 1–17 FR 为失败；执行 rename/`cargo publish`（→ Epic 85）。

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| 戳齐全 | proposal approved + addendum Phase 18 + epics stamp | ATDD 绿 | N/A |
| 缺 approved / Phase 18 / stamp | 任一缺失 | ATDD 红 | 补齐合同戳（勿重写 Phase 1–17） |
| 84.1 未 done | sprint 缺 84-1 done | ATDD 红 | 先完成 84.1 |

</frozen-after-approval>

## Code Map

- `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-11-phase18-cli-crates-io-publish.md` — Correct Course **approved**
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md` — Phase 18 段落
- `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md` — `phase18-cli-crates-io-publish` amendment
- `_agile-output/planning-artifacts/epics.md` — `correctCoursePhase18Approved: 2026-09-11`；Epic 84–86 Inventory
- `crates/bitloom/tests/fr148_prd_phase18_gate.rs` — 本故事 ATDD
- `_agile-output/implementation-artifacts/nfr14-risk-epic84-phase18-cli-crates-io-publish.md` — 84.1 门禁
- `_agile-output/implementation-artifacts/sprint-status.yaml` — 84-2 键

## Story

As a 产品负责人,
I want PRD/addendum 经 Correct Course 明确批准「CLI / 依赖 crate crates.io 可发布」,
So that Phase 18 具备合同授权。

## Acceptance Criteria

1. Correct Course approved + addendum Phase 18：FR148–153；NFR64；宣称须引 FR148–153；CLI 上架 ≠ 清空 NFR59；Q 默认（rename bitloom-*；FR152(b)）
2. 公开品牌仍为 Bitloom / `bitloom-*`
3. Epic 84–86 / FR 映射指针（`epics.md`）
4. `correctCoursePhase18Approved` 戳可验证
5. Story 84.1 NFR14 已 done（软闸门）

## Tasks / Subtasks

- [x] T1: 核对合同戳（AC: 1–5）— 已落地于 Correct Course finalize；本故事验收不重写
- [x] T2: ATDD `fr148_prd_phase18_gate.rs`
- [x] T3: sprint `84-2` done；epic-84 保持 in-progress；不标 85–86 ready
- [x] T4: code-review Approve
- [x] T5: automation-summary

## Dev Notes

- **上下文：** Correct Course / PRD / addendum / `correctCoursePhase18Approved` **已批准落地**。本故事 = FR148 **验收闸门**（ATDD + 故事记录），非重写合同正文。
- **不**做 84.3 README/deferred、**不**做 84.4 AD 指针收口。
- 镜像 Story 79.2 / `fr141_prd_phase17_gate.rs`。
- 品牌 Bitloom；设计 crate → `bitloom-prelude` only。

## Testing

- `cargo test -p bitloom --test fr148_prd_phase18_gate`
- 回归：`cargo clean && cargo fmt --all && just test`

## Dev Agent Record

### Completion Notes List

- 验收已落地 Correct Course + addendum/prd Phase 18；`correctCoursePhase18Approved` 可验证；ATDD 绿
- sprint：84-2 done；epic-84 in-progress；85–86 仍 backlog
- code-review Approve；automation-summary 已写

### File List

- `crates/bitloom/tests/fr148_prd_phase18_gate.rs`
- `_agile-output/implementation-artifacts/84-2-correct-course-prd-批准-phase-18-fr148.md`
- `_agile-output/implementation-artifacts/84-2-code-review.md`
- `_agile-output/implementation-artifacts/84-2-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
