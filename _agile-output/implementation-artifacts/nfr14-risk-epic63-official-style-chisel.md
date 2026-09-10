# NFR14 风险记录 — Epic 63 官方风格 Chisel 全家桶（FR122）

> **权威：** PRD NFR14；AD-28；Phase 14 **NFR48 / NFR49 / NFR50 / NFR51**；交付 **FR122**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic53-idiomatic-chisel-depth.md`；`nfr14-risk-epic42-idiomatic-chisel.md`；`nfr14-risk-epic57-phase14-nfr47-deferred-deepen.md`。  
> **前置：** Epic 57 **closed**（FR116）；Epic 42 **closed**（FR97）；Epic 53 **closed**（FR111 D1+D3）；修订后 **AD-27**。  
> **门禁：** 无本有效记录（或缺字段 a–d / 下方 O1–O4 清单）⇒ **不得**将 **63.2–63.3** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR122 / Epic 63；NFR14、NFR48、NFR49、NFR50、NFR51；对照 FR97、FR111、FR28/FR46、AD-27 |
| 记录日期 | 2026-09-10 |
| 状态 | **closed** — Story 63.3 / Epic 63（2026-09-10） |

### (a) 上游约束

- **Epic 57 / FR116 已关闭：** Phase 14 加深合同已开闸；本 epic 将 NFR47「官方风格全家桶」升格为 **FR122** 必选。
- **FR97 / Epic 42（已关闭 · 隔离）：** idiomatic MVP emit/check **仍有效**（NFR48）；**不得**改写为失败，亦不得以 FR97 alone 冒充 FR122。
- **FR111 / Epic 53（已关闭 · 隔离）：** D1+D3 可维护加深 **仍有效**；**不得**以 FR111 alone / D1+D3 alone 冒充官方风格全家桶。
- **FR28 / FR46：** 机械可编译 Chisel **仍诚实**；**不得**单独关闭 FR122。
- **AD-27（现行 → 视需修订）：** 现行允许 FR97/FR111；**未**把官方风格全家桶写成完成面。Story **63.2 须视需要修订 ARCHITECTURE-SPINE AD-27**，允许 FR122 官方/idiomatic 风格全家桶为合法产品路径（NFR50）；**默认仍不恢复** Scala `Parser.parse`（本记录**不**另开 Parser 口；**prefer NOT restoring Parser**）。
- **设计依赖：** 设计 crate 只依赖 `bitloom-prelude`；Chisel 发射在工具链 crate（`rhdl-firrtl` / `bitloom`）。
- **品牌：** Bitloom / `bitloom-*`。

### (b) 粗工期带

- **预计：** Epic 63 整体约 **1–4 人周**（63.1 ≤0.25；63.2 AD 修订 + 全家桶路径 0.75–3；63.3 收口 0.25–0.75）。置信度：**中**。
- **假设：** 不恢复 Parser；不宣称「任意设计完美符合 Chisel Style Guide 全文」；FR97/FR111 回归不破。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **63.2–63.3** 标 `ready`。
- **不得仅以 FR97 MVP alone 关闭 FR122。**
- **不得仅以 FR111 D1+D3 alone 关闭 FR122。**
- **不得仅以机械 `emit_chisel` / FR28 关闭 FR122。**
- **不得 docs-only / 仅改文档关闭。**
- **默认禁止恢复废弃 Scala `Parser.parse`**（若将来另选须显式写明并修订 AD-27；**本记录不开该口**）。
- 不得改写 FR97 / FR111「已关闭」为失败（NFR48）。
- 不得静默扩大超出下方 O1–O4 清单（NFR51）。
- 不得冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR50** / **NFR51** 共同责任人；NFR48/NFR49 共同注意人。
- 备份 / 升级路径：缩回「仅 FR111 D1+D3」口径或扩大至完整 Chisel Style Guide 全文 / 恢复 Parser 须升级至产品 / Correct Course 批准人。

---

### 官方风格全家桶清单（供 63.2 / FR122 · O1–O4）

相对 FR111 **D1+D3** 的**规则集增量**（须全部交付；ATDD 断言）：

| # | 钉死项 | 合同内容 | 63.2 验收证据 |
| --- | --- | --- | --- |
| **O1 · package + FR122 宣称** | 文件级官方包与完成面标记 | 产出含文档钉死的 `package bitloom.generated`（或风险记录/文档等价包名）且连续宣称 `FR122 official-style`（或 `FR122 official`）；**不得**仅依赖 FR97/FR111 头冒充 | 正例含 package + FR122；负向：剥 package/宣称 → `check_*` Err |
| **O2 · 官方分节顺序** | Style-guide 子集：分节顺序 | 每模块 class 内，出现的分节标记须按官方顺序：`--- io ---` → `--- registers ---` → `--- wires ---` → `--- instances ---` → `--- memories ---` → `--- logic ---`（仅对实际出现的节检查相对顺序） | 正例顺序正确；负向：打乱顺序 → Err |
| **O3 · 每模块 FR122 标记** | 超出 FR111 per-module | 每个模块 class 含 `// --- FR122 official ---`（保留 FR111 `// --- FR111 per-module ---` 作为子集） | 剥任一模块 FR122 标记 → Err；FR111 alone 缺此标记 → Err |
| **O4 · API + 多模块失败语义** | emit/check 形状与隔离 | **API 形状：** `emit_chisel_idiomatic_fr122(hir) -> Result<Artifact, ChiselGenError>`；`check_idiomatic_chisel_fr122(scala, hir) -> Result<(), IdiomaticCheckError>`（E0904）。须先通过 FR111 子集（NFR48）。≥2 模块夹具。机械 / FR97 alone / FR111 alone **不得** silent Ok。公开品牌 Bitloom。**不**恢复 Parser | `cargo test -p bitloom --test fr122_official_style_chisel`（或等价）纳入 `just test` |

