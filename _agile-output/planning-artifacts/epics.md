---
stepsCompleted:
  - step-01-validate-prerequisites
  - step-02-design-epics
  - step-03-create-stories
  - step-04-final-validation
  - step-01-validate-prerequisites-phase-2
  - step-02-design-epics-phase-2
  - step-03-create-stories-phase-2
  - step-04-final-validation-phase-2
  - step-01-validate-prerequisites-phase-3
  - step-02-design-epics-phase-3
  - step-03-create-stories-phase-3
  - step-04-final-validation-phase-3
  - step-01-validate-prerequisites-phase-4
  - step-02-design-epics-phase-4
  - step-03-create-stories-phase-4
  - step-04-final-validation-phase-4
  - step-01-validate-prerequisites-phase-5
  - step-02-design-epics-phase-5
  - step-03-create-stories-phase-5
  - step-04-final-validation-phase-5
status: complete
phase1Status: complete
phase2Status: complete
phase3Status: complete
phase4Status: complete
phase5Status: complete
phase3Scope: Bitloom rename + maturity closeout + crates.io publish
phase4Scope: True standalone after cargo install (bitloom-* publish graph)
phase5Scope: Teaching RV32 example core + step-by-step tutorial (Bitloom)
inputDocuments:
  - _agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md
  - _agile-output/planning-artifacts/prds/prd-rhdl-2026-08-19/addendum.md
  - _agile-output/planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md
  - _agile-output/planning-artifacts/research/technical-riscv-32-cpu-example-and-step-by-step-tu-2026-08-20/research.md
  - AGENTS.md
excludedFromPhase5Rewrite:
  - Phase 1–4 epics/stories (Epic 1–14 retained as historical complete)
uxDesign: none
---

# rhdl - Epic Breakdown

## Overview

This document provides the complete epic and story breakdown for rhdl, decomposing the requirements from the PRD, UX Design if it exists, and Architecture requirements into implementable stories.

阶段一：SPEC CAP-1…CAP-9（Epic 1–4）。阶段二：PRD `prd-rhdl-2026-08-19`（Epic 5–10）。阶段三：公开品牌 **Bitloom** 改名 + 成熟度结项 + crates.io 首次发布（Epic 11–12）。阶段四：`cargo install bitloom` 后真独立（Epic 13–14）。**阶段五：** 教学向 **RV32 示例核 + step-by-step 教程**（主需求源：RV32 technical research；继承 AD / 真独立合同）。无 UX。

## Requirements Inventory

### Functional Requirements

FR1: 设计者用合法 Rust 描述 RTL：类型 `Bool` / `Bits<N>` / `UInt<N>` / `SInt<N>` / `Clock` / `Reset`，端口 `Input<T>` / `Output<T>`，强制 `#[combinational]` 与 `#[sequential]`。
FR2: 含组合与时序的模块可以 `elaborate()` 得到 `FrozenHir`。
FR3: 组合逻辑不完整赋值被拒绝（无隐式 latch）。组合不得写 `Reg.d`；时序不得驱动组合网。
FR4: 每个模块恰好一个 `Clock` 端口和一个同步、高有效 `Reset` 端口；`tick` 是该时钟一个上升沿；emit 不得发明端口。
FR5: 表面算术与连接严格同位宽；扩展/截断必须是显式 pad/trunc 节点。
FR6: 设计者用 const 泛型写一份源码、以两种宽度实例化；发出的 Verilog 端口宽度与参数一致。
FR7: 设计者层次化实例化子模块并连接端口。
FR8: 位宽或方向不匹配在发出网表前失败。
FR9: 未驱动输入在发出网表前失败，除非显式悬空标记。
FR10: 从同一描述发出 Yosys 友好 Verilog，文件为 `<abi_name>.v`（`wire`/`reg`/`assign`/`always @(posedge …)`）。
FR11: 设计 crate 的 `cargo test` 先 `elaborate()` 再 `tick`；小型计数器与黄金值一致。
FR12: `rhdl-sim` 能从同一 `tick` 路径 dump VCD。
FR13: 设计者可手写 `#[functional_model]`；对照只比较 `PortValues`；`#[functional_state]` 不进入 HIR。
FR14: 禁止从 HIR 生成 TLM / 无定时功能模拟器。
FR15: `freeze` 拒绝多驱动。
FR16: 周期精确路径拒绝堆、无界递归、`dyn Trait`、捕获闭包、文件/网络/线程、默认 `f32`/`f64`。
FR17: 用户错误产生结构化诊断 `rhdl::E0xxx`（span + 英文码 + 中文说明），不 `panic` / `custom attribute panicked`。
FR18: FrozenHir ↔ 带 `FIRRTL version 6.0.0` 头的文本；HIR → `.fir` → HIR 在模块层次、公开端口名/宽/向、实例图、ground 运算、寄存器上相等。
FR19: FIRRTL 导入把 last-connect 规范化为唯一驱动后再 freeze。
FR20: `#[rhdl::top]` 是唯一生成器发现入口；`cargo rhdl build` 在本机 elaborate 并 emit。
FR21: 文档首页声明与 `samitbasu/rhdl` 无关；crates.io 发布名是 `rhdl-rs`，禁止 `rhdl` 与 `rhdl-bits`。

### NonFunctional Requirements

NFR1: 嵌入式 DSL：硬件描述是合法 Rust，无独立语言或解析器。
NFR2: 无云端控制面；工具链是本机 `cargo rhdl`。
NFR3: 默认不信任 PATH 上的 firtool；仅 CLI 下载/缓存/调用 firtool-1.155.0（`firrtl-bin-linux-x64.tar.gz` + 同 tag `.sha256`）；缓存后离线；覆盖用 `RHDL_FIRTOOL_PATH`。
NFR4: 阶段一宿主 linux-x64。Yosys / Verilator 可选，不是必装。
NFR5: MSRV rustc 1.97.1，edition 2024。CLI 日志 tracing 0.1.44。
NFR6: 设计 crate `[dependencies]` 只能是 `rhdl-prelude`；`[dev-dependencies]` 可另加 `rhdl-sim`。宏不得依赖 hir 或后端。
NFR7: 工具链 crate 许可 Apache-2.0 OR MIT。生成 HDL 不强制 SPDX。
NFR8: 所有权/线性类型不得作为多驱动声音性证明。
NFR9: 不承诺可维护 Chisel Scala；不依赖 Chisel 解析 `.fir`。

### Additional Requirements

- 无 starter 模板；绿场按脊柱 Structural Seed 建 `crates/rhdl-*` workspace。
- Epic 1 须先落地 crate 图与依赖方向（AD-6），以及 `FrozenHir` 层次 AST（AD-12）、builder 会话（AD-13）、私有 freeze（AD-7）。
- `cargo rhdl build` 用 `target/` 下 host/shim crate 进程内链接设计 crate 与后端（AD-14）；阶段一禁止 JSON 网表当 CLI 协议。
- 阶段一用户可见 HDL 只有 `rhdl-vlog` 的 `.v`。`rhdl-firrtl` 阶段一可为解析/AST，不得写用户面向 `.fir`、不得调 firtool（AD-8）。
- `Artifact` 由 hir 定义；freeze 分配 `abi_name`；私有模块 freeze 时 mangling（AD-16）。
- `PortValues` 是 `tick` 与功能模型的唯一对照形状（AD-17）。
- FIRRTL 导出与 firtool 降 Verilog 是阶段二；阶段一可演示成功不要求 `.fir` 出口。
- ~~`later-product.md` 整表不在阶段一交付~~ — 已升格为阶段二 PRD（见文末 Phase 2 Requirements Inventory）。

### UX Design Requirements

无 UI。本产品是 CLI 与 Rust 库，无 UX-DR。

### FR Coverage Map

FR1: Epic 1 — 合法 Rust RTL 类型、方向端口、强制 comb/seq
FR2: Epic 1 — 含组合与时序的模块可 elaborate
FR3: Epic 1 — 不完整组合赋值拒绝；comb/seq 写权限
FR4: Epic 1 — 显式 Clock + 同步高有效 Reset；tick 为 posedge
FR5: Epic 1 — 表面严格同位宽；显式 pad/trunc
FR6: Epic 2 — const 泛型多宽度实例化
FR7: Epic 2 — 层次化实例化与连接
FR8: Epic 2 — 位宽/方向不匹配在发出前失败
FR9: Epic 2 — 未驱动输入失败除非悬空标记
FR10: Epic 1 — Yosys 友好 `<abi>.v`
FR11: Epic 3 — cargo test：elaborate 再 tick；计数器黄金值
FR12: Epic 3 — tick 路径 dump VCD
FR13: Epic 3 — 手写 functional_model；PortValues 对照；functional_state 不进 HIR
FR14: Epic 3 — 禁止从 HIR 生成 TLM
FR15: Epic 1 — freeze 拒绝多驱动
FR16: Epic 1 — 周期精确路径拒绝不可综合构造
FR17: Epic 1 — 结构化诊断，不 panic
FR18: Epic 4 — FrozenHir ↔ FIRRTL 6 文本往返谓词
FR19: Epic 4 — 导入 last-connect 规范化
FR20: Epic 1 — `#[rhdl::top]` 与本机 `cargo rhdl build`
FR21: Epic 1 — 发布名 `rhdl-rs`；与 samitbasu/rhdl 无关

## Epic List

### Epic 1: 写出可综合模块并得到 Verilog
设计者用显式时钟/复位、方向端口和强制 comb/seq 写出 RTL，`cargo rhdl build` 产出 Yosys 友好 `.v`；非法电路在发出前以 `rhdl::E0xxx` 拒绝。含工作区、CLI host、发布身份。
**FRs covered:** FR1, FR2, FR3, FR4, FR5, FR10, FR15, FR16, FR17, FR20, FR21

### Epic 2: 参数化复用与层次连接
一份源码多种位宽；实例化子模块并连接；未驱动与宽/向错误在发出前失败。
**FRs covered:** FR6, FR7, FR8, FR9

### Epic 3: 测试里仿真并对照功能视图
`cargo test` 中 `tick` 并 dump VCD；可选手写 `#[functional_model]`，只比 `PortValues`；禁止从 HIR 生成 TLM。
**FRs covered:** FR11, FR12, FR13, FR14

### Epic 4: 与 FIRRTL 6 文本互转
FrozenHir ↔ `.fir`；往返保住层次、端口、实例、运算与寄存器；导入先把 last-connect 收成唯一驱动。
**FRs covered:** FR18, FR19

