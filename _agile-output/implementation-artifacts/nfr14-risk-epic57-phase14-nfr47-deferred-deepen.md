# NFR14 风险记录 — Epic 57 Phase 14 加深合同闸门（NFR47 未选加深升格 / FR116）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 14 **NFR48–NFR51**；闸门 **FR116**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic48-mvp-commercial-deepen.md`。  
> **前置：** Phase 12（Epic 40–47）字面绿 MVP complete；Phase 13（Epic 48–56）商业加深 complete；Correct Course `sprint-change-proposal-2026-09-10-phase14-nfr47-deferred-deepen.md` **approved**（2026-09-10）；PRD/addendum Phase 14 段落已落地（commit `6781d53`）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 57 后续故事 **57.2–57.4** 标为 `ready`，亦不得开工实现。**Epic 57 未关闭 / FR116 未验收前，Epic 58–63 不得标 `ready`。**

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR116 / Epic 57；NFR14、NFR48、NFR49、NFR50、NFR51；对照 FR94–115 / NFR40–47；FR123 宣称门面 |
| 记录日期 | 2026-09-10 |
| 状态 | **open** — Story 57.1 done；Story 57.2 done（FR116 合同戳验收）；57.3–57.4 仍 backlog；Epic 58–63 仍 backlog（不得 ready） |

### Phase 12/13 关闭面 vs Phase 14 加深边界（NFR48 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 12 字面绿 MVP** | FR94–105 / Epic 40–47 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 13 MVP→商业加深** | FR106–115 / Epic 48–56 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 14 NFR47 未选加深升格** | FR116–123 / Epic 57–63；deferred→显式 FR | **本合同**（Correct Course 已批准） |

**选定：在保留 Phase 12/13 关闭面的前提下，授权 NFR47 未选子集升格加深。**  
本记录禁止把加深叙述偷换成「Phase 12/13 AC 未达标后的补救」。

### (a) 上游约束

- **Correct Course 批准（2026-09-10）：** `sprint-change-proposal-2026-09-10-phase14-nfr47-deferred-deepen.md` 批准 Phase 14 = Epic 57–63 · FR116–FR123 / NFR48–NFR51；不回滚 Phase 12/13。
- **PRD addendum：** 「2026-09-10 Update — Phase 14 NFR47 未选加深升格」已落地；「Tywaves / syn-scan / SBY / VIP GPIO / Handshake / 官方风格全家桶」类宣称仅可引用 FR116–122 对应关闭证据（**FR123**）；**禁止**用 Phase 13 商业加深冒充本批未选加深完成面。
- **仍待故事落地（本记录不开工）：** README / `deferred-work.md` 诚实面同步（→ **57.3**）；ARCHITECTURE-SPINE Deferred / AD 指针（→ **57.4** / **NFR50**）。Correct Course + addendum 已授权合同；正文差分由 57.3–57.4 落地。
- **FR117–122 加深范围摘要：**
  1. **FR117** — Tywaves 级 typed IDE 波形（超出 FR104 `interactive.html` / FR114 LCOV GUI）；Epic 58
  2. **FR118** — 无 metadata 全树 `#[bitloom::top]` syn-scan（超出 FR99 DesignFixture / FR113 Cargo-graph+metadata）；Epic 59
  3. **FR119** — SymbiYosys/SMT 形式路径（原 FR112 分支 A；超出 FR100 F1-(i) / FR112 分支 B）；Epic 60；**须钉死 formal/SBY 工具链与义务**
  4. **FR120** — 商业 VIP GPIO 全家桶（超出 FR98 四类近 VIP / FR108 P1–P4）；Epic 61
  5. **FR121** — Handshake / 动态数据流默认可综合；须修订 **AD-25**；Epic 62
  6. **FR122** — 官方风格 Chisel 全家桶（超出 FR111 D1+D3）；可能修订 **AD-27**；默认不恢复 Parser；Epic 63
- **NFR48：** Phase 14 关闭不得改写 Phase 12 FR94–105 或 Phase 13 FR106–115「已关闭」。
- **NFR49：** Phase 14 各实现 epic 开工前独立 NFR14（本记录为 Epic 57 门）。
- **NFR50：** 触及 AD-25/27（及 formal/SBY 路径）时须先修订脊柱再标实现 story ready。
- **NFR51：** 升格后的 FR 禁止静默扩大超出各 epic 风险记录钉死的子集；未列入本清单的 deferred 仍须另开合同。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **与 Phase 12/13：** FR94–115 关闭证据保留；**不得**作为 Phase 14 加深完成口径。

### (b) 粗工期带

