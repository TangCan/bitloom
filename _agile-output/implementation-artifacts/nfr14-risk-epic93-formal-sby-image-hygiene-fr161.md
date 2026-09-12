# NFR14 风险记录 — Epic 93 formal-sby 镜像卫生跟踪（FR161）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 19 **NFR68–NFR72**；实现面 **FR161**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic67-forced-sby-ci.md` / `nfr14-risk-epic92-non-cargo-path-scan-fr160.md`。  
> **前置：** Epic 87 / FR154 **closed**；Epic 67 / **FR127** 默认 CI 强制真 `sby` **closed**；Epic 60 / **FR119** 本机 `just formal-sby-check` **closed**；Epic 92 / FR160 **closed**。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 93 后续故事 **93.2–93.3** 标为 `ready`，亦不得开工实现。**不得**以 FR127 / FR119 alone 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR161 / Epic 93；NFR14、NFR68、NFR69、NFR70、NFR71、NFR72；对照 FR127 / FR119 |
| 记录日期 | 2026-09-12 |
| 状态 | open / in-progress — Story 93.1；Epic 93 未关闭 |
| **选定** | 在保留 FR127（required `formal-sby` job）与 FR119（本机可选路径）关闭面的前提下，授权 **MVP：为 GHA `formal-sby` 安装面钉死可复现的来源/标签（或 commit）策略 + 可验证卫生跟踪（文档/脚本/CI 断言），使过期/缺钉死/缺工具行为可读失败** |

### Phase 12–18 / FR127 关闭面 vs Epic 93 实现边界（NFR68 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR119 / Epic 60** | 本机可选 `just formal-sby-check` / 文档路径 | **仍有效；不得改写为失败**；**alone ≠ FR161** |
| **FR127 / Epic 67** | 默认 CI **required** `formal-sby` job（安装 + 跑检查；无 continue-on-error） | **仍有效；不得改写为失败**；**alone ≠ FR161**（安装现为 apt + `git clone --depth 1` HEAD，**无**钉死卫生跟踪） |
| **Epic 87–92** | 闸门 / lsp / FSM / genhtml / MemRead / 非 Cargo 扫描 | **已关闭；本 epic 不做** |
| **Epic 93 / FR161** | **镜像/工具链卫生跟踪**（版本钉死 + 跟踪面 + 卫生谓词） | **本实现 epic** |
| **Epic 94–98** | 其余 NFR59 + 宣称 | **本 epic 不做** |

**选定：关闭「CI 真 sby 已强制但安装漂移无记录」缺口；≠ 重做 FR127 required job；≠ 新建平行 formal 证明义务。**

### FR161 钉死子集（本 epic · NFR71）

| 维度 | MVP（93.2 必须） | 明确不在本 epic（须新合同） |
| --- | --- | --- |
| **镜像 / 来源策略** | 文档钉死 **一类**可复现策略（允许组合）：(A) **容器镜像** tag/digest（若 93.2 引入）；或 (B) 保持宿主机安装但 **钉死** `sby` git **tag/SHA** + 记录 `yosys`/`z3` 期望探测命令/版本约束 | 自建私有 registry 全家桶；任意商业 SaaS formal 云 |
| **更新跟踪面** | ≥1 权威钉死文件（例：`scripts/ci-sby-pins.env` / `docs/fr161-*` 表 / workflow `env:`）+ 安装脚本或 CI step **读取并遵守**该钉死 | 仅口头「请关注上游」 |
| **卫生检查谓词** | ATDD 或可重复脚本断言：(1) 钉死文件存在且含非空 pin；(2) 安装路径/探测输出与 pin **一致或可解析**；(3) 故意错误 pin / 缺工具 → **非零可读** | 「job 绿了就算卫生」而无 pin |
| **缺镜像 / 过期失败语义** | 缺钉死工具 / pin 不匹配（若启用严格模式）→ **非零** + 可读（含 FR161 / 安装提示）；**不得** silent-Ok 宣称卫生绿 | 缺 sby 时 skip/`continue-on-error` |
| **与 FR127 边界** | FR127 = required job + 真跑 `formal-sby-check`；FR161 = **版本/卫生可跟踪**；FR127 job **不得**被改写为失败；可在同 job 内加深 install | 把 FR127 关闭证据改写成「已含镜像卫生跟踪」 |
| **验收谓词** | (1) pin 文件/文档 + 脚本遵守；(2) ATDD/检查可复现；(3) FR127 / FR119 回归不破 | 仅改 FR127 文档声称已卫生 |

