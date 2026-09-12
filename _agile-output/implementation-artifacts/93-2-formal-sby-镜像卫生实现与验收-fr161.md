---
title: '93.2 formal-sby 镜像卫生实现与验收（FR161）'
type: 'feat'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: 'bf2e5d5 Story 93.1: Epic 93 NFR14 risk record for FR161 sby hygiene.'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic93-formal-sby-image-hygiene-fr161.md'
  - '{project-root}/docs/fr127-forced-sby-ci.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR127 requires CI sby but install floated to git HEAD without hygiene pins.

**Approach:** `ci-sby-pins.env` + install obeys ref/SHA; `ci-sby-hygiene-check.sh` in CI; docs + ATDD. FR127 alone ≠ FR161.

## Boundaries & Constraints

**Always:** pin file; readable fail; FR127 job remains; Bitloom.
**Never:** claim FR127 substitutes FR161; silent-Ok missing pins; float HEAD clone.

</frozen-after-approval>

## Story

As a 维护者,
I want CI/`formal-sby` 镜像版本与卫生可跟踪、可验证,
So that 形式验证门禁不依赖无记录的临时镜像。

## Tasks / Subtasks

- [x] T1: pins + install + hygiene script + CI step
- [x] T2: `docs/fr161-*` + FR127 cross-link
- [x] T3: ATDD
- [x] T4: sprint 93-2 done / 93-3 ready
- [x] T5: code-review + automation-summary
