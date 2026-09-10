# FR114 — LCOV + in-tree coverage GUI deepen

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 56 / FR114 closed** (Story **56.3**). Subset **(B)** delivered in Story **56.2**.
Phase 12 FR104/FR105 MVP remain closed (NFR44). Subset **(A) Tywaves** stays **deferred** (NFR47).

**Phase 13:** Planning/implementation stories for Epic **48–56** are complete; remaining gates are
per-epic close evidence (this epic closed) and **FR115** claim discipline for commercial deepen.

This page is the **FR114 completion surface**. It deepens observability **beyond** Phase 12 FR104
`interactive.html` I1–I3 and FR105 Mux v2 text reports.

## Selected deepen subset (NFR14)

| Subset | Status |
|--------|--------|
| **(B) LCOV + in-tree coverage GUI** | **FR114 face — closed** (this page) |
| (A) Tywaves-class typed IDE waveform | **deferred** at Epic 56 — **Phase 14 FR117** in-house subset B → [`fr117-typed-ide-wave.md`](fr117-typed-ide-wave.md); Tywaves A still deferred (NFR51) |

**Forbidden closes:** static `timing.html` / VCD /「请开 GTKWave」alone; FR104 I1–I3 alone;
FR105 Mux v2 alone; FR109 text report alone; docs-only.

## Product path

```bash
cargo bitloom coverage --out-dir target/cov --ticks 4
# optional: --input path/to/design.fir
```

Writes:

| Artifact | Role |
|----------|------|
| `coverage.lcov` | LCOV dialect (`TN` / `SF` / `DA` / `LH` / `LF` / `end_of_record`) |
| `coverage.html` | **Bitloom** in-tree coverage GUI (`data-bitloom-coverage-gui`) — search hit/miss |

Library API (`bitloom-sim`):

```rust
use bitloom_sim::{Coverage, write_coverage_artifacts};
// or after Sim ticks:
sim.write_coverage_artifacts(out_dir)?;
```

Empty coverage → readable error `bitloom-sim.coverage-empty` (must not silent-Ok FR114).

## Recipe

```text
cargo test -p bitloom --test fr114_lcov_coverage_gui
cargo test -p bitloom --test fr114_epic56_closeout
cargo bitloom coverage --out-dir target/cov
```

## Non-regression

- FR104 `interactive.html` / VCD path remains (`cargo bitloom wave`)
- FR105 `# bitloom-sim coverage v2` text report remains (`coverage_report()`)

## Cross-links

| Doc | Role |
|-----|------|
| [`fr104-interactive-wave.md`](fr104-interactive-wave.md) | FR104 I1–I3 MVP — still closed (NFR44); ≠ FR114 alone |
| [`fr105-sim-coverage-ext.md`](fr105-sim-coverage-ext.md) | Mux v2 text — still closed; ≠ FR114 alone |
| [`fr109-fsm-state-visit-coverage.md`](fr109-fsm-state-visit-coverage.md) | C3 recorder — closed; text alone ≠ FR114 |
| NFR14 | `_agile-output/implementation-artifacts/nfr14-risk-epic56-waveform-coverage-gui.md` |

## Non-goals (NFR47)

- Tywaves-class first-class integration remains deferred; Phase 14 delivers **in-house typed IDE wave** under FR117 — see [`fr117-typed-ide-wave.md`](fr117-typed-ide-wave.md)
- Claiming FR104 I1–I3 or FR105 Mux v2 alone closes FR114
