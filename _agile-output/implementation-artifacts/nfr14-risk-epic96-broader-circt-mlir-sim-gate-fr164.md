# NFR14 风险记录 — Epic 96 更广 CIRCT/MLIR / 仿真门禁（FR164）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 19 **NFR68–NFR72**；实现面 **FR164**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic76-external-circt-compile-sim-gate.md` / `nfr14-risk-epic95-unlisted-protocol-handwritten-fl-fr163.md`。  
> **前置：** Epic 87 / FR154 **closed**；Epic 76 / **FR137** 外部 CIRCT **编译**门禁 MVP **closed**；Epic 69 / **FR129** C1–C4 **closed**；Epic 95 / FR163 **closed**。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 96 后续故事 **96.2–96.3** 标为 `ready`，亦不得开工实现。**不得**以 FR137 编译门禁 alone 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR164 / Epic 96；NFR14、NFR68、NFR69、NFR70、NFR71、NFR72；对照 FR137 / FR129 / FR121 / AD-9 / AD-25 |
| 记录日期 | 2026-09-12 |
| 状态 | open / in-progress — Story 96.1；Epic 96 未关闭 |
| **选定** | 在保留 FR137 **编译门禁** MVP（`just circt-external-check` / CI `circt-external` / firtool-1.155.0）关闭面的前提下，授权 **MVP：外部 CIRCT / firtool 仿真门禁加深** — 文档钉死路径 + CI required（或文档钉死可复现路径在 CI 强制跑通）对代表性 `.fir` 做 **sim/执行谓词**（超出「仅 compile」）；更广 CIRCT/MLIR allocation / 多 dialect lower 全家桶 **不做**（须新合同 / NFR71） |

### Phase 12–18 / FR137 关闭面 vs Epic 96 实现边界（NFR68 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR129 / Epic 69** | 树内 Handshake 标记 / 弹性缓冲 | **仍有效**；alone ≠ FR164 |
| **FR137 / Epic 76** | 外部 firtool **编译**门禁 MVP | **仍有效；不得改写为失败**；**alone ≠ FR164**（FR137 明确 **未选** sim gate） |
| **Epic 87–95** | 闸门 / … / FR163 | **已关闭；本 epic 不做** |
| **Epic 96 / FR164** | **更广 CIRCT/MLIR / 仿真门禁**（本记录选定：外部 **仿真**门禁） | **本实现 epic** |
| **Epic 97–98** | 其余 NFR59 + 宣称 | **本 epic 不做** |

**选定：关闭 FR137「仅 compile、sim deferred」缺口；≠ 重做 FR137 compile；≠ 本批交付完整 CIRCT MLIR lower 全家桶。**

### FR164 候选子集与本批选定（NFR71）

| 候选（Epic / README 明示） | 本 epic |
| --- | --- |
| **外部 CIRCT / firtool 仿真门禁**（FR137 deferred） | **选定 = FR164 MVP** |
| 更广 CIRCT/MLIR dialect allocation / multi-lower 全家桶 | **不做**（须新合同 / NFR71） |
| 升钉 firtool 超 AD-9 配对版本 | **不做**（须 AD-9 / Stack 修订 + NFR12） |

### 选定子集钉死（96.2 必须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **门禁形状** | 外部 firtool（或文档钉死的 firtool 驱动仿真路径）对代表性 `.fir`：**仿真/执行谓词 Pass**；`just …` / `scripts/…` 可复现；CI **required** job **或** 强制跑通的文档路径（无 `continue-on-error`） | 仅再跑一遍 FR137 compile 冒充加深 |
| **工具版本** | **firtool-1.155.0**（AD-9；与 FR137 同 pin）；`RHDL_FIRTOOL_PATH` / CLI cache；**禁止**裸 PATH / CIRCT HEAD | 升钉 1.156+ 无 AD-9 修订 |
| **失败语义** | 缺工具 / 版本不符 / sim fail → **非零可读**；`BITLOOM_CIRCT_SIM_FORCE_MISSING`（或等价）强制失败 | silent skip / continue-on-error |
| **验收谓词** | (1) docs `fr164-*` 声明 ≠ FR137 compile alone；(2) sim 路径 ATDD/CI 可复现；(3) FR137/FR129 回归不破 | docs-only |
| **AD-25 / NFR70** | **本批不修订 AD-25**（仿真门禁不改变 Handshake 默认可综合合同）；若 96.2 改选触及 Handshake 方言合同须先修订脊柱再标 ready | 静默改 AD-25 |

