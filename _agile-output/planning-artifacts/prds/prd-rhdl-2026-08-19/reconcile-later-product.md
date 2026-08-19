# Reconcile: later-product.md → PRD Traceability / FR / NFR

- **Input:** `_agile-output/specs/spec-rhdl/later-product.md`
- **Against:** `_agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md` §5 FR, §6 NFR, §9 Traceability
- **Job:** line-by-line WHAT coverage. No rewrite of either source.

---

## Covered bullets

Every later-product load-bearing line has a Traceability row and a matching FR/NFR body. Combined later-product lines that the PRD splits are listed as one source → multiple IDs.

| later-product (verbatim gist) | Trace row | FR/NFR body |
|---|---|---|
| Multi-clock HIR; Clash-style CDC phantom domains; DoubleFlop / SyncFIFO as language-level CDC | Multi-clock HIR; CDC phantom; DoubleFlop/SyncFIFO → **FR23** | FR23: 多时钟 HIR；Clash 风格域（或同等可执行检查）；DoubleFlop / SyncFIFO 语言级 CDC |
| Async reset; clock gating / enables | Async reset; clock gating/enables → **FR24, FR25** | FR24 异步复位；FR25 时钟门控与使能 |
| `Mem` / sync-read vs FIRRTL `cmem`/`smem` | Mem / sync-read vs cmem/smem → **FR26** | FR26: 对齐 FIRRTL `mem` / SyncReadMem；不以 CHIRRTL `cmem`/`smem` 为产品契约 |
| Analog, InOut, tri-state (top-level IO only, when added) | Analog, InOut, tri-state → **FR27** | FR27: 仅顶层 IO；非顶层拒绝 |
| Best-effort FIRRTL→Chisel Scala *generator tool* (not the interop contract) | FIRRTL→Chisel Scala generator → **FR28** | FR28: 尽力生成器；**不是**互转契约；FrozenHir↔FIRRTL 6 仍为合同 |
| HIR→RHDL source regen is debug-only already allowed by the spine; not a product interchange format | HIR→RHDL regen (debug-only) → **NFR10** | NFR10: 仅调试；非产品互转；发行测试不宣称源码往返稳定 |
| Handwritten `#[bridge]` / `#[abstraction]` / mixed `both` sim | bridge/abstraction/both → **FR29** | FR29: 手写三视图；不从 HIR 生成 TLM |
| Dual-view formal equivalence | Dual-view formal equivalence → **FR30** | FR30 |
| FST waveforms (VCD is in CAP-5) | FST waveforms → **FR31** | FR31: 可选 FST；VCD 仍为 CAP-5 默认 |
| Interpreter vs compiled `tick` engine | Interpreter vs compiled tick → **FR32** | FR32 |
| C ABI / `cdylib` for functional and cycle-accurate sim | C ABI / cdylib → **FR33** | FR33: 功能与周期精确 |
| Coverage | Coverage → **FR34** | FR34: 仿真覆盖率 |
| In-tree HLS frontend (`#[hls]`); optional later attach to Bambu/XLS, no in-house scheduler | HLS `#[hls]` + Bambu/XLS → **FR35** | FR35: 树内 `#[hls]`；调用 Bambu/XLS；**不**自研调度 |
| `rhdl-float` / synthesizable float | rhdl-float → **FR36** | FR36 |
| IP product crates (UART/SPI/I2C/FIFO/AXI) and black-box foreign IP wrappers | IP crates + black-box → **FR37** | FR37 |
| Visualization, LSP, HTML docs from HIR | Visualization, LSP, HTML → **FR38** | FR38: FrozenHir 驱动 |
| Formal / SVA export | Formal / SVA export → **FR39** | FR39 |
| Extra CLI verbs: `check`, `import`, `visualize`, `wave`, `doc`, `build-sim` | CLI verbs check/import/… → **FR40** | FR40 六动词；`cargo rhdl build` 仍为主路径 |
| firtool assets for macos / windows / linux-aarch64 | firtool macos/windows/linux-aarch64 → **NFR11** | NFR11 |
| firtool-1.156.0 until Chisel pairs it | firtool-1.156.0 until Chisel pairs → **NFR12** | NFR12: 默认钉 **1.155.0**，有记录的 Chisel 配对后再升 1.156.0 |
| rustc 1.97.1 MSRV bump | rustc 1.97.1 MSRV → **NFR13** | NFR13 |

**Missing later-product → FR/NFR:** none. All 21 WHAT lines map.

---

## Missing / orphan mappings

**Missing (later-product with no Trace/FR/NFR):** none.

**Orphans (PRD FR/NFR with no later-product source).** Traceability already labels these; they are extra scope, not dropped WHAT.

| PRD ID | Trace label | later-product? |
|---|---|---|
| **FR21** README 免责 / `rhdl-rs` | （调研缺口）README / rhdl-rs | No |
| **NFR3** linux-x64 firtool-1.155.0 pin + checksum + cache + `RHDL_FIRTOOL_PATH` | （调研缺口）linux-x64 firtool pin | No |
| **FR22** 单时钟语言表面加厚 | （调研加厚）单时钟表面 | No |

No other FR22–FR40 / NFR10–NFR13 IDs sit outside the table. NFR3/FR21/FR22 appear in both §5–6 and §9.

---

## Qualitative gaps

Coverage is complete; these are wording / strength mismatches, not unmapped bullets.

1. **CDC mechanism (FR23).** later-product names Clash-style CDC *phantom domains*. FR23 allows 「Clash 风格域（或同等可执行检查）」; §8 Q1 leaves phantom vs ClockDomain vs 库级 to architecture. Mapping exists; Clash phantom is no longer the unique WHAT.

2. **firtool 1.156.0 (NFR12).** later-product line is `firtool-1.156.0 until Chisel pairs it`. NFR12 pins **1.155.0** until a recorded Chisel pairing, then bump to 1.156.0. Trace maps the line, but the default pin is the stage-1 tool, not 1.156.0.

3. **HLS attach (FR35).** later-product: in-tree `#[hls]` plus *optional later* attach to Bambu/XLS. FR35 success requires a `#[hls]` function to *call* Bambu or XLS and produce documented artifacts. Stronger than “later attach.”

4. **HIR→RHDL regen (NFR10).** later-product frames regen as already spine-allowed (not a product interchange). PRD correctly maps it as a P0 *constraint* NFR, not a feature to build. No coverage hole; role change (reserved WHAT → enforce-not-a-contract).

5. **Mem contract (FR26).** later-product lists the contrast `Mem` / sync-read vs `cmem`/`smem` without picking. FR26 picks FIRRTL `mem` / SyncReadMem and rejects CHIRRTL `cmem`/`smem` as product contract. Decision, not a drop.
