# FR40 — extra CLI verbs

Shipped verbs (authoritative; includes Story 20.5 `import`, Story 23.2 `visualize`/`doc`, Story 23.3 `wave`):

| Verb | Role |
|------|------|
| `build` | elaborate + Verilog (primary emit path) |
| `import` | FIRRTL 6.0.0 `.fir` → same Verilog emit path（FR46 反向产品入口） |
| `visualize` / `doc` | FIRRTL `.fir` → hierarchy HTML（FR38 / FR49） |
| `wave` | FIRRTL `.fir` → VCD + browsable timing HTML（FR38 / FR49） |
| `firtool` | pinned CIRCT binary (`ensure` / `info`) |
| `sim-engines` | list tick engines |
| `hls` | optional Bambu front-end |
| `new` | scaffold prelude-only design crate |

```bash
cargo bitloom import --input path/to/design.fir --out-dir out
cargo bitloom visualize --input path/to/design.fir --out-dir out
cargo bitloom wave --input path/to/design.fir --out-dir out --ticks 8
```

双向互操作文档：[`fr46-chisel-import.md`](fr46-chisel-import.md)。  
层次：[`fr38-viz-lsp.md`](fr38-viz-lsp.md)。时序：[`fr38-wave.md`](fr38-wave.md)。

Earlier PRD drafts also listed `check` / `build-sim` — those remain deferred wrappers around existing libraries.
