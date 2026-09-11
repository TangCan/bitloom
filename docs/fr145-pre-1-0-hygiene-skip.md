# FR145 — Pre-1.0 surface hygiene decision (skip)

> **Contract:** Phase 17 / **FR145** / Epic 82 / Story 82.2.  
> **Q3 default:** skip when no blocking hygiene required before 1.0.0.

## Decision: **FR145-skip**

No separate blocking hygiene PR is required before **FR146 / 1.0.0**.

### Evidence

1. **FR144 gate** (`just semver-check`) runs `cargo-semver-checks` on `bitloom-prelude` and `bitloom-sim` with pre-1.0 default `--release-type major` (see `docs/semver-1-0-policy.md`).
2. Against crates.io **0.1.2**, `bitloom-prelude` shows a **major**-level `PortField` API rename (`describe` → `flatten`). That drift is **expected** to be absorbed by publishing **1.0.0** (major), not by a fake-compatible 0.x patch.
3. `bitloom-sim` reports clean under the same check.
4. No additional undocumented in-surface breakage was identified in the FR142 inventory that must be patched *before* the 1.0 tag.

### Explicit non-claims

- Skip does **not** mean “zero API change since 0.1.2”.
- Skip does **not** close **NFR59** product deepen items.
- Skip does **not** authorize silent expansion of `docs/public-api-1-0-surface.md`.

### Brand

Public product name **Bitloom**; crates.io / CLI **`bitloom`**.
