# Later product — 索引（已升格）

> **本文件不再承载无 ID 的需求。**  
> 原条目已全部升格为阶段二 PRD 中的 FR/NFR。  
> 权威文件：[`../../planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md`](../../planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md)

下游不得把「本文件沉默」解释为禁止或许可；请查 PRD 中的稳定 ID。

| 原 later-product 主题 | FR / NFR | Phase |
|----------------------|----------|-------|
| Multi-clock HIR; CDC; DoubleFlop / SyncFIFO | FR23 | P1 |
| Async reset; clock gating / enables | FR24, FR25 | P1 |
| Mem / sync-read vs cmem/smem | FR26 | P1 |
| Analog, InOut, tri-state | FR27 | P2c |
| FIRRTL→Chisel Scala generator | FR28 | P2b |
| HIR→RHDL source regen (debug-only) | NFR10 | P0 |
| `#[bridge]` / `#[abstraction]` / mixed `both` | FR29 | P2a |
| Dual-view formal equivalence | FR30 | P2a |
| FST waveforms | FR31 | P2a |
| Interpreter vs compiled `tick` | FR32 | P2a |
| C ABI / cdylib | FR33 | P2a |
| Coverage | FR34 | P2a |
| HLS `#[hls]` + Bambu/XLS（无自研调度） | FR35 | P2b |
| `rhdl-float` | FR36 | P2c |
| IP crates + black-box wrappers | FR37 | P2c |
| Visualization, LSP, HTML docs | FR38 | P2c |
| Formal / SVA export | FR39 | P2b |
| CLI verbs: check, import, visualize, wave, doc, build-sim | FR40 | P2c |
| firtool macos / windows / linux-aarch64 | NFR11 | P2c |
| firtool pairing / 1.156.0 | NFR12 | P0 |
| rustc 1.97.1 MSRV | NFR13 | P2c |

阶段一缺口（原不在本文件，由调研补入 PRD）：**FR21**（README）、**NFR3**（linux-x64 firtool 钉死）、**FR22**（单时钟表面加厚）。

Architecture spine 中标记为 `Deferred` 的 HOW 仍由架构变更采纳；本索引只指向 **WHAT**（FR/NFR）。
