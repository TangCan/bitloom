# Bitloom 1.0 public API surface (FR142 + FR183 + FR190 expand)

> **Contract:** Phase 17 / **FR142** / Epic 80; Phase 22 / **FR183** / Epic 116; Phase 23 / **FR190** / Epic 123 further expand.  
> **Authority:** Correct Course 2026-09-11 (`correctCoursePhase17Approved`); Phase 22 Correct Course 2026-09-12 (`correctCoursePhase22Approved`); Phase 23 Correct Course 2026-09-14 (`correctCoursePhase23Approved`).  
> **Claim discipline:** 「1.0 / 公开 API 稳定」requires FR141–146 closed (**FR147**). This file alone ≠ 1.0 shipped.  
> **FR183:** Explicit additive expand of in-surface (NFR14 S1–S3). **≠** silent expand. See [`docs/fr183-explicit-fr142-api-expand.md`](fr183-explicit-fr142-api-expand.md).  
> **FR190:** Further additive expand beyond FR183 (FIRRTL text emit/import + documented `check_*` family). **≠** FR183 alone. See [`docs/fr190-further-fr142-api-expand.md`](fr190-further-fr142-api-expand.md).  
> **NFR63 / NFR91:** Listing a surface does **not** clear deferred deepen outside each NFR14 subset.

## Purpose

Pin the **in-surface** vs **out-of-promise / out-of-surface** partition for SemVer **1.0** major stability. Breaking changes **inside** in-surface require a **major** bump once 1.0.0 is published (see [`docs/semver-1-0-policy.md`](semver-1-0-policy.md) / FR143). **Additive** in-surface expands require an explicit doc + story update (**FR183** / **FR190**) and are **minor**-class under FR143.

## Design crate dependency boundary (AD-6)

- Design crates depend **only** on **`bitloom-prelude`**.
- Design crates must **not** depend on `bitloom` (CLI), `bitloom-sim`, `bitloom-hir`, `bitloom-builder`, `bitloom-vlog`, `bitloom-macro` (direct), `bitloom-firrtl`, or `bitloom-lsp`.
- Macros and attributes are consumed via the prelude `rhdl` facade / re-exports.
- **FR183 honesty:** Promoting documented `bitloom-firrtl` interop into in-surface does **not** authorize design crates to depend on `bitloom-firrtl`. That crate remains a **maintainer / toolchain interop** surface.

## In-surface (1.0 SemVer promise)

### `bitloom` (CLI / crates.io package)

Documented `cargo bitloom` / `cargo-bitloom` subcommands (names may gain aliases; removals/renames of documented verbs are breaking once 1.0):

| Verb | Role |
| --- | --- |
| `build` | Elaborate → Verilog |
| `new` | Scaffold design crate (prelude-only) |
| `firtool` (`ensure` / `info`) | Pinned firtool (AD-9) |
| `sim-engines` | List tick engines |
| `hls` | Product HLS paths |
| `import` | FIRRTL / Chisel import path |
| `gen-func` | Functional-sim crate emit |
| `gen-cycle` | Cycle-accurate emit path |
| `gen-tlm` / `gen-tlm-at` | SystemC TLM emit |
| `visualize` | Hierarchy / timing viz |
| `doc` | Doc emit helper |
| `wave` | Waveform artifacts |
| `coverage` | Coverage artifacts (FR114 LCOV + in-tree GUI; optional `--genhtml` FR158 third-party path) |

Library modules under `bitloom` that are **not** documented as CLI surface are **not** automatically in-surface.

### `bitloom-prelude`

Primary design-facing path:

- Re-exports used by designs: elaboratable / HIR handles as documented in crate docs (`FrozenHir`, ports, diagnostics via prelude).
- `bitloom_prelude::rhdl` attribute facade (`module`, `top`, `sequential`, `combinational`, `process`, `hls`, multi-view attrs).
- `bitloom_prelude::ip` first-class IP surface as documented.
- `Bundle` derive (via prelude) and related documented traits / helpers in crate docs.

Undocumented `pub` items are **not** promised.