- **预计：** Epic 57 整体约 **0.75–2 人周**（57.1 本风险记录 ≤0.25 人周；57.2 Correct Course/PRD 验收对齐 0.25–0.5 人周；57.3 README/deferred 同步 0.25–0.75 人周；57.4 AD 指针收口 0.25–0.5 人周）。**Phase 14 全盘（Epic 58–63）为多年/多 epic 加深带**，不计入本 epic 人周，但必须在计划中显式承认。
- **置信度 / 假设：** 中（对本 epic 文档/合同故事为中–高；对加深总工期为低）。假设不回滚 FR94–115；假设 57.3–57.4 只改合同指针与诚实文档、不提前实现 FR117+。若提前开闸 58–63，维护叠加显著上修。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **57.2–57.4** 标 `ready` 或开工实现。
- **FR116 未验收 / Epic 57 未关闭前，不得将 Epic 58–63 标 `ready` 或开工实现。**
- **不得改写 Phase 12/13 FR94–115「已关闭」为失败**（NFR48）。
- **不得用 Phase 13 商业加深冒充 NFR47 未选加深 / Phase 14 完成面**（FR123）。
- **不得静默扩大超出本记录 / 各实现 epic NFR14 钉死的加深子集**（NFR51）。
- 不得在未引用/修订适用 **AD-25 / AD-27**（及 formal/SBY 工具链说明）时声称 FR121/122/119 已合法关闭（NFR50；指针 → 57.4；实质修订 → 62/63/60）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR48 / NFR49 / NFR50 / NFR51** 共同责任人
- 备份 / 升级路径：撤回 Phase 14 / 缩回仅 Phase 13 商业加深口径须升级至产品 / Correct Course 批准人；AD 修订争议升级至架构（AD-28）维护者。

---

### FR117–122 → Epic 对照

| FR | Epic | 加深相对 Phase 12/13 关闭面 |
| --- | --- | --- |
| FR117 | 58 | Tywaves typed IDE vs FR104 / FR114 LCOV |
| FR118 | 59 | syn-scan vs FR99 DesignFixture / FR113 metadata |
| FR119 | 60 | SBY/SMT vs FR100 F1-(i) / FR112 分支 B |
| FR120 | 61 | 商业 VIP GPIO vs FR98 / FR108 近 VIP |
| FR121 | 62 | Handshake 默认可综合 vs FR95/96 / FR110（须 AD-25） |
| FR122 | 63 | 官方风格全家桶 vs FR97 / FR111 D1+D3（可能 AD-27） |

### 须同步的 AD（NFR50 · 本记录登记）

| AD / 工具链 | Phase 13 现状 | Phase 14 加深目标 |
| --- | --- | --- |
| **AD-25** | 允许 FR95/FR110 树内 HLS；**禁止** Handshake 为默认可综合语义 | 允许 **FR121** Handshake / 动态数据流默认可综合（Epic 62 修订） |
| **AD-27** | 允许 FR97/FR111 idiomatic；不恢复 Parser | 允许 **FR122** 官方风格全家桶（Epic 63；默认仍不恢复 Parser） |
| **formal / SBY** | FR100 F1-(i)；FR112 分支 B MemRead≡tick | **FR119** SymbiYosys 与/或 SMT 绑定路径（Epic 60）；工具链形状、CI 义务、缺工具失败可读性由该 epic NFR14 钉死 |
| Deferred 指针 | 现行完成标签 = Phase 12+13 | 区分 Phase 13 商业加深 vs Phase 14 NFR47 升格（→ **57.4**） |

### Epic 57 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **57.1** | 本 NFR14 风险记录 + ATDD；钉死 NFR48 边界 / FR117–122 摘要 / 禁止项 | **本故事** |
| **57.2** | Correct Course + PRD/addendum Phase 14 验收（合同戳） | Gate：须本记录后才可 ready |
| **57.3** | 同步 README / deferred / 路线图指针（FR116 / FR123） | Gate：须本记录后才可 ready |
| **57.4** | AD 指针与 Epic 57 收口；勾选关闭条件 | Gate：须本记录后才可 ready；关闭后才可开 58–63 |

### 并行 / 维护叠加（Chipyard 式 · NFR49）

- Epic 58–63 **可并行**，但均硬依赖 Epic 57 关闭；并行时回归面膨胀 — 须各 epic 自有 NFR14（NFR49）。
- 文档面叠加：addendum、README、`deferred-work.md`、ARCHITECTURE-SPINE 须同一「Phase 13 vs Phase 14」叙事；禁止混用。
- `ip.rs` 体积风险：Epic 61 可能触碰 — 拆分非 FR120 关闭条件。

### 引用

- AD-28 — 风险门禁（NFR14）
- PRD NFR14 / FR116；NFR48–NFR51；对照 FR94–115 / NFR40–47
- Correct Course：`sprint-change-proposal-2026-09-10-phase14-nfr47-deferred-deepen.md`（approved）
- Addendum：`prd-rhdl-2026-08-19/addendum.md` — Phase 14
- 体例：`nfr14-risk-epic48-mvp-commercial-deepen.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 57 关闭条件（Story 57.4 勾选）

- [x] **FR116 / Correct Course + PRD：** Phase 14 批准文案验收 — Story 57.2
- [ ] **README / deferred：** Phase 13 商业加深 vs Phase 14 加深区分 — Story 57.3
- [ ] **AD 指针：** Deferred / NFR50 门禁可执行 — Story 57.4
- [ ] **NFR48–51：** 加深隔离与未选子集显式合同已登记
- [ ] **禁止事项未触发：** 无改写 FR94–115 为失败；无未合 FR116 开 58–63；无静默扩大子集
- [ ] **品牌 / 依赖：** 仍为 Bitloom；设计 crate 只依赖 `bitloom-prelude`
- [ ] **Epic 58–63：** 本 epic 关闭前不得标 ready（关闭后仍 backlog，直至各自 NFR14）

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 57 故事 57.2–57.4 标 `ready`。**  
**Phase 12/13 关闭面保留（NFR48）；FR116 未验收 / Epic 57 未关闭前不得开闸 Epic 58–63。**
