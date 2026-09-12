# FR170 — Chisel update-mainline / HEAD Parser migration

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 103 / FR170 in progress** (Story **103.2** implementation). Story **103.3** closeout pending.

Phase 16 **FR138** `BitloomFirrtlParser.parse` and Phase 19 **FR165** Style Guide/linter **remain closed and valid** (NFR73).
This FR delivers the NFR14 **document-pinned update-mainline** Parser product path beyond FR138 alone. **AD-9 is not revised** this batch (firtool stays **1.155.0**; unpaired CIRCT HEAD / firtool bump remains **NFR76** — Epic 102 option B deferred).

## Selected shape

| Layer | Role |
|-------|------|
| **FR138** | `BitloomFirrtlParser.parse` on FIRRTL **v4** fixture / `parser-restore-check` — **still valid**; alone ≠ FR170 |
| **FR165** | Style Guide / linter deepen; L3 historically deferred HEAD — **still valid**; alone ≠ FR170 |
| **FR130** | Style Guide S1–S4 — **still valid**; alone ≠ FR170 |
| **FR170** | **update-mainline** Parser: `BitloomFirrtlParser.parseUpdateMainline` on FIRRTL **version 6.0.0** (AD-3 mainline exchange header) @ Chisel **7.14.0** ↔ firtool-**1.155.0** |

**Pin (document-pinned update-mainline):** Chisel **7.14.0** ↔ firtool-**1.155.0** (same AD-9 Stack; migration is dialect/API surface, not unpaired HEAD binaries).

**Forbidden closes:** FR138 alone; FR165 alone; FR130 alone; docs-only; PATH-random firtool; unpaired CIRCT HEAD without AD-9; silent-Ok under `BITLOOM_PARSER_HEAD_FORCE_MISSING`.

## Reproducible steps

```bash
just parser-head-migration-check
# → bash scripts/parser-head-migration-check.sh

BITLOOM_PARSER_HEAD_FORCE_MISSING=1 bash scripts/parser-head-migration-check.sh
# → non-zero; refusing silent success
```

CI: required job **`parser-head-migration`** (no `continue-on-error`).

Fixture: `crates/rhdl-firrtl/fixtures/fr170_chisel_head_parser.fir` (`FIRRTL version 6.0.0`).

Scala façade (interop only; not a design-crate dep):
`crates/rhdl-firrtl/testdata/fr170_bitloom_firrtl_parser_mainline.scala`.

## AD-27 + Correct Course (NFR75)

- ARCHITECTURE-SPINE **AD-27** revised **2026-09-12** for FR170 update-mainline Parser.
- Correct Course trail: `correctCoursePhase20Approved: 2026-09-12`
  (`sprint-change-proposal-2026-09-12-phase20-nfr71-four-leftovers.md`).

## Standing honesty

Unpaired Chisel/CIRCT **HEAD binaries** / firtool beyond 1.155.0 require formal pairing + **AD-9** revise (**NFR76**).
Scala/Parser runtime must **not** enter `bitloom-prelude` design crates.

```text
cargo test -p bitloom --test fr170_chisel_head_parser
just parser-head-migration-check
```
