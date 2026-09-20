# UJ-6 — Visualization half (hierarchy + timing)

**Product:** Bitloom. Unrelated to `samitbasu/rhdl`.

This walkthrough is the **UJ-6 visualization half**: one fixture end-to-end for hierarchy and timing product entries (FR38 / FR49). Full LSP hover/goto is **not** required for this epic.

## Fixture

Use the shipped FIRRTL hierarchy fixture:

`crates/rhdl-firrtl/fixtures/external_hierarchy.fir`

## 1. Hierarchy view

```bash
cargo bitloom visualize \
  --input crates/rhdl-firrtl/fixtures/external_hierarchy.fir \
  --out-dir target/uj6-viz
```

Open `target/uj6-viz/hierarchy.html` in a browser. You should see:

- Bitloom branding
- Modules and ports (`ExternalTop`, `Child`, …)
- Instance hierarchy (`ExternalTop → u0 : Child`)
- Mermaid flowchart block

Alias: `cargo bitloom doc …` writes the same artifact.

Details: [`fr38-viz-lsp.md`](../fr38-viz-lsp.md).

## 2. Timing / wave view

```bash
cargo bitloom wave \
  --input crates/rhdl-firrtl/fixtures/external_wave_counter.fir \
  --out-dir target/uj6-wave \
  --ticks 8
```

Open `target/uj6-wave/timing.html` in a browser (product timing table + ASCII lanes).  
Sibling `target/uj6-wave/wave.vcd` is the default dump (FR31). GTKWave/Surfer are **optional**, not the sole path.

Optional FST (needs `vcd2fst`):

```bash
cargo bitloom wave --input … --out-dir target/uj6-wave --fst
```

Cross-link: [`fr31-optional-fst.md`](../fr31-optional-fst.md) · [`fr38-wave.md`](../fr38-wave.md).

## 3. What is deferred

- Complete LSP hover/goto (language-server binary) — **not** an Epic 23 done criterion.
- Tywaves-class typed source-level wave IDE — out of scope.

## CLI verb table

See [`fr40-cli-verbs.md`](../fr40-cli-verbs.md).

仿真与波形目前只支持单模块且无实例；上述层级夹具用于可视化，波形使用独立单模块计数器。层级 wave/coverage 输入明确返回 unsupported，不会仅执行首模块。
