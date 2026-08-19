# FR40 — extra CLI verbs

Shipped verbs (authoritative for Story 10.5 / FR40 as of 2026-08-19):

| Verb | Role |
|------|------|
| `build` | elaborate + Verilog (primary emit path) |
| `firtool` | pinned CIRCT binary (`ensure` / `info`) |
| `sim-engines` | list tick engines |
| `hls` | optional Bambu front-end |

Earlier PRD drafts also listed `check` / `import` / `visualize` / `wave` / `doc` / `build-sim` — those remain **deferred** wrappers around existing libraries (`emit_chisel`, `emit_sva`, `to_html`, …) and are not required for FR40 closeout.
