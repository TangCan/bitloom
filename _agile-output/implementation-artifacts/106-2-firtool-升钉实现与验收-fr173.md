---
title: '106.2 firtool 升钉实现与验收（FR173）'
type: 'feature'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: 'da5298e Story 106.1: Epic 106 NFR14 risk record for firtool bump (FR173).'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr173-firtool-bump-ad9.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic106-firtool-bump-ad9-fr173.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** Product still pinned to AD-9 firtool-1.155.0 / Chisel 7.14.0; FR173 requires official paired bump.

**Approach:** Bump CLI/scripts/Stack/AD-9 to Chisel 7.15.0 ↔ firtool-1.158.0; keep FR169/FR170 historical close pins documented (NFR78); ATDD + live ensure.

## Boundaries & Constraints

**Always:** Official pairing table; AD-9 revise before ready; ≠ FR169 alone; Bitloom; non-zero on mismatch.

**Never:** PATH-random; unpaired HEAD as this FR; claim FR174–177 closed.

</frozen-after-approval>

## Story

As a 工具链维护者,
I want 产品钉死 firtool 升至经 Chisel 正式配对的新版本,
So that 不再停留在 AD-9 `firtool-1.155.0` 钉死面。

## Acceptance Criteria

1. Product pin Chisel 7.15.0 ↔ firtool-1.158.0; AD-9 / Stack revised
2. FR169 / FR164 / FR137 close evidence still valid (NFR78)
3. Missing tool / version mismatch → non-zero readable failure

## Tasks / Subtasks

- [x] T1: firtool.rs + CHISEL_TARGET + scripts + CI comments
- [x] T2: AD-9 / Stack / README / addendum NFR12 + docs/fr173-*
- [x] T3: ATDD `fr173_firtool_bump_ad9` + fix pin-era tests
- [x] T4: sprint 106.2 done；106.3 ready-for-dev
- [x] T5: code-review + automation-summary

## Testing

- `cargo test -p bitloom --test fr173_firtool_bump_ad9`
- 回归：`cargo fmt --all && just test`
