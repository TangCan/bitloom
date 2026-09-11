# Bitloom SemVer / release policy (pre-1.0 / 0.x era)

> **Historical for the 0.x era (NFR15).** The living 1.0 contract is [`docs/semver-1-0-policy.md`](semver-1-0-policy.md) (**FR143**). This file remains the record of why closeout stayed on **0.x**.

## Version axis

- Toolchain remains on **0.x** until maintainers intentionally declare a stable public API (**1.0**) via **FR143** / **FR146**.
- Closing a sprint or backlog is **not** a reason to bump to 1.0 (NFR15).
- Changing `rust-version` / MSRV is treated as a **minor**-incompatible change under Cargo's rules; document and bump accordingly.

## What is published

- Primary registry crate: **`bitloom`** (CLI).
- Other workspace crates default to `publish = false` until their APIs are intentionally promised.
- Phase 17 **FR142** pins which published crates carry a **1.0** SemVer promise (`docs/public-api-1-0-surface.md`).

## Tagging

- Annotated git tags `vMAJOR.MINOR.PATCH` on the release commit that is published.
- `CHANGELOG.md` section for that version must exist before tag.

## Docs

- `cargo doc -p bitloom` must succeed before publish.
- docs.rs builds from the published crate; set `[package.metadata.docs.rs]` only if needed.
