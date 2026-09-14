# FR176 — Deeper Parser / Chisel ecosystem (combined pack)

**Product:** Bitloom (`bitloom_firrtl` / Chisel emit + FIRRTL Parser gates). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 109 / FR176 closed** (Story **109.3**). Product path delivered in Story **109.2** (combined Style Guide + update-mainline ecosystem deepen beyond FR165 / FR170 alone).

**≠ FR170 alone** (`BitloomFirrtlParser.parseUpdateMainline` still valid). **≠ FR165 alone** (L1–L5 still valid). **≠ FR138 alone**. **≠ FR130 alone**. **AD-27 revised** 2026-09-12 for FR176 (**NFR80**).

## Contract (beyond FR165 + FR170)

| # | Gate | Evidence |
|---|------|----------|
| **E1** | FR176 header | `FR176 Chisel ecosystem pack` |
| **E2** | Ecosystem markers | `chisel-ecosystem-pack` + `parser-mainline-bridge` + `chisel-official-style-pack` |
| **E3** | API + per-module marker | `emit_chisel_ecosystem_fr176` / `check_chisel_ecosystem_fr176` + `--- FR176 ecosystem ---` |
| **E4** | Combined product gate | `just chisel-ecosystem-deepen-check` = FR165 style-lint **and** FR170 parser-head-migration **and** FR176 emit/check |

Superset of FR165: emit/check still require FR165 L1–L5 markers. Live Parser path remains the FR170 sub-gate @ AD-9 **firtool-1.158.0 ↔ Chisel 7.15.0**.

## Workflow

```bash
just chisel-ecosystem-deepen-check
# → bash scripts/chisel-ecosystem-deepen-check.sh
```

Force-missing:

```bash
BITLOOM_ECOSYSTEM_FORCE_MISSING=1 just chisel-ecosystem-deepen-check   # expect non-zero
```

## Forbidden closes

≠ FR170 alone; ≠ FR165 alone; ≠ FR138 alone; ≠ FR130 alone; ≠ docs-only; ≠ style-lint without parser-head; ≠ parser-head without FR176 markers.

## Non-regression (NFR78)

FR97 / FR111 / FR122 / FR130 / FR138 / FR165 / FR170 closes remain valid.

Deeper Style Guide / linter beyond this combined pack → **FR181** / Epic 114（[`docs/fr181-deeper-style-guide-linter.md`](fr181-deeper-style-guide-linter.md)）.

```text
cargo test -p bitloom --test fr176_deeper_parser_chisel_ecosystem
cargo run -p bitloom --example fr176_ecosystem_gate
```
