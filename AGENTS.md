<!-- Preserved outside bmad:context (not replaced on refresh) -->
## Brand lock (2026-08-19)

- **Public product name: Bitloom**
- crates.io package + CLI binary: **`bitloom`**
- Design crates depend only on **`bitloom-prelude`** (never on the CLI package)
- Never publish `rhdl` / `rhdl-bits`; do not use `rhdl-rs` as the publish name
- Naming research: `_agile-output/planning-artifacts/research/technical-rhdl-rename-alternatives-product-naming-2026-08-19/research.md`
- Standalone path: `_agile-output/planning-artifacts/research/technical-cargo-bitloom-standalone-usage-after-ins-2026-08-20/research.md`
- AD-5 / FR47: toolchain may generate Rust functional-sim crates; SystemC TLM-2.0 is not contracted (see ARCHITECTURE-SPINE AD-5)
- AD-20 / FR51: documented `Bundle` / `Vec<T,N>` (or equiv.) allowed on synthesizable path; width/dir fail before emit; FR22 surface thicken does not deliver composites (see ARCHITECTURE-SPINE AD-20)

## Process

- Future epics: **one story → one commit** (see `_agile-output/implementation-artifacts/process-one-story-one-commit.md`).

<!-- bmad:context -->
<!-- Verified 2026-08-18 against 7fe8d78. Managed by bmad-project-context; edits inside this block are replaced on refresh. Keep anything you want preserved outside the markers. -->

## rhdl (workspace; public brand Bitloom)

Rust 嵌入式 RTL HDL：设计是生成器，冻结 HIR 后再降后端。需求在 `docs/requirements/`，规划在 `_agile-output/planning-artifacts/`。公开产品名 **Bitloom**（crates.io / CLI：`bitloom`）。

## Policy

- 禁止向 crates.io 发布名 `rhdl` 或 `rhdl-bits`；发布名与 CLI 用 **`bitloom`**（不用 `rhdl-rs`）。设计 crate 只依赖 **`bitloom-prelude`**。文档须声明与 `samitbasu/rhdl` 无关。

## Where things are

- 改语言或工具链（将来的 `crates/`）：先读 `_agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md`，按 AD 做；不要另立 HIR，不要在 rustc 编译期抽网表。
- 给人讲架构：同目录 `team-walkthrough.html`。
- 技术依据：`_agile-output/planning-artifacts/research/technical-rhdl-rust-edsl-hdl-implementation-archit-2026-08-18/research.md`。
- 命名依据：`_agile-output/planning-artifacts/research/technical-rhdl-rename-alternatives-product-naming-2026-08-19/research.md`。

## Running and verifying

- TODO：按脊柱钉死的 rustc 1.97.1 / edition 2024 跑 `just test`，并 refresh 本块。公开 CLI：`cargo bitloom`（包 `bitloom`，二进制 `cargo-bitloom`）。

<!-- /bmad:context -->
