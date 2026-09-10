# NFR14 风险记录 — Epic 48 Phase 13 加深合同闸门（MVP→商业加深 / FR106）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 13 **NFR44–NFR47**；闸门 **FR106**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic40-literal-path-b.md`。  
> **前置：** Phase 12（Epic 40–47）字面绿 MVP complete；Correct Course `sprint-change-proposal-2026-09-10-phase13-mvp-commercial-deepen.md` **approved**（2026-09-10）；PRD/addendum Phase 13 段落已落地（commit `501f3be`）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 48 后续故事 **48.2–48.4** 标为 `ready`，亦不得开工实现。**Epic 48 未关闭 / FR106 未验收前，Epic 49–56 不得标 `ready`。**

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR106 / Epic 48；NFR14、NFR44、NFR45、NFR46、NFR47；对照 FR94–105 / NFR40–43 |
| 记录日期 | 2026-09-10 |
| 状态 | draft — Story 48.1；Epic 48 未关闭；Epic 49–56 仍 backlog |

### Phase 12 MVP vs Phase 13 加深边界（NFR44 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 12 字面绿 MVP** | FR94–105 / Epic 40–47 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 13 MVP→商业加深** | FR106–115 / Epic 48–56；optional→显式 FR | **本合同**（Correct Course 已批准） |

**选定：在保留 Phase 12 MVP 关闭面的前提下，授权商业/可选加深。**  
本记录禁止把加深叙述偷换成「Phase 12 AC 未达标后的补救」。

### (a) 上游约束

- **Correct Course 批准（2026-09-10）：** `sprint-change-proposal-2026-09-10-phase13-mvp-commercial-deepen.md` 批准 Phase 13 = Epic 48–56 · FR106–FR115 / NFR44–NFR47；不回滚 Phase 12。
- **PRD addendum：** 「2026-09-10 Update — Phase 13 MVP→商业加深」已落地；「商业加深 / 非 MVP」宣称仅可引用 FR106–114（**FR115**）；**禁止**用 Phase 12 MVP 冒充商业完整面。
- **仍待故事落地（本记录不开工）：** README / `deferred-work.md` 诚实面同步（→ **48.3**）；ARCHITECTURE-SPINE Deferred / AD 指针（→ **48.4** / **NFR46**）。Correct Course + addendum 已授权合同；正文差分由 48.3–48.4 落地。
- **FR107–114 加深范围摘要：**
  1. **FR107** — SystemC TLM-2.0 AT / `nb_transport`（超出 FR101 LT-only）；须引用/修订 **AD-5**
  2. **FR108** — GPIO 一级近 VIP（FR98 G0 可选升格）
  3. **FR109** — FSM / state-visit 覆盖率（FR105 C3 cropped 升格）
  4. **FR110** — 树内 HLS 商业深度（超出 FR95/96 MVP stub）；须引用/修订 **AD-25**
  5. **FR111** — Idiomatic Chisel 可维护深度（超出 FR97 MVP）；须引用/修订 **AD-27**
  6. **FR112** — 形式等价 / 双模型深度（超出 FR100 F1-(i) / FR103 SyncFifo MVP）
  7. **FR113** — LSP 设计根发现加深（超出 FR99 DesignFixture）
  8. **FR114** — Tywaves / LCOV GUI 加深（超出 FR104 interactive.html / FR105 Mux v2）
- **NFR44：** Phase 13 关闭不得改写 Phase 12 FR94–105「已关闭」。
- **NFR45：** Epic 48+ 各实现 epic 开工前独立 NFR14（本记录为 Epic 48 门）。
- **NFR46：** 触及 AD-5/25/27 时须先修订脊柱再标实现 story ready。
- **NFR47：** 升格后的 FR 禁止静默扩大超出各 epic 风险记录钉死的子集。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **与 Phase 12：** FR94–105 MVP 关闭证据保留；**不得**作为商业加深完成口径。

### (b) 粗工期带

- **预计：** Epic 48 整体约 **0.75–2 人周**（48.1 本风险记录 ≤0.25 人周；48.2 Correct Course/PRD 验收对齐 0.25–0.5 人周；48.3 README/deferred 同步 0.25–0.75 人周；48.4 AD 指针收口 0.25–0.5 人周）。**Phase 13 全盘（Epic 49–56）为多年/多 epic 加深带**，不计入本 epic 人周，但必须在计划中显式承认。
- **置信度 / 假设：** 中（对本 epic 文档/合同故事为中–高；对加深总工期为低）。假设不回滚 FR94–105；假设 48.3–48.4 只改合同指针与诚实文档、不提前实现 FR107+。若提前开闸 49–56，维护叠加显著上修。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **48.2–48.4** 标 `ready` 或开工实现。
- **FR106 未验收 / Epic 48 未关闭前，不得将 Epic 49–56 标 `ready` 或开工实现。**
- **不得改写 Phase 12 FR94–105「已关闭」为失败**（NFR44）。
- **不得用 Phase 12 MVP 冒充商业加深 / 非 MVP 完成面**（FR115）。
- **不得静默扩大超出本记录 / 各实现 epic NFR14 钉死的加深子集**（NFR47）。
- 不得在未引用/修订适用 **AD-5 / AD-25 / AD-27** 时声称 FR107/110/111 已合法关闭（NFR46；指针 → 48.4；实质修订 → 49/52/53）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR44 / NFR45 / NFR46 / NFR47** 共同责任人
- 备份 / 升级路径：撤回 Phase 13 / 缩回仅 Phase 12 MVP 口径须升级至产品 / Correct Course 批准人；AD 修订争议升级至架构（AD-28）维护者。

