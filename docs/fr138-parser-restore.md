# FR138 — Restore deprecated Parser as product close condition

**Product:** Bitloom (`bitloom` CLI / FIRRTL interop). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 77 / FR138 closed** (Story **77.3**). Product path + **AD-27 revise** delivered in Story **77.2** (P1–P4).

Phase 16 **规划故事已齐（Epic 72–78）**；实现关闭态：**Epic 72–78 全部已关闭**（含本 FR / Epic 77）。FR130 Style Guide S1–S4 **仍有效**（NFR56）and is **not** this face alone（Style Guide alone ≠ FR138）。未列更深 Chisel/Parser 生态（社区 Style Guide/linter 全家桶、任意 Chisel HEAD Parser 回迁等）仍属 **NFR59**。终局宣称须对应 **FR133–139** 关闭后方可勾选（**FR140**）。

## Product-equivalent API (P1)

Upstream Scala `firrtl.Parser.parse` / `Parser.parse` was removed with the CIRCT migration
([chipsalliance/chisel#4899](https://github.com/chipsalliance/chisel/issues/4899)).

| Role | Name |
|------|------|
| Historical | `firrtl.Parser.parse` / `Parser.parse` |
| **Bitloom product-equivalent (AD-27)** | **`BitloomFirrtlParser.parse`** (`bitloom.firrtl`) |
| Implementation | AD-9 **`firtool-1.155.0 -parse-only`** on representative `.fir` |

Scala façade (interop/fixture only; **not** a design-crate dependency):
`crates/rhdl-firrtl/testdata/fr138_bitloom_firrtl_parser.scala`.

## Workflow

```text
representative .fir
  → BitloomFirrtlParser.parse (≡ Parser.parse)
  → firtool-1.155.0 -parse-only
  → target/parser-restore-check/*.parse-ok
```

Documented commands:

```bash
just parser-restore-check
# or
bash scripts/parser-restore-check.sh
```

Fixture: `crates/rhdl-firrtl/fixtures/fr138_parser_restore.fir`.

## Version pairing (AD-9)

**Chisel 7.14.0** ↔ **firtool-1.155.0**. ≠ PATH-random firtool; ≠ CIRCT HEAD.

## Failure semantics (P3)

| Trigger | Behavior |
|---------|----------|
| `BITLOOM_PARSER_FORCE_MISSING=1` | non-zero + readable (Parser / BitloomFirrtlParser unavailable) |
| firtool missing / ensure fail | non-zero + readable |
| version ≠ 1.155.0 | non-zero + readable mismatch |
| `-parse-only` fail | non-zero |

Silent skip is forbidden.

## AD-27 + Correct Course (P2 / NFR58)

- ARCHITECTURE-SPINE **AD-27** revised **2026-09-11** to allow FR138 Parser / `BitloomFirrtlParser.parse` as a product close condition.
- Correct Course trail reused: `correctCoursePhase16Approved: 2026-09-11`
  (`_agile-output/planning-artifacts/sprint-change-proposal-2026-09-11-phase16-nfr55-final-closeout.md`).
- Close condition = **P1–P4** (API/workflow/pairing + AD-27 revise + failure semantics + ATDD) — delivered in 77.2; docs/NFR14 closeout in 77.3.

## Forbidden closes (P4)

≠ **FR130** Style Guide alone (S3 Parser not restored for that FR — close remains valid; NFR56).
≠ **FR122** O1–O4 alone; ≠ **FR111** alone; ≠ **FR97** alone; ≠ **docs-only**.

## Non-regression (NFR56)

FR97 / FR111 / FR122 / FR130 closes remain valid. FR130 Style Guide S1–S4 (including S3 wording for that FR) still holds; FR138 is the separate Parser **product** path.

```text
cargo test -p bitloom --test fr138_parser_restore_ad27
cargo test -p bitloom --test fr138_epic77_closeout
BITLOOM_PARSER_FORCE_MISSING=1 just parser-restore-check   # expect non-zero
```
