---
title: 'technical research: doc19 seven-stage full-green product plan'
type: 'technical'
topic: 'doc19 seven-stage full-green product plan'
decision: 'How to achieve product-complete / docs-requirements seven-stage green under redefined completion standards'
source: 'native-run'
status: complete
preset: 'standard'
validation: 'normal'
created: '2026-09-09'
updated: '2026-09-09'
verified_claims: 5
unverified_claims: 1
claims_total: 6
---

# technical research: doc19 seven-stage full-green product plan

**Decision this research serves:** 如何把「产品做完 / docs/requirements 路线图七阶段全绿」变成可验收、可排期的下一合同（默认：按同业实践**重定义完成标准**，而非字面每一条）。

## Executive summary

**建议：不要把 doc-19 字面七阶段当作单一「全绿」清单；先用 Correct Course 重写阶段五–七的完成定义，再按 4 波交付「合同绿」。字面全做（自研 HLS、idiomatic Chisel 往返、自动 TLM≡CA、一等 LSP、全协议 IP）同业也未作为核心产品一次兑现。**

驱动结论的证据：

1. **同类 eDSL 把「done」钉在 RTL 生成 + 宿主工具 + 薄 stdlib/仿真，并公开排除或未宣称 HLS / 一等 LSP / IP 商城。** Spinal 明文「不是 HLS」；Clash/Amaranth/Chisel 发布面同构。[1][2][11]
2. **互操作现实边界是 FIRRTL/Verilog/CIRCT 文件面，不是 Scala 源码往返；HLS 是 CLI/AppImage 外挂；多视图是同刺激共测，不是形式化 FL≡RTL。** [3][4][5][6]
3. **产品化模式：薄生成器 stdlib + 树外深 IP；LSP 用浅层分析或直接复用宿主 LSP；双模拟共用刺激/生成 TB，而非两套独立产品。** [7][8][9][10]

最大 caveat：本报告回答的是**同业如何定义晚阶段「完成」与排期形状**；仓库内 Phase 1–10 已结范围不作为外部证据。若坚持 doc-19 字面全绿，证据表明那是**多年并行、高维护、且同业未打包交付**的目标——应改完成定义，而不是假装有捷径。

---

## Landscape & maturity

同类 Scala/Python/Haskell RTL eDSL 公开把产品完成度钉在：**可预测的 RTL/netlist 生成 + 宿主语言 IDE/REPL + 薄标准库/构建/仿真**；**HLS、一等 HDL LSP、商业级 IP 目录**不是发布门槛。[1][2][11]

- Spinal：明确「不是 HLS」；IDE = IntelliJ/Metals；仿真对接 Verilator + Gtkwave。[1]
- Amaranth：定位完整 FPGA 工作流（语言/stdlib/Python sim/板级），仍声明原生协仿等为将来项；成熟度靠 RFC/语言合同而非 IDE/HLS。[2]
- Clash：类型化 HDL + stdlib + REPL；主页不宣称一等 LSP/HLS/IP 产品线。[11]
- Chisel v7 发布强调语言/FIRRTL/CIRCT/`firtool`，不以 HLS/IP/IDE 为 ship 标准。[12]
- 近 12 个月：**Chisel→CIRCT/`firtool` 已巩固**；源级波形/调试元数据仍在 UHDI/probe 等提案中 churn——富可视化是相邻集成，不是「可视化 done」勾选。[13][14]

**检索未找到**把「HLS + IP 商城 + 一等 LSP + 多视图形式等价」绑成单一晚阶段 done 定义的同业路线图（缺失本身是发现）。

*本维度在 round 1 覆盖后停止（novelty：无新的「全绿打包」反例）。*

---

## Integration & interoperability

**Chisel/FIRRTL：** 官方路径单向：Chisel → CHIRRTL/FIRRTL → CIRCT/`firtool` → Verilog。维护者确认自迁 CIRCT 后，**不再支持**把 `.fir` 解析回 Scala `Circuit`；应使用 `firtool -parse-only` / CIRCT，或序列化对象、或把变换做成 CIRCT plugin。[3] CIRCT FIRRTL dialect 定位为 SFC 的 drop-in **降低/发射**，不是 idiomatic Chisel 重建后端。[15]

**HLS：** Bambu/PandA 与 Vitis HLS 均为 **CLI/文件/IP 产物**集成（AppImage/`vitis-run`），不是嵌入宿主 HDL 编译器的进程内库。[4][5]

**多视图：** PyMTL3 实践是 FL/CL/RTL 同 Python TB 精化 + Verilator 导入，**不是**自动形式 FL≡RTL。[6] SystemC TLM LT/AT 与 cycle-accurate 目标不同，工业混合靠 adapter/共仿，而非默认 bit-identical 锁步等价。[16]

**行业现实的「互操作 done」：** 稳定 IR/文件边界 + 跨抽象同刺激/记分板 + 仅在有时钟端口处要求周期精确；**不要**把 idiomatic Chisel 往返、进程内 HLS、TLM≡CA 形式证明打包进完成定义。

