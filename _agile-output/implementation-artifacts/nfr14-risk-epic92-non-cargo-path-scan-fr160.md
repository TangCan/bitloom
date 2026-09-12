# NFR14 风险记录 — Epic 92 非 Cargo monorepo 路径扫描（FR160）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 19 **NFR68–NFR72**；实现面 **FR160**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic59-syn-scan-design-root-discovery.md` / `nfr14-risk-epic91-memread-full-emit-fr159.md`。  
> **前置：** Epic 87 / FR154 **closed**；Epic 59 / **FR118** workspace `#[bitloom::top]` syn-scan（**Cargo members**）**closed**；Epic 55 / **FR113** Cargo-graph + metadata **closed**；Epic 91 / FR159 **closed**。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 92 后续故事 **92.2–92.3** 标为 `ready`，亦不得开工实现。**不得**以 FR118 / FR113 alone 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR160 / Epic 92；NFR14、NFR68、NFR69、NFR71、NFR72；对照 FR118 / FR113 / FR99 / FR155 |
| 记录日期 | 2026-09-12 |
| 状态 | closed — Story 92.3 勾选完成；Epic 92 关闭；FR160 实现面可宣称；**FR161–165 / FR156** 仍属 Epic 93–98；**不得**宣称 NFR59「全清」 |
| **选定** | 在保留 FR118（Cargo workspace members syn-scan）与 FR113（metadata）关闭面的前提下，授权 **MVP：对非 Cargo 全 monorepo / 非 members 任意路径，按约定根目录或显式路径列表扫描 `#[bitloom::top]`（及过渡 `#[rhdl::top]`），发现设计根** |

### Phase 12–18 / FR118 关闭面 vs Epic 92 实现边界（NFR68 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR113 / Epic 55** | Cargo-graph + `[package.metadata.bitloom] design_roots` | **仍有效；不得改写为失败**；**alone ≠ FR160** |
| **FR118 / Epic 59** | **Cargo workspace members** 内无 metadata 时 `#[bitloom::top]` syn-scan | **仍有效；不得改写为失败**；**alone ≠ FR160**（明确不扫非 members / 无 Cargo.toml 树） |
| **FR99 / Epic 44** | DesignFixture MVP | **仍有效**；alone ≠ FR160 |
| **Epic 87–91** | 闸门 / lsp / FSM / genhtml / MemRead | **已关闭；本 epic 不做** |
| **Epic 92 / FR160** | **非 Cargo** / 非 members **任意约定路径**扫描 | **本实现 epic** |
| **Epic 93–98** | 其余 NFR59 + 宣称 | **本 epic 不做** |

**选定：关闭「仅能经 Cargo.toml / workspace members 发现」缺口；≠ 重做 FR118 members syn-scan；≠ 全磁盘无界 crawl。**

### FR160 钉死子集（本 epic · NFR71）

| 维度 | MVP（92.2 必须） | 明确不在本 epic（须新合同） |
| --- | --- | --- |
| **布局形状** | ≥1 **非 Cargo 包布局**夹具：扫描根**无**（或不依赖）作为 workspace member 的 `Cargo.toml` 发现路径；可为纯目录树 + `.rs`，或孤立 `Cargo.toml` 但不经 workspace `members` 枚举 | 任意远程 URL / VFS 插件全家桶；IDE 商店多端 |
| **入口 API / CLI / LSP** | 文档钉死的公开入口（优先扩展 `bitloom-lsp`：`discover_*` / 等价，**或** `cargo bitloom` 扫描子命令）；须接受 **显式路径列表** 和/或 **约定扫描根** | 仅改 FR118 文档声称「已含非 Cargo」 |
| **识别** | 与 FR118 同形：`#[bitloom::top]` / `#[rhdl::top]` → 类型名 = `root_id` | 新注解方言；强制编译期 proc-macro 发现 |
| **权限 / 失败语义** | 路径不存在 / 非目录 / **权限拒绝（EACCES）** / IO 失败 → **可读失败**（诊断码或非零退出）；**不得** silent-Ok 空列表并宣称 FR160 绿 | 吞掉权限错误当「无设计」 |
| **与 FR118 边界** | FR118 = **Cargo members** syn-scan；FR160 = **members 之外 / 无 Cargo 图**路径；两者可并存；`discover_design_roots(Cargo root)` 行为**不得**被改写为失败 | 把 FR118 关闭证据改写成「已含任意 monorepo 路径」 |
| **验收谓词** | (1) 非 Cargo 布局夹具：约定入口发现 ≥1 top；(2) 非法/无权限路径可读失败；(3) FR118 / FR113 回归不破 | 仅 docs；仅 members 夹具冒充 |