**与既有路径边界：**

| 路径 | 角色 | 可否单独关闭 FR122 |
| --- | --- | --- |
| 机械 `emit_chisel` / FR28 | 可编译诚实面 | **否** |
| FR97 idiomatic MVP | Phase 12 MVP | **否** |
| FR111 D1+D3 deepen | Phase 13 加深 | **否** |
| docs-only | 文档 | **否** |
| 本记录 O1–O4 +（视需）修订 AD-27 + ATDD | FR122 | **是**（须 63.2→63.3） |

### 明确非目标 / deferred（NFR51 · 诚实披露）

下列 **未**列入本 FR 关闭清单；**不得 silent 宣称已交付：**

- 恢复废弃 Scala `Parser.parse` / `firrtl.Parser`（**本记录明确不开**）
- 完整 Chisel Style Guide / community linter 全文自动合规
- 任意手写 Scala 的 idiomatic「完美」证明
- 将 FR122 强制替换为唯一合法发射路径（FR97/FR111/机械须保留可回归）

### Epic 63 关闭条件（Story 63.3 勾选）

- [x] **63.2 / FR122：** O1–O4 +（视需）修订 AD-27 + ATDD；FR97/FR111 回归不破
- [x] **文档 / deferred / README / AD-27 修订戳**
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖：** Bitloom；prelude 边界
- [x] **FR97 / FR111 关闭仍有效**（NFR48）

---

## 门禁一句话

**缺 NFR14（或缺 a–d / O1–O4）⇒ 不得将 63.2–63.3 标 `ready`。**  
**不得以 FR97 alone / FR111 alone / 机械 emit / docs-only 冒充官方风格全家桶；默认不恢复 Parser。**  
**未列入协议保持 deferred（NFR51）。**  
**Epic 63 / FR122 已关闭（Story 63.3）：** 官方风格全家桶（O1–O4；AD-27 修订）；FR97/FR111 关闭仍有效；完整 Style Guide / Parser 恢复仍 deferred。
