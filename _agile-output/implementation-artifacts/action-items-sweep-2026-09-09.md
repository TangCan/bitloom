# Action-items sweep — 2026-09-09

Cleared **29** open retro action items (epics 26–35). Authoritative dispositions also landed in `deferred-work.md` and (for diff purity) `process-one-story-one-commit.md`.

| ID | Short theme | Disposition |
|----|-------------|-------------|
| 58 | Gate: Epic 27–30 ready / NFR14+PRD | **done-satisfied** — epics 27–35 + NFR14 stories done |
| 59 | Diff purity on contract commits | **done-deferred** — process note |
| 60 | Gate: Epic 28 ready (FR75 + Epic 27) | **done-satisfied** |
| 61 | Typed Wire/Reg → auto hw-capture | **done-deferred** — future typed trigger |
| 62 | bitloom-builder domain split | **done-implemented** — `closures.rs` extract (+ defer further) |
| 63 | Gate: Epic 29 ready (29.1 NFR14 + D1) | **done-satisfied** |
| 64 | Typed inline → rustc closure analysis | **done-deferred** — future typed trigger |
| 65 | Fulfill builder split assessment | **done-implemented** — same as 62 |
| 66 | Gate: Epic 30 ready (30.1 NFR14) | **done-satisfied** |
| 67 | Typed inline + FR76 descriptor path | **done-deferred** — future typed trigger |
| 68 | ip.rs size watch + builder debt | **done-deferred** — ip split budget; builder first step done |
| 69 | Gate: Epic 31 ready (31.1 NFR14) | **done-satisfied** |
| 70 | Typed inline + D1/FR78 host path | **done-deferred** — future typed trigger |
| 71 | Optional: DUT busy × FR47 deep fixture | **done-deferred** — optional product |
| 72 | Gate: Epic 32 ready (32.1 NFR14) | **done-satisfied** |
| 73 | SyncFIFO vs SyncFifo NFR37 | **done-deferred** — standing contract |
| 74 | Optional: DEPTH/WIDTH / dual clk ports | **done-deferred** — optional product |
| 75 | Gate: Epic 33 ready (33.1 NFR14) | **done-satisfied** |
| 76 | Bundle nesting honesty + diff purity | **done-deferred** — standing contract |
| 77 | Optional: ≥2-level / HwVec&lt;Bundle&gt; | **done-deferred** — optional product |
| 78 | Gate: Epic 34 ready (34.1 NFR14) | **done-satisfied** |
| 79 | Mem/Chisel Path A + E0901 honesty | **done-deferred** — standing contract |
| 80 | Optional: dual-clk mem / Path B | **done-deferred** — optional product |
| 81 | Gate: Epic 35 ready (35.1 NFR14) | **done-satisfied** |
| 82 | FR82 five-class vs stub NFR37 | **done-deferred** — standing contract |
| 83 | Optional: full protocol / VIP / baud | **done-deferred** — optional product |
| 84 | NFR14 before deepening FR83–85 / new epic | **done-deferred** — standing contract |
| 85 | C ABI / SoftF16 / formal / LSP honesty | **done-deferred** — standing contract |
| 86 | Optional: C loader / SoftF16 Path A / LSP bin | **done-deferred** — optional product |

## Engineering

- **Builder:** `bitloom-builder` `mod closures` (~379 LOC) extracted; `lib.rs` ~2731; public API unchanged via `pub use closures::*`.
- **ip.rs:** assessed ~1357 LOC; **no split this sweep**; defer until ≥~1800 or new IP class (see deferred-work).

## Tests

- `cargo fmt -p bitloom-builder`
- `cargo test -p bitloom-builder --lib` — 45 passed
