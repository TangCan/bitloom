# FR38 / FR49 — timing / wave product entry

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

## Wave entry (Story 23.3 + FR104 Story 47.2)

```bash
cargo bitloom wave \
  --input crates/rhdl-firrtl/fixtures/external_wave_counter.fir \
  --out-dir target/wave \
  --ticks 8
```

Writes:

| Artifact | Role |
|----------|------|
| `interactive.html` | **FR104** interactive browse / zoom·pan / search — see [`fr104-interactive-wave.md`](fr104-interactive-wave.md) |
| `timing.html` | **FR38/49** browsable static timing/value view |
| `wave.vcd` | Default dump (FR31 / **AD-5/24**); optional input to GTKWave/Surfer |

Optional FST (requires `vcd2fst` / `RHDL_VCD2FST`):

```bash
cargo bitloom wave --input … --out-dir target/wave --fst
```

If FST conversion is unavailable, the command still succeeds with **VCD + `timing.html` + `interactive.html`**. Closing FST does not remove the VCD path.

## Not GTKWave-only

FR49 is satisfied by `timing.html` from the product CLI. External viewers remain useful for large dumps but are **not** the sole completion path for FR38/49.

**FR104** (interactive rich wave) is a separate completion face: open `interactive.html`. Static `timing.html` alone does **not** close FR104.

## Interactive wave (FR104)

Delivered in Story **47.2**: self-contained `interactive.html` (I1–I3). Contract and acceptance checklist: [`fr104-interactive-wave.md`](fr104-interactive-wave.md).

Cross-link: optional FST details in [`fr31-optional-fst.md`](fr31-optional-fst.md). Hierarchy entry: [`fr38-viz-lsp.md`](fr38-viz-lsp.md).