**明确不在本 epic：** 无界全盘扫描；跨机网络挂载策略；替换 rust-analyzer；重写 FR99 DesignFixture。

### 目标产品形状（92.2）

- **入口：** 文档钉死（示例名义）：`discover_design_roots_under(paths: &[Path])`（或等价 CLI/LSP）；可与既有 `discover_design_roots` 共存。
- **产物：** `DiscoveredDesignRoot`（或文档等价）列表；可进入既有 elaborate / analyze 路径（或诚实声明发现-only + 后续 elaborate 边界）。
- **语义锚：** 识别规则对齐 FR118；布局锚为 **非 Cargo members 路径**。

### 验收谓词 / 失败语义（92.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 非 Cargo 布局夹具经文档入口发现 top；ATDD 可复现 |
| **负向 / 失败** | 缺失路径 / 权限拒绝 → **可读失败**；**不得** silent-Ok 宣称 FR160 |
| **禁止勾选** | 仅 FR118 members syn-scan；仅 FR113 metadata；仅 docs |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **92.1** | 本 NFR14 |
| **92.2** | 扫描实现 + 非 Cargo 夹具 + ATDD；`docs/fr160-*` |
| **92.3** | README/deferred 收口；勾选 Epic 92 |

### (a) 上游约束

- **Epic 87 已关闭；Epic 88–91 已关闭。**
- **FR118：** Cargo members syn-scan **仍有效**；本 epic 升格 **非 Cargo / 非 members** 缺口，不得把 FR118 改写成「已含任意路径」。
- **FR113 / FR99：** 仍有效；alone ≠ FR160。
- **NFR68：** 不得改写 FR94–159 / FR118「已关闭」。
- **NFR71：** 禁止超出上表钉死子集静默扩大。
- **NFR72：** 未关 FR160 前不得宣称非 Cargo 路径扫描已交付；**不得以 FR118 alone 冒充 FR160**。
- **品牌：** **Bitloom**；设计 crate → **`bitloom-prelude`**（AD-6）。
- **软序：** 92.2 → 92.3。

### (b) 粗工期带

- **预计：** Epic 92 整体约 **0.75–2 人周**（92.1 ≤0.25；92.2 扫描 + 夹具 + ATDD 0.5–1.5；92.3 收口 0.25）。
- **置信度 / 假设：** 中（路径 walk / 权限在 CI 可模拟）。假设不改 FR118 members 合同语义。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **92.2–92.3** 标 `ready` 或开工实现。
- **不得以 FR118 Cargo members syn-scan alone 勾选 FR160。**
- **不得以 FR113 metadata `design_roots` alone 冒充 FR160。**
- **不得以 FR99 DesignFixture alone 冒充 FR160。**
- **不得仅改文档关闭 FR160。**
- **不得对权限/缺失路径 silent-Ok。**
- **不得静默扩大到无界全盘 / 远程 VFS / IDE 商店。**
- **不得静默扩大 FR142**（若加 CLI 须在 92.2 显式列出）。
- **不得改写 Phase 12–18 / FR118 关闭证据为失败。**
- 不得把本记录冒充 **NFR14-crates**（FCFS）完成定义。
- 不得在 FR160 未关时宣称 NFR59「全清」。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 与 **NFR68 / NFR69 / NFR71 / NFR72** 共同责任人
- 备份 / 升级路径：改为「仅文档说明请手动列路径」而无扫描实现，或扩大到无界 crawl，须升级至产品 / Correct Course 批准人。

---

### Epic 92 关闭条件（Story 92.3 勾选）

- [x] **FR160 钉死子集实现 + 验收** — Story 92.2
- [x] **文档 / deferred / README 收口** — Story 92.3
- [x] **NFR68/71/72：** 边界与诚实义务保持
- [x] **品牌 / AD-6：** Bitloom；设计 crate → `bitloom-prelude`
- [x] **其余 FR161–165：** 未关前不得宣称 NFR59 全清