### `bitloom-macro`

Documented proc-macro attributes and derives **as reached through `bitloom-prelude`** (design crates do not depend on this crate directly). Direct dependency on `bitloom-macro` is **out of the design-crate contract** even if the crate remains publishable.

### `bitloom-sim` (Q1 — IN)

Public simulation / dual-model API used by maintainers and product paths:

- Cycle-accurate `tick` / engine selection surface.
- VCD / waveform-related public helpers documented in crate docs.
- Dual-model / functional-equiv / IP dual-model public types used by product docs (e.g. `UartTxFunctional`, `GpioFunctional`, scoreboard helpers as documented).
- Coverage report helpers that are part of the documented product path.

Internal modules and undocumented `pub` items are **not** promised.

### `bitloom-firrtl` — FR183 / v1.x expand (maintainer / toolchain interop)

**Promoted (additive · FR183):** the following **documented** interop entries are **in-surface** for SemVer promise on the `bitloom-firrtl` crate:

| Entry | Role |
| --- | --- |
| `emit_chisel` | FrozenHir → compilable Chisel Scala (FR28 mechanical face) |
| `emit_chisel_idiomatic` / documented idiomatic·style·ecosystem variants already productized (`emit_chisel_idiomatic_fr111`, `emit_chisel_idiomatic_fr122`, `emit_chisel_style_guide_fr130`, `emit_chisel_style_guide_fr165`, `emit_chisel_ecosystem_fr176`, `emit_chisel_style_linter_fr181`, `emit_chisel_style_guide_pack_fr188`) | Documented maintainable / Style / deepen faces |
| `CHISEL_TARGET` / `FIRTOOL_TARGET` | Documented AD-9 pin constants |
| `BitloomFirrtlParser.parse` | Product-equivalent Parser path (FR138; `just parser-restore-check`) |
| `BitloomFirrtlParser.parseUpdateMainline` | Update-mainline Parser path (FR170; FIRRTL 6.0.0) |

**Not** in this expand: undocumented `pub` items; whole-crate internal modules; design-crate dependency on `bitloom-firrtl` (forbidden — **AD-6**).

**SemVer honesty (FR143):** this expand is **additive**. The next crates.io publish of `bitloom-firrtl` that cuts after this surface revision is a **minor** bump (e.g. 1.0.0 → 1.1.0), not a silent major and not “already expanded without a doc/story”. Tree may remain at 1.0.0 until that publish; surface honesty is this file + [`docs/fr183-explicit-fr142-api-expand.md`](fr183-explicit-fr142-api-expand.md).

### `bitloom-firrtl` — FR190 / v1.x expand (beyond FR183)

**Promoted (additive · FR190):** the following **documented** entries are **also in-surface** (superset of FR183):

| Entry | Role |
| --- | --- |
| `emit` | FrozenHir → FIRRTL 6.0.0 text (AD-3) |
| `import` | FIRRTL text → FrozenHir |
| `ports_roundtrip_ok` / `instance_graph_roundtrip_ok` | Documented interop roundtrip predicates |
| `check_idiomatic_chisel` / `check_idiomatic_chisel_fr111` / `check_idiomatic_chisel_fr122` | Documented acceptance checks paired with idiomatic emit faces |
| `check_chisel_style_guide_fr130` / `check_chisel_style_guide_fr165` | Style Guide check faces |
| `check_chisel_ecosystem_fr176` / `check_chisel_style_linter_fr181` / `check_chisel_style_guide_pack_fr188` | Ecosystem / deepen / pack check faces |

**Not** in this expand: undocumented `pub`; design-crate dependency on `bitloom-firrtl` (**AD-6**); promoting `bitloom-lsp` / `bitloom-hir` / `bitloom-builder` / `bitloom-vlog` to 1.0-stable.

**SemVer honesty (FR143):** FR190 is **additive** beyond FR183 → **minor** on the next `bitloom-firrtl` publish that cuts after this revision. See [`docs/fr190-further-fr142-api-expand.md`](fr190-further-fr142-api-expand.md). Workspace **1.1.0** (2026-09-14) is that minor cut — see [`docs/bitloom-1-1-0-release.md`](bitloom-1-1-0-release.md).

