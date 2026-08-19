---
title: RHDL 阶段二 — later-product 升格为正式需求
status: final
created: 2026-08-19
updated: 2026-08-19
---

# PRD: RHDL 阶段二（later-product → FR）

*Working title — 确认：本 PRD 覆盖原 `later-product.md` 全部条目的正式升格，并补齐阶段一缺口 FR21 / NFR3；阶段一 CAP-1…CAP-9 / FR1–FR20 仍有效。*

## 0. Document Purpose

本 PRD 面向 PM、架构与 epic 拆解：把 `later-product.md` 中条目**正式升格为带稳定 ID 的 FR/NFR**。

**权威边界（Finalize 决议）：**
- **阶段一**产品合同：`_agile-output/specs/spec-rhdl/SPEC.md`（CAP-1…CAP-9）+ companions；`epics.md` 中 FR1–FR20 为阶段一拆解。
- **阶段二及以后**产品范围：**本 PRD 为唯一升格后的需求源**（FR21–FR40、NFR3、NFR10–NFR13）。`later-product.md` 降为**索引**，不再承载无 ID 需求。
- **FR21 / NFR3**：阶段一已声明但未交付的缺口；本 PRD 将其列为 **P0 交付所有权**（不另立第二真相），直至合入后仍保留 ID。

随时可用 `bmad-party-mode` / `bmad-advanced-elicitation` 深挖某一节。

## Glossary

| 术语 | 含义 |
|------|------|
| FrozenHir | 冻结后的层次电路 IR；后端与 `tick` 的只读输入 |
| elaborate | 运行时展开设计得到 FrozenHir |
| emit | 从 FrozenHir 写出 Artifact（如 `.v` / `.fir`） |
| tick | 模块 Clock 的一个上升沿步进 |
| PortValues | tick / 功能模型共用的端口取值 |
| CAP-n | 阶段一 SPEC 能力编号 |
| FR-n / NFR-n | 可验收功能/非功能需求 ID（阶段一见 epics；阶段二见本 PRD） |

## 1. Vision

RHDL 阶段一把「合法 Rust → FrozenHir → Yosys 友好 `.v` + `tick`/VCD + FIRRTL 文本互转」立住了。阶段二要在**不偏离生成器阐述、不自研 HLS 调度、不占用 crates.io `rhdl`/`rhdl-bits`** 的前提下，让工具从「骨架」变成可写真实小设计的语言，并按证据排序引入 Mem、多时钟/CDC、可选 FST，以及可选的外挂 HLS。身份上，公开文档必须说清本仓库与 `samitbasu/rhdl` 无关，发布名为 `rhdl-rs`。

成功时，设计者不必离开 Cargo：能写更完整的 comb/seq，能钉死可离线的 firtool，能按里程碑打开 Mem 与 CDC，而不是把这些永远堆在 later-product 清单里无法验收。

## 2. Target User

### 2.1 Jobs To Be Done

- **功能：** 用 Rust 写可综合 RTL，尽早失败，接上现有 Verilog/FIRRTL 流程。
- **情境：** 嵌入式/FPGA 小团队或个人，本机 CLI，无云控。
- **情感：** 信任工具不会静默截断位宽、不会 silently 变成 latch、不会把我当成另一个叫 rhdl 的项目。
- **社会：** 能把模块级 IP 以 `.fir` 文本与 Chisel 生态交换，而不被迫写可维护 Scala。

### 2.2 Non-Users（本阶段明确不做的用户）

- 需要完整 IDE/可视化优先于语言正确性的用户（FR38 在 P2，不挡主路径）。
- 需要自研 HLS 调度器或把 RHDL 当高位综合主产品的用户（FR35 仅外挂）。
- 需要以 crates.io 名 `rhdl` 发布的用户（禁止）。

### 2.3 Key User Journeys

*[ASSUMPTION] 工具链 PRD 以能力为主；旅程缩到三条可验收场景。*

- **UJ-1. Alex 加厚表面后写出可测 FIFO 形模块。**
  - **Persona：** 熟悉 Rust 的 FPGA 工程师，已有阶段一计数器。
  - **Entry：** 单时钟设计 crate，仅依赖 `rhdl-prelude`。
  - **Path：** 写 comb/seq + 同位宽运算 → `cargo test` tick → `cargo rhdl build` 出 `.v`。
  - **Climax：** 黄金向量通过且 Yosys 可读 `.v`。
  - **Resolution：** 准备加 Mem（FR26），仍不碰多时钟。

