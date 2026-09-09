# NFR14 风险记录 — Epic 40 字面绿合同翻转（Path B / FR94）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 12 **NFR40–NFR43**；闸门 **FR94**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic36-contract-green.md`；`nfr14-risk-epic39-ide-multiview.md`。  
> **前置：** Phase 11（Epic 36–39）合同绿 complete；Correct Course `sprint-change-proposal-2026-09-09-phase12-path-b.md` **approved**（2026-09-09）；PRD addendum Phase 12 段落已落地。  
> **调研：** `planning-artifacts/research/technical-doc19-seven-stage-full-green-product-pla-2026-09-09/research.md`（**建议勿走字面全绿** — 已被产品 **B1** 强制否决）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 40 后续故事 **40.2–40.4** 标为 `ready`，亦不得开工实现。**Epic 40 未关闭 / FR94 未合入前，Epic 41–47 不得标 `ready`。**

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR94 / Epic 40；NFR14、NFR40、NFR41、NFR42、NFR43；对照 FR87/FR93/NFR38 |
| 记录日期 | 2026-09-09 |
| 状态 | accepted — Story 40.1；关闭条件待 40.4 勾选 |

### B1 强制选择与 research 冲突说明（必读）

| 选项 | 含义 | 本阶段决策 |
| --- | --- | --- |
| **合同绿唯一口径（Phase 11）** | 「产品做完」= FR87/NFR38；FR93 永久非目标锁定 | **不再作为唯一完成口径**（历史里程碑保留） |
| **B1 / Path B 字面七阶段全绿** | 推翻 FR93；以 FR94–105 为字面完成合同 | **强制选用** |

**选定：B1 — 字面七阶段全绿（Path B）。**

**与 research 的冲突：** `technical-doc19-seven-stage-full-green-product-pla-2026-09-09` **明确建议勿走字面全绿**（同业未打包交付；工期与维护不可控）。产品 Correct Course **强制选择 B1**，接受 **多年并行 / 高维护**（**NFR40**），并否决「以 research 排期默认」继续停在合同绿。本记录把该冲突写死，防止后续用「调研不支持」静默缩回假完成。

### (a) 上游约束

- **Correct Course 批准（2026-09-09）：** `sprint-change-proposal-2026-09-09-phase12-path-b.md` 批准 Phase 12 = Epic 40–47 · FR94–FR105 / NFR40–NFR43；**推翻 FR93**；不回滚 Phase 11 工程。
- **PRD addendum：** 「2026-09-09 Update — Phase 12 字面绿（Path B）」已落地；字面宣称仅可引用 FR94–105（**NFR42**）；**禁止**用 FR87 冒充字面 B。
- **仍待故事落地（本记录不开工）：** doc-19 §19.7–19.9 字面绿勾选（→ **40.3**）；ARCHITECTURE-SPINE **AD-5 / AD-25 / AD-27** 与 Deferred/永久非目标指针（→ **40.4** / **NFR41**）。Correct Course + addendum 已授权；正文差分仍属后续故事。
- **推翻 FR93 五条的范围（→ 对应交付 FR）：**
  1. 树内/自研 HLS 调度器 → **FR95**（+ **FR96** 闭包数据流变换）；须修订 **AD-25**
  2. FIRRTL→idiomatic / 可维护 Scala → **FR97**；须修订 **AD-27**
  3. 默认 TLM≡CA / 形式证明 → **FR100**（形式等价产品；**FR101** SystemC TLM-2.0 产品路径）；须修订 **AD-5**
  4. VIP 级全协议 IP → **FR98**
  5. 按键全设计 elaborate 的 netlist LSP → **FR99**
  - 字面绿扩展面另含 **FR102–FR105**（多视图属性全矩阵、一级 IP 双模型、富波形、覆盖率）
- **NFR40：** 多年并行、无捷径；不得用「短期 sprint 假绿」交差。
- **NFR41：** 推翻 FR93 / 冲突 AD 的实现 epic 须先修订脊柱对应 AD 再标 story ready。
- **NFR42：** 「产品做完 / 字面全绿」不得仅引用 FR87；须引用 FR94–105 关闭证据。
- **NFR43：** Epic 40+ 各实现 epic 开工前独立 NFR14（本记录为 Epic 40 门）。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **与 Phase 11：** FR87 合同绿保留为**历史已交付标签**；**不得**作为字面 Path B 完成口径。

### (b) 粗工期带

