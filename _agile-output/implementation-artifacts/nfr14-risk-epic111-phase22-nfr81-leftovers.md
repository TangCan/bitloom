# NFR14 风险记录 — Epic 111 Phase 22 合同闸门（NFR81 leftovers 升格 / FR178）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 22 **NFR83–NFR87**；闸门 **FR178**；宣称须引 **FR178–184**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic105-phase21-nfr76-leftovers.md`。  
> **前置：** Phase 12–21 FR94–177 / Epic 40–110 **closed**；Correct Course `sprint-change-proposal-2026-09-12-phase22-nfr81-leftovers.md` **approved**（2026-09-12；`correctCoursePhase22Approved: 2026-09-12`；planning commit `b859bb7`）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 111 后续故事 **111.2–111.4** 标为 `ready`，亦不得开工实现。**Epic 111 未关闭 / FR178 未验收前，Epic 112–117 不得标 `ready`。**

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR178 / Epic 111；NFR14、NFR83、NFR84、NFR86、NFR87；对照 FR172–177 / NFR78–82；FR179–184 实现面 |
| 记录日期 | 2026-09-14 |
| 状态 | **open — Story 111.1**（in-progress）；Epic 111 闸门尚未关闭 |
| **选定** | 在保留 Phase 12–21 关闭面的前提下，授权 Phase 22「NFR81 leftovers 升格」合同闸门（FR178）；实现属 Epic 112–117 |

### Phase 12–21 关闭面 vs Phase 22 边界（NFR83 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 12–20** | FR94–171 / Epic 40–104 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 21 NFR76 leftovers** | FR172–177 / Epic 105–110 | **仍有效；不得改写「已关闭」**（NFR83） |
| **Phase 22 NFR81 leftovers** | FR178–184 / Epic 111–117；NFR83–NFR87 | **本合同**（Correct Course 已批准 2026-09-12；planning-complete 2026-09-14） |

**选定：在保留 Phase 12–21 关闭面的前提下，授权 NFR81 leftovers 升格。**  
本记录禁止把本批叙述偷换成「Phase 21 失败后的补救」或「NFR76 alone ⇒ NFR81 leftover 已授权」。  
**Phase 22 口径** = FR178–184 对应关闭 + 诚实列出超出各 NFR14 钉死子集的仍须新合同项（**NFR86**）；**不等于**静默扩大 FR142；**`git push` 不是 FR**。

### 批准默认（Q1–Q7 · Correct Course）

| # | 决议 |
| --- | --- |
| **Q1** | NFR81 **五条全部**升格为 **FR179–FR183**（各 epic NFR14 钉死子集；禁止静默超子集 — NFR86） |
| **Q2** | 各加深条独立 NFR14；浮动 HEAD / Handshake / Style / unpaired firtool / FR142 扩展分 epic |
| **Q3** | **不得**改写 Phase 12–21「已关闭」（**NFR83**） |
| **Q4** | 宣称须引已关 FR（**FR184** / **NFR87**）；**禁止**用 Phase 21 alone 冒充本批五条 |
| **Q5** | **「扩 FR142」= 独立 FR183**（显式公开 API 表面扩展 + SemVer/docs；不是静默扩大） |
| **Q6** | **MSRV** 默认保持现行；超出本批钉死子集仍须新合同（**NFR86**） |
| **Q7** | **FR182** — 无上游配对 firtool **产品钉**再升钉须修订 **AD-9** 明示 *unpaired product-pin* 例外（≠ FR173；≠ FR174 alone） |

### (a) 上游约束

- **Correct Course 批准（2026-09-12）：** `sprint-change-proposal-2026-09-12-phase22-nfr81-leftovers.md` 批准 Phase 22 = Epic 111–117 · FR178–FR184 / NFR83–NFR87；不回滚 Phase 12–21。
- **PRD addendum：** 「Phase 22」合同戳待 **Story 111.2** 验收；「五条已交付」类宣称仅可引用对应 FR 关闭证据；**禁止**用 Phase 21 alone 冒充。
- **合同落地：** README / deferred 诚实面 — **Story 111.3**；ARCHITECTURE-SPINE / AGENTS 指针 — **Story 111.4**。
- **FR179–184 范围摘要：**
  1. **FR179** — 浮动 CIRCT git HEAD（超 FR174）；Epic 112
  2. **FR180** — Handshake dialect 加深（超 FR129/FR175）；Epic 113
  3. **FR181** — 更深 Style Guide / linter（超 FR176）；Epic 114
  4. **FR182** — unpaired firtool 产品钉再升钉（AD-9 例外）；Epic 115
  5. **FR183** — 显式扩大 FR142 公开 API 表面；Epic 116
  6. **FR184** — Phase 22 宣称诚实门；Epic 117
- **NFR83：** Phase 22 关闭不得改写 Phase 12–21 FR94–177「已关闭」。
- **NFR84：** Phase 22 规划故事齐（Epic 111–117）后方可宣称规划 complete（已于 2026-09-14 create-epics 完成）。
- **NFR85：** 触及 AD-9 / AD-25 / AD-27 / FR142 表面须先修订再 story ready。
- **NFR86：** 禁止静默扩大超出各 epic NFR14 钉死子集；未写入 FR179–183 的新加深须另开合同。
- **NFR87：** 宣称仅经 FR184；不得用 FR172–177 或 Phase 21 alone 冒充本批。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **软序（非硬依赖）：** `111 →（112 ‖ 113 ‖ 114 ‖ 115 ‖ 116）→ 117`；Epic 112/115 触及 AD-9 建议串行。

