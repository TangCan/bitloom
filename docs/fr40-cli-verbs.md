# FR40 — extra CLI verbs

Shipped verbs (authoritative; includes Story 20.5 / FR40 `import` and Story 23.2 `visualize`/`doc`):

| Verb | Role |
|------|------|
| `build` | elaborate + Verilog (primary emit path) |
| `import` | FIRRTL 6.0.0 `.fir` → same Verilog emit path（FR46 反向产品入口） |
| `visualize` / `doc` | FIRRTL `.fir` → hierarchy HTML（FR38 / FR49） |
| `firtool` | pinned CIRCT binary (`ensure` / `info`) |
| `sim-engines` | list tick engines |
| `hls` | optional Bambu front-end |
| `new` | scaffold prelude-only design crate |

```bash
cargo bitloom import --input path/to/design.fir --out-dir out
cargo bitloom import --input path/to/design.fir --out-dir out --also-fir
cargo bitloom visualize --input path/to/design.fir --out-dir out
cargo bitloom doc --input path/to/design.fir --out-dir out
```

双向互操作文档：[`fr46-chisel-import.md`](fr46-chisel-import.md)；混合夹具：`examples/chisel_mixed`。  
层次可视化：[`fr38-viz-lsp.md`](fr38-viz-lsp.md)。

Earlier PRD drafts also listed `check` / `wave` / `build-sim` — `wave` lands in Epic 23 Story 23.3; others remain deferred wrappers around existing libraries.
