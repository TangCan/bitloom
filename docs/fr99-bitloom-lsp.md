# FR99 — Bitloom language-server MVP（Story 44.2）

**Product:** Bitloom（`bitloom-lsp`）。与 [samitbasu/rhdl](https://github.com/samitbasu/rhdl) 无关。

**Scope (this story):** installable / startable Bitloom **language-server** binary with minimal LSP `initialize` / capabilities, plus editor wiring so a fixture can reproduce an LSP session.

**Not this story:** keystroke full-design elaborate diagnostics / symbols (**Story 44.3**); FR99 epic closeout and Path B completion-narrative revocation (**Story 44.4**).

**MUST NOT** claim that **rust-analyzer alone** (FR90) completes this story or FR99. Host rust-analyzer remains the Rust IDE path; Bitloom LSP is the hardware-semantics product path under Epic 44.

## Install / start

From the workspace root (MSRV **1.97.1** / edition 2024):

```bash
cargo run -p bitloom-lsp
# or:
cargo install --path crates/bitloom-lsp --locked
bitloom-lsp
```

The server speaks **stdio** Language Server Protocol (LSP 3.x framing: `Content-Length` headers).

## Minimal capabilities (MVP)

On `initialize`, `bitloom-lsp` returns:

- `serverInfo.name` = `bitloom-lsp`
- `capabilities.textDocumentSync` = Full

Full-elaborate `publishDiagnostics` / document symbols are **out of scope** until Story 44.3.

## VS Code (or Cursor) wiring

1. Build or install `bitloom-lsp` so it is on `PATH`, **or** point at the cargo-built binary under `target/debug/bitloom-lsp` / `target/release/bitloom-lsp`.
2. In VS Code / Cursor user or workspace settings, register a language client that launches Bitloom LSP over stdio. Example (`settings.json` fragment — adjust path):

```json
{
  "bitloom.lsp.path": "${workspaceFolder}/target/debug/bitloom-lsp",
  "bitloom.lsp.trace": "off"
}
```

Minimal `launch` / task alternative (Run Task → shell):

```bash
cargo run -q -p bitloom-lsp
```

3. Open this repository as the workspace root. Keep **rust-analyzer** enabled for Rust completion / goto / rustc diagnostics (FR90). Start **bitloom-lsp** as a **second** language server for the Bitloom LSP product surface — do **not** replace rust-analyzer with Bitloom LSP for Rust IDE features.

> Tip: Until a published extension exists, you can drive the same stdio session with any LSP client (VS Code `vscode-languageclient`, Neovim `vim.lsp`, or the ATDD fixture below).

## Fixture: reproduce an LSP session

Automated fixture (CI / local):

```bash
cargo test -p bitloom --test fr99_bitloom_lsp_server_mvp fr99_bitloom_lsp_initialize_session -- --nocapture
```

Manual smoke (send one `initialize` over stdio):

```bash
printf 'Content-Length: 152\r\n\r\n{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"processId":null,"capabilities":{},"clientInfo":{"name":"manual","version":"0"}}}' \
  | cargo run -q -p bitloom-lsp
```

Expect a JSON-RPC `result` containing `capabilities` and `serverInfo` for `bitloom-lsp`.

## Design crate dependency boundary

Design crates still depend **only** on [`bitloom-prelude`](../crates/bitloom-prelude). Do **not** add `bitloom-lsp` or the CLI package `bitloom` to design `[dependencies]`.

## Cross-links

- Host IDE (FR90): [`fr90-host-ide-rust-analyzer.md`](fr90-host-ide-rust-analyzer.md)
- Hierarchy HTML ≠ LSP: [`fr38-viz-lsp.md`](fr38-viz-lsp.md)
- NFR14 Epic 44 gate: `_agile-output/implementation-artifacts/nfr14-risk-epic44-full-elaborate-lsp.md`
