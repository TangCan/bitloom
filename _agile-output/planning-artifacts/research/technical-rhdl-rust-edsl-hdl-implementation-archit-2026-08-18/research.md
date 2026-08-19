---
title: 'technical research: RHDL Rust eDSL HDL implementation architecture'
type: 'technical'
topic: 'RHDL Rust eDSL HDL implementation architecture'
decision: 'Ground a feasible 2026 implementation architecture for a Rust-embedded RTL HDL (proc-macro to HIR to Verilog/FIRRTL, with Chisel interop and multi-view simulation) and produce a Phase-1-2-first implementation scheme.'
source: 'native-run'
status: complete
preset: 'deep'
validation: 'normal'
created: '2026-08-18'
updated: '2026-08-18'
verified_claims: 3
unverified_claims: 8
---

# technical research: RHDL Rust eDSL HDL implementation architecture

**Decision this research serves:** 把一份 Rust 嵌入式 RTL HDL（过程宏 → HIR → Verilog/FIRRTL，含 Chisel 互转与多视图仿真）落到 2026 年可执行的实现方案。

## 执行摘要

**现在该做的事：** 按 **Chisel 式分阶段展开 + 自有 HIR** 实现本仓库的 RHDL；阶段一直接出可综合 Verilog；阶段二再出 **FIRRTL 6.0 子集** 并 **钉死 firtool-1.155.0**。把与 Chisel 的关系改成 **单向导出 + 有损导入**，不要承诺源码级往返。多视图用 **两套模型（功能 Rust + 周期精确 HIR）**，不要从同一 IR 生成 TLM。crates.io 上的名字 `rhdl` 已被占用，发布时必须换 crate 名。

驱动该结论的三条证据：

