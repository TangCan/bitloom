# FR38 / FR49 — built-in hierarchy visualization

**Product:** Bitloom (`cargo bitloom`). Unrelated to `samitbasu/rhdl`.

## Hierarchy entry (Story 23.2)

```bash
cargo bitloom visualize --input crates/rhdl-firrtl/fixtures/external_hierarchy.fir --out-dir target/viz
# alias:
cargo bitloom doc --input crates/rhdl-firrtl/fixtures/external_hierarchy.fir --out-dir target/viz
```

Writes `hierarchy.html` with:

- Modules and ports
- Instance hierarchy list
- Mermaid flowchart of the instance tree

Open the HTML in a browser. This is the product hierarchy path for FR38/FR49 — not a library-only dump.

## LSP

Full LSP hover/goto remains **deferred** as a later epic (no language-server binary). Reinforced in `_agile-output/implementation-artifacts/deferred-work.md`. Hierarchy/HTML does **not** claim LSP done.

## Wave / timing

See [`fr38-wave.md`](fr38-wave.md) — `cargo bitloom wave` emits `timing.html` + `wave.vcd` (not GTKWave-only).
