# NFR14 风险记录 — Epic 94 更深 GUI/IDE 子集（FR162）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 19 **NFR68–NFR72**；实现面 **FR162**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic73-upstream-tywaves-gui-ide.md` / `nfr14-risk-epic93-formal-sby-image-hygiene-fr161.md`。  
> **前置：** Epic 87 / FR154 **closed**；Epic 73 / **FR134** G1–G4 upstream Tywaves GUI/IDE **closed**；Epic 65 / **FR125** `--tywaves` **closed**；Epic 93 / FR161 **closed**。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 94 后续故事 **94.2–94.3** 标为 `ready`，亦不得开工实现。**不得**以 FR134 G1–G4 alone 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR162 / Epic 94；NFR14、NFR68、NFR69、NFR70、NFR71、NFR72；对照 FR134 / FR125 / FR117 / FR104 |
| 记录日期 | 2026-09-12 |
| 状态 | open / in-progress — Story 94.1；Epic 94 未关闭 |
| **选定** | 在保留 FR134 G1–G4（opt-in `--tywaves-gui` / GUI·IDE manifest）关闭面的前提下，授权 **MVP：替换「默认仅 VCD / `typed-wave.html` 为唯一波形完成面」** — 使更深 GUI/Tywaves 面成为 `cargo bitloom wave` **一等默认/主表面**（VCD/`typed-wave.html` **保留为次要/并存**，不得再 alone 冒充唯一完成口径） |

### Phase 12–18 / FR134 关闭面 vs Epic 94 实现边界（NFR68 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR104 / FR117** | `interactive.html` / 自研 `typed-wave.html` | **仍有效**；alone ≠ FR162 |
| **FR125 / Epic 65** | `--tywaves` sidecar / launch | **仍有效**；alone ≠ FR162 |
| **FR134 / Epic 73** | G1–G4 opt-in `--tywaves-gui` + GUI/IDE pin/manifest | **仍有效；不得改写为失败**；**alone ≠ FR162**（仍属「额外开关」加深，未改默认唯一波形面） |
| **Epic 87–93** | 闸门 / lsp / … / sby 卫生 | **已关闭；本 epic 不做** |
| **Epic 94 / FR162** | **更深 GUI/IDE 子集**（本记录选定：默认波形面升格） | **本实现 epic** |
| **Epic 95–98** | 其余 NFR59 + 宣称 | **本 epic 不做** |

**选定：关闭「默认完成面仍仅是 VCD/`typed-wave.html`」缺口；≠ 重做 FR134 G1–G4；≠ 本批交付完整 ChiselSim 或额外 IDE 商店多端。**

### FR162 候选子集与本批选定（NFR71）

| 候选（Epic / README 明示） | 本 epic |
| --- | --- |
| **替换默认 VCD / `typed-wave.html` 唯一波形面** | **选定 = FR162 MVP** |
| 完整 ChiselSim 耦合 | **不做**（须新合同 / NFR71） |
| G1 外额外 IDE 商店多端发布（Open VSX / JetBrains 等） | **不做**（须新合同 / NFR71） |

### 选定子集钉死（94.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **默认/主表面** | 文档 + CLI：`cargo bitloom wave`（无强制仅 typed-wave）将 **Tywaves GUI 深度面**（FR134 契约产物：`tywaves.gui.install.json` / `tywaves.gui.manifest.json` 与/或文档钉死的等价主表面）作为 **一等默认或显式 primary**；`typed-wave.html` / VCD **可并存但不得再被文档写成唯一完成面** | 删除 FR117/FR104 产物；强制用户安装真实 Surfer GUI 才能 `just test` |
| **上游 / 商店契约** | 复用 FR134 G1 已钉死的 GUI 包 + VS Code Surfer marketplace id；本 epic **不**新开第二商店 | Open VSX / JetBrains 多端 |
| **失败语义** | 主表面路径在 force-missing / 无效 root 时 **非零可读**（对齐 FR134 G3 `bitloom.tywaves*`）；**不得** silent-Ok 宣称 FR162 | skip/`continue-on-error` |
| **验收谓词** | (1) 文档声明主表面 ≠ typed-wave alone；(2) 默认 `wave`（或文档钉死的 primary 开关）发出 FR134 级 GUI manifests / 等价；(3) FR134/FR125/FR117 回归不破；(4) ATDD 可复现 | 仅改 README 一句而无 CLI/产物 |
| **与 FR134 边界** | FR134 = G1–G4 **opt-in** `--tywaves-gui`；FR162 = **默认/主表面升格** beyond「typed-wave 唯一」；FR134 关闭证据不得改写为失败 | 把 FR134 alone 写成已含默认面替换 |

