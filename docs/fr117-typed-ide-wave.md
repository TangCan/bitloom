# FR117 — Typed IDE waveform (in-house subset B)

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

**Status:** Story **58.2** product path (NFR14 subset **B**). Epic 58 / FR117 closeout → Story **58.3** (not this page alone).

## Selected deepen subset (NFR14)

| Subset | Status |
|--------|--------|
| **(B) In-house equivalent typed IDE waveform** | **FR117 face — this page** |
| (A) Tywaves first-class integration | **deferred** (NFR51) |

**Forbidden closes:** FR104 `interactive.html` I1–I3 alone; static VCD /「请开 GTKWave」alone; FR114 LCOV/`coverage.html` alone; docs-only.

## What is beyond FR104 I1–I3

| Artifact | Role |
|----------|------|
| `typed-wave.html` | **FR117** typed IDE viewer — hierarchical signals with **type / kind / module** metadata + value timeline |
| `wave.typed.json` | Machine-readable typed sidecar (`name`, `ty`, `kind`, `module`, `width`, samples) |
| `interactive.html` | FR104 I1–I3 name-only browse/zoom/search — **still emitted** (NFR48); ≠ FR117 alone |
| `timing.html` / `wave.vcd` | FR38/49 / AD-5·24 baseline — **still emitted** |

## Reproducible steps

```bash
cargo bitloom wave \
  --input crates/rhdl-firrtl/fixtures/external_hierarchy.fir \
  --out-dir target/wave \
  --ticks 8
```

Open `typed-wave.html` in a browser (Cursor/VS Code Simple Browser or system browser):

1. **Typed hierarchy:** left panel lists modules → signals with type badges (e.g. `UInt<8>`, `Clock`) and kind (`port:input`, `reg`, …).
2. **Metadata:** select a signal to inspect type / kind / module / width.
3. **Timeline:** canvas shows values for the selected typed signal.
4. **Filter:** search by name, type, kind, or module.

## Manual acceptance checklist

- [ ] `cargo bitloom wave …` succeeds and prints path to `typed-wave.html` + `wave.typed.json`
- [ ] Opening `typed-wave.html` shows Bitloom brand + typed signal tree (not I1–I3-only canvas)
- [ ] Selected signal metadata shows a concrete type string (not name-only)
- [ ] Sibling `interactive.html`, `wave.vcd`, and `timing.html` still exist (NFR48)
- [ ] Docs state Tywaves (A) remains **deferred** (NFR51)
- [ ] Must not claim Epic 58 / FR117 closed until Story 58.3

ATDD: `cargo test -p bitloom --test fr117_typed_ide_wave`

## Non-regression (NFR48)

```bash
cargo bitloom wave --input … --out-dir target/wave   # interactive.html + VCD
cargo bitloom coverage --out-dir target/cov            # coverage.lcov + coverage.html
```

## Cross-links

| Doc | Role |
|-----|------|
| [`fr104-interactive-wave.md`](fr104-interactive-wave.md) | FR104 I1–I3 — still closed; ≠ FR117 alone |
| [`fr114-lcov-coverage-gui.md`](fr114-lcov-coverage-gui.md) | FR114 LCOV GUI — still closed; ≠ FR117 alone |
| NFR14 | `_agile-output/implementation-artifacts/nfr14-risk-epic58-tywaves-typed-ide-waveform.md` |

## Non-goals (this story)

- Tywaves / Chisel typed viewer first-class integration (subset A — deferred)
- Epic 58 / FR117 documentation closeout (→ Story 58.3)
- Removing default VCD, `interactive.html`, or FR114 coverage path
