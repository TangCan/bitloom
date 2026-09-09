# NFR14 风险记录 — Epic 31 CDC 同步器真 RTL（FR79 / AD-29）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；**AD-29**（CDC 须可综合真 RTL）；Phase 10 **NFR37**（规划 done ≠ 深度 done）。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **历史最小合同：** Epic 7 / FR23 + Epic 19 FR52 产品夹具 — phantom 域、非法跨域失败、ZST 叙事锚点 + `mark_cdc_bridge`。  
> **体例对照：** `nfr14-risk-epic30-bridge-adapter-closures.md`；`nfr14-risk-epic34-ip-baseline.md`；`nfr14-risk-phase9-closures.md`。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 31 后续故事 **31.2–31.4** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR79（加深 FR23/FR52）/ Epic 31；NFR14、NFR37；AD-29 |
| 记录日期 | 2026-09-09 |
| 状态 | accepted — Epic 31 关闭条件已由 Story 31.4 勾选 |

### (a) 上游约束

- **AD-29：** 语言级 `DoubleFlop` / `SyncFIFO`（或文档等价）在 Phase 10 / Epic 31 深度验收中必须 **emit 可综合同步器 RTL**，并支持按域 tick 黄金夹具。phantom 域与非法跨域失败（既有 FR23）仍然有效，但**不足以单独交差**深度合同。
- **crates 现状（诚实基线 → Story 31.2–31.3 已兑现）：** `DoubleFlop` / `SyncFIFO` 类型本身仍可为 **ZST 标记**，但 `::elaborate()` 已 emit 可识别同步/FIFO 网表（两级 `sync_ff*`；DEPTH=4/WIDTH=8 灰码 SyncFIFO）。FR52 `examples/clockdomain_skel` 仍演示 **仅** `mark_cdc_bridge` 最小合同，**不得**冒充 FR79 深度（NFR37）。
- **与历史 Epic 7「done」冲突的诚实度风险（NFR37）：** Epic 7 交付了跨域诊断、prelude 标记名与（部分）注释形 emit；sprint 可将 Epic 7 标 `done`，但那是 **FR23 最小合同**（诊断 + 标记路径），**≠** FR79 / AD-29 **真 RTL 深度**。公开/内部文档若仅以 Epic 7 / FR52「done」关闭 CDC 深度，构成 **诚实度失败** — 须以本 epic FR79 验收，不得静默沿用历史话术。
- **双 FF 延迟 / 亚稳态文档边界（强制写清，留给 31.2–31.4 实现兑现）：**
  - **合同内：** 同步器级数（默认两级，或文档钉死 N 级）、源域→目的域的 **可观测延迟（以目的域 tick 计）**、黄金夹具期望值、非法未标记跨域仍 `E0220`（或等价）失败。
  - **合同外 / 不得过度承诺：** 不保证消除真实硅片亚稳态概率；不提供 MTBF 公式或工艺相关时序签核；不把「双 FF」写成对所有异步输入的万能安全证明。文档须区分 **RTL/仿真语义延迟** vs **物理亚稳态风险提示**。
- **继承且保留：** Clash 式 phantom `ClockDomain`；默认模块仍单时钟 + sync active-high Reset（AD-15/AD-22）；非法跨域 freeze 失败不得回退。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**；公开品牌 **Bitloom**。
- **NFR37：** 规划/sprint `done` ≠ 深度 done；深度缺口以 **FR79** 验收。

### (b) 粗工期带

