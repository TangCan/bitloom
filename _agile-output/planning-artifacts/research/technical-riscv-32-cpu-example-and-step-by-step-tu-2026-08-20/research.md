---
title: 'technical research: RV32 CPU example + step-by-step tutorial for Bitloom'
type: 'technical'
topic: 'RV32 CPU example + step-by-step tutorial for Bitloom'
decision: 'Pick a feasible path to ship an RV32 example core and tutorial on Bitloom'
source: 'native-run'
status: complete
preset: 'standard'
validation: 'normal'
created: '2026-08-20'
updated: '2026-08-20'
verified_claims: 6
unverified_claims: 0
---

# technical research: RV32 CPU example + step-by-step tutorial for Bitloom

**Decision this research serves:** 为 Bitloom 选定一条可交付的 **教学向 RV32 示例核 + step-by-step 教程**路径（默认不做完整 SoC / MMU / Linux，除非证据强制）。

## Executive summary

**建议：自写教学核，按「FemtoRV / Harris」式递进教程推进；不要以 VexRiscv/Linux 或 SERV 位串行作为第一课。**

三条驱动证据：

1. **教学微架构已收敛**为单周期 → 多周期 → 流水线阶梯；社区「成品核」则按目标分裂为极小面积（PicoRV32/SERV）与可配置流水 Softcore（VexRiscv→VexiiRiscv）[1][2][4]。
2. **首核 ISA 子集应砍特权**：多门课先做 RV32I（甚至去掉 CSR/ECALL）；特权规范明确 **仅 M-mode 强制**，CSR 需 Zicsr——适合作为 Episode II，而非 Day-1 [5][7]。
3. **可交付教程形态已有现成范本**：blinker→decoder→…→GNU C，全程仿真优先；流水线作 Episode II；宣称 ISA 完整前用 riscv-tests / RISCOF 作**最小门禁**（不是完整 DV）[3][6]。

**最大 caveat：** 公开样本里 **几乎没有「用 Rust eDSL 写教学 RV32 核」的对标包**；Bitloom 的 `cargo`/`examples/`/`bitloom-sim` 路径是差异化，也意味着教程必须自造「第一步可跑」合同，不能照搬 Make/SBT 惯例 [8]。

---

## 1. Landscape & maturity

开源/教学 RV32 景观可分成四层：

| 层 | 代表 | 角色 |
|---|---|---|
| 教材阶梯 | Harris DDCA RISC-V + 配套 HDL/Labs | 单→多→流水教学主路径 [2][4] |
| 极小 HDL 核 | PicoRV32（单文件 Verilog，可配 RV32E/I/C/M）、SERV（位串行） | 面积/可移植参考；SERV 文档强调教育用途但非默认「第一 datapath」[1][9] |
| 可配置 FPGA Softcore | VexRiscv；继任 VexiiRiscv（教程 + Spike lock-step） | Linux 能力端在**换代**；不适合作「手写 datapath」第一示例 [10][11] |
| 非 HDL 参照 | Spike（功能 ISA sim）、riscv-formal（RVFI 形式化） | 黄金模型 / 验证工具，不是教学微架构 [12][13] |

**当今代新能力：** `docs.riscv.org` 批准规范库、RISCOF + `riscv-arch-test`（相对 Spike/Sail 签名）、riscv-formal、常见 `gcc-riscv64-unknown-elf` 打包——使「教学核 + 教程」可在无自建完整 SoC 的情况下宣称最小合规门禁 [3][14]。RISCOF 明确 arch-test 是**最小过滤器**，不能替代完整设计验证 [3]。

**成熟度信号（本轮）：** Spike / VexRiscv 有 2026 年推送；SERV RTD 文档质量高；PicoRV32 星标与 README 完整，但 IRQ 特性**故意不跟** RISC-V IRQ 惯例——作金标准易误导学生 [1][15]。

*维度停止原因：Coverage（谱系与「教学 vs 成品核」分工已回答）。*

