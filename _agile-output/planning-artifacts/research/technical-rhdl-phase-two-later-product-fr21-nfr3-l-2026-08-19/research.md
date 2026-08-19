---
title: 'technical research: rhdl 阶段二路线图（later-product / FR21 / NFR3 / 语言表面）'
type: 'technical'
topic: 'rhdl phase-two later-product FR21 NFR3 language surface roadmap'
decision: '为下一阶段排优先级：later-product（多时钟/Mem/FST/HLS）、FR21 文档首页、NFR3 firtool 钉死、语言表面从骨架加厚'
source: 'native-run'
status: complete
preset: 'deep'
validation: 'normal'
created: '2026-08-19'
updated: '2026-08-19'
claims_verified: 4
claims_unverified: 2
claims_overturned: 0
---

# technical research: rhdl 阶段二路线图

**Decision this research serves:** 下一阶段应先做什么、后做什么，才能在不偏离「Rust 嵌入式 RTL + FrozenHir + 阶段一 Verilog」主航道的前提下，补齐 FR21 / NFR3、加厚语言表面，并把 later-product 中的多时钟、Mem、FST、HLS 变成可交付里程碑而不是无限延期。

## 执行摘要

证据支持的顺序是：

1. **立刻（文档与工具契约）**：补齐仓库首页 README（FR21：发布名 `rhdl-rs`、与 samitbasu/rhdl 无关）；在 CLI 落地 firtool **钉死下载 + `.sha256` 校验 + 缓存 + 覆盖环境变量**（对标 CIRCT `firtool-1.155.0` 的 `firrtl-bin-linux-x64.tar.gz` 资产，并接受 Chisel 文档可能跟踪更新版本如 1.156.0）。[21][22][23]
2. **紧接着（语言表面加厚）**：在**不引入 HLS / 多时钟**的前提下，把 comb/seq/运算/控制流做成「可写真实小设计」的表面——同代工具的教训是：过薄的 Verilog 形骨架（如 `.val`/`.next`）日后会逼出重写。[15][16]
3. **然后 Mem**：以 FIRRTL 规范 `mem` / Chisel `Mem`·`SyncReadMem`（CHIRRTL→`firrtl.mem`）为语义锚，先单时钟同步读写子集，再谈双口跨时钟。[5][6][7]
4. **再多时钟 / CDC**：选一种可执行策略——Clash/Spinal 型域检查，或 Chisel 型库级 FIFO + 文档纪律；不要假设 FIRRTL 有原生 CDC 检查。[1][2][3][4]
5. **FST 可选、HLS 最后**：VCD 仍是互操作默认；FST 通过 Verilator/GTKWave/Surfer 路径增值。HLS 应是「发射支持 IR/C 并调用 Bambu/XLS 调度器」，不是自研调度；CIRCT HLS 明确非生产。[11][12][13][14][17][18]

**最大 caveat：** Chisel↔firtool **配对随 Chisel 发行变更**；把「永远 1.155.0」写死而不留 `RHDL_FIRTOOL_PATH` / 版本表会与上游文档漂移（FAQ 样本已出现 `firtool-1.156.0`）。[23][24]

## 1. 景观与成熟度：多时钟、Mem、波形

### 多时钟 / CDC

- **Chisel**：主表面是隐式时钟 + `withClock` / `withClockAndReset`；CDC **不**由语言强制，文档要求设计者插入同步（如 AsyncQueue）。长期 RFC 希望在 FIRRTL 侧标注合法跨域，但未变成「语言级 CDC IR」。[1][2]
- **Clash**：`Signal dom a` + `KnownDomain`；跨域是类型错误，除非用双触发器 / async FIFO 同步器。[3]
- **SpinalHDL**：一等 `ClockDomain` + 编译期跨域读错误；`BufferCC` / `StreamFifoCC` 等库路径。[4]
- **Rust 侧**：kaze 文档假定单时钟；rust-hdl 有显式 `Clock` 端口与 async FIFO 部件但无 phantom 域；公开的 samitbasu/rhdl（LATTE’25）用 `Signal<T, Color>` 让 rustc 拒绝误跨域——这是重要对照，但与本仓库 crates.io/`rhdl-rs` 命名策略正交。[8][9][10]

