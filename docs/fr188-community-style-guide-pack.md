# FR188 — Community Style Guide pack (beyond FR181)

**Product:** Bitloom (`bitloom_firrtl` / Chisel emit). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 121 / FR188 closed** (Story **121.3**). Product path: Community Style Guide pack with **`chisel-community-style-guide` + `scalafmt-community`** beyond FR181 (`emit_chisel_style_guide_pack_fr188` / `just chisel-style-guide-pack-check`).

Phase 22 **FR181** wartremover+fatal-warnings deepen and Phase 21 **FR176** ecosystem pack **remain closed and valid** (NFR88). Those closes alone ≠ FR188.

**AD-27 revised** 2026-09-14 for FR188 (**NFR90**).

## Contract (beyond FR181)

| # | Gate | Evidence |
|---|------|----------|
| **P1** | FR188 header | `FR188 community Style Guide pack` |
| **P2** | Pack markers | `chisel-community-style-guide` + `scalafmt-community` |
| **P3** | API + per-module marker | `emit_chisel_style_guide_pack_fr188` / `check_chisel_style_guide_pack_fr188` + `--- FR188 style-guide-pack ---` |
| **P4** | Combined product gate | `just chisel-style-guide-pack-check` = FR181 style-linter deepen **and** FR188 pack emit/check |

Superset of FR181: emit/check still require FR181 L1–L4 markers (and thus FR176).

## Workflow

```bash
just chisel-style-guide-pack-check
# → bash scripts/chisel-style-guide-pack-check.sh
```

Force-missing:

```bash
BITLOOM_STYLE_GUIDE_PACK_FORCE_MISSING=1 just chisel-style-guide-pack-check   # expect non-zero
```

## Forbidden closes

≠ FR181 alone; ≠ FR176 alone; ≠ FR165 alone; ≠ FR130 alone; ≠ docs-only; ≠ style-linter gate without FR188 markers.

## Non-goals (NFR91)

Arbitrary IDE plugin suite / Style Guide subsets beyond this NFR14 pack still need a new contract.

```text
cargo test -p bitloom --test fr188_community_style_guide_pack
cargo run -p bitloom --example fr188_style_guide_pack_gate
```
