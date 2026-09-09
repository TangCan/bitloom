# NFR14 风险记录 — Epic 44 按键全 elaborate Bitloom LSP（FR99）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 12 **NFR40 / NFR43**；交付 **FR99**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic43-vip-full-protocol-ip.md`；`nfr14-risk-epic39-ide-multiview.md`；`nfr14-risk-visualization.md`。  
> **前置：** Epic 40 **closed**（FR94；Path B 字面绿）；Epic 39 **closed**（FR90 宿主 rust-analyzer；**FR91 Path B** 显式 defer 自研 LSP）；FR38/FR49 层次/时序 HTML 已有——**≠** LSP。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 44 后续故事 **44.2–44.4** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR99 / Epic 44；NFR14、NFR40、NFR43；推翻 FR91 Path B defer 作为完成口径；对照 FR90 / FR38 / FR49 |
| 记录日期 | 2026-09-09 |
| 状态 | accepted — Story 44.1（门禁开启；关闭条件留给 44.4） |

### (a) 上游约束

- **Epic 40 / FR94 已关闭：** Path B 字面绿合同已开闸；本 epic 实现 FR93#5 → **FR99**（按键全设计 elaborate 的自研 Bitloom netlist / 硬件语义 LSP）。
- **FR91 Path B（Epic 39）：** 已合同化 **显式 defer** 自研 Bitloom LSP。本 epic **推翻**「Path B defer 即完成口径」——FR91 defer **不再**可替代 FR99 关闭。**不得**继续用「仍 deferred」宣称 Phase 12 LSP 字面条已绿。
- **FR90（宿主路径仍有效）：** rust-analyzer 继续为设计 crate 提供 **Rust** 补全 / 跳转 / rustc 诊断；**不**替代 FR99 自研硬件语义 LSP。见 `docs/fr90-host-ide-rust-analyzer.md`。
- **FR38 / FR49：** 层次 / 时序 **HTML** 可视化已交付；**不得**计入 LSP / FR99 完成（HTML ≠ LSP）。见 `docs/fr38-viz-lsp.md`。
- **FR99 完成面（本记录钉死）：** 可安装/可启动的 Bitloom language-server + 至少一种编辑器接线；在**按键（或文档化编辑触发）路径**上执行**全设计 elaborate**，并暴露硬件语义诊断 + 符号/跳转（或记录等价能力集）；超时/过大设计行为按下方性能/范围边界文档化；ATDD/集成测相对浅层/非 elaborate 路径可区分。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）；LSP 属工具链面，不得要求设计 crate 依赖 CLI 包。
- **NFR40：** 全 elaborate LSP 为多年/高维护字面条；不得用半成品二进制、HTML 冒充、或仅浅层诊断假绿交差。

### (b) 粗工期带

- **预计：** Epic 44 整体约 **4–10 人周**（44.1 本风险记录 ≤0.25 人周；44.2 LSP 服务器 MVP + 编辑器接线 1.5–3.5 人周；44.3 按键全 elaborate 诊断/符号 1.5–4 人周；44.4 FR99 收口与撤销非目标话术 0.5–1.5 人周）。置信度：**低–中**（elaborate 延迟与设计规模方差大；编辑器接线面 NFR40）。
- **假设：** 不回滚 FR90 宿主路径；不把 SystemC / 形式等价（Epic 45–46）并入本 epic；不并行冒充 Epic 45–47 关闭。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **44.2–44.4** 标 `ready` 或开工实现。
- **不得交付半成品 language-server 二进制交差**（无 initialize/capabilities、无编辑器接线文档、或无法复现启动会话却宣称 FR99 / 44.2 完成）。
- **不得把 HTML 可视化（层次/时序 FR38/FR49）计入 LSP / FR99 完成。**
- **不得仅浅层诊断关闭 FR99**（无按键全设计 elaborate 合同、或相对浅层路径无可区分证据）。
- 不得把 **宿主 rust-analyzer（FR90）** 冒充自研 Bitloom 硬件语义 LSP / FR99 已交付。
- 不得继续把 **FR91 Path B defer** 当作 FR99 / Phase 12 LSP 字面条的完成口径。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR40**（多年维护 / 字面绿诚实）共同责任人；**NFR43** 共同注意人
- 备份 / 升级路径：放宽/收紧按键全 elaborate 性能边界、改选浅层-only 关闭口径、或放弃自研 LSP 回 Path B 须升级至产品 / Correct Course；与 FR90/FR38 话术冲突升级至 AD-28 维护者。

---

### 「按键全 elaborate」性能 / 范围边界（本记录钉死 · 供 44.2–44.4）

> 未列能力默认**非目标**；静默扩大未列模式禁止。超时/过大设计须**可读诊断或文档化降级行为**，不得静默挂死编辑器。

