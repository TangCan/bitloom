---
title: '87.3 同步 README / deferred / 路线图指针（FR154）'
type: 'chore'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: 'ddae902 Story 87.2: Verify Phase 19 Correct Course + PRD gate (FR154).'
review_loop_iteration: 0
context:
  - '{project-root}/README.md'
  - '{project-root}/_agile-output/implementation-artifacts/deferred-work.md'
  - '{project-root}/docs/requirements/19. 实施路线图.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Phase 19 合同已批，但 README / deferred 诚实面仍可能用「NFR59 仍 deferred」或 Phase 18 alone 混淆 lsp/NFR59 加深状态。

**Approach:** 同步 README「状态与 deferred」、`deferred-work.md`、doc-19 指针：区分 Phase 18 关闭 vs Phase 19 合同进行中；FR154–165 表；未关 FR 前不得宣称；`git push` 非 FR。**不**收口 AD / Epic 87 关闭（→ 87.4）。

## Boundaries & Constraints

**Always:** Phase 18 关闭仍有效；Phase 19 合同已批准；FR 表；Bitloom；ATDD 绿。

**Never:** 勾选 Epic 87 关闭；将 88–98 ready；宣称 FR155–165 已交付；改写 Phase 12–18 关闭面。

</frozen-after-approval>

## Story

As a 文档维护者,
I want 公开状态页区分 Phase 18 完成面与 Phase 19 加深/lsp 上架面,
So that 对外不混淆「CLI 已上架」与「lsp/NFR59 加深」。

## Tasks / Subtasks

- [x] T1: README Phase 19 段 + FR 表
- [x] T2: deferred Phase 19 pointer 同源
- [x] T3: doc-19 Phase 19 交叉指针
- [x] T4: ATDD `fr154_readme_deferred_honesty.rs`
- [x] T5: sprint 87-3 done；87-4 ready-for-dev
- [x] T6: code-review + automation-summary

## Testing

- `cargo test -p bitloom --test fr154_readme_deferred_honesty`
- `cargo fmt --all && just test`

## Dev Agent Record

### Completion Notes List

- README / deferred / doc-19 Phase 19 诚实面；ATDD 绿；87-3 done

### File List

- `README.md`
- `_agile-output/implementation-artifacts/deferred-work.md`
- `docs/requirements/19. 实施路线图.md`
- `crates/bitloom/tests/fr154_readme_deferred_honesty.rs`
- `_agile-output/implementation-artifacts/87-3-*.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