## Epic 1: 写出可综合模块并得到 Verilog

设计者用显式时钟/复位、方向端口和强制 comb/seq 写出 RTL，`cargo rhdl build` 产出 Yosys 友好 `.v`；非法电路在发出前以 `rhdl::E0xxx` 拒绝。含工作区、CLI host、发布身份。

### Story 1.1: 声明带方向端口的模块并 elaborate

As a 硬件设计者,
I want 用合法 Rust 声明带 `Input`/`Output` 的模块并 `elaborate()`,
So that 在任何后端之前就有一张冻结电路图。

**Acceptance Criteria:**

**Given** 空的 rhdl 工作区
**When** 我写一个带 `Input<UInt<8>>` / `Output<UInt<8>>`、`Clock`、`Reset` 的模块并调用 `Elaboratable::elaborate()`
**Then** 得到 `FrozenHir`（层次 AST），未冻结 HIR 不能从 `rhdl-hir` 外拿到
**And** 设计 crate 只依赖 `rhdl-prelude`；宏只展开到 builder
**And** 裸 `UInt` 当端口无法通过（FR1、FR2）

### Story 1.2: 强制 comb/seq，拦住隐式 latch

As a 硬件设计者,
I want 必须标注 `#[combinational]` / `#[sequential]`，并且组合路径写全,
So that 不会在综合里变成隐式 latch，也不会把 comb 和 seq 写串。

**Acceptance Criteria:**

**Given** Story 1.1 已能 elaborate 带端口的模块
**When** 组合块对某输出有的分支没赋值
**Then** `elaborate()` 失败，诊断指向该赋值缺口，不生成 latch（FR3）
**And** 组合逻辑写 `Reg.d`、时序逻辑驱动组合网，均失败
**And** 没有 comb/seq 标注的硬件过程不能作为电路进入 HIR

### Story 1.3: 显式时钟复位与严格同位宽

As a 硬件设计者,
I want 每个模块都有显式 `Clock` 和同步高有效 `Reset`，算术也必须同位宽,
So that 不会出现隐式全局时钟，也不会在 Verilog 和 FIRRTL 里一个截断、一个变宽。

**Acceptance Criteria:**

**Given** Story 1.2 的 comb/seq 模块能 elaborate
**When** 模块缺少 `Clock` 或 `Reset` 端口，或试图省略它们
**Then** `elaborate()` 失败；FrozenHir 上每个模块恰好这两个端口（FR4）
**And** `Reg` 绑定该时钟上升沿 + 同步复位；emit 不得补隐式端口
**And** 不同宽度相加/连接失败，除非经过显式 pad/trunc；FIRRTL 的 n+1 只能来自这些节点（FR5）

### Story 1.4: freeze 拦住多驱动与不可综合构造

As a 硬件设计者,
I want 发出网表前就看到结构化错误,
So that 多驱动、堆分配这类问题不会拖到 Yosys。

**Acceptance Criteria:**

**Given** Story 1.3 的模块能 elaborate
**When** 同一网有两个驱动，或周期精确路径使用 `Vec`/`Box`/`String`、无界递归、`dyn Trait`、捕获闭包、文件/网络/线程、默认 `f32`/`f64`
**Then** `freeze`/`elaborate` 失败，不产出电路（FR15、FR16）
**And** 错误是 `Diagnostic { span, code, en, zh }`，码为 `rhdl::E0xxx`，不 `panic` / `custom attribute panicked`（FR17）
**And** 过程宏对非法输入走 `compile_error` 或同一诊断类型

### Story 1.5: 从 FrozenHir 发出 Yosys 友好 Verilog

As a 硬件设计者,
I want 冻结电路变成 `<abi_name>.v`,
So that 可以交给现有 FPGA/ASIC 流程。

**Acceptance Criteria:**

**Given** Story 1.4 能得到合法 `FrozenHir`
**When** 调用 `rhdl-vlog` 对它 emit
**Then** 产物是 `Artifact`，文件茎为 freeze 分配的 `abi_name`，扩展名 `.v`（FR10）
**And** 内容仅用 `wire`/`reg`/`assign`/`always @(posedge …)`；无 packed array、无 `automatic` 局部变量、无作为契约的 `logic`/`always_ff`
**And** Verilog 使用 HIR 上已有的 clock/reset 端口，不发明端口；私有模块名已在 freeze 时 mangling
**And** 本故事不写用户面向 `.fir`、不调用 firtool

### Story 1.6: `#[rhdl::top]` 与本机 cargo rhdl build

As a 硬件设计者,
I want 用属性标出顶层并用本机 CLI 一键 elaborate + 出 `.v`,
So that 设计就是普通 Cargo 包，不必手写 host 胶水。

**Acceptance Criteria:**

**Given** Story 1.5 的 vlog emit 可用
**When** 设计 crate 用 `#[rhdl::top]` 标注 `Elaboratable`，运行 `cargo rhdl build`
**Then** CLI 只发现该属性；生成 `target/` 下 host crate，进程内 elaborate 再 emit（FR20）
**And** 无云端；设计 crate 不依赖 CLI/后端；阶段一不把 FrozenHir JSON 当 CLI 协议
**And** 发布身份：crates.io 名为 `rhdl-rs`；禁止文档暗示 crates.io `rhdl`；首页声明与 `samitbasu/rhdl` 无关（FR21）
**And** 文档说明设计 crate 依赖 `rhdl-prelude`，不是 `cargo add rhdl`

## Epic 2: 参数化复用与层次连接

一份源码多种位宽；实例化子模块并连接；未驱动与宽/向错误在发出前失败。

### Story 2.1: const 泛型多宽度实例化

As a 硬件设计者,
I want 用一份带 const 泛型的模块源码实例化不同位宽,
So that 不必为每种宽度复制粘贴。

**Acceptance Criteria:**

**Given** Epic 1 能 elaborate 并 emit 固定宽度模块
**When** 我写 `const W: usize` 参数化的加法器/寄存器，并以 W=8 与 W=16 各 elaborate 一次
**Then** 两次 FrozenHir / `.v` 的端口宽度分别为 8 与 16，与参数一致（FR6）
**And** 设计 crate 仍只依赖 `rhdl-prelude`

### Story 2.2: 层次实例化与端口连接

As a 硬件设计者,
I want 在父模块里实例化子模块并连接端口,
So that 可以组装更大的设计而不展平成一张网。

**Acceptance Criteria:**

**Given** Story 2.1 的参数化模块可用
**When** 父模块实例化至少一个子模块并连接匹配的 Input/Output
**Then** FrozenHir 保留 Instance 层次（不在 elaborate 时展平）（FR7）
**And** 发出的 `.v` 默认保留层次（不强制 flatten）
**And** 子模块端口名与参数元数据可随 HIR 存活（供后续 FIRRTL annotation）

### Story 2.3: 连接完整性（宽/向与未驱动）

As a 硬件设计者,
I want 错误的连接和漏接在发出网表前失败,
So that 浮空输入和方向反接不会进到仿真或综合。

**Acceptance Criteria:**

**Given** Story 2.2 的层次连接可用
**When** 连接位宽不匹配或方向不合法
**Then** freeze/elaborate 失败并带 span 诊断（FR8）
**And** 未驱动输入失败，除非显式悬空标记；未连接输出至少警告或按约定报错（FR9）
**And** 合法全连接的层次设计仍能 emit `.v`

## Epic 3: 测试里仿真并对照功能视图

`cargo test` 中 `tick` 并 dump VCD；可选手写 `#[functional_model]`，只比 `PortValues`；禁止从 HIR 生成 TLM。

### Story 3.1: cargo test 里 elaborate 再 tick

As a 硬件设计者,
I want 在 `cargo test` 里对 FrozenHir 做周期精确 `tick`,
So that 不用另开仿真器就能验证计数器行为。

**Acceptance Criteria:**

**Given** Epic 1 的 FrozenHir（例如计数器）
**When** 测试调用 `elaborate()` 再 `rhdl_sim::tick`，输入/复位按 PortValues
**Then** 若干周期后输出与黄金值一致（FR11）
**And** 设计 crate 仅在 `[dev-dependencies]` 依赖 `rhdl-sim`；prelude 不依赖 sim
**And** `tick` 是模块 Clock 的一个 posedge，Reset 按端口采样

### Story 3.2: 从 tick 路径 dump VCD

As a 硬件设计者,
I want 仿真时写出 VCD,
So that 可以用波形查看器对照信号。

**Acceptance Criteria:**

**Given** Story 3.1 的 tick 可用
**When** 测试在同一记录器 API 上启用 VCD dump 并跑若干 tick
**Then** 生成可读 VCD 文件，含模块端口/寄存器名与变化（FR12）
**And** 不引入第二套并行的波形 API；本故事不做 FST（later-product）

### Story 3.3: 手写功能模型与 PortValues 对照

As a 硬件设计者,
I want 为同一模块手写 `#[functional_model]` 并与 tick 对照,
So that 能快速验算法，又不用从网表生成 TLM。

**Acceptance Criteria:**

**Given** Story 3.1 的 PortValues tick
**When** 我写 `#[functional_model]`，其 `cycle` 吃/吐 PortValues，并用随机向量与 tick 比较
**Then** 对照只比 PortValues；不一致则测试失败（FR13）
**And** `#[functional_state]` 字段不出现在 FrozenHir
**And** 工具链不提供从 HIR 生成 TLM/无定时 sim 的路径（FR14）

## Epic 4: 与 FIRRTL 6 文本互转

FrozenHir ↔ `.fir`；往返保住层次、端口、实例、运算与寄存器；导入先把 last-connect 收成唯一驱动。

### Story 4.1: 从 FrozenHir 导出 FIRRTL 6 文本

As a 硬件设计者,
I want 把冻结电路写成带 `FIRRTL version 6.0.0` 头的 `.fir`,
So that 可以交给 firtool / Chisel 生态（模块级交换）。

**Acceptance Criteria:**

**Given** Epic 1 的合法 FrozenHir（标量端口子集）
**When** 调用 `rhdl-firrtl` emit
**Then** 写出带 `FIRRTL version 6.0.0` 头的文本；文件茎用 `abi_name`（FR18 的导出半边）
**And** 不生成可维护 Chisel Scala；不调用 firtool 覆盖 `<abi>.v`
**And** 源模块名/参数以 annotation 或等价元数据保留

### Story 4.2: 导入 FIRRTL 并完成可逆子集往返

