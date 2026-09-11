# FR151 — bitloom CLI 1.0.0 crates.io publish

> **Contract:** Phase 18 / **FR151** / Epic 85 / Story 85.5.  
> **Brand:** Bitloom. Binary: `cargo-bitloom` (invoke as `cargo bitloom` after install).

## Preconditions

- [x] FR149 `bitloom-firrtl` 1.0.0 on crates.io
- [x] FR150 `bitloom-viz` 1.0.0 on crates.io
- [x] FR152(b) lsp does not block packaging
- [x] Library crates (`bitloom-hir`, …) already 1.0.0 on crates.io

## Package

| Field | Value |
| --- | --- |
| Package | `bitloom` |
| Binary | `cargo-bitloom` |
| Version | `1.0.0` |
| Runtime deps | versioned workspace crates (`bitloom-firrtl`, `bitloom-viz`, …) — no path-only blockers |

## Checklist

- [x] `cargo publish -p bitloom --dry-run` succeeds
- [x] Live `cargo publish -p bitloom` (1.0.0) — uploaded 2026-09-11 to crates.io (**Published bitloom v1.0.0**)
- [x] Install path documented: `cargo install bitloom` → `cargo bitloom --help`

## Install / verify

```bash
cargo install bitloom
cargo bitloom --help
# or: cargo-bitloom --help
```

## Honesty

- FR151 closes CLI **installability**; does **not** expand FR142 command surface.
- NFR59 remains deferred (**NFR67**).
- FR153 (SemVer assume-published / release honesty) remains Epic 86.