---

## Architecture patterns in practice

**IP：** 薄 in-tree 生成器（`chisel3.util` / `spinal.lib`：总线、FIFO、Stream）+ **树外**深 SoC/外设；Rocket 系把 Diplomacy 等拆成独立库，避免「胖 stdlib」。[7][17] 可复用价值在参数化生成器与协商接口，而非手写 RTL 目录堆砌。

**LSP/IDE：** 可扩展 HDL LSP 用**分层/浅层编译**（HRT slang-server：按键路径浅层 elaborate；全设计可选）。[8][18] Verible 默认单文件；工程级 goto 需 filelist。[19] eDSL 通常**复用宿主 LSP**（Metals/Pyright）；自研 netlist LSP 多为非目标或极初级（Spade 因自有语法才投）。[9]

**双模拟：** Clash：宿主行为仿真 + **同刺激生成 HDL TB**。[10] Chisel 当代路径是 peek/poke 覆盖外部周期仿真（Verilator/VCS），不是独立「功能模拟器产品」。[20] **共享刺激 / 生成 TB** 主导，避免两套 TB 产品。

---

## Implementation reality

把「七阶段字面全绿」当一个里程碑，会撞上同业已公开拒绝或永久外置的工作：

| 字面目标 | 同业现实 | 维护含义 |
|---|---|---|
| 自研 HLS 调度 | Spinal 等排除；Clash/Amaranth 不宣称 | 外挂 CLI 即可「绿」 |
| idiomatic Chisel 往返 | 官方不支持 FIRRTL→Scala Circuit | 机械 emit / `.fir` 边界即可 |
| 自动 TLM≡CA | 非 TLM 成功标准 | 共仿 + 同 TB |
| 一等 eDSL netlist LSP | SV LSP 尚需浅层技巧与团队投入 | 先 rust-analyzer；浅层诊断可选 |
| 全协议 IP 目录 | stdlib 明确非完整目录 | FR82 基线 + 树外加深 |

可执行排序（同业暗示）：**(1)** 钉死 IR/文件与发射质量 → **(2)** 薄生成器 + 树外深 IP → **(3)** 宿主 LSP，必要时浅层诊断 → **(4)** 功能/周期路径共享刺激，而非双产品 → **(5)** HLS 永外挂。[21]

公开**人年定量**缺失（检索未找到）；定性结论足够支撑「分波合同」而非「一次全绿」。

---

## Cross-dimension insights

1. **「全绿」冲突来自完成定义，不是缺某一个 epic：** Landscape 排除的能力，正是 Integration 证明官方不支持、Architecture 建议外置的能力。
2. **工具链越「完整」，越依赖文件边界与外挂**（firtool、Bambu、Verilator、Gtkwave/Surfer），而不是把一切吸进语言核心。
3. **多视图与 LSP 的杠杆点相同：浅层、增量、同刺激**——深 elaborate / 形式等价不该挡发布。

---

## Recommendations

绑定下游：**Correct Course → PRD/Epic 增补（建议 Phase 11+）→ Architecture spine 修订 → Sprint。**

1. **重定义 doc-19 阶段五–七「绿」为合同条款（高置信，基于 [1][3][4][7][10]）**  
   - P5 绿 = 外挂 HLS 可复现产物 + 薄 IP 生成器 + VCD/层次可视化（已有能力对齐即可勾）  
   - P6 绿 = 精选 IP **深度合同**（非 VIP）+ 文档站 + **宿主 LSP 路径**（rust-analyzer）+ 可选浅层 Bitloom 诊断  
   - P7 绿 = 多抽象 **同刺激** 功能/周期路径 + 桥接 adapter 模板；**不**承诺自动等价证明 / SystemC TLM 产品

2. **排成 4 波（中高置信，模式综合）**  
   - **Wave A — 合同对齐（短）：** Correct Course；改写 `19. 实施路线图` 完成定义；README/deferred 对齐；禁止字面勾选未交付项  
   - **Wave B — 互操作硬化：** CIRCT/`firtool` 版本钉死运维；机械 Chisel emit 保持「可编译非 idiomatic」诚实；HLS 夜间真机可选  
   - **Wave C — IP/生态深度：** 按需加深 UART/SPI/I2C/AXI **显式子集**；树外示例包；禁止 silent 全协议  
   - **Wave D — IDE + 多视图：** rust-analyzer 工作流文档 → 可选浅层 LSP；`build-sim`/功能路径与 tick **共享刺激夹具**；形式等价另合同

3. **明确永久非目标（除非新 PRD）（高置信 [1][3][16]）**  
   自研 HLS 调度器；FIRRTL→idiomatic Scala；默认 TLM≡CA 形式证明；按键全设计 elaborate 的 netlist LSP；VIP 级全协议 IP。

4. **置信说明：** 版本/官方立场类主张已对 Chisel#4899、Spinal About、Clash FAQ、Bambu AppImage、HRT shallow LSP 做 landing spot-check；工作量人年仍为 **unverified**（公开数据薄）。

