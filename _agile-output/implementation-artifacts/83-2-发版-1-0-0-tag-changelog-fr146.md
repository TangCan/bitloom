---
title: '83.2 发版 1.0.0 + tag + CHANGELOG（FR146）'
type: 'chore'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: '04731f9 Story 83.1: Epic 83 NFR14 risk record for Bitloom 1.0.0 release (FR146).'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr146-bitloom-1-0-0-release.md'
  - '{project-root}/CHANGELOG.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** 需将表面承诺 crate 升至 1.0.0 并具备 tag/CHANGELOG/发版证据。

**Approach:** workspace **1.0.0**；CHANGELOG；`docs/fr146-bitloom-1-0-0-release.md` dry-run+清单；annotated tag `v1.0.0`；`cargo doc` 绿。真实 crates.io 全序 publish 记为清单 follow-up（依赖未上架 1.0.0 / firrtl publish=false）。

## Boundaries & Constraints

**Always:** 1.0.0；CHANGELOG；tag；dry-run 证据；NFR59 诚实；Bitloom。

**Never:** 声称 NFR59 已关；用 Phase 16 alone 冒充 1.0。

</frozen-after-approval>

## Story

As a 维护者,
I want 表面承诺 crate 达 1.0.0 并完成 tag/CHANGELOG,
So that Bitloom 合法进入 1.0。

## Acceptance Criteria

1. 表面承诺 crate 版本 1.0.0；tag `v1.0.0`；CHANGELOG；doc 绿
2. publish 或 dry-run+手动清单完成态
3. README 声明 1.0 表面并保留 NFR59

## Tasks / Subtasks

- [x] T1: version bump + CHANGELOG + checklist + dry-run evidence
- [x] T2: tag v1.0.0
- [x] T3: ATDD
- [x] T4: code-review + automation-summary；sprint 83-2 done

## Testing

- `cargo test -p bitloom --test fr146_bitloom_1_0_0_release`
- `just semver-check`
- `cargo clean && cargo fmt --all && just test`

## Dev Agent Record

### Completion Notes List

- 1.0.0 tree + tag + dry-run leaves OK；dependent publishes blocked until crates.io has 1.0.0 parents

### File List

- Cargo.toml / CHANGELOG.md / docs/fr146-* / scripts/semver-check.sh / 83-2-*