---

## 2. Architecture patterns in practice

**主导教学模式：** 单周期 → 多周期 FSM → 经典五级流水；常配**裁剪的 RV32I**（如 add/sub/and/or/slt/lw/sw/beq/jal + I-type ALU）[4][2]。SERV 位串行是另一条「极小面积 / 深度内嵌」轴，可选 Zicsr，不宜替代教材阶梯的第一站 [9]。

**失败模式（课设/实验反复强调）：**

- 流水：RAW 需转发；load-use 需 stall；预测不跳时 taken branch 常付 2 bubble [16]。
- 访存路径：`sw` 同时依赖转发 `rs2` 与 immediate；波形调试多指令流水状态是常态 [17]。

**最小 ISA / 特权建议：**

- Day-1：**无 CSR / 无 ECALL·EBREAK·FENCE** 的 RV32I 子集即可上板（例：NJU Lab 11，37 条）[7]。
- 特权地板：硬件平台**仅强制 M-mode**；CSR 读写修改依赖 **Zicsr**——适合后置 [5]。

**成功教程拆分：**

- Harris/课设：先单周期处理器实验，再多周期 datapath/control，再流水 [4]。
- DINOCPU 式流水作业：流水寄存器 → 转发 → 分支冲刷 → hazard detection [18]。
- FemtoRV：blink → ROM → **decoder** → 寄存器/状态机 → ALU → … → load/store → MMIO → GNU asm/C；流水线是 Episode II [6]。

*维度停止原因：Coverage。*

---

## 3. Implementation reality & packaging

**学习曲线现实：** RVfpga 类包按 **1–2 学期**设计，痛点常在工具链/SoC 拼装与「非玩具」商业核扩展，而非「从零发明 ISA」[19]。纵向「教软核 6–12 个月」教师回顾本轮**未找到**。

**验证栈（教学仓常见组合）：** Verilator（及 Icarus）≫ 语言测试（cocotb/ChiselTest）≫ `riscv-tests` 或 RISCOF/arch-test ≫ riscv-formal（偏参考/进阶）[1][3][13][15][20]。

**教程打包形态：**

1. 仓内超长递进 Markdown（FemtoRV）[6]
2. 编号 lab 目录 + 统一 Make/SBT（ca2025-mycpu：`0-minimal`→`1-single-cycle`→trap→pipeline→`4-soc`）[20]
3. 可下载 lab 包 + IDE 章节（RVfpga）[19]
4. CPU+SoC+SW 一体 + 用户手册（NEORV32；arch-test CI）[15]

**与 Rust/`cargo` 的关系：** 本轮样本中，教学 RTL 主流是 **Make/SBT/PlatformIO**；Rust 多作核上软件而非教学 HDL。**未发现**可对标的「Rust eDSL 教学 RV32 核」包 [8]。Generator 型（VexRiscv `sbt runMain …` → `.v`）适合「用核」，弱于「写 datapath」[10]。

**生态健康（选参照核时）：** 偏好宽松许可证 + 活跃推送 + 可控 issue；警惕「非规范金标准」（PicoRV32 IRQ 注记）与大 issue 积压的生成器核 [1][10][15]。

*维度停止原因：Coverage（打包与验证合同已够决策；Rust-HDL 空白记为 open question）。*

---

## Cross-dimension insights

- **教学阶梯 ≠ 成品 Softcore 路线：** Harris/FemtoRV 要的是「一步一变」；Vex/Vexii 要的是配置与 Linux——混作第一示例会拖进 SoC。
- **最小合规门禁已商品化，但与仿真入口分离：** RISCOF/arch-test 可作「宣称完整」门槛 [3]；逐步章节仍需本地仿真（Verilator/`tick` 类）支撑「每步可跑」[6]。
- **Bitloom 的差异化与空白重合：** 真独立 `cargo bitloom` / `bitloom-sim` 可成为教程脊柱，但公开世界没有现成 Rust eDSL RV32 教程可抄——必须自设计章节 DoD [8]。

