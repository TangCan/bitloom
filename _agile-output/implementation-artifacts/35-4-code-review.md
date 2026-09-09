# Code Review — Story 35.4 (FR85 formal fixture + LSP deferred)

**Verdict: Approve**

**Date:** 2026-09-09  
**Scope:** FR85 Option A — Counter `emit_sva` fixture + `just formal-sva-check` / Verilator; LSP deferred docs; NFR14 Epic 35 close

## Summary

FR85 **Option A** is landed: real Counter HIR exports SVA; documented `scripts/formal-sva-check.sh` invokes Verilator (or optional `sby`) and fails clearly when the checker is missing. `check_sva_text` remains FR39 toy-only and is not used to close FR85. LSP hover/goto is documented as **not** an Epic 35 completion criterion. NFR14 Epic 35 close checklist fully ticked. No fake language-server. No blocking defects.

## AC checklist

| AC | Result |
| --- | --- |
| Given 35.1–35.3; real design exports SVA (or documented formal toolchain) and runs a **non-toy** check (FR85) | **pass** — Counter → `emit_sva` → fixture + `just formal-sva-check` (Verilator) |
| Acceptance exceeds `check_sva_text` toy assertions | **pass** — external tool path; ATDD forbids toy-only close |
| User docs: LSP hover/goto **not** this epic’s completion criterion (remains deferred) | **pass** — `docs/fr38-viz-lsp.md` Epic 35 paragraph |
| NFR14 Epic 35 close conditions ticked | **pass** — FR83/84/85/LSP/NFR37/禁止事项 all `[x]` |

## Findings

1. **None blocking.** Export example, committed fixture, script missing-tool failure, docs, addendum, and NFR14 agree on Option A.
2. **Non-blocking:** Default checker is `verilator --lint-only --assert` (parse/assert enablement), not a full SymbiYosys proof. Matches NFR14 “不必自研完整 model checker” and documented optional `sby` hook.
3. **Non-blocking:** Field (a) still narrates Partial *status quo* (toy `check_sva_text`); decision of record is options table **已选 A** + close checklist.

## Verification run (targeted; no full `just test`)

- `cargo test -p bitloom --test fr85_formal_fixture_beyond_toy` — 7 OK
- `just formal-sva-check` — OK (verilator)
- `BITLOOM_FORMAL_FORCE_MISSING=1` path — non-zero (via ATDD)
- `cargo test -p bitloom --test nfr14_risk_epic35_residual_partials` — OK
- `cargo test -p rhdl-formal` — OK
- `cargo test -p bitloom --test fr84_softf16_explicit_defer` — OK (regression)

## testarch-automate

ATDD + documented `just formal-sva-check` gate are sufficient; no additional automate pass required.