| # | 边界项 | 钉死值（MVP 合同） |
| --- | --- | --- |
| P1 | **触发** | 按键路径或文档化的编辑触发（保存 / 显式命令 / debounce 后的 change）；须在 44.3 文档写明选用触发面 |
| P2 | **Elaborate 合同** | 触发后对**当前设计根（或文档钉死的 crate/模块集）**跑**全设计 elaborate**（非仅词法/浅层语法扫）；相对浅层路径须可测区分 |
| P3 | **交互预算** | 目标：夹具规模设计 **≤ 2s** 返回首批诊断（冷启动可另计，须文档化）；超过预算须发**进行中/超时**诊断或可取消，不得无限阻塞 UI |
| P4 | **规模上限** | MVP 验收夹具：单 crate / 文档钉死的模块数与端口规模；超出上限时须**明确失败或降级提示**（不得假装已全 elaborate） |
| P5 | **增量** | MVP **不要求**细粒度增量 elaborate；允许全量重算，但须遵守 P3/P4 |
| P6 | **能力最小集** | 至少：**诊断**（硬件语义 / elaborate 错误）+ **符号或跳转**（或记录等价硬件语义能力集） |

### 与宿主 rust-analyzer 分工（钉死）

| 职责 | 宿主 rust-analyzer（FR90） | Bitloom LSP（FR99） |
| --- | --- | --- |
| Rust 补全 / 跳转 / rustc 诊断 | ✅ | ❌ 不替代 |
| 设计 crate 依赖解析（Cargo） | ✅ | 可复用索引提示，但完成定义不依赖「仅 RA」 |
| 全设计 elaborate / 硬件语义诊断 | ❌ | ✅ 必选 |
| Netlist / HIR 级符号导航 | ❌ | ✅（或文档等价能力集） |
| 层次/时序 HTML | ❌（属 FR38/FR49 CLI） | ❌ 不计入本 LSP |

**原则：** 两路**并存**；FR90 仍可用；**不得**声称「装了 rust-analyzer = FR99 完成」。

### FR90 / FR91 Path B / FR38 vs FR99（摘要）

| 路径 | 角色 | 可否单独关闭 FR99 |
| --- | --- | --- |
| FR90 宿主 rust-analyzer | Rust 智能；Wave D 必做 | **否** |
| FR91 Path B defer | Epic 39 显式不交付自研 LSP | **否**（本 epic 推翻其作为完成口径） |
| FR38/FR49 HTML | 层次/时序可视化 | **否**（HTML ≠ LSP） |
| 本记录全 elaborate Bitloom LSP + ATDD | FR99 完成面 | **是**（须 44.2–44.4） |

### Epic 44 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **44.1** | 本 NFR14 风险记录 + ATDD | **本故事** |
| **44.2** | Bitloom LSP 服务器 MVP + 编辑器接线 | Gate：须本记录后才可 ready |
| **44.3** | 按键全设计 elaborate 诊断 / 符号（P1–P6） | Gate：须本记录后才可 ready |
| **44.4** | FR99 收口；撤销 LSP 非目标 / Path B 完成口径 | Gate：须本记录后才可 ready |

### 并行 / 维护叠加（Chipyard 式 · NFR40 / NFR43）

- 可与 Epic 45–47 **并行规划**，但各自须独立 NFR14；本 epic 不得冒充其他字面条关闭。
- `docs/fr38-viz-lsp.md` / FR90 / deferred-work「按键全 elaborate LSP → FR99」须同一叙事；禁止「HTML 或 RA 已有、文档仍写永久非目标却宣称 FR99」。
- 工具链 churn：elaborate API 变更须同步 LSP 诊断路径，避免编辑器假绿。

### 引用

- AD-28 — 风险门禁（NFR14）
- PRD NFR14 / FR99；NFR40 / NFR43；推翻 FR91 Path B 完成口径
- 对照：FR90；FR38 / FR49；`docs/fr38-viz-lsp.md`；`docs/fr90-host-ide-rust-analyzer.md`
- 前置：Epic 40 / FR94；Epic 39 / FR90–FR92（FR91 Path B）
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 44 关闭条件（Story 44.4 勾选）

- [x] **44.2：** 可安装/可启动 language-server + 编辑器接线 + 可复现会话 — Story 44.2
- [x] **44.3：** 按键全 elaborate 诊断 + 符号/跳转（P1–P6）+ ATDD 可区分浅层路径 — Story 44.3
- [ ] **文档 / deferred：** 撤销「LSP 永久非目标 / FR91 Path B defer 作为完成」；FR90 仍可用但不替代 FR99
- [ ] **禁止事项未触发：** 无半成品二进制交差；无 HTML 计入 LSP；无仅浅层诊断关闭 FR99
- [ ] **品牌 / 依赖：** 仍为 Bitloom；设计 crate 只依赖 `bitloom-prelude`

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 44 故事 44.2–44.4 标 `ready`。**  
**FR99 完成面 = 按键全设计 elaborate 的自研 Bitloom LSP（性能/范围见上表）+ 与 rust-analyzer 分工诚实 + ATDD；不得以半成品二进制、HTML 可视化、宿主 RA、或仅浅层诊断单独关闭 FR99。**
