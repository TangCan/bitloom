# NFR14 风险记录 — Epic 118 Phase 23 合同闸门（NFR86 leftovers 升格 / FR185）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 23 **NFR88–NFR92**；闸门 **FR185**；宣称须引 **FR185–191**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic111-phase22-nfr81-leftovers.md`。  
> **前置：** Phase 12–22 FR94–184 / Epic 40–117 **closed**；工程/合同结项 `engineeringCloseoutStatus: complete`；Correct Course `sprint-change-proposal-2026-09-14-phase23-nfr86-leftovers.md` **approved**（2026-09-14；`correctCoursePhase23Approved: 2026-09-14`；planning commit `bdc03a4`）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 118 后续故事 **118.2–118.4** 标为 `ready`，亦不得开工实现。**Epic 118 未关闭 / FR185 未验收前，Epic 119–124 不得标 `ready`。**

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR185 / Epic 118；NFR14、NFR88、NFR89、NFR91、NFR92；对照 FR178–184 / NFR83–87；FR186–191 实现面 |
| 记录日期 | 2026-09-14 |
| 状态 | **open / in-progress**（Story 118.1；Epic 118 闸门未关；关闭勾选 → Story 118.4） |
| **选定** | 在保留 Phase 12–22 + 工程结项关闭面的前提下，授权 Phase 23「NFR86 leftovers 升格」合同闸门（FR185）；实现属 Epic 119–124 |

### Phase 12–22 / 结项关闭面 vs Phase 23 边界（NFR88 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 12–21** | FR94–177 / Epic 40–110 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 22 NFR81 leftovers** | FR178–184 / Epic 111–117 | **仍有效；不得改写「已关闭」**（NFR88） |
| **Engineering closeout** | `engineeringCloseoutApproved` / `complete` | **仍有效**；alone ≠ FR186–190 已交付 |
| **Phase 23 NFR86 leftovers** | FR185–191 / Epic 118–124；NFR88–NFR92 | **本合同**（Correct Course 已批准 2026-09-14；planning-complete 2026-09-14） |

**选定：在保留 Phase 12–22 / 结项关闭面的前提下，授权 NFR86 leftovers 升格。**  
本记录禁止把本批叙述偷换成「Phase 22 / 结项失败后的补救」或「结项 alone ⇒ NFR86 leftover 已交付」。  
**Phase 23 口径** = FR185–191 对应关闭 + 诚实列出超出各 NFR14 钉死子集的仍须新合同项（**NFR91**）；**不等于**静默扩大 FR142；**`git push` 不是 FR**。

### 批准默认（Q1–Q8 · Correct Course）

| # | 决议 |
| --- | --- |
| **Q1** | NFR86 **五条全部**升格为 **FR186–FR190**（各 epic NFR14 钉死子集；禁止静默超子集 — NFR91） |
| **Q2** | 各加深条独立 NFR14；无界 tip / Handshake lower / Style 全家桶 / firtool 再升钉 / FR142 再扩展分 epic |
| **Q3** | **不得**改写 Phase 12–22「已关闭」/ 工程结项（**NFR88**） |
| **Q4** | 宣称须引已关 FR（**FR191** / **NFR92**）；**禁止**用 Phase 22 / 结项 alone 冒充本批五条；**禁止**宣称「NFR86 账本已空」超本批 |
| **Q5** | **「继续扩 FR142」= 独立 FR190**（显式公开 API 表面扩展 + SemVer/docs；不是静默扩大） |
| **Q6** | **MSRV** 默认保持现行；超出本批钉死子集仍须新合同（**NFR91**） |
| **Q7** | 触及 **AD-9 / AD-25 / AD-27 / FR142** 须按 **NFR90** 先修订再 story ready |
| **Q8** | **`git push` / 远程同步不在本合同内**（不是 FR） |

### (a) 上游约束

- **Correct Course 批准（2026-09-14）：** `sprint-change-proposal-2026-09-14-phase23-nfr86-leftovers.md` 批准 Phase 23 = Epic 118–124 · FR185–FR191 / NFR88–NFR92；不回滚 Phase 12–22 / 结项。
- **PRD addendum：** 「Phase 23」合同戳待 **Story 118.2** 验收；「五条已交付」类宣称仅可引用对应 FR 关闭证据；**禁止**用 Phase 22 / 结项 alone 冒充。
- **合同落地：** README / deferred 诚实面 — **Story 118.3**；ARCHITECTURE-SPINE / AGENTS 指针 — **Story 118.4**。
- **FR186–191 范围摘要：**
  1. **FR186** — 无界 CIRCT tip（超 FR179）；Epic 119
  2. **FR187** — Handshake lower/dialect 加深（超 FR180）；Epic 120
  3. **FR188** — 社区 Style Guide 全家桶（超 FR181）；Epic 121
  4. **FR189** — 继续 firtool 产品钉升钉（超 FR182；须修订 AD-9）；Epic 122
  5. **FR190** — 继续显式扩 FR142（超 FR183）；Epic 123
  6. **FR191** — Phase 23 宣称诚实门；Epic 124
- **NFR88：** Phase 23 关闭不得改写 Phase 12–22 FR94–184 / 结项「已关闭」。
- **NFR89：** Phase 23 规划故事齐（Epic 118–124）后方可宣称规划 complete（已于 2026-09-14 create-epics 完成）。
- **NFR90：** 触及 AD-9 / AD-25 / AD-27 / FR142 表面须先修订再 story ready。
- **NFR91：** 禁止静默扩大超出各 epic NFR14 钉死子集；未写入 FR186–190 的新加深须另开合同。
- **NFR92：** 宣称仅经 FR191；不得用 FR178–184 或 Phase 22 / 结项 alone 冒充本批。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **软序（非硬依赖）：** `118 →（119 ‖ 120 ‖ 121 ‖ 122 ‖ 123）→ 124`；Epic 119/122 触及 AD-9 建议串行。

