---
id: SPEC-rhdl
companions:
  - language-surface.md
  - later-product.md
  - ../../planning-artifacts/architecture/architecture-rhdl-2026-08-18/ARCHITECTURE-SPINE.md
  - ../../planning-artifacts/prds/prd-rhdl-2026-08-19/prd.md
sources:
  - ../../../docs/requirements
---

> **Canonical contract.** This SPEC and the files in `companions:` are the complete, preservation-validated contract for what to build, test, and validate. Source documents listed in frontmatter are for traceability — consult them only if you need narrative rationale or prose color this contract intentionally omits.

# Bitloom（工作区 rhdl）

## Why

**痛点 + 愿景 + 概述字面闭环。** FPGA/ASIC 设计者用 Verilog/VHDL 时，位宽、多驱动、锁存器和跨时钟域往往拖到综合或仿真才爆；用 Chisel 能提高构造力，但离开 Scala/FIRRTL 工具链。公开产品 **Bitloom**（crates.io / CLI：`bitloom`）让硬件描述就是合法 Rust：在发出网表前拦住这类错误，用 Cargo 做项目与测试，用 Verilog 接入现有后端，用 FIRRTL 文本与 **可编译 Chisel Scala** 双向对接生态，并交付双模拟器生成、一级 IP、内置层次/时序可视化与外挂 HLS 产品路径——使 `docs/requirements` 概述愿景可按字面验收，而非「尽力 / deferred」冒充完成。

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
  - **intent:** 设计者为模块提供功能视图（手写或工具链生成），并与周期精确行为对照。
  - **success:** 对照比较 `PortValues`；`#[functional_state]` 不出现在冻结电路里。允许生成 **Rust** 功能模拟器 crate（FR47）；**不**要求 SystemC TLM-2.0。

- **CAP-7**
  - **intent:** 非法硬件在发出 Verilog/仿真前被拒绝，错误指向源码。
  - **success:** 多驱动、位宽/方向、未驱动输入、周期精确路径上的不可综合构造均产生结构化诊断（`rhdl::E0xxx` / Bitloom 等价码），不 `panic`。

- **CAP-8**
  - **intent:** 设计者在本工具链的冻结电路与 FIRRTL 6.0.0 文本之间交换可逆子集（对接 firtool）。
  - **success:** HIR → `.fir` → HIR 在模块层次、公开端口名/宽/向、实例图、ground 运算、寄存器上相等。不依赖 Chisel 解析 `.fir`。Chisel Scala 产品路径见 **CAP-12**。

- **CAP-9**
  - **intent:** 设计者把设计当普通 Cargo 包，用本机 CLI 展开并生成产物。
  - **success:** `#[bitloom::top]` / 过渡期 `#[rhdl::top]` 是发现入口；`cargo bitloom build` 在本机完成 elaborate 与 emit；无云端控制面。

- **CAP-10**
  - **intent:** 设计者用参数化复合类型（`Bundle` / `Vec<T,N>` 或文档等价）构造可复用硬件聚合。
  - **success:** 文档化表面可 elaborate / emit / tick；位宽或方向错误在 emit 前失败（FR51）。

- **CAP-11**
  - **intent:** 设计者显式绑定 ClockDomain（或等价）并强制合法跨域。
  - **success:** 文档与夹具展示时钟/复位极性与同步·异步绑定；跨域无显式同步器则 freeze 失败（FR52 / FR23）。

- **CAP-12**
  - **intent:** 设计者在 Bitloom 与 Chisel 之间做双向可维护源码级互操作（以 FIRRTL 为桥）。
  - **success:** (1) FrozenHir/`.fir` → **可编译** Chisel Scala + 端口/层次往返谓词（机械风格可接受）；(2) Chisel 或 `.fir` → Bitloom 表面或 FrozenHir 再 emit，对称谓词通过；(3) 文档化混合设计夹具进入同一后端。NFR10 调试再生不得冒充本能力（FR28 / FR46 / AD-27）。

- **CAP-13**
  - **intent:** 工具链生成功能模拟器与周期精确模拟器，并桥接/等价检查。
  - **success:** CLI/API 生成 Rust 功能模拟器 crate 与周期精确模拟器工件；经桥接或对照运行；与 FR30 联验；故意破坏等价则 fail（FR47）。

- **CAP-14**
  - **intent:** 设计者例化一级官方 IP：UART、SPI、I2C、FIFO、AXI。
  - **success:** 五类均可经 `bitloom-prelude` 依赖例化；各至少一 smoke：elaborate → emit → tick（或文档等价）。AXI = AXI4-Lite 最小从接口达标；保留黑盒路径（FR37 / FR48）。

