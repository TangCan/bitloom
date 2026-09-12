---
title: '99.3 同步 README / deferred / 路线图指针（FR166）'
type: 'chore'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: '57ac503 Story 99.2: Verify Correct Course + PRD Phase 20 gate stamps (FR166).'
review_loop_iteration: 0
context:
  - '{project-root}/README.md'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/_agile-output/implementation-artifacts/99-2-correct-course-prd-批准-phase-20-fr166.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Phase 20 合同已批准，但公开 README / deferred 仍主要呈现 Phase 19 完成面，读者易把 NFR71 leftover 当成已交付或已开闸。

**Approach:** 更新 README 诚实面与 `deferred-work.md` Phase 20 pointer：区分 Phase 19 关闭 vs Phase 20 合同；列出 FR166–171；标明 Epic 99 关闭前 100–104 不得 ready；未关闭前不得宣称；`git push` 非 FR。**不**修订 AD/脊柱收口（→ 99.4）。

## Boundaries & Constraints

**Always:** Phase 19 关闭仍有效；Phase 20 FR166–171 映射；闸门未关不得 ready 100–104；Bitloom；ATDD 绿。

**Never:** 勾选 Epic 99 关闭；标 100–104 ready；实现加深。

</frozen-after-approval>

## Story

As a 维护者,
I want 公开状态页区分 Phase 19 完成面与 Phase 20 加深面,
So that 读者不把 NFR71 leftover 当成已交付。

## Acceptance Criteria

1. README 诚实面含 Phase 20 / FR166–171，并保留 Phase 19 关闭
2. deferred Phase 20 pointer + 闸门未关不得 ready 100–104
3. `git push` 非 FR；Bitloom

## Tasks / Subtasks

- [x] T1: README + deferred Phase 20 诚实面
- [x] T2: ATDD `fr166_readme_deferred_honesty.rs`
- [x] T3: sprint 99-3 done；99.4 ready-for-dev；不标 100–104 ready
- [x] T4: code-review + automation-summary

## Testing

- `cargo test -p bitloom --test fr166_readme_deferred_honesty`
- 回归：`cargo fmt --all && just test`

## Dev Agent Record

### File List

- `README.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `_agile-output/implementation-artifacts/99-3-同步-readme-deferred-路线图指针-fr166.md`
- `_agile-output/implementation-artifacts/99-3-code-review.md`
- `_agile-output/implementation-artifacts/99-3-automation-summary.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
- `crates/bitloom/tests/fr166_readme_deferred_honesty.rs`
