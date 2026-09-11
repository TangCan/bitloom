# NFR14 风险记录 — Epic 86 发版后诚实面与 SemVer 跟进（FR153）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 18 **NFR64–NFR67**；实现面 **FR153**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic85-cli-install-fr149-152.md`。  
> **前置：** Epic 85 / FR149–152 **closed**；`bitloom-firrtl` / `bitloom-viz` / `bitloom` **1.0.0** 已在 crates.io。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 86 后续故事 **86.2–86.3** 标为 `ready`，亦不得开工实现。**不得**在本 epic 关闭前宣称 FR153 已关闭。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR153 / Epic 86；NFR14、NFR64、NFR65、NFR67；对照 FR148–152 |
| 记录日期 | 2026-09-11 |
| 状态 | closed — Story 86.3 勾选完成；Epic 86 关闭；FR153 可宣称；**NFR59 仍 deferred**（NFR67） |
| **选定** | 在保留 Phase 17–18 关闭面的前提下，授权 SemVer assume-published / 移除 1.0.0 特例，并更新发版诚实文档；**不清空 NFR59**；**不扩大 FR142** |

### Phase 17 / 85 关闭面 vs Epic 86 实现边界（NFR64 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **Phase 17 库 1.0** | FR141–147；库 crate 1.0.0 已上 crates.io | **仍有效；不得改写「已关闭」**（NFR64） |
| **Epic 84–85** | FR148–152；CLI / firrtl / viz 已上架 | **已关闭；不得回写为未批准** |
| **Epic 86 / FR153** | SemVer assume-published + 文档/Release 诚实 | **本实现 epic** |

**选定：文档与 SemVer 默认跟进「库+CLI 均已 1.0.0 上架」，不扩大 FR142，不清空 NFR59（NFR67）。**

### FR153 范围（本 epic）

| 交付 | 故事 |
| --- | --- |
| NFR14 风险记录 + ATDD；门禁 86.2–86.3 | **86.1** |
| 落地 `BITLOOM_SEMVER_ASSUME_PUBLISHED` **或移除 1.0.0 特例**；更新 `docs/fr146-*` / README / GitHub Release（若 `gh` 可用）；`just semver-check` 默认 `--release-type minor` | **86.2** |
| 文档/deferred/NFR14 勾选；Epic 86 / Phase 18 故事清单收口；NFR59 仍 deferred | **86.3** |

### (a) 上游约束

- **Epic 85 已关闭：** FR149–152；`bitloom` / `bitloom-firrtl` / `bitloom-viz` **1.0.0** 已在 crates.io；`cargo install bitloom` 可用（FR151）。
- **SemVer 门禁：** `scripts/semver-check.sh` / `just semver-check` / CI `semver-check`（FR144）仍有效；首次 1.0.0 相对 crates.io 0.x 的 `--release-type major` 特例在实发后须切换。
- **NFR64：** 不得改写 Phase 12–17「已关闭」；不得把诚实更新偷换成 Phase 17 失败补救。
- **NFR67：** 不得静默吞并 **NFR59**；不得静默扩大 **FR142** 表面。
- **诚实面：** 文案须区分库 crate 与 CLI 上架状态；未上架时不得写「CLI 已可 install」——**本 epic 时已上架，须与 crates.io 证据一致**。
- **公开品牌：** **Bitloom**（crates.io / CLI：`bitloom`）。

### (b) 粗工期带

- **预计：** Epic 86 整体约 **0.5–1 人周**（86.1 ≤0.25；86.2 SemVer+文档 0.25–0.5；86.3 收口 ≤0.25）。
- **置信度 / 假设：** 高（无 rename/publish；仅脚本默认与文档）。假设 registry 上仍为 1.0.0；`gh` 不可用则文档化 Release 更新路径。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **86.2–86.3** 标 `ready` 或开工实现。
- **不得暗示 NFR59 已清**（NFR67）。
- **不得静默扩大 FR142** 表面清单。
- **不得在未上架时写「CLI 已可 install」**；本 epic 时已上架则宣称须与证据一致（FR151）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。
- 不得把 SemVer 默认切换冒充扩大公开 API 承诺或静默 major 破坏。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR64 / NFR65 / NFR67** 共同责任人
- 备份 / 升级路径：若需恢复 1.0.0 major 特例或扩大 FR142，须升级至产品 / Correct Course 批准人。

---

### Epic 86 故事分工

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **86.1** | 本 NFR14 + ATDD | **本故事** |
| **86.2** | FR153 SemVer + 文档诚实 | Gate：须本记录后才可 ready |
| **86.3** | 收口；勾选 Epic 86 / Phase 18 | 关闭本记录 |

### 引用

- AD-28 — 风险门禁（NFR14）
- PRD NFR14；FR153；NFR64–NFR67
- Correct Course：`sprint-change-proposal-2026-09-11-phase18-cli-crates-io-publish.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 86 关闭条件（Story 86.3 勾选）

- [x] **FR153：** SemVer 默认正确（assume-published / 无 1.0.0 特例）+ `docs/fr146-*` / README / Release 诚实 — Story 86.2
- [x] **NFR14 / NFR64–67：** 本记录勾选关闭；**NFR59 仍 deferred**（NFR67）
- [x] **Phase 18：** Epic 84–86 故事清单齐；CLI crates.io 宣称须引 FR148–153

**Epic 86 已关闭（Story 86.3）：** FR153 可宣称；`just semver-check` 对 ≥1.0.0 默认 minor；库+CLI 上架诚实面齐；Phase 18 实现故事齐（Epic 84–86）；**不得**静默吞并 NFR59（NFR67）；公开品牌 **Bitloom**。
