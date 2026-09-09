# NFR14 风险记录 — Epic 39 宿主 IDE 与多视图同刺激（FR90 / FR91 / FR92）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；**AD-5**（双模型仿真；SystemC TLM-2.0 非合同）；Phase 11 **NFR39**（禁止静默扩大子集）。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic38-uarttx-deepen.md`；`nfr14-risk-epic37-interop-hls.md`。  
> **前置：** Epic 36 合同绿 / FR93 永久非目标已锁定；Epic 38 已关闭（`60588ae`）；`docs/fr38-viz-lsp.md` 与 deferred-work 已声明完整 LSP deferred、层次 HTML ≠ LSP。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 39 后续故事 **39.2–39.4** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR90, FR91, FR92 / Epic 39；NFR14、NFR39；对照 AD-5 / FR93 |
| 记录日期 | 2026-09-09 |
| 状态 | accepted — Story 39.1；FR91 分支 B 钉死；关闭条件留给 39.4 |

### FR91 分支选择（Story 39.1 钉死 · 必读）

| 选项 | 含义 | 本 epic 决策 |
| --- | --- | --- |
| **(A) 浅层 Bitloom LSP MVP** | 交付浅层（非按键全 elaborate）诊断/符号 LSP + 最小编辑器接线 | **不选用** |
| **(B) 显式 defer Bitloom LSP** | 合同化 defer：自研 Bitloom LSP 本阶段不交付；不得声称 LSP 已完成 | **选用** |

**选定：分支 B — 显式 defer Bitloom LSP（Path B）。**

**选型理由（可行性与证据）：** epics.md Epic 39 **Assumption** 默认走分支 B；仓库现状无 language-server 二进制；`docs/fr38-viz-lsp.md` / deferred-work 已声明完整 LSP deferred；FR93 永久非目标含「按键全设计 elaborate 的 netlist LSP」。本阶段 Wave D **必做**为 FR90 宿主 rust-analyzer 路径，FR91 为可选——无半成品浅层 LSP 可落地，**不**强行选 A。改选 A 须改本记录、扩大 39.3 范围并产品确认。

### (a) 上游约束

- **FR90（必做）：** 文档化 Bitloom 设计 crate 的 **rust-analyzer / 宿主 LSP** 工作流（补全、跳转、诊断）；验收=可复现步骤 + 至少一夹具工程说明；**不**要求硬件语义 netlist LSP。
- **FR91（可选 · 本记录钉死 B）：** 本 epic **显式 defer** 自研 Bitloom LSP；39.3 须更新 README / `docs/fr38-viz-lsp.md`（或等价）合同化 defer，并声明不得声称 LSP 已交付；**不得**交付未文档化的半成品 LSP 二进制冒充完成。
- **FR92：** 功能模拟路径与周期精确 `tick`（及/或 `build-sim`）共享刺激/记分板夹具；桥接 adapter **模板**（文档+代码骨架）；**不**承诺自动形式 FL≡RTL / SystemC TLM-2.0 产品（**AD-5**）。
- **FR93 对照（不得吞掉）：** 按键全 elaborate netlist LSP 为永久非目标；层次/时序 HTML（FR38/FR49）**不**计入 LSP 完成。
- **AD-5：** 双模型仿真合同；SystemC TLM-2.0 **不是**合同交付物。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **NFR39：** 禁止静默扩大子集；未选分支 A、形式等价、TLM 产品不得冒充完成。
- **与 deferred-work：** 完整 LSP hover/goto 继续 deferred；本 epic 仅合同化 FR91 Path B + FR90/FR92 交付；39.3/39.4 交叉引用收口。

### (b) 粗工期带

- **预计：** Epic 39 整体约 **1–2.5 人周**（39.1 本风险记录 ≤0.25 人周；39.2 宿主 IDE 文档+夹具说明 0.25–0.75 人周；39.3 Path B defer 文档收口 0.15–0.5 人周；39.4 同刺激夹具+adapter 模板 0.5–1.25 人周）。
- **置信度 / 假设：** 中；假设沿用既有功能 sim / `tick` / `build-sim` 表面，不引入第二套仿真语义。若改选 FR91 分支 A（浅层 LSP），工期显著上修且须改本记录 / 扩大 39.3。Epic 38 IP 加深工期**不**计入本记录。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **39.2–39.4** 标 `ready` 或开工实现。
- **不得承诺 SystemC TLM-2.0**（或等价 TLM 产品）作为本 epic 关闭条件（AD-5）。
- **不得声称自动 FL≡RTL 形式证明**（或默认 TLM≡CA）作为 FR92 完成定义。
- **不得把层次 / 时序 HTML（FR38/FR49）冒充 LSP 完成**（FR90/FR91）。
- **分支 B 下不得交付未文档化的半成品 LSP 二进制**冒充 FR91 完成（NFR39）。
- 不得在未改 PRD / 本记录的前提下把 FR91 静默改写为「已交付浅层 LSP」而不走分支 A 合同。
- 不得把宿主 rust-analyzer 工作流冒充「Bitloom 自研硬件语义 LSP」。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR39** 子集边界共同责任人
- 备份 / 升级路径：FR91 改选 A（浅层 LSP MVP）或扩大到全 elaborate netlist LSP 须升级至产品 / PM 并改本记录与 PRD；与 AD-5 / FR93 冲突升级至 AD-28 维护者。

