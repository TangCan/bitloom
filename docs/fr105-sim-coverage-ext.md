# FR105 — Simulation coverage recording extension

**Product:** Bitloom (`cargo bitloom` / `bitloom-sim`). Unrelated to `samitbasu/rhdl`.

**Status:** Story **47.3** delivered. **Epic 47 / FR104 + FR105 closed.** Phase 12 implementation story list complete; remaining literal-green gate is only each epic’s implementation close (planning stories already complete; optional retros may stay optional).

## Contract (NFR14 C* / R*)

| # | Metric / artifact | Product evidence |
|---|-------------------|------------------|
| **C1 · Toggle** | Port/reg toggle hit/miss (FR34 baseline) | `hit` / `miss` lines retained in report |
| **C2 · Branch** | Mux decision-point coverage | `branch_hit` / `branch_miss` for `mux:<sel>:t` / `mux:<sel>:f` |
| **C3 · FSM** | State-enum visit coverage | Phase 12 **MVP crop** in 47.3; **Phase 13 FR109** delivers (`docs/fr109-fsm-state-visit-coverage.md`) |
| **R1 · Report** | Stable parseable dialect | `# bitloom-sim coverage v2` header |
| **R2 · Recorder** | Runnable sim-path recorder | Enabled on `Sim::tick` → `coverage_report()` |

**FR34 toggle alone ≠ FR105.** Docs-only / no recorder ≠ FR105.

## Report dialect (v2)

```
# bitloom-sim coverage v2
hit y
miss clk
branch_hit mux:sel:f
branch_miss mux:sel:t
```

- Toggle lines (`hit` / `miss`) keep FR34 semantics (`parse_report`).
- Branch lines (`branch_hit` / `branch_miss`) are Mux arms taken / never taken (`parse_branch_report`).

## Fixture (reproducible)

ATDD builds a Mux DUT (`sel` / `a` / `b` → `y`), drives only the false arm, toggles `a`, then asserts:

- header `coverage v2`
- ≥1 `branch_hit` and ≥1 `branch_miss`
- ≥1 toggle `hit` and ≥1 toggle `miss`

```text
cargo test -p bitloom --test fr105_sim_coverage_ext
cargo test -p bitloom --test fr105_epic47_closeout
```

## Cross-links

- Baseline toggle: [`fr34-sim-coverage.md`](fr34-sim-coverage.md)
- Interactive waveform (FR104): [`fr104-interactive-wave.md`](fr104-interactive-wave.md)
- NFR14 gate: `_agile-output/implementation-artifacts/nfr14-risk-epic47-waveform-coverage.md`

## Non-goals (this story / MVP crop)

- **C3 FSM / state-visit coverage** — cropped for Phase 12 MVP in 47.3; **delivered via FR109 / Epic 51** (Story 51.2; see [`fr109-fsm-state-visit-coverage.md`](fr109-fsm-state-visit-coverage.md))
- LCOV / third-party coverage GUI — **Phase 13 FR114** (see [`fr114-lcov-coverage-gui.md`](fr114-lcov-coverage-gui.md))
- Removing FR34 toggle sampling
- Weakening FR104 `interactive.html`
- Starting Epic 48+ (historical; Phase 13 contract supersedes)
