# FR122 — Official-style Chisel full pack

**Product:** Bitloom (`rhdl_firrtl` / Chisel emit). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 63 / FR122 closed** (Story **63.3**). O1–O4 + AD-27 revise delivered in Story **63.2**. FR97 / FR111 remain closed (NFR48). **Parser.parse not restored.** Full Style Guide / community linter suite still needs a new contract (NFR51).

## Contract (NFR14 O1–O4)

| # | Gate | Product evidence |
|---|------|------------------|
| **O1** | package + FR122 claim | `package bitloom.generated` + `FR122 official-style` header |
| **O2** | Official section order | Per module: appearing markers in order io→registers→wires→instances→memories→logic |
| **O3** | Per-module FR122 marker | Each class has `// --- FR122 official ---` (keeps FR111 per-module) |
| **O4** | API + multi-module | `emit_chisel_idiomatic_fr122` / `check_idiomatic_chisel_fr122`; ≥2 modules; FR97/FR111/mechanical alone fail |
| **AD** | Revise AD-27 | ARCHITECTURE-SPINE AD-27 Revised 2026-09-10 / FR122 |

**FR97 alone ≠ FR122. FR111 alone ≠ FR122. Mechanical `emit_chisel` ≠ FR122. Docs-only ≠ FR122.**

## API

```rust
use rhdl_firrtl::{
    emit_chisel_idiomatic_fr122, check_idiomatic_chisel_fr122,
};

let art = emit_chisel_idiomatic_fr122(&frozen)?;
check_idiomatic_chisel_fr122(&art.files[0].contents, &frozen)?;
```

FR111 path unchanged: `emit_chisel_idiomatic_fr111` / `check_idiomatic_chisel_fr111`.  
FR97 path unchanged: `emit_chisel_idiomatic` / `check_idiomatic_chisel`.

## Cross-links

- FR111 deepen: [`fr111-idiomatic-chisel-depth.md`](fr111-idiomatic-chisel-depth.md)
- FR97 MVP: [`fr97-idiomatic-chisel.md`](fr97-idiomatic-chisel.md)
- Mechanical FR28: [`fr28-chisel-compilable.md`](fr28-chisel-compilable.md)
- NFR14: `_agile-output/implementation-artifacts/nfr14-risk-epic63-official-style-chisel.md`
- AD-27: revised 2026-09-10 for FR122 (default still no Parser)

## Non-goals (NFR51 → Phase 15)

- Restoring deprecated `Parser.parse` (still forbidden by default)
- Full Style Guide pack → **FR130** / [`fr130-style-guide.md`](fr130-style-guide.md)
- Claiming mechanical / FR97 / FR111 alone is FR122

```text
cargo test -p bitloom --test fr122_official_style_chisel
cargo test -p bitloom --test fr122_epic63_closeout
cargo test -p bitloom --test fr111_idiomatic_chisel_depth
cargo test -p bitloom --test fr97_idiomatic_chisel
```
