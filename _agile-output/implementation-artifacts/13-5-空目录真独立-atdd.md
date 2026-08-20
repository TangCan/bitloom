# Story 13.5: 空目录真独立 ATDD

Status: done

## Story

As a 维护者,
I want 自动化验收「install → new → build → .v」无需 clone,
So that 回归不会再把 monorepo 绑回主路径。

## Acceptance Criteria

见 `epics.md` Story 13.5（FR47, FR53, NFR20）。

## Tasks / Subtasks

- [x] 空临时目录：`new` → `build` → 非空 `.v`
- [x] 不 `git clone`；host 用 registry 后端
- [x] `crates/bitloom/tests/standalone_empty_dir.rs` + `just test`
