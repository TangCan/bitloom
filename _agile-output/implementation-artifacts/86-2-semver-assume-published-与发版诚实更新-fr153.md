---
title: '86.2 SemVer assume-published 与发版诚实更新（FR153）'
type: 'chore'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: 'c5245e7 Story 86.1: Record Epic 86 NFR14 for FR153 SemVer honesty gate.'
review_loop_iteration: 0
context:
  - '{project-root}/scripts/semver-check.sh'
  - '{project-root}/docs/semver-1-0-policy.md'
  - '{project-root}/docs/fr146-bitloom-1-0-0-release.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic86-fr153-semver-honesty.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Workspace 已是 1.0.0 且库+CLI 均在 crates.io，但 `semver-check.sh` 仍对 1.0.0 默认 `--release-type major`；`docs/fr146-*` / Release / README 仍把 FR153 / assume-published 写成待办。

**Approach:** 移除 1.0.0 特例（≥1.0 默认 minor；0.x 仍 major）；更新 policy / fr146 / README / GitHub Release；ATDD 锁脚本与诚实指针。不勾选 Epic 86 关闭（→ 86.3）。

## Boundaries & Constraints

**Always:** `just semver-check` 在已发布前提下默认 minor；库+CLI 上架区分；NFR59 deferred；Bitloom。

**Never:** 扩大 FR142；清空 NFR59；勾选 epic-86 done（→ 86.3）。

</frozen-after-approval>

## Story

As a 维护者,
I want 更新 SemVer 检查默认与发版文档/Release,
So that CI/文档反映「1.0.0 已在 crates.io（含 CLI）」的真实状态。

## Acceptance Criteria

1. 移除 1.0.0 特例（或 ASSUME_PUBLISHED）；`just semver-check` 默认 minor
2. 更新 `docs/fr146-*` / README / GitHub Release；区分库+CLI；NFR59 deferred
3. ATDD 可验证诚实指针

## Tasks / Subtasks

- [x] T1: `scripts/semver-check.sh` + `docs/semver-1-0-policy.md`
- [x] T2: `docs/fr146-*` / README / Release（gh）
- [x] T3: ATDD + sprint 86-2 done；86-3 ready-for-dev
- [x] T4: code-review + automation-summary + just test + commit

## Dev Agent Record

### Completion Notes List

- Removed 1.0.0 major special-case; ≥1.0 defaults minor
- Docs/Release honesty: library + CLI published; FR153 landed in 86.2; closeout → 86.3

### File List

- scripts/semver-check.sh
- docs/semver-1-0-policy.md
- docs/fr146-bitloom-1-0-0-release.md
- docs/fr153-semver-honesty.md
- README.md
- crates/bitloom/tests/fr153_semver_honesty.rs
- sprint-status / 86-2-*
