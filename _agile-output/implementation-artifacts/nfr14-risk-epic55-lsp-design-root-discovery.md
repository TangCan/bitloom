# NFR14 风险记录 — Epic 55 LSP 设计根发现加深（FR113）

> **权威：** PRD NFR14；AD-28；Phase 13 **NFR44 / NFR45 / NFR47**；交付 **FR113**。  
> **前置：** Epic 48 **closed**（FR106）；Epic 44 **closed**（FR99 `bitloom-lsp` DesignFixture MVP）。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **55.2–55.3** 标 `ready`。  
> **隔离：** ≠ **FR90** rust-analyzer alone；≠ FR38/FR49 HTML 可视化。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR113 / Epic 55；NFR14、NFR44、NFR45、NFR47；对照 FR99、FR90、FR38 |
| 记录日期 | 2026-09-10 |
| 状态 | **closed** — Story **55.3** / Epic 55 / FR113 |
| **选定发现策略** | **Cargo-graph + `[package.metadata.bitloom] design_roots`**（超越 DesignFixture-only） |

### (a) 上游约束

- **Epic 48 / FR106 已关闭：** Phase 13 加深合同已开闸。
- **FR99 / Epic 44（已关闭 · 隔离）：** `bitloom-lsp` DesignFixture 全 elaborate MVP **仍有效**（NFR44）；**不得**改写为失败，亦不得把「仅 DesignFixture enum」冒充 FR113 根发现加深。
- **FR90：** 宿主 rust-analyzer 仍为 Rust IDE 路径；**alone ≠ FR113**，亦 **不替代** Bitloom LSP 硬件语义路径。
- **FR38 / FR49：** 层次/时序 HTML **≠ LSP**；不得计入 FR113。
- **设计依赖：** 设计 crate 只依赖 `bitloom-prelude`；LSP 在工具链 `bitloom-lsp`。
- **品牌：** Bitloom / `bitloom-lsp`。

### (b) 粗工期带

- **预计：** Epic 55 整体约 **1–5 人周**（55.1 ≤0.25；55.2 发现+夹具 0.75–4；55.3 收口 0.25–0.75）。置信度：**中**。
- **假设：** 未钉死的「无 metadata 的全树 `#[bitloom::top]` syn 扫描」保持 deferred（NFR47）；FR99 回归不破。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **55.2–55.3** 标 `ready`。
- **不得以 FR99 DesignFixture MVP alone 冒充 FR113。**
- **不得以 FR90 rust-analyzer alone 关闭 FR113。**
- **不得把 HTML 可视化（FR38/FR49）计入 FR113。**
- **不得仅浅层诊断（`AnalysisMode::Shallow` / 词法扫描）关闭 FR113** — 须仍走全设计 elaborate（或文档钉死的等价 `finish` 路径）。
- **不得半成品二进制交差**（须可启动 `bitloom-lsp` + 可复现发现/elaborate 夹具 + ATDD）。
- **不得仅改文档关闭 FR113。**
- 不得改写 FR99「已关闭」为失败（NFR44）。
- 不得 silent 宣称未钉死的全树 syn-scan 策略已交付（NFR47）。
- 不得冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR47** 共同责任人；NFR44/45 共同注意人。

---

### 发现策略（本 epic 钉死）

| 策略 | 内容 | 本 epic |
| --- | --- | --- |
| **Cargo-graph + metadata `design_roots`** | 从 workspace `Cargo.toml` 图发现依赖 `bitloom-prelude` 的设计包；根由 `[package.metadata.bitloom] design_roots = [...]`（或文档等价字段）指向可 elaborate 的 `.rs`/类型入口 | **选定 = FR113 完成面** |
| DesignFixture-only enum | FR99 MVP 夹具枚举 | **仍保留为回归**；**alone ≠ FR113** |
| 无 metadata 的全 workspace `#[bitloom::top]` syn 扫描 | 任意 `.rs` 自动发现 | **deferred**（NFR47；须新合同） |

### 选定策略 — 性能 / 范围 / 与 FR90 分工

| 项 | 钉死 |
| --- | --- |
| **性能** | 沿用 FR99 P3：夹具级首次诊断目标 ≤ 2s（冷启动可排除）；超时 → `bitloom-lsp.timeout` |
| **范围天花板** | 沿用 / 文档化 FR99 P4 量级（模块数天花板）；过大 → `bitloom-lsp.oversized`，`called_finish` 不得伪装成功 |
| **夹具** | ≥1 **非 DesignFixture-only** 工程（真实/迷你 Cargo 包 + metadata roots）；ATDD 证明 didSave/库 API 走发现路径并全 elaborate |
| **失败可读** | 发现失败 / 无根 / 超时 / 过大须有可读诊断码或消息 |
| **vs FR90** | rust-analyzer = Rust 语言服务；Bitloom LSP = 硬件 elaborate 语义；二者互补，**FR90 不关闭 FR113** |
| **工具依赖** | 树内 `bitloom-lsp` / `just test`；不要求外挂 IDE 插件商店上架 |

### Epic 55 关闭条件（Story 55.3 勾选）

- [x] **55.2 / FR113：** 选定发现策略可运行 + ATDD + ≥1 非 DesignFixture-only 夹具；负向/超时可读
- [x] **文档 / deferred / FR99 交叉链**（未选全树 syn-scan 保持 deferred）
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖：** Bitloom；prelude 边界
- [x] **FR99 MVP 关闭仍有效**（NFR44）

---

## 门禁一句话

**缺 NFR14（或缺 a–d / 未钉死发现策略）⇒ 不得将 55.2–55.3 标 `ready`。**  
**FR113 完成面 = Cargo-graph + metadata `design_roots`（超越 DesignFixture）；不得以 FR99 DesignFixture / FR90 / HTML / 浅层诊断 alone 关闭。**