- **预计：** Epic 31 整体约 **1–2.5 人周**（31.1 本风险记录 ≤0.25 人周；31.2 DoubleFlop 可综合 RTL + 按域 tick 0.5–1 人周；31.3 SyncFIFO 或文档等价 0.5–1 人周；31.4 ATDD + 文档叙事收口 0.25–0.5 人周）。
- **置信度 / 假设：** 中；假设沿用既有 `ElaborateSession` / emit / `Sim::tick`，且「真 RTL」定义为 emit `.v`（及可选 FIRRTL）含可识别同步/FIFO 结构 + 黄金延迟语义。若要求多级可配置同步器全家桶、完整异步 FIFO 灰码证明或形式化 CDC，工期显著上修，须改本记录而非静默缩范围。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **31.2–31.4** 标 `ready` 或开工实现。
- **不得仅改文档 / README / 注释**声称 `DoubleFlop`/`SyncFIFO` 已是真 RTL，而不 emit 可识别同步器结构与按域 tick 黄金（FR79 / AD-29 / NFR37）。
- 不得把 Epic 7「done」、FR52 产品夹具或 ZST + `mark_cdc_bridge` 冒充 FR79 深度关闭（**NFR37：规划 done ≠ 深度 done**）。
- 不得仅保留空 ZST 无网表，或只加注释形 emit 而无真实寄存器链路交差。
- 不得删除或弱化非法跨域失败（E0220 等）来换取「合法路径」看起来简单。
- 不得把物理亚稳态/MTBF 签核写进默认完成定义，或反过来用「我们不谈亚稳态」回避文档化 **双 FF 延迟语义边界**。
- 不得把一级 IP SyncFifo（Epic 34）与 CDC `SyncFIFO` 原语完成话术混成同一条，而不区分跨域同步器 vs IP 基线。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR37** 深度诚实度共同责任人
- 备份 / 升级路径：双 FF 级数/延迟语义或「真 RTL」定义争议升级至 ARCHITECTURE-SPINE（AD-29）维护者；与 Epic 7 历史话术冲突升级至产品 / Phase 10 排程负责人。

### 历史最小合同 vs FR79 深度（对照）

| 维度 | Epic 7 / FR23（+ FR52 夹具） | Epic 31 / FR79（AD-29） |
| --- | --- | --- |
| 完成话术 | phantom 域 + 非法跨域失败 + DoubleFlop/SyncFIFO **标记名**；`mark_cdc_bridge` | emit **可综合同步器 RTL** + 按域 tick 黄金 |
| prelude 形态 | ZST 叙事锚点 + `mark_cdc_bridge` | ZST 标记 **+** elaborate 真网表（不得仅 bridge） |
| 诚实度 | sprint 可 `done`（最小合同） | **不得**用该 done 关闭深度（NFR37） |
| 亚稳态 | 多未钉死 | 文档钉死 **延迟语义**；物理 MTBF **非**默认合同 |

### 并行 / 维护叠加（Chipyard 式）

- 与 Epic 34（一级 IP FIFO）并行时：文档须区分 **CDC SyncFIFO 原语** vs **IP FIFO 基线**，避免两边各自发明「FIFO 已完成」话术。
- 与 Phase 9 闭包 / 桥接 epic 并行时：CDC 真 RTL 不得引入闭包对象进 `tick`（AD-18 / NFR35）；同步器是硬件结构，不是适配器模板。
- 升钉 emit / tick 语义时须同步 DoubleFlop 与 SyncFIFO（或已书面降级的子集）夹具，禁止「只修 DoubleFlop、文档仍写两者都真」。

### 引用

- AD-28 — 风险门禁（NFR14）
- AD-29 — CDC 原语须可综合真 RTL（深度）
- AD-22 / AD-15 — phantom 域；默认单时钟
- PRD NFR14 / FR23 / FR52 / FR79；NFR37（规划 done ≠ 深度 done）
- `bitloom-prelude` — `DoubleFlop` / `SyncFIFO`（ZST 标记 + elaborate 真 RTL）；FR52 `mark_cdc_bridge` 仍保留
- 跟练 / 收口：`docs/tutorials/cdc-depth.md`；ATDD `fr79_cdc_depth_closeout`
- 体例：`nfr14-risk-epic30-bridge-adapter-closures.md`；`nfr14-risk-epic34-ip-baseline.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 31 关闭条件（Story 31.4 勾选）

- [x] **FR79 DoubleFlop：** emit `.v` 含可识别两级（或文档钉死级数）同步寄存器；按域 tick 黄金满足延迟语义
- [x] **FR79 SyncFIFO（或等价）：** 可 elaborate/emit/tick；深度/宽度文档化；跨域满/空（或书面最小子集）正确
- [x] **负例保留：** 未标记非法跨域仍失败（继承 FR23）
- [x] **NFR37：** 相对 Epic 7 / ZST+`mark_cdc_bridge` 历史已文档化；不得用历史 `done` 冒充深度关闭
- [x] **ATDD / 配方：** DoubleFlop + SyncFIFO 黄金 + 负例（亦含于 `just test` 或文档化配方）
- [x] **禁止事项未触发：** 无仅改文档声称真 RTL；无仅 ZST 无网表交差；无静默砍掉 SyncFIFO 却宣称 FR79 全完成

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 31 故事 31.2–31.4 标 `ready`。**  
**Epic 31 关闭条件（上节）已由 Story 31.4 勾选（2026-09-09）。**