**明确不在本 epic：** 更换 solver 全家桶；扩展 formal 证明义务超出既有 FR119 夹具；默认 CI 超时策略大改（除非卫生检查必需）。

### 目标产品形状（93.2）

- **入口：** 既有 `.github/workflows/ci.yml` → `formal-sby` + `scripts/ci-install-sby.sh`（加深，不平行拆 job 除非必要）。
- **产物：** 钉死来源文件 + `docs/fr161-*` + 卫生断言（脚本和/或 `cargo test`）。
- **语义锚：** 可复现安装；漂移可发现；缺工具可读失败（对齐 FR127 诚实）。

### 验收谓词 / 失败语义（93.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | pin 存在且安装脚本/CI 遵守；卫生检查通过；ATDD 可复现 |
| **负向 / 失败** | 缺 pin / 缺 sby（模拟）→ **可读非零**；**不得** silent-Ok |
| **禁止勾选** | 仅 FR127 job 存在；仅 FR119 本机路径；仅 docs |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **93.1** | 本 NFR14 |
| **93.2** | pin + 安装遵守 + 卫生断言 + ATDD；`docs/fr161-*` |
| **93.3** | README/deferred 收口；勾选 Epic 93 |

### (a) 上游约束

- **Epic 87 已关闭；Epic 88–92 已关闭。**
- **FR127：** required `formal-sby` **仍有效**；本 epic 升格 **卫生/钉死** 缺口，不得把 FR127 改写成「已含 pin 跟踪」。
- **FR119：** 本机路径仍有效；alone ≠ FR161。
- **NFR68：** 不得改写 FR94–160 / FR127「已关闭」。
- **NFR70：** 若触 CI 运维形状，保持与 AD/脊柱诚实一致（不另立 HIR）。
- **NFR71：** 禁止超出上表钉死子集静默扩大。
- **NFR72：** 未关 FR161 前不得宣称 formal-sby 镜像卫生已交付；**不得以 FR127 alone 冒充 FR161**。
- **品牌：** **Bitloom**；设计 crate → **`bitloom-prelude`**（AD-6）。
- **软序：** 93.2 → 93.3。

### (b) 粗工期带

- **预计：** Epic 93 整体约 **0.5–1.5 人周**（93.1 ≤0.25；93.2 pin+脚本+ATDD 0.25–1；93.3 收口 0.25）。
- **置信度 / 假设：** 中–高（依赖上游 sby/yosys 可 pin；CI 时长仍受 FR127 ≤20m 约束）。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **93.2–93.3** 标 `ready` 或开工实现。
- **不得以 FR127 required job alone 勾选 FR161。**
- **不得以 FR119 本机 `just formal-sby-check` alone 冒充 FR161。**
- **不得仅改文档关闭 FR161。**
- **不得对缺 pin / 缺 sby silent-Ok 或 continue-on-error。**
- **不得静默扩大到商业 formal SaaS / 私有 registry 全家桶。**
- **不得改写 Phase 12–18 / FR127 / FR119 关闭证据为失败。**
- 不得把本记录冒充 **NFR14-crates**（FCFS）完成定义。
- 不得在 FR161 未关时宣称 NFR59「全清」。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 与 **NFR68 / NFR69 / NFR70 / NFR71 / NFR72** 共同责任人
- 备份 / 升级路径：改为「仅文档建议盯上游」而无 pin/断言，或引入未钉死的浮动 `latest` 镜像，须升级至产品 / Correct Course 批准人。

---

### Epic 93 关闭条件（Story 93.3 勾选）

- [ ] **FR161 钉死子集实现 + 验收** — Story 93.2
- [ ] **文档 / deferred / README 收口** — Story 93.3
- [ ] **NFR68/70/71/72：** 边界与诚实义务保持
- [ ] **品牌 / AD-6：** Bitloom；设计 crate → `bitloom-prelude`
- [ ] **其余 FR162–165：** 未关前不得宣称 NFR59 全清
