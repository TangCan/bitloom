---
title: '105.3 同步 README / deferred / 路线图指针（FR172）'
type: 'chore'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: 'b95405f Story 105.2: Verify Correct Course and PRD Phase 21 gate (FR172).'
review_loop_iteration: 0
context:
  - '{project-root}/README.md'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** 公开状态页须区分 Phase 20 完成面与 Phase 21 加深面，避免读者把 NFR76 leftover 当成已交付。

**Approach:** 更新 README「诚实面」与 Phase 21 加深表；更新 `deferred-work.md` Phase 21 pointer；ATDD 锁住 FR172–177 映射、Epic 106–110 闸门、NFR78、git push 非 FR。

## Boundaries & Constraints

**Always:** Phase 20 vs 21 区分；FR172–177 列表；106–110 不得 ready 直至 Epic 105 关；Bitloom；ATDD 绿。

**Never:** 修订 ARCHITECTURE-SPINE 收口（→ 105.4）；勾选 Epic 105 关闭；将 106–110 标 ready；宣称 FR173–176 已交付。

</frozen-after-approval>

## Story

As a 维护者,
I want 公开状态页区分 Phase 20 完成面与 Phase 21 加深面,
So that 读者不把 NFR76 leftover 当成已交付。

## Acceptance Criteria

1. README 区分 Phase 20 vs Phase 21；列出 FR172–177
2. 标明 Epic 106–110 在闸门关闭前不得 ready
3. deferred-work Phase 21 pointer
4. 公开品牌 Bitloom；git push 非 FR

## Tasks / Subtasks

- [x] T1: README Phase 21 诚实面
- [x] T2: deferred-work Phase 21 pointer
- [x] T3: ATDD `fr172_readme_deferred_honesty.rs`
- [x] T4: sprint 105-3 done；105.4 ready-for-dev；106–110 仍 backlog
- [x] T5: code-review + automation-summary

## Testing

- `cargo test -p bitloom --test fr172_readme_deferred_honesty`
- 回归：`cargo fmt --all && just test`

## Dev Agent Record

### File List

- `README.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `_agile-output/implementation-artifacts/105-3-同步-readme-deferred-路线图指针-fr172.md`
- `_agile-output/implementation-artifacts/105-3-code-review.md`
- `_agile-output/implementation-artifacts/105-3-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `crates/bitloom/tests/fr172_readme_deferred_honesty.rs`
