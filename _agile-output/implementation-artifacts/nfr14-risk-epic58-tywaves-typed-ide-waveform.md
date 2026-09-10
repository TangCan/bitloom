# NFR14 风险记录 — Epic 58 Tywaves 级 typed IDE 波形（FR117）

> **权威：** PRD NFR14；AD-28；Phase 14 **NFR48 / NFR49 / NFR51**；交付 **FR117**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic56-waveform-coverage-gui.md`（FR114 子集 A 当时 deferred → 本 epic 升格）。  
> **前置：** Epic 57 **closed**（FR116）；Epic 47 **closed**（FR104 `interactive.html` I1–I3 + FR105）；Epic 56 **closed**（FR114 LCOV + 树内 `coverage.html`）。  
> **门禁：** 无本有效记录（或缺字段 a–d / 未钉死 A/B 子集）⇒ **不得**将 **58.2–58.3** 标 `ready`。  
> **隔离：** ≠ 仅 FR104 I1–I3；≠ 静态 VCD /「请开 GTKWave」；≠ FR114 LCOV/`coverage.html` alone；≠ docs-only。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR117 / Epic 58；NFR14、NFR48、NFR49、NFR51；对照 FR104、FR105、FR114、FR34 |
| 记录日期 | 2026-09-10 |
| 状态 | **closed** — Story **58.3**（Epic 58 / FR117 收口；子集 B 已交付；A 仍 deferred） |
| **选定加深子集** | **(B) 自研等价 typed IDE 波形** |

### (a) 上游约束

- **Epic 57 / FR116 已关闭：** Phase 14 加深合同闸门已开；本 epic 可合法开工（仍须本 NFR14）。
- **FR104 / Epic 47（已关闭 · 隔离）：** `interactive.html` I1–I3（Browse / Zoom·pan / Search）**仍有效**（NFR48）；**不得**改写为失败，亦不得把 I1–I3 alone 冒充 FR117。
- **FR105：** coverage v2 Mux 分支 **仍有效**；**alone ≠ FR117**。
- **FR114 / Epic 56（已关闭 · 隔离）：** LCOV + 树内 `coverage.html` **仍有效**；**alone ≠ FR117**。Epic 56 风险记录当时将 Tywaves 子集标 deferred（原 NFR47）— 本 epic 承接升格后的 **FR117**。
- **FR34：** toggle 基线为配套；**alone ≠ FR117**。
- **产品形态：** Bitloom 为 Rust eDSL（设计 crate 只依赖 `bitloom-prelude`）；波形/IDE 工具在工具链 / CLI / LSP 邻接，**不**把 Chisel/Tywaves 运行时塞进设计依赖。
- **品牌：** Bitloom / `bitloom-*`。

### (b) 粗工期带

- **预计：** Epic 58 整体约 **1.5–6 人周**（58.1 ≤0.25；58.2 选定子集 1–5；58.3 收口 0.25–0.75）。置信度：**中**。
- **假设：** 未选 (A) Tywaves 一等集成保持 deferred（NFR51）；FR104/114 回归不破；typed 元数据可从现有 dump / HIR 旁路导出而不破坏 AD-5 默认 VCD 路径。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **58.2–58.3** 标 `ready`。
- **不得仅以 FR104 `interactive.html` I1–I3 关闭 FR117。**
- **不得仅以静态 VCD /「请开 GTKWave」/ Surfer alone 关闭 FR117。**
- **不得仅以 FR114 LCOV / `coverage.html` alone 关闭 FR117。**
- **不得仅改文档关闭 FR117**（须有可复现产品路径 + 夹具/步骤 + ATDD 或文档化手动验收清单）。
- **不得 silent 宣称未选子集 (A) 已交付**（NFR51）。
- 不得改写 FR104/105/FR114「已关闭」为失败（NFR48）。
- 不得冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR51** 共同责任人；NFR48/NFR49 共同注意人。
- 备份 / 升级路径：若改选 (A) Tywaves 一等集成或扩大至第三方 IDE 插件商店发布，须修订本记录并获产品确认。

---

### 加深子集（至少钉死一类）

| 子集 | 内容 | 本 epic |
| --- | --- | --- |
| **(A) Tywaves 一等集成** | 将上游 Tywaves（或文档钉死的同族 Chisel/CIRCT typed viewer）作为 Bitloom **一等**打开/集成路径 | **deferred**（未选；须新合同 / 修订本记录） |
| **(B) 自研等价 typed IDE 波形** | 树内（或文档钉死的 Bitloom 品牌）**typed** 源级/结构级 IDE 波形：信号带类型/字段语义，可在 IDE 或等价交互壳中浏览；超出 FR104 I1–I3 纯名字符串 timeline | **选定 = FR117 完成面** |

### 选定子集 (B) — 证明义务 / 夹具 / 工具依赖

| 项 | 钉死 |
| --- | --- |
| **证明义务** | 交付可复现产品路径，使仿真/elaborate 旁路产生的波形带有 **typed** 信息（至少：结构化信号名与类型/字段语义可观察，不仅是扁平 VCD 名）；在 **IDE 或文档等价交互壳**中可浏览；ATDD 或手动验收清单可检查「typed ≠ 仅 I1–I3」 |
| **夹具** | ≥1 可复现 `cargo bitloom` / 库 API / LSP 邻接路径写出 typed 波形工件（名可文档等价，例：`typed-wave.html` / `wave.typed.json` + IDE 打开步骤）；负向：缺 typed 元数据不得 silent 宣称 FR117 绿 |
| **工具 / 版本** | **树内** Bitloom（`just test` 可验）；IDE 路径钉死为 Cursor/VS Code 打开 HTML/JSON 夹具 **或** `bitloom-lsp` 邻接命令（58.2 文档写死具体入口）；**不**要求 CI 安装外部 Tywaves / Chisel；可选 Surfer/GTKWave 仅为补充，不得单独关闭 |
| **超出 MVP** | 明确超出 FR104 I1–I3（无类型语义的 timeline）与 FR114 LCOV GUI — 本面是 **typed IDE 波形**，不是覆盖率 |
| **不回归** | `interactive.html` / VCD / `coverage.lcov`+`coverage.html` 默认路径仍可用（NFR48） |

### 未选子集 (A) — 保持 deferred（NFR51）

- 上游 Tywaves（Chisel/CIRCT debug 方言 viewer）一等集成：**不做**为本 epic 关闭条件。
- 若未来改选 (A)，须新合同或修订本记录，并写清 Tywaves/ChiselSim 钉死版本与失败可读性。

### Epic 58 故事分工（历史；Epic 已关闭）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **58.1** | 本 NFR14 + ATDD；钉死子集 B；禁止项 | **done** |
| **58.2** | 子集 B 产品路径 + 夹具/ATDD 或手动清单 | **done** |
| **58.3** | `docs/fr117-*`（或扩展）/ README / deferred 收口；勾选关闭 | **done**（本收口） |

### Epic 58 关闭条件（Story 58.3 勾选）

- [x] **58.2 / FR117：** 子集 B 可运行 + ATDD/验收清单 + ≥1 夹具；负向可读
- [x] **文档 / deferred / FR104·FR114 交叉链**（未选 A 保持 deferred）
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖：** Bitloom；prelude 边界
- [x] **FR104/114 关闭仍有效**（NFR48）

---

## 门禁一句话

**（历史门禁，Epic 58 已于 Story 58.3 关闭）** 缺 NFR14（或缺 a–d / 未钉死 A/B 子集）曾禁止将 58.2–58.3 标 `ready`。  
**FR117 完成面 = (B) 自研等价 typed IDE 波形；不得以 FR104 I1–I3 / VCD·GTKWave / FR114 LCOV alone / docs-only 关闭；A 未交付不得 silent 宣称。**