## Out-of-promise (may publish; not 1.0-stable) — Q2

| Crate | Note |
| --- | --- |
| `bitloom-hir` | May stay on crates.io; **no** 1.0 SemVer stability promise |
| `bitloom-builder` | Same |
| `bitloom-vlog` | Same |

Breaking changes in these crates do **not** by themselves require a Bitloom **1.0** major, but must not silently break the **prelude** / **sim** / **FR183/FR190 firrtl interop** in-surface contracts.

## Out-of-surface (not in 1.0 promise)

| Item | Note |
| --- | --- |
| `bitloom-lsp` / LSP | Product exists; not part of 1.0 SemVer surface promise |
| `bitloom-firrtl` **undocumented** `pub` / non-listed modules | Publishable crate; only the FR183 + FR190 tables above are in-surface — **no** silent promotion |
| remaining `rhdl-*` (`rhdl-formal`, …) | Stay unpublished internal names |
| Undocumented internal `pub` APIs | Forbidden from silent promotion to in-surface |
| NFR59 / NFR91 deferred deepen | Remains deferred; 1.0 ≠ clear leftovers |

## Blocking hygiene candidates (FR145 / Epic 82)

At FR142 lock time, **no blocking breaking items** are listed against the in-surface partition above. Epic 82 may **skip** FR145 with documentation if semver baseline agrees (Q3).

## Change process

- Expanding in-surface requires an explicit doc + story update (do not silent-expand). **FR183** (Phase 22) and **FR190** (Phase 23) are the contracts for the firrtl interop promotes above.
- After 1.0.0, in-surface breaking → major (FR143); additive documented expands → minor.
- Brand: **Bitloom** / `bitloom` / `bitloom-*`; never publish `rhdl` / `rhdl-bits` as the product name.


### `bitloom-prelude` — FR194 / Story126.2 显式追加

本故事把下列设计入口显式纳入已文档化表面，遵循 FR142/FR143 的追加流程：

| 符号 | 契约 |
|---|---|
| `ElaborateSession::define_module` | 在同一session中按显式名字、规范参数和完整定义复用模块；非捕获定义函数，错误通过Diagnostics返回并保留 |
| `ip::Gpio::define_module` | 注册可实例化GPIO定义，与原有Gpio::elaborate共享逻辑体 |

完整签名、参数与命名约束、可编译设计例及支持边界见[模块组合](ip/module-composition.md)。不扩展设计crate依赖边界；不承诺原生层级仿真或Verilog实例参数覆盖。新增API按SemVer minor策略发布，本次不修改现有版本号；代码合入不代表crates.io已提供这些接口。

### `bitloom-prelude` — FR195 / Story126.3 显式追加

| 符号 | 契约 |
|---|---|
| `ip::RvRegSlice<const WIDTH: u32 = 32>` | 两槽注册切片，WIDTH 1..64；寄存状态输出、同步 reset > flush > 握手 |
| `ip::RvRegSlice::define_module` | `(&mut ElaborateSession, impl Into<String>) -> Result<String, Diagnostics>`；完整 WIDTH 身份，与独立 elaborate 共用模块体 |
| `Elaboratable for ip::RvRegSlice<WIDTH>` | `elaborate() -> Result<FrozenHir, Diagnostics>`；独立 session，仅 finish 一次 |

此处显式纳入 FR142 文档表面；使用与边界见[注册切片](ip/rv-reg-slice.md)。新增API按SemVer minor处理；未更改版本号或发布，不能声称crates.io已包含。此追加不包含ParamSyncFifo，也不承诺native层级仿真。

### `bitloom-prelude` — FR195 / Story126.4 显式追加