### Mem

- **FIRRTL 规范**仍以 `mem`（端口 bundle、读写延迟、RUW）为准；`cmem`/`smem`/`mport` 属于 **CHIRRTL**，由 CIRCT `LowerCHIRRTL` 降到 `firrtl.mem`。[5][6]
- **Chisel**：`Mem`≈异步读，`SyncReadMem`≈同步读；较新的 `SRAM(...)` 更接近显式端口计数的规范 mem。[7]
- **Clash**：不走 FIRRTL mem IR，而用 `blockRam` / `trueDualPortBlockRam` 等命名原语。[3]

### VCD vs FST

- Verilator 5.x：`--trace`/`--trace-fst`；同一仿真不能同时开两种；FST 文件更小，但 dump 不一定更快；原生查看以 GTKWave、Surfer 为主。[11][12]
- Surfer 原生加载 VCD/FST；FST 利于按信号随机访问。[13]
- 嵌入式 Rust HDL 仿真器普遍先发 **VCD**；FST 多出现在 Verilog→Verilator 之后。[8]

## 2. 集成：NFR3 firtool 钉死怎么落地

### 上游事实（已核验资产表）

- CIRCT 以 `firtool-<semver>` 打 tag；`firtool-1.155.0`（2026-08-11）与更新的 `firtool-1.156.0`（2026-08-16）均存在。[21][22]
- **linux-x64 firtool 专用包**：`firrtl-bin-linux-x64.tar.gz`（1.155.0 上约 96.7 MB）+ 同名 `.sha256`（约 65 B）。另有 `circt-full-shared-*` / `circt-full-static-*`。[22]
- URL 形态：`https://github.com/llvm/circt/releases/download/firtool-<ver>/<asset>`。[22]

### Chisel 生态做法

- Chisel 6+ 通过 **firtool-resolver** 管理版本：`CHISEL_FIRTOOL_PATH`（**目录**内含 `firtool` 二进制）、缓存、`org.chipsalliance:llvm-firtool` Maven 坐标。[23][25]
- 官方政策：每个 Chisel 发行测一个 firtool；其它版本「常能用」但不保证。应用 `BuildInfo.firtoolVersion` 查询配对，而不是死记一个数字。[24]
- CI 常见：`circt/install-circt` action 拉 tarball，再设 `CHISEL_FIRTOOL_PATH`——action 本身**不强调** checksum（校验应在你自己的 CLI 里做）。[26]

### 对 rhdl CLI 的可执行设计（建议）

| 步骤 | 做法 |
|------|------|
| 默认钉 | 配置表：`firtool-1.155.0` + `firrtl-bin-linux-x64.tar.gz`（与既有 AD/NFR 一致） |
| 下载 | GitHub releases URL；失败可回退 Maven `llvm-firtool` classifier |
| 校验 | 读取 sidecar `.sha256`（注意可能是「仅 digest」行，需兼容 `sha256sum -c`） |
| 缓存 | 如 `{XDG_CACHE_HOME}/rhdl/firtool/<ver>/bin/firtool` |
| 覆盖 | `RHDL_FIRTOOL_PATH`（目录或二进制；文档写清） |
| 漂移 | 文档写明「默认钉」与「Chisel FAQ 可能更新」；提供 `rhdl firtool --print-version` |

**置信度：** 资产命名与 1.155.0 表项 **高**（本轮 WebFetch 核验）；「Chisel 当前推荐永远等于 1.155」**低/勿断言**。[22][23]

## 3. 架构模式：HLS 外挂而非自研调度

