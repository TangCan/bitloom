# NFR14 风险记录 — Epic 90 第三方 LCOV GUI 一等集成（FR158）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 19 **NFR68–NFR72**；实现面 **FR158**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic89-auto-fsm-labels-fr157.md` / `nfr14-risk-epic56-waveform-coverage-gui.md`。  
> **前置：** Epic 87 / FR154 **closed**；Epic 56 / **FR114** LCOV + **树内** `coverage.html` **closed**；Correct Course Phase 19 **approved**（Q1：NFR59→FR158）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 90 后续故事 **90.2–90.3** 标为 `ready`，亦不得开工实现。**不得**在未验收前宣称第三方 LCOV GUI 已交付。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR158 / Epic 90；NFR14、NFR68、NFR69、NFR71、NFR72；对照 FR114 / FR105 / FR154 |
| 记录日期 | 2026-09-12 |
| 状态 | open / in-progress — Story 90.1；Epic 90 未关闭 |
| **选定** | 在保留 FR114 树内 GUI 关闭面的前提下，授权 **MVP：以 `genhtml`（`lcov` 工具包）为一等第三方路径，消费 Bitloom `coverage.lcov` 并生成/打开可浏览 HTML** |

### Phase 12–18 / FR114 关闭面 vs Epic 90 实现边界（NFR68 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR114 / Epic 56** | LCOV 导出 + **Bitloom 树内** `coverage.html` | **仍有效；不得改写为失败**；**alone ≠ FR158** |
| **FR105 / FR109** | 文本覆盖率 / FSM visit | **仍有效**；alone ≠ FR158 |
| **Epic 87–89** | 闸门 / lsp / FSM 标签 | **已关闭；本 epic 不做** |
| **Epic 90 / FR158** | **第三方** LCOV GUI / 浏览路径 | **本实现 epic** |
| **Epic 91–98** | 其余 NFR59 + 宣称 | **本 epic 不做** |

**选定：交付「约定第三方工具打开/导入 LCOV」产品路径；≠ 重做树内 GUI；≠ 仅导出 LCOV 文件 alone。**

### FR158 钉死子集（本 epic · NFR71）

| 维度 | MVP（90.2 必须） | 明确不在本 epic（须新合同 / 其他 FR） |
| --- | --- | --- |
| **第三方 GUI / 工具** | **`genhtml`**（来自 **`lcov`** 工具包；契约：接受标准 LCOV `TN`/`SF`/`DA`/… 方言） | 专有商业 GUI；VS Code 扩展商店多端发布；Tywaves |
| **版本契约** | 文档钉死：**任一**能解析 FR114 `coverage.lcov` 的 `genhtml`（建议 CI/文档写明探测命令 `genhtml --version`）；不强制特定 semver，但须记录实测版本于 90.2 证据 |
| **输入产物** | Bitloom **`coverage.lcov`**（FR114 路径 / `write_coverage_artifacts` / `cargo bitloom coverage`） | 自研非 LCOV 专有格式冒充 |
| **一等集成形状** | 文档化一等配方（CLI 包装 **或** 稳定 README/`docs/fr158-*` 配方 + 可选 `just`/`cargo bitloom` 子命令）；须能从仓库产物走到 **第三方生成的 HTML 目录**（`genhtml` 输出） | 仅口头「请自行打开 genhtml」而无配方/ATDD |
| **验收谓词** | (1) 有 `genhtml`：对夹具 `coverage.lcov` 运行成功并产生第三方 HTML（含 `index.html` 或等价入口）；(2) ATDD 或文档化手动清单可复现 | 「文件存在即算 GUI」 |
| **缺工具失败语义** | **`genhtml` 不在 PATH** → **显式失败**（非零退出 / 可读错误，含安装提示）；**不得**静默成功并宣称 FR158 | 缺工具时假装树内 `coverage.html` 已满足 FR158 |
| **与 FR114 边界** | FR114 = 导出 LCOV + **树内** GUI；FR158 = **第三方** `genhtml` 路径；两者可并存 | 把 FR114 关闭证据改写成「已含第三方 GUI」 |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **90.1** | 本 NFR14 |
| **90.2** | 实现/配方 + ATDD 或手动清单；`docs/fr158-*`；缺工具失败 |
| **90.3** | README/deferred 收口；勾选 Epic 90；明确其余 FR 状态 |

### (a) 上游约束

- **Epic 87 已关闭；Epic 88–89 已关闭**（不挡本 epic）。
- **FR114：** `coverage.lcov` + 树内 `coverage.html` 产品路径已关闭；本 epic **复用** LCOV 产物，**不**重写树内 GUI。
- **NFR68：** 不得改写 FR94–153 / FR114「已关闭」。
- **NFR71：** 禁止超出上表钉死子集静默扩大。
- **NFR72：** 未关 FR158 前不得宣称第三方 LCOV GUI 已交付；**不得以「仅导出 LCOV」或 FR114 树内 HTML alone 冒充 FR158**。
- **品牌：** **Bitloom**；设计 crate → **`bitloom-prelude`**（AD-6）。
- **软序：** 90.2 → 90.3。不得在 90.1 前开 90.2。

### (b) 粗工期带

- **预计：** Epic 90 整体约 **0.5–1.5 人周**（90.1 ≤0.25；90.2 配方/包装 + ATDD 0.25–1；90.3 收口 0.25）。
- **置信度 / 假设：** 中–高（依赖宿主机是否安装 `lcov`/`genhtml`；CI 可 skip-with-explicit-fail 或安装工具）。假设 FR114 LCOV 方言不变。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **90.2–90.3** 标 `ready` 或开工实现。
- **不得以「仅导出 LCOV 文件」alone 冒充 FR158**（除非打开/genhtml 步骤按本记录必验且已验）。
- **不得以 FR114 树内 `coverage.html` alone 冒充 FR158。**
- **不得在缺 `genhtml` 时静默成功宣称第三方路径可用。**
- **不得静默扩大到商业 GUI / IDE 商店多端 / Tywaves。**
- **不得静默扩大 FR142**（若加 CLI 子命令须在 90.2 显式列出）。
- **不得改写 Phase 12–18 / FR114 关闭证据为失败。**
- 不得把本记录冒充 **NFR14-crates**（FCFS）完成定义。
- 不得在 FR158 未关时宣称 NFR59「全清」。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 与 **NFR68 / NFR69 / NFR71 / NFR72** 共同责任人
- 备份 / 升级路径：改选其他第三方 GUI（非 `genhtml`）须升级至产品 / Correct Course 批准人并修订本记录。

---

### Epic 90 关闭条件（Story 90.3 勾选）

- [ ] **FR158 钉死子集实现 + 验收** — Story 90.2
- [ ] **文档 / deferred / README 收口** — Story 90.3
- [ ] **NFR68/71/72：** 边界与诚实义务保持
- [ ] **品牌 / AD-6：** Bitloom；设计 crate → `bitloom-prelude`
- [ ] **其余 FR159–165：** 未关前不得宣称 NFR59 全清