## Contrary evidence

- SERV 与 PicoRV32 证明「极小核 + 合规」可教且文档成熟 [1][9]——但课程主路径仍是 Harris 阶梯，位串行/单文件核更适合**对照阅读**或进阶选修，而非替换 Day-1 datapath。
- RVfpga/NEORV32 显示「给完整 SoC + 再讲微架构」也是合法产品形态 [15][19]——若目标是「两学期体系课」可采纳；与「Bitloom 示例 + 短教程」目标不对齐。

## Recommendations

| # | 建议 | 下游绑定 | 置信依据 |
|---|---|---|---|
| R1 | **示例核范围：** Episode I = 裁剪 RV32I、无 CSR/trap；单周期或多周期 FSM；哈佛或简单统一存储器 + MMIO LED/UART。Episode II = 流水或 Zicsr/M-trap。明确不做 MMU/Linux。 | architecture spine / epics 候选 | [4][5][7] high |
| R2 | **教程骨架抄 FemtoRV 合同：** 每步只改一件事；从 blink/ROM 到 decoder→ALU→mem→GNU C；流水线另章；每步仿真优先。 | `docs/` step-by-step + `examples/rv32_*` | [6] high |
| R3 | **验收门禁分层：** 章节 DoD = `elaborate`/`tick` 或等效仿真断言；「ISA 完整」章再挂 riscv-tests 或 RISCOF 子集；可选 RVFI 仅挂参考核。 | CI / ATDD | [3][1] high |
| R4 | **打包：** monorepo `examples/` + `docs/tutorials/rv32-…` 编号章节；对齐已发布 `cargo bitloom new/build` 与 `bitloom-sim --dev`；勿要求 clone 才能读教程前几章。 | README / Epic 规划 | [8] medium（Rust 路径无对标，属项目约束+证据空白） |
| R5 | **参照阅读清单（非 fork）：** Harris Ch.7 + FemtoRV README；对照 PicoRV32/SERV；避免把 Vexii Linux 教程当第一路径。 | 教程「延伸阅读」 | [2][6][1][9][11] high |

## Open questions

1. Bitloom 当前宏/HIR 表面能否表达「多周期 FSM CPU」所需的全部控制与存储器端口，而不引入未交付语言特性？（需项目内可行性 spike，本轮防火墙未读代码。）
2. 教学用最小 `riscv-arch-test` / `riscv-tests` 子集在 CI 上的墙钟与二进制依赖如何钉死？
3. 是否存在更新的（≤12 个月）教学核横向评测，可替代「星标 = 主导」启发式？
4. Rust-HDL / 其他 Rust eDSL 是否在本轮视野外已有 RV32 教程核？（本轮检索未找到。）

## Source appendix