- **Bambu**：合同是 C/C++ 或 GCC/Clang IR；插件把 SSA/LLVM 变成 Bambu IR（ASCII）；调度仍在工具内。宿主应 **生成 C/LLVM 再调 `bambu`**，而不是假定有稳定的 in-process Rust builder。[17]
- **XLS**：DSLX / C++ `FunctionBuilder` / 实验性 xlscc；调度与 codegen 在 XLS 管线。实验性、非 Google 官方产品；新 opcode 成本极高。[18]
- **CIRCT HLS / Handshake**：研究向、文档自称非生产；Handshake 用动态数据流，**不是** Clash/Spinal/Chisel 那种 RTL 语义。[19]
- **结论：** later-product 的 HLS 应标为 **可选后端适配器**；主产品仍是 RTL eDSL。Clash FAQ 明确「不是 HLS」。[20]

## 4. 实现现实：从「骨架」加厚的顺序

多方证据收敛到同一排序：[14][15][16][7]

1. **冻结层次 IR + comb/seq/ops + 保名 emit + 同语义 sim**（FIRRTL/Chisel3 教训：先把语义与降级边界说清）。
2. **加厚宿主语言表面**（选择、let、聚合、带载荷 enum）——在堆 stdlib / Mem / 多时钟之前；rust-hdl 作者自述 Verilog 形表面导致全面重写。[15]
3. **Mem**（单时钟同步子集对齐 FIRRTL `mem` / `SyncReadMem`）。
4. **多时钟 / CDC**（ClockDomain 或 phantom domain + 库级 FIFO）。
5. **FST**（可选：Verilator dump 或 vcd2fst；默认保留 VCD）。
6. **HLS 适配器**（最后）。

kaze 的公开姿态是最小 Module/Signal/Register + 双 emit，组合 `if_` 仍标 UNSTABLE——说明「emit/sim 可用」可以早于完整语法糖。[16]

## 5. 生态健康（轻量）

- CIRCT firtool **高频发版**（1.150→1.156 可见于 2026 年中）；钉死版本是正确的，但要有升级路径。[21]
- firtool-resolver / Maven `llvm-firtool` 说明「下载+缓存」已是 Chisel 用户预期能力；rhdl 不发明新模式，只换语言与环境变量前缀即可。[25]
- 命名冲突：crates.io **不能真正 rename**；实践是换发布名、README 指路、必要时 facade 重导出——支撑 FR21「首页声明」而非仅内部 AGENTS 注释。[27][28]

## 跨维度洞察

- **钉 firtool（NFR3）与 Mem/多时钟互锁：** 没有可重复的 firtool，阶段二 FIRRTL→SV / Mem 降级就无法做回归；但 firtool 本身不替代语言表面加厚。
- **FST 不是阻塞项：** 在 Verilator 路径上「免费」可得；优先把 tick/VCD 做对即可。[11]
- **HLS 与 RTL 表面争抢注意力：** 过早 HLS 会把工程拖进调度器/IR opcode 税，而用户此刻缺的是 comb/seq 可写性。[18][19][15]
- **与公开 rhdl（samitbasu）的关系：** 其 phantom CDC 是技术对标，也是 FR21 必须在首页划清的身份边界——文档债与架构债绑在一起。[10][27]

## 建议（绑定下游产物）