---

### FR90 / FR91 / FR92 边界对照

| 维度 | 本 epic 合同 | 明确非目标 / 非关闭条件 |
| --- | --- | --- |
| FR90 | 宿主 rust-analyzer 可复现工作流 + 夹具说明 | 硬件语义 / netlist LSP |
| FR91 | **分支 B：** 显式 defer 自研 Bitloom LSP | 浅层 MVP（A）本 epic 不交付；全 elaborate LSP（FR93） |
| FR92 | 同刺激夹具 + adapter 模板 | 自动 FL≡RTL；SystemC TLM-2.0（AD-5） |
| 可视化 | 层次/时序 HTML 已有（FR38/FR49） | **不**计入 LSP 完成 |

### 风险主题对照（本 epic）

| 主题 | 风险 | 缓解（本 epic 故事） |
| --- | --- | --- |
| 自研 LSP 冒充关闭 | HTML / 宿主 IDE 被写成「LSP 已交付」 | 39.1 钉死 B；39.3 合同化 defer；ATDD |
| 形式等价 / TLM 偷换 | FR92 写成 TLM≡CA 或 SystemC 产品 | 禁止清单 + AD-5；39.4 文档诚实 |
| 半成品 LSP 二进制 | 未接线的 server 冒充 FR91 | 分支 B 禁止未文档化半成品；NFR39 |
| 未开门禁开工 | 39.2–39.4 提前 ready | Gate 句 + ATDD |

### Epic 39 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **39.1** | 本 NFR14 风险记录 + ATDD；钉死 FR91 **分支 B** | **本故事** |
| **39.2** | 宿主 IDE / rust-analyzer 工作流文档 + 夹具说明（FR90） | Gate：须本记录后才可 ready |
| **39.3** | FR91 Path B：README / fr38 显式 deferred；无半成品 LSP 冒充 | Gate：须本记录后才可 ready |
| **39.4** | 多视图同刺激 + adapter 模板（FR92）；勾选关闭条件 | Gate：须本记录后才可 ready |

### 并行 / 维护叠加（Chipyard 式）

- 与 Epic 38（已关闭）并行叙事：IDE / 多视图合同**不**依赖 UART 加深 API；IP 与 IDE 分离。
- 与 FR38 可视化叠加：须同一叙事——HTML ≠ LSP；禁止「有 hierarchy.html 即 LSP done」。
- 与 deferred-work LSP 条目：本 epic Path B 强化 defer，**不**静默关闭为「已实现」。

### 引用

- AD-28 — 风险门禁（NFR14）
- AD-5 — 双模型仿真；SystemC TLM-2.0 非合同
- PRD NFR14 / FR90 / FR91 / FR92；NFR39；FR93 永久非目标
- `docs/fr38-viz-lsp.md` — LSP deferred；HTML ≠ LSP
- deferred-work — 完整 LSP hover/goto 后续 epic
- 体例：`nfr14-risk-epic38-uarttx-deepen.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 39 关闭条件（Story 39.4 勾选）

- [ ] **FR90：** 宿主 rust-analyzer（或等价）工作流文档 + 至少一夹具工程说明可复现 — Story 39.2
- [ ] **FR91：** 分支 **B** 合同化 defer（README / `docs/fr38-viz-lsp.md` 或等价）；无半成品 LSP 二进制冒充完成 — Story 39.3
- [ ] **FR92：** 同刺激夹具 + adapter 模板；文档写明不承诺自动 FL≡RTL / SystemC TLM-2.0 — Story 39.4
- [ ] **HTML ≠ LSP：** 层次/时序 HTML 不计入 LSP 完成
- [ ] **禁止事项未触发：** 无 TLM-2.0 / 形式等价宣称；无 HTML 冒充 LSP；无未文档化半成品 LSP
- [ ] **品牌 / 依赖：** 仍为 Bitloom；设计 crate 只依赖 `bitloom-prelude`

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 39 故事 39.2–39.4 标 `ready`。**  
**FR91 分支已钉死为 B（显式 defer Bitloom LSP / Path B）；Epic 39 关闭条件由 Story 39.4 勾选。**