---

## Open questions

1. 产品是否接受「合同绿」替代「字面绿」？若不接受，需单独做多年预算与裁员式范围，本报告不提供捷径。  
2. Wave C 优先加深哪一类 IP？（需产品优先级，非技术可独断）  
3. Wave D 的 LSP：仅文档化 rust-analyzer，还是要交付浅层 Bitloom LSP MVP？  
4. P7 是否引入 SystemC/TLM 叙事，还是只保留 Rust host 功能模型？（ARCHITECTURE 曾排除 SystemC TLM 合同——需产品确认）

---

## Source appendix

| n | Supports | Publisher | Pub date | Accessed | Confidence |
|---|---|---|---|---|---|
| [1] | Spinal not HLS; host IDE | [SpinalHDL About](https://spinalhdl.github.io/SpinalDoc-RTD/master/SpinalHDL/Introduction/SpinalHDL.html) | master (live) | 2026-09-09 | high |
| [2] | Amaranth toolchain scope; cosim deferred | [Amaranth intro](https://amaranth-lang.org/docs/amaranth/latest/intro.html) | latest docs | 2026-09-09 | high |
| [3] | No FIRRTL→Scala Circuit post-CIRCT | [chisel#4899](https://github.com/chipsalliance/chisel/issues/4899) | ~2024–2025 | 2026-09-09 | high (spot-verified) |
| [4] | Bambu CLI/AppImage | [PandA docker/AppImage docs](https://docs.bambuhls.eu/da/df6/install_docker.html) | PandA-2024.02 | 2026-09-09 | high |
| [5] | Vitis HLS CLI/`vitis-run` | AMD UG1399/UG1702 2026.1 | 2026.1 | 2026-09-09 | high |
| [6] | PyMTL3 multi-level + Verilator import | [PyMTL3 Ext Verilog](https://pymtl3.readthedocs.io/) / Jiang IEEE Micro 2020 | 2017–2022 / 2020 | 2026-09-09 | high |
| [7] | Thin stdlib vs out-of-tree IP | Chisel util / Spinal lib docs | live | 2026-09-09 | high |
| [8] | Shallow compilation LSP | [HRT slang-server design](https://www.hudsonrivertrading.com/hrtbeat/designing-a-systemverilog-language-server/) | ~2026 | 2026-09-09 | high (spot-verified) |
| [9] | eDSL piggybacks host LSP | Chipyard Metals issues; amaranth-stubs | mixed | 2026-09-09 | medium |
| [10] | Shared stimuli / generated TB | [Clash FAQ](https://docs.clash-lang.org/compiler-user-guide/general/faqs.html) + Clash TB docs | live | 2026-09-09 | high |
| [11] | Clash public surface | [clash-lang.org](https://clash-lang.org/) | live | 2026-09-09 | high |
| [12] | Chisel v7 ship criteria | [chisel v7.0.0](https://github.com/chipsalliance/chisel/releases/tag/v7.0.0) | 2025-09-08 | 2026-09-09 | high |
| [13] | UHDI debug format churn | [LLVM Discourse UHDI](https://discourse.llvm.org/t/uhdi-unified-hardware-debug-info-structured-debug-info-export-for-chisel-firrtl/90973) | 2026-06/07 | 2026-09-09 | high |
| [14] | Tywaves typed waves adjacent | arXiv 2408.10082 | 2024-08 | 2026-09-09 | high |
| [15] | CIRCT FIRRTL = lower/emit | [FIRRTL Rationale](https://circt.llvm.org/docs/Dialects/FIRRTL/RationaleFIRRTL/) | live | 2026-09-09 | high |
| [16] | TLM LT ≠ CA equivalence goal | ACM TODAES TLM survey / SystemC practice | 2025 / mixed | 2026-09-09 | medium-high |
| [17] | Diplomacy extract from Rocket | rocket-chip PRs #2741/#3571 | 2020–2024 | 2026-09-09 | high |
| [18] | slang-server shallow feature | [shallow compilation](https://hudson-trading.github.io/slang-server/features/design/shallow/) | live | 2026-09-09 | high |
| [19] | Verible filelist for project nav | verible LS README | live | 2026-09-09 | high |
| [20] | ChiselSim peek/poke path | Chisel Testing docs | live | 2026-09-09 | high |
| [21] | Sequencing synthesis | digests landscape+integration+architecture+impl | 2026-09-09 | 2026-09-09 | medium |

---

## Staleness map

| Claim class | Window | Earliest re-check |
|---|---|---|
| Chisel/CIRCT/`firtool` versions & FIRRTL parse stance | ≤1 month | 2026-10-09 |
| HLS tool CLI (Bambu/Vitis) | ≤6 months | 2027-03-09 |
| Landscape “peers exclude HLS/LSP as core done” | ≤12 months | 2027-09-09 |
| Architecture patterns (stdlib/LSP/dual-sim) | ≤24 months | 2028-09-09 |

Refresh 优先：`firtool`/Chisel 版本钉与 #4899 类互操作声明。
