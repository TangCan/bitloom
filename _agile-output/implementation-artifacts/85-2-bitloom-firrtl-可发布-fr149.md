---
title: '85.2 bitloom-firrtl 可发布（FR149）'
type: 'feature'
created: '2026-09-11'
status: 'done'
route: 'oneshot'
baseline_commit: '44f0bb2 Story 85.1: Epic 85 NFR14 risk record for CLI install FR149–152.'
review_loop_iteration: 0
context:
  - '{project-root}/_agile-output/planning-artifacts/epics.md'
  - '{project-root}/crates/rhdl-firrtl/Cargo.toml'
  - '{project-root}/Cargo.toml'
  - '{project-root}/crates/bitloom-hir/Cargo.toml'
warnings: []
deferred: []
---

<frozen-after-approval reason="human-owned intent — do not modify unless human renegotiates">

## Intent

**Problem:** `rhdl-firrtl` is `publish=false` without a registry version, blocking `cargo publish -p bitloom` and violating AD-2 `bitloom-*` packaging.

**Approach:** Rename package to `bitloom-firrtl` (dir may stay `rhdl-firrtl`), set `publish=true` + workspace version `1.0.0`, update workspace/code/docs package references (`rhdl_firrtl`→`bitloom_firrtl`), keep filesystem paths under `crates/rhdl-firrtl`. Dry-run + live publish 1.0.0 when credentials work. Document in `docs/fr149-bitloom-firrtl-publish.md`. Never publish as `rhdl-firrtl`.

## Boundaries & Constraints

**Always:** package name `bitloom-firrtl`; AD-2; version align; dry-run green; Bitloom brand.

**Never:** publish as `rhdl-firrtl` / `rhdl` / `rhdl-bits`; rename viz/CLI publish (→ 85.3/85.5); change lsp policy (→ 85.4); expand FR142.

## I/O & Edge-Case Matrix

| Scenario | Input / State | Expected Output / Behavior | Error Handling |
|----------|--------------|---------------------------|----------------|
| Package identity | Cargo.toml name | `bitloom-firrtl`, publish=true | ATDD 红 |
| Dry-run | `cargo publish -p bitloom-firrtl --dry-run` | success | fix deps |
| Live publish | credentials OK | 1.0.0 on crates.io | document auth failure |
| Old name | rhdl-firrtl as publish name | forbidden | ATDD 红 |

</frozen-after-approval>

## Story

As a 维护者,
I want 将现 `rhdl-firrtl` 变为可 crates.io 发布的 `bitloom-firrtl`,
So that CLI 与工具链可对 FIRRTL 腿使用 version 依赖。

## Acceptance Criteria

1. Given Story 85.1；`bitloom-hir` 1.0.0 已在 crates.io, when 完成包名/workspace/依赖迁移，`publish = true`，version 对齐, then `cargo publish -p bitloom-firrtl --dry-run` 成功；实发 1.0.0 可勾选
2. And 仓库内引用更新为 `bitloom-firrtl`（目录可暂留 `rhdl-firrtl`）
3. And 禁止以 `rhdl-firrtl` 为 crates.io 发布名
4. And ATDD/文档指针可验证（`docs/fr149-bitloom-firrtl-publish.md`）

## Tasks / Subtasks

- [x] T1: Cargo.toml + workspace rename + publish metadata
- [x] T2: Update Rust/docs package refs; keep dir paths
- [x] T3: docs/fr149 + ATDD
- [x] T4: dry-run + live publish (1.0.0 on crates.io)
- [x] T5: sprint / review / just test / commit

## Testing

- `cargo test -p bitloom --test fr149_bitloom_firrtl_publish`
- `cargo publish -p bitloom-firrtl --dry-run`
- `cargo clean && cargo fmt --all && just test`

## Dev Agent Record

### Completion Notes List

- Renamed package to `bitloom-firrtl`; dir `crates/rhdl-firrtl` retained
- Live published **bitloom-firrtl 1.0.0** to crates.io
- ATDD + docs/fr149; sprint 85-2 done; 85-3 ready-for-dev

### File List

- `crates/rhdl-firrtl/Cargo.toml`
- `Cargo.toml`
- `crates/bitloom/Cargo.toml` / `src/main.rs` / tests (crate path rename)
- `docs/fr149-bitloom-firrtl-publish.md`
- `docs/public-api-1-0-surface.md`
- `README.md`
- `crates/bitloom/tests/fr149_bitloom_firrtl_publish.rs`
- `_agile-output/implementation-artifacts/85-2-*`
- `_agile-output/implementation-artifacts/sprint-status.yaml`
