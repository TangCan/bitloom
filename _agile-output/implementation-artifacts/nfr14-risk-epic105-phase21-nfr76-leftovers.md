# NFR14 风险记录 — Epic 105 Phase 21 合同闸门（NFR76 leftovers 升格 / FR172）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 21 **NFR78–NFR82**；闸门 **FR172**；宣称须引 **FR172–177**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic99-phase20-nfr71-four-leftovers.md`。  
> **前置：** Phase 12–20 FR94–171 / Epic 40–104 **closed**；Correct Course `sprint-change-proposal-2026-09-12-phase21-nfr76-leftovers.md` **approved**（2026-09-12；`correctCoursePhase21Approved: 2026-09-12`；commit `f5d527a`）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 105 后续故事 **105.2–105.4** 标为 `ready`，亦不得开工实现。**Epic 105 未关闭 / FR172 未验收前，Epic 106–110 不得标 `ready`。**

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR172 / Epic 105；NFR14、NFR78、NFR79、NFR81、NFR82；对照 FR166–171 / NFR73–77；FR173–177 实现面 |
| 记录日期 | 2026-09-12 |
| 状态 | open — Story 105.1；Epic 105 闸门进行中；105.2–105.4 待本记录后 ready；Epic 106–110 在 Epic 105 关闭前不得 ready |
| **选定** | 在保留 Phase 12–20 关闭面的前提下，授权 Phase 21「NFR76 leftovers 升格」合同闸门（FR172）；实现属 Epic 106–110 |

### Phase 12–20 关闭面 vs Phase 21 边界（NFR78 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 12–19** | FR94–165 / Epic 40–98 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 20 NFR71 四条** | FR166–171 / Epic 99–104 | **仍有效；不得改写「已关闭」**（NFR78） |
| **Phase 21 NFR76 leftovers** | FR172–177 / Epic 105–110；NFR78–NFR82 | **本合同**（Correct Course 已批准 2026-09-12） |

**选定：在保留 Phase 12–20 关闭面的前提下，授权 NFR76 leftovers 升格。**  
本记录禁止把本批叙述偷换成「Phase 20 失败后的补救」或「NFR71 alone ⇒ NFR76 leftover 已授权」。  
**Phase 21 口径** = FR172–177 对应关闭 + 诚实列出超出各 NFR14 钉死子集的仍须新合同项（**NFR81**）；**不等于**静默扩大 FR142；**`git push` 不是 FR**。

### 批准默认（Q1–Q5 · Correct Course）

| # | 决议 |
| --- | --- |
| **Q1** | NFR76 **四条全部**升格为 **FR173–FR176**（各 epic NFR14 钉死子集；禁止静默超子集 — NFR81） |
| **Q2** | **FR173** — firtool 升钉须上游 Chisel **正式配对**后修订 **AD-9 / Stack**（禁止 unpaired bump） |
| **Q3** | **不得**改写 Phase 12–20「已关闭」（**NFR78**） |
| **Q4** | **不得**静默扩大 FR142；宣称须引已关 FR（**FR177** / **NFR82**） |
| **Q5** | **MSRV** 默认保持现行；超出本批钉死子集仍须新合同（**NFR81**）；**FR172** 须 Correct Course + PRD 戳后方可开实现 epic |

### (a) 上游约束

- **Correct Course 批准（2026-09-12）：** `sprint-change-proposal-2026-09-12-phase21-nfr76-leftovers.md` 批准 Phase 21 = Epic 105–110 · FR172–FR177 / NFR78–NFR82；不回滚 Phase 12–20。
- **PRD addendum：** 「Phase 21」合同戳待 **Story 105.2** 验收；「四条已交付」类宣称仅可引用对应 FR 关闭证据；**禁止**用 Phase 20 alone 冒充。
- **合同落地：** README / deferred 诚实面 — **Story 105.3**；ARCHITECTURE-SPINE / AGENTS 指针 — **Story 105.4**。
- **FR173–177 范围摘要：**
  1. **FR173** — firtool 升钉超 AD-9（配对 + AD-9 修订）；Epic 106
  2. **FR174** — unpaired CIRCT/Chisel HEAD 产品路径；Epic 107
  3. **FR175** — 更广 CIRCT/MLIR/sim（超 FR169 NFR14）；Epic 108
  4. **FR176** — 更深 Parser/Chisel 生态（超 FR170 NFR14）；Epic 109
  5. **FR177** — Phase 21 宣称诚实门；Epic 110
- **NFR78：** Phase 21 关闭不得改写 Phase 12–20 FR94–171「已关闭」。
- **NFR79：** Phase 21 各实现 epic 开工前独立 NFR14（本记录为 Epic 105 门）。
- **NFR80：** 触及 AD-9 / AD-27 / firtool·HEAD 时须先修订脊柱/文档/CI 再 story ready。
- **NFR81：** 禁止静默扩大超出各 epic NFR14 钉死子集；未写入 FR173–176 的新加深须另开合同。
- **NFR82：** 宣称仅经 FR177；不得用 FR169/170 或 Phase 20 alone 冒充本批。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **软序（非硬依赖）：** `105 →（106 ‖ 107 ‖ 108 ‖ 109）→ 110`；Epic 106/107 触及 AD-9 建议串行。

