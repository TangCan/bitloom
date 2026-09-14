---
title: '115.2 unpaired firtool 产品钉再升钉实现与验收（FR182）'
type: 'feature'
created: '2026-09-14'
status: 'done'
route: 'oneshot'
baseline_commit: '1216094 Story 115.1: Epic 115 NFR14 risk record for unpaired firtool product-pin (FR182).'
review_loop_iteration: 0
context:
  - '{project-root}/docs/fr182-unpaired-firtool-product-pin.md'
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic115-unpaired-firtool-product-pin-fr182.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** NFR81 leftover needs an unpaired firtool **product-pin** bump beyond FR173's paired 1.158.0.

**Approach:** Bump AD-9 default product pin to firtool-**1.159.0** with Chisel remaining **7.15.0**; revise AD-9 *unpaired product-pin* exception (NFR85). When FR179 floating-track coincides at 1.159.0, distinguish channels by cache/path.

## Boundaries & Constraints

**Always:** ≠ FR173/FR174/FR179 alone; AD-9 exception; Bitloom; non-zero on missing/mismatch.

**Never:** Claim FR182 closed (→ 115.3); bump Chisel; treat FR174/FR179 as the product pin; silent-Ok.

</frozen-after-approval>

## Story

As a 工具链维护者,
I want 在 AD-9 例外纪律下交付无上游配对的 firtool 产品钉再升钉,
So that NFR81 unpaired-bump leftover 可勾选关闭。

## Acceptance Criteria

1. Product pin 1.159.0 + ATDD/CI
2. AD-9 unpaired product-pin exception (NFR85)
3. FR173/174/179 close evidence still valid (NFR83)
4. Missing tool / version mismatch → non-zero readable

## Tasks

- [x] Bump CLI / FIRTOOL_TARGET / gate scripts to 1.159.0
- [x] Revise AD-9 + Stack; docs/fr182-*
- [x] Soften FR179 version-inequality reject; channel-by-path
- [x] ATDD fr182_* + update live-pin consumers
- [x] cargo clean && cargo fmt --all && just test
