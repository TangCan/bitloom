---
title: 'technical research: RV32 Episode II pipeline / Zicsr / full RV32I'
type: 'technical'
topic: 'RV32 Episode II: classic 5-stage pipeline + hazards; optional Zicsr + M-mode trap; fuller B-imm / sign-extend / complete RV32I'
decision: 'Choose an implementable Episode II path for a Bitloom teaching RV32 core evolving from a single-cycle / edge-commit subset'
source: 'deep-recon run'
status: complete
preset: 'standard'
validation: 'normal'
created: '2026-08-20'
updated: '2026-08-20'
verified_claims: 5
unverified_claims: 1
---

# technical research: RV32 Episode II pipeline / Zicsr / full RV32I

**Decision this research serves:** Choose an implementable Episode II path for a Bitloom teaching RV32 core evolving from a single-cycle / edge-commit subset — covering classic 5-stage pipeline + hazards (forwarding/stalls), optional Zicsr + M-mode trap, and fuller B-imm / sign-extend / complete RV32I.

## Executive summary

**建议路径（按证据）：** ① 在现有单周期/边沿提交核上**先冻结完整用户态 RV32I**（含符号扩展与正确 B/J 立即数）→ ② 再做**经典 5 级流水 + 转发 + load-use 停顿 + 分支冲刷** → ③ **可选** 精简 M-mode Zicsr/trap（mstatus/mtvec/mepc/mcause/mscratch ± mie/mip + mret）。不要把特权与第一次 hazard 联调绑在同一里程碑。

驱动该结论的三条证据：教学主流是 Harris/FemtoRV 式 5 级 + hazard 单元 [1][7]；load-use 必须停顿、ALU RAW 用转发即可 [2]；课程与开源路径普遍先用户 ISA、后流水、再中断/特权 [5][9][11]。

最大 caveat：FemtoRV 的「精简 CSR 即够」主要来自其 WIP 中断教程 [8]（ledger 中仍标 `unverified` 作产品级合规依据）；完整特权精确异常与 CSR 副作用冲刷是另一座验证悬崖 [15]。

---

## 1. Landscape & maturity

教学向 RV32 流水几乎都收敛到 **Patterson/Hennessy / Harris「超经典」F→D→E→M→W**，而不是多发射或乱序。FemtoRV「Blinker→RISC-V」Episode II 提供最清晰的开源递进：顺序 5 级 → 并发流水 → 控制冒险 → 数据冒险（stall/flush）→ forwarding，并附 CPI/Raystones 表 [1]。Harris 课程材料把 hazard 写成固定三元组：**能转发则转发、load-use 停顿、控制流冲刷**（ForwardAE / StallF·D / FlushD·E）[7]。大学实验（如 Rose-Hulman）克隆同一模式 [10]。

对照谱系：

| 角色 | 代表 | 对 Episode II 的含义 |
|------|------|----------------------|
| 教学递进脊柱 | FemtoRV pipeline*.v [1] | 优先对标的章节结构 |
| 成熟参考核 | VexRiscv（~5 级 + HazardSimplePlugin + CsrPlugin）[26] | 对照黄金行为，不宜当从零 HDL 教材 |
| 尺寸优先对照 | PicoRV32（多周期；自定义 IRQ，非 Privileged）[27] | 说明「小核」常绕过标准 trap |
| 规范基线 | Zicsr 已批准；最简合规平台可为 **仅 M-mode** [6] | 「可选 CSR」与规范不冲突 |

**2024–2026：** Harris/FemtoRV 教学模板稳定；Spinal 栈出现 **VexiiRiscv**（2025 文档、仍 WIP）[12]——参考成熟度在变，教学大纲不应押在其上。小教学核里「完整 Zicsr + 精确 M 异常」少见：FemtoRV 自带部分 mepc/mtvec/mstatus/mcause + mret，但 Episode III 中断文档仍标 WIP 并列出 ECALL、mtime、mscratch、mtval、PLIC/CLINT 等缺口 [8]。

---

## 2. Architecture patterns

### 2.1 五级与数据冒险

命名模式：**IF → ID → EX → MEM → WB**，级间 IF/ID、ID/EX、EX/MEM、MEM/WB 寄存器 [13]。ALU 生产者的 RAW 优先用 **EX/MEM→EX 与 MEM/WB→EX 转发** [2]。**Load-use** 不能单靠转发：load 数据在 MEM 末才就绪，消费者在 EX 初就要用 → **冻结 PC 与 IF/ID、向 ID/EX 插入 bubble（通常 1 拍）**，再做 MEM→EX 转发 [2]。缺 load-use 检测时，即使开了转发也会读到陈旧 RF 值 [14]。

### 2.2 控制冒险