- **UJ-2. Blair 在本机离线复现 firtool 钉死构建。**
  - **Persona：** CI 维护者，linux-x64 runner。
  - **Entry：** 干净缓存目录。
  - **Path：** 首次 `cargo rhdl` 相关 firtool 路径下载并校验 → 断网第二次命中缓存 → 设 `RHDL_FIRTOOL_PATH` 覆盖。
  - **Climax：** checksum 失败则拒绝继续；成功则可调用钉死版本。
  - **Edge：** checksum 失败或缓存损坏则拒绝继续（NFR3）；多平台扩展见 NFR11（P2）。

- **UJ-3. Casey 打开双时钟 SyncFIFO 并被错误的跨域拦住。**
  - **Persona：** 有过 CDC bug 的 ASIC 设计者。
  - **Entry：** 两时钟端口的模块。
  - **Path：** 非法跨域赋值 → freeze 失败带 `rhdl::E0xxx`；改用语言级 SyncFIFO/DoubleFlop → elaborate/emit/tick。
  - **Climax：** 错误在网表前，不在硅上。

## 3. Success Metrics

| ID | 指标 | 目标 | 反指标 |
|----|------|------|--------|
| SM-1 | P0（FR21,NFR3,FR22,NFR10,NFR12）可演示 | 全部有自动化验收 | 「文档写了但 CLI 仍信任 PATH firtool」 |
| SM-2 | P1（FR26→FR23/24/25）按序交付 | Mem 先于多时钟合并 | 多时钟合并时仍无 Mem 语义锚 |
| SM-3 | later-product 无「无 ID 保留项」 | 每条映射到 FR/NFR 或明确拒绝进产品 | 重新出现无 ID 的 later 清单作为唯一真相 |
| SM-4 | 身份清晰 | README 可见免责与 `rhdl-rs` | crates.io/文档暗示名 `rhdl` |
| SM-5 | FR22 语言条 | 文档化构造清单中的项均可 elaborate/emit/tick；未列入清单的 `language-surface.md` Deferred 类型不得 silently 可用 | 仅靠阶段一骨架通过「两个 fixture」冒充 FR22 |

## 4. Scope & Phasing

**In scope：** 下文全部 FR22–FR40 与 NFR3/NFR10–NFR13，以及阶段一缺口 **FR21**。  
**Out of scope（仍禁止）：** 自研 HLS 调度器；以 Chisel Scala 为互转契约；HIR→TLM；发布 `rhdl`/`rhdl-bits`；云控。

**交付阶段（验收顺序，不是「非 FR」）：**

| Phase | 内容 | 依据 |
|-------|------|------|
| **P0** | FR21, NFR3, FR22, NFR10, NFR12 | 调研 R1–R3；关闭阶段一缺口 |
| **P1** | FR26, 然后 FR23, FR24, FR25 | Mem → 多时钟/复位/门控 |
| **P2a** | FR31, FR32, FR29, FR33, FR34 | 仿真加厚（FST 可选、双引擎、bridge、C ABI、覆盖率） |
| **P2b** | FR35, FR28, FR39 | 外挂 HLS / 尽力 Scala / formal·SVA |
| **P2c** | FR27, FR36, FR37, FR38, FR40, NFR11, NFR13 | IO 扩展、浮点、IP、可视化/LSP、CLI 动词、多平台 firtool、MSRV |

*[ASSUMPTION] P2a→P2b→P2c 为默认建议顺序；同桶内可并行拆 epic。不提升到 P0/P1 除非改本 PRD。*

**与 SPEC 关系：** 已 Finalize：`later-product.md` 为索引；`SPEC.md` Non-goals 仅约束阶段一，并指向本 PRD。

## 5. Features & Functional Requirements

### 5.1 身份与阶段一缺口

**F-Identity**

- **FR21 — 公开 README 免责与发布名**  
  - **intent：** 公开首页声明与 `samitbasu/rhdl` 无关，发布名为 `rhdl-rs`。  
  - **success：** 仓库 README 含免责声明；写明 crates.io 名 `rhdl-rs`；禁止暗示 `rhdl` / `rhdl-bits`。  
  - **phase：** P0 · 实现 UJ 外的合规门槛。

