---
title: Bitloom 阶段二 PRD — 2026-08-21 概述字面升格修订
status: final
created: 2026-08-19
updated: 2026-08-21
amendment: overview-literal-C-2026-08-21; fr71-jvm-ci-2026-08-21
---

# PRD: Bitloom 阶段二及愿景闭环（later-product → FR + 概述字面升格）

*Working title — 2026-08-21 Update（①C / 只改 PRD）：在保留 FR21–FR40 / NFR3 / NFR10–NFR13 稳定 ID 的前提下，将 `docs/requirements/1. 项目概述.md` §1.3.7–11 与 §1.5 亮点升格为**可验收硬需求**；**推翻**本 PRD 原「不以 Chisel Scala 为互转契约 / 禁止 HIR→TLM」等 non-goal。概述 `.md` 本轮不改。*

*2026-08-21 追加（`fr71-jvm-ci`）：为 FR28「必须编译通过」补默认 CI JVM 真编译门禁 **FR71** + 工具链合同 **NFR34**（与 `epics.md` Phase 8 / Epic 25 对齐）。*

## 0. Document Purpose

本 PRD 面向 PM、架构与 epic 拆解。

**权威边界：**
- **阶段一**产品合同：`_agile-output/specs/spec-rhdl/SPEC.md`（CAP-1…CAP-9）+ companions；`epics.md` 中 FR1–FR20。
- **阶段二基线（仍有效）：** FR21–FR40、NFR3、NFR10–NFR13（本文件原 Finalize 正文；下文有修订处以其新 success 为准）。
- **2026-08-21 升格（本修订）：** FR46–FR52 将概述愿景中尚未被「弱定义」覆盖的部分升为硬 FR；并对 FR28 / FR29 / FR30 / FR35 / FR37 / FR38 的 **success 条**做字面加强。
- **2026-08-21 CI 证据补强：** **FR71** / **NFR34** — 默认 CI 强制 Chisel JVM 真编译（禁止 skip=0）；编号避开历史 Phase 3–5 撞号（见 addendum）。
- **身份 supersession：** 公开产品名 **Bitloom**，crates.io / CLI **`bitloom`**（阶段三 FR41）；禁止发布 `rhdl` / `rhdl-bits`。正文不再以 `rhdl-rs` 为发布名。[ASSUMPTION] 概述仍写 RHDL；合同以 Bitloom 为准，概述另开任务对齐。
- `later-product.md` 仍为索引，不承载无 ID 需求。

**显式推翻（须记入架构/epics，不得 silent 保留旧 non-goal）：**

| 原决议 | 本修订 |
|--------|--------|
| 不以可维护 Chisel Scala 为互转契约；FR28 仅尽力 | **FR28 修订 + FR46：** 双向可维护源码互转列为合同 |
| 禁止从 HIR 生成 TLM / 功能模拟器 | **FR47：** 工具链须能生成功能模拟器与周期精确模拟器 |
| FR37 至少一个树内 IP + 黑盒即可 | **FR37 修订 + FR48：** UART/SPI/I2C/FIFO/AXI 一级产品 IP |
| FR38 HTML 层次即可；LSP deferred 可长期挂起 | **FR38 修订 + FR49：** 内置层次图 + 时序图为验收条 |
| FR35 可选外挂、未启用可标 unsupported | **FR35 修订 + FR50：** HLS 为产品路径（仍可外挂调度，但不可永久 unsupported） |
| 调研 2026-08-21「重定义 done / 勿字面全做」建议 | **用户明示 ①C 拒绝**；接受多年与上游方向冲突风险（见 addendum） |

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
| FR-n / NFR-n | 可验收功能/非功能需求 ID |
| 概述愿景 | `docs/requirements/1. 项目概述.md` 中的设计目标与亮点（本轮未改文件，但已映射进 FR） |
| 可维护 Chisel（验收） | 生成 Scala 在钉死版本下**可编译**；端口/层次往返谓词通过；允许机械风格（Open Q5 已关闭） |
| 功能模拟器（FR47） | 默认生成 **Rust** 功能模拟器 crate（非强制 SystemC） |
| 一级 IP | 官方可依赖例化的 UART/SPI/I2C/FIFO/AXI 类模块；AXI 默认 = AXI4-Lite 最小从接口 |
| ClockDomain | 时钟/复位极性/同步·异步绑定；跨域须语言级同步器（FR23/FR52） |
| FR41 | 阶段三身份升格（Bitloom）；本 PRD FR 编号自 FR40 后跳至 FR46 为刻意预留，非缺失 |

