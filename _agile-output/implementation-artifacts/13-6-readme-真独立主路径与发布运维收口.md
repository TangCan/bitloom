# Story 13.6: README 真独立主路径与发布运维收口

Status: done

## Story

As a 仓库/crates.io 访客,
I want 快速开始以真独立路径为主,
So that 不会误以为必须 clone 才能出 Verilog。

## Acceptance Criteria

见 `epics.md` Story 13.6（FR54, NFR21, NFR22）。

## Tasks / Subtasks

- [x] README 主路径：install → new → build；clone 降为贡献者
- [x] 消歧 bitbloom / samitbasu/rhdl；设计依赖 `bitloom-prelude`
- [x] 文档化多包锁步发布与 Trusted Publishing 覆盖 `bitloom-*`
- [x] ATDD `readme_standalone_path.rs` + `just test`
