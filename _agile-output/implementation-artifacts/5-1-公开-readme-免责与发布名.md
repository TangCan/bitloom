# Story 5.1: 公开 README 免责与发布名

Status: done

## Story

As a 硬件设计者 / 潜在贡献者,
I want 在仓库首页看到与 `samitbasu/rhdl` 无关且发布名为 `rhdl-rs` 的声明,
so that 不会把本项目当成 crates.io 上的 `rhdl`。

## Acceptance Criteria

1. **Given** 空克隆的仓库根目录 **When** 打开 `README.md` **Then** 可见与 `samitbasu/rhdl` 无关的免责声明
2. **And** 写明 crates.io 发布名 `rhdl-rs`，并禁止暗示 `rhdl` / `rhdl-bits`（FR21）

## Tasks

- [ ] 根目录 `README.md`：项目简介、免责、发布名、指向 docs/SPEC/架构
- [ ] 简短如何开始（rust-toolchain、just test）

## Dev Notes

- AD-2 / AD-26 / FR21；勿暗示 crates.io 名 `rhdl`
- 与 `samitbasu/rhdl` 划清界限

## Dev Agent Record

### Completion Notes

### File List