教学第一步标准是 **静态预测不跳转（predict-not-taken）**：顺序取指；分支确认 taken 时 **冲刷** 错误路径指令并改 PC（常在 ID 比较时约 1 bubble）[16]。动态 BTB/预测器属于后续选修。

### 2.3 立即数与 B-imm（完整 RV32I 的前置）

规范意图：立即数一律符号扩展；**符号位固定在指令字 bit31**，便于与译码并行 [3]。B 型相对 S 型：偏移为 2 的倍数，**立即数 bit0 不编码**；重建为 `{inst.bit31, inst.bit7, inst.bits30_25, inst.bits11_8, 1'b0}` 再符号扩展 [3][17]。常见 bug：把 B-imm 当连续字段、或在已拼好的字段上再 `<<1`、或与汇编器字节偏移混淆 [17]。

**架构含义：** 立即数拼装应落在 **译码局部**（产出统一 `imm` 总线），流水级只消费——这样后续插入 pipeline register / hazard 单元不必改 ISA 前端。

### 2.4 可选 Zicsr + M-mode trap

Privileged 列出的 Machine Trap Setup/Handling 至少包括：mstatus、mie、mtvec、mscratch、mepc、mcause、mtval、mip（及 misa 等）[4]。进入 M trap：PC→mepc、原因→mcause、更新 mstatus 特权/IE 栈、可选 mtval，PC←mtvec；退出用 **mret** [6]。教学向「最小集」常见为 **mstatus、mtvec、mepc、mcause、mscratch**，开中断再加 **mie/mip**；mtval 可后补 [18]（教学列表，非规范白名单）。

**勿用 PicoRV32 当标准 CSR 模板**——其 IRQ 走自定义路径 [27]。生产级失败模式：**写 mie/mstatus 使能中断后不冲刷流水** → interrupt skid / 精确异常破坏 [15]；流水核常对 CSR 操作串行化并 flush [19]。

建议分期（不改前端）：

1. 完整 RV32I 译码（I/S/B/U/J imm + 符号扩展）  
2. 插入五级寄存器  
3. 转发 mux  
4. Load-use stall  
5. 分支 flush  
6. 可选：SYSTEM/Zicsr + 像 taken branch 一样的 trap flush  

---

## 3. Implementation reality

### 3.1 课程顺序证据

| 来源 | 顺序信号 |
|------|----------|
| Harris DDCA | 子集 → 单周期 → 多周期 → 5 级 + hazard [9][7] |
| MIT Computation Structures | 建 CPU →（cache）→ pipeline → **稍后** VM/中断 [11] |
| Berkeley EECS151 | ~6 周做出带 hazard 的 3 级 **`rv32ui` 级**功能；cache 可选 [20] |
| Verbeure 形式化核 | 先完整 RV32I；长期推迟 CSR/IRQ [21] |

**与「流水优先于 CSR」偏好一致，且进一步要求：流水之前先冻结用户态 ISA。** 时间紧时可像 EECS 一样「对冻结的 `rv32ui` 列表做流水」，但仍应 **推迟 Zicsr/M-trap**。应避免「8 条玩具指令先流水，再在 hazard 下回头补全部立即数」。

### 3.2 验证阶梯与成本

| 手段 | 教学现实 | 覆盖缺口 |
|------|----------|----------|
| 定向 asm / 寄存器比对 TB | 默认 | 手工维护 |
| `riscv-tests` **`rv32ui`** | 最佳下一步 | **排除 SYSTEM**（无 CSR/trap）[22] |
| RISCOF / arch-test | 合规层；明确「不是完整 DV」[23][24] | 插件+YAML+签名成本高 |
| `riscv-formal` | 对用户 RV32I 流水很强 | wrapper 可假绿；CSR/IRQ 历史上弱；部分证明极长 [21] |

特权引入后会出现第二悬崖：CSR 使能与 in-flight 指令、以及与 Spike 的 CSR RMW cosim 失步 [15][25]。

### 3.3 通常推迟的内容

Cache、MMU、完整 CSR、标准 CLINT/PLIC、完整异常集合——原因是与 datapath 正交，且早期测试套件几乎不覆盖 [4][21][22]。

---

## Cross-dimension insights

1. **景观脊柱 = 架构模式 = 实现顺序：** FemtoRV/Harris 不仅是「流行」，其 hazard 分期与 MIT/Harris 课程顺序互相印证——Episode II 大纲三项不应并行开工。  
2. **「完整 RV32I」是流水的前置，不是并行选修：** B-imm/符号扩展错误在流水下极难与 hazard bug 解耦 [3][17][5]。  
3. **可选 CSR 有双层含义：** 规范允许 M-only + Zicsr [6]；但教学核常只实现子集 [8]，合规与教学「能跑 mret」不是同一目标。  
4. **成熟参考核（VexRiscv）与教学路径脱钩：** 插件化特权栈证明「能做完」，也证明「从零抄它」成本不对 [26][12]。