As a 硬件设计者,
I want 导入 FIRRTL 子集并与导出往返一致,
So that 模块级 IP 能进 RHDL，且 last-connect 不会破坏唯一驱动。

**Acceptance Criteria:**

**Given** Story 4.1 能导出 `.fir`
**When** 对可逆子集做 HIR → `.fir` → HIR
**Then** 模块层次、公开端口名/宽/向、实例图、ground 运算、寄存器相等（FR18）
**And** 导入路径把 last-connect 规范化为唯一驱动后再 freeze；不得在 HIR 中留下多驱动（FR19）
**And** 导入子集拒绝 property、CHIRRTL 特有 mem、Analog/InOut；`import` 与 `elaborate` 共用同一私有 freeze

---

## Phase 2 Requirements Inventory

> 来源：阶段二 PRD（final）+ ARCHITECTURE-SPINE AD-20…AD-26 + 技术调研。阶段一 FR1–FR20 / NFR1–NFR9 仍有效；本清单为阶段二交付范围。FR Coverage Map / Epic List 在 step-02 填写。

### Phase 2 Functional Requirements

FR21: 仓库 README 声明与 `samitbasu/rhdl` 无关；crates.io 发布名 `rhdl-rs`；禁止暗示 `rhdl` / `rhdl-bits`。（P0；阶段一缺口，本阶段交付所有权）
FR22: 单时钟语言表面加厚：`if`/`match`、严格同位宽运算与连接、显式 pad/trunc、同步 Reg 复位语义；集成 fixture=计数器+单时钟 FIFO 形；`Bundle`/`Vec` 本 FR 非目标。（P0 · AD-20）
FR23: 多时钟 HIR + Clash 式 phantom 域 CDC（AD-22）；非法跨域 freeze 失败；语言级 DoubleFlop/SyncFIFO；默认模块仍 AD-15 单时钟。（P1，在 FR26 之后）
FR24: 异步复位包络；emit 边沿敏感复位；仿真对齐置位/释放黄金值。（P1）
FR25: 时钟门控 / enable；enable=1 对齐无门控黄金值；可综合 emit。（P1）
FR26: CHIRRTL 友好名 `Mem`/`SyncReadMem`（文档对应 cmem/smem）；降级/互转锚 FIRRTL `mem`；跨时钟 Mem 仅经命名 CDC FIFO。（P1，先于 FR23 · AD-21）
FR27: Analog/InOut/三态仅顶层 IO；非顶层拒绝。（P2c）
FR28: 可选 FIRRTL→Chisel Scala 尽力生成器；非互转契约。（P2b）
FR29: 手写 `#[bridge]`/`#[abstraction]`/mixed `both`；PortValues 对照；无 HIR→TLM。（P2a）
FR30: 功能视图与周期精确视图形式等价检查；不一致则 fail。（P2a）
FR31: 可选 FST（Verilator/vcd2fst）；默认仍 VCD。（P2a · AD-24）
FR32: 解释器与编译 tick 引擎对同一 FrozenHir 产出一致 PortValues。（P2a）
FR33: C ABI / cdylib 消费功能与周期精确仿真。（P2a）
FR34: 仿真覆盖率报告（至少一 hit 与一 miss）。（P2a）
FR35: 可选 `#[hls]` 外挂 Bambu 或 XLS（钉死其一）；无自研调度。（P2b · AD-25）
FR36: 可综合浮点产品 crate `rhdl-float`。（P2c）
FR37: 至少一个树内 IP + 一个黑盒 wrapper 可 elaborate/emit。（P2c）
FR38: FrozenHir→HTML 文档 + 实例层次；LSP hover/goto deferred。（P2c）
FR39: Formal/SVA 导出路径。（P2b）
FR40: CLI 动词 `build`/`firtool`/`sim-engines`/`hls`；`check`/… deferred；`build` 仍为主路径。（P2c）

### Phase 2 NonFunctional Requirements

NFR3: linux-x64 下载钉死 firtool-1.155.0 的 `firrtl-bin-linux-x64.tar.gz` + `.sha256`；缓存离线；`RHDL_FIRTOOL_PATH`；默认不信任 PATH。（P0 · AD-9）
NFR10: HIR→RHDL 源码再生仅调试；非产品互转。（P0）
NFR11: firtool 资产覆盖 macos/windows/linux-aarch64；机制同 NFR3。（P2c）
NFR12: 默认钉 1.155.0 直至有记录的 Chisel 配对再升 1.156.0。（P0）
NFR13: MSRV 为 rustc 1.97.1；workspace/CI/文档一致。（P2c）

继承阶段一 NFR1–NFR2、NFR4–NFR9（合法 Rust eDSL、无云控、许可、不把 Scala 当契约等）。

### Phase 2 Additional Requirements (Architecture)

- 遵守 AD-1…AD-19；阶段二另遵 **AD-20…AD-26**（表面、Mem、phantom CDC、异步复位/enable、FST、HLS 外挂、合同海拔）。
- AD-22 已确认 Clash 式 phantom；Epic 拆解不得另起并行 CDC 模型。
- AD-21：表面 CHIRRTL 友好名；互转仍 `firrtl.mem`。
- AD-9 / NFR3：CLI 必须真正实现下载校验缓存（不仅 smoke skip）。
- AD-26：阶段二 epic 以 `prd-rhdl-2026-08-19` 为需求源；`later-product.md` 仅为索引。
- 交付顺序约束：P0（FR21,NFR3,FR22,NFR10,NFR12）→ P1（**FR26 先于** FR23/24/25）→ P2a→P2b→P2c。
- 无 starter 模板；无 UX。
- `Bundle`/`Vec` 仍 Deferred（AD-20），不得写入阶段二 epic 默认可交付范围。

### Phase 2 UX Design Requirements

无 UI。无 UX-DR。

### Phase 2 FR Coverage Map

FR21: Epic 5 — README / rhdl-rs 身份
FR22: Epic 6 — 单时钟表面加厚
FR23: Epic 7 — phantom 多时钟 CDC
FR24: Epic 7 — 异步复位
FR25: Epic 7 — 时钟门控/enable
FR26: Epic 7 — Mem/SyncReadMem（先于 FR23）
FR27: Epic 10 — 顶层 Analog/InOut/三态
FR28: Epic 9 — FIRRTL→Chisel 尽力生成
FR29: Epic 8 — bridge/abstraction/both
FR30: Epic 8 — 双视图形式等价
FR31: Epic 8 — 可选 FST
FR32: Epic 8 — 解释器/编译 tick
FR33: Epic 8 — C ABI cdylib
FR34: Epic 8 — 覆盖率
FR35: Epic 9 — HLS 外挂
FR36: Epic 10 — rhdl-float
FR37: Epic 10 — IP 与黑盒
FR38: Epic 10 — 可视化/LSP/HTML
FR39: Epic 9 — Formal/SVA 导出
FR40: Epic 10 — 额外 CLI 动词
NFR3: Epic 5 — firtool 钉死下载
NFR10: Epic 5 — HIR→源码仅调试
NFR11: Epic 10 — 多平台 firtool
NFR12: Epic 5 — 1.155 直至 Chisel 配对
NFR13: Epic 10 — MSRV 1.97.1


## Phase 2 Epic List

### Epic 5: 身份可信与可复现 firtool
设计者与 CI 能从 README 认清本仓库发布身份，并在 linux-x64 上校验下载钉死的 firtool（缓存后离线可复现）。
**FRs covered:** FR21; NFR3, NFR10, NFR12

### Epic 6: 可写的单时钟 RTL 表面
设计者能用 if/match、同位宽运算、pad/trunc 与同步 Reg 复位写出并通过黄金测试的小设计（计数器 + FIFO 形）。
**FRs covered:** FR22

### Epic 7: 存储器与多时钟时序
设计者先能用 CHIRRTL 友好名写 Mem/SyncReadMem；再打开 phantom 域 CDC、异步复位与 clock enable。
**FRs covered:** FR26, FR23, FR24, FR25

### Epic 8: 更丰富的仿真体验
设计者可用 bridge/形式等价、可选 FST、双 tick 引擎、C ABI 与覆盖率加深验证。
**FRs covered:** FR29, FR30, FR31, FR32, FR33, FR34

### Epic 9: 外挂桥接（HLS / Scala / formal）
设计者可选用外挂 HLS、尽力 Chisel 生成与 formal/SVA 导出，且工具链不自研调度器。
**FRs covered:** FR28, FR35, FR39

### Epic 10: 生态与主机扩展
设计者获得顶层特殊 IO、浮点、IP、可视化/LSP、额外 CLI 动词，以及多平台 firtool 与可选 MSRV 升级。
**FRs covered:** FR27, FR36, FR37, FR38, FR40; NFR11, NFR13



## Epic 5: 身份可信与可复现 firtool

设计者与 CI 能从 README 认清本仓库发布身份，并在 linux-x64 上校验下载钉死的 firtool（缓存后离线可复现）。

### Story 5.1: 公开 README 免责与发布名

As a 硬件设计者 / 潜在贡献者,
I want 在仓库首页看到与 `samitbasu/rhdl` 无关且发布名为 `rhdl-rs` 的声明,
So that 不会把本项目当成 crates.io 上的 `rhdl`。

**Acceptance Criteria:**

**Given** 空克隆的仓库根目录
**When** 打开 `README.md`
**Then** 可见与 `samitbasu/rhdl` 无关的免责声明
**And** 写明 crates.io 发布名 `rhdl-rs`，并禁止暗示 `rhdl` / `rhdl-bits`（FR21）

### Story 5.2: CLI 钉死下载并校验 firtool

As a CI 维护者,
I want `cargo rhdl` 在 linux-x64 上下载 `firtool-1.155.0` 的 `firrtl-bin-linux-x64.tar.gz` 并校验 `.sha256`,
So that 不依赖 PATH 上的随机 firtool，且缓存后可离线。

**Acceptance Criteria:**

**Given** 干净缓存目录的 linux-x64 环境
**When** 触发需要 firtool 的 CLI 路径（或显式 firtool 获取命令）
**Then** 下载钉死资产并校验 sibling `.sha256`，失败则拒绝继续
**And** 第二次运行命中缓存且可断网成功
**And** `RHDL_FIRTOOL_PATH` 可覆盖为含 `firtool` 的目录（NFR3, NFR12；AD-9）

### Story 5.3: 文档化 HIR→源码仅为调试

As a 工具链用户,
I want 文档明确 HIR→RHDL 再生不是产品互转,
So that 不会把它当成 FrozenHir↔FIRRTL 合同的一部分。

