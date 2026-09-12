# Story 94.2 — FR162 deeper GUI/IDE default wave surface

**Status:** done (implementation)

## Delivered

- `cargo bitloom wave` **defaults** to FR134-level GUI primary (`tywaves.gui.*` + FR125 sidecar).
- Opt-out: `--no-tywaves-gui` (legacy typed-wave / VCD-focused; FR125 needs `--tywaves`).
- Docs: `docs/fr162-deeper-gui-ide-default-wave.md`
- ATDD: `crates/bitloom/tests/fr162_deeper_gui_ide_default_wave.rs`
- FR125/FR134 boundary tests updated for FR162 default.

## Next

Story 94.3 closeout (README / deferred / AGENTS / epic complete).
