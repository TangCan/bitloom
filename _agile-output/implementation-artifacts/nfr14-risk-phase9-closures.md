# NFR14 风险记录 — Phase 9 受控泛型闭包（Epic 26–30 / FR72–78）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 9 扩展至 **FR72–FR78** / **NFR35–NFR36**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Story **26.3–26.4** 及 **Epic 27+**（含 27.x–30.x）标为 `ready`，亦不得开工实现正向闭包 API。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR72–FR78（澄清 FR16）/ Epic 26–30；NFR35–NFR36 |
| 记录日期 | 2026-09-08 |
| 状态 | accepted |

### (a) 上游约束

- **FR16 / AD-18 冲突面：** 现行 AD-18 与 FR16 **拒绝捕获闭包**进入周期精确路径。Phase 9 仅允许 **elaborate-time 非捕获 `Fn`（或文档等价）且必须在 `freeze` 前消解为 HIR**；在 **26.3 修订 AD-18 落地前**，不得合并任何正向闭包表面 API。
- **术语消歧（强制）：** **生成器闭包**（elaborate-time 初值/工厂）≠ **FR47** 双视图 / sim **generators** ≠ Phase 7 英文 overview「**closure** / 闭环」。文档与故事标题不得把三者写成同一完成定义。
- **诊断逃逸：** 捕获硬件引用、逃逸到 `tick`/后端的闭包对象、或 FIRRTL/Chisel 侧「闭包节点」均须可诊断失败；不得 silent 成功。
- **后端合同（Cap-R-58）：** Verilog / FIRRTL / Chisel / tick **不得**编码闭包 IR；闭包只存在于 elaborate 期。
- **品牌 / 依赖：** 公开产品 **Bitloom**；设计 crate 只依赖 **`bitloom-prelude`**；遵守 AD-1（非 rustc 期抽网表）与 AD-7/13（经 `ElaborateSession`）。
- **FR86 / AD-25：** 树内 HLS 调度仍非目标；HLS 闭包定制（Epic 29）受 Epic 26.2 决策表与外挂 Bambu 路径约束，不得借闭包故事引入树内 scheduler。

### (b) 粗工期带

- **预计：** Phase 9 整体约 **3–6 人周**（26.1 风险记录 ≤0.25 人周；26.2 决策表 ≤0.5；26.3–26.4 合同修订 ≤0.5；Epic 27 生成器 1–2 人周；Epic 28 可综合内联 1–2 人周；Epic 29–30 各 0.5–1.5 人周，且 Epic 29 依赖 Epic 34 IP 基线优先）。
- **置信度 / 假设：** 中；假设 26.2 明确 HLS 自由 vs 可综合裁决，且不把 const fn 双轨与生成器闭包混为实现。诊断矩阵与 FR16 负例回归是主要不确定项。

### (c) 禁止的静默降级清单

- 不得在 **未修订 AD-18**（Story 26.3）前合并正向闭包 API 或将 Epic 27+ 标 `ready`。
- 不得在 **FIRRTL / Chisel**（或任何后端 IR）中编码闭包节点。
- 不得把 **FR47** sim generators / 双视图生成器冒充 **生成器闭包**完成，或把 Phase 7「闭环」冒充本主题 done。
- 不得允许捕获硬件引用的闭包进入 freeze/`tick` 而不失败。
- 不得在缺本记录（或缺 a–d）时将 **26.3–26.4** 或 **Epic 27+** 标 `ready` 或开工实现。
- 不得借 Epic 29 闭包定制引入树内 HLS scheduler（AD-25 / FR86）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）
- 备份 / 升级路径：AD-18 / FR16 语义争议升级至 ARCHITECTURE-SPINE 维护者；HLS 边界升级至 AD-25。

### 并行 / 维护叠加（Chipyard 式）

- 与 Phase 10（Epic 31–35）并行时：闭包实现不得阻塞 CDC/IP 深度故事，但 **Epic 29 应在 Epic 34 IP 基线之后**；文档「支持功能」表避免两边各自发明完成话术。
- FR16 负例矩阵与新闭包正例必须共存回归，防止诊断逃逸在并行改动中被冲掉。

### 引用

- AD-28 — 风险门禁（NFR14）
- AD-18 — 语言表面 / 捕获闭包（待 26.3 修订）
- AD-25 / FR86 — HLS 仅外挂
- PRD NFR14；Phase 9 FR72–78 / NFR35–36（`epics.md` inventory → 26.4 写入 PRD）
- 调研：`technical-requirements-implementation-gap-generic-2026-09-08/research.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Story 26.3–26.4 及 Epic 27+ 标 `ready`。**
