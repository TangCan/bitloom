---
title: '81.3 表面 crate semver CI 门禁（FR144）'
type: 'chore'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: '68234b6 Story 81.2: Land SemVer 1.0 policy (FR143) and cross-link 0.x / surface docs.'
review_loop_iteration: 0
context:
  - '{project-root}/scripts/semver-check.sh'
  - '{project-root}/docs/semver-1-0-policy.md'
  - '{project-root}/.github/workflows/ci.yml'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR143 政策已成文但缺可复现的破坏性变更门禁，易静默缺工具或 continue-on-error。

**Approach:** `scripts/semver-check.sh` + `just semver-check` + CI required job `semver-check`（安装 `cargo-semver-checks`；无 continue-on-error）；ATDD 锁脚本/Just/CI/缺工具路径。**不**收口 Epic 81（→ 81.4）。

## Boundaries & Constraints

**Always:** 缺工具非零；禁止 continue-on-error；库表面 prelude+sim；pre-1.0 默认 `--release-type major`；Bitloom。

**Never:** publish 1.0.0；吞并 NFR59；把 macro/CLI 冒充已全量 rustdoc-semver 覆盖。

</frozen-after-approval>

## Story

As a 维护者,
I want CI / `just semver-check` 对表面 crate 跑破坏性变更检查,
So that 违规变更非零失败。

## Acceptance Criteria

1. `cargo-semver-checks`（或等价）接入 just + CI；失败非零；禁止 continue-on-error
2. 缺工具有可读诊断；文档指出本地复现

## Tasks / Subtasks

- [x] T1: scripts/semver-check.sh + Justfile recipe
- [x] T2: CI job semver-check
- [x] T3: ATDD
- [x] T4: code-review + automation-summary；sprint 81-3 done

## Testing

- `cargo test -p bitloom --test fr144_semver_ci_gate`
- `BITLOOM_SEMVER_FORCE_MISSING=1 bash scripts/semver-check.sh` → non-zero
- `just semver-check`

## Dev Agent Record

### Completion Notes List

- FR144 gate: just + CI + missing-tool ATDD；pre-1.0 major release-type

### File List

- `scripts/semver-check.sh`
- `Justfile`
- `.github/workflows/ci.yml`
- `crates/bitloom/tests/fr144_semver_ci_gate.rs`
- `_agile-output/implementation-artifacts/81-3-*.md`
- `sprint-status.yaml`