---

## Contrary evidence / caveats

- **EECS151「先流水较大 `rv32ui`」** 显示不必先做 Harris 全书子集再流水 [20]——但目标仍是用户整数套件，不是 CSR。  
- **精简 CSR 是否「够 Episode II」** 主要依赖 FemtoRV WIP 教程 [8]，未作双源合规验证 → 产品若要宣称 Privileged 合规，证据不足。  
- 公开的「教学团队 6–12 个月工时日记」稀少；成本数字多来自 hobby 形式化与工业 issue。

---

## Recommendations（绑定决策）

| # | 建议 | 置信度 | 下游 |
|---|------|--------|------|
| R1 | Episode II 主线按 **ISA 冻结 → 5 级 + 转发/load-use/分支 flush →（可选）精简 M-CSR/trap** | high（[1][2][5]） | 教程大纲 / sprint 史诗拆分 |
| R2 | 立刻补齐 **I/S/B/U/J 立即数重建 + 符号扩展 + LB/LH 等 load 符号扩展**；用定向测试 + `rv32ui` | high（[3][17][22]） | `rv32_core` / COMPLIANCE |
| R3 | Hazard 单元按 Harris 命名实现；**单独 ATDD load-use** | high（[2][7]） | 章节实验 DoD |
| R4 | CSR 若做：最小集 mstatus/mtvec/mepc/mcause/mscratch（±mie/mip）+ mret；**CSR 写副作用必须 flush**；勿抄 PicoRV32 IRQ | medium–high（[4][15][18]） | Episode II.5 可选章 |
| R5 | VexRiscv 仅作对照；验证阶梯：定向 → `rv32ui` →（可选）RISCOF/formal；勿把 arch-test 绿当作流水正确 | high（[3][22][23][21]） | 质量门槛 |

---

## Open questions

1. Bitloom 当前 builder/`tick` 表面是否足够表达级间寄存器 + 组合转发 mux，还是需要新 HIR 原语？（需对照本仓库架构脊柱，非本次 web 证据。）  
2. Episode II 目标是「能教」还是「riscv-arch-test M-mode 子集绿」？后者会显著抬高 CSR 范围。  
3. 分支比较放在 ID 还是 EX（影响 flush 拍数）——教材两种都有；需按实现语言时序再钉。  
4. On-chip I-fetch（SyncReadMem）与流水 IF 同章还是先外供 instr 口？（前序研究/实现曾推迟片上取指。）

---

## Source appendix

