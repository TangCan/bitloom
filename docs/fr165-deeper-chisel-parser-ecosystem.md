# FR165 — Deeper Chisel / Parser ecosystem (Style Guide / linter pack)

**Product:** Bitloom (`bitloom_firrtl` / Chisel emit). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 97 / FR165 in progress** (Story **97.2** product path). NFR14 selected **Style Guide / linter deepen**; **arbitrary Chisel HEAD Parser migration deferred** (**NFR71**).

**≠ FR138 alone** (`BitloomFirrtlParser.parse` P1–P4 still valid). **≠ FR130 alone** (S1–S4 Style Guide markers still valid; NFR68). **AD-27 not revised** this batch (**NFR70**).

## Contract (beyond FR130)

| # | Gate | Evidence |
|---|------|----------|
| **L1** | FR165 header | `FR165 Style Guide / linter pack` |
| **L2** | Community lint markers | `chisel-lint-rules` + `scalafmt.conf` + `import-hygiene-lint` |
| **L3** | HEAD Parser deferred | `no-Chisel-HEAD-Parser` |
| **L4** | API + per-module marker | `emit_chisel_style_guide_fr165` / `check_chisel_style_guide_fr165` + `--- FR165 style-lint ---` |
| **L5** | Product gate | `just chisel-style-lint-check` |

Superset of FR130: emit/check still require FR130 S1–S4 markers.

## Workflow

```bash
just chisel-style-lint-check
# or
bash scripts/chisel-style-lint-check.sh
```

Force-missing:

```bash
BITLOOM_STYLE_LINT_FORCE_MISSING=1 just chisel-style-lint-check   # expect non-zero
```

## Forbidden closes

≠ FR138 P1–P4 alone; ≠ FR130 S1–S4 alone; ≠ FR122 alone; ≠ docs-only; ≠ Chisel HEAD Parser migration (deferred).

## Non-regression (NFR68)

FR97 / FR111 / FR122 / FR130 / FR138 closes remain valid.

```text
cargo test -p bitloom --test fr165_deeper_chisel_parser_ecosystem
cargo run -p bitloom --example fr165_style_lint_gate
```
