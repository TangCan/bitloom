# FR99 — Bitloom language-server（Stories 44.2 + 44.3）

**Product:** Bitloom（`bitloom-lsp`）。与 [samitbasu/rhdl](https://github.com/samitbasu/rhdl) 无关。

**Scope:**

| Story | Delivered |
| --- | --- |
| **44.2** | Installable/startable `bitloom-lsp` + minimal `initialize` / capabilities + editor wiring |
| **44.3** | Documented edit-trigger → **full-design elaborate**; `publishDiagnostics` + document symbols / goto (hardware-semantic) |
| **44.4** | FR99 epic closeout / Path B completion-narrative revocation — **not yet** |

**MUST NOT** claim that **rust-analyzer alone** (FR90) completes FR99. Host rust-analyzer remains the Rust IDE path; Bitloom LSP is the hardware-semantics product path under Epic 44. HTML visualization (FR38/FR49) ≠ LSP.

## Install / start

From the workspace root (MSRV **1.97.1** / edition 2024):

```bash
cargo run -p bitloom-lsp
# or:
cargo install --path crates/bitloom-lsp --locked
bitloom-lsp
```

The server speaks **stdio** Language Server Protocol (LSP 3.x framing: `Content-Length` headers).

## Capabilities

On `initialize`, `bitloom-lsp` returns:

- `serverInfo.name` = `bitloom-lsp`
- `capabilities.textDocumentSync` = Full + **save** notifications
- `documentSymbolProvider` = true
- `definitionProvider` = true

## P1 — Edit trigger (Story 44.3)

**Chosen trigger:** `textDocument/didSave`.

On save, the server runs the **full-design elaborate** path for the documented MVP design root (`DesignFixture::OkCounter` / `bitloom_lsp::analyze_on_did_save`), then `publishDiagnostics`. Document symbols / goto use the last successful analyze cache (or re-run the MVP root).

Debounced `didChange` is **not** required for MVP (NFR14 P5 allows full recompute).

## P2 — Full-design elaborate contract

The full path calls `ElaborateSession::finish()` (or equivalent) on the documented fixture module set — **not** a lexical-only scan.

**Shallow / non-elaborate contrast:** `AnalysisMode::Shallow` returns without calling `finish()`. ATDD (`fr99_bitloom_lsp_full_elaborate`) proves the same failing fixture yields `rhdl::E0142` only on the full path.

Library API (for tests and tooling):

```rust
bitloom_lsp::analyze(mode, fixture, timeout)
```

## P3 — Interactive budget / timeout

- Fixture-scale target: first diagnostics within **≤ 2s** (`MVP_INTERACTIVE_BUDGET`). Cold start may be excluded and is documented here.
- On timeout: diagnostic code **`bitloom-lsp.timeout`** with a readable message — do not hang the editor indefinitely.

## P4 — Scale ceiling / oversized

- MVP ceiling: **`MVP_MAX_MODULES = 8`** modules in the documented acceptance fixture set.
- When exceeded: diagnostic code **`bitloom-lsp.oversized`**; **`called_finish` stays false** — never pretend full elaborate succeeded.

## P5 — Incrementality

MVP does **not** require fine-grained incremental elaborate; full recompute on each didSave is allowed within P3/P4.

## P6 — Capability minimum set

- **Diagnostics:** hardware-semantic / elaborate errors (e.g. `rhdl::E0142`) via `publishDiagnostics`.
- **Symbols / goto:** `textDocument/documentSymbol` and `textDocument/definition` over FrozenHir modules/ports (or last analyze cache).

## VS Code (or Cursor) wiring

1. Build or install `bitloom-lsp` so it is on `PATH`, **or** point at `target/debug/bitloom-lsp` / `target/release/bitloom-lsp`.
2. Register a language client that launches Bitloom LSP over stdio. Example settings fragment:

```json
{
  "bitloom.lsp.path": "${workspaceFolder}/target/debug/bitloom-lsp",
  "bitloom.lsp.trace": "off"
}
```

3. Keep **rust-analyzer** enabled for Rust IDE features (FR90). Use **bitloom-lsp** as a second server for hardware-semantic elaborate diagnostics/symbols. Save a document to trigger the P1 didSave path.

## Fixture: reproduce sessions

```bash
# 44.2 initialize session
cargo test -p bitloom --test fr99_bitloom_lsp_server_mvp fr99_bitloom_lsp_initialize_session -- --nocapture

# 44.3 full vs shallow elaborate + diagnostics/symbols
cargo test -p bitloom --test fr99_bitloom_lsp_full_elaborate -- --nocapture
```

## Design crate dependency boundary

Design crates still depend **only** on [`bitloom-prelude`](../crates/bitloom-prelude). Do **not** add `bitloom-lsp` or the CLI package `bitloom` to design `[dependencies]`. The LSP crate may depend on `bitloom-builder` / `bitloom-hir` (toolchain face).

## Cross-links

- Host IDE (FR90): [`fr90-host-ide-rust-analyzer.md`](fr90-host-ide-rust-analyzer.md)
- Hierarchy HTML ≠ LSP: [`fr38-viz-lsp.md`](fr38-viz-lsp.md)
- NFR14 Epic 44 gate (P1–P6): `_agile-output/implementation-artifacts/nfr14-risk-epic44-full-elaborate-lsp.md`