| Ref | Supports | Publisher | Pub date | Accessed | Confidence |
|---|---|---|---|---|---|
| [1] | PicoRV32 landscape / tests / IRQ caveat | [YosysHQ/picorv32](https://github.com/YosysHQ/picorv32) | ~2015–2026 | 2026-08-20 | high |
| [2] | Harris companion + labs/HDL | [HMC DDCA RISC-V](https://pages.hmc.edu/harris/ddca/ddcarv.html) | ongoing | 2026-08-20 | high |
| [3] | RISCOF = minimal arch filter | [RISCOF intro](https://riscof.readthedocs.io/en/stable/intro.html) | docs 1.24.x | 2026-08-20 | high |
| [4] | Single→multi→pipe teaching ladder + subset | [DDCArv Ch.7 PDF](https://pages.hmc.edu/harris/class/e85/DDCArv_Ch7.pdf); [WCAE’21 slides](https://pages.hmc.edu/harris/research/WCAE_Paper8_2021_DDCA_RISCV_Harris_lightningslides.pdf) | 2020–2021 | 2026-08-20 | high |
| [5] | M-mode only mandatory; Zicsr for CSR | [RISC-V priv intro](https://docs.riscv.org/reference/isa/v20260120/priv/priv-intro.html) | v20260120 | 2026-08-20 | high |
| [6] | Blinker→RISC-V step tutorial; pipeline Episode II | [FemtoRV FROM_BLINKER_TO_RISCV](https://github.com/BrunoLevy/learn-fpga/blob/master/FemtoRV/TUTORIALS/FROM_BLINKER_TO_RISCV/README.md) | 2020–2022; repo 2025 | 2026-08-20 | high |
| [7] | RV32I without CSR/ecall as first FPGA core | [NJU DLCO Lab 11](https://nju-projectn.github.io/dlco-lecture-note/en/exp/11.html) | undated | 2026-08-20 | high |
| [8] | No Rust-eDSL RV32 teaching-core package found; Make/SBT dominate | this-run negative finding (impl digest) | 2026-08-20 | 2026-08-20 | medium |
| [9] | SERV bit-serial + education use | [SERV datasheet](https://serv.readthedocs.io/en/latest/datasheet.html) | ©2020+ | 2026-08-20 | high |
| [10] | VexRiscv generator softcore | [SpinalHDL/VexRiscv](https://github.com/SpinalHDL/VexRiscv) | push 2026-02 | 2026-08-20 | high |
| [11] | VexiiRiscv succession + tutorial | [SpinalHDL/VexiiRiscv](https://github.com/SpinalHDL/VexiiRiscv) | since 2023-11 | 2026-08-20 | high |
| [12] | Spike ISA simulator | [riscv-isa-sim](https://github.com/riscv-software-src/riscv-isa-sim) | push 2026-05 | 2026-08-20 | high |
| [13] | riscv-formal / RVFI | [YosysHQ/riscv-formal](https://github.com/YosysHQ/riscv-formal) | ongoing | 2026-08-20 | high |
| [14] | Ratified specs library | [riscv.org specifications](https://riscv.org/technical/specifications/) | updated 2026-08-04 | 2026-08-20 | high |
| [15] | NEORV32 + arch-test CI; ecosystem metrics | [stnolting/neorv32](https://github.com/stnolting/neorv32) | push 2026-08 | 2026-08-20 | high |
| [16] | Pipeline hazard teaching (RAW/load-use/branch) | [Vassar CMPU-224 lab](https://cs224.cs.vassar.edu/labs/riscv_pipeline/) | undated | 2026-08-20 | high |
| [17] | Memory-path / sw forwarding stalls | [Rose-Hulman CSSE232 Practical9](https://www.rose-hulman.edu/Class/csse/csse232/Practical9/) | undated | 2026-08-20 | medium |
| [18] | Pipeline assignment staging | [DINOCPU assignment-3](https://raw.githubusercontent.com/jlpteaching/dinocpu/main/assignments/assignment-3.md) | 2019–2020 | 2026-08-20 | high |
| [19] | RVfpga two-semester packaging / barriers | ASEE RVfpga paper (Harris et al., cited in digests) | ~2023 | 2026-08-20 | medium |
| [20] | Numbered labs + RISCOF in monorepo | [sysprog21/ca2025-mycpu](https://github.com/sysprog21/ca2025-mycpu) | CA2025 | 2026-08-20 | high |

## Staleness map

| Claim class | Window | Example claims | Re-check by |
|---|---|---|---|
| version/compatibility | ≤1 mo | Spike/Vex/NEORV32 push dates; priv spec label v20260120 | 2026-09 |
| ecosystem signals | ≤6 mo | stars/issues/last-push tables | 2027-02 |
| landscape | ≤12 mo | teaching vs softcore split | 2027-08 |
| patterns | ≤2 yr | Harris ladder; FemtoRV staging | 2028-08 |

**Earliest re-check:** **2026-09**（版本与规范钉扎）。之后用 Refresh 更新生态表与「是否出现 Rust eDSL RV32 教程核」。
