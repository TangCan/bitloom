# FR38 / FR49 — timing / wave product entry

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

## Wave entry (Story 23.3)

```bash
cargo bitloom wave \
  --input crates/rhdl-firrtl/fixtures/external_hierarchy.fir \
  --out-dir target/wave \
  --ticks 8
```

Writes:

| Artifact | Role |
|----------|------|
| `timing.html` | **Product** browsable timing/value view (open in a browser) |
| `wave.vcd` | Default dump (FR31); optional input to GTKWave/Surfer |

Optional FST (requires `vcd2fst` / `RHDL_VCD2FST`):

```bash
cargo bitloom wave --input … --out-dir target/wave --fst
```

If FST conversion is unavailable, the command still succeeds with **VCD + `timing.html`**. Closing FST does not remove the VCD path.

## Not GTKWave-only

FR49 is satisfied by `timing.html` from the product CLI. External viewers remain useful for large dumps but are **not** the sole completion path.

## Deferred — richer interactive wave

Interactive / richer waveform browsing beyond the static `timing.html` value table is a **future epic** (not claimed done here). See `_agile-output/implementation-artifacts/deferred-work.md`.

Cross-link: optional FST details in [`fr31-optional-fst.md`](fr31-optional-fst.md). Hierarchy entry: [`fr38-viz-lsp.md`](fr38-viz-lsp.md).
