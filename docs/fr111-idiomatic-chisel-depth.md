# FR111 — Idiomatic Chisel maintainability depth

**Product:** Bitloom (`rhdl_firrtl` / Chisel emit). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 53 / FR111 closed** (Story **53.3**). D1+D3 deepen path delivered in Story **53.2**. FR97 MVP remains closed (NFR44). Official-style full pack → **FR122** / [`fr122-official-style-chisel.md`](fr122-official-style-chisel.md) (Epic 63).

## Contract (NFR14 D1 + D3)

| # | Gate | Product evidence |
|---|------|------------------|
| **D1** | Multi-module style consistency | ≥2 modules; each class has `// --- IO ---` |
| **D3** | Stricter style subset | Each class has `// --- FR111 per-module ---`; file claims `FR111 deepen` |

**FR97 MVP alone ≠ FR111. Mechanical `emit_chisel` ≠ FR111. Docs-only ≠ FR111. Parser restore not required.**

## API

```rust
use rhdl_firrtl::{
    emit_chisel_idiomatic_fr111, check_idiomatic_chisel_fr111,
};

let art = emit_chisel_idiomatic_fr111(&frozen)?;
check_idiomatic_chisel_fr111(&art.files[0].contents, &frozen)?;
```

FR97 path unchanged: `emit_chisel_idiomatic` / `check_idiomatic_chisel`.

## Cross-links

- FR97 MVP: [`fr97-idiomatic-chisel.md`](fr97-idiomatic-chisel.md)
- Mechanical FR28: [`fr28-chisel-compilable.md`](fr28-chisel-compilable.md)
- NFR14: `_agile-output/implementation-artifacts/nfr14-risk-epic53-idiomatic-chisel-depth.md`

## Non-goals (NFR47 → Phase 14)

- Restoring deprecated `Parser.parse` (still forbidden by default under FR122 / AD-27)
- Claiming mechanical emit is FR111
- FR122 official-style pack is a **separate** contract — see [`fr122-official-style-chisel.md`](fr122-official-style-chisel.md)

```text
cargo test -p bitloom --test fr111_idiomatic_chisel_depth
cargo test -p bitloom --test fr111_epic53_closeout
cargo test -p bitloom --test fr97_idiomatic_chisel
```
