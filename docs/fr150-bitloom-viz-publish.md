# FR150 — bitloom-viz crates.io publishability

> **Contract:** Phase 18 / **FR150** / Epic 85 / Story 85.3.  
> **Brand:** Bitloom. **AD-2:** publish as **`bitloom-viz`** only — never `rhdl-viz` / `rhdl` / `rhdl-bits`.  
> **Directory:** workspace path may remain `crates/rhdl-viz`.

## Package identity

| Field | Value |
| --- | --- |
| `[package].name` | `bitloom-viz` |
| Rust crate path | `bitloom_viz` |
| `publish` | `true` |
| Version | workspace `1.0.0` |
| Depends on | `bitloom-hir` `1.0.0` (crates.io) |

## Checklist

- [x] Workspace dependency key `bitloom-viz = { path = "crates/rhdl-viz", version = "1.0.0" }`
- [x] In-repo package references updated (`rhdl_viz` → `bitloom_viz`)
- [x] Filesystem paths still under `crates/rhdl-viz/`
- [x] `cargo publish -p bitloom-viz --dry-run` succeeds
- [x] Live `cargo publish -p bitloom-viz` (1.0.0) — uploaded 2026-09-11 to crates.io

## Commands

```bash
cargo publish -p bitloom-viz --dry-run
cargo publish -p bitloom-viz
```

## Honesty

- FR150 ≠ FR151 CLI install claim.
- Does **not** expand FR142 SemVer surface.
- NFR59 remains deferred.
