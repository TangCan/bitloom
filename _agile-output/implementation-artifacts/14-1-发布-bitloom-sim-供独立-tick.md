# Story 14.1: 发布 bitloom-sim 供独立 tick

Status: done

## Story

As a 设计者,
I want `cargo add bitloom-sim --dev` 后在 `cargo test` 里 `tick`,
So that 仿真也不必 clone 工具链仓库。

## Acceptance Criteria

见 `epics.md` Story 14.1（FR55）。

## Tasks / Subtasks

- [x] Package rename `bitloom-sim`, `publish = true`, registry `bitloom-hir`
- [x] Update Rust paths / workspace deps; design deps stay prelude-only
- [x] README optional独立仿真小节
- [x] `cargo publish -p bitloom-sim` → **0.1.2** on crates.io (2026-08-20)
- [x] ATDD + `just test`
