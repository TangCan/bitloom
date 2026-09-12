# NFR14 风险记录 — Epic 87 Phase 19 合同闸门（NFR59 全子集升格 + FR152(a) / FR154）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 19 **NFR68–NFR72**；闸门 **FR154**；宣称须引 **FR154–165**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic84-phase18-cli-crates-io-publish.md`。  
> **前置：** Phase 12–18 FR94–153 / Epic 40–86 **closed**；Correct Course `sprint-change-proposal-2026-09-12-phase19-nfr59-fr152a.md` **approved**（2026-09-12；`correctCoursePhase19Approved: 2026-09-12`）；Epic 87–98 已 seed（commit `ef62055`）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 87 后续故事 **87.2–87.4** 标为 `ready`，亦不得开工实现。**Epic 87 未关闭 / FR154 未验收前，Epic 88–98 不得标 `ready`。**

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR154 / Epic 87；NFR14、NFR68、NFR69、NFR71、NFR72；对照 FR148–153 / NFR64–67；FR155–165 实现面 |
| 记录日期 | 2026-09-12 |
| 状态 | open / in-progress — Story 87.1；Epic 87 未关闭；闸门未开 |
| **选定** | 在保留 Phase 12–18 关闭面的前提下，授权 Phase 19「NFR59 全子集升格 + FR152(a)」合同闸门（FR154）；实现属 Epic 88–98 |

### Phase 12–18 关闭面 vs Phase 19 边界（NFR68 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 12–16** | FR94–140 / Epic 40–78 关闭证据 | **仍有效；不得改写为失败** |
| **Phase 17 公开 API 稳定门 / Bitloom 1.0** | FR141–147；库 crate **1.0.0** | **仍有效** |
| **Phase 18 CLI crates.io** | FR148–153；CLI **1.0.0**；FR152 **(b)** | **仍有效；不得改写「已关闭」**（NFR68） |
| **Phase 19 NFR59 + FR152(a)** | FR154–165 / Epic 87–98；NFR68–NFR72 | **本合同**（Correct Course 已批准 2026-09-12） |

**选定：在保留 Phase 12–18 关闭面的前提下，授权 NFR59 九条升格与 FR152(a) lsp live publish。**  
本记录禁止把本批叙述偷换成「Phase 18 / CLI 失败后的补救」或「1.0 alone ⇒ NFR59/lsp 已授权」。  
**Phase 19 口径** = FR154–165 对应关闭 + 诚实列出超出各 NFR14 钉死子集的仍须新合同项（**NFR71**）；**不等于**静默扩大 FR142；**`git push` 不是 FR**。

### 批准默认（Q1–Q5 · Correct Course）

| # | 决议 |
| --- | --- |
| **Q1** | NFR59 README **九条全部**升格为 **FR157–FR165**（各 epic NFR14 钉死子集；禁止静默超子集 — NFR71） |
| **Q2** | **FR152 (a)** — `bitloom-lsp`：`publish=true` + version；重写 `docs/fr152-*`；(a)；**live** `cargo publish -p bitloom-lsp`（Epic 88 / FR155） |
| **Q3** | **不得**改写 Phase 12–18「已关闭」（**NFR68**） |
| **Q4** | **不得**静默扩大 FR142；宣称须引已关 FR（**FR156** / **NFR72**） |
| **Q5** | **MSRV** 保持现行；lsp 版本建议与工作区 **1.0.0** 对齐（除非故事另决）；**FR154** 须 Correct Course + PRD 戳后方可开实现 epic |

### (a) 上游约束

- **Correct Course 批准（2026-09-12）：** `sprint-change-proposal-2026-09-12-phase19-nfr59-fr152a.md` 批准 Phase 19 = Epic 87–98 · FR154–FR165 / NFR68–NFR72；不回滚 Phase 12–18。
- **PRD addendum：** 「Phase 19」合同戳待 **Story 87.2** 验收；「lsp 已上架 / NFR59 某条已交付」类宣称仅可引用对应 FR 关闭证据；**禁止**用 Phase 18 alone 冒充。
- **合同落地：** README / deferred 诚实面 — **Story 87.3**；ARCHITECTURE-SPINE / AGENTS 指针 — **Story 87.4**。
- **FR155–165 范围摘要：**
  1. **FR155** — bitloom-lsp FR152(a) live crates.io；Epic 88
  2. **FR156** — Phase 19 宣称诚实门；Epic 98
  3. **FR157** — 自动 FSM 标签提取；Epic 89
  4. **FR158** — 第三方 LCOV GUI 一等集成；Epic 90
  5. **FR159** — MemRead stub→完整生成；Epic 91
  6. **FR160** — 非 Cargo monorepo 路径扫描；Epic 92
  7. **FR161** — formal-sby 镜像卫生；Epic 93
  8. **FR162** — 更深 GUI/IDE；Epic 94
  9. **FR163** — 未列协议手写 FL；Epic 95
  10. **FR164** — 更广 CIRCT/MLIR / 仿真门禁；Epic 96
  11. **FR165** — 更深 Chisel/Parser 生态；Epic 97
