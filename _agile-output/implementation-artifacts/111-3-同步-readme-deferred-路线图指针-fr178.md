---
title: '111.3 同步 README / deferred / 路线图指针（FR178）'
type: 'chore'
created: '2026-09-14'
status: 'done'
route: 'oneshot'
baseline_commit: '5c34fcd Story 111.2: Verify Correct Course and PRD Phase 22 gate (FR178).'
review_loop_iteration: 0
context:
  - '{project-root}/README.md'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/docs/fr177-phase21-claim-honesty.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** 公开 README / deferred 仍以 Phase 21 为最新加深面，读者易把 NFR81 leftover 当成已交付。

**Approach:** 区分 Phase 21 完成面 vs Phase 22 加深面；列出 FR178–184；标明 Epic 112–117 闸门前不得 ready；更新 fr177 NFR81→Phase 22 指针。**不**收口 AD（→ 111.4）。

## Boundaries & Constraints

**Always:** README / deferred Phase 22 指针；Bitloom；112–117 不得 ready；NFR83/NFR86/NFR87；ATDD 绿。

**Never:** 勾选 Epic 111 关闭（→ 111.4）；将 112–117 标 ready；实现 FR179–183。

</frozen-after-approval>

## Story

As a 维护者,
I want 公开状态页区分 Phase 21 完成面与 Phase 22 加深面,
So that 读者不把 NFR81 leftover 当成已交付。

## Acceptance Criteria

1. README 明确 Phase 21 vs Phase 22 与 FR178–184 映射
2. 标明 Epic 112–117 在闸门关闭前不得 ready
3. deferred-work Phase 22 pointer；公开品牌 Bitloom

## Tasks / Subtasks

- [x] T1: README 诚实面 + Phase 22 表
- [x] T2: deferred-work Phase 22 pointer；fr177 NFR81→Phase 22
- [x] T3: ATDD `fr178_readme_deferred_honesty.rs`
- [x] T4: sprint 111-3 done；111-4 ready-for-dev；112–117 backlog
- [x] T5: code-review + automation-summary

## Testing

- `cargo test -p bitloom --test fr178_readme_deferred_honesty`
- 回归：`cargo fmt --all && just test`

## Dev Agent Record

### File List

- `README.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `docs/fr177-phase21-claim-honesty.md`
- `crates/bitloom/tests/fr178_readme_deferred_honesty.rs`
- `_agile-output/implementation-artifacts/111-3-同步-readme-deferred-路线图指针-fr178.md`
- `_agile-output/implementation-artifacts/111-3-code-review.md`
- `_agile-output/implementation-artifacts/111-3-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