| # | 建议 | 置信 | 下游 |
|---|------|------|------|
| R1 | 本周落地 **README 首页**：`rhdl-rs` 发布名、禁止暗示 crates.io `rhdl`/`rhdl-bits`、与 samitbasu/rhdl 无关 | 高（规范要求清晰；实现成本低） | FR21 / AGENTS 对外化 |
| R2 | 下一里程碑实现 **firtool fetch**：钉 `firtool-1.155.0` + `firrtl-bin-linux-x64.tar.gz` + `.sha256` + 缓存 + `RHDL_FIRTOOL_PATH`；文档注明上游可能更新 | 高（资产已核验）[22] | NFR3 / architecture AD-9 |
| R3 | **语言表面 sprint**：宏/builder 完整 if/match、同位宽算子、reg 复位语义、至少 2 个非玩具示例（计数器+简易 FIFO 接口形状但仍单时钟） | 中高（来自多工具后验）[15][16] | epics / SPEC 下一 epic |
| R4 | **Mem epic**：单时钟 `SyncReadMem` 语义 → HIR 节点 → vlog/`firrtl.mem`；双口跨时钟仅经命名 CDC FIFO | 高（规范锚清晰）[5][7] | later-product → 正式 FR |
| R5 | **多时钟 epic**：在 R3–R4 后；在「Spinal 式检查」vs「Clash 式 phantom」vs「Chisel 式库」中显式选一并写进 spine | 中（选型未定）[1][3][4][10] | architecture Deferred → ADOPTED |
| R6 | **FST**：作为 sim/Verilator 可选开关；不阻塞 R3 | 高[11][12] | later-product |
| R7 | **HLS**：仅设计「发射 + 调 Bambu/XLS」spike；不进下一主 epic | 高[17][18][19] | later-product |

## 开放问题

1. rhdl 是否要对齐 samitbasu/rhdl 的 **phantom Color 域**，还是坚持阶段一单时钟更久？（需产品决策）
2. 默认 firtool 钉在 1.155.0 时，是否跟随 Chisel 7.x 表自动升到 1.156.0？（需政策）
3. Mem 是否阶段二就暴露 CHIRRTL 友好 API，还是只暴露规范 `mem`？
4. FST 是否要自研 writer，还是永远依赖 Verilator/GTKWave 转换？

## Source appendix