**Acceptance Criteria:**

**Given** 发布/用户文档与 CLI 帮助
**When** 查阅源码再生或调试相关说明
**Then** 标明 debug-only，且无发行测试宣称源码往返稳定（NFR10）


## Epic 6: 可写的单时钟 RTL 表面

设计者能用 if/match、同位宽运算、pad/trunc 与同步 Reg 复位写出并通过黄金测试的小设计。

### Story 6.1: 分支与同位宽运算表面

As a 硬件设计者,
I want 在 comb/seq 中使用 `if`/`match`（或等价）与严格同位宽运算/连接,
So that 能表达真实组合与时序逻辑而不靠骨架占位。

**Acceptance Criteria:**

**Given** 仅依赖 `rhdl-prelude` 的设计 crate
**When** 编写含分支与同位宽二元运算/连接的模块并 `elaborate()`
**Then** 得到合法 `FrozenHir`，emit 的 `.v` 含对应逻辑
**And** 不同宽度运算/连接失败，除非显式 pad/trunc（FR22；FR5）

### Story 6.2: 同步 Reg 复位语义

As a 硬件设计者,
I want 同步复位下的 `Reg` 赋值语义明确且可仿真,
So that 复位行为与 emit/`tick` 一致。

**Acceptance Criteria:**

**Given** 带 `Clock`/`Reset` 与 `Reg` 的模块
**When** 在复位有效/无效下 `tick`
**Then** 寄存器值符合文档化同步复位语义
**And** emit 为 `always @(posedge clk)` 下的同步复位形（FR22；AD-15/AD-20）

### Story 6.3: 计数器与 FIFO 形黄金 fixture

As a 硬件设计者,
I want 计数器与单时钟 FIFO 形示例均通过黄金测试,
So that FR22 不是靠阶段一骨架冒充完成。

**Acceptance Criteria:**

**Given** 文档化的计数器与单时钟 FIFO 形示例
**When** 运行 `cargo test`（elaborate → tick）并 `cargo rhdl build`
**Then** 黄金向量通过且产出 Yosys 可读 `.v`
**And** `Bundle`/`Vec` 仍不可 silently 使用（FR22 NON-GOAL；SM-5）


## Epic 7: 存储器与多时钟时序

设计者先能用 CHIRRTL 友好名写 Mem/SyncReadMem；再打开 phantom 域 CDC、异步复位与 clock enable。

### Story 7.1: CHIRRTL 友好名 Mem/SyncReadMem

As a 硬件设计者,
I want 用 `Mem` / `SyncReadMem` 友好名写单时钟存储,
So that 表面贴近 Chisel 而降级与互转仍锚定 FIRRTL 规范 `mem`。

**Acceptance Criteria:**

**Given** 单时钟设计 crate 与文档化的 Mem/SyncReadMem API
**When** 编写 SyncReadMem（及文档化的 Mem）fixture 并 elaborate
**Then** HIR 降为规范 mem 语义，emit `.v` 与/或 `firrtl.mem`
**And** `tick` 满足读延迟黄金值
**And** 未封装的双时钟裸 mem 被拒绝；跨时钟存储须经命名 CDC FIFO（FR26；AD-21）

### Story 7.2: Phantom 域与语言级 CDC

As a 硬件设计者,
I want Clash 式 phantom 域以及 DoubleFlop / SyncFIFO,
So that 非法跨域在发出网表前失败。

**Acceptance Criteria:**

**Given** 声明两个时钟域的模块
**When** 非法跨域赋值并 freeze/elaborate
**Then** 失败并产生 `rhdl::E0xxx` 诊断
**And** 合法 DoubleFlop 或 SyncFIFO fixture 可 elaborate、emit、按域 tick（FR23；AD-22）
**And** 未声明多时钟的模块仍满足 AD-15（单 Clock + 同步高有效 Reset）

### Story 7.3: 异步复位

As a 硬件设计者,
I want 可选异步复位包络,
So that 复位行为可表达为边沿敏感复位。

**Acceptance Criteria:**

**Given** 选用异步复位的模块
**When** emit 与 `tick`
**Then** Verilog 含边沿敏感复位形，仿真对齐置位/释放黄金值（FR24；AD-23）

### Story 7.4: 时钟门控与使能

As a 硬件设计者,
I want 寄存器可带 clock enable 或门控,
So that 可写能控更新的时序逻辑。

**Acceptance Criteria:**

**Given** 带 enable/门控的寄存器模块
**When** enable 恒为 1 时跑黄金向量
**Then** 行为对齐无门控对照
**And** emit 为可综合 enable 或门控形（FR25；AD-23）


## Epic 8: 更丰富的仿真体验

设计者可用 bridge/形式等价、可选 FST、双 tick 引擎、C ABI 与覆盖率加深验证。

### Story 8.1: 手写 bridge / abstraction / mixed both

As a 硬件设计者,
I want 手写 `#[bridge]` / `#[abstraction]` 或 mixed `both` 仿真视图,
So that 能在不生成 TLM 的前提下做多视图验证。

**Acceptance Criteria:**

**Given** 文档化的混合仿真 fixture
**When** 按文档视图运行对照
**Then** 仅比较 `PortValues`，不一致则失败
**And** 工具链不提供从 HIR 生成 TLM 的路径（FR29；FR14）

### Story 8.2: 双视图形式等价

As a 硬件设计者,
I want 功能视图与周期精确视图的形式等价检查,
So that 能自动发现两视图偏离。

**Acceptance Criteria:**

**Given** 一对功能模型与 FrozenHir tick 路径
**When** 运行等价检查
**Then** 一致时 pass，故意不一致时 fail（FR30）

### Story 8.3: 可选 FST 波形

As a 硬件设计者,
I want 可选写出 FST，同时保留默认 VCD,
So that 大波形可用 GTKWave/Surfer 打开。

**Acceptance Criteria:**

**Given** 启用 FST 的仿真配置
**When** 跑若干 tick
**Then** 产出可被 GTKWave 或 Surfer 打开的 FST（允许 Verilator `--trace-fst` 或文档化 vcd2fst）
**And** 关闭 FST 时仍写出 VCD（FR31；AD-24）

### Story 8.4: 解释器与编译 tick 引擎

As a 硬件设计者,
I want 同一 FrozenHir 可用解释器或编译引擎 tick,
So that 可在调试速度与执行速度间选择。

**Acceptance Criteria:**

**Given** 同一 fixture suite 与文档化的引擎选择 API/CLI
**When** 分别用解释器与编译引擎运行
**Then** `PortValues` 序列一致（FR32）

### Story 8.5: C ABI / cdylib 仿真

As a 集成工程师,
I want 通过 C ABI cdylib 消费功能与周期精确仿真,
So that 非 Rust 宿主也能驱动 tick。

**Acceptance Criteria:**

**Given** 构建出的 cdylib 与 C harness
**When** 加载库并 tick 两视图
**Then** 结果对齐 Rust 侧黄金值（FR33）

### Story 8.6: 仿真覆盖率

As a 硬件设计者,
I want 仿真结束后看到覆盖率报告,
So that 知道测到了什么、漏了什么。

**Acceptance Criteria:**

**Given** 跑完的 fixture
**When** 查看覆盖率输出
**Then** 报告至少一 hit 与一 miss（分支或翻转），格式稳定可解析（FR34）


## Epic 9: 外挂桥接（HLS / Scala / formal）

设计者可选用外挂 HLS、尽力 Chisel 生成与 formal/SVA 导出，且工具链不自研调度器。

### Story 9.1: FIRRTL→Chisel 尽力生成器

As a 需要对接 Chisel 工具链的设计者,
I want 可选地将 `.fir` 尽力生成 Chisel Scala,
So that 在不改变 FrozenHir↔FIRRTL 合同的前提下获得便利。

**Acceptance Criteria:**

**Given** 文档钉死的 Chisel 版本与 fixture `.fir`
**When** 运行尽力生成器
**Then** 产出可编译的 Scala，或结构化尽力失败诊断
**And** 互转契约仍是 FrozenHir↔FIRRTL 6 文本（FR28；AD-3）

### Story 9.2: 可选 HLS 外挂

As a 算法加速设计者,
I want `#[hls]` 调用钉死的 Bambu 或 XLS,
So that 无需自研调度器即可得到 HLS 产物。

**Acceptance Criteria:**

**Given** 启用 HLS 路径且文档钉死一个后端（Bambu 或 XLS）
**When** 编译带 `#[hls]` 的函数
**Then** 调用该后端并产生可复现文档化产物
**And** 未启用时 CLI/文档标明 unsupported
**And** 无 rhdl crate 实现 scheduling（FR35；AD-25）

### Story 9.3: Formal / SVA 导出

As a 验证工程师,
I want 导出 formal/SVA 输入,
So that 能用外部检查器验证断言。

**Acceptance Criteria:**

**Given** 含断言的 fixture
**When** 运行 formal/SVA 导出
**Then** 产出检查器可接受的输入；假断言可被失败检出（FR39）

## Epic 10: 生态与主机扩展

设计者获得顶层特殊 IO、浮点、IP、可视化/LSP、额外 CLI 动词，以及多平台 firtool 与可选 MSRV 升级。

### Story 10.1: 顶层 Analog / InOut / 三态

As a 芯片顶层设计者,
I want 仅在顶层使用 Analog/InOut/三态,
So that 板级 IO 可表达且内部误用被拦住。

**Acceptance Criteria:**

**Given** 顶层与非顶层模块
**When** 在顶层声明 InOut/Analog/三态并 emit
**Then** 产物合法
**And** 非顶层同构构造被 `rhdl::E0xxx` 拒绝（FR27）

### Story 10.2: 可综合浮点 rhdl-float

As a 需要定点/浮点混合的设计者,
I want `rhdl-float` 产品 crate,
So that 可综合浮点有明确舍入语义。

**Acceptance Criteria:**

**Given** `rhdl-float` fixture
**When** elaborate、emit 与 tick
**Then** 对齐文档化舍入用例（FR36）

### Story 10.3: IP 产品箱与黑盒封装

As a 系统集成者,
I want 至少一个树内 IP 与一个外部 IP 黑盒 wrapper,
So that 可复用常见外设并接入不透明 IP。

**Acceptance Criteria:**

**Given** 文档指定的树内 IP 与黑盒 wrapper
**When** elaborate 与 emit
**Then** 两者成功；黑盒实例保持不透明（不内联 HIR）（FR37）

