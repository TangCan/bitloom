# Code Review — Story 94.2 (FR162)

## Verdict

**Pass** — default `wave` emits FR134-level GUI primary; opt-out and force-missing semantics hold; FR134/FR125 boundary ATDD updated.

## Checks

| Check | Result |
|-------|--------|
| Default `wave` → `tywaves.gui.*` | yes |
| `--no-tywaves-gui` skips GUI + sidecar | yes |
| FORCE_MISSING non-zero readable | yes |
| FR134 / FR125 alone bounds preserved | yes (via `--no-tywaves-gui`) |
| NFR71 non-goals (ChiselSim / multi-store) not claimed | docs only |

## Residual

Closeout README/deferred/AGENTS → Story 94.3.