| [n] | 支撑要点 | Publisher | pub | accessed | conf |
|-----|----------|-----------|-----|----------|------|
| [1] | FemtoRV 经典 5 级递进教程 | [Bruno Levy / learn-fpga](https://raw.githubusercontent.com/BrunoLevy/learn-fpga/master/FemtoRV/TUTORIALS/FROM_BLINKER_TO_RISCV/PIPELINE.md) | living | 2026-08-20 | high |
| [2] | 转发优先；load-use 需 stall | [MIT CSAIL 6.823](https://csg.csail.mit.edu/6.823S14/StudyMaterials/pset_pipelining_sol.pdf) | 2014 | 2026-08-20 | high |
| [3] | 立即数符号位@31；B/S 编码理由 | [UC Berkeley EECS-2016-118](https://www2.eecs.berkeley.edu/Pubs/TechRpts/2016/Archive/EECS-2016-118.pdf) | 2016 | 2026-08-20 | high |
| [4] | Machine CSR 地址表 | [RISC-V priv-csrs v20250508](https://docs.riscv.org/reference/isa/v20250508/priv/priv-csrs.html) | 2025-05 | 2026-08-20 | high |
| [5] | 用户 ISA → 流水 → 特权 的教学共识（综） | 见 [9][11][20][21] | — | 2026-08-20 | high |
| [6] | M-mode / trap 语义；Zicsr 基线 | [riscv-privileged PDF](https://docs.riscv.org/reference/isa/_attachments/riscv-privileged.pdf) | living | 2026-08-20 | high |
| [7] | Harris hazard 单元讲义 | [HMC Harris lect22](https://pages.hmc.edu/harris/class/e85/old/fall21/lect22.pdf) | 2020/2021 | 2026-08-20 | high |
| [8] | FemtoRV 精简 CSR / 中断 WIP | [FemtoRV INTERRUPTS.md](https://raw.githubusercontent.com/BrunoLevy/learn-fpga/master/FemtoRV/TUTORIALS/FROM_BLINKER_TO_RISCV/INTERRUPTS.md) | WIP | 2026-08-20 | medium |
| [9] | SC→MC→pipeline 课程 | [Harris DDCArv Ch7](https://pages.hmc.edu/harris/class/e85/DDCArv_Ch7.pdf) | ©2020 | 2026-08-20 | high |
| [10] | 大学 hazard 实验 | [Rose-Hulman CSSE232](https://www.rose-hulman.edu/Class/csse/csse232/Practical9/) | n/a | 2026-08-20 | medium |
| [11] | 流水后再讲中断/VM | [MIT Computation Structures](https://computation-structures.github.io/course/) | living | 2026-08-20 | high |
| [12] | VexiiRiscv 2025 继任 WIP | [VexiiRiscv docs](https://spinalhdl.github.io/VexiiRiscv-RTD/master/artefacts/VexiiRiscv_docs-master.pdf) | 2025-01 | 2026-08-20 | high |
| [13] | 五级命名与停顿机制 | [KFUPM COE501](https://faculty.kfupm.edu.sa/coe/mudawar/coe501/lectures/09-Pipelining.pdf) | course | 2026-08-20 | high |
| [14] | 缺 load-use 的错误结果 | [NTHU Arch sheet](https://www.cs.nthu.edu.tw/~tingting/Archi_25/week11_class_sheet_ans.pdf) | course | 2026-08-20 | high |
| [15] | CSR 使能导致 interrupt skid | [OpenHW CVA6 #3175](https://github.com/openhwgroup/cva6/issues/3175) | issue | 2026-08-20 | high |
| [16] | Flush vs predict-not-taken | [CTU BE35APO](https://cw.fel.cvut.cz/wiki/courses/b35apo/en/tutorials/07/start) | 2026-04 | 2026-08-20 | high |
| [17] | B-imm 重建与常见误解 | [Electronics SE](https://electronics.stackexchange.com/questions/751468/confused-about-the-usage-of-immediate-encoding-of-branch-instructions) | 2025-07 | 2026-08-20 | high |
| [18] | 教学最小 CSR 列表 | [Ecrionix Day-21](https://ecrionix.org/riscv-from-scratch/day-21-csrs-traps-exceptions/) | n/a | 2026-08-20 | medium |
| [19] | CSR 异常串行化模式 | [Taiga commit](https://gitlab.com/sfu-rcl/Taiga/-/commit/669b217ea0304d41cf78aa233439050ce820b748) | commit | 2026-08-20 | medium |
| [20] | EECS151 3-stage + rv32ui | [EECS150 asic-project-sp24](https://raw.githubusercontent.com/EECS150/asic-project-sp24/main/overview.md) | sp24 | 2026-08-20 | high |
| [21] | Formal 路径与 CSR 推迟 | [Tom Verbeure 2018-11-19](https://tomverbeure.github.io/risc-v/2018/11/19/A-Bug-Free-RISC-V-Core-without-Simulation.html) | 2018-11 | 2026-08-20 | high |
| [22] | rv32ui 不含 SYSTEM | [riscv-tests README](https://github.com/riscv-software-src/riscv-tests/blob/master/README.md) | living | 2026-08-20 | high |
| [23] | Arch-test ≠ 完整验证 | [riscv-arch-test](https://github.com/riscv/riscv-arch-test) | living | 2026-08-20 | high |
| [24] | RISCOF 集成税 | [RISCOF docs](https://riscof.readthedocs.io/en/stable/) | living | 2026-08-20 | high |
| [25] | Cosim CSR 失步 | [Chipyard #2320](https://github.com/ucb-bar/chipyard/issues/2320) | issue | 2026-08-20 | medium |
| [26] | VexRiscv README | [SpinalHDL VexRiscv](https://raw.githubusercontent.com/SpinalHDL/VexRiscv/master/README.md) | living | 2026-08-20 | high |
| [27] | PicoRV32 自定义 IRQ | [YosysHQ picorv32](https://github.com/YosysHQ/picorv32) | living | 2026-08-20 | high |

---

## Staleness map

按 pack 窗口，以**本 run 核验日 2026-08-20**为锚（教科书/课程 PDF 的原始 pub_date 不单独触发「过时」——模式类证据以复检日为准）：

| claim class | freshness bar | re-check |
|-------------|---------------|----------|
| version/compatibility（规范 CSR、Zicsr 附件） | ≤ 1 mo | **2026-09-20** |
| landscape（教学核格局、Vexii 状态） | ≤ 12 mo | 2027-08-20 |
| patterns（5 级/hazard） | ≤ 24 mo | 2028-08-20 |
| curriculum / implementation | ≤ 12 mo | 2027-08-20 |

**最早复检：2026-09-20**（规范 CSR 表与 privileged 附件）。用 Deepen/Refresh 更新本 run 即可。

注：若用原始文献 pub_date 喂 `recon_kit staleness`，2014–2016 教材会被标 stale——对「经典流水模式」属假阳性；Refresh 时请以 access/verification 日期入账。