- **NFR68：** Phase 19 关闭不得改写 Phase 12–18 FR94–153「已关闭」。
- **NFR69：** Phase 19 各实现 epic 开工前独立 NFR14（本记录为 Epic 87 门）。
- **NFR71：** 禁止静默扩大超出各 epic NFR14 钉死子集；未写入 FR157–165 的新加深须另开合同。
- **NFR72：** 宣称仅经 FR156；不得用 Phase 18 CLI 上架冒充 lsp/NFR59。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **软序（非硬依赖）：** `87 →（88 ‖ 89…97）→ 98`；Epic 95（`ip/`）与其它触及面建议串行。

### (b) 粗工期带

- **预计：** Epic 87 整体约 **0.75–2 人周**（87.1 本风险记录 ≤0.25 人周；87.2 Correct Course/PRD 验收 0.25–0.5；87.3 README/deferred 0.25–0.75；87.4 AD 指针收口 0.25–0.5）。**Phase 19 全盘（Epic 88–98）为 High effort**（九条加深 + live publish），不计入本 epic 人周，但必须在计划中显式承认。
- **置信度 / 假设：** 中（对本 epic 文档/合同故事为中–高；对 FR157–165 / CIRCT/Chisel/IDE 为中–低）。假设不回滚 FR94–153；假设 87.3–87.4 只改合同指针与诚实文档、不提前 publish/加深。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **87.2–87.4** 标 `ready` 或开工实现。
- **FR154 未验收 / Epic 87 未关闭前，不得将 Epic 88–98 标 `ready` 或开工实现。**
- **不得改写 Phase 12–18 FR94–153「已关闭」为失败**（NFR68）。
- **不得用 1.0 / CLI 上架 alone 冒充 NFR59 / FR152(a) 合同已授权。**
- **不得静默扩大 FR142** 表面承诺。
- **不得把未写入 FR157–165 的新加深冒充已交付**（NFR71）。
- **不得把 `git push` / 远程同步当成产品 FR。**
- 不得静默 publish `rhdl` / `rhdl-bits`（AD-2 / 继承）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR68 / NFR69 / NFR71 / NFR72** 共同责任人
- 备份 / 升级路径：缩回「仅 lsp / 不做 NFR59 全九条」须升级至产品 / Correct Course 批准人；AD-25/AD-27 争议升级至架构（AD-28）维护者。

---

### FR155–165 → Epic 对照

| FR | Epic | 相对 Phase 18 |
| --- | --- | --- |
| FR155 | 88 | FR152(a) lsp live（≠ 改写 FR152(b) 关闭证据为失败） |
| FR157–165 | 89–97 | NFR59 九条升格 |
| FR156 | 98 | 宣称诚实门 |

### Epic 87 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **87.1** | 本 NFR14 风险记录 + ATDD；钉死 NFR68 边界 / FR155–165 / Q1–Q5 / 禁止项 | **本故事** |
| **87.2** | Correct Course + PRD/addendum Phase 19 验收（合同戳） | Gate：须本记录后才可 ready |
| **87.3** | 同步 README / deferred / 路线图指针（FR154） | Gate：须本记录后才可 ready |
| **87.4** | AD 指针与 Epic 87 收口；勾选关闭条件 | Gate：须本记录后才可 ready；关闭后才可开 88–98 |

### 并行 / 维护叠加（Chipyard 式 · NFR69）

- Epic 88–97 **可并行规划**但硬依赖 Epic 87 关闭；软序 `87 →（88 ‖ 89…97）→ 98`。
- 文档面叠加：addendum、README、`deferred-work.md`、ARCHITECTURE-SPINE、fr152 须同一「Phase 18 vs Phase 19」叙事；禁止混用。
- Publish / 加深风险：Epic 88 live publish；Epic 94–97 外部工具；须各自 NFR14。

### 引用

- AD-28 — 风险门禁（NFR14）；AD-2 — 对外 `bitloom-*`；AD-6 — `bitloom-prelude`
- PRD NFR14 / FR154；NFR68–NFR72；对照 FR148–153 / NFR64–67
- Correct Course：`sprint-change-proposal-2026-09-12-phase19-nfr59-fr152a.md`（approved 2026-09-12）
- Addendum：`prd-rhdl-2026-08-19/addendum.md` — Phase 19（→ 87.2）
- 体例：`nfr14-risk-epic84-phase18-cli-crates-io-publish.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 87 关闭条件（Story 87.4 勾选）

- [ ] **FR154 / Correct Course + PRD：** Phase 19 批准文案验收 — Story 87.2
- [ ] **README / deferred：** Phase 18 vs Phase 19 合同区分 — Story 87.3
- [ ] **AD 指针：** ARCHITECTURE-SPINE / AGENTS Phase 19 指针 — Story 87.4
- [ ] **NFR68–72：** 边界与诚实义务写入本记录并保持
- [ ] **禁止事项未触发：** 88–98 在 Epic 87 关闭前未标 ready
- [ ] **品牌 / 依赖：** Bitloom / `bitloom-prelude`；禁 publish `rhdl`/`rhdl-bits`
- [ ] **Epic 88–98：** 仍须各自 NFR14；未实现前不得宣称对应 FR 关闭