1. 2026 年生产级 FIRRTL 路径是 **Chisel 7.14.0 → firtool-1.155.0**；Scala FIRRTL 编译器已归档。Chisel **不能**把 `.fir` 解析回 `Circuit`（[#4899](https://github.com/chipsalliance/chisel/issues/4899) 仍开）。[16][17][18][24]
2. 同名项目 `samitbasu/rhdl` 走 **RHIF → Verilog**，README 与 LATTE’25 **不提 FIRRTL**；crates.io `rhdl` 0.1.0 是 2023-09 的 3 行占位包，不是当前编译器。[1][3][8][32]
3. 业界双视图是 **两套源模型或 TLM↔管脚桥**；从同一模块 IR 生成 TLM-2.0 的生产工具未找到。XLS 的 JIT+Verilog 最接近，但仍标 experimental；Z3 LEC 覆盖函数/网表而非 proc/channel。[36][37][40][41]

**最大保留：** 过程宏无法共享状态、rust-analyzer 看不到中端——这是作者自己标出的结构风险；阶段一必须把 HIR 构建成可独立调试的 crate，而不是把语义全塞进宏。[3][46]

需求文档用于框定问题（要 FIRRTL、Chisel 互转、所有权防多驱动、多视图），**不作为证据**。下文实现方案是研究结论对需求的映射。

## 景观与成熟度（Rust HDL）

Rust **嵌入式 eDSL** 没有收敛。2026 年仓库活跃度最高的是 **独立语言**：Veryl（Rust 实现、面向可读 SV，v0.20.2 于 2026-07-01）与 Spade（独立 HDL + Swim，Hackaday 2025-04 仍称 WIP）。[9][10][11][12]

`samitbasu/rhdl` 是 rust-hdl 的活继承者：`main` HEAD 2026-02-03，仓库 `pushed_at` 2026-04-21（日期分裂未完全解释），未归档。[2] 设计目标是“就是 Rust”、payload 枚举、更快仿真；清单仍未勾选 widget/BSP 移植。[3] crates.io `rhdl` **0.1.0**（2023-09-02）仅 3 行 Rust、无 lib，**不是** workspace 编译器；`rhdl-core` 未发布。[1][8] 第三方 `zebreus/a5-1-rhdl` 称只能 path 依赖、文档几乎等于编译器源码。[47]

`rust-hdl` 0.46.0 停在 2023-07-02；宏 panic（信号名 `output`）与 Signed 仿真溢出在 2025 仍开。[4][5][6] `kaze` **已归档**（`archived: true`），crates.io 最后发布 2021-03。[7][51] Git 上的 `crates/rhdl` workspace 是 2024 edition、path 依赖若干 crate，与 crates.io 占位包不是同一产物。[52] Cement/`cmtrs` 是 Rust→FIRRTL 的成文路径，但 crates 停在 2025-01-09、git push 停在 2025-01-16。[13][14][15]

`VHDL-LS/rust_hdl` 是 VHDL 语言服务器，不是 HDL。[48]

**对本项目：** 不要 fork `samitbasu/rhdl` 当产品（无 FIRRTL、crate 名冲突、BSP 未完成）。可借鉴其 RHIF/原生仿真，但 FIRRTL 一级目标必须另建。

## 景观与成熟度（编译器 IR）

Chisel 自 v3.6 用 CIRCT `firtool` 出 Verilog；`chipsalliance/firrtl` 于 2024-08-20 归档。[19][20] 当前配对：**Chisel 7.14.0（2026-08-13）钉死 firtool-1.155.0**（发布说明 + `etc/circt.json`）。版本页上的 `llvm-firtool` 1.153.0 是 **sbt 覆盖示例**，不是该配对。[16][17][18][23]

FIRRTL 语言 spec **v6.0.0**（2026-05-12）；3.x–5.x 有破坏性语法；属性 ABI 仍实现定义。[21][22] CIRCT FIRRTL dialect 宣称完整实现 spec + CHIRRTL；早期“更 MLIR 原生”的类型规范化已被撤回，因无法与 SFC 对齐。[19]

SpinalHDL、Clash 1.10.0（2026-04-23）、Amaranth 仍走私有 IR → Verilog/VHDL/RTLIL，**没有 CIRCT 前端**。[26][27][28]

**对本项目：** 2026 年真正的跨 HDL 交换物是 **Verilog**；FIRRTL 只对 Chisel/`firtool`/Chipyard 有意义。不要把 CIRCT HW dialect dump 当生态 ABI。

## 互操作

FIRRTL 是 **单向 lowering**（CHIRRTL → low FIRRTL → HW → SV），不是双向 HDL。[19][24] Chisel 可序列化 `.fir`，**不能解析回来**；维护者建议 `firtool -parse-only`、对象序列化或 firtool pass plugin。[24]

元数据死亡点（文档化）：annotation 是旁路 JSON；`_` 前缀名可丢；bundle lowering 强制改名（这是公开 ABI）；源定位只剩 Verilog 注释；property 无 ABI。[19][22][25]

默认 firtool SV **不能直接喂 Yosys**（需 `disallowLocalVariables,disallowPackedArrays`）。Verilator 有 location 注释与 `automatic logic` 坑。`--split-verilog` 与“单一输出文件”的 cargo/Bazel 声明冲突。[25]

Cargo HDL 的失败模式是 **过期 firtool**：Cement 测试 **firtool-1.86.0**，落后当前 1.155.0 约一年半。[14] 钉死方式：CIRCT `firrtl-bin-*.tar.gz` 或 Maven `org.chipsalliance:llvm-firtool:1.155.0`（FNDDS 平台 classifier），不要信任系统 PATH。[16][44][45]

## 架构模式

生产级规模在 **宿主语言展开图 + 独立硬件编译器**（Chisel/Spinal），不在“把 Rust 子集翻译成 Verilog”的 kernel 宏。[3][4] rust-hdl 先死在表达力（`.val()`/`.next`、C 式 enum、循环限制），再死在仿真速度；作者把过程宏“不能共享状态、没有上下文”列为结构风险。[3][4]

cmt2 明确选择 **过程宏而非 rustc plugin**（对比 HazardFlow）：宏划清 DSL 与宿主边界。[33] 前端应 **记录 HIR**，而不是复用 rustc 中端。

LATTE’25：samitbasu RHDL 是 rustc **旁路 co-compiler**，RHIF（有类型 SSA VM）→ RTL → Verilog，文中 **无 FIRRTL、无仿真数字**。[32] Cement2 后续 arXiv（2025-11）给出 FPGA 表：CMT2-RV 相对 Sodor LUT 0.82×、Fmax 377 MHz（Vivado 2024.1，XCVU9P），后端 **firtool-1.108.0**。[34]

**所有权防多驱动：** 未找到生产级 Rust HDL 用 ownership/线性类型禁止多驱动。CirQTS（ECOOP 2023，Idris2）把线性用在 SV 网表上，半加器门数翻倍，作者认为线性对普通 RTL 过严。[29] Hardcaml 在 ** elaboration 时** 检查唯一驱动与位宽。[49] **阶段一应做图上唯一驱动检查，不要赌 Rust 所有权语义。**

**时钟域：** Clash 的 `Signal dom a` 能在类型上拦住跨域，但同一 `dom` 仍可能接无关时钟；跨域原语是 `unsafeSynchronizer` + FIFO。[30] Chisel `withClock` 是动态作用域，**不在 Data 上类型化 CDC**。[31] LATTE RHDL 用 `Signal<T, Color>` 标记类型。[32] **阶段三用 phantom domain，不要复制 withClock。**

## 落地现实

过程宏 HDL 在 2026 继承 rust-analyzer 的展开延迟与“crate 未编译则无法展开”。[46] rust-hdl 的 `custom attribute panicked` 仍开。[5] 编译期塞 Verilator（`zebreus/rust_hls`）被作者称为性能极差、兼容噩梦。[未作为推荐依据：单作者实验]

MINRES 把五级 RISC-V 用 RHDL 跑到 Trenz TEC0117（Yosys 在 crate 外），**无 LUT/MHz**；不支持负边沿写回，改了原 VHDL 设计。[35] 未找到独立仿真加速数字（README“快 1–2 个数量级”为作者自述，low）。[3]

## HLS 与多视图

XLS：DSLX + 数据流 SSA + JIT + Verilog，项目自称软硬件功能等价；**仍 experimental、无兼容承诺**。Z3 LEC 文档是 **IR 函数 / 网表**；proc 用另一套 `eval_proc_main`。[36][37]

Vitis HLS 活着但锁 AMD 与 IDE 变迁。Bambu/Dynamatic 是学术开源；Dynamatic 作者记录 MLIR-on-HLS 与 XLS 日更导致翻译层过时的风险。[56]

SystemC TLM（IEEE 1666-2023）是工业双视图契约；落地是 **桥**（`libsystemctlm-soc`：QEMU ↔ TLM ↔ Verilator RTL），不是单 IR 生成 TLM socket。[40][41] Verilator `--sc` 是管脚级 `SC_MODULE`，不是 TLM。[38] Codasip 用 **IA 与 CA 两套 CodAL**。[50] Chisel 7 的 ChiselSim 是对生成 SV 的 peek/poke，不是生成的无定时 golden。[42]

essent 钉在 Scala FIRRTL **1.6.1** 且明确不再升；**不能**当 spec 6.0.0 仿真后端。[43] Verilator 5.050（2026-07-01）与 Yosys 0.68（2026-08-05）是更稳的周期精确/综合后端。[38][39]

## 跨维洞察

- **名字与架构双重碰撞：** 需求里的“RHDL + FIRRTL 一级”与现存 `samitbasu/rhdl`（RHIF、无 FIRRTL）不是同一条产品线。继续用 crate 名 `rhdl` 会在 crates.io 与论文检索上持续误导。
- **FIRRTL 不能同时满足“与 Chisel 互转”和“无损往返”。** 互转若定义为 `.fir` → Chisel 源码，2026 年上游已拆除解析器。若定义为 `.fir` → firtool → SV 与 Chipyard 拼接，则可行，但那是 **导出**，不是往返。
- **过程宏 + FIRRTL 钉版本是同一类运维税。** Cement 证明能出 FIRRTL，也证明 firtool 会冻结在 1.86.0。本项目若把 FIRRTL 当一级目标，必须从第一天做 **托管二进制**，否则会重复该失败。
- **多视图与 HLS 被需求放在后期是对的，但机制写错了。** 需求写“从同一模块生成功能/周期精确模拟器”；证据显示生成功能视图的成功案例是 **另一套源**（XLS 的 DSL 解释器、CodAL IA、手工 `#[functional_model]`），不是 HIR 降 TLM。

## 相反证据

- rust-hdl README 称有商业 FPGA 固件出货——**仅作者自述、无料号/LUT/数量**；且出货的是 rust-hdl 不是 RHDL。[4][3] 不据此认为嵌入式 Rust HDL 已验证到 ASIC。
- Cement2 FPGA 数字显示规则型 Rust eDSL **可以**达到与 Chisel Sodor 可比的 QoR。[34] 这支持“Rust 前端 + FIRRTL 后端”可行，**不支持** Cement 的 firtool 钉死策略或当前维护状态（19 个月无内容 push）。[13]
- XLS 确实从同一设计跑宿主与 Verilog。[36] 若本项目改做 HLS 语言，应评估 XLS 而不是自研调度器。需求定位是 RTL eDSL，故不采纳“先做 XLS 前端”。

Red-team 本轮关闭；以上为检索中自然出现的反例。

## 建议（实现方案）

每条绑定决策；置信度写在句内。需求编号仅作映射，不是引用。

### 架构脊骨（喂 architecture）

1. **前端（高）：** `#[module]` 过程宏 **记录** 端口/实例/连接，组合/时序体用受限 Rust 子集生成 HIR 节点——Chisel Builder + cmt2 宏边界，而不是 rust-hdl 式“内核必须是合法 Rust 且被翻译成 Verilog 字符串”。HIR crate 与宏 crate 分离，避免宏内不可调试。[3][33]
2. **HIR（高）：** 自研结构化 HIR（模块/端口/线网/寄存器/时钟域元数据），语义向 FIRRTL 对齐，但 **前端不发射 CIRCT MLIR**。[19] 阶段二增加 `FIRRTL version 6.0.0` 文本导出（避开 CHIRRTL 特有 mem 形态，除非要嵌 Chisel 标准库）。[21][22]
3. **Verilog 阶段一（高）：** HIR → 可综合 Verilog/SV，用 firtool 同款 lowering 纪律喂 Yosys（禁止 packed array / local `automatic`）。[25] 不要把 sv2v 当主路径。
4. **FIRRTL 阶段二（高）：** 导出后 **shell 出钉死的 firtool-1.155.0**（CIRCT tarball 或 `llvm-firtool` Maven 平台 jar）。cargo 子命令模仿 Chisel：可覆盖路径，默认不读系统 firtool。[16][18][44][45]
5. **仿真（高）：** 周期精确：HIR 解释/编译为 Rust（cargo test + VCD）。Verilog 对照：Verilator 5.x，**构建脚本调用**，不要编进过程宏。[38][3] 功能视图：需求已写的 `#[functional_model]` 走普通 rustc——保持这条，不要从 HIR 生成 TLM。[40][41]
6. **多驱动（高）：** 阶段一在 HIR 上做唯一驱动 + 位宽 + 方向，Hardcaml 式 elaboration 检查。[49] 所有权模型可作 API 引导（`Wire` 移动语义），**不作为声音性证明**。[29]
7. **CDC（中）：** 阶段三用 Clash 式 phantom `Signal<D, T>` + 库内 DoubleFlop/SyncFIFO；禁止 Chisel 式仅 `withClock` 作用域。[30][31][32]

### 互转（喂 roadmap）

8. **导出（高）：** `cargo rhdl emit --format firrtl|sv`。FIRRTL 供 firtool/Chipyard 级拼接；**不要**宣称生成可维护的 Chisel 源码。[24]
9. **导入（中，阶段四降级）：** 用 `firtool -parse-only` / `circt-opt -import-firrtl` 进 CIRCT，再降到本 HIR 的 **可逆子集**（标量端口、无 property、无 CHIRRTL mem）。输出 RHDL 源码仅为调试，不保证美化往返。[19][24]
10. **需求修正（高）：** 将“与 Chisel 双向互转”改为“**FIRRTL 导出一级；Chisel 源码导入不做；FIRRTL 导入为有损子集**”。否则阶段四会按已拆除的 Scala parser 规划。

### 产品与生态（喂 brief / 风险）

11. **命名（高）：** 不要在 crates.io 发布 `rhdl`。建议 workspace 名与 GitHub 组织显式区分（例如 `rhdl-lang` / `rhdlrs`），文档首页声明与 `samitbasu/rhdl`、Veryl、Spade 的关系。[1][8][9]
12. **HLS（高）：** 阶段五不要自研调度器。可选：标注算法走 **Bambu（LLVM IR）** 或评估 XLS；承认 XLS 无兼容承诺。[36][56]
13. **IP / 可视化 / IDE：** 维持需求阶段五–六，但 IP 必须 **双模型手写**（功能 + RTL），一致性用随机比对，而不是生成器保证。[50][42]
14. **阶段一最小切片（高）：** 类型（`Bits`/`UInt`/`Clock`/`Reset`）→ `#[module]` + comb/seq → HIR → Verilog → Yosys 冒烟 → 原生 `tick` 仿真。参数化、层次、FIRRTL、CDC、形式化全部后移。与需求 19.3 一致，但 **去掉“所有权即多驱动证明”作为阶段一门控**。

## 未决问题

| 问题 | 要什么才能回答 |
| --- | --- |
| USPTO/`rhdl` 商标与 crates.io 抢名的法律风险 | 商标检索 + crates.io 政策，非本轮范围 |
| `samitbasu/rhdl` 全仓库是否仍有隐藏 FIRRTL 字符串 | 需 clone grep；README/LATTE 已是阴性 |
| 自研 HIR 仿真 vs Verilator 的数量级 | 需要基准设计；LATTE 与 README 均无表 |
| FIRRTL 6 property/layer 是否要进可逆子集 | 等 property ABI 进 spec；当前实现定义 [21][22] |
| Cement CTIR 与 GitHub `cmtir` 是否同一 IR | 对照 arXiv PDF 与仓库 |

## 来源附录

| # | 支撑的发现 | 出版方 | 发布 | 访问 | 置信 |
| --- | --- | --- | --- | --- | --- |
| [1] | crates.io `rhdl` 0.1.0 为 3 行占位包 | [crates.io](https://crates.io/crates/rhdl) | 2023-09-02 | 2026-08-18 | high |
| [2] | samitbasu/rhdl 未归档；pushed_at 2026-04-21 | [GitHub API](https://api.github.com/repos/samitbasu/rhdl) | 2026-04-21 | 2026-08-18 | high |
| [3] | RHIF→Verilog；宏限制；仿真自述 | [README](https://raw.githubusercontent.com/samitbasu/rhdl/main/README.md) | 2026-02 | 2026-08-18 | high |
| [4] | rust-hdl 0.46.0（2023-07-02） | [crates.io](https://crates.io/crates/rust-hdl) | 2023-07-02 | 2026-08-18 | high |
| [5] | rust-hdl #29 宏 panic 仍开 | [GitHub](https://github.com/samitbasu/rust-hdl/issues/29) | 2023-07-22 | 2026-08-18 | high |
| [6] | rust-hdl #47 Signed 仿真溢出 | [GitHub](https://github.com/samitbasu/rust-hdl/issues/47) | 2025-04-26 | 2026-08-18 | high |
| [7] | kaze crates.io 0.1.19 | [crates.io](https://crates.io/crates/kaze) | 2021-03-14 | 2026-08-18 | high |
| [8] | crates.io rhdl 0.1.0 体积/行数；kaze archived | [crates.io API](https://crates.io/api/v1/crates/rhdl/0.1.0) | 2023-09-02 | 2026-08-18 | high |
| [9] | Veryl 仓库活跃 | [GitHub](https://api.github.com/repos/veryl-lang/veryl) | 2026-08-18 | 2026-08-18 | high |
| [10] | Veryl v0.20.2 | [GitHub Releases](https://github.com/veryl-lang/veryl/releases/tag/v0.20.2) | 2026-07-01 | 2026-08-18 | medium |
| [11] | Spade GitHub mirror | [GitHub](https://api.github.com/repos/spade-lang/spade) | 2026-08-18 | 2026-08-18 | high |
| [12] | Spade WIP（Hackaday） | [Hackaday](https://hackaday.com/2025/04/13/the-spade-hardware-description-language/) | 2025-04-13 | 2026-08-18 | medium |
| [13] | Cement pushed_at 2025-01-16 | [GitHub API](https://api.github.com/repos/pku-liang/Cement) | 2025-01-16 | 2026-08-18 | high |
| [14] | Cement FIRRTL + firtool-1.86.0 | [README](https://raw.githubusercontent.com/pku-liang/Cement/cmt2/README.md) | 2025-01 | 2026-08-18 | high |
| [15] | cmtrs 0.1.2 | [crates.io API](https://crates.io/api/v1/crates/cmtrs) | 2025-01-09 | 2026-08-18 | high |
| [16] | firtool-1.155.0 | [CIRCT](https://github.com/llvm/circt/releases/tag/firtool-1.155.0) | 2026-08-11 | 2026-08-18 | high |
| [17] | Chisel 7.14.0 配对 firtool 1.155.0 | [Chisel](https://github.com/chipsalliance/chisel/releases/tag/v7.14.0) | 2026-08-13 | 2026-08-18 | high |
| [18] | `etc/circt.json` 钉死 firtool-1.155.0 | [raw](https://raw.githubusercontent.com/chipsalliance/chisel/v7.14.0/etc/circt.json) | 2026-08-13 | 2026-08-18 | high |
| [19] | FIRRTL dialect rationale；CHIRRTL；命名 | [CIRCT](https://circt.llvm.org/docs/Dialects/FIRRTL/RationaleFIRRTL/) | living | 2026-08-18 | high |
| [20] | Scala FIRRTL 仓库归档 | [GitHub](https://github.com/chipsalliance/firrtl) | 2024-08-20 | 2026-08-18 | high |
| [21] | firrtl-spec v6.0.0 | [GitHub](https://github.com/chipsalliance/firrtl-spec/releases/tag/v6.0.0) | 2026-05-12 | 2026-08-18 | high |
| [22] | FIRRTL ABI | [abi.md](https://raw.githubusercontent.com/chipsalliance/firrtl-spec/v6.0.0/abi.md) | 2026-05-12 | 2026-08-18 | high |
| [23] | Chisel 版本与 firtool 覆盖示例 | [chisel-lang.org](https://www.chisel-lang.org/docs/appendix/versioning) | living | 2026-08-18 | high |
| [24] | 无 `.fir` 回解析 | [chisel#4899](https://github.com/chipsalliance/chisel/issues/4899) | 2025-04-22 | 2026-08-18 | high |
| [25] | firtool SV lowering-options | [CIRCT](https://circt.llvm.org/docs/VerilogGeneration/) | living | 2026-08-18 | high |
| [26] | Clash 1.10.0 | [GitHub](https://github.com/clash-lang/clash-compiler/releases/tag/v1.10.0) | 2026-04-23 | 2026-08-18 | high |
| [27] | SpinalHDL Verilog 后端 | [SpinalDoc](https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Other%20language%20features/vhdl_generation.html) | living | 2026-08-18 | high |
| [28] | Amaranth NIR/RTLIL | [Amaranth](https://amaranth-lang.org/play/) | living | 2026-08-18 | medium |
| [29] | CirQTS 线性网表过严 | [LIPIcs](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ECOOP.2023.8) | 2023 | 2026-08-18 | high |
| [30] | Clash typed domains | [Hackage](https://hackage.haskell.org/package/clash-prelude) | 1.8.5 docs | 2026-08-18 | high |
| [31] | Chisel withClock 非类型化 CDC | [chisel-lang.org](https://www.chisel-lang.org/docs/explanations/multi-clock) | living | 2026-08-18 | high |
| [32] | LATTE’25 RHDL：RHIF→Verilog | [PDF](https://capra.cs.cornell.edu/latte25/paper/2.pdf) | 2025-03-30 | 2026-08-18 | high |
| [33] | LATTE’25 cmt2 过程宏→FIRRTL | [PDF](https://capra.cs.cornell.edu/latte25/paper/1.pdf) | 2025-03-30 | 2026-08-18 | high |
| [34] | Cement2 FPGA 数字；firtool-1.108.0 | [arXiv](https://arxiv.org/html/2511.15073v1) | 2025-11 | 2026-08-18 | high |
| [35] | MINRES RHDL RISC-V，无面积数字 | [MINRES](https://www.minres.com/pipelined-riscv-in-rhdl/) | undated | 2026-08-18 | high |
| [36] | XLS experimental；JIT+Verilog | [XLS](https://google.github.io/xls/) | living | 2026-08-18 | high |
| [37] | XLS Z3 为 IR function；proc 另工具 | [XLS tools](https://google.github.io/xls/tools/) | living | 2026-08-18 | high |
| [38] | Verilator 5.050 | [Veripool](https://verilator.org/guide/latest/changes.html) | 2026-07-01 | 2026-08-18 | high |
| [39] | Yosys 0.68 | [GitHub](https://github.com/YosysHQ/yosys/releases/tag/v0.68) | 2026-08-05 | 2026-08-18 | high |
| [40] | SystemC TLM / IEEE 1666-2023 | [systemc.org](https://systemc.org/overview/systemc-tlm/) | 2023 | 2026-08-18 | high |
| [41] | TLM↔RTL 桥 | [libsystemctlm-soc](https://github.com/xilinx/libsystemctlm-soc) | living | 2026-08-18 | high |
| [42] | ChiselSim 非 golden 生成 | [Chisel testing](https://www.chisel-lang.org/docs/explanations/testing) | living | 2026-08-18 | high |
| [43] | essent 停在 FIRRTL 1.6.1 | [GitHub](https://github.com/ucsc-vama/essent/issues/24) | 2026-07-22 | 2026-08-18 | high |
| [44] | firtool-resolver / FNDDS | [README](https://raw.githubusercontent.com/chipsalliance/firtool-resolver/main/README.md) | living | 2026-08-18 | high |
| [45] | Maven llvm-firtool 1.155.0 | [Maven](https://repo1.maven.org/maven2/org/chipsalliance/llvm-firtool/1.155.0/) | 2026-08-11 | 2026-08-18 | high |
| [46] | rust-analyzer 过程宏架构 | [JetBrains](https://blog.jetbrains.com/rust/2026/05/29/how-rust-ides-understand-code/) | 2026-05-29 | 2026-08-18 | high |
| [47] | 下游 path 依赖 RHDL | [GitHub](https://github.com/zebreus/a5-1-rhdl) | undated | 2026-08-18 | medium |
| [48] | VHDL-LS/rust_hdl 名称碰撞 | [GitHub](https://github.com/VHDL-LS/rust_hdl/releases) | 2025-03-22 | 2026-08-18 | high |
| [49] | Hardcaml elaboration 多驱动检查 | [wiki](https://ocamlstreet.gitbook.io/hardcaml-wiki/hardcaml/arxiv-paper) | living | 2026-08-18 | medium |
| [50] | Codasip IA/CA 双模型 | [DVCon PDF](https://dvcon-proceedings.org/wp-content/uploads/uvm-based-verification-of-a-risc-v-processor-core-using-a-golden-predictor-model-and-a-configuration-layer.pdf) | undated | 2026-08-18 | medium |
| [51] | yupferris/kaze archived=true | [GitHub API](https://api.github.com/repos/yupferris/kaze) | 2023-11-15 | 2026-08-18 | high |
| [52] | workspace Cargo.toml 成员 | [raw](https://raw.githubusercontent.com/samitbasu/rhdl/main/Cargo.toml) | 2026-02-03 | 2026-08-18 | high |
| [56] | Dynamatic MLIR-HLS 悔点 | [arXiv](https://arxiv.org/html/2603.19856v1) | 2026 | 2026-08-18 | high |

## 过期图

技术包窗口：version/compat **1** 个月 · landscape **12** 个月 · pattern **24** 个月 · ecosystem **6** 个月。`recon_kit.py staleness`（today=2026-08-18）：

| 声明 | 类 | pub | 复查日 | 状态 |
| --- | --- | --- | --- | --- |
| crates.io `rhdl` 0.1.0 占位 | version/compat | 2023-09 | 2023-10-01 | 历史 pub 已过窗；**本轮 2026-08-18 已再验仍为 0.1.0**，下次 2026-09 再查 crates.io |
| FIRRTL 6.0.0 / 无回解析 | version/compat | 2026-05 | 2026-06-01 | 窗已过；本轮 r2 确认 #4899 仍开、spec 仍为 6.0.0 |
| Chisel 7.14.0 ↔ firtool-1.155.0 | version/compat | 2026-08 | **2026-09-01** | 前瞻最早复查点 |
| Rust eDSL 未收敛 | landscape | 2026-08 | 2027-08-01 | 未过期 |
| 分阶段展开 + 硬件编译器 | pattern | 2026-08 | 2028-08-01 | 未过期 |
| 双视图=两模型/桥 | pattern | 2026-08 | 2028-08-01 | 未过期 |
| Cement 闲置 / XLS LEC 范围 | pattern | 2026-08 | 2028-08-01 | 未过期 |
| samitbasu RHIF 非 FIRRTL | pattern | 2025-03 | 2027-03-01 | 未过期 |

**前瞻最早复查：2026-09-01（firtool 钉死）。** Refresh 时优先拉 Chisel 最新 release 的 `etc/circt.json`。