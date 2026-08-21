# FR40 — extra CLI verbs

Shipped verbs (authoritative; includes Story 20.5 / FR40 `import`):

| Verb | Role |
|------|------|
| `build` | elaborate + Verilog (primary emit path) |
| `import` | FIRRTL 6.0.0 `.fir` → same Verilog emit path（FR46 反向产品入口） |
| `firtool` | pinned CIRCT binary (`ensure` / `info`) |
| `sim-engines` | list tick engines |
| `hls` | optional Bambu front-end |
| `new` | scaffold prelude-only design crate |

```bash
cargo bitloom import --input path/to/design.fir --out-dir out
cargo bitloom import --input path/to/design.fir --out-dir out --also-fir
```

双向互操作文档：[`fr46-chisel-import.md`](fr46-chisel-import.md)；混合夹具：`examples/chisel_mixed`。

Earlier PRD drafts also listed `check` / `visualize` / `wave` / `doc` / `build-sim` — those remain **deferred** wrappers around existing libraries and are not required for FR40 `import` closeout.