### Story 10.4: HIR 可视化、LSP 与 HTML 文档

As a 设计者,
I want 从 FrozenHir 得到 HTML 文档与实例层次列表,
So that 无需另开工具链即可浏览设计。

**Acceptance Criteria:**

**Given** 同一 FrozenHir fixture
**When** 运行 `rhdl_viz::to_html`
**Then** 产出 HTML 文档，含模块/端口与实例层次
**And** 完整 LSP hover/goto 为 deferred（文档声明；无 language-server 二进制）（FR38）

### Story 10.5: 额外 CLI 动词

As a 工具链用户,
I want `build` / `firtool` / `sim-engines` / `hls`,
So that 常用工作流不必手写胶水。

**Acceptance Criteria:**

**Given** 安装好的 `cargo-rhdl`
**When** 对每个已交付动词运行 `--help` 与 smoke
**Then** 均成功
**And** `cargo rhdl build` 仍为生成主路径
**And** `check`/`import`/`visualize`/`wave`/`doc`/`build-sim` 为 deferred（FR40；见 `docs/fr40-cli-verbs.md`）

### Story 10.6: 多平台 firtool 资产

As a 非 linux-x64 宿主上的用户,
I want 同一套下载/校验/缓存机制覆盖 macos/windows/linux-aarch64,
So that 本地开发不绑死一种 OS。

**Acceptance Criteria:**

**Given** 支持列表中的平台
**When** 触发 firtool 获取
**Then** 下载匹配三元组的 CIRCT 资产并校验 `.sha256`（机制同 NFR3）
**And** 不支持平台给出明确错误（NFR11）

### Story 10.7: MSRV 钉为 1.97.1

As a 工具链维护者,
I want MSRV 明确为 rustc 1.97.1,
So that workspace / CI / 文档与真实可获取的 stable 一致。

**Acceptance Criteria:**

**Given** 工作区与文档
**When** 检查 `rust-toolchain.toml`、`rust-version` 与 NFR13 文档
**Then** 均为 **1.97.1** 且 `just test` 通过（NFR13；不再要求不存在的 1.98.0）

## Phase 3 Requirements Inventory

> 范围确认（2026-08-19）：**追加阶段三**，不重写 Epic 1–10。来源：命名研究（Bitloom 锁定）+ 结项/发布研究（包名由 `rhdl-rs` 校正为 **`bitloom`**）+ ARCHITECTURE-SPINE（须修订 AD-2）+ 阶段二 PRD/addendum（FR21 发布名条款被本阶段 FR41 取代）。阶段一/二 FR1–FR40 / NFR1–NFR13 仍有效；本清单为阶段三交付范围。

### Phase 3 Functional Requirements

FR41: **公开品牌 Bitloom 与发布身份** — 仓库 README / 公开首页以 **Bitloom** 为产品名；声明与 `samitbasu/rhdl` 无关；crates.io 发布名为 **`bitloom`**；禁止暗示或占用 `rhdl` / `rhdl-bits`；**不再**以 `rhdl-rs` 为发布名。（取代阶段二 FR21 中的 `rhdl-rs` 发布名条款；免责声明义务延续。）
FR42: **用户可见表面改名** — 对外 CLI 二进制 / `cargo` 调用品牌为 Bitloom（如 `bitloom` / `cargo bitloom`）；`--help`、用户文档、`AGENTS.md`、发布相关说明中的产品/包名与 FR41 一致；历史 `rhdl-rs` /「产品名 RHDL」表述从用户可见表面清除或明确标为历史内部名。
FR43: **架构身份决议落地** — 修订并采纳 ARCHITECTURE-SPINE **AD-2**：crates.io 发布身份为 **`bitloom`**（非 `rhdl-rs`）；Git 仓库路径可仍为 `rhdl`；内部 `rhdl-*` 库 crate 名可暂留，但新对外发布名发布前须查 crates.io；设计 crate 仍只依赖 prelude。
FR44: **发布成熟度合同（干净结项）** — 在首次 crates.io 发布前具备可验证合同：`rust-version`/MSRV（1.97.1）有 CI 验证；书面 SemVer 政策（默认停在 **0.x**）；`SECURITY.md`（或等价安全联系路径）；CHANGELOG + 与发布对齐的 git tag 约定；README 诚实列出 deferred / non-goals（不得暗示未交付 API）；目标发布 crate 的 `cargo doc` / docs.rs 可构建路径明确。
FR45: **首次 crates.io 发布 `bitloom`** — 发布前再次确认 `bitloom` 可用；补齐 C-METADATA（description、license、repository、readme、keywords≤5、categories、authors、`rust-version`）；`cargo publish --dry-run` 通过后**手动**首次发布至少一个对外 crate（至少 CLI/伞形包 `bitloom`）；其余未承诺 API 的库 crate 可 `publish = false`。
FR46: **首发后的发布自动化** — 配置 crates.io Trusted Publishing（OIDC）；后续版本走 release-plz（或等价）流水线；可选 cargo-dist 仅负责安装器用二进制，不替代 registry 发布。

### Phase 3 NonFunctional Requirements

