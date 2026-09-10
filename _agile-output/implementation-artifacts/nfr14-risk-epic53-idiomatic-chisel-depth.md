# NFR14 风险记录 — Epic 53 Idiomatic Chisel 可维护深度（FR111）

> **权威：** PRD NFR14；AD-28；Phase 13 **NFR44 / NFR45 / NFR46 / NFR47**；交付 **FR111**。  
> **前置：** Epic 48 **closed**（FR106）；Epic 42 **closed**（FR97 idiomatic MVP；`emit_chisel_idiomatic` + `check_idiomatic_chisel`）；修订后 **AD-27**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **53.2–53.3** 标 `ready`。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR111 / Epic 53；NFR14、NFR44、NFR45、NFR46、NFR47；对照 FR97、FR28/FR46、AD-27 |
| 记录日期 | 2026-09-10 |
| 状态 | **closed** — Story 53.3 / Epic 53（2026-09-10） |

### (a) 上游约束

- **Epic 48 / FR106 已关闭：** Phase 13 加深合同已开闸；本 epic 将 FR97 **MVP（单夹具 idiomatic）** 升格为 **FR111** 可维护深度。
- **FR97 / Epic 42（已关闭 · 隔离）：** idiomatic emit + check **仍有效**（NFR44）；**不得**改写为失败，亦不得把「单夹具 FR97 已关」冒充 FR111 加深全家桶。
- **FR28 / FR46：** 机械可编译 Chisel **仍诚实**；**不得**单独关闭 FR111。
- **AD-27 / NFR46：** 实现须引用现行 AD-27；本 epic 可在风险记录钉死范围内加深 idiomatic 验收（必要时 Story 53.3 交叉修订 AD-27）。
- **Parser：** CIRCT 时代 **不**要求恢复废弃 `Parser.parse`（AD-27 Prevents）；本 epic **不得**把恢复 Parser 写成关闭条件。
- **设计依赖：** 设计 crate 只依赖 `bitloom-prelude`；Chisel 发射在工具链 crate。
- **品牌：** Bitloom / `bitloom-*`。

### (b) 粗工期带

- **预计：** Epic 53 整体约 **1–5 人周**（53.1 ≤0.25；53.2 加深 0.75–4；53.3 收口 0.25–0.75）。置信度：**中**。
- **假设：** 不要求未合同的官方 Chisel 风格全家桶 / Parser 恢复（NFR47）；FR97 回归不破。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **53.2–53.3** 标 `ready`。
- **不得仅文案把机械 `emit_chisel` / FR28 标成 FR111。**
- **不得把 FR97 MVP 单夹具 alone 重标为 FR111 可维护加深完成面。**
- **不得要求无合同恢复废弃 Scala `Parser.parse`。**
- **不得仅改文档关闭 FR111**（须有加深 emit/check 或等价往返 + ATDD + ≥1 加深夹具）。
- 不得改写 FR97「已关闭」为失败（NFR44）。
- 不得静默扩大至未钉死的 idiomatic 全家桶（NFR47）。
- 不得冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR46** 共同责任人；NFR44/45/47 共同注意人。

---

### FR111 加深验收条（钉死 · 供 53.2；至少一类）

本 epic **至少交付下列一类**（可多类；ATDD 须断言所选门）：

| # | 加深验收条 | 验收证据 |
| --- | --- | --- |
| **D1 · 多模块风格一致性** | ≥2 模块（或父子层次）的 idiomatic 发射在命名/分节/结构约定上一致，且可检查 | 加深夹具含多模块；`check_idiomatic_chisel`（或 FR111 加严检查）全绿 |
| **D2 · 命名 / 层次合同** | 文档钉死的端口/实例/层次命名谓词超出 FR97 单模块最小集 | 检查器断言层次路径或实例名合同；负向可读失败 |
| **D3 · 官方风格子集加严** | 相对 FR97 MVP 增加至少一条可自动化加严规则（如强制分节标记集、禁止项、或额外可读性约束） | 新规则有正负向 ATDD；机械 emit 仍须失败该加严面 |

**本记录默认主门（53.2 实现指引）：** 优先 **D1 + D3**（多模块一致性 + 至少一条加严规则）；若取舍，至少保留 **D1 或 D2 或 D3** 之一并在 Story 53.2 文档明示。

**与 MVP 边界：**

| 路径 | 角色 | 可否单独关闭 FR111 |
| --- | --- | --- |
| 机械 `emit_chisel` / FR28 | 可编译诚实面 | **否** |
| FR97 单夹具 idiomatic MVP | Phase 12 MVP | **否** |
| 本记录 D* + ATDD + 加深夹具 | FR111 | **是**（须 53.2→53.3） |

**明确非目标（NFR47）：** 恢复废弃 Parser；未合同的 Chisel 官方风格全家桶；仅文案重标机械 emit；宣称「任意设计 idiomatic 完美」。

### Epic 53 关闭条件（Story 53.3 勾选）

- [x] **53.2 / FR111：** 至少一类 D* + ATDD + ≥1 加深夹具；负向/边界可读
- [x] **文档 / deferred / AD-27 交叉链**（区分 FR97 MVP vs FR111；机械 emit 仍诚实）
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖：** Bitloom；prelude 边界
- [x] **FR97 MVP 关闭仍有效**（NFR44）

---

## 门禁一句话

**缺 NFR14（或缺 a–d）⇒ 不得将 53.2–53.3 标 `ready`。**  
**不得以机械 emit / FR97 单夹具 alone 冒充 FR111；不得要求恢复废弃 Parser；不得仅改文档关闭。**  
**Epic 53 / FR111 已关闭（Story 53.3）：** D1+D3 可维护加深完成面；FR97 MVP 关闭仍有效；未列入风格全家桶仍须新合同（NFR47）。