### 目标产品形状（96.2）

- **入口：** `just circt-external-sim-check`（或文档钉死等价名）+ CI job / 强制步骤。
- **产物：** 仿真门禁脚本 + `docs/fr164-*`；ATDD。
- **语义锚：** 「外部 CIRCT 仿真门禁」须引 **FR164**，不得只引 FR137。

### 验收谓词 / 失败语义（96.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 钉死 firtool 下仿真/执行谓词 Pass；文档对照 FR137；ATDD 可复现 |
| **负向 / 失败** | force-missing / 缺工具 / sim fail → **可读非零**；**不得** silent-Ok |
| **禁止勾选** | 仅 FR137 compile；仅 FR129；仅 docs；continue-on-error |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **96.1** | 本 NFR14 |
| **96.2** | 仿真门禁实现 + ATDD/CI；`docs/fr164-*` |
| **96.3** | README/deferred/spine 收口；勾选 Epic 96；写明未选更广 MLIR lower 仍须新合同 |

### (a) 上游约束

- **Epic 87 已关闭；Epic 88–95 已关闭。**
- **FR137：** 编译门禁 **仍有效**；alone ≠ FR164；其「sim deferred」由本 epic 承接。
- **FR129 / FR121：** 仍有效；alone ≠ FR164。
- **AD-9：** 保持 firtool-1.155.0；升钉须另开 Stack/AD-9。
- **NFR68：** 不得改写 FR94–163 / FR137「已关闭」。
- **NFR70：** 本批 **不**修订 AD-25；若改选触及 Handshake 合同须先修订脊柱。
- **NFR71：** 禁止静默扩大到完整 CIRCT/MLIR allocation 全家桶。
- **NFR72：** 未关 FR164 前不得宣称仿真门禁加深已交付；**不得以 FR137 alone 冒充 FR164**。
- **品牌：** **Bitloom**；设计 crate → **`bitloom-prelude`**（AD-6）；CIRCT 运行时 **不得**进入设计 crate。
- **软序：** 96.2 → 96.3。

### (b) 粗工期带

- **预计：** Epic 96 整体约 **0.75–2.5 人周**（96.1 ≤0.25；96.2 脚本+CI+ATDD 0.5–2；96.3 收口 0.25）。
- **置信度 / 假设：** 中（复用 FR137 firtool pin / CI 体例；仿真驱动形态在 96.2 钉死为 firtool 自带或文档等价）。假设不升 AD-9 版本。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **96.2–96.3** 标 `ready` 或开工实现。
- **不得以 FR137 编译门禁 alone 勾选 FR164。**
- **不得以 FR129 / FR121 alone 冒充 FR164。**
- **不得仅改文档关闭 FR164。**
- **不得 continue-on-error / silent skip。**
- **不得静默扩大到完整 CIRCT/MLIR lower 全家桶。**
- **不得未修订 AD-9 即升钉 firtool。**
- **不得未履行 NFR70 即静默改 AD-25。**
- **不得改写 Phase 12–18 / FR137 / FR129 关闭证据为失败。**
- 不得把本记录冒充 **NFR14-crates**（FCFS）完成定义。
- 不得在 FR164 未关时宣称 NFR59「全清」。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 与 **NFR68 / NFR69 / NFR70 / NFR71 / NFR72** 共同责任人
- 备份 / 升级路径：改选「更广 MLIR lower」代替仿真门禁，或升钉 firtool，须升级至产品 / Correct Course 批准人并修订本记录（及 AD-9/AD-25 若触及）。

---

### Epic 96 关闭条件（Story 96.3 勾选）

- [ ] **FR164 钉死子集实现 + 验收** — Story 96.2
- [ ] **文档 / deferred / README / spine 收口** — Story 96.3
- [ ] **NFR68/70/71/72：** 边界与诚实义务保持；未选更广 MLIR lower / 升钉仍须新合同
- [ ] **品牌 / AD-6：** Bitloom；设计 crate → `bitloom-prelude`
- [ ] **其余 FR165：** 未关前不得宣称 NFR59 全清
