# NFR14 风险记录 — Epic 29 HLS/IP 闭包定制（FR76 + FR77）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **Phase 9 总览：** `nfr14-risk-phase9-closures.md`（Epic 26.1；本记录为 Epic 29 专用收紧）。  
> **HLS 产品路径：** `nfr14-risk-hls.md`（Epic 24 / FR35+FR50；外挂 Bambu）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 29 后续故事 **29.2–29.4** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR76 + FR77 / Epic 29；NFR14、NFR36 |
| 记录日期 | 2026-09-08 |
| 状态 | accepted |

### (a) 上游约束

- **依赖 Epic 27（FR73）：** HLS/IP 闭包定制建立在 elaborate-time 生成器闭包通路上；无 Epic 27 不得假装 FR76/FR77 完成。
- **FR35 / FR50 产品路径：** HLS 腿必须走已钉死的**外挂** Bambu（见 `nfr14-risk-hls.md` / AD-25）；本 epic **不**另选后端，不引入树内调度。
- **Epic 26.2 决策表 D1（强制）：** HLS 闭包「自由」vs「可综合」**分裂路径** —
  - **HLS 自由闭包**仅允许在 **AD-25 外挂 HLS** 路径与功能侧；须在调度/降低**前**消解（FR76）。
  - **可综合 Bitloom 路径**（comb/seq / elaborate→freeze / IP 可综合腿）一律 **`SynthesizableClosure`**（FR74）；不得借「HLS 自由」绕过。
- **AD-25 / FR86：** **禁止**树内 HLS scheduler / allocation；闭包定制不得把 scheduler 偷渡进工具链。
- **Epic 34 IP 基线优先于 29.3：** FR77（IP 生成器闭包定制）建议叠在 **非 stub、无闭包** 的可综合 IP 基线（FR82 / Epic 34）之上；在 Epic 34 基线未就绪时，不得将 **29.3** 标 `ready` 或用 stub 重命名冒充定制完成。**（2026-09-08：Epic 34 / FR82 五类基线已交付并关闭；29.3 可依赖本基线开工。）**
- **NFR36 / D4：** freeze 后 FrozenHir / Verilog / FIRRTL / Chisel / tick **无**闭包残留；后端不编码闭包 IR。
- **品牌 / 依赖：** 公开产品 **Bitloom**；设计 crate 只依赖 **`bitloom-prelude`**。

### (b) 粗工期带

- **预计：** Epic 29 整体约 **0.5–1.5 人周**（29.1 本风险记录 ≤0.25 人周；29.2 HLS 数据流闭包 0.25–0.75 人周；29.3 IP 闭包定制 0.25–0.5 人周且受 Epic 34 排程牵制；29.4 透明抽检与文档 ≤0.25 人周）。
- **置信度 / 假设：** 中；假设 Epic 24 HLS 外挂路径与 Epic 27 生成器通路可用，D1 不回退；**29.3** 工期假设 Epic 34 至少交付一种可 elaborate 的非 stub IP（FIFO/UART 或文档等价）。若 34 滞后，29.3 须显式 defer，不得静默砍完成定义。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **29.2–29.4** 标 `ready` 或开工实现。
- 不得引入树内 HLS scheduler / allocation（**AD-25 / FR86**），或以「闭包定制需要调度」为借口在树内落地 scheduler。
- 不得把 **D1 HLS 自由闭包**偷用到可综合 Bitloom / IP 可综合腿，绕过 **`SynthesizableClosure`**（FR74）。
- 不得在 **Epic 34 IP 基线未就绪** 时把 **29.3** 标 ready，或仅重命名 / 包一层 stub 冒充 FR77 闭包定制完成（**测序：Epic 34 应先于 29.3**）。
- 不得以「后端缺失」silent skip / 假成功冒充 FR76 HLS 路径完成（须可读失败）。
- 不得在 FIRRTL / Chisel / Verilog / tick 中残留闭包语义或编码闭包节点（NFR36）。
- 不得把 FR47 sim generators / Phase 7「闭环」冒充本 epic 的 HLS/IP **闭包定制**完成定义。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）
- 备份 / 升级路径：D1 / AD-25 边界升级至 ARCHITECTURE-SPINE 维护者；IP 测序（34→29.3）争议升级至产品 / Phase 10 排程负责人。

### 约束类摘要（实现指针）

| 路径 | 允许的闭包约束类 | 消解时机 | 非目标 |
| --- | --- | --- | --- |
| AD-25 外挂 HLS（FR76） | D1：**HLS 自由**（无状态或本记录允许的约束类） | 调度/降低**前** | 树内 scheduler（AD-25 / FR86） |
| 可综合 Bitloom / IP 腿（FR77） | **`SynthesizableClosure`**（FR74） | elaborate → freeze 前 | 借 HLS 自由绕过 FR74 |
| 功能侧 | 普通 Rust 闭包（不进 FrozenHir） | 不进入 HIR | 与周期精确路径混写完成定义 |

### 并行 / 维护叠加（Chipyard 式）

- 与 Epic 30（桥接闭包）及 Phase 10（Epic 31–35）并行时：文档「支持功能」表须区分 HLS 自由 vs SynthesizableClosure vs IP 无闭包基线（Epic 34）；避免两边各自发明完成话术。
- **测序硬约束：** Epic 34 IP 基线 **应先于** Story **29.3**；29.2（HLS）可与 34 并行，但不得把 29.3 的阻塞伪装成 29.2 完成。
- 与 Epic 24 HLS 烟测 / optional Bambu job 叠加时：闭包夹具失败不得 `continue-on-error` 假装绿。

### 引用

- AD-28 — 风险门禁（NFR14）
- AD-25 / FR86 — HLS 仅外挂；**无树内 scheduler**
- AD-18（Revised）— 禁捕获；elaborate-time 非捕获须冻前消解
- PRD NFR14 / FR76 / FR77 / NFR36；FR35 / FR50（HLS 产品路径）
- 决策表 D1：`architecture/architecture-rhdl-2026-08-18/closure-decision-table-2026-09-08.md`
- Phase 9 总览：`nfr14-risk-phase9-closures.md`
- HLS 产品路径：`nfr14-risk-hls.md`
- Epic 34（FR82）— 无闭包 IP 基线，**优先于 29.3**
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 29 故事 29.2–29.4 标 `ready`。**
