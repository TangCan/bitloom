# NFR14 风险记录 — Epic 64 Phase 15 加深合同闸门（NFR51 剩余升格 / FR124）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 15 **NFR52–NFR55**；闸门 **FR124**；宣称 **FR132**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic57-phase14-nfr47-deferred-deepen.md`。  
> **前置：** Phase 12–14 FR94–123 已关闭；Correct Course `sprint-change-proposal-2026-09-10-phase15-nfr51-leftover-deepen.md` **approved**（2026-09-10）；PRD/addendum Phase 15 段落已落地（commit `855bd75`）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 64 后续故事 **64.2–64.4** 标为 `ready`，亦不得开工实现。**Epic 64 未关闭 / FR124 未验收前，Epic 65–71 不得标 `ready`。**

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR124 / Epic 64；NFR14、NFR52、NFR53、NFR54、NFR55；对照 FR94–123 / NFR40–51；FR132 宣称门面 |
| 记录日期 | 2026-09-10 |
| 状态 | accepted — Story 64.1；Epic 64 关闭勾选见 Story 64.4 |

### Phase 12–14 关闭面 vs Phase 15 加深边界（NFR52 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 12 字面绿 MVP** | FR94–105 / Epic 40–47 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 13 MVP→商业加深** | FR106–115 / Epic 48–56 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 14 NFR47 未选加深升格** | FR116–123 / Epic 57–63 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 15 NFR51 剩余升格** | FR124–132 / Epic 64–71；deferred→显式 FR | **本合同**（Correct Course 已批准） |

**选定：在保留 Phase 12–14 关闭面的前提下，授权 NFR51 明示剩余升格加深。**  
本记录禁止把加深叙述偷换成「Phase 14 AC 未达标后的补救」。

### (a) 上游约束

- **Correct Course 批准（2026-09-10）：** `sprint-change-proposal-2026-09-10-phase15-nfr51-leftover-deepen.md` 批准 Phase 15 = Epic 64–71 · FR124–FR132 / NFR52–NFR55；不回滚 Phase 12–14。
- **PRD addendum：** 「2026-09-10 Update — Phase 15 NFR51 剩余升格」已落地；「上游 Tywaves 一等 / 更多 IP FL / 强制 sby CI / 全 SoC pad / CIRCT Handshake / Style Guide·Parser / ip.rs 拆分」类宣称仅可引用 FR124–131 对应关闭证据（**FR132**）；**禁止**用 Phase 14 完成面冒充本批加深完成面。
- **合同落地：** README / deferred 诚实面 — **Story 64.3**；ARCHITECTURE-SPINE / AGENTS 指针 — **Story 64.4** / **NFR54**。Correct Course + addendum 已授权合同（**Story 64.2** 验收戳）。
- **FR125–131 加深范围摘要：**
  1. **FR125** — 上游 Tywaves 一等集成（超出 FR117 自研 typed-wave）；Epic 65
  2. **FR126** — 更多 IP 手写 FL（原 formal 分支 C；超出 FR103/FR112-B / FR119-(A)）；Epic 66
  3. **FR127** — 默认 CI 强制真 `sby`（超出本机可选 `just formal-sby-check` / FR119）；Epic 67；**须钉死 CI/formal 运维**
  4. **FR128** — 全 SoC pad / 商业对拍深度（超出 FR120 `GpioVip` C1–C4）；Epic 68
  5. **FR129** — CIRCT Handshake 方言 / 多时钟弹性缓冲全家桶；须评估/修订 **AD-25**；Epic 69
  6. **FR130** — 完整 Style Guide 全文 ± Parser 恢复；若恢复 Parser **必须**修订 **AD-27**；Epic 70
  7. **FR131** — `ip.rs` 按协议拆分卫生（拆分本身为关闭条件）；Epic 71；**软序先于 Epic 68**
- **NFR52：** Phase 15 关闭不得改写 Phase 12 FR94–105、Phase 13 FR106–115 或 Phase 14 FR116–123「已关闭」。
- **NFR53：** Phase 15 各实现 epic 开工前独立 NFR14（本记录为 Epic 64 门）。
- **NFR54：** 触及 AD-25/27（及 Phase 15 CI formal / sby 运维）时须先修订脊柱/CI 合同再标实现 story ready。
- **NFR55：** 升格后的 FR 禁止静默扩大超出各 epic 风险记录钉死的子集；未列入本清单的 deferred 仍须另开合同。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **与 Phase 12–14：** FR94–123 关闭证据保留；**不得**作为 Phase 15 加深完成口径。

### (b) 粗工期带