- **预计：** Epic 40 整体约 **1–3 人周**（40.1 本风险记录 ≤0.25 人周；40.2 Correct Course/PRD 验收对齐 0.25–0.75 人周；40.3 doc-19 字面绿重写 0.5–1 人周；40.4 AD/deferred/README 同步 0.5–1.25 人周）。**Phase 12 全盘（Epic 41–47）为多年带（NFR40）**，不计入本 epic 人周，但必须在计划中显式承认。
- **置信度 / 假设：** 中–低（对本 epic 文档/合同故事为中；对字面全绿总工期为低）。假设不回滚 FR1–FR92；假设 40.3–40.4 只改合同指针、不提前实现 FR95+。若提前开闸 41–47，维护叠加显著上修。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **40.2–40.4** 标 `ready` 或开工实现。
- **FR94 未合入 / Epic 40 未关闭前，不得将 Epic 41–47 标 `ready` 或开工实现。**
- **不得用 FR87 合同绿宣称字面全绿 / Path B 已完成**（NFR42）。
- **不得静默交付半成品 LSP / HLS（或 stub 外挂路径）冒充字面条（FR95/FR99 等）完成。**
- 不得在未修订 **AD-5 / AD-25 / AD-27**（及 Deferred 指针）时声称实现 FR95/97/101 已合法（NFR41；正文 → 40.4）。
- 不得把 research「建议勿走字面」歪曲为「已取消 Path B」而不经新 Correct Course。
- 不得回滚已交付 **FR46–FR92** 验收以「配合」字面绿叙事。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR40 / NFR41 / NFR42 / NFR43** 共同责任人
- 备份 / 升级路径：撤回 Path B / 改回合同绿唯一口径须升级至产品 / Correct Course 批准人；AD 修订争议升级至架构（AD-28）维护者。

---

### FR93 五条 → Phase 12 交付对照

| FR93 历史锁定项 | 推翻后交付 | 须同步 AD |
| --- | --- | --- |
| 树内 HLS 调度器 | FR95 / FR96（Epic 41） | AD-25 |
| FIRRTL→idiomatic Scala | FR97（Epic 42） | AD-27 |
| 默认 TLM≡CA 形式证明 | FR100（Epic 45）；FR101 TLM 产品（Epic 46） | AD-5 |
| VIP 级全协议 IP | FR98（Epic 43） | — |
| 按键全 elaborate netlist LSP | FR99（Epic 44） | — |

### 须同步修订的 AD（NFR41 · 本记录登记；正文 → 40.4）

| AD | 现状（Phase 11） | Path B 目标 |
| --- | --- | --- |
| **AD-5** | 不承诺 SystemC TLM-2.0 | 允许 FR101 产品路径 |
| **AD-25** | HLS 仅外挂 | 允许 FR95 树内调度为主路径之一 |
| **AD-27** | 机械可编译即可 | 增加 FR97 idiomatic 验收面 |
| Deferred / 永久非目标指针 | 「须新 PRD 才能推翻」 | 指向 Phase 12 FR；FR94 已满足新 PRD |

### Epic 40 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **40.1** | 本 NFR14 风险记录 + ATDD；钉死 B1 / FR93 范围 / 禁止项 | **本故事** |
| **40.2** | Correct Course + PRD/addendum 推翻 FR93 验收（合同戳） | Gate：须本记录后才可 ready |
| **40.3** | 重写 doc-19 阶段五–七为字面绿定义 | Gate：须本记录后才可 ready |
| **40.4** | 修订 AD + 撤销 FR93 永久非目标锁定；勾选关闭条件 | Gate：须本记录后才可 ready；关闭后才可开 41–47 |

### 并行 / 维护叠加（Chipyard 式 · NFR40）

- Epic 41–47 **可并行**，但均硬依赖 Epic 40 关闭；并行时回归面与语义 LCD 膨胀 — 须各 epic 自有 NFR14（NFR43）。
- 文档面叠加：addendum、doc-19、README、`deferred-work.md`、ARCHITECTURE-SPINE 须同一 Path B 叙事；禁止「合同绿话术」与「字面绿宣称」混用。
- 不得在缺 AD 修订时用半成品工具链冒充字面关闭。

### 引用

- AD-28 — 风险门禁（NFR14）
- PRD NFR14 / FR94；NFR40–NFR43；对照 FR87 / FR93 / NFR38
- Correct Course：`sprint-change-proposal-2026-09-09-phase12-path-b.md`（approved）
- Research：`technical-doc19-seven-stage-full-green-product-pla-2026-09-09/research.md`（建议勿走字面；B1 否决）
- Addendum：`prd-rhdl-2026-08-19/addendum.md` — Phase 12 字面绿
- 体例：`nfr14-risk-epic36-contract-green.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 40 关闭条件（Story 40.4 勾选）

- [ ] **FR94 / Correct Course + PRD：** Path B 批准与 FR93 推翻文案验收 — Story 40.2
- [ ] **doc-19：** §19.7–19.9 改为字面绿勾选条件 — Story 40.3
- [ ] **AD / deferred：** AD-5/25/27 + 永久非目标指针与 Path B 一致 — Story 40.4
- [ ] **NFR40–42：** 多年/高维护已登记；宣称纪律明确；AD 同步门禁可执行
- [ ] **禁止事项未触发：** 无 FR87 冒充字面绿；无未合 FR94 开 41–47；无半成品 LSP/HLS 冒充字面
- [ ] **品牌 / 依赖：** 仍为 Bitloom；设计 crate 只依赖 `bitloom-prelude`
- [ ] **Epic 41–47：** 本 epic 关闭前不得标 ready

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 40 故事 40.2–40.4 标 `ready`。**  
**B1 / Path B 已强制选定（与 research「勿走字面」冲突已记录）；FR94 未合入 / Epic 40 未关闭前不得开闸 Epic 41–47。**
