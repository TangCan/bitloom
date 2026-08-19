---
id: SPEC-rhdl
companions:
  - language-surface.md
  - later-product.md
  - ../../planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md
sources:
  - ../../../docs/requirements
---

> **Canonical contract.** This SPEC and the files in `companions:` are the complete, preservation-validated contract for what to build, test, and validate. Source documents listed in frontmatter are for traceability — consult them only if you need narrative rationale or prose color this contract intentionally omits.

# RHDL

## Why

**痛点 + 愿景。** FPGA/ASIC 设计者用 Verilog/VHDL 时，位宽、多驱动、锁存器和跨时钟域往往拖到综合或仿真才爆；用 Chisel 能提高构造力，但离开 Scala/FIRRTL 工具链，且不能把 `.fir` 解析回可维护源码。RHDL 要让硬件描述就是合法 Rust：在发出网表之前拦住这类错误，用 Cargo 做项目和测试，用 Verilog 接入现有后端，用 FIRRTL 文本与 Chisel 生态交换模块级 IP。多视图让同一模块既能手写快速功能模型，又能跑周期精确 `tick`，而不从网表生成 TLM。

## Capabilities

- **CAP-1**
  - **intent:** 设计者用合法 Rust 描述 RTL 模块：硬件标量类型、带方向的端口、显式组合逻辑与时序逻辑。
  - **success:** 一个含组合+时序的模块能 elaborate；不完整组合赋值被拒绝；端口必须是 `Input<T>` / `Output<T>`。

- **CAP-2**
  - **intent:** 设计者用 const 泛型写一份源码、实例化多种位宽。
  - **success:** 同一模块在两种宽度下发出的 Verilog 端口宽度不同且与参数一致。

- **CAP-3**
  - **intent:** 设计者层次化实例化子模块并连接端口。
  - **success:** 位宽或方向不匹配、未驱动输入（未标悬空）在发出网表前失败。

- **CAP-4**
  - **intent:** 设计者从同一描述得到可综合、Yosys 可读的 Verilog，接入现有 FPGA/ASIC 流程。
  - **success:** 示例设计产出 `<abi>.v`；Yosys 能读取（Yosys 为可选对照，不是必装门槛）。

- **CAP-5**
  - **intent:** 设计者在 Cargo 测试里对同一描述做周期精确仿真，并导出波形。
  - **success:** 测试先 elaborate 再 `tick`；能 dump VCD；小型计数器与黄金值一致。

- **CAP-6**
  - **intent:** 设计者可为模块手写功能视图，并与周期精确行为对照。
  - **success:** 对照只比较 `PortValues`；`#[functional_state]` 不出现在冻结电路里。禁止从 HIR 生成 TLM。

- **CAP-7**
  - **intent:** 非法硬件在发出 Verilog/仿真前被拒绝，错误指向源码。
  - **success:** 多驱动、位宽/方向、未驱动输入、周期精确路径上的不可综合构造均产生结构化诊断（`rhdl::E0xxx`），不 `panic`。

- **CAP-8**
  - **intent:** 设计者在本工具链的冻结电路与 FIRRTL 6.0.0 文本之间交换可逆子集，以对接 Chisel/firtool 生态。
  - **success:** HIR → `.fir` → HIR 在模块层次、公开端口名/宽/向、实例图、ground 运算、寄存器上相等。不承诺可维护的 Chisel Scala，不依赖 Chisel 解析 `.fir`。

- **CAP-9**
  - **intent:** 设计者把设计当普通 Cargo 包，用本机 CLI 展开并生成产物。
  - **success:** `#[rhdl::top]` 是唯一发现入口；`cargo rhdl build` 在本机完成 elaborate 与 emit；无云端控制面。

## Constraints

- 硬件描述是合法 Rust 嵌入式 DSL，没有独立语言或解析器。
- 显式优于隐式：`#[combinational]` / `#[sequential]` 强制；禁止推断 latch；端口用方向包装类型；无隐式全局时钟；表面算术与连接严格同位宽，扩展/截断必须显式。
- 周期精确 / 综合路径遵守可综合子集；功能视图可用堆等软件构造。`#[functional_state]` 不得进入 HIR。
- 多驱动、完整性、宽/向错误在冻结电路时判定。Rust 所有权可引导 API，不是声音性证明。
- 互转契约是 FrozenHir ↔ 带 `FIRRTL version 6.0.0` 头的文本。导入须把 last-connect 收成唯一驱动。细节与钉死版本见架构脊柱。
- HOW（crate 图、FrozenHir 模式、host crate、firtool 钉死）以架构脊柱 AD-1…AD-19 为准；本 SPEC 不重写那些规则。

## Non-goals

- 独立 HDL 语法与自定义解析器。
- 以可维护 Chisel Scala 源码为互转契约；把 Chisel 当 `.fir` 解析器。
- 从 HIR 降低 TLM-2.0 / 无定时功能模拟器。
- 向 crates.io 发布名 `rhdl` 或 `rhdl-bits`。
- 用所有权/线性类型作为多驱动的声音性证明。
- 云端托管工具链；默认信任 PATH 上的 firtool。
- 自研 HLS 调度器。
- **阶段一交付范围内不做**多时钟 HIR、Analog/InOut/Mem、可视化 IDE、形式验证/SVA、C ABI、FST、HLS 等；这些能力已升格为阶段二需求，见 [`../../planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md`](../../planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md) 与 [`later-product.md`](later-product.md) 索引。

## Success signal

设计者写一个带 const 泛型的时序模块，`cargo rhdl build` 产出 Yosys 可读的 `.v`，`cargo test` 能 `tick` 并 dump VCD；可选手写功能模型与 `tick` 在 `PortValues` 上一致。FIRRTL 可逆子集往返保住层次与端口。文档首页声明与 `samitbasu/rhdl` 无关。

## Assumptions

- 阶段一可演示成功不要求 FIRRTL 导出或 firtool 降出的第二份 Verilog。
- 双视图的「拆开」现在就做（手写功能 + `tick`）；TLM 桥与 mixed `both` 仿真属阶段二（见阶段二 PRD FR29 等）。
- 阶段二及以后的 FR/NFR 以阶段二 PRD 为权威；本 SPEC 仍是阶段一 CAP 合同。