# Story 11.2: 对外包名与 CLI 品牌改为 bitloom

Status: done

## Story

As a 本机用户,
I want 安装与调用的 CLI 品牌是 Bitloom,
so that 帮助文本与包名不再像另一个 RHDL 项目。

## Acceptance Criteria

1. 对外 `[package].name` 与 cargo 子命令二进制为 Bitloom（`bitloom` / `cargo-bitloom`）
2. `--help` / 用户可见文案不以 RHDL/`rhdl-rs` 为产品/发布名
3. `just`/脊柱用户路径中的 `cargo rhdl` 改为 `cargo bitloom` 或标明遗留
4. `just test` 通过

## Tasks / Subtasks

- [x] 将 CLI crate 发布名改为 `bitloom`，二进制 `cargo-bitloom`（目录 `crates/bitloom`）
- [x] 更新 clap / about / 测试 `CARGO_BIN_EXE_*`
- [x] 更新 AGENTS、ARCHITECTURE-SPINE 用户可见 CLI 调用约定
- [x] ATDD：`cli_brand_bitloom.rs`
- [x] `cargo fmt --all && just test`

## Dev Agent Record

### Completion Notes List

- `git mv crates/rhdl-rs → crates/bitloom`; package `bitloom`; bin `cargo-bitloom`
- README/docs CLI invocations updated

### File List

- `crates/bitloom/**`, `Cargo.toml`, `README.md`, `AGENTS.md`, spine, docs/fr32|35|40