---

### FR107–114 → Epic 对照

| FR | Epic | 加深相对 Phase 12 MVP |
| --- | --- | --- |
| FR107 | 49 | AT / nb_transport vs FR101 LT-only |
| FR108 | 50 | GPIO vs FR98 四类近 VIP（G0 可选） |
| FR109 | 51 | C3 FSM vs FR105 Mux v2 |
| FR110 | 52 | HLS 商业深度 vs FR95/96 stub |
| FR111 | 53 | idiomatic 深度 vs FR97 MVP |
| FR112 | 54 | formal/dual-model 深度 vs FR100/103 MVP |
| FR113 | 55 | Cargo-graph 根 vs FR99 DesignFixture |
| FR114 | 56 | Tywaves/LCOV vs FR104/105 MVP |

### 须同步的 AD（NFR46 · 本记录登记）

| AD | Phase 12 现状 | Phase 13 加深目标 |
| --- | --- | --- |
| **AD-5** | 允许 FR101 LT-only TLM | 允许 **FR107** AT / nb_transport（Epic 49 修订） |
| **AD-25** | 允许 FR95 树内 MVP | 允许 **FR110** 商业调度深度（Epic 52 修订） |
| **AD-27** | 允许 FR97 idiomatic MVP | 允许 **FR111** 可维护加严（Epic 53 修订） |
| Deferred 指针 | 现行完成标签 = Phase 12 MVP | 区分 Phase 12 MVP vs Phase 13 加深（→ **48.4**） |

### Epic 48 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **48.1** | 本 NFR14 风险记录 + ATDD；钉死 NFR44 边界 / FR107–114 摘要 / 禁止项 | **本故事** |
| **48.2** | Correct Course + PRD/addendum Phase 13 验收（合同戳） | Gate：须本记录后才可 ready |
| **48.3** | 同步 README / deferred / 路线图指针（FR106 / FR115） | Gate：须本记录后才可 ready |
| **48.4** | AD 指针与 Epic 48 收口；勾选关闭条件 | Gate：须本记录后才可 ready；关闭后才可开 49–56 |

### 并行 / 维护叠加（Chipyard 式 · NFR45）

- Epic 49–56 **可并行**，但均硬依赖 Epic 48 关闭；并行时回归面膨胀 — 须各 epic 自有 NFR14（NFR45）。
- 文档面叠加：addendum、README、`deferred-work.md`、ARCHITECTURE-SPINE 须同一「MVP vs 加深」叙事；禁止混用。
- `ip.rs` 体积风险：Epic 50 与 54 可能先后触碰 — 接受风险域拆分，禁止静默全家桶。

### 引用

- AD-28 — 风险门禁（NFR14）
- PRD NFR14 / FR106；NFR44–NFR47；对照 FR94–105 / NFR40–43
- Correct Course：`sprint-change-proposal-2026-09-10-phase13-mvp-commercial-deepen.md`（approved）
- Addendum：`prd-rhdl-2026-08-19/addendum.md` — Phase 13
- 体例：`nfr14-risk-epic40-literal-path-b.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 48 关闭条件（Story 48.4 勾选）

- [ ] **FR106 / Correct Course + PRD：** Phase 13 批准文案验收 — Story 48.2
- [ ] **README / deferred：** Phase 12 MVP vs Phase 13 加深区分 — Story 48.3
- [ ] **AD 指针：** Deferred / NFR46 门禁可执行 — Story 48.4
- [ ] **NFR44–47：** 加深隔离与可选显式合同已登记
- [ ] **禁止事项未触发：** 无改写 FR94–105 为失败；无未合 FR106 开 49–56；无静默扩大子集
- [ ] **品牌 / 依赖：** 仍为 Bitloom；设计 crate 只依赖 `bitloom-prelude`
- [ ] **Epic 49–56：** 本 epic 关闭前不得标 ready（关闭后仍 backlog，直至各自 NFR14）

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 48 故事 48.2–48.4 标 `ready`。**  
**Phase 12 MVP 关闭面保留（NFR44）；FR106 未验收 / Epic 48 未关闭前不得开闸 Epic 49–56。**