### 5.2 语言表面加厚

**F-Surface**

- **FR22 — 单时钟语言表面加厚**  
  - **intent：** 在不上 HLS/多时钟的前提下，comb/seq/运算/控制流可写真实小 RTL。  
  - **success：**  
    1. **构造条（必须）：** 文档化清单中的项均可经 prelude/builder elaborate → emit `.v` → `tick`：`if`/`match`（或等价分支）、严格同位宽二元运算与连接、显式 pad/trunc、同步复位 `Reg` 的复位赋值语义、组合完整赋值检查（延续 FR3）。  
    2. **集成 fixture：** 计数器 + 单时钟 FIFO 形示例均对齐黄金值（不得仅靠阶段一骨架冒充）。  
    3. **明确非目标（本 FR）：** `Bundle`、`Vec<T,N>` 仍 defer，除非另开 FR；不得 silently 可用。[NON-GOAL for this FR]  
  - **phase：** P0 · UJ-1 · SM-5。

### 5.3 时钟、复位与存储（原 Clocking and memory）

**F-Clock-Mem**

- **FR23 — 多时钟 HIR 与语言级 CDC**  
  - **intent：** 多时钟 HIR；**Clash 式 phantom 域**（AD-22）；DoubleFlop / SyncFIFO 为语言级 CDC。  
  - **success：** 非法跨域在 freeze 失败；双时钟 DoubleFlop 或 SyncFIFO fixture 可 elaborate/emit/按域 tick。机制已定：**Clash 式 phantom 域**（架构脊柱 **AD-22**，2026-08-19）。
  - **phase：** P1（在 FR26 之后交付）。

- **FR24 — 异步复位**  
  - **intent：** 时序包络支持异步复位（相对阶段一同频高有效同步复位）。  
  - **success：** async-reset fixture 发出边沿敏感复位 Verilog，仿真对齐置位/释放黄金值。  
  - **phase：** P1。

- **FR25 — 时钟门控与使能**  
  - **intent：** 寄存器/时序块可带 clock enable 或门控。  
  - **success：** enable=1 时行为对齐无门控黄金值；emit 为可综合 enable/门控形。  
  - **phase：** P1。

- **FR26 — 同步读 Mem（CHIRRTL 友好表面）**  
  - **intent：** 语言表面暴露 **CHIRRTL 友好名**（对齐 Chisel：`Mem` ≈ comb/async-read、`SyncReadMem` ≈ sync-read；文档可对应 `cmem`/`smem` 语义）；降级与互转锚定 FIRRTL 规范 `mem`。  
  - **success：** 用户用友好名编写的 SyncReadMem（及文档化的 Mem）fixture → HIR → `.v`/`firrtl.mem`；tick 满足读延迟黄金值。双口跨时钟 Mem **仅**允许经命名 CDC FIFO（与 FR23 衔接），不得以裸双时钟 `mem` 作为默认可综合路径。  
  - **phase：** P1（先于 FR23）。

- **FR27 — 顶层 Analog / InOut / 三态**  
  - **intent：** 仅顶层 IO 允许 Analog/InOut/三态。  
  - **success：** 顶层可 emit；非顶层同构构造被 `rhdl::E0xxx` 拒绝。  
  - **phase：** P2。

### 5.4 互操作扩展

**F-Interop**

- **FR28 — FIRRTL→Chisel 尽力生成器**  
  - **intent：** 可选工具从 FIRRTL 生成 Chisel Scala；**不是**互转契约。  
  - **success：** fixture `.fir` 产出可在文档钉死的 Chisel 版本编译的 Scala，或结构化尽力失败；FrozenHir↔FIRRTL 6 文本仍为互转合同。  
  - **phase：** P2。

### 5.5 仿真扩展

**F-Sim**

- **FR29 — 手写 bridge / abstraction / mixed both**  
  - **intent：** 支持 `#[bridge]`、`#[abstraction]`、混合 `both` 仿真视图。  
  - **success：** 混合 fixture 按文档视图跑通；`PortValues` 对照；不从 HIR 生成 TLM。  
  - **phase：** P2。

