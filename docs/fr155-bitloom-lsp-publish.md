# FR155 — bitloom-lsp FR152(a) crates.io publish

> **Contract:** Phase 19 / **FR155** / Epic 88 / Stories 88.2–88.3.  
> **Brand:** Bitloom. Binary: `bitloom-lsp`.  
> **Policy:** [`fr152-bitloom-lsp-publish-policy.md`](fr152-bitloom-lsp-publish-policy.md) **(a)**.

## Preconditions

- [x] Epic 87 / FR154 Phase 19 gate closed
- [x] Story 88.1 NFR14 for Epic 88
- [x] Story 88.2: `publish = true` + policy (a) + `cargo publish -p bitloom-lsp --dry-run` green
- [x] Library crates (`bitloom-hir`, `bitloom-builder`, …) already 1.0.0 on crates.io
- [x] FR151 CLI path remains independent (no `bitloom-lsp` dep on `bitloom`)

## Package

| Field | Value |
| --- | --- |
| Package | `bitloom-lsp` |
| Binary | `bitloom-lsp` |
| Version | `1.0.0` |
| Runtime deps | versioned workspace crates (`bitloom-builder`, `bitloom-hir`) + crates.io crates (`tokio`, `tower-lsp-server`) |

## Checklist

- [x] `cargo publish -p bitloom-lsp --dry-run` succeeds (Story 88.2)
- [x] Live `cargo publish -p bitloom-lsp` (1.0.0) — uploaded 2026-09-12 to crates.io (**Published bitloom-lsp v1.0.0**)
- [x] Install path documented: `cargo install bitloom-lsp` → `bitloom-lsp --help`

## Install / verify

```bash
cargo install bitloom-lsp
bitloom-lsp --help
```

## Honesty

- FR155 / FR152(a) closes **crates.io packaging + live publish** for the existing LSP surface.
- **(a) ≠** shipping a deeper LSP product (feature deepen remains other FR / NFR59 items if any).
- Does **not** expand FR142 CLI command surface; does **not** rewrite FR151 close evidence.
- Live upload succeeded 2026-09-12; crates.io install path is documentable. Epic 88 formal close / README sync → Story **88.4**.
