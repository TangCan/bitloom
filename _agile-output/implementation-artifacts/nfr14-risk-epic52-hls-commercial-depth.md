# NFR14 风险记录 — Epic 52 树内 HLS 商业深度（FR110）

> **权威：** PRD NFR14；AD-28；Phase 13 **NFR44 / NFR45 / NFR46 / NFR47**；交付 **FR110**。  
> **前置：** Epic 48 **closed**（FR106）；Epic 41 **closed**（FR95/FR96 树内 MVP；`in-tree-mvp` / loop-unroll / dissolve）；修订后 **AD-25**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **52.2–52.3** 标 `ready`。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR110 / Epic 52；NFR14、NFR44、NFR45、NFR46、NFR47；对照 FR95/FR96、FR35、AD-25 |
| 记录日期 | 2026-09-10 |
| 状态 | draft — Story 52.1 |

### (a) 上游约束

- **Epic 48 / FR106 已关闭：** Phase 13 加深合同已开闸；本 epic 将 FR95/96 **MVP** 升格为 **FR110** 可验收商业深度（仍非「无限商业编译器」口号）。
- **FR95 / FR96 / Epic 41（已关闭 · 隔离）：** 树内 loop-unroll MVP + 闭包 dissolve + `in-tree-mvp` RTL stub **仍有效**（NFR44）；**不得**改写为失败，亦不得把「MVP / stub / 仅展开」冒充 FR110 商业深度完成面。
- **AD-25 / NFR46：** 实现须引用现行 AD-25；本 epic 可在风险记录钉死范围内加深树内调度质量门（必要时 Story 52.3 交叉修订 AD-25 文字）。
- **外挂 Bambu（FR35）：** stub / `BITLOOM_HLS_USE_REAL` **不得单独**关闭 FR110。
- **设计依赖：** 设计 crate 只依赖 `bitloom-prelude`；HLS 调度在工具链 crate。
- **品牌：** Bitloom / `bitloom-*`。

### (b) 粗工期带

- **预计：** Epic 52 整体约 **2–8 人周**（52.1 ≤0.25；52.2 加深实现 1.5–6；52.3 收口 0.25–1）。置信度：**低–中**。
- **假设：** 不要求完整商业 HLS 全家桶（allocation/binding/全优化套件未列入须新合同 / NFR47）；FR95/96 回归不破。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **52.2–52.3** 标 `ready`。
- **不得把 FR95/96 MVP stub / 仅 loop-unroll / `in-tree-mvp` 重标为商业完整 HLS / FR110。**
- **不得以外挂 Bambu stub（或仅真机绿灯而无本记录质量门）单独关闭 FR110。**
- **不得仅改文档关闭 FR110**（须有可运行加深路径 + ATDD + ≥1 夹具满足质量门）。
- 不得改写 FR95/96「已关闭」为失败（NFR44）。
- 不得静默扩大至未钉死的动态数据流 / Handshake 默认可综合语义（NFR47）。
- 不得冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR46** 共同责任人；NFR44/45/47 共同注意人。

---

### FR110 可验收质量门（钉死 · 供 52.2；至少一类）

本 epic **至少交付下列一类**（可多类；ATDD 须断言所选门）：

| # | 质量门 | 验收证据（文档化 + 自动化） |
| --- | --- | --- |
| **Q1 · 流水阶段** | 树内 schedule 产出 **pipeline stage 数 ≥ 2**（超出「纯展开扁平」MVP 叙述） | schedule IR / 报告含可解析 `pipeline_stages`（或等价）且 ATDD ≥ 2 |
| **Q2 · II / 资源约束** | 显式 **initiation interval（II）** 或文档钉死的资源/II 约束进入 schedule 并被检查 | schedule IR 含 `ii`（或等价）字段；未达约束时失败可读或明确降级（不得 silent 宣称） |
| **Q3 · 对照基线调度质量** | 相对 FR95 MVP 基线的可检查质量度量（如阶段数、II、或文档钉死的对比指标） | ATDD 对比 MVP 夹具 vs FR110 夹具；报告含 **`fr110`**（或等价）族标识 |

**本记录默认主门（52.2 实现指引）：** 优先 **Q1 + Q2**（`pipeline_stages ≥ 2` 且报告/IR 含 `ii`）；若实现取舍，至少保留 **Q1 或 Q2 或 Q3** 之一并在 Story 52.2 文档明示。

**与 MVP 边界：**

| 路径 | 角色 | 可否单独关闭 FR110 |
| --- | --- | --- |
| FR95 loop-unroll + `in-tree-mvp` stub | Phase 12 MVP | **否** |
| FR96 dissolve → FR95 | Phase 12 MVP | **否** |
| 外挂 Bambu stub / 真机 alone | FR35 诚实路径 | **否** |
| 本记录 Q* + ATDD + 夹具 | FR110 | **是**（须 52.2→52.3） |

**明确非目标（NFR47）：** 商业级完整 HLS 编译器口号；未合同的 Handshake/动态数据流默认语义；仅文档重标 MVP；用 Bambu stub 冒充树内商业深度。

### Epic 52 关闭条件（Story 52.3 勾选）

- [ ] **52.2 / FR110：** 至少一类 Q* + ATDD + ≥1 夹具；负向/边界可读
- [ ] **文档 / deferred / AD-25 交叉链**（区分 FR95/96 MVP vs FR110）
- [ ] **禁止事项未触发**
- [ ] **品牌 / 依赖：** Bitloom；prelude 边界
- [ ] **FR95/96 MVP 关闭仍有效**（NFR44）

---

## 门禁一句话

**缺 NFR14（或缺 a–d）⇒ 不得将 52.2–52.3 标 `ready`。**  
**不得以 FR95/96 MVP stub / 仅展开 / 外挂 Bambu stub 冒充 FR110 商业深度；不得仅改文档关闭。**
