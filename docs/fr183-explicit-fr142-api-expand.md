# FR183 — Explicit FR142 public API surface expand

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 116 / FR183 in progress** (Story **116.2** implement). Product path: **explicit** additive promote of documented `bitloom-firrtl` interop into the FR142 surface list ([`docs/public-api-1-0-surface.md`](public-api-1-0-surface.md)); SemVer honesty per FR143 (**minor** for additive); **AD-6** unchanged.

Phase 17 **FR142** surface lock and **FR143** SemVer policy **remain closed and valid** (NFR83). Those closes alone ≠ FR183. Silent expand remains forbidden.

## Selected shape (NFR14)

| Layer | Role |
|-------|------|
| **FR142 baseline** | 1.0 in-surface lock (CLI / prelude / macro-via-prelude / sim) |
| **FR183 promote (S1)** | Documented `bitloom-firrtl`: `emit_chisel` (+ documented idiomatic/style/ecosystem variants), `CHISEL_TARGET` / `FIRTOOL_TARGET`, `BitloomFirrtlParser.parse` / `parseUpdateMainline` |
| **Surface doc (S2 / NFR85)** | FR183 / v1.x expand section in `public-api-1-0-surface.md`; design crates still **prelude-only** |
| **SemVer (S3)** | Additive → **minor** on next `bitloom-firrtl` publish; not silent; not major without breaking |

**Forbidden closes:** FR142 alone; “code already has `pub`” alone; surface claim without updating `public-api-1-0-surface.md`; design crates depending on `bitloom-firrtl`; promoting `bitloom-lsp` / hir / builder / vlog to 1.0-stable in this FR.

## Reproducible checks

```bash
cargo test -p bitloom --test fr183_explicit_fr142_api_expand
# surface file lists FR183 + emit_chisel + BitloomFirrtlParser + AD-6
# bitloom-prelude must not depend on bitloom-firrtl
```

## Standing honesty

Further API expands beyond this NFR14 subset need a new contract (**NFR86**).
Do **not** claim FR142 alone delivers this FR.

```text
cargo test -p bitloom --test fr183_explicit_fr142_api_expand
```
