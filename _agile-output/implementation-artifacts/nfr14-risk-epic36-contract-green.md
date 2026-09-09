# NFR14 风险记录 — Epic 36 合同绿 / 路线图重定义（FR87 / FR93 / NFR38）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 11 **NFR38**（合同绿 ≠ 字面七阶段）。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic35-residual-partials.md`；`nfr14-risk-epic34-ip-baseline.md`。  
> **前置：** Phase 1–10（Epic 1–35）complete；Correct Course `sprint-change-proposal-2026-09-09.md` **approved**（2026-09-09）。  
> **调研：** `planning-artifacts/research/technical-doc19-seven-stage-full-green-product-pla-2026-09-09/research.md`。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 36 后续故事 **36.2–36.3** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR87 / FR93 / Epic 36；NFR14、NFR38 |
| 记录日期 | 2026-09-09 |
| 状态 | accepted — Story 36.1；36.2–36.3 仍 backlog 直至本记录门禁满足后标 ready |

### (a) 上游约束

- **Correct Course 批准（2026-09-09）：** `sprint-change-proposal-2026-09-09.md` 将「产品做完 / `docs/requirements/19` 七阶段全绿」定义为 **合同绿**（可验收重定义完成标准），正式 Phase 11（Epic 36–39 · FR87–FR93 / NFR38–NFR39）。窄范围：只改写路线图阶段五–七「绿」标签与剩余缺口验收。
- **与 2026-08-21 ①C 的关系（必须同时读）：**
  - ①C 升格的 FR46–52 及后续深度 FR71–FR86 **仍然有效**；已交付验收 **不回滚**。
  - ①C「拒绝重定义 done」适用于当时「概述愿景是否升格为硬 FR」的决策，**不**禁止其后用合同条款定义 **路线图阶段五–七的「绿」标签** 与剩余生态缺口。
  - 对外「概述愿景已全部兑现 / 七阶段字面全绿」类表述：在合同绿落地前 **禁止**；落地后仅可按 **FR87 / NFR38** 合同条款宣称。
- **调研结论：** 字面全做（自研 HLS、idiomatic Chisel 往返、自动 TLM≡CA、一等 netlist LSP、VIP 全协议 IP）同业亦未作为核心产品一次兑现；合同绿分波交付（Wave A=本 epic）。
- **本 epic 范围：** FR87（doc-19 P5–P7 完成定义重写）+ FR93（永久非目标锁定）；**不**实现 Epic 37–39 加深。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **NFR38：** 公开/内部「路线图全绿 / 产品做完」表述必须引用本阶段重定义条款；不得仅以概述字面未交付项宣称完成。

### (b) 粗工期带

- **预计：** Epic 36 整体约 **0.5–1.5 人周**（36.1 本风险记录 ≤0.25 人周；36.2 重写 doc-19 P5–P7 完成定义 + README/deferred 交叉链 0.25–0.75 人周；36.3 永久非目标锁定文案 + addendum 指针 0.1–0.5 人周）。
- **置信度 / 假设：** 高；本 epic 主要为文档/合同对齐，不依赖新 RTL 路径。若要求同步改写 `docs/requirements/` 其他概述文档（超出 FR87 钉死的 19 + 交叉链），工期上修。Epic 37–39 工期**不**计入本记录。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **36.2–36.3** 标 `ready` 或开工实现。
- **不得未改文档（至少未按 FR87 更新 `docs/requirements/19. 实施路线图.md`）就宣称七阶段字面全绿 / 产品做完。**
- **不得把永久非目标（FR93：树内 HLS 调度器；FIRRTL→idiomatic Scala；默认 TLM≡CA 形式证明；VIP 级全协议 IP；按键全设计 elaborate 的 netlist LSP）标成 done** 或静默交付交差。
- 不得回滚或改写已交付 **FR46–FR86** 验收以「配合」合同绿叙事。
- 不得把本阶段「合同绿」冒充 ①C 概述字面愿景已全部兑现（**NFR38：合同绿 ≠ 字面七阶段**）。
- 不得在未改 PRD / 本记录的前提下把 FR87/FR93 静默砍掉或改回「字面七阶段一次性全绿」而不改合同。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR38** 合同绿诚实度共同责任人
- 备份 / 升级路径：合同绿 vs 字面绿表述争议升级至产品 / Correct Course 批准人；永久非目标清单增删须 **新 PRD**（FR93），升级至架构 / PM。

