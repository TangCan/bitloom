# Later product — 索引

> **本文件不承载无 ID 的需求。**  
> 权威 WHAT 细目：[`../../planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md`](../../planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md)（amendment `overview-literal-C-2026-08-21`）。  
> 跨阶段 CAP 合同：[`SPEC.md`](SPEC.md) CAP-1…CAP-16。  
> HOW：[`ARCHITECTURE-SPINE.md`](../../planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md)。

下游不得把「本文件沉默」解释为禁止或许可；请查 PRD 稳定 ID 与 SPEC CAP。

| 主题 | FR / NFR | SPEC CAP | Phase |
|------|----------|----------|-------|
| Multi-clock HIR; CDC; DoubleFlop / SyncFIFO | FR23 | CAP-11（叙事收口） | P1 / P3 |
| Async reset; clock gating / enables | FR24, FR25 | — | P1 |
| Mem / sync-read vs cmem/smem | FR26 | — | P1 |
| Analog, InOut, tri-state | FR27 | — | P2c |
| FIRRTL→Chisel Scala（可编译） | FR28 | CAP-12 | P2b |
| HIR→源码再生（调试-only） | NFR10 | 非 CAP-12 | P0 |
| `#[bridge]` / `#[abstraction]` / mixed `both` | FR29 | CAP-6 / CAP-13 | P2a |
| Dual-view formal equivalence | FR30 | CAP-13 | P2a → P3 |
| FST waveforms | FR31 | —（VCD=CAP-5） | P2a |
| Interpreter vs compiled `tick` | FR32 | — | P2a |
| C ABI / cdylib | FR33 | — | P2a |
| Coverage | FR34 | — | P2a |
| HLS `#[hls]` + 外挂调度 | FR35, **FR50** | CAP-16 | P2b / P3 |
| `bitloom-float` | FR36 | — | P2c |
| IP crates + black-box | FR37, **FR48** | CAP-14 | P2c / P3 |
| Visualization, HTML; LSP 可分期 | FR38, **FR49** | CAP-15 | P2c / P3 |
| Formal / SVA export | FR39 | — | P2b |
| CLI verbs: import / visualize / wave / … | FR40 | CAP-9 / CAP-12 / CAP-15 | P2c / P3 |
| Bitloom ↔ Chisel 双向 | **FR46** | CAP-12 | P3 |
| 双模拟器生成 | **FR47** | CAP-13 | P3 |
| 一级 IP 五类 | **FR48** | CAP-14 | P3 |
| 内置层次+时序图 | **FR49** | CAP-15 | P3 |
| Bundle / Vec | **FR51** | CAP-10 | P3 |
| ClockDomain 产品叙事 | **FR52** | CAP-11 | P1 / P3 |
| firtool macos / windows / linux-aarch64 | NFR11 | — | P2c |
| firtool pairing / 钉死 | NFR3, NFR12 | — | P0 |
| rustc 1.97.1 MSRV | NFR13 | — | P2c |
| P3 风险门禁 | **NFR14** | Constraint / AD-28 | P3 |

阶段一缺口（已在 PRD）：**FR21**（README / Bitloom 身份）、**FR22**（单时钟表面加厚）。

### 历史 ID 别名（勿混）

| 别名 | 含义 | ≠ |
|------|------|---|
| **FR46-tp** | Phase 3 Trusted Publishing | PRD **FR46** Chisel 双向 |
| **NFR14-crates** | crates.io FCFS | PRD **NFR14** P3 风险门禁 |

Architecture spine 中 `Deferred` 的 HOW 仍由架构变更采纳；本索引只指向 **WHAT**（FR/NFR ↔ CAP）。