### (b) 粗工期带

- **预计：** Epic 105 整体约 **0.75–2 人周**（105.1 本风险记录 ≤0.25 人周；105.2 Correct Course/PRD 验收 0.25–0.5；105.3 README/deferred 0.25–0.75；105.4 AD 指针收口 0.25–0.5）。**Phase 21 全盘（Epic 106–110）为 High effort**（firtool 升钉、HEAD、更广 CIRCT、更深 Parser），不计入本 epic 人周，但必须在计划中显式承认。
- **置信度 / 假设：** 中（对本 epic 文档/合同故事为中–高；对 FR173–176 外部 firtool/Chisel HEAD 为中–低）。假设不回滚 FR94–171；假设 105.3–105.4 只改合同指针与诚实文档、不提前实现加深。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **105.2–105.4** 标 `ready` 或开工实现。
- **FR172 未验收 / Epic 105 未关闭前，不得将 Epic 106–110 标 `ready` 或开工实现。**
- **不得改写 Phase 12–20 FR94–171「已关闭」为失败**（NFR78）。
- **不得用 Phase 20 alone 冒充 NFR76 四条合同已授权。**
- **不得静默扩大 FR142** 表面承诺。
- **不得把未写入 FR173–176 的新加深冒充已交付**（NFR81）。
- **不得把 `git push` / 远程同步当成产品 FR。**
- 不得静默 publish `rhdl` / `rhdl-bits`（AD-2 / 继承）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR78 / NFR79 / NFR81 / NFR82** 共同责任人
- 备份 / 升级路径：缩回「只做子集 / 不做四条全做」须升级至产品 / Correct Course 批准人；AD-9/AD-27 争议升级至架构（AD-28）维护者。

---

### FR173–177 → Epic 对照

| FR | Epic | 相对 Phase 20 |
| --- | --- | --- |
| FR173 | 106 | 超 FR169(A)（firtool 升钉配对 AD-9） |
| FR174 | 107 | 超 FR170（unpaired HEAD） |
| FR175 | 108 | 超 FR169 NFR14（更广 CIRCT/sim） |
| FR176 | 109 | 超 FR170 NFR14（更深 Parser 生态） |
| FR177 | 110 | 宣称诚实门 |

### Epic 105 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **105.1** | 本 NFR14 风险记录 + ATDD；钉死 NFR78 边界 / FR173–177 / Q1–Q5 / 禁止项 | **本故事** |
| **105.2** | Correct Course + PRD/addendum Phase 21 验收（合同戳） | Gate：须本记录后才可 ready |
| **105.3** | 同步 README / deferred / 路线图指针（FR172） | Gate：须本记录后才可 ready |
| **105.4** | AD 指针与 Epic 105 收口；勾选关闭条件 | Gate：须本记录后才可 ready；关闭后才可开 106–110 |

### 并行 / 维护叠加（Chipyard 式 · NFR79）

- Epic 106–109 **可并行规划**但硬依赖 Epic 105 关闭；软序 `105 →（106 ‖ 107 ‖ 108 ‖ 109）→ 110`。
- 文档面叠加：addendum、README、`deferred-work.md`、ARCHITECTURE-SPINE 须同一「Phase 20 vs Phase 21」叙事；禁止混用。
- 加深风险：Epic 106/107 AD-9；Epic 108 CIRCT；Epic 109 Parser；须各自 NFR14。

### 引用

- AD-28 — 风险门禁（NFR14）；AD-2 — 对外 `bitloom-*`；AD-6 — `bitloom-prelude`
- PRD NFR14 / FR172；NFR78–NFR82；对照 FR166–171 / NFR73–77
- Correct Course：`sprint-change-proposal-2026-09-12-phase21-nfr76-leftovers.md`（approved 2026-09-12）
- Addendum：`prd-rhdl-2026-08-19/addendum.md` — Phase 21（→ 105.2）
- 体例：`nfr14-risk-epic99-phase20-nfr71-four-leftovers.md`
- **NFR14-crates** ≠ 本门禁

---

### Epic 105 关闭条件（Story 105.4 勾选）

- [ ] **FR172 / Correct Course + PRD：** Phase 21 批准文案验收 — Story 105.2
- [ ] **README / deferred：** Phase 20 vs Phase 21 合同区分 — Story 105.3
- [ ] **AD 指针：** ARCHITECTURE-SPINE / AGENTS Phase 21 指针 — Story 105.4
- [ ] **NFR78–82：** 边界与诚实义务写入本记录并保持
- [ ] **禁止事项未触发：** 106–110 在 Epic 105 关闭前未标 ready
- [ ] **品牌 / 依赖：** Bitloom / `bitloom-prelude`；禁 publish `rhdl`/`rhdl-bits`
- [ ] **Epic 106–110：** 仍须各自 NFR14；未实现前不得宣称对应 FR 关闭
