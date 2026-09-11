# FR152 — bitloom-lsp publish strategy (b)

> **Contract:** Phase 18 / **FR152** / Epic 85 / Story 85.4.  
> **Brand:** Bitloom.

## Selected policy: **(b)** default

| Option | Meaning | Status |
| --- | --- | --- |
| **(b)** | Keep `bitloom-lsp` `publish = false`, and ensure it does **not** block `cargo publish -p bitloom` | **Selected** |
| (a) | Promote `bitloom-lsp` to `publish = true` + version on crates.io | Deferred — requires a new contract |

## Workspace rules

- `crates/bitloom-lsp/Cargo.toml`: `publish = false` (unchanged).
- `crates/bitloom/Cargo.toml`: **no** `bitloom-lsp` path dependency (removed from `[dev-dependencies]`).
- LSP library ATDD that needs `use bitloom_lsp::…` lives under `crates/bitloom-lsp/tests/` so the published CLI package does not declare a path-only lsp dep.
- File/doc/CLI ATDD for LSP product claims may remain under `crates/bitloom/tests/` when they only spawn `cargo run -p bitloom-lsp` or read docs.

## Publish packaging assert

`cargo publish -p bitloom` (dry-run or live) must succeed **without** requiring `bitloom-lsp` on crates.io.

## Honesty

- FR152(b) ≠ shipping a deeper LSP product surface.
- FR152(a) / NFR59 LSP deepen remain out of this batch (**NFR67**).
- FR151 (`cargo install bitloom`) is a separate story after deps are versioned.
