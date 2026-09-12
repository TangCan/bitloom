# FR162 — Deeper GUI/IDE subset (default wave primary surface)

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

**Status:** Epic 94 / FR162 — Story **94.2** implementation (closeout Story **94.3**).

Phase 16 **FR134** G1–G4 (opt-in `--tywaves-gui`) **remains closed and valid** (NFR68). This FR
**promotes** Tywaves GUI depth to the **default / primary** `cargo bitloom wave` surface so
VCD / `typed-wave.html` are **no longer the sole default completion face**.

## Selected face (NFR14)

| Layer | Role |
|-------|------|
| **FR104 / FR117** | `interactive.html` / `typed-wave.html` — **still emit**; **secondary**; alone ≠ FR162 |
| **FR125** | `--tywaves` sidecar — **still valid**; alone ≠ FR162 |
| **FR134** | G1–G4 opt-in GUI/IDE depth — **still closed**; alone ≠ FR162 (was “extra switch”) |
| **FR162** | **Default primary** = FR134-level `tywaves.gui.manifest.json` + `tywaves.gui.install.json` on plain `cargo bitloom wave` |

**Forbidden closes:** FR134 G1–G4 alone; FR125 alone; FR117 typed-wave alone; docs-only;
silent-Ok under `BITLOOM_TYWAVES_GUI_FORCE_MISSING`.

## Default / primary vs opt-out

```bash
# FR162 primary (default): GUI manifests + FR125 sidecar + typed-wave (secondary)
cargo bitloom wave \
  --input crates/rhdl-firrtl/fixtures/external_hierarchy.fir \
  --out-dir target/wave-fr162 \
  --ticks 8

# Legacy typed-wave / VCD-focused path (no GUI primary; FR125 needs --tywaves)
cargo bitloom wave ... --no-tywaves-gui

# FR134 explicit flag still accepted (compatible; default path already on)
cargo bitloom wave ... --tywaves-gui
```

Primary artifacts:

- `tywaves.gui.manifest.json` / `tywaves.gui.install.json` / `tywaves.gui.install.sh`
- (implies) `wave.tywaves.json` / `tywaves.launch.sh`

Secondary (still written): `typed-wave.html`, `interactive.html`, `wave.vcd`, `timing.html`.

## Failure semantics

| Condition | Result |
|-----------|--------|
| `BITLOOM_TYWAVES_GUI_FORCE_MISSING=1` on primary path | **non-zero** + `bitloom.tywaves*` (manifests still written) |
| Invalid `BITLOOM_TYWAVES_GUI_ROOT` | same |
| Missing root without force | soft OK + advisory (CI/ATDD stub optional) |

## Recipe

```text
cargo test -p bitloom --test fr162_deeper_gui_ide_default_wave
cargo test -p bitloom --test fr134_tywaves_gui_ide
```

## Cross-links

| Doc | Role |
|-----|------|
| [`fr134-upstream-tywaves-gui-ide.md`](fr134-upstream-tywaves-gui-ide.md) | FR134 G1–G4 (still closed; ≠ FR162 alone) |
| [`fr125-upstream-tywaves.md`](fr125-upstream-tywaves.md) | FR125 sidecar |
| [`fr117-typed-ide-wave.md`](fr117-typed-ide-wave.md) | typed-wave (secondary under FR162) |
| NFR14 | `_agile-output/implementation-artifacts/nfr14-risk-epic94-deeper-gui-ide-fr162.md` |

## Non-goals (NFR71)

- Full ChiselSim coupling
- Extra IDE-store multi-target (Open VSX / JetBrains) beyond FR134 G1 pin
- Deleting FR104/FR117 artifacts
