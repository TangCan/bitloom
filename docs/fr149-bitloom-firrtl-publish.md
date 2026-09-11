# FR149 — bitloom-firrtl crates.io publishability

> **Contract:** Phase 18 / **FR149** / Epic 85 / Story 85.2.  
> **Brand:** Bitloom. **AD-2:** publish as **`bitloom-firrtl`** only — never `rhdl-firrtl` / `rhdl` / `rhdl-bits`.  
> **Directory:** workspace path may remain `crates/rhdl-firrtl`.

## Package identity

| Field | Value |
| --- | --- |
| `[package].name` | `bitloom-firrtl` |
| Rust crate path | `bitloom_firrtl` |
| `publish` | `true` |
| Version | workspace `1.0.0` |
| Depends on | `bitloom-hir` `1.0.0` (crates.io) |

## Checklist

- [x] Workspace dependency key `bitloom-firrtl = { path = "crates/rhdl-firrtl", version = "1.0.0" }`
- [x] In-repo package references updated (`rhdl_firrtl` → `bitloom_firrtl`)
- [x] Filesystem fixtures still under `crates/rhdl-firrtl/`
- [x] `cargo publish -p bitloom-firrtl --dry-run` succeeds
- [x] Live `cargo publish -p bitloom-firrtl` (1.0.0) — uploaded 2026-09-11 to crates.io

## Commands

```bash
cargo publish -p bitloom-firrtl --dry-run
cargo publish -p bitloom-firrtl
```

## Honesty

- FR149 ≠ FR151 CLI install claim.
- FR149 does **not** expand FR142 SemVer surface (see `docs/public-api-1-0-surface.md`).
- NFR59 remains deferred.
