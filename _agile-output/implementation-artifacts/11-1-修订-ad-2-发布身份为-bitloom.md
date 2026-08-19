# Story 11.1: 修订 AD-2 发布身份为 bitloom

Status: done

<!-- Ultimate context engine analysis completed - comprehensive developer guide created -->

## Story

As a 架构与实现者,
I want AD-2 明确 crates.io 身份为 `bitloom`,
so that 后续改名与发布不会再写回 `rhdl-rs`。

## Acceptance Criteria

1. **Given** ARCHITECTURE-SPINE 中现有 AD-2（发布名 `rhdl-rs`）  
   **When** 修订 AD-2 并标记决议日期  
   **Then** 规则写明：crates.io 发布名为 **`bitloom`**；禁止 `rhdl` / `rhdl-bits`；Git 路径可仍为 `rhdl`；内部 `rhdl-*` 可暂留但新对外名须先查 crates.io；设计 crate 仍只依赖 prelude  
   **And** `AGENTS.md` / 策略块与 AD-2 一致（FR43, NFR14）

## Tasks / Subtasks

- [x] 修订 `ARCHITECTURE-SPINE.md` 的 AD-2 Rule（AC: 1）
  - [x] 将发布名从 `rhdl-rs` 改为 `bitloom`
  - [x] 保留禁止 `rhdl` / `rhdl-bits`、Git 路径可叫 `rhdl`、内部 `rhdl-*` 可暂留、设计 crate 只依赖 `rhdl-prelude`
  - [x] 标注决议日期 2026-08-19（Bitloom 锁定）
- [x] 同步脊柱 Consistency Conventions 中「发布 CLI」一行到 `bitloom`（AC: 1）
- [x] 核对 `AGENTS.md` Brand lock / Policy 与 AD-2 一致（AC: 1）
- [x] 增加可重复的身份断言（ATDD）：脊柱 AD-2 含 `bitloom` 且不以 `rhdl-rs` 为发布身份（AC: 1）

## Dev Notes

### Guardrails

- **本故事只改身份决议文档与对齐策略**，不要在本故事内重命名 Cargo 包或 CLI 二进制（那是 Story 11.2）。
- 架构图顶层仍可能写 `rhdl-rs (CLI)`：可改为 `bitloom (CLI)` 以与 AD-2 一致；**不要**批量改内部 crate 名 `rhdl-hir` 等。
- 禁止向 crates.io 暗示可占用 `rhdl` / `rhdl-bits`。
- 不另立 HIR；不碰生成器/后端代码路径。

### References

- [Source: `_agile-output/planning-artifacts/epics.md` — Story 11.1 / FR43]
- [Source: naming research — Decision locked Bitloom]
- [Source: `AGENTS.md` — Brand lock]

## Dev Agent Record

### Agent Model Used

Composer (Cursor agent)

### Debug Log References

- ATDD red: `ad2_publish_identity` failed until AD-2 revised
- Green after spine + AGENTS assert

### Completion Notes List

- AD-2 Rule → publish `bitloom`; Revised 2026-08-19; consistency table + diagram CLI label updated
- Structural seed still lists `rhdl-rs/` path until Story 11.2 renames crate
- Guardrail test: `crates/rhdl-rs/tests/ad2_publish_identity.rs`

### File List

- `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md`
- `crates/rhdl-rs/tests/ad2_publish_identity.rs`
- `_agile-output/implementation-artifacts/11-1-修订-ad-2-发布身份为-bitloom.md`
- `_agile-output/implementation-artifacts/sprint-status.yaml`

## Change Log

- 2026-08-19: Implement AD-2 Bitloom identity + ATDD guardrail
