# NFR13 — MSRV rustc 1.97.1

**Status (2026-08-19):** **Satisfied.** NFR13 targets the pinned toolchain **rustc 1.97.1** (not 1.98.0).

## Decision

Earlier PRD drafts said “可升至 rustc 1.98.0”. Official `static.rust-lang.org` never published `rust-1.98.0` (HTTP 404 while stable remained 1.97.1). Product decision: **revise NFR13** so MSRV **is** the workspace pin:

| Surface | Value |
|---------|--------|
| `rust-toolchain.toml` | `channel = "1.97.1"` |
| `[workspace.package] rust-version` | `"1.97.1"` |
| README / AGENTS / architecture Stack | rustc **1.97.1** / edition 2024 |

Future bumps (1.99+, etc.) require a new PRD/AD change — they are not implied by NFR13.

## Verification

```bash
rustc --version   # rustc 1.97.1 …
just test
```
