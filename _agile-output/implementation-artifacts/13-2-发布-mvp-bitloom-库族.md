# Story 13.2: 发布 MVP bitloom-* 库族

Status: done

## Story

As a 终端设计者,
I want 在 crates.io 上 `cargo add bitloom-prelude`,
So that 无需 path/git 指向 Bitloom monorepo 即可编写设计。

## Acceptance Criteria

见 epics.md Story 13.2（FR48, NFR18–19）。

## Completion Notes

- Renamed package names to bitloom-{hir,builder,macro,prelude,vlog}; dirs remain crates/rhdl-*
- Workspace version 0.1.2; workspace deps include version for publish
- Published to crates.io 2026-08-20: all five crates at 0.1.2
- just test green; trybuild stderr blessed

## File List

- Cargo.toml, crates/rhdl-*/Cargo.toml & sources, examples/*, crates/bitloom/src/main.rs