## 1. Vision

Bitloom（公开品牌；仓库历史名可含 rhdl）要把「合法 Rust → FrozenHir → Yosys 友好 `.v` + `tick`/波形 + FIRRTL」做成可写真实设计的语言，并**按概述字面**兑现：与 Chisel **双向**互操作、**内置/产品化 HLS**、**丰富一级 IP 库**、**内置可视化**、**多视图建模与双模拟器生成**。身份上须声明与 `samitbasu/rhdl` 无关，发布名 **`bitloom`**（禁止 `rhdl` / `rhdl-bits`）。

成功时，设计者不必离开 Cargo：能写完整 comb/seq 与 CDC，能钉死 firtool，能把模块以 FIRRTL **与可维护 Chisel 源**双向交换，能从算法级路径得到 RTL，能例化一级 IP，能看层次/时序可视化，能对同一模块生成功能与周期精确模拟器并做一致性验证——而不是把这些永远标成「尽力 / deferred / 非目标」。

*[ASSUMPTION] 「自研 HLS 调度器」仍可不实现：允许钉死单一外挂（Bambu 或 Vitis），但产品路径必须默认可用且有 CI 夹具——见 FR35/FR50。*

## 2. Target User

### 2.1 Jobs To Be Done

- **功能：** 用 Rust 写可综合 RTL，尽早失败，接上 Verilog/FIRRTL/**Chisel** 与 HLS/IP/可视化工作流。
- **情境：** 嵌入式/FPGA 小团队或个人，本机 CLI，无云控。
- **情感：** 信任工具不会静默截断位宽、不会 silently 变成 latch、不会冒充另一个 rhdl；愿景条目可验收而非营销幻灯片。
- **社会：** 能与 Chisel 生态**双向**复用 IP；能用一级 IP 与可视化向同事展示设计。

### 2.2 Non-Users（仍明确不做）

- 需要以 crates.io 名 `rhdl` / `rhdl-bits` 发布的用户（禁止）。
- 需要云控/远程 elaboration 的用户（禁止）。
- *[修订]* 原「不服务完整 IDE/可视化优先」与「不服务以 HLS 为主产品」——**在 ①C 下移出 Non-Users**；改由 FR38/FR49、FR35/FR50 服务。

### 2.3 Key User Journeys

*[ASSUMPTION] 保留 UJ-1…3；新增 UJ-4…6 覆盖概述字面升格。*

- **UJ-1. Alex 加厚表面后写出可测 FIFO 形模块。**（不变要旨；prelude = `bitloom-prelude`；CLI = `cargo bitloom`。）
- **UJ-2. Blair 本机离线复现 firtool 钉死构建。**（`cargo bitloom`；`BITLOOM_FIRTOOL_PATH` 或文档化的 env 别名可与历史 `RHDL_FIRTOOL_PATH` 并存。[ASSUMPTION]）
- **UJ-3. Casey 打开双时钟 SyncFIFO 并被错误跨域拦住。**
- **UJ-4. Dana 将 Bitloom 模块导出为可维护 Chisel，再导入回 Bitloom。**  
  - **Path：** elaborate → emit `.fir` → **生成可编译 Chisel Scala** → 在钉死 Chisel/firtool 版本编译；反向：Chisel/`fir` **导入**为 FrozenHir/Bitloom 模块表面 → elaborate/emit/tick。  
  - **Climax：** 往返后公开端口名/宽/向与实例图在文档谓词下相等（强于「尽力失败」）。
- **UJ-5. Ellis 用 `#[hls]` 从算法级函数得到可综合 RTL，并例化一级 UART+FIFO。**  
  - **Climax：** 默认启用路径产出 RTL；树内 IP crate 可 `use` 并 elaborate。
- **UJ-6. Flynn 对同一 UART 模块生成功能模拟器与周期精确模拟器，跑一致性检查，并打开层次图+时序图。**  
  - **Climax：** 故意不一致则 fail；可视化产物可人工打开且含层次与时序两类视图。

## 3. Success Metrics

| ID | 指标 | 目标 | 反指标 |
|----|------|------|--------|
| SM-1 | P0 可演示 | 全部有自动化验收 | 仍信任 PATH firtool |
| SM-2 | P1 按序交付 | Mem 先于多时钟 | 多时钟无 Mem 锚 |
| SM-3 | later-product 无无 ID 项 | 映射到 FR/NFR 或拒绝 | 无 ID 清单回流 |
| SM-4 | 身份清晰 | README：Bitloom / `bitloom`；与 samitbasu/rhdl 无关 | 暗示 crates.io `rhdl` 或仍写死唯一发布名 `rhdl-rs` |
| SM-5 | FR22 构造条 | 清单项均可 elaborate/emit/tick | 两 fixture 冒充表面 |
| SM-6 | 概述字面升格（新） | FR46–FR52 与修订后的 FR28/30/35/37/38 均有自动化或文档化黄金验收；FR28「必须编译」默认 CI 证据见 **FR71** | 「尽力 / deferred / 非目标」冒充概述完成；仅 Rust 谓词或 skip=0 冒充 FR28 编译 |
| SM-7 | 双模拟器（新） | UJ-6 夹具绿 | 仅手写 functional 无生成路径 |

**反指标（合同级）：** 用调研「同业未做满」作为永久免责而不改 FR——在 ①C 下**不可接受**；风险须进计划与估算，不得进 non-goal。

## 4. Scope & Phasing

**In scope：** FR21–FR40（含本修订加强条）、NFR3/NFR10–NFR13、**FR46–FR52**。  
**仍禁止：** 发布 `rhdl` / `rhdl-bits`；云控；*[ASSUMPTION]* 树内自研 HLS **调度器实现**（外挂调度允许，见 FR35/FR50）。

**已移出「禁止」、改为必须交付：** 可维护 Chisel 双向源码互操作；HIR/双视图驱动的功能模拟器生成；一级完整 IP 线；内置层次+时序可视化；产品化 HLS 路径。

| Phase | 内容 | 依据 |
|-------|------|------|
| **P0** | FR21（按 Bitloom 修订）、NFR3、FR22、NFR10、NFR12 | 基线 |
| **P1** | FR26 → FR23 → FR24 → FR25 | Mem → CDC/复位/门控 |
| **P2a** | FR31、FR32、FR29、FR33、FR34 | 仿真加厚 |
| **P2b** | FR35（加强）、FR28（加强）、FR39、**FR50** | HLS + Chisel 生成起步 + formal |
| **P2c** | FR27、FR36、FR37（加强）、FR38（加强）、FR40、NFR11、NFR13 | IO/浮点/IP 起步/可视化起步/CLI |
| **P3 — 概述字面闭环（新）** | **FR46、FR47、FR48、FR49、FR51、FR52**（及未在 P2 收口的加强验收） | 用户 ①C；概述 §1.3.7–11 / §1.5 |

**P3 建议交付序（可并行处已标明）：** FR52 叙事随 FR23 → FR51（语言）∥ FR28→**FR46** → FR30 加强→**FR47** → FR37 加强→**FR48** ∥ FR38 加强→**FR49**；**FR50** 随 FR35。禁止无 NFR14 记录即把 P3 FR 标 ready。

**Launch 里程碑（Finalize 决议 2026-08-21）：**  
- **Launch（对外可宣称 Bitloom 工具链可用）** = P0+P1 完成 + FR21 Bitloom 身份 + 阶段三已交付之真独立路径（若已合入）仍有效。  
- **P2** = 仿真/生态加厚（仍为正式合同，可分批宣称）。  
- **P3 / SM-6** = 概述字面闭环；**不**阻塞上述 Launch 宣称，但**阻塞**「概述愿景已全部兑现」类对外表述。

## 5. Features & Functional Requirements

### 5.1 身份与阶段一缺口

**F-Identity**

- **FR21 — 公开 README 免责与发布名** *(修订 2026-08-21)*  
  - **intent：** 与 `samitbasu/rhdl` 无关；发布名为 **`bitloom`**。  
  - **success：** README 含免责；写明公开产品名 Bitloom 与 crates.io **`bitloom`**；禁止暗示 `rhdl` / `rhdl-bits`；不得再将 **`rhdl-rs`** 表述为现行唯一发布名。  
  - **phase：** P0。

### 5.2 语言表面加厚

**F-Surface**

- **FR22 — 单时钟语言表面加厚**  
  - **intent：** 在不上 HLS/多时钟的前提下，comb/seq/运算/控制流可写真实小 RTL。  
  - **success：**  
    1. **构造条（必须）：** 文档化清单中的项均可经 prelude/builder elaborate → emit `.v` → `tick`：`if`/`match`（或等价分支）、严格同位宽二元运算与连接、显式 pad/trunc、同步复位 `Reg` 的复位赋值语义、组合完整赋值检查（延续 FR3）。  
    2. **集成 fixture：** 计数器 + 单时钟 FIFO 形示例均对齐黄金值（不得仅靠阶段一骨架冒充）。  
    3. **本 FR 非目标：** `Bundle`、`Vec<T,N>` 走 **FR51**；不得 silently 可用。  
  - **phase：** P0 · UJ-1 · SM-5。

### 5.3 时钟、复位与存储

**F-Clock-Mem**

- **FR23 — 多时钟 HIR 与语言级 CDC**  
  - **intent：** 多时钟 HIR；**Clash 式 phantom 域**（AD-22）；DoubleFlop / SyncFIFO 为语言级 CDC。  
  - **success：** 非法跨域在 freeze 失败；双时钟 DoubleFlop 或 SyncFIFO fixture 可 elaborate/emit/按域 tick。机制：**Clash 式 phantom 域**（AD-22）。  
  - **phase：** P1（在 FR26 之后）。

- **FR24 — 异步复位**  
  - **intent：** 时序包络支持异步复位（相对阶段一同频高有效同步复位）。  
  - **success：** async-reset fixture 发出边沿敏感复位 Verilog，仿真对齐置位/释放黄金值。  
  - **phase：** P1。

- **FR25 — 时钟门控与使能**  
  - **intent：** 寄存器/时序块可带 clock enable 或门控。  
  - **success：** enable=1 时行为对齐无门控黄金值；emit 为可综合 enable/门控形。  
  - **phase：** P1。

- **FR26 — 同步读 Mem（CHIRRTL 友好表面）**  
  - **intent：** 语言表面暴露 **CHIRRTL 友好名**（`Mem` ≈ comb/async-read、`SyncReadMem` ≈ sync-read；文档可对应 `cmem`/`smem`）；降级与互转锚定 FIRRTL 规范 `mem`。  
  - **success：** SyncReadMem（及文档化的 Mem）fixture → HIR → `.v`/`firrtl.mem`；tick 满足读延迟黄金值。双口跨时钟 Mem **仅**经命名 CDC FIFO（与 FR23 衔接）。  
  - **phase：** P1（先于 FR23）。

- **FR27 — 顶层 Analog / InOut / 三态**  
  - **intent：** 仅顶层 IO 允许 Analog/InOut/三态。  
  - **success：** 顶层可 emit；非顶层同构构造被结构化诊断拒绝。  
  - **phase：** P2c。

### 5.4 互操作扩展

**F-Interop**

- **FR28 — FIRRTL→Chisel 生成器** *(修订 2026-08-21)*  
  - **intent：** 从 FIRRTL/FrozenHir 生成 **可编译** 的 Chisel Scala（概述正向腿）。  
  - **success：** 文档钉死的 Chisel + firtool 版本下，fixture → Scala **必须编译通过**；公开端口名/宽/向与实例层次满足往返谓词。**「可维护」验收条（已关闭 Open Q5）：** 允许机械/生成风格；不要求手写 idiomatic；不得以「尽力失败」交差。**默认 CI 证据：** 「必须编译通过」不得仅由 Rust 往返谓词或「缺 JDK/sbt 则 skip=0」交差；须满足 **FR71** / **NFR34**。  
  - **phase：** P2b · UJ-4。完整双向见 **FR46**。CI 硬门见 **FR71**。

### 5.5 仿真扩展

**F-Sim**

- **FR29 — bridge / abstraction / mixed both** *(修订)*  
  - **intent：** 手写多视图标注仍支持。  
  - **success：** 混合 fixture `PortValues` 对照通过。生成路径见 **FR47**（不再禁止生成功能模拟器）。  
  - **phase：** P2a。

- **FR30 — 双视图形式等价** *(修订)*  
  - **intent：** 功能视图与周期精确视图一致性检查为产品能力。  
  - **success：** 一致则 pass、故意不一致则 fail；P3 收口时须接入 **FR47** 生成的双模拟器路径。  
  - **phase：** P2a → P3 与 FR47 联验。

- **FR31 — 可选 FST 波形**  
  - **intent：** FST 可选；VCD 仍为默认。  
  - **success：** 开关写出可被 GTKWave 或 Surfer 打开的 FST（允许 Verilator `--trace-fst` 或文档化 vcd2fst；不要求自研 FST writer）；关闭时仍写 VCD。  
  - **phase：** P2a。

- **FR32 — 解释器与编译 tick 引擎**  
  - **intent：** 同一 FrozenHir 可用解释或编译引擎 tick。  
  - **success：** 同一 suite 两引擎 `PortValues` 一致；文档 API/CLI 可选引擎。  
  - **phase：** P2a。

- **FR33 — C ABI / cdylib 仿真**  
  - **intent：** 功能与周期精确仿真可经 C ABI cdylib 消费。  
  - **success：** C harness 加载 cdylib，tick 两视图，对齐 Rust 黄金值。  
  - **phase：** P2a。

- **FR34 — 仿真覆盖率**  
  - **intent：** 仿真可报告覆盖。  
  - **success：** fixture 跑后报告至少一 hit 与一 miss（分支或翻转），格式稳定。  
  - **phase：** P2a。

### 5.6 生态扩展

**F-Ecosystem**

- **FR35 — HLS 前端** *(修订)*  
  - **intent：** 算法级 Rust（`#[hls]` 或等价）→ RTL 为产品路径。  
  - **success：** 默认文档路径下，至少一钉死后端（Bambu **或** Vitis）对夹具可复现产出可综合 RTL；CI/发布烟测覆盖；不可永久 unsupported。无树内自研 scheduler。  
  - **phase：** P2b · UJ-5 · 与 **FR50** 联验。

- **FR36 — 可综合浮点 `bitloom-float`**  
  - **intent：** 可综合浮点类型产品 crate（公开名对齐 Bitloom；过渡期可双名文档）。  
  - **success：** fixture elaborate/emit；tick 对齐文档化舍入用例。  
  - **phase：** P2c。

- **FR37 — IP 产品箱与黑盒** *(修订)*  
  - **intent：** 一级硬件 IP + 黑盒。  
  - **success：** 树内至少 **FIFO + UART** 可 elaborate/emit/tick；另至少一黑盒 wrapper。完整五类见 **FR48**。  
  - **phase：** P2c · UJ-5。

- **FR38 — 可视化与 HTML** *(修订)*  
  - **intent：** 内置可视化（概述 §1.3.10）。  
  - **success：** 同一 fixture：**HTML/文档化层次视图** + **时序图或等价波形视图**（可基于 VCD/FST，但须产品命令/文档入口；不得仅「用户自行打开 GTKWave」）。完整 LSP 可分阶段。  
  - **phase：** P2c → P3 与 **FR49** 联验。

- **FR39 — 形式验证 / SVA 导出**  
  - **intent：** 可向 formal/SVA 流程导出。  
  - **success：** 含断言的 fixture 导出 SVA 或文档化 formal 输入；假断言可被检查器失败。  
  - **phase：** P2b。

- **FR40 — 额外 CLI 动词** *(修订)*  
  - **intent：** CLI 覆盖常用工作流动词；`cargo bitloom build` 仍为生成主路径。  
  - **success：** 已有 `build` / `firtool` / `sim-engines` / `hls`。**P3 前必须**交付（名称可调整，能力不可缺）：`import`（FR46 反向）、`visualize` 或 `doc`（FR49 层次）、`wave` 或等价（FR49 时序/波形入口）。其余动词仍可后续加。  
  - **phase：** P2c / P3。

### 5.7 概述字面升格（新 · 2026-08-21）

**F-Overview-Literal**

- **FR46 — Bitloom ↔ Chisel 双向可维护源码互操作**  
  - **intent：** 概述 §1.3.7 / §1.5.3：以 FIRRTL 为桥的**双向**转换与混合设计。  
  - **success：**  
    1. Bitloom → Chisel Scala（满足 FR28：**可编译** + 端口/层次谓词；机械风格可接受）；  
    2. Chisel 或 `.fir` → Bitloom 可编辑模块表面或 FrozenHir 再 emit，公开端口与实例图往返谓词通过；  
    3. 文档化混合设计夹具可进入同一后端流程。  
  - **phase：** P3 · UJ-4 · SM-6。**门禁：** 无 NFR14 风险记录不得标 ready。

- **FR47 — 双视图模拟器生成与桥接**  
  - **intent：** 概述 §1.3.11 / §1.5.5。  
  - **success：** CLI/API **生成**功能模拟器工件（**Rust crate**，Open Q6 已关闭）与周期精确模拟器工件；二者经桥接或对照运行；与 FR30 联验；故意破坏等价则 fail。  
  - **phase：** P3 · UJ-6 · SM-7。**门禁：** NFR14。

- **FR48 — 一级 IP 库（UART / SPI / I2C / FIFO / AXI）**  
  - **intent：** 概述 §1.3.9 / §1.5.4。  
  - **success：** 五类均有可 `bitloom-prelude` 依赖例化的官方 IP；各至少一 smoke：elaborate → emit → tick（或文档等价）。**AXI 类（Open Q7 已关闭）= AXI4-Lite 最小从接口**即达标；另保留黑盒路径。  
  - **phase：** P3 · UJ-5 · SM-6。**门禁：** NFR14（含稳定收编/树外策略）。

- **FR49 — 内置层次图与时序图**  
  - **intent：** 概述 §1.3.10。  
  - **success：** 产品入口生成**模块层次图**与**时序图**（或等价交互视图）；夹具可无人工手写 GTKWave 脚本作为唯一路径。  
  - **phase：** P3 · UJ-6。**门禁：** NFR14。

- **FR50 — HLS 产品路径（概述 §1.3.8）**  
  - **intent：** 从算法级 Rust 自动生成 RTL 为对外承诺。  
  - **success：** 与加强后的 FR35 联验；发布文档将 HLS 列为支持功能；至少一个端到端算法夹具进 CI。  
  - **phase：** P2b / P3。

- **FR51 — 参数化复合类型 `Bundle` / `Vec`**  
  - **intent：** 概述 §1.3.4 复合结构参数化。  
  - **success：** 文档化 `Bundle` 与 `Vec<T,N>`（或等价）可 elaborate/emit/tick；位宽/方向错误在 emit 前失败。  
  - **phase：** P3。

- **FR52 — 显式 ClockDomain 产品叙事与跨域强制**  
  - **intent：** 概述 §1.5.2。  
  - **success：** 文档与夹具展示 ClockDomain（或等价）绑定时钟/复位极性/同步·异步；跨域无显式同步器则 freeze 失败。  
  - **phase：** P1（机制随 FR23）/ P3（对外叙事收口）。

### 5.8 FR28 CI 证据补强（新 · 2026-08-21）

**F-Chisel-CI**

- **FR71 — 默认 CI 强制 Chisel JVM 真编译门禁**  
  - **intent：** 兑现 FR28「必须编译通过」在**默认流水线**上的硬证据（调研 `technical-forcing-jvm-chisel-compile-in-default-ci-2026-08-21`）。  
  - **success：** GitHub Actions（或等价默认 CI）存在 **required** job，对文档化黄金夹具生成的 `.scala`，在钉死 Chisel 版本下执行真实 `scalac`/Chisel 插件编译（推荐 `sbt -batch compile`）；失败则流水线红。禁止以「Java/sbt 缺失则 exit 0 skip」冒充通过。Rust 往返谓词测试可并行保留，**不得**单独充当 FR28「必须编译通过」的唯一 CI 证据。本机 `just test` 默认可仍仅 Rust；须提供文档化的 `just chisel-fr28-jvm`（或等价）供 CI/维护者复现。  
  - **phase：** P3 / 结项后补强 · UJ-4 · 支撑 FR28/FR46 正向腿。**ID 注记：** 使用 **FR71**，避免与 epics 历史 Phase 4 FR53、Phase 5 FR60 撞号。  
  - **实现指引（非替代架构）：** Pattern A — 并行 required job（推荐名 `fr28-chisel-jvm`）；钉死版本对齐 AD-9 / Stack（Chisel 7.14.0）；本 job **不**要求跑 firtool 降级。

## 6. Non-Functional Requirements

| ID | 要求 | Phase |
|----|------|-------|
| **NFR3** | linux-x64：钉死 firtool 下载+`.sha256`+缓存；支持文档化 firtool 路径覆盖；默认不信任 PATH。 | P0 |
| **NFR10** | HIR→源码再生仍可仅调试；**不**因 FR46 而把「调试再生」冒充 Chisel 双向合同。 | P0 |
| **NFR11** | firtool 资产 macos / windows / linux-aarch64；同机制。 | P2c |
| **NFR12** | 默认 firtool 钉死版本直至有记录的 Chisel 配对再升；升钉更新校验表。 | P0 |
| **NFR13** | MSRV rustc **1.97.1**；workspace/CI/文档一致。 | P2c |
| **NFR14** | **P3 风险门禁（强制）：** 每个 FR46/47/48/49（及启动前的 FR50 若尚无记录）在 epic 标 `ready` **之前**，须在 `_agile-output/implementation-artifacts/`（或 epic 故事文件）存在风险记录，字段至少含：`(a)` 上游约束（如 CIRCT/Chisel 版本、HLS 许可）、`(b)` 粗工期带、`(c)` **禁止的静默降级**清单（例：不得把 FR46 改回「尽力失败」而不改本 PRD）、`(d)` 负责人。缺记录 = 不得开工。并行 P3 多项时另记 Chipyard 式维护风险。 | P3 |
| **NFR34** | **CI JVM 工具链合同（FR71）：** required Chisel 编译 job 使用 Temurin（或文档等价）**Java ≥ 17**；启用 sbt 依赖缓存（如 `actions/setup-java` `cache: sbt`）+ 官方 `sbt/setup-sbt`（或文档等价）；job **不得** `continue-on-error: true`；缺 JDK/sbt 必须失败。可选 `BITLOOM_CHISEL_JVM_SKIP=1` 仅文档化逃生舱，**默认 CI 不设**。墙钟：实现时实测冷/热后钉 `timeout-minutes`（建议初值 15–20）。**ID 注记：** 使用 **NFR34**，避免与 Phase 3 **NFR15**（0.x 版本政策）撞号。 | P3 |

继承：合法 Rust eDSL；显式 comb/seq；同位宽；无云控；禁止发布 `rhdl`/`rhdl-bits`。  
**不再继承：** 「互转契约仅 FrozenHir↔FIRRTL 文本、禁止 Chisel 源码合同」「禁止 HIR→功能模拟器生成」。

## 7. Assumptions（索引）

- [ASSUMPTION] 公开品牌/发布名 = Bitloom/`bitloom`；概述文件本轮不改。
- [ASSUMPTION] 外挂 HLS 调度可接受；树内不实现 scheduler。
- [ASSUMPTION] FR46 可在 CIRCT 时代通过「生成 Scala + 导入 `.fir`/CIRCT」组合满足，不要求恢复已删除的 Scala FIRRTL Parser API——但验收不因上游删除 API 而自动豁免。
- [ASSUMPTION] P2a→P2b→P2c→P3 为默认顺序；同桶可并行 epic。
- [ASSUMPTION] UJ-1…6 足够 launch 文档；不另开 UX 规格。
- 既有：CDC=AD-22；Mem 友好名=AD-21；FST 不自研 writer。

## 8. Open Questions

1. ~~FR23 CDC~~ — 已关闭（AD-22）。  
2. ~~Mem 友好名~~ — 已关闭（AD-21）。  
3. ~~FST writer~~ — 已关闭。  
4. ~~权威边界~~ — 已关闭；概述字面由 FR46–FR52 进合同；概述文件同步另任务。  
5. ~~FR46「可维护」条~~ — **已关闭（Finalize 2026-08-21）：** 可编译 + 端口/层次谓词；允许机械风格。  
6. ~~FR47 功能模拟器形态~~ — **已关闭：** 生成 Rust crate；不强制 SystemC。  
7. ~~FR48 AXI 范围~~ — **已关闭：** AXI4-Lite 最小从接口算 AXI 类达标。  

**非阻塞跟进（不挡 Finalize）：** 概述 `.md` 与 Bitloom 命名/elaborate 叙事对齐；调研 reconcile 中 HLS 许可 spike、IP 稳定收编细则 → 架构/epic。

## 9. Traceability

### 9.1 later-product → FR（基线，仍有效）

| later-product 条目 | FR/NFR |
|--------------------|--------|
| Multi-clock / CDC | FR23, FR52 |
| Async reset; clock gating | FR24, FR25 |
| Mem | FR26 |
| Analog/InOut/tri-state | FR27 |
| FIRRTL→Chisel | FR28, FR46, **FR71** |
| HIR→源码 regen debug | NFR10 |
| bridge/abstraction/both | FR29, FR47 |
| Dual-view equivalence | FR30, FR47 |
| FST | FR31 |
| Interpreter vs compiled tick | FR32 |
| C ABI | FR33 |
| Coverage | FR34 |
| HLS | FR35, FR50 |
| float crate | FR36 |
| IP + black-box | FR37, FR48 |
| Visualization / LSP / HTML | FR38, FR49 |
| Formal / SVA | FR39 |
| CLI verbs | FR40 |
| firtool multi-arch | NFR11 |
| firtool pin policy | NFR12 |
| MSRV | NFR13 |
| README / 发布名 | FR21 |
| linux-x64 firtool | NFR3 |
| 单时钟表面 | FR22 |
| Bundle / Vec | **FR51**（升格；不再是「非本 PRD FR」） |
| 默认 CI Chisel JVM 真编译 | **FR71**, **NFR34** |

### 9.2 概述 → FR（2026-08-21）

| 概述条目 | FR |
|----------|-----|
| §1.3.1–6 类型/所有权/comb-seq/参数化/Cargo/可综合 | 阶段一 + FR22 等（已覆盖） |
| §1.3.7 / §1.5.3 Chisel 双向 | FR28, FR46, **FR71**（默认 CI 编译证据） |
| §1.3.8 / §1.5.4 HLS | FR35, FR50 |
| §1.3.9 / §1.5.4 IP 库 | FR37, FR48 |
| §1.3.10 可视化 | FR38, FR49 |
| §1.3.11 / §1.5.5 多视图与双模拟器 | FR29, FR30, FR47 |
| §1.5.1 所有权防多驱动 | 阶段一 FR15 |
| §1.5.2 ClockDomain / 显式 CDC | FR23, FR24, FR52 |
| §1.3.4 复合参数化缺口 | FR51 |
