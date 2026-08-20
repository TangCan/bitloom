# Story 13.2: 发布 MVP bitloom-* 库族

Status: in-progress

## Story

As a 终端设计者,
I want 在 crates.io 上 `cargo add bitloom-prelude`,
So that 无需 path/git 指向 Bitloom monorepo 即可编写设计。

## Acceptance Criteria

见 `epics.md` Story 13.2（FR48, NFR18–19, NFR21–22）。

## Tasks / Subtasks

- [ ] 探测 crates.io 名仍空闲
- [ ] 将 MVP 五包 `[package].name` → `bitloom-*`，`publish = true`，补齐 metadata
- [ ] 全工作区依赖与 `use` 路径对齐
- [ ] `cargo test --workspace` 绿
- [ ] 按序 dry-run / publish：hir → builder → macro → prelude → vlog
- [ ] 文档/描述声明与 samitbasu/rhdl 无关

## Dev Notes

- 目录可暂留 `crates/rhdl-*`
- 锁步版本（建议 bump 0.1.2）
- CLI `bitloom` 可同版或随后 bump

## Dev Agent Record

### File List

- (filled on completion)
