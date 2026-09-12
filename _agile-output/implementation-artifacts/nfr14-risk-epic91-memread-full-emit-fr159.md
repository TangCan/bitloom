# NFR14 风险记录 — Epic 91 MemRead stub→完整生成（FR159）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 19 **NFR68–NFR72**；实现面 **FR159**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic90-third-party-lcov-gui-fr158.md` / `nfr14-risk-epic54-formal-dual-model-depth.md`。  
> **前置：** Epic 87 / FR154 **closed**；Epic 54 / **FR112** GeneratedFunctional MemRead≡tick（**in-process**）**closed**；`generate_functional_sim` 发出的 crate 仍可将 `MemRead` stub 为 `0`（见 `docs/fr112-*`）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 91 后续故事 **91.2–91.3** 标为 `ready`，亦不得开工实现。**不得**以 stub alone / FR112 alone 勾选本 FR。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR159 / Epic 91；NFR14、NFR68、NFR69、NFR71、NFR72；对照 FR112 / FR100 / FR103 / FR47 |
| 记录日期 | 2026-09-12 |
| 状态 | closed — Story 91.3 勾选完成；Epic 91 关闭；FR159 实现面可宣称；**FR160–165 / FR156** 仍属 Epic 92–98；**不得**宣称 NFR59「全清」 |
| **选定** | 在保留 FR112 in-process MemRead≡tick 关闭面的前提下，授权 **MVP：升格 `generate_functional_sim` 发出的功能仿真 crate，使 `MemRead` 不再恒 stub 为 `0`，而与 GeneratedFunctional / `Sim::tick` 语义对齐（含 SyncReadMem latency-1）** |

### Phase 12–18 / FR112 关闭面 vs Epic 91 实现边界（NFR68 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR112 / Epic 54** | **In-process** `GeneratedFunctional` MemRead≡tick | **仍有效；不得改写为失败**；**alone ≠ FR159** |
| **FR100 / FR103** | F1-(i) / SyncFifo 手写 FL | **仍有效**；alone ≠ FR159 |
| **Epic 87–90** | 闸门 / lsp / FSM / genhtml | **已关闭；本 epic 不做** |
| **Epic 91 / FR159** | **Emitted** functional-sim crate 的 MemRead **完整生成** | **本实现 epic** |
| **Epic 92–98** | 其余 NFR59 + 宣称 | **本 epic 不做** |

**选定：关闭「发出 crate 仍 stub MemRead=0」缺口；≠ 重做 FR112 in-process 证明；≠ 多端口/异步 mem 全家桶。**

### 相对 stub 的缺口清单（本 epic · NFR71）

| # | Stub / 半成品现状 | FR159 目标（完整生成） |
| --- | --- | --- |
| **G1** | `generate_functional_sim` 对 `AssignExpr::MemRead` emit 字面 **`0`**（`crates/bitloom-sim/src/generate.rs`） | Emit **真实读路径**：查 mem 存储 + addr；SyncReadMem 遵守 **latency-1**（与 `GeneratedFunctional` / `Sim::tick` 一致） |
| **G2** | 发出 crate 无法单独复现 MemRead≡tick（须靠 in-process 视图） | 发出 crate 在文档化夹具刺激下 **rdata/ports 与 tick 对齐**（或显式等价谓词） |
| **G3** | 文档诚实声明「emitted may stub MemRead as 0」 | 文档改为 **FR159 已关闭 stub**；交叉链 FR112（in-process 仍有效） |

**明确不在本 epic：** 任意多读口/banked mem；写冲突完整仲裁模型；Verilog/FIRRTL 后端 Mem 语义加深（除非 emit 功能 crate 路径必需）；SymbiYosys（→ 已有 FR119）。

### 目标 emit / 生成形状

- **入口：** 既有 `bitloom_sim::generate_functional_sim`（或文档钉死的等价 API）— **不**新开平行生成器除非 91.2 证明必要。
- **产物：** 可 `cargo` 编译的功能仿真 crate 源；`MemRead` 求值代码 **非**恒 `0`。
- **语义锚：** 与 `GeneratedFunctional` / cycle-accurate `Sim::tick` 对 SyncReadMem 的既有行为一致（latency-1）。

### 验收谓词 / 失败语义（91.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | ≥1 SyncReadMem 夹具：经 **发出 crate**（或文档等价「emit 后编译运行」路径）在文档化刺激下，读数据/端口与 `Sim::tick`（或 `GeneratedFunctional`）一致；ATDD 可复现 |
| **负向 / 失败** | 故意错误的 MemRead emit（若保留回归）或未实现路径 → **可读失败**；**不得** silent-Ok 宣称 FR159 |
| **禁止勾选** | 仅证明 in-process FR112；仅文档改字；仍 emit `0` |

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **91.1** | 本 NFR14 |
| **91.2** | 关闭 G1–G3；ATDD/黄金；`docs/fr159-*` |
| **91.3** | README/deferred 收口；勾选 Epic 91 |

### (a) 上游约束

- **Epic 87 已关闭；Epic 88–90 已关闭。**
- **FR112：** in-process MemRead≡tick **仍有效**；本 epic 升格 **emitted crate** 缺口，不得把 FR112 改写成「已含 emit 完整 MemRead」。
- **NFR68：** 不得改写 FR94–153 / FR112「已关闭」。
- **NFR71：** 禁止超出 G1–G3 / SyncReadMem MVP 静默扩大。
- **NFR72：** 未关 FR159 前不得宣称 MemRead 完整生成已交付；**不得以 stub alone 勾选。**
- **品牌：** **Bitloom**；设计 crate → **`bitloom-prelude`**（AD-6）。
- **软序：** 91.2 → 91.3。

### (b) 粗工期带

- **预计：** Epic 91 整体约 **0.75–2 人周**（91.1 ≤0.25；91.2 emit 语义 + ATDD 0.5–1.5；91.3 收口 0.25）。
- **置信度 / 假设：** 中（emit 模板与 sync pending 队列需对齐 GeneratedFunctional）。假设不改 FR112 夹具语义。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **91.2–91.3** 标 `ready` 或开工实现。
- **不得以 MemRead stub（恒 `0`）alone 勾选 FR159。**
- **不得以 FR112 in-process alone 冒充 FR159。**
- **不得仅改文档关闭 FR159。**
- **不得静默扩大到多 bank / 全 mem 仲裁 / 新后端。**
- **不得静默扩大 FR142。**
- **不得改写 Phase 12–18 / FR112 关闭证据为失败。**
- 不得把本记录冒充 **NFR14-crates**（FCFS）完成定义。
- 不得在 FR159 未关时宣称 NFR59「全清」。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 与 **NFR68 / NFR69 / NFR71 / NFR72** 共同责任人
- 备份 / 升级路径：缩回「仅文档声明 stub」或扩大到异步/多口 mem 须升级至产品 / Correct Course 批准人。

---

### Epic 91 关闭条件（Story 91.3 勾选）

- [x] **FR159 缺口 G1–G3 关闭 + ATDD** — Story 91.2
- [x] **文档 / deferred / README 收口** — Story 91.3
- [x] **NFR68/71/72：** 边界与诚实义务保持
- [x] **品牌 / AD-6：** Bitloom；设计 crate → `bitloom-prelude`
- [x] **其余 FR160–165：** 未关前不得宣称 NFR59 全清