- **预计：** Epic 64 整体约 **0.75–2 人周**（64.1 本风险记录 ≤0.25 人周；64.2 Correct Course/PRD 验收对齐 0.25–0.5 人周；64.3 README/deferred 同步 0.25–0.75 人周；64.4 AD 指针收口 0.25–0.5 人周）。**Phase 15 全盘（Epic 65–71）为多年/多 epic 加深带**，不计入本 epic 人周，但必须在计划中显式承认。
- **置信度 / 假设：** 中（对本 epic 文档/合同故事为中–高；对加深总工期为低）。假设不回滚 FR94–123；假设 64.3–64.4 只改合同指针与诚实文档、不提前实现 FR125+。若提前开闸 65–71，维护叠加显著上修。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **64.2–64.4** 标 `ready` 或开工实现。
- **FR124 未验收 / Epic 64 未关闭前，不得将 Epic 65–71 标 `ready` 或开工实现。**
- **不得改写 Phase 12–14 FR94–123「已关闭」为失败**（NFR52）。
- **不得用 Phase 14 完成面冒充 NFR51 剩余加深 / Phase 15 完成面**（FR132）。
- **不得静默扩大超出本记录 / 各实现 epic NFR14 钉死的加深子集**（NFR55）。
- 不得在未引用/修订适用 **AD-25 / AD-27**（及 CI sby 运维说明）时声称 FR129/130/127 已合法关闭（NFR54；指针 → 64.4；实质修订 → 69/70/67）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR52 / NFR53 / NFR54 / NFR55** 共同责任人
- 备份 / 升级路径：撤回 Phase 15 / 缩回仅 Phase 14 口径须升级至产品 / Correct Course 批准人；AD 修订争议升级至架构（AD-28）维护者。

---

### FR125–131 → Epic 对照

| FR | Epic | 加深相对 Phase 12–14 关闭面 |
| --- | --- | --- |
| FR125 | 65 | 上游 Tywaves 一等 vs FR117 自研 typed |
| FR126 | 66 | 更多 IP 手写 FL vs FR103/112-B / FR119-(A) |
| FR127 | 67 | 默认 CI 真 sby vs 本机 `formal-sby-check` / FR119 |
| FR128 | 68 | 全 SoC pad vs FR120 GpioVip C1–C4 |
| FR129 | 69 | CIRCT Handshake / 多时钟 vs FR121 ready/valid（须 AD-25） |
| FR130 | 70 | 完整 Style Guide ± Parser vs FR122 O1–O4（可能 AD-27） |
| FR131 | 71 | `ip.rs` 协议拆分卫生（软序先于 68） |

### 须同步的 AD / CI（NFR54 · 本记录登记）

| AD / 工具链 | Phase 14 现状 | Phase 15 加深目标 |
| --- | --- | --- |
| **AD-25** | 允许 FR121 Handshake ready/valid 默认可综合 | Epic 69 **可能**再修订以覆盖 CIRCT / 多时钟弹性缓冲（FR129） |
| **AD-27** | 允许 FR122 O1–O4；默认禁止 Parser | Epic 70 完整 Style Guide；**若恢复 Parser 必须**修订 AD-27（FR130） |
| **CI / sby** | FR119 本机可选 `just formal-sby-check` | **FR127** 默认 CI required 真 sby（Epic 67）；缺工具非零可读 |
| Deferred 指针 | 现行完成标签 = Phase 12–14 | 区分 Phase 14 vs Phase 15（→ **64.3 / 64.4**） |

### Epic 64 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **64.1** | 本 NFR14 风险记录 + ATDD；钉死 NFR52 边界 / FR125–131 摘要 / 禁止项 | **本故事** |
| **64.2** | Correct Course + PRD/addendum Phase 15 验收（合同戳） | Gate：须本记录后才可 ready |
| **64.3** | 同步 README / deferred / 路线图指针（FR124 / FR132） | Gate：须本记录后才可 ready |
| **64.4** | AD 指针与 Epic 64 收口；勾选关闭条件 | Gate：须本记录后才可 ready；关闭后才可开 65–71 |

### 并行 / 维护叠加（Chipyard 式 · NFR53）

- Epic 65–71 **可并行**（硬依赖 Epic 64 关闭）；软序建议 **71 → 68**（同触 `ip.rs`）。
- 文档面叠加：addendum、README、`deferred-work.md`、ARCHITECTURE-SPINE 须同一「Phase 14 vs Phase 15」叙事；禁止混用。
- `ip.rs` 体积风险：Epic 71 拆分 + Epic 68 SoC pad — 分 epic 验收。

### 引用

- AD-28 — 风险门禁（NFR14）
- PRD NFR14 / FR124；NFR52–NFR55；对照 FR94–123 / NFR40–51
- Correct Course：`sprint-change-proposal-2026-09-10-phase15-nfr51-leftover-deepen.md`（approved）
- Addendum：`prd-rhdl-2026-08-19/addendum.md` — Phase 15
- 体例：`nfr14-risk-epic57-phase14-nfr47-deferred-deepen.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 64 关闭条件（Story 64.4 勾选）

- [ ] **FR124 / Correct Course + PRD：** Phase 15 批准文案验收 — Story 64.2
- [ ] **README / deferred：** Phase 14 vs Phase 15 加深区分 — Story 64.3
- [ ] **AD 指针：** ARCHITECTURE-SPINE / AGENTS Phase 15 指针 — Story 64.4
- [ ] **NFR52–55：** 边界与诚实义务写入本记录并保持
- [ ] **禁止事项未触发：** 65–71 在 Epic 64 关闭前未标 ready
- [ ] **品牌 / 依赖：** Bitloom / `bitloom-prelude`
- [ ] **Epic 65–71：** 仍须各自 NFR14；未实现前不得宣称对应 FR 关闭
