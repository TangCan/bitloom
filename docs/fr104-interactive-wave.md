# FR104 — Interactive rich waveform

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

**Status:** Story **47.2** product path (I1–I3). **Epic 47 / FR104 + FR105 closed** (Story 47.3 delivered coverage extension).

## Contract (NFR14 I1–I3)

| # | Capability | Product evidence |
|---|------------|------------------|
| **I1 · Browse** | Multi-signal timeline canvas | `interactive.html` canvas lanes; signal names from VCD/`tick` dump |
| **I2 · Zoom** | Time-axis zoom / pan (viewport) | Zoom ± / Pan / Fit buttons; wheel zoom; drag pan |
| **I3 · Search** | Filter signals by name | `#signal-search` input |

**Static `timing.html` alone ≠ FR104.** GTKWave/Surfer on `wave.vcd` alone ≠ FR104 (optional supplement only).

## Reproducible steps

```bash
cargo bitloom wave \
  --input crates/rhdl-firrtl/fixtures/external_hierarchy.fir \
  --out-dir target/wave \
  --ticks 8
```

Writes:

| Artifact | Role |
|----------|------|
| `interactive.html` | **FR104** interactive viewer — open in a browser |
| `timing.html` | FR38/49 static value table / ASCII lanes (retained) |
| `wave.vcd` | Default dump (**AD-5 / AD-24**); optional GTKWave/Surfer |

Open `interactive.html` in a browser:

1. **Browse (I1):** waveform timeline canvas shows signals matching the dump.
2. **Zoom / pan (I2):** use Zoom ±, Pan ←→, Fit, mouse wheel, or drag on the canvas.
3. **Search (I3):** type in the signal search box to filter the displayed set.

## Manual acceptance checklist

- [ ] `cargo bitloom wave …` succeeds and prints path to `interactive.html`
- [ ] Opening `interactive.html` shows Bitloom brand + canvas timeline (not a GTKWave-only stub)
- [ ] Zoom / pan changes the viewport label time window
- [ ] Signal search filters lanes by name substring
- [ ] Sibling `wave.vcd` and `timing.html` still exist (VCD path retained)

ATDD: `cargo test -p bitloom --test fr104_interactive_wave`

## Cross-links

- Baseline wave entry: [`fr38-wave.md`](fr38-wave.md)
- Coverage extension (FR105): [`fr105-sim-coverage-ext.md`](fr105-sim-coverage-ext.md)
- FR114 LCOV + coverage GUI: [`fr114-lcov-coverage-gui.md`](fr114-lcov-coverage-gui.md) (**Epic 56 closed** / Story 56.3)
- FR117 typed IDE wave: [`fr117-typed-ide-wave.md`](fr117-typed-ide-wave.md) (**Epic 58 closed** / Story 58.3; in-house typed; Tywaves A deferred)
- NFR14 gate: `_agile-output/implementation-artifacts/nfr14-risk-epic47-waveform-coverage.md`

## Non-goals (this story)

- Tywaves-class typed source-level IDE waveform — **Phase 14 FR117 / Epic 58 closed** (in-house subset B) → [`fr117-typed-ide-wave.md`](fr117-typed-ide-wave.md); Tywaves A remains **deferred** (NFR51)
- Removing default VCD or static `timing.html`

