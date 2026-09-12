---
title: '92.2 非 Cargo 路径扫描实现与验收（FR160）'
type: 'feat'
created: '2026-09-12'
status: 'done'
route: 'oneshot'
baseline_commit: '80a9f80 Story 92.1: Epic 92 NFR14 risk record for FR160 path scan.'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/implementation-artifacts/nfr14-risk-epic92-non-cargo-path-scan-fr160.md'
  - '{project-root}/docs/fr118-syn-scan-design-root-discovery.md'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** FR118 only scans Cargo workspace members; FR160 needs non-Cargo / non-members path scan.

**Approach:** `discover_design_roots_under`; bare `.rs` fixture without Cargo.toml; readable NotFound/PermissionDenied; docs + ATDD. FR118 alone ≠ FR160.

## Boundaries & Constraints

**Always:** explicit paths; FR118 isolation; readable failure; Bitloom.
**Never:** claim FR118 substitutes FR160; unbounded disk crawl; add bitloom→bitloom-lsp dep.

</frozen-after-approval>

## Story

As a 用户/工具集成者,
I want 在非 Cargo monorepo 下扫描约定路径,
So that 全仓设计发现不依赖 Cargo metadata alone。

## Tasks / Subtasks

- [x] T1: `discover_design_roots_under` + fixture
- [x] T2: `docs/fr160-*` + FR118 cross-link
- [x] T3: ATDD (bitloom-lsp + bitloom docs guard)
- [x] T4: sprint 92-2 done / 92-3 ready
- [x] T5: code-review + automation-summary
