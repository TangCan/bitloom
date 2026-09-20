# FR117 — Typed IDE waveform (in-house subset B)

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

**Status:** **Epic 58 / FR117 closed** (Story **58.3**). Subset **(B)** delivered in Story **58.2**.
Phase 12 FR104/FR105 and Phase 13 FR114 remain closed (NFR48/NFR52). Subset **(A) Tywaves** was **deferred** from FR117 (historical NFR51); Phase 15 product path is **FR125** (≠ this page).

This page is the **FR117 completion surface**. It deepens observability **beyond** FR104
`interactive.html` I1–I3 and FR114 LCOV/`coverage.html`.

## Selected deepen subset (NFR14)

| Subset | Status |
|--------|--------|
| **(B) In-house equivalent typed IDE waveform** | **FR117 face — closed** (this page) |
| (A) Tywaves first-class integration | **deferred from FR117** (historical NFR51); product path now **FR125 / Epic 65** (≠ this page) |

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
  --input crates/rhdl-firrtl/fixtures/external_wave_counter.fir \
  --out-dir target/wave \
  --ticks 8
```

Open `typed-wave.html` in a browser (Cursor/VS Code Simple Browser or system browser):

1. **Typed hierarchy:** left panel lists modules → signals with type badges (e.g. `UInt<8>`, `Clock`) and kind (`port:input`, `reg`, …).
2. **Metadata:** select a signal to inspect type / kind / module / width.
3. **Timeline:** canvas shows values for the selected typed signal.
4. **Filter:** search by name, type, kind, or module.

## Manual acceptance checklist

Satisfied by Story **58.2** ATDD + Story **58.3** closeout (re-run locally if needed):

- [x] `cargo bitloom wave …` succeeds and prints path to `typed-wave.html` + `wave.typed.json`
- [x] Opening `typed-wave.html` shows Bitloom brand + typed signal tree (not I1–I3-only canvas)
- [x] Selected signal metadata shows a concrete type string (not name-only)
- [x] Sibling `interactive.html`, `wave.vcd`, and `timing.html` still exist (NFR48)
- [x] Docs state Tywaves (A) was **deferred** from FR117 (NFR51 historical); **FR125** is the Tywaves claim face
- [x] Epic 58 / FR117 closed on subset B only (Story 58.3); Tywaves A not claimed **as FR117**

ATDD: `cargo test -p bitloom --test fr117_typed_ide_wave` · closeout `cargo test -p bitloom --test fr117_epic58_closeout`

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
| [`fr162-deeper-gui-ide-default-wave.md`](fr162-deeper-gui-ide-default-wave.md) | FR162 default GUI primary — typed-wave is secondary |
| NFR14 | `_agile-output/implementation-artifacts/nfr14-risk-epic58-tywaves-typed-ide-waveform.md` |

## Non-goals

- Tywaves / Chisel typed viewer first-class integration as **FR117** close (subset A was **deferred** from FR117; see **FR125** / [`fr125-upstream-tywaves.md`](fr125-upstream-tywaves.md) for the Phase 15 product path)
- Removing default VCD, `interactive.html`, or FR114 coverage path
- Silent claim that subset A is delivered **as FR117**
