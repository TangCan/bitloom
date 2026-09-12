# NFR14 风险记录 — Epic 89 自动 FSM 标签提取（FR157）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 19 **NFR68–NFR72**；实现面 **FR157**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic88-bitloom-lsp-fr152a-fr155.md` / `nfr14-risk-epic51-fsm-state-visit-coverage.md`。  
> **前置：** Epic 87 / FR154 **closed**；Epic 51 / **FR109** C3 FSM state-visit **closed**（显式 `register_fsm_states`）；Correct Course Phase 19 **approved**（Q1：NFR59→FR157）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 89 后续故事 **89.2–89.3** 标为 `ready`，亦不得开工实现。**不得**在未验收前宣称自动 FSM 标签已交付。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR157 / Epic 89；NFR14、NFR68、NFR69、NFR71、NFR72；对照 FR109 / FR105 / FR114 / FR154 |
| 记录日期 | 2026-09-12 |
| 状态 | closed — Story 89.3 勾选完成；Epic 89 关闭；FR157 实现面可宣称；**FR158–165 / FR156** 仍属 Epic 90–98；**不得**宣称 NFR59「全清」 |
| **选定** | 在保留 Phase 12–18 与 FR109 关闭面的前提下，授权 **MVP：从设计源上的显式 FSM 标注自动提取状态标签集**，并可接入既有 `register_fsm_states` / FR109 覆盖率路径 |

### Phase 12–18 / FR109 关闭面 vs Epic 89 实现边界（NFR68 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR109 / Epic 51** | 显式 `register_fsm_states` + state-visit 报告 | **仍有效；不得改写为失败**；本 epic **不**重做 visit 记录器 |
| **FR105 / FR114** | Mux / LCOV / 树内覆盖率 GUI | **仍有效**；本 epic **不**交付第三方 LCOV GUI（→ FR158） |
| **Epic 87 / FR154** | Phase 19 合同闸门 | **已关闭** |
| **Epic 88 / FR155** | lsp crates.io | **已关闭；本 epic 不做** |
| **Epic 89 / FR157** | 自动 FSM **标签集**提取 | **本实现 epic** |
| **Epic 90–98** | 其余 NFR59 + 宣称 | **本 epic 不做** |

**选定：交付「自动得到标签集」产品路径；≠ 全自动从任意 RTL 推断 FSM；≠ 波形逆向；≠ IDE/LCOV 产品面。**

### FR157 钉死子集（本 epic · NFR71）

| 维度 | MVP（89.2 必须） | 明确不在本 epic（须新合同 / 其他 FR） |
| --- | --- | --- |
| **输入面** | 设计 crate **Rust 源**：带约定标注的 FSM `enum`（推荐属性/文档约定，如 `#[bitloom::fsm]` 或等价稳定约定，在 89.2 文档钉死语法） | 任意无标注 HDL/Verilog 反推；仅波形/VCD；仅 coverage 报告回灌；ML |
| **提取对象** | **状态标签集**（字符串标签列表，可命名 FSM id） | 转移边完备性证明；时序等价；合成网表 FSM 识别 |
| **输出形态** | (1) 可机读标签列表（文档化格式：JSON 或稳定文本行）；(2) 可调用 API/库路径把标签交给既有 `register_fsm_states`（或等价注册） | 新独立覆盖率方言替换 FR109 v3；树内/第三方 GUI |
| **验收谓词** | ≥1 夹具：标注 enum → 提取结果 **等于** 期望标签集；可喂入 FR109 注册路径（集成或文档化一步调用） | 「看起来像 FSM」的启发式召回率 KPI |
| **失败语义** | 缺标注 / 空变体 / 非法标注 → **显式失败**（错误码或 Result/诊断），不得静默空成功冒充已提取 | 部分成功后静默丢标签 |
| **与 FR109 边界** | FR109 = visit hit/miss；FR157 = **标签集从何而来**（自动 vs 手写字符串） | 把 FR109 关闭证据改写成「已含自动提取」 |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **89.1** | 本 NFR14 |
| **89.2** | 实现钉死子集 + ATDD/夹具；产品文档 `docs/fr157-*` |
| **89.3** | README/deferred 收口；勾选 Epic 89；明确 FR158–165 未关 |

### (a) 上游约束

- **Epic 87 已关闭；Epic 88 已关闭**（FR155 不挡本 epic，亦不替代本 epic）。
- **FR109：** `bitloom-sim` 已提供 `register_fsm_states` / `sample_state_visit` / coverage v3；本 epic 须 **复用** 而非分叉第二套 visit 语义。
- **NFR68：** 不得改写 FR94–153 / FR109「已关闭」。
- **NFR71：** 禁止超出上表钉死子集静默扩大（尤其：无标注全自动推断、波形逆向、GUI）。
- **NFR72：** 未关 FR157 前不得宣称「自动 FSM 标签」已交付；不得用 FR109 冒充 FR157。
- **品牌：** **Bitloom**；设计 crate 仍只依赖 **`bitloom-prelude`**（AD-6）；工具链 crate 可扩展，但不得把提取依赖泄漏进用户设计包的默认路径而不文档化。
- **软序：** 89.2 实现+ATDD → 89.3 收口。不得在 89.1 前开 89.2。

### (b) 粗工期带

- **预计：** Epic 89 整体约 **0.75–2 人周**（89.1 ≤0.25；89.2 提取器+夹具 0.5–1.5；89.3 收口 0.25）。
- **置信度 / 假设：** 中（标注语法与 syn/HIR 接入点需在 89.2 选定）。假设不改写 FR109 报告方言；假设不提前开 FR158 GUI。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **89.2–89.3** 标 `ready` 或开工实现。
- **不得仅文档宣称而无可运行证据**（89.2）。
- **不得把 FR109 显式注册冒充 FR157 自动提取。**
- **不得静默扩大到无标注全自动 FSM 推断 / 波形逆向 / 第三方 LCOV GUI。**
- **不得静默扩大 FR142** CLI 表面（若加 CLI 子命令须在 89.2 NFR14/文档显式列出，否则只走库 API）。
- **不得改写 Phase 12–18 / FR109 关闭证据为失败。**
- 不得把本记录冒充 **NFR14-crates**（FCFS）完成定义。
- 不得在 FR157 未关时宣称 NFR59「全清」。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 与 **NFR68 / NFR69 / NFR71 / NFR72** 共同责任人
- 备份 / 升级路径：缩回「仅文档约定、无提取器」或扩大到无标注推断须升级至产品 / Correct Course 批准人。

---

### Epic 89 关闭条件（Story 89.3 勾选）

- [x] **FR157 钉死子集实现 + ATDD** — Story 89.2
- [x] **文档 / deferred / README 收口** — Story 89.3
- [x] **NFR68/71/72：** 边界与诚实义务保持
- [x] **品牌 / AD-6：** Bitloom；设计 crate → `bitloom-prelude`
- [x] **其余 FR158–165：** 未关前不得宣称 NFR59 全清