---

### 与 2026-08-21 ①C 对照（关系说明）

| 维度 | 2026-08-21 ①C | Phase 11 / Epic 36（本记录） |
| --- | --- | --- |
| 决策对象 | 概述愿景是否升格为硬 FR（FR46–52） | 路线图 **P5–P7「绿」标签** 与剩余缺口验收标准 |
| 「重定义 done」 | **拒绝**（当时）— 概述字面硬 FR | **批准窄范围** — 只改写路线图完成标签；**不回滚** FR46–FR86 |
| 已交付 FR | 其后 Phase 7–10 深度交付 FR71–FR86 | **保持有效**；禁止回滚验收 |
| 「全绿」话术 | 概述愿景兑现门槛 | 仅可按 **FR87 / NFR38** 合同条款宣称；禁止字面勾选未交付项 |

### Epic 36 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **36.1** | 本 NFR14 风险记录 + ATDD | **本故事** |
| **36.2** | 重写 doc-19 P5–P7 完成定义（FR87） | Gate：须本记录后才可 ready |
| **36.3** | 永久非目标锁定文案（FR93） | Gate：须本记录后才可 ready；关闭时勾选下节 |

### 并行 / 维护叠加（Chipyard 式）

- 与 Epic 37–39 并行前：建议先合入 Epic 36 合同文本（软依赖）；不得在缺 FR87 定义时对外宣称 Phase 11「全绿」。
- 文档面叠加：`19. 实施路线图.md`、README、`deferred-work.md`、PRD addendum 须保持同一合同绿叙事，禁止「代码/规划 done、路线图仍字面甘特」分裂话术。
- 不引入新 RTL / 闭包路径；若后续 epic 触及 HLS / Chisel / LSP / IP，须遵守 FR93 永久非目标与各 epic 自有 NFR14。

### 引用

- AD-28 — 风险门禁（NFR14）
- PRD NFR14 / FR87 / FR93；NFR38（合同绿 ≠ 字面七阶段）
- Correct Course：`sprint-change-proposal-2026-09-09.md`（approved）
- Research：`technical-doc19-seven-stage-full-green-product-pla-2026-09-09/research.md`
- Addendum：`prd-rhdl-2026-08-19/addendum.md` — Phase 11 合同绿 / ①C 边界
- 体例：`nfr14-risk-epic35-residual-partials.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 36 关闭条件（Story 36.3 勾选）

- [ ] **FR87：** `docs/requirements/19. 实施路线图.md` P5–P7「绿」已改为合同条款（外挂 HLS + 薄 IP + VCD/层次；精选 IP 深度 + 文档站 + 宿主 LSP；同刺激多视图 + adapter 模板；禁字面未交付勾选）
- [ ] **FR93：** README 与/或 `deferred-work.md` + addendum 指针锁定永久非目标清单；须新 PRD 才能推翻
- [ ] **NFR38：** 公开/内部「全绿」表述引用合同条款；无字面七阶段假完成话术
- [ ] **禁止事项未触发：** 无未改文档宣称字面全绿；无永久非目标标 done；无回滚 FR46–FR86；无提前标 36.2–36.3 ready（对本记录而言）
- [ ] **品牌：** 仍为 Bitloom / `bitloom-*`

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 36 故事 36.2–36.3 标 `ready`。**  
**Epic 36 关闭条件（上节）须由 Story 36.3 勾选。**