- **FR30 — 双视图形式等价**  
  - **intent：** 功能视图与周期精确视图可做形式等价检查。  
  - **success：** fixture 输出 pass/fail；故意不一致则 fail。  
  - **phase：** P2。

- **FR31 — 可选 FST 波形**  
  - **intent：** FST 为可选项；VCD 仍为 CAP-5 默认。  
  - **success：** 开关写出可被 GTKWave 或 Surfer 打开的 FST（允许经 Verilator `--trace-fst` 或文档化的 vcd2fst 路径；不要求自研 FST writer）；关闭时仍写 VCD。  
  - **phase：** P2a。

- **FR32 — 解释器与编译 tick 引擎**  
  - **intent：** 同一 FrozenHir 可用解释或编译引擎 tick。  
  - **success：** 同一 suite 两引擎 `PortValues` 一致；文档 API/CLI 可选引擎。  
  - **phase：** P2。

- **FR33 — C ABI / cdylib 仿真**  
  - **intent：** 功能与周期精确仿真可经 C ABI cdylib 消费。  
  - **success：** C harness 加载 cdylib，tick 两视图，对齐 Rust 黄金值。  
  - **phase：** P2。

- **FR34 — 仿真覆盖率**  
  - **intent：** 仿真可报告覆盖。  
  - **success：** fixture 跑后报告至少一 hit 与一 miss（分支或翻转），格式稳定。  
  - **phase：** P2。

### 5.6 生态扩展

**F-Ecosystem**

- **FR35 — 可选 HLS 前端（无自研调度）**  
  - **intent：** 树内 `#[hls]` 可发射 IR/C 并调用 Bambu/XLS；**不**实现自研调度器。  
  - **success：** 当启用 HLS 路径时，`#[hls]` 至少对 **一个** 文档钉死的后端（Bambu **或** XLS，须写明）产生可复现产物；未启用时 CLI/文档标明 unsupported。无 rhdl crate 实现 scheduling。  
  - **phase：** P2b。

- **FR36 — 可综合浮点 `rhdl-float`**  
  - **intent：** 提供可综合浮点类型产品 crate。  
  - **success：** fixture elaborate/emit；tick 对齐文档化舍入用例。  
  - **phase：** P2。

- **FR37 — IP 产品箱与黑盒封装**  
  - **intent：** UART/SPI/I2C/FIFO/AXI 等 IP 与外部 IP 黑盒封装。  
  - **success：** 至少一个树内 IP + 一个黑盒 wrapper 可 elaborate/emit，外部实例保持不透明。  
  - **phase：** P2。

- **FR38 — HIR 可视化、LSP 与 HTML 文档**  
  - **intent：** FrozenHir 驱动可视化与 HTML 文档；LSP 为后续增强。  
  - **success：** 同一 fixture 产出 HTML（含模块/端口与实例层次列表）。完整 LSP hover/goto **deferred**（见 `docs/fr38-viz-lsp.md`）。  
  - **phase：** P2。

- **FR39 — 形式验证 / SVA 导出**  
  - **intent：** 可向 formal/SVA 流程导出。  
  - **success：** 含断言的 fixture 导出 SVA 或文档化 formal 输入；假断言可被检查器失败。  
  - **phase：** P2。

- **FR40 — 额外 CLI 动词**  
  - **intent：** CLI 覆盖常用工作流动词；`cargo rhdl build` 仍为生成主路径。  
  - **success：** 已交付 `build` / `firtool` / `sim-engines` / `hls`（各有 `--help` 与 smoke）。`check`/`import`/`visualize`/`wave`/`doc`/`build-sim` **deferred**（见 `docs/fr40-cli-verbs.md`）。  
  - **phase：** P2。

## 6. Non-Functional Requirements