| n | 支撑 | 发布方 | 日期 | 访问 | 置信 |
|---|------|--------|------|------|------|
| [1] | Chisel multi-clock / withClock | [chisel-lang.org](https://www.chisel-lang.org/docs/explanations/multi-clock) | living | 2026-08-19 | high |
| [2] | Chisel CDC RFC discussion | [github.com/chipsalliance/chisel#1085](https://github.com/chipsalliance/chisel/issues/1085) | ~2019+ | 2026-08-19 | high |
| [3] | Clash Signal / synchronizers / blockRam | [Hackage clash-prelude](https://hackage.haskell.org/package/clash-prelude) | 1.8.x docs | 2026-08-19 | high |
| [4] | Spinal ClockDomain / CDC errors | [SpinalDoc](https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Structuring/clock_domain.html) | master | 2026-08-19 | high |
| [5] | FIRRTL spec mem | [firrtl-spec v6](https://github.com/chipsalliance/firrtl-spec/blob/v6.0.0/spec.md) | v6.0.0 ~2025-05 | 2026-08-19 | high |
| [6] | CHIRRTL vs mem / CIRCT rationale | [CIRCT FIRRTL rationale](https://circt.llvm.org/docs/Dialects/FIRRTL/RationaleFIRRTL/) | living | 2026-08-19 | high |
| [7] | Chisel memories | [chisel-lang memories](https://www.chisel-lang.org/docs/explanations/memories) | living | 2026-08-19 | high |
| [8] | rust-hdl 0.46 docs | [docs.rs/rust-hdl](https://docs.rs/rust-hdl/0.46.0/rust_hdl/) | 0.46.0 | 2026-08-19 | high |
| [9] | kaze Module/Mem | [docs.rs/kaze](https://docs.rs/kaze/latest/kaze/) | 0.1.19 | 2026-08-19 | high |
| [10] | LATTE’25 RHDL paper | [capra.cs.cornell.edu](https://capra.cs.cornell.edu/latte25/paper/2.pdf) | 2025 | 2026-08-19 | high |
| [11] | Verilator trace VCD/FST | [verilator.org guide](https://verilator.org/guide/latest/exe_verilator.html) | 5.050 docs | 2026-08-19 | high |
| [12] | Verilator FAQ FST size/viewers | [veripool FAQ](https://veripool.org/guide/latest/faq.html) | 5.050 | 2026-08-19 | high |
| [13] | Surfer / FST | [CAV 2025 Surfer PDF](https://ics.jku.at/files/2025CAV_Surfer.pdf) | 2025 | 2026-08-19 | high |
| [14] | FIRRTL/Chisel3 split rationale | [UCB-BAR firrtl](https://bar.eecs.berkeley.edu/projects/firrtl.html) | ~2016 | 2026-08-19 | high |
| [15] | rust-hdl→RHDL rewrite README | [github.com/samitbasu/rhdl](https://github.com/samitbasu/rhdl) | living | 2026-08-19 | high |
| [16] | kaze minimal API | [docs.rs/kaze](https://docs.rs/kaze/latest/kaze/) | 0.1.19 | 2026-08-19 | high |
| [17] | PandA-bambu / IR frontends | [github.com/ferrandi/PandA-bambu](https://github.com/ferrandi/PandA-bambu) · [panda.deib](https://panda.deib.polimi.it/?page_id=31) | living / 2021+ | 2026-08-19 | high |
| [18] | Google XLS / DSLX / xlscc | [google.github.io/xls](https://google.github.io/xls/) | living | 2026-08-19 | high |
| [19] | CIRCT HLS | [circt.llvm.org/docs/HLS](https://circt.llvm.org/docs/HLS/) | living | 2026-08-19 | high |
| [20] | Clash FAQ not HLS | [clash-lang docs FAQ](https://docs.clash-lang.org/compiler-user-guide/general/faqs.html) | living | 2026-08-19 | high |
| [21] | CIRCT firtool tags list | [github.com/llvm/circt/releases](https://github.com/llvm/circt/releases) | living | 2026-08-19 | high |
| [22] | firtool-1.155.0 assets（本轮核验） | [releases/tag/firtool-1.155.0](https://github.com/llvm/circt/releases/tag/firtool-1.155.0) | 2026-08-11 | 2026-08-19 | high |
| [23] | Chisel installation / CHISEL_FIRTOOL_PATH | [chisel-lang installation](https://www.chisel-lang.org/docs/installation) | living | 2026-08-19 | high |
| [24] | Chisel firtool versioning policy | [chisel-lang versioning](https://www.chisel-lang.org/docs/appendix/versioning) | living | 2026-08-19 | high |
| [25] | firtool-resolver Main.scala | [chipsalliance/firtool-resolver](https://github.com/chipsalliance/firtool-resolver) | living | 2026-08-19 | high |
| [26] | circt/install-circt action | [github.com/circt/install-circt](https://github.com/circt/install-circt) | v1.1.1 | 2026-08-19 | high |
| [27] | crates.io rename impossibility | [crates.io#2902](https://github.com/rust-lang/crates.io/issues/2902) | closed pending RFC | 2026-08-19 | high |
| [28] | Rename + README / facade practice | [oxo-flow PR](https://github.com/Traitome/oxo-flow/pull/25) · [justerm ADR](https://github.com/kihyun1998/justerm/blob/master/docs/adr/0010-all-prefixed-crate-naming.md) | 2025–2026 | 2026-08-19 | medium |

## Staleness map

| 类别 | 窗口 | 例 | 建议复检 |
|------|------|----|----------|
| versions & compatibility | ≤1 月 | firtool 1.155/1.156、Chisel `firtoolVersion` | **2026-09-19** |
| ecosystem signals | ≤6 月 | firtool-resolver、Maven llvm-firtool | 2027-02-19 |
| landscape | ≤12 月 | CDC/Mem 语言模型 | 2027-08-19 |
| patterns | ≤2 年 | HLS 外挂、表面加厚顺序 | 2028-08-19 |

**最早复检：`versions`（约一个月内）**——firtool 发版快，默认钉可能需要跟随政策更新。