| 符号 | 契约 |
|---|---|
| `ip::ParamSyncFifo<const WIDTH: u32 = 32, const DEPTH: u32 = 4>` | 寄存器 FIFO，WIDTH 1..64、DEPTH 1..16；同步 reset > flush > 握手，无空直通，满时不接收 |
| `ip::ParamSyncFifo::define_module` | `(&mut ElaborateSession, impl Into<String>) -> Result<String, Diagnostics>`；完整 WIDTH/DEPTH 身份，与独立入口共用唯一模块体 |
| `Elaboratable for ip::ParamSyncFifo<WIDTH, DEPTH>` | `elaborate() -> Result<FrozenHir, Diagnostics>`；独立 session，仅 finish 一次 |

此处逐符号显式纳入 FR142 文档表面，未列出的内部接口不自动扩大承诺。中文契约与完整 prelude-only 示例见[参数 FIFO](ip/param-sync-fifo.md)。追加属 SemVer minor；本故事不改包版本、不发布，代码合入不代表 crates.io 已包含。保留旧 SyncFifo 行为及设计依赖边界，不承诺 native/generated 层级仿真、异步 FIFO 或 BRAM 推断。

### `bitloom-prelude` — FR196 / Story127.2 显式追加

| 符号 | 契约 |
|---|---|
| `ip::CsrAccess::{Rw, Ro, Wo, W1c}` | 寄存器与全部字段一致的访问枚举；Clone/Copy/Debug/PartialEq/Eq |
| `ip::CsrOwner::{Leaf, External, None}` | 唯一状态owner枚举；Clone/Copy/Debug/PartialEq/Eq |
| `ip::CsrField` | 公开字段 `name:String`, `mask:u64`, `reset:u32`, `access:CsrAccess`；Clone/Debug/PartialEq/Eq |
| `ip::CsrRegister` | 公开字段 `name:String`, `offset:u32`, `reset:u32`, `access:CsrAccess`, `owner:CsrOwner`, `event:Option<String>`, `read_reject:bool`, `write_reject:bool`, `fields:Vec<CsrField>`；Clone/Debug/PartialEq/Eq |
| `ip::CsrBlock` | 公开字段 `name:String`, `registers:Vec<CsrRegister>`；Clone/Debug/PartialEq/Eq |
| `ip::CsrBlock::validate` | `(&self) -> Result<(), Diagnostics>`；所有输出共同预校验 |
| `ip::CsrBlock::define_module` | `(&self, &mut ElaborateSession, impl Into<String>) -> Result<String, Diagnostics>`；完整规范参数身份、共享无捕获定义体 |
| `ip::CsrBlock::elaborate` | `(&self, impl Into<String>) -> Result<FrozenHir, Diagnostics>`；独立session，一次finish |
| `ip::CsrBlock::emit_markdown` | `(&self) -> Result<String, Diagnostics>`；确定的local地址/字段表 |
| `ip::CsrBlock::emit_c_header` | `(&self) -> Result<String, Diagnostics>`；确定的namespace/include guard/local offset宏 |

此处明确纳入FR142文档表面，端口/时序/合法配置与prelude-only例见[CSR](ip/csr.md)。这是SemVer minor追加；不改版本、不发布，不宣称crates.io已包含。私有codec/RTL模块不入表面；设计依赖仍仅prelude。FR196整体/M2尚未关闭，桥、四窗译码和外设后续独立验收；FR189 deferred和NFR91保留。Bitloom与samitbasu/rhdl无关。

### `bitloom-prelude` — FR196 / Story127.3 显式追加

| 符号 | 契约 |
|---|---|
| `ip::AxiLiteCsrBridge` | 固定addr16/data32/WSTRB4、同步reset的AXI4-Lite→CSR桥；AW/W/AR各一捕获槽，单CSR执行owner，B/R独立保持 |
| `ip::AxiLiteCsrBridge::define_module` | `(&mut ElaborateSession, impl Into<String>) -> Result<String, Diagnostics>`；共享无捕获定义体，完整HIR身份与既有helper诊断 |
| `Elaboratable for ip::AxiLiteCsrBridge` | `elaborate() -> Result<FrozenHir, Diagnostics>`；独立入口电路/模块名为`AxiLiteCsrBridge`，finish一次 |

固定端口、时序和prelude-only组合例见[桥接文档](ip/axi-lite-csr-bridge.md)。上述符号显式纳入FR142，未列出的私有状态或辅助方法不自动进入表面。追加属SemVer minor；不改版本、不发布，不宣称crates.io已有。旧Axi4LiteSlave接口保持，四窗译码/M2仍待127.4；设计依赖与native/generated层级限制不变。

### `bitloom-prelude` — FR196 / Story127.4 显式追加

| 符号 | 契约 |
|---|---|
| `ip::CsrDecoder` | 固定52端口、四个256字节窗口，addr16/data32/WSTRB4，同域同步reset、单请求owner与持久DECERR |
| `ip::CsrDecoder::define_module` | `(&mut ElaborateSession, impl Into<String>) -> Result<String, Diagnostics>`；共享无捕获定义体，完整定义复用与既有helper错误语义 |
| `Elaboratable for ip::CsrDecoder` | `elaborate() -> Result<FrozenHir, Diagnostics>`；独立电路/模块名`CsrDecoder`，finish一次 |

上述逐符号显式纳入FR142，完整端口/气泡/reset/支持矩阵和prelude-only例见[四窗译码](ip/csr-decoder.md)。追加属SemVer minor；未改版本或发布，不宣称crates.io已包含。私有状态、任意map、多主与native/generated层级未扩承诺；历史段落为当时状态，M2最终状态以故事关闭记录为准。

### `bitloom-prelude` — FR197 / Story128.2 Timer显式追加

| 符号 | 契约 |
|---|---|
| `ip::Timer` | 固定32位CSR计时器，13端口，同步reset；原始match与sticky EVENT分离 |
| `ip::Timer::registers` | `() -> CsrBlock`；固定本地00/04/08/0c描述，CTRL/COUNT External、COMPARE/EVENT Leaf；返回值修改不改变Timer硬件 |
| `ip::Timer::define_module` | `(&mut ElaborateSession, impl Into<String>) -> Result<String, Diagnostics>`；同session共享CSR与Timer定义，完整HIR复用与既有诊断 |
| `Elaboratable for ip::Timer` | `elaborate() -> Result<FrozenHir, Diagnostics>`；独立电路/模块名`Timer`，finish一次 |

上述符号显式纳入FR142；私有CSR模块名/状态/helper不自动入表面。完整时序、地址、prelude-only例与适用验证见[Timer32](ip/timer.md)。追加属SemVer minor，未改版本或发布，不宣称crates.io已有；native/generated层级仍unsupported。只交付FR197 Timer子集，不关闭M3/Phase24，FR189 deferred/NFR91保持。

### `bitloom-prelude` — FR197 / Story128.3 IRQ显式追加

| 符号 | 契约 |
|---|---|
| `ip::Irq` | 固定五路同步事件，14端口；PENDING/ENABLE由CSR唯一存储，TEST无存储，RAW只硬件 |
| `ip::Irq::registers` | `() -> CsrBlock`；固定本地00/04/08/0c，五具名字段与mask31；修改返回描述不改变固定硬件 |
| `ip::Irq::define_module` | `(&mut ElaborateSession, impl Into<String>) -> Result<String, Diagnostics>`；同session共享IRQ/CSR定义，完整身份复用与既有诊断 |
| `Elaboratable for ip::Irq` | `elaborate() -> Result<FrozenHir, Diagnostics>`；独立电路/模块名`Irq`，finish一次 |

以上逐符号显式纳入FR142，私有CSR名字/helper/内部状态不自动纳入。端口/时序、prelude-only原文例与证据见[五路事件IRQ](ip/irq.md)。追加属SemVer minor；未改版本或发布，不宣称crates.io已有。设计依赖仍仅prelude，native/generated层级unsupported；仅FR197 IRQ子集，M3/Phase24及FR189 deferred/NFR91不由此关闭。

### `bitloom-prelude` — FR197 / Story128.4 GPIO显式追加