| ID | 要求 | Phase |
|----|------|-------|
| **NFR3** | linux-x64：下载钉死 `firtool-1.155.0` 的 `firrtl-bin-linux-x64.tar.gz`，校验 sibling `.sha256`，缓存可离线，支持 `RHDL_FIRTOOL_PATH`；默认不信任 PATH firtool。 | P0 |
| **NFR10** | HIR→RHDL 源码再生仅调试；非产品互转格式；发行测试不宣称源码往返稳定。 | P0 |
| **NFR11** | firtool 资产覆盖 macos / windows / linux-aarch64；**同一套**下载/`.sha256`/缓存/`RHDL_FIRTOOL_PATH` 机制（扩展 NFR3 的平台三元组）；不支持的平台给出明确错误。 | P2c |
| **NFR12** | 默认钉 1.155.0，直至有记录的 Chisel 配对再升到 1.156.0；升钉须更新校验表。 | P0 |
| **NFR13** | MSRV 为 rustc **1.97.1**；workspace/`rust-version`/CI/文档一致（原「升至 1.98.0」因上游未发布而修订）。 | P2 |

继承阶段一约束：合法 Rust eDSL；显式 comb/seq；同位宽；无云控；互转契约 FrozenHir↔FIRRTL 6 文本；禁止自研 HLS 调度；禁止发布 `rhdl`/`rhdl-bits`。

## 7. Assumptions（索引）

- [ASSUMPTION] FR 编号自 FR22 起连续；FR21/NFR3 复用阶段一编号以关闭缺口。
- [ASSUMPTION] 「升格为 FR」含分阶段交付；P2 项仍是正式需求，不是 later-product 垃圾桶。
- ~~[ASSUMPTION] CDC 机制待 architecture~~ — **已关闭：** 见 AD-22 / Open Q1。
- [ASSUMPTION] 工具链旅程 UJ-1…3 足够 launch 文档；不设独立 UX 规格。
- [ASSUMPTION] Finalize 后 `later-product.md` 改为指向本 PRD 的索引，不再作为唯一需求源。

## 8. Open Questions

1. ~~FR23 的 CDC 机制选型~~ — **已关闭（2026-08-19）：** 由架构脊柱 **AD-22** 采纳 Clash 式 phantom 域；合法跨越仅语言级 `DoubleFlop` / `SyncFIFO`。若改选型须修订 AD-22，不得 silent 分叉。
2. ~~Mem 用户 API 是否暴露 CHIRRTL 友好名~~ — **已关闭（2026-08-19）：** **暴露** CHIRRTL 友好名（`Mem` / `SyncReadMem` 及文档化的 `cmem`/`smem` 语义对应）；FIRRTL 文本互转与降级仍锚定规范 `mem`（AD-21）。
3. ~~FST 自研 writer~~ — **已决议（Finalize）：** 不要求自研；Verilator/vcd2fst 即可（见 FR31）。
4. ~~权威边界~~ — **已决议（Finalize）：** 阶段一=SPEC；阶段二+=本 PRD；`later-product.md`=索引。

## 9. Traceability（later-product → FR）

| later-product 条目 | FR/NFR |
|--------------------|--------|
| Multi-clock HIR; CDC phantom; DoubleFlop/SyncFIFO | FR23 |
| Async reset; clock gating/enables | FR24, FR25 |
| Mem / sync-read vs cmem/smem | FR26 |
| Analog, InOut, tri-state | FR27 |
| FIRRTL→Chisel Scala generator | FR28 |
| HIR→RHDL regen (debug-only) | NFR10 |
| bridge/abstraction/both | FR29 |
| Dual-view formal equivalence | FR30 |
| FST waveforms | FR31 |
| Interpreter vs compiled tick | FR32 |
| C ABI / cdylib | FR33 |
| Coverage | FR34 |
| HLS `#[hls]` + Bambu/XLS | FR35 |
| rhdl-float | FR36 |
| IP crates + black-box | FR37 |
| Visualization, LSP, HTML | FR38 |
| Formal / SVA export | FR39 |
| CLI verbs build/firtool/sim-engines/hls | FR40 |
| firtool macos/windows/linux-aarch64 | NFR11 |
| firtool-1.156.0 until Chisel pairs | NFR12 |
| rustc 1.97.1 MSRV | NFR13 |
| （调研缺口）README / rhdl-rs | FR21 |
| （调研缺口）linux-x64 firtool pin | NFR3 |
| （调研加厚）单时钟表面 | FR22 |
| `language-surface.md` Deferred：`Bundle` / `Vec` | **非本 PRD FR**（FR22 NON-GOAL）；升格需新变更 |
| `Polarity` / `ResetKind` | 随 FR23/FR24 架构选型一并定；未单列 FR |