### 目标产品形状（94.2）

- **入口：** 既有 `cargo bitloom wave`（加深默认行为或文档钉死的 primary 标志）；可保留 `--tywaves-gui` 兼容。
- **产物：** FR134 级 GUI install/manifest（或文档等价主表面）在默认/primary 路径发出；`docs/fr162-*`。
- **语义锚：** 「唯一默认完成面」不再是 VCD/`typed-wave.html` alone。

### 验收谓词 / 失败语义（94.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 默认/primary `wave` 路径发出更深 GUI 面产物；文档对照 FR134；ATDD 可复现 |
| **负向 / 失败** | force-missing → **可读非零**；**不得** silent-Ok |
| **禁止勾选** | 仅 FR134 G1–G4；仅 FR125；仅 FR117 typed-wave；仅 docs |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **94.1** | 本 NFR14 |
| **94.2** | 默认/primary 面实现 + ATDD；`docs/fr162-*` |
| **94.3** | README/deferred 收口；勾选 Epic 94；写明未选 ChiselSim/多商店仍须新合同 |

### (a) 上游约束

- **Epic 87 已关闭；Epic 88–93 已关闭。**
- **FR134：** G1–G4 **仍有效**；本 epic 升格 **默认波形面**，不得把 FR134 改写成「已含默认唯一面替换」。
- **FR125 / FR117 / FR104：** 仍有效；alone ≠ FR162。
- **NFR68：** 不得改写 FR94–161 / FR134「已关闭」。
- **NFR70：** 若触 IDE 商店/上游 pin，保持与 FR134 G1 诚实一致（本批不扩第二商店）。
- **NFR71：** 禁止静默扩大到 ChiselSim / 额外商店多端。
- **NFR72：** 未关 FR162 前不得宣称更深 GUI/IDE 已交付；**不得以 FR134 alone 冒充 FR162**。
- **品牌：** **Bitloom**；设计 crate → **`bitloom-prelude`**（AD-6）；Tywaves runtime **不得**进入设计 crate 依赖。
- **软序：** 94.2 → 94.3。

### (b) 粗工期带

- **预计：** Epic 94 整体约 **0.75–2 人周**（94.1 ≤0.25；94.2 CLI/默认面 + ATDD 0.5–1.5；94.3 收口 0.25）。
- **置信度 / 假设：** 中（默认行为变更须保持 CI stub GUI root 可用）。假设不改 FR134 pin 身份。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **94.2–94.3** 标 `ready` 或开工实现。
- **不得以 FR134 G1–G4 alone 勾选 FR162。**
- **不得以 FR125 / FR117 / FR104 alone 冒充 FR162。**
- **不得仅改文档关闭 FR162。**
- **不得静默扩大到完整 ChiselSim 或额外 IDE 商店多端。**
- **不得把 Tywaves 塞进 `bitloom-prelude`。**
- **不得改写 Phase 12–18 / FR134 关闭证据为失败。**
- 不得把本记录冒充 **NFR14-crates**（FCFS）完成定义。
- 不得在 FR162 未关时宣称 NFR59「全清」。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 与 **NFR68 / NFR69 / NFR70 / NFR71 / NFR72** 共同责任人
- 备份 / 升级路径：改选 ChiselSim 或额外商店多端代替本选定，须升级至产品 / Correct Course 批准人并修订本记录。

---

### Epic 94 关闭条件（Story 94.3 勾选）

- [ ] **FR162 钉死子集实现 + 验收** — Story 94.2
- [ ] **文档 / deferred / README 收口** — Story 94.3
- [ ] **NFR68/70/71/72：** 边界与诚实义务保持；未选 ChiselSim/多商店须新合同
- [ ] **品牌 / AD-6：** Bitloom；设计 crate → `bitloom-prelude`
- [ ] **其余 FR163–165：** 未关前不得宣称 NFR59 全清