### (b) 粗工期带

- **预计：** Epic 118 整体约 **0.75–2 人周**（118.1 本风险记录 ≤0.25 人周；118.2 Correct Course/PRD 验收 0.25–0.5；118.3 README/deferred 0.25–0.75；118.4 AD 指针收口 0.25–0.5）。**Phase 23 全盘（Epic 119–124）为 High effort**（无界 tip、Handshake lower、Style 全家桶、firtool 再升钉、API 再扩展），不计入本 epic 人周，但必须在计划中显式承认。
- **置信度 / 假设：** 中（对本 epic 文档/合同故事为中–高；对 FR186–190 外部工具链与 SemVer 为中–低）。假设不回滚 FR94–184 / 结项；假设 118.3–118.4 只改合同指针与诚实文档、不提前实现加深。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **118.2–118.4** 标 `ready` 或开工实现。
- **FR185 未验收 / Epic 118 未关闭前，不得将 Epic 119–124 标 `ready` 或开工实现。**
- **不得改写 Phase 12–22 FR94–184「已关闭」/ 工程结项为失败**（NFR88）。
- **不得用 Phase 22 / 结项 alone 冒充 NFR86 五条合同已授权或已交付。**
- **不得静默扩大 FR142** 表面承诺（继续扩表面须走 **FR190**）。
- **不得把未写入 FR186–190 的新加深冒充已交付**（NFR91）。
- **不得宣称「NFR86 账本已空」** 若超出本批各 NFR14（→ NFR91）。
- **不得把 `git push` / 远程同步当成产品 FR。**
- 不得静默 publish `rhdl` / `rhdl-bits`（AD-2 / 继承）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR88 / NFR89 / NFR91 / NFR92** 共同责任人
- 备份 / 升级路径：缩回「只做子集 / 不做五条全做」须升级至产品 / Correct Course 批准人；AD-9/AD-25/AD-27 / FR142 争议升级至架构（AD-28）维护者。

---

### FR186–191 → Epic 对照

| FR | Epic | 相对 Phase 22 / 前序 |
| --- | --- | --- |
| FR186 | 119 | 超 FR179（无界 CIRCT tip） |
| FR187 | 120 | 超 FR180（Handshake lower/dialect） |
| FR188 | 121 | 超 FR181（Style Guide 全家桶） |
| FR189 | 122 | 超 FR182（继续 firtool 产品钉 + AD-9） |
| FR190 | 123 | 超 FR183（继续显式 FR142 表面扩展） |
| FR191 | 124 | 宣称诚实门 |

### Epic 118 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **118.1** | 本 NFR14 风险记录 + ATDD；钉死 NFR88 边界 / FR186–191 / Q1–Q8 / 禁止项 | **本故事** |
| **118.2** | Correct Course + PRD/addendum Phase 23 验收（合同戳） | Gate：须本记录后才可 ready |
| **118.3** | 同步 README / deferred / 路线图指针（FR185） | Gate：须本记录后才可 ready |
| **118.4** | AD 指针与 Epic 118 收口；勾选关闭条件 | Gate：须本记录后才可 ready；关闭后才可开 119–124 |

### 并行 / 维护叠加（Chipyard 式 · NFR14）

- Epic 119–123 **可并行规划**但硬依赖 Epic 118 关闭；软序 `118 →（119 ‖ 120 ‖ 121 ‖ 122 ‖ 123）→ 124`。
- 文档面叠加：addendum、README、`deferred-work.md`、ARCHITECTURE-SPINE 须同一「结项 / Phase 22 vs Phase 23」叙事；禁止混用。
- 加深风险：Epic 119/122 AD-9；Epic 120 AD-25；Epic 121 AD-27；Epic 123 FR142；须各自 NFR14。

### 引用

- AD-28 — 风险门禁（NFR14）；AD-2 — 对外 `bitloom-*`；AD-6 — `bitloom-prelude`
- PRD NFR14 / FR185；NFR88–NFR92；对照 FR178–184 / NFR83–87
- Correct Course：`sprint-change-proposal-2026-09-14-phase23-nfr86-leftovers.md`（approved 2026-09-14）
- Addendum：`prd-rhdl-2026-08-19/addendum.md` — Phase 23（→ 118.2）
- 体例：`nfr14-risk-epic111-phase22-nfr81-leftovers.md`
- **NFR14-crates** ≠ 本门禁

---

### Epic 118 关闭条件（Story 118.4 勾选）

- [ ] **FR185 / Correct Course + PRD：** Phase 23 批准文案验收 — Story 118.2
- [ ] **README / deferred：** 结项 / Phase 22 vs Phase 23 合同区分 — Story 118.3
- [ ] **AD 指针：** ARCHITECTURE-SPINE / AGENTS Phase 23 指针 — Story 118.4
- [ ] **NFR88–92：** 边界与诚实义务写入本记录并保持
- [ ] **禁止事项未触发：** 119–124 在 Epic 118 关闭前未标 ready
- [ ] **品牌 / 依赖：** Bitloom / `bitloom-prelude`；禁 publish `rhdl`/`rhdl-bits`
- [ ] **Epic 119–124：** 仍须各自 NFR14；未实现前不得宣称对应 FR 关闭
