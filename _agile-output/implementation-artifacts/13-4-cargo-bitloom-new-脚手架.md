# Story 13.4: cargo bitloom new 脚手架

Status: done

## Story

As a 新用户,
I want `cargo bitloom new <name>` 生成最小设计 crate,
So that 不必手写 Cargo.toml 与 elaborate 入口。

## Acceptance Criteria

见 `epics.md` Story 13.4（FR52）。

## Tasks / Subtasks

- [x] `cargo bitloom new <name>` 生成仅依赖 `bitloom-prelude` 的设计 crate
- [x] 含 `#[module]` 示例与 `rhdl_elaborate` 入口
- [x] `--help` 描述子命令
- [x] ATDD `new_scaffold.rs` + `just test`
