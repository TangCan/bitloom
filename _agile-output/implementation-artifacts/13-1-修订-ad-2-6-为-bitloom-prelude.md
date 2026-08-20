# Story 13.1: 修订 AD-2/6 为 bitloom-prelude

Status: done

## Story

As a 架构与实现者,
I want AD-2/AD-6 明确设计 crate 只依赖 `bitloom-prelude`,
So that 真独立发布不会再把用户面钉在 `rhdl-prelude` 或 CLI 包上。

## Acceptance Criteria

1. **Given** ARCHITECTURE-SPINE 中 AD-2 仍写设计依赖 `rhdl-prelude`，且 AD-6 图/规则使用 `rhdl-*` 名  
   **When** 修订 AD-2 与 AD-6（含依赖图标签）并标注决议日期  
   **Then** 规则写明：设计 crate `[dependencies]` 唯一允许 **`bitloom-prelude`**；设计不得依赖 CLI 包 `bitloom`；继续禁止发布/暗示 `rhdl` / `rhdl-bits` / `rhdl-rs`；内部目录可暂留 `rhdl-*` 但对外 `[package].name` 走 `bitloom-*`（FR49）  
   **And** `AGENTS.md` / 策略块与修订后的 AD 一致（NFR21）

## Tasks / Subtasks

- [x] 修订 AD-2 Rule：设计依赖 → `bitloom-prelude`；标注 2026-08-20（FR49）
- [x] 修订 AD-6 规则与 mermaid：PRE/`[dependencies]` → `bitloom-prelude`；dev-dep 后续 `bitloom-sim`；设计不得依赖 CLI
- [x] 同步脊柱摘要/Consistency 中「设计只依赖 prelude」表述
- [x] 更新 `AGENTS.md` Brand lock：用户面依赖 `bitloom-prelude`
- [x] ATDD：`crates/bitloom/tests/ad2_design_dep_bitloom_prelude.rs`
- [ ] 本故事**不**重命名 Cargo 包（留给 13.2）

## Dev Notes

### Guardrails

- 只改架构决议与策略文档 + 身份断言测试；**不要**改 `Cargo.toml` 的 `name = "rhdl-prelude"`（Story 13.2）。
- 保留 AD-14 host shim；禁止设计依赖 CLI。
- 继续禁止 crates.io `rhdl` / `rhdl-bits` / `rhdl-rs`。
- AD-19 中的 `rhdl_prelude::` Rust 路径可暂留至包改名故事。

### References

- [Source: `epics.md` Story 13.1 / FR49]
- [Source: standalone research §4.1 naming]
- [Source: Story 11.1 pattern — `ad2_publish_identity.rs`]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Completion Notes List

- Pipeline iteration 1: create-story → atdd → implement AD-2/6 → guardrail test → commit

### File List

- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md`
- `AGENTS.md`
- `crates/bitloom/tests/ad2_design_dep_bitloom_prelude.rs`
- `_agile-output/implementation-artifacts/13-1-修订-ad-2-6-为-bitloom-prelude.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-08-20: Story created; AD-2/6 → bitloom-prelude; ATDD green