NFR14: crates.io 名 **FCFS 永久**；yank 不释放名；不计划收回 `rhdl`；禁止发布 `rhdl` / `rhdl-bits`；产品登记身份仅为 **`bitloom`**。
NFR15: 首次及结项期版本停在 **0.x**；不以「冲 1.0」作为结项条件（1.0 = 公开 API 稳定门，非 backlog 空）。
NFR16: 首次 publish 前必须通过 `cargo publish --dry-run`；人工复核 [crates.io/policies](https://crates.io/policies)；发布瞬间再探测一次 `bitloom` 可用性。
NFR17: 改名在**首次 publish 之前**完成用户可见表面（FR41–FR43）；禁止先以旧身份上架再双名长期并存。

### Phase 3 Additional Requirements (Architecture / Research)

- 遵守既有 AD-1…AD-26 语言/HIR/后端不变量；阶段三**不**另立 HIR，不在 rustc 编译期抽网表。
- 必须修订 **AD-2** 文本：`rhdl-rs` → **`bitloom`**（与命名锁定一致）。
- 结项路径（调研 R1–R4，包名已校正）：成熟度合同 → dry-run → 手动首发 `bitloom` → Trusted Publishing → release-plz（+ 可选 cargo-dist）。
- 仓库目录 / GitHub remote 是否改名为 `bitloom`：**可选**，单独故事；默认可保持路径 `rhdl`。
- 商标/域名深度检索为可选合规（命名 N6），非阻断语言功能交付；若做则作为独立故事。
- 阶段二 PRD Vision/SM-4 中「发布名 `rhdl-rs`」由本阶段 FR41 取代；后续 PRD 补丁或 addendum 应记录此变更（可与 FR41 同故事）。

### Phase 3 UX Design Requirements

无 UI。无 UX-DR。

### Phase 3 FR Coverage Map

FR41: Epic 11 — 公开品牌 Bitloom；发布名 `bitloom`；免责；禁 `rhdl`/`rhdl-bits`
FR42: Epic 11 — CLI/文档/AGENTS 等用户可见表面改名
FR43: Epic 11 — 修订 AD-2 发布身份为 `bitloom`
FR44: Epic 12 — 发布成熟度合同（MSRV CI、0.x、SECURITY、CHANGELOG、deferred）
FR45: Epic 12 — dry-run 后手动首发 `bitloom`
FR46: Epic 12 — Trusted Publishing + 后续 release-plz

### Phase 3 NFR notes

NFR14, NFR17 → Epic 11（身份与「先改名再上架」）  
NFR15, NFR16 → Epic 12（0.x 与 dry-run/政策复核）

## Phase 3 Epic List

### Epic 11: 世界认出 Bitloom
设计者、文档读者与 crates.io 访客看到的产品名是 **Bitloom**（包/CLI：`bitloom`），而非 RHDL/`rhdl-rs`；架构 AD-2 与公开表面一致；持续声明与 `samitbasu/rhdl` 无关。
**FRs covered:** FR41, FR42, FR43（NFR14, NFR17）

### Epic 12: 干净结项并上架 bitloom
维护者具备可验证的发布合同后，将至少一个对外 crate 以 **0.x** 手动首发到 crates.io，并接上 Trusted Publishing + 后续自动化；诚实披露 deferred，不以冲 1.0 当结项。
**FRs covered:** FR44, FR45, FR46（NFR15, NFR16）
**Depends on:** Epic 11 完成用户可见改名（NFR17）后方可首发；Epic 12 本身交付完整发布路径。

## Epic 11: 世界认出 Bitloom

设计者、文档读者与 crates.io 访客看到的产品名是 **Bitloom**（包/CLI：`bitloom`），而非 RHDL/`rhdl-rs`；架构 AD-2 与公开表面一致；持续声明与 `samitbasu/rhdl` 无关。

### Story 11.1: 修订 AD-2 发布身份为 bitloom

As a 架构与实现者,
I want AD-2 明确 crates.io 身份为 `bitloom`,
So that 后续改名与发布不会再写回 `rhdl-rs`。

**Acceptance Criteria:**

**Given** ARCHITECTURE-SPINE 中现有 AD-2（发布名 `rhdl-rs`）
**When** 修订 AD-2 并标记决议日期
**Then** 规则写明：crates.io 发布名为 **`bitloom`**；禁止 `rhdl` / `rhdl-bits`；Git 路径可仍为 `rhdl`；内部 `rhdl-*` 可暂留但新对外名须先查 crates.io；设计 crate 仍只依赖 prelude
**And** `AGENTS.md` / 策略块与 AD-2 一致（FR43, NFR14）

### Story 11.2: 对外包名与 CLI 品牌改为 bitloom

As a 本机用户,
I want 安装与调用的 CLI 品牌是 Bitloom,
So that 帮助文本与包名不再像另一个 RHDL 项目。

**Acceptance Criteria:**

**Given** Story 11.1 已锁定 AD-2
**When** 将对外发布/CLI crate 的 `[package].name` 与二进制（及 `cargo` 调用约定）改为 **`bitloom`**，并更新用户可见 `--help`/错误里的产品名
**Then** 本地可按文档化方式调用 Bitloom CLI；用户可见表面不再宣称产品名为 RHDL 或发布名为 `rhdl-rs`
**And** `just`/文档中的历史 `cargo rhdl` 引用改为 Bitloom 等价物或标明遗留（FR42, NFR17）
**And** 工作区测试（如 `just test`）在改名后仍通过

### Story 11.3: README 与公开文档 Bitloom 身份

As a 仓库访客,
I want README 以 Bitloom 自称并划清与 samitbasu/rhdl 的边界,
So that 不会误以为本仓库占用 crates.io `rhdl`。

**Acceptance Criteria:**

**Given** Story 11.2 已完成 CLI/包名改名
**When** 更新仓库 README 与必要的公开身份文档（含阶段二 PRD/addendum 中「发布名 `rhdl-rs`」的取代说明）
**Then** 首页产品名为 **Bitloom**；写明 crates.io 名 **`bitloom`**；含与 `samitbasu/rhdl` 无关的免责声明；禁止暗示 `rhdl` / `rhdl-bits`（FR41）
**And** 公开表面满足「先改名再上架」前提（NFR17）

## Epic 12: 干净结项并上架 bitloom

维护者具备可验证的发布合同后，将至少一个对外 crate 以 **0.x** 手动首发到 crates.io，并接上 Trusted Publishing + 后续自动化；诚实披露 deferred，不以冲 1.0 当结项。  
**Depends on:** Epic 11（NFR17）。

### Story 12.1: 发布成熟度合同

As a 维护者,
I want 可验证的结项/发布合同（仍可 0.x）,
So that 上架前不把未声明债务写进永久登记。

**Acceptance Criteria:**

**Given** Epic 11 公开身份已为 Bitloom / `bitloom`
**When** 落地成熟度工件与检查
**Then** 存在：`rust-version`/MSRV 1.97.1 的 CI 验证；书面 SemVer 政策（结项期停 **0.x**，不以冲 1.0 为结项）；`SECURITY.md`（或等价联系路径）；CHANGELOG + 与发布对齐的 tag 约定；README 诚实列出 deferred/non-goals（FR44, NFR15）
**And** 目标发布 crate 的 `cargo doc`（及 docs.rs 构建路径说明）可用

### Story 12.2: 首次手动发布 bitloom

As a 维护者,
I want 将至少一个对外 crate 以 `bitloom` 手动首发到 crates.io,
So that 产品有真实 registry 身份且不占用 `rhdl`。

**Acceptance Criteria:**

**Given** Story 12.1 合同已满足
**When** 发布前再探测 `bitloom` 可用，补齐 C-METADATA，执行 `cargo publish --dry-run`，再**手动** `cargo publish`
**Then** 至少一个对外包（CLI/伞形 `bitloom`）以 **0.x** 成功上架；未承诺 API 的库 crate 可为 `publish = false`（FR45, NFR16）
**And** 禁止发布 `rhdl` / `rhdl-bits`；描述/README 不暗示占用名
**And** 发布前人工复核 crates.io policies（记录于发布笔记或 PR）

### Story 12.3: Trusted Publishing 与后续自动化

As a 维护者,
I want 首发后用 OIDC Trusted Publishing + release-plz（可选 cargo-dist）,
So that 后续发版不必长期持有 crates.io token。

**Acceptance Criteria:**

**Given** Story 12.2 已完成首次手动发布
**When** 配置 crates.io Trusted Publishing，并接入 release-plz（或文档等价流水线）；可选 cargo-dist 仅用于安装器二进制
**Then** 文档化：后续版本如何经 CI 发 registry；cargo-dist 不替代 registry 发布（FR46）
**And** 至少有一次 dry-run/文档化演练证明工作流可触发（不要求本 story 内再发第二个正式版本）

---

# Phase 4 — True standalone (Bitloom after `cargo install`)

> 范围确认（2026-08-20）：**B** — 追加 Epic 13+；不重写 Epic 1–12。  
> 输入：PRD/addendum（约束继承）、ARCHITECTURE-SPINE（AD-2/6/14）、standalone research（主需求源）、naming research、AGENTS.md。无 UX。

## Phase 4 Requirements Inventory

### Functional Requirements

FR47: 设计者在**不 clone** Bitloom 工具链仓库的前提下，仅依赖 crates.io 上的公开库（用户面：`bitloom-prelude`）编写设计 crate，并能 elaborate。  
FR48: 将语言/emit 最小发布面以 **`bitloom-*`** 包名发布到 crates.io（MVP：`bitloom-hir`、`bitloom-builder`、`bitloom-macro`、`bitloom-prelude`、`bitloom-vlog`）；禁止以 `rhdl-prelude` 等 `rhdl-*` 作为对外用户依赖名；继续禁止 `rhdl` / `rhdl-bits` / `rhdl-rs`。  
FR49: 修订 AD-2（及 AD-6 中设计依赖表述）：设计 crate `[dependencies]` 唯一允许 **`bitloom-prelude`**；设计不得依赖 CLI 包 `bitloom`；可选 `[dev-dependencies]` 后续波次的 `bitloom-sim`。  
FR50: `cargo bitloom build` 的 host/shim **不得** path 依赖工具链 monorepo 的 hir/vlog；须依赖与 CLI **同版本钉死** 的已发布 `bitloom-vlog`（及传递的 `bitloom-hir`）；仍遵守 AD-14（进程内 elaborate+emit，不以 FrozenHir 文件协议替代）。  
FR51: `cargo bitloom build --package` 通过用户 `--manifest-dir` 工作区的 Cargo 元数据解析设计包（不假定仅 `examples/<name>`）。  
FR52: 提供 `cargo bitloom new <name>`（或等价脚手架），生成只依赖 `bitloom-prelude` 的最小设计 crate + elaborate 入口。  
FR53: 存在可自动化的真独立验收：空目录 + 已安装 CLI → `new` → `build` → 产出 `.v`，全程无 clone 工具链仓库。  
FR54: README / crates.io README 以真独立路径为主 onboarding；clone monorepo 降为贡献者/工具链开发文档；诚实区分「今日过渡」与「已达成」。  
FR55: （可选第二波，可另 epic）发布 `bitloom-sim`，允许用户设计 crate 以 dev-dependency 做 `tick`/VCD，仍不要求 clone monorepo。

### NonFunctional Requirements

NFR18: 发布前再探测 `bitloom-*` MVP 名在 crates.io 仍可用；若被抢注须改前缀策略并更新 AD，不得静默改用 `rhdl-*` 用户面。  
NFR19: MVP 库与 CLI **锁步同版本**发布（bindgen 风格）；已发布 crate 的 `Cargo.toml` 不得含指向 monorepo 的 `path`/`git` 用户依赖。  
NFR20: 真独立流程 MSRV 仍为 **rustc 1.97.1** / edition 2024（继承 NFR13）。  
NFR21: 文档与帮助文本持续声明与 `samitbasu/rhdl` 无关，并消歧易混淆的 `bitbloom`。  
NFR22: Trusted Publishing / release-plz 扩展覆盖新发布的 `bitloom-*` 库（或文档化等价多包发布流程）。

### Additional Requirements (Architecture / Research)

- 保留 AD-14 host shim 模式；后端依赖改为 registry 版本，不改为设计依赖 CLI（拒 AD-2/6 违规方案）。
- 拒阶段一 FrozenHir 序列化 CLI 协议。
- git 依赖脚手架仅允许作**显式过渡**，不得标为真独立 DoD。
- 目录名可暂留 `crates/rhdl-*`，但 `[package].name` 对外必须是 `bitloom-*`（实现偏好，故事中二选一写清）。
- 继承阶段三：CLI 包 `bitloom` 已发布；库此前 `publish = false`。

### UX Design Requirements

无 UI。无 UX-DR。

### Phase 4 FR Coverage Map

FR47: Epic 13 — 不 clone 即可用公开库编写并 elaborate  
FR48: Epic 13 — 发布 MVP `bitloom-*` 库族  
FR49: Epic 13 — 修订 AD-2/6 为 `bitloom-prelude`  
FR50: Epic 13 — host shim 使用 registry `bitloom-vlog`（AD-14）  
FR51: Epic 13 — `--package` 经用户 workspace metadata 解析  
FR52: Epic 13 — `cargo bitloom new`  
FR53: Epic 13 — 空目录 ATDD：install → new → build → `.v`  
FR54: Epic 13 — README 真独立为主路径  
FR55: Epic 14 — （可选）发布 `bitloom-sim` 供独立 tick

### Phase 4 NFR notes

NFR18, NFR19 → Epic 13（名称探测 + 锁步发布）  
NFR20, NFR21, NFR22 → Epic 13（MSRV、免责、Trusted Publishing 扩包）  
Epic 14 继承 NFR18–20 若发布 sim

## Phase 4 Epic List

### Epic 13: 装完就能写电路并出 Verilog
终端用户在 `cargo install bitloom` 之后，无需 clone 工具链仓库，即可 `new` 设计、依赖 `bitloom-prelude`、用 `cargo bitloom build` 得到 `.v`；公开 AD 与 README 与该路径一致。
**FRs covered:** FR47, FR48, FR49, FR50, FR51, FR52, FR53, FR54（NFR18–NFR22）
**Depends on:** Epic 11–12（Bitloom CLI 已上架）。本 epic 内库发布先于 CLI 真独立改造。

### Epic 14: 不 clone 也能 tick（可选第二波）
用户在独立设计 crate 上以 `bitloom-sim` 为 dev-dependency 做周期精确仿真 / VCD，仍不要求 clone monorepo。
**FRs covered:** FR55
**Depends on:** Epic 13（`bitloom-hir` / prelude 已发布且版本可对齐）


## Epic 13: 装完就能写电路并出 Verilog

终端用户在 `cargo install bitloom` 之后，无需 clone 工具链仓库，即可 `new` 设计、依赖 `bitloom-prelude`、用 `cargo bitloom build` 得到 `.v`；公开 AD 与 README 与该路径一致。  
**FRs covered:** FR47, FR48, FR49, FR50, FR51, FR52, FR53, FR54（NFR18–NFR22）  
**Depends on:** Epic 11–12。

### Story 13.1: 修订 AD-2/6 为 bitloom-prelude

As a 架构与实现者,
I want AD-2/AD-6 明确设计 crate 只依赖 `bitloom-prelude`,
So that 真独立发布不会再把用户面钉在 `rhdl-prelude` 或 CLI 包上。

**Acceptance Criteria:**

**Given** ARCHITECTURE-SPINE 中 AD-2 仍写设计依赖 `rhdl-prelude`，且 AD-6 图/规则使用 `rhdl-*` 名
**When** 修订 AD-2 与 AD-6（含依赖图标签）并标注决议日期
**Then** 规则写明：设计 crate `[dependencies]` 唯一允许 **`bitloom-prelude`**；设计不得依赖 CLI 包 `bitloom`；继续禁止发布/暗示 `rhdl` / `rhdl-bits` / `rhdl-rs`；内部目录可暂留 `rhdl-*` 但对外 `[package].name` 走 `bitloom-*`（FR49）
**And** `AGENTS.md` / 策略块与修订后的 AD 一致（NFR21）

### Story 13.2: 发布 MVP bitloom-* 库族

As a 终端设计者,
I want 在 crates.io 上 `cargo add bitloom-prelude`,
So that 无需 path/git 指向 Bitloom monorepo 即可编写设计。

**Acceptance Criteria:**

**Given** Story 13.1 已锁定对外名，且发布前再探测 `bitloom-{hir,builder,macro,prelude,vlog}` 仍可用（NFR18）
**When** 将 MVP 五包的 `[package].name` 设为对应 `bitloom-*`，`publish = true`，去掉已发布清单中的 monorepo `path`/`git` 用户依赖，按锁步版本执行 dry-run 后发布（NFR19）
**Then** crates.io 上可解析到同版本的 `bitloom-prelude`（传递依赖 builder/macro/hir）与 `bitloom-vlog`；文档/描述声明与 `samitbasu/rhdl` 无关（FR48, NFR21）
**And** 未纳入 MVP 的库可保持 `publish = false`；禁止发布 `rhdl` / `rhdl-bits`
**And** Trusted Publishing / release-plz（或文档等价流程）覆盖新包或写明多包发布步骤（NFR22 起步）

### Story 13.3: Host shim 使用 registry 后端并解析用户包

As a 本机用户,
I want `cargo bitloom build` 对**我的** Cargo 工作区生效且不 path 依赖工具链仓库,
So that 安装的 CLI 能真正驱动已发布的 emit 栈。

**Acceptance Criteria:**

**Given** Story 13.2 已发布同版本 `bitloom-vlog` / `bitloom-hir`
**When** 改写 `build` 生成的 host/shim：依赖改为 crates.io（或与 CLI 同版本钉死）的 `bitloom-vlog`（及 hir），并用 `cargo metadata`（或等价）在 `--manifest-dir` 解析 `--package`，不再假定仅 `examples/<name>`
**Then** host 遵守 AD-14（进程内 elaborate+emit；不以 FrozenHir 文件协议替代）（FR50, FR51）
**And** 在仅含用户设计 crate + `bitloom-prelude` 的临时工作区上，`build` 不再因缺少 monorepo `crates/rhdl-vlog` path 而失败
**And** 若仍检测到「仅有旧 path 布局且库未发布」的贡献者场景，错误信息须可读（可选）

### Story 13.4: cargo bitloom new 脚手架

As a 新用户,
I want `cargo bitloom new <name>` 生成最小设计 crate,
So that 不必手写 Cargo.toml 与 elaborate 入口。

**Acceptance Criteria:**

**Given** Story 13.2 的 `bitloom-prelude` 已可 `cargo add`
**When** 实现 `cargo bitloom new <name>`（可用内置模板；可选 cargo-generate，非必须）
**Then** 生成的包：`[dependencies]` 仅含 `bitloom-prelude`（版本与当前 CLI 文档一致）；含 `#[module]`（或现行宏表面）示例与可发现的 elaborate 入口（与 AD-19 / 现有 `rhdl_elaborate` 约定对齐或文档化迁移）（FR52）
**And** 生成物不依赖 CLI 包 `bitloom` 作为 library
**And** `--help` 描述该子命令

### Story 13.5: 空目录真独立 ATDD

As a 维护者,
I want 自动化验收「install → new → build → .v」无需 clone,
So that 回归不会再把 monorepo 绑回主路径。

**Acceptance Criteria:**

**Given** Story 13.3–13.4 已落地，且测试环境可用 MSRV **1.97.1**（NFR20）
**When** 在空临时目录运行（或 CI 等价）：使用已安装/刚构建的 `cargo-bitloom`，执行 `new` 与 `build --package … --manifest-dir . --out-dir …`
**Then** 产出非空 Yosys 友好 `.v`；全过程**不** `git clone` TangCan/bitloom（FR47, FR53）
**And** 该验收作为工作区测试或 `just`/CI job 固定下来；失败信息指向真独立合同

### Story 13.6: README 真独立主路径与发布运维收口

As a 仓库/crates.io 访客,
I want 快速开始以真独立路径为主,
So that 不会误以为必须 clone 才能出 Verilog。

**Acceptance Criteria:**

**Given** Story 13.5 验收已绿
**When** 更新 README 与 crates.io 挂载 README：主路径为 install → new → build；clone 降为贡献者章节；消歧 `bitbloom` / samitbasu/rhdl（FR54, NFR21）
**And** 文档化多包锁步发布与 Trusted Publishing 覆盖 `bitloom-*`（或显式手册步骤）（NFR22）
**Then** 公开文档不再把 `cargo run -p bitloom -- build --manifest-dir .`（monorepo）写成唯一快速开始

## Epic 14: 不 clone 也能 tick（可选第二波）

用户在独立设计 crate 上以 `bitloom-sim` 为 dev-dependency 做周期精确仿真 / VCD，仍不要求 clone monorepo。  
**FRs covered:** FR55  
**Depends on:** Epic 13。

### Story 14.1: 发布 bitloom-sim 供独立 tick

As a 设计者,
I want `cargo add bitloom-sim --dev` 后在 `cargo test` 里 `tick`,
So that 仿真也不必 clone 工具链仓库。

**Acceptance Criteria:**

**Given** Epic 13 已发布可对齐版本的 `bitloom-hir` / `bitloom-prelude`
**When** 将 sim 包以 `bitloom-sim` 发布（`publish = true`），依赖 registry 上的 hir；更新 AD-6 后端集合命名若需要
**Then** 独立设计 crate 可仅用 crates.io 依赖运行至少一条 tick（或 VCD）测试夹具（FR55）
**And** 设计 `[dependencies]` 仍不得加入 sim（仅 dev-dependencies，继承 AD-6）
**And** README 增加可选「独立仿真」小节

---

# Phase 5 — Teaching RV32 example + step-by-step tutorial

> 范围确认（2026-08-20）：**Phase 5** — 追加 Epic 15+；不重写 Epic 1–14。  
> 输入：PRD/addendum（约束继承）、ARCHITECTURE-SPINE（AD-1/2/5/6/14/15/19/20…）、RV32 technical research（主需求源）、AGENTS.md。无 UX。  
> 非目标（Episode I）：完整 SoC、MMU、Linux 可启动、以 VexRiscv/SERV 为第一课 fork。

## Phase 5 Requirements Inventory

### Functional Requirements

FR56: 仓库提供**自写**教学向 RV32 示例核（Episode I）：裁剪 **RV32I**（可无 CSR / ECALL / EBREAK / FENCE）；微架构为**单周期或多周期 FSM**（非位串行 SERV、非 Linux 向 Softcore 作为第一交付）；含简单指令/数据存储器（哈佛或统一）与最小 MMIO（如 LED/UART 形）。明确不做 MMU / Linux。  
FR57: 提供 **FemtoRV 合同式** step-by-step 教程：每步只改一件事；路径覆盖 blink/ROM → decoder → 寄存器/控制 → ALU → load/store → MMIO → 手写 asm / GNU C 可跑；**流水线（及 CSR/trap）为 Episode II**，不得塞进 Episode I 必做路径。  
FR58: 教程/示例的章节验收分层：默认 DoD 为 Bitloom 路径上的 `elaborate` +（dev）`bitloom-sim` **`tick`**（或等价断言）；仅在「宣称 ISA 子集完整」的章节才允许引入 **riscv-tests 或 RISCOF/`riscv-arch-test` 子集**作为额外门禁（该门禁是最小过滤器，非完整 DV）；可选 RVFI/`riscv-formal` 仅作进阶/对照，非 Episode I 必做。  
FR59: 打包形态为 monorepo **`examples/`**（如 `examples/rv32_*`）+ **`docs/tutorials/rv32-…`** 编号章节；教程脊柱对齐已发布的 `cargo bitloom`（`new`/`build`）与 `cargo add bitloom-sim --dev`；设计 crate `[dependencies]` 仅 **`bitloom-prelude`**；文档须说明早期章节如何在真独立路径下跟练（不得把 clone monorepo 写成唯一入口）。  
FR60: 在实现 Episode I 核之前完成**语言表面可行性闸门**：证明现行 Bitloom 宏/HIR/`tick` 合同足以表达该核所需的控制/存储端口（若缺口，先立语言/HIR 故事，不得静默缩小核范围却宣称「RV32 示例」）。  
FR61: 教程含**延伸阅读（非 fork）**：Harris DDCA Ch.7 / FemtoRV `FROM_BLINKER_TO_RISCV`；对照 PicoRV32/SERV；明确避免把 VexiiRiscv Linux 教程当作第一路径。  
FR62: README / 教程首页声明本示例的教学范围与非目标（无 SoC/MMU/Linux；Bitloom 品牌；与 `samitbasu/rhdl` 无关）。

### NonFunctional Requirements

NFR23: 继承 MSRV **rustc 1.97.1** / edition 2024；示例与教程 CI/`just test`（或等价）在该工具链上可跑（继承 NFR13/NFR20）。  
NFR24: 遵守 AD-2/6：示例设计依赖仅为 `bitloom-prelude`；仿真仅 `[dev-dependencies]` 的 `bitloom-sim`；不得依赖 CLI 包 `bitloom` 作 library；emit 仍走 AD-14 host shim。  
NFR25: 教程主路径使用 Bitloom **`cargo`/`tick` 合同**，不以 Make/SBT/PlatformIO 作为学生必装主流程（可文档化为可选外部对照）。  
NFR26: 合规门禁若启用，文档必须写明 RISCOF/arch-test 为**最小过滤器**，不得宣称等价于完整设计验证。  
NFR27: 文档与示例持续声明与 `samitbasu/rhdl` 无关，公开名 **Bitloom** / `bitloom-*`；消歧 `bitbloom`（继承 NFR21）。

### Additional Requirements (Architecture / Research)

- 继承 AD-1（先 elaborate 再 tick）、AD-5（周期精确仅 FrozenHir）、AD-15（默认单时钟 + 同步复位，除非故事显式扩展）、AD-19（`rhdl_elaborate` / 现行入口约定）、AD-20（阶段二表面：分支/同位宽/复位语义）。
- 若示例需要同步读存储器，对齐 AD-21（`Mem`/`SyncReadMem` 表面）；不得引入未立项的 Bundle/Vec 可综合路径。
- Research R1–R5：自写核 + FemtoRV 教程合同 + 分层验收 + `examples/`+`docs/tutorials` 打包 + 延伸阅读清单。
- Open question 跟踪：语言表面能否表达多周期 FSM（→ FR60）；教学用 arch-test 子集 CI 墙钟（可后置 story）。
- 无 UX / 无 UI；无新 starter template（沿用 `cargo bitloom new`）。

### UX Design Requirements

无 UI。无 UX-DR。

### Phase 5 FR Coverage Map

FR56: Epic 15 — Episode I 教学 RV32 核（裁剪 ISA、单/多周期、存储+MMIO）  
FR57: Epic 16 — FemtoRV 合同式 step-by-step 教程（流水/CSR 为 Episode II，非本 epic 必做）  
FR58: Epic 15 — 核级 DoD（elaborate + tick；可选合规）；Epic 16.2 — 教程章节级 DoD  
FR59: Epic 16 — `examples/` + `docs/tutorials/rv32-…` 打包与真独立跟练入口  
FR60: Epic 15 — 语言表面可行性闸门（先于或紧挨首个核实现）  
FR61: Epic 16 — 延伸阅读清单（Harris/FemtoRV；对照 Pico/SERV）  
FR62: Epic 16 — README/教程首页范围与非目标声明  

### Phase 5 NFR notes

NFR23–NFR27 → Epic 15–16（MSRV、AD-2/6/14、cargo/`tick` 主路径、合规门表述、品牌免责）

## Phase 5 Epic List

### Epic 15: 能跑的教学 RV32（Episode I）
学习者与维护者拥有一个**自写**的 Episode I 核：裁剪 RV32I、单周期或多周期 FSM、简单存储与 MMIO；可在 Bitloom 上 `elaborate`、用 `bitloom-sim` `tick`，并用 `cargo bitloom build` 出 `.v`。实现前通过语言表面可行性闸门。不含流水线/CSR/MMU/Linux。  
**FRs covered:** FR56, FR58, FR60（NFR23–NFR24, NFR26 按故事）  
**Depends on:** Epic 13–14（真独立 prelude / CLI / sim 已可用）

### Epic 16: Step-by-step 教程与公开入口
学习者可按编号章节（一步一变）从 blink/ROM 跟到 asm/C；文档落在 `docs/tutorials/rv32-…`，示例在 `examples/rv32_*`；主路径对齐 `cargo bitloom` + `bitloom-sim --dev`；含延伸阅读与范围/非目标声明。流水线章节仅作 Episode II 大纲或明确延期。  
**FRs covered:** FR57, FR59, FR61, FR62（NFR23, NFR25, NFR27）  
**Depends on:** Epic 15（至少有一个可引用的 Episode I 核 / 脚手架路径）

## Epic 15: 能跑的教学 RV32（Episode I）

学习者与维护者拥有一个**自写**的 Episode I 核：裁剪 RV32I、单周期或多周期 FSM、简单存储与 MMIO；可在 Bitloom 上 `elaborate`、用 `bitloom-sim` `tick`，并用 `cargo bitloom build` 出 `.v`。实现前通过语言表面可行性闸门。不含流水线/CSR/MMU/Linux。  
**FRs covered:** FR56, FR58, FR60（NFR23–NFR24, NFR26 按故事）  
**Depends on:** Epic 13–14。

### Story 15.1: 语言表面可行性闸门

As a 实现者,
I want 在写完整 RV32 核之前证明 Bitloom 表面能表达 Episode I 所需控制与存储端口,
So that 不会在实现中途才发现语言缺口。

**Acceptance Criteria:**

**Given** ARCHITECTURE-SPINE AD-18/20/21 与现行 `bitloom-prelude` 宏表面
**When** 用最小 spike（可在 `examples/` 或测试夹具）表达：多周期/FSM 控制（或已选定的单周期等价）、指令/数据存储端口、同步复位时序
**Then** spike 能 `elaborate` 且至少一条 `bitloom-sim` `tick`（或文档化的等价）通过（FR60）
**And** 若缺口存在：开出明确语言/HIR 后续故事或缩小范围的书面决议，**禁止**在未记录的情况下宣称 FR56 已满足
**And** 设计依赖仍仅为 `bitloom-prelude`（NFR24）

### Story 15.2: Episode I 核 — 译码/ALU/寄存器与 tick

As a 学习者/维护者,
I want 一个裁剪 RV32I 的 Episode I 核能执行 ALU/分支类指令并在 `cargo test` 里 tick 对齐黄金值,
So that 教学核在仿真路径上「真的在跑」。

**Acceptance Criteria:**

**Given** Story 15.1 闸门已通过（或书面接受单周期路径）
**When** 在 `examples/rv32_*`（或约定名）实现 Episode I 核：无 CSR/ECALL/EBREAK/FENCE；单周期或多周期 FSM；文档化指令子集（至少覆盖若干 R/I 与控制流；`lw`/`sw` 可延后到 15.3）（FR56）
**Then** `cargo test` 中至少两条路径：`elaborate` 成功；`bitloom-sim` `tick` 与黄金 `PortValues`/寄存器可见效果一致（FR58）
**And** `[dependencies]` 仅 `bitloom-prelude`；`bitloom-sim` 仅在 `[dev-dependencies]`（NFR24）
**And** 明确非目标：无 MMU/Linux/流水线（FR56）

### Story 15.3: 访存、MMIO 与 emit `.v`

As a 学习者,
I want 核支持 load/store 与最小 MMIO，并能 `cargo bitloom build` 出 Yosys 友好 `.v`,
So that 教程后半段与真独立 emit 路径打通。

**Acceptance Criteria:**

**Given** Story 15.2 的核与 tick 夹具存在
**When** 增加简单存储（哈佛或统一）与最小 MMIO（如 LED/状态寄存器形）；补 load/store（及必要的访存 tick 测试）（FR56）
**Then** `cargo bitloom build --package <rv32_example> …` 产出非空 `.v`（Yosys 友好子集）（FR58）
**And** 至少一条 tick/测试覆盖 MMIO 或 mem 可见副作用
**And** 仍无 CSR/trap/MMU/Linux

### Story 15.4: 子集完整声明与可选合规门禁

As a 维护者,
I want 在宣称「子集完整」时挂上最小合规过滤器并写清其含义,
So that 教学完整与完整 DV 不被混淆。

**Acceptance Criteria:**

**Given** Story 15.2–15.3 已文档化指令子集
**When** 增加「子集完整」检查：或 (a) 仓库内黄金程序集 + tick/emit 门禁，或 (b) 可选 `riscv-tests` / RISCOF 子集 CI job（可 `ignore`/nightly）（FR58）
**Then** 文档写明该门禁是**最小过滤器**，非完整 DV（NFR26）
**And** 未启用 (b) 时不得在 README 宣称已通过 arch-test
**And** RVFI/`riscv-formal` 若出现仅为可选进阶，非本 story 必做

## Epic 16: Step-by-step 教程与公开入口

学习者可按编号章节（一步一变）从 blink/ROM 跟到 asm/C；文档落在 `docs/tutorials/rv32-…`，示例在 `examples/rv32_*`；主路径对齐 `cargo bitloom` + `bitloom-sim --dev`；含延伸阅读与范围/非目标声明。流水线章节仅作 Episode II 大纲或明确延期。  
**FRs covered:** FR57, FR59, FR61, FR62（NFR23, NFR25, NFR27）  
**Depends on:** Epic 15。

### Story 16.1: 教程骨架与真独立跟练入口

As a 新学习者,
I want 有编号教程目录与「如何跟练」的第 0 章,
So that 不必猜路径或只能 clone monorepo。

**Acceptance Criteria:**

**Given** Epic 15 至少提供可引用的 `examples/rv32_*`（或文档化占位名）
**When** 创建 `docs/tutorials/rv32-…/`（或等价）含索引 + Chapter 0：工具链（MSRV 1.97.1）、`cargo install bitloom` / workspace `cargo bitloom`、`bitloom-sim --dev`、指向示例包（FR59, NFR23, NFR25）
**Then** Chapter 0 说明真独立路径与贡献者 monorepo 路径的差异，**不得**把 clone 写成唯一入口（FR59）
**And** 索引列出后续章节与每章 DoD（elaborate / tick / build 之一）

### Story 16.2: 递进教程正文（blink → asm/C）

As a 学习者,
I want 按 FemtoRV 合同一步一变跟到能跑 asm/C,
So that 每步都有可验证结果。

**Acceptance Criteria:**

**Given** Story 16.1 骨架存在，且 Epic 15 核可 tick/build
**When** 编写编号章节覆盖：blink/ROM 概念 → decoder → 寄存器/控制 → ALU → load/store → MMIO → 手写 asm 与 GNU C（可简化工具链步骤）（FR57）
**Then** 每章只引入一类变化，并写明该章验收（测试名或命令）（FR57）
**And** 主命令为 Bitloom `cargo`/`tick`/`build`，不以 Make/SBT 为必装（NFR25）
**And** 流水线与 CSR **不**作为这些章节的必做内容（FR57）

### Story 16.3: Episode II 大纲（延期声明）

As a 学习者,
I want 看到流水线/CSR 被明确标为 Episode II,
So that 不会误以为 Episode I 未完成。

**Acceptance Criteria:**

**Given** Story 16.2 正文存在
**When** 增加 Episode II 大纲页（或 README 小节）：主题含流水 hazard / 可选 Zicsr·M-trap；状态为 **延期/大纲**（FR57）
**Then** 不要求本阶段实现流水核或 CSR
**And** 指向 Harris/FemtoRV Episode II 类外部阅读（可与 16.4 交叉链接）

### Story 16.4: README 入口、延伸阅读与范围声明

As a 仓库访客,
I want README/教程首页写清范围、非目标与延伸阅读,
So that 不会误走 Linux Softcore 或混淆品牌。

**Acceptance Criteria:**

**Given** Story 16.1–16.3 文档就位
**When** 更新 README（及教程首页）：链到 `docs/tutorials/rv32-…`；声明教学范围与非目标（无 SoC/MMU/Linux）（FR62）
**And** 延伸阅读：Harris DDCA Ch.7、FemtoRV `FROM_BLINKER_TO_RISCV`；对照 PicoRV32/SERV；**避免** Vexii Linux 作第一路径（FR61）
**Then** 持续声明 Bitloom / 与 `samitbasu/rhdl` 无关（NFR27）
