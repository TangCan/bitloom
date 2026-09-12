---
title: '97.2 更深 Chisel/Parser 生态实现与验收（FR165）'
type: 'feature'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: 'f1942a8 Story 97.1: Epic 97 NFR14 risk record for FR165 Style Guide/linter deepen.'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic97-deeper-chisel-parser-ecosystem-fr165.md'
  - '{project-root}/docs/fr165-deeper-chisel-parser-ecosystem.md'
warnings: []
deferred:
  - 'Arbitrary Chisel HEAD Parser migration (NFR71)'
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR130 markers and FR138 Parser alone do not cover community Style Guide/linter deepen.

**Approach:** Emit/check FR165 linter pack (superset of FR130) + `just chisel-style-lint-check`; defer HEAD Parser; no AD-27 revise.

## Boundaries & Constraints

**Always:** NFR14 MVP; FR138/FR130 isolation; force-missing non-zero; Bitloom.
**Never:** claim HEAD Parser delivered; rewrite FR138/FR130 closes; revise AD-27 silently.

</frozen-after-approval>

## Story

As a 维护者/Chisel 消费者,
I want 超出 FR138 的更深生态路径,
So that Style Guide/Parser 加深可勾选。

## Tasks / Subtasks

- [x] T1: `emit_chisel_style_guide_fr165` / `check_chisel_style_guide_fr165`
- [x] T2: `scripts/chisel-style-lint-check.sh` + Just + example
- [x] T3: `docs/fr165-*` + ATDD
- [x] T4: code-review + automation-summary
