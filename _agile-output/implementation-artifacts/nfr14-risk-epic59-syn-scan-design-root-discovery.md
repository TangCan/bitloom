# NFR14 风险记录 — Epic 59 无 metadata 全树 syn-scan（FR118）

> **权威：** PRD NFR14；AD-28；Phase 14 **NFR48 / NFR49 / NFR51**；交付 **FR118**。  
> **前置：** Epic 57 **closed**（FR116）；Epic 44 **closed**（FR99 DesignFixture MVP）；Epic 55 **closed**（FR113 Cargo-graph+metadata）。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **59.2–59.3** 标 `ready`。  
> **隔离：** ≠ FR99 DesignFixture alone；≠ FR113 metadata `design_roots` alone；≠ FR90 rust-analyzer alone。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR118 / Epic 59；NFR14、NFR48、NFR49、NFR51；对照 FR99、FR113、FR90 |
| 记录日期 | 2026-09-10 |
| 状态 | **accepted** — Story **59.1**；关闭勾选留给 Story **59.3** |
| **选定发现策略** | **Workspace `#[bitloom::top]` syn-scan**（无 metadata 时扫描 `.rs`；与 Cargo-graph+metadata 共存） |

### (a) 上游约束

- **Epic 57 / FR116 已关闭：** Phase 14 加深合同已开闸。
- **FR99 / Epic 44（已关闭 · 隔离）：** `bitloom-lsp` DesignFixture 全 elaborate MVP **仍有效**（NFR48）；**不得**改写为失败，亦不得把「仅 DesignFixture enum」冒充 FR118。
- **FR113 / Epic 55（已关闭 · 隔离）：** Cargo-graph + `[package.metadata.bitloom] design_roots` **仍有效**（NFR48）；**不得**以 metadata alone 冒充无 metadata syn-scan 完成面。
- **FR90：** rust-analyzer alone ≠ FR118。
- **设计依赖：** 设计 crate 只依赖 `bitloom-prelude`；LSP 在工具链 `bitloom-lsp`。
- **品牌：** Bitloom / `bitloom-lsp`。
- **证明义务 / 夹具 / 工具：** 树内 `bitloom-lsp` + ≥1 **无 metadata** 迷你包夹具（源含 `#[bitloom::top]`）+ ATDD；`just test` / `cargo test -p bitloom`；不要求外挂 IDE 商店上架。

### (b) 粗工期带

- **预计：** Epic 59 整体约 **1–4 人周**（59.1 ≤0.25；59.2 syn-scan+夹具 0.75–3；59.3 收口 0.25–0.75）。置信度：**中**。
- **假设：** FR99 / FR113 回归不破；根 elaborate 仍可走 LSP 文档等价注册表（与 FR113 同形），但**发现源**须来自 syn-scan 而非 metadata。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **59.2–59.3** 标 `ready`。
- **不得以 FR99 DesignFixture MVP alone 冒充 FR118。**
- **不得以 FR113 metadata `design_roots` alone 冒充 FR118。**
- **不得仅浅层诊断（`AnalysisMode::Shallow`）伪装 finish 成功关闭 FR118。**
- **不得仅改文档（docs-only）关闭 FR118。**
- 不得以 FR90 rust-analyzer alone 关闭 FR118。
- 不得改写 FR99 / FR113「已关闭」为失败（NFR48）。
- 不得半成品二进制交差（须可复现发现/elaborate 夹具 + ATDD）。
- 不得冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR51** 共同责任人；NFR48/49 共同注意人。

---

### 发现策略（本 epic 钉死）

| 策略 | 内容 | 本 epic |
| --- | --- | --- |
| **Workspace `#[bitloom::top]` syn-scan** | 对依赖 `bitloom-prelude` 且**无** metadata `design_roots` 的包，扫描包内 `.rs` 识别 `#[bitloom::top]`（及过渡期 `#[rhdl::top]`），以标注类型名为根 id 进入 elaborate | **选定 = FR118 完成面** |
| Cargo-graph + metadata `design_roots` | FR113 完成面 | **仍保留**；**alone ≠ FR118**；与 syn-scan **共存** |
| DesignFixture-only enum | FR99 MVP | **仍保留为回归**；**alone ≠ FR118** |

### 选定策略 — 扫描范围 / 识别 / 失败语义 / 共存

| 项 | 钉死 |
| --- | --- |
| **Workspace 扫描范围** | 与 FR113 同形：输入为 Cargo workspace `members` 或单包 `Cargo.toml` 根；仅考虑依赖 `bitloom-prelude` 的成员包；扫描该包 `src/` 下 `.rs`（含一层 `src/**/*.rs` 常规布局）。不要求扫整个 monorepo 非成员路径。 |
| **`#[bitloom::top]` 识别策略** | 源码文本/语法级识别属性路径 `bitloom::top` 或 `rhdl::top`（过渡）紧邻的 `struct` / `enum` / `type` 标识符，取其 **类型名** 为 `root_id`（文档等价入口；可与 LSP 注册表衔接）。不得把未标注类型计入。 |
| **无 metadata 根解析** | 包无 `[package.metadata.bitloom] design_roots`（或缺该节）时走 syn-scan；发现的每个 top → `DiscoveredDesignRoot{package, dir, root_id}`。 |
| **失败语义** | 无 prelude 包 / 无 top / IO 失败 → 可读诊断（沿用或扩展 `bitloom-lsp.no-design-roots` / `discover-failed` / `unknown-design-root`）；elaborate 失败须可读；**不得** `called_finish` 伪装成功。 |
| **与 metadata 共存** | 包**有** metadata `design_roots` → 仍走 FR113 metadata 路径（不因 syn-scan 覆盖或删除）；包**无** metadata → syn-scan。同一 workspace 可同时含两类包。`discover_design_roots`（或等价公开 API）须能返回 syn-scan 根。 |
| **性能 / 范围天花板** | 沿用 FR99/FR113 P3/P4：超时 → `timeout`；过大 → `oversized`；shallow **不得** finish。 |
| **夹具** | ≥1 **无 metadata** 正例（`#[bitloom::top]` → Pass）；≥1 失败可读；FR99 DesignFixture + FR113 metadata 回归不破。 |
| **工具依赖** | 树内 `bitloom-lsp` / `just test` |

### Epic 59 关闭条件（Story 59.3 勾选）

- [ ] **59.2 / FR118：** syn-scan 产品路径可运行 + ATDD + 无 metadata 正例夹具；失败可读；FR99/FR113 回归；shallow ≠ finish
- [ ] **文档 / deferred / README / FR99·FR113 交叉链**
- [ ] **禁止事项未触发**
- [ ] **品牌 / 依赖：** Bitloom；prelude 边界
- [ ] **FR99 MVP 与 FR113 关闭仍有效**（NFR48）

---

## 门禁一句话

**缺 NFR14（或缺 a–d / 未钉死 syn-scan 策略）⇒ 不得将 59.2–59.3 标 `ready`。**  
**FR118 完成面 = 无 metadata 时 workspace `#[bitloom::top]` syn-scan；不得以 DesignFixture / metadata `design_roots` / shallow finish / docs-only alone 关闭。**