### (b) 粗工期带

- **预计：** Epic 111 整体约 **0.75–2 人周**（111.1 本风险记录 ≤0.25 人周；111.2 Correct Course/PRD 验收 0.25–0.5；111.3 README/deferred 0.25–0.75；111.4 AD 指针收口 0.25–0.5）。**Phase 22 全盘（Epic 112–117）为 High effort**（浮动 HEAD、Handshake、Style、unpaired 产品钉、API 表面），不计入本 epic 人周，但必须在计划中显式承认。
- **置信度 / 假设：** 中（对本 epic 文档/合同故事为中–高；对 FR179–183 外部工具链与 SemVer 为中–低）。假设不回滚 FR94–177；假设 111.3–111.4 只改合同指针与诚实文档、不提前实现加深。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **111.2–111.4** 标 `ready` 或开工实现。
- **FR178 未验收 / Epic 111 未关闭前，不得将 Epic 112–117 标 `ready` 或开工实现。**
- **不得改写 Phase 12–21 FR94–177「已关闭」为失败**（NFR83）。
- **不得用 Phase 21 alone 冒充 NFR81 五条合同已授权。**
- **不得静默扩大 FR142** 表面承诺（扩表面须走 **FR183**）。
- **不得把未写入 FR179–183 的新加深冒充已交付**（NFR86）。
- **不得把 `git push` / 远程同步当成产品 FR。**
- 不得静默 publish `rhdl` / `rhdl-bits`（AD-2 / 继承）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR83 / NFR84 / NFR86 / NFR87** 共同责任人
- 备份 / 升级路径：缩回「只做子集 / 不做五条全做」须升级至产品 / Correct Course 批准人；AD-9/AD-25/AD-27 / FR142 争议升级至架构（AD-28）维护者。

---

### FR179–184 → Epic 对照

| FR | Epic | 相对 Phase 21 / 前序 |
| --- | --- | --- |
| FR179 | 112 | 超 FR174（浮动 CIRCT git HEAD） |
| FR180 | 113 | 超 FR129/FR175（Handshake dialect） |
| FR181 | 114 | 超 FR176（更深 Style/linter） |
| FR182 | 115 | ≠ FR173/FR174（unpaired 产品钉 + AD-9 例外） |
| FR183 | 116 | 显式 FR142 表面扩展 |
| FR184 | 117 | 宣称诚实门 |

### Epic 111 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **111.1** | 本 NFR14 风险记录 + ATDD；钉死 NFR83 边界 / FR179–184 / Q1–Q7 / 禁止项 | **本故事** |
| **111.2** | Correct Course + PRD/addendum Phase 22 验收（合同戳） | Gate：须本记录后才可 ready |
| **111.3** | 同步 README / deferred / 路线图指针（FR178） | Gate：须本记录后才可 ready |
| **111.4** | AD 指针与 Epic 111 收口；勾选关闭条件 | Gate：须本记录后才可 ready；关闭后才可开 112–117 |

### 并行 / 维护叠加（Chipyard 式 · NFR14）

- Epic 112–116 **可并行规划**但硬依赖 Epic 111 关闭；软序 `111 →（112 ‖ 113 ‖ 114 ‖ 115 ‖ 116）→ 117`。
- 文档面叠加：addendum、README、`deferred-work.md`、ARCHITECTURE-SPINE 须同一「Phase 21 vs Phase 22」叙事；禁止混用。
- 加深风险：Epic 112/115 AD-9；Epic 113 AD-25；Epic 114 AD-27；Epic 116 FR142；须各自 NFR14。

### 引用

- AD-28 — 风险门禁（NFR14）；AD-2 — 对外 `bitloom-*`；AD-6 — `bitloom-prelude`
- PRD NFR14 / FR178；NFR83–NFR87；对照 FR172–177 / NFR78–82
- Correct Course：`sprint-change-proposal-2026-09-12-phase22-nfr81-leftovers.md`（approved 2026-09-12）
- Addendum：`prd-rhdl-2026-08-19/addendum.md` — Phase 22（→ 111.2）
- 体例：`nfr14-risk-epic105-phase21-nfr76-leftovers.md`
- **NFR14-crates** ≠ 本门禁

---

### Epic 111 关闭条件（Story 111.4 勾选）

- [ ] **FR178 / Correct Course + PRD：** Phase 22 批准文案验收 — Story 111.2
- [ ] **README / deferred：** Phase 21 vs Phase 22 合同区分 — Story 111.3
- [ ] **AD 指针：** ARCHITECTURE-SPINE / AGENTS Phase 22 指针 — Story 111.4
- [ ] **NFR83–87：** 边界与诚实义务写入本记录并保持
- [ ] **禁止事项未触发：** 112–117 在 Epic 111 关闭前未标 ready
- [ ] **品牌 / 依赖：** Bitloom / `bitloom-prelude`；禁 publish `rhdl`/`rhdl-bits`
- [ ] **Epic 112–117：** 仍须各自 NFR14；未实现前不得宣称对应 FR 关闭