| 符号 | 契约 |
|---|---|
| `ip::GpioCsr` | 固定32位GPIO CSR，16端口；同步针脚、沿前DIR事件、唯一OUT与原始事件 |
| `ip::GpioCsr::registers` | `() -> CsrBlock`；固定local00/04/08/0c/10/14，六个bits字段全mask；修改返回值不改变硬件 |
| `ip::GpioCsr::define_module` | `(&mut ElaborateSession, impl Into<String>) -> Result<String, Diagnostics>`；同session复用定义/共享CSR，一次finish |
| `Elaboratable for ip::GpioCsr` | `elaborate() -> Result<FrozenHir, Diagnostics>`；独立电路/模块名`GpioCsr`，共用定义体 |

上述四符号逐项纳入FR142，私有CSR/helper/状态不自动纳入。端口、时序和原文例见[GPIO32 CSR](ip/gpio-csr.md)。追加属SemVer minor，未改版本或发布；旧GPIO8/FL、prelude-only依赖与native/generated层级unsupported保持。仅GPIO子集，M3/Phase24未关闭；FR189 deferred/NFR91保持。

### `bitloom-prelude` — FR197 / Story128.5 UART显式追加

| 符号 | 契约 |
|---|---|
| `ip::UartCsr` | 固定15端口、8N1、全宽DIV、双8×4 FIFO、四原始事件 |
| `ip::UartCsr::registers` | `() -> CsrBlock`；固定local00/04/08/0c/10/14，六bits字段；修改返回值不改变固定硬件 |
| `ip::UartCsr::define_module` | `(&mut ElaborateSession, impl Into<String>) -> Result<String, Diagnostics>`；同session共享CSR/FIFO定义，最终一次finish |
| `Elaboratable for ip::UartCsr` | `elaborate() -> Result<FrozenHir, Diagnostics>`；独立UartCsr电路，复用同定义体 |

以上四符号显式纳入FR142，私有状态/模块/helper不纳入。见[UART CSR](ip/uart-csr.md)。SemVer minor追加，未改版本或发布；旧UART8/VIP/FL保持，设计仅依赖prelude。native/generated层级unsupported；M3关闭以最终七步记录为准，Epic129/130、Phase24整体及FR189 deferred/NFR91不由此关闭。

### `bitloom` CLI — FR199 / Story130.2 来源重放追加

| 命令 | 契约 |
|---|---|
| `external-ip lock --manifest --lock --cache` | 显式联网解析固定来源，生成完整内容锁及许可证归档；既有锁不兼容时失败，不自动重锁 |
| `external-ip verify --manifest --lock --cache --offline` | 只读核验 manifest、来源闭包、许可证和工具身份；缺失或漂移非零失败 |
| `external-ip replay --manifest --lock --cache --compile` | 要求由隔离编排器提供禁网环境，验证后真实编译冻结 filelist；不构成 FIFO 行为验收 |

以上 CLI 追加按 FR142/FR143/SemVer minor 登记；未改包版本、未发布。来源 schema 是单一冻结试点的私有版本化格式，不承诺任意外部 IP 包管理。真实隔离入口为 `scripts/phase24-external-ip-replay.py replay --compile`，来源验收与 Story130.3 行为验收分别记录。

### `bitloom` CLI — Story130.3 外部 FIFO 绑定追加

| 命令 | 契约 |
|---|---|
| `external-ip binding --manifest --lock --cache --out` | 校验固定来源后生成包含 Bitloom 组合父模块内容/hash 的绑定产物；输出不得覆盖输入或缓存 |
| `external-ip behavior --manifest --lock --cache` | 验证工具/来源身份，再执行真实 Bitloom 组合 RTL 与独立 FIFO oracle；错误和超时非零失败 |

这是 additive CLI surface，按 FR142/FR143/SemVer minor 管理；未修改包版本或发布，不声称 crates.io 已包含。未新增 prelude/hir/builder/vlog 公共符号，私有 pilot schema 不自动提升为通用外部包管理 API。固定参数、来源层与原生 unsupported 边界见 [外部 FIFO 试点](ip/external-fifo-pilot.md)。支持状态仍以 Story130.3 真实验收为准。
