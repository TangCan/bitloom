# FR181 — Deeper Style Guide / linter (beyond FR176)

**Product:** Bitloom (`bitloom_firrtl` / Chisel emit). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 114 / FR181 closed** (Story **114.3**). Product path: Style Guide / linter deepen beyond FR176 (`emit_chisel_style_linter_fr181` / `just chisel-style-linter-deepen-check`).

Phase 21 **FR176** combined ecosystem pack and Phase 19 **FR165** L1–L5 **remain closed and valid** (NFR83). Those closes alone ≠ FR181.

**AD-27 revised** 2026-09-14 for FR181 (**NFR85**).

## Contract (beyond FR176)

| # | Gate | Evidence |
|---|------|----------|
| **L1** | FR181 header | `FR181 Style Guide / linter deepen` |
| **L2** | Style-linter markers | `chisel-wartremover-rules` + `fatal-warnings-lint` |
| **L3** | API + per-module marker | `emit_chisel_style_linter_fr181` / `check_chisel_style_linter_fr181` + `--- FR181 style-linter ---` |
| **L4** | Combined product gate | `just chisel-style-linter-deepen-check` = FR176 ecosystem **and** FR181 emit/check |

Superset of FR176: emit/check still require FR176 E1–E4 markers (and thus FR165).

## Workflow

```bash
just chisel-style-linter-deepen-check
# → bash scripts/chisel-style-linter-deepen-check.sh
```

Force-missing:

```bash
BITLOOM_STYLE_LINTER_DEEPEN_FORCE_MISSING=1 just chisel-style-linter-deepen-check   # expect non-zero
```

## Forbidden closes

≠ FR176 alone; ≠ FR165 alone; ≠ FR130 alone; ≠ FR138 alone; ≠ docs-only; ≠ ecosystem gate without FR181 markers.

## Non-goals (NFR86)

Full community Style Guide 全家桶 / IDE plugin suite still needs a new contract.

```text
cargo test -p bitloom --test fr181_deeper_style_guide_linter
cargo run -p bitloom --example fr181_style_linter_gate
```