- **CAP-15**
  - **intent:** 设计者从产品入口获得模块层次图与时序图（或等价交互视图）。
  - **success:** 同一夹具可生成层次视图与时序/等价视图；不得以「仅用户自开 GTKWave」为唯一路径（FR38 / FR49）。

- **CAP-16**
  - **intent:** 设计者走算法级 Rust → RTL 的 HLS 产品路径。
  - **success:** 默认文档路径下钉死单一外挂调度后端（Bambu 或 Vitis/XLS）对夹具可复现产出可综合 RTL；CI/烟测覆盖；不可永久 unsupported；无树内自研 scheduler（FR35 / FR50 / AD-25）。

## Constraints

- 硬件描述是合法 Rust 嵌入式 DSL；没有独立语言或解析器。
- 显式优于隐式：`#[combinational]` / `#[sequential]` 强制；禁止推断 latch；端口用方向包装类型；无隐式全局时钟；表面算术与连接严格同位宽，扩展/截断必须显式。
- 周期精确 / 综合路径遵守可综合子集；`#[functional_state]` 不得进入 HIR。功能视图可为手写或生成的 Rust。
- 多驱动、完整性、宽/向错误在冻结电路时判定。Rust 所有权可引导 API，不是声音性证明。
- FIRRTL 文本契约仍是 FrozenHir ↔ 带 `FIRRTL version 6.0.0` 头的文本（AD-3）。Chisel Scala 产品路径是并列合同（AD-27），不替代 AD-3。
- 公开发布名与 CLI 为 **Bitloom** / **`bitloom`**；设计 crate 只依赖 **`bitloom-prelude`**；禁止向 crates.io 发布 `rhdl` 或 `rhdl-bits`。
- HOW（crate 图、FrozenHir、host、firtool/Chisel 钉死）以架构脊柱 **AD-1…AD-28** 为准；本 SPEC 不重写那些规则。
- **NFR14 / AD-28：** FR46/47/48/49（及适用的 FR50）在 epic/story 标 `ready` 前须有风险记录（上游约束、粗工期带、禁止静默降级清单、负责人）；缺记录不得开工。历史别名 **FR46-tp** / **NFR14-crates** ≠ 本 PRD 的 FR46 / NFR14。
- 相位、FR 细目与旅程以 adopted PRD companion 为准；本 SPEC 的 CAP 是跨阶段 WHAT 合同。

## Non-goals

- 独立 HDL 语法与自定义解析器。
- 把 Chisel 当 `.fir` 解析器；恢复已删除的 Scala `firrtl.Parser` API。
- 以 SystemC TLM-2.0 为默认/强制功能模拟合同。
- 要求 idiomatic 手写 Chisel 风格（机械可编译即可）。
- 向 crates.io 发布名 `rhdl` 或 `rhdl-bits`。
- 用所有权/线性类型作为多驱动的声音性证明。
- 云端托管工具链；默认信任 PATH 上的 firtool。
- 树内自研 HLS 调度器 / allocation。
- 用 NFR10 调试用 HIR→源码再生冒充 CAP-12。
- 无 NFR14 记录却把概述字面 FR 标 ready，或以「尽力失败 / deferred」静默降级合同。

## Success signal

设计者用 Bitloom 写带 const 泛型的时序模块，`cargo bitloom build` 产出 Yosys 可读 `.v`，`cargo test` 能 `tick` 并 dump VCD；功能视图（手写或生成）与 `tick` 在 `PortValues` 上可对照。FIRRTL 可逆子集往返保住层次与端口。P3 闭环时：CAP-10…16 各有自动化或文档化黄金验收；Chisel 双向、双模拟器生成、五类一级 IP、内置层次+时序可视化、HLS 默认路径均不以「尽力」交差。文档声明与 `samitbasu/rhdl` 无关。

## Assumptions

- 概述 `docs/requirements` 文件本轮未改字；合同品牌与 FR 以 PRD/本 SPEC 为准，概述改写另任务。
- FR46 可通过「生成 Scala + 导入 `.fir`/CIRCT」满足，不恢复已删 Parser；上游删除 API 不自动豁免验收。
- 外挂 HLS 调度可接受；启用时钉死单一后端。
- Launch（P0+P1+Bitloom 身份）可不阻塞于 P3；但「概述完全完成」声明必须 CAP-10…16 / FR46–52 收口。
