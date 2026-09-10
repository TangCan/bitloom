# NFR14 风险记录 — Epic 54 形式等价 / 双模型深度（FR112）

> **权威：** PRD NFR14；AD-28；Phase 13 **NFR44 / NFR45 / NFR47**；交付 **FR112**。  
> **前置：** Epic 48 **closed**（FR106）；Epic 45 **closed**（FR100 F1-(i) 有界穷举 + FR102 + FR103 SyncFifo 手写 FL MVP）。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **54.2–54.3** 标 `ready`。  
> **隔离：** ≠ **FR107** SystemC TLM AT。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR112 / Epic 54；NFR14、NFR44、NFR45、NFR47；对照 FR100/FR102/FR103、FR92、FR30 |
| 记录日期 | 2026-09-10 |
| 状态 | **closed** — Story **54.3** / Epic 54 / FR112 |
| **选定加深分支** | **(B) GeneratedFunctional MemRead ≡ tick** |

### (a) 上游约束

- **Epic 48 / FR106 已关闭：** Phase 13 加深合同已开闸。
- **FR100 / Epic 45（已关闭 · 隔离）：** F1-(i) 有界穷举 `FormalEquivProduct` **仍有效**（NFR44）；**不得**改写为失败，亦不得把 F1-(i) alone 冒充 FR112。
- **FR103：** SyncFifo 手写 FL 诚实 MVP **仍有效**；不得把「仅 SyncFifo 双模型」冒充 FR112 选定加深面（除非选定分支 C）。
- **FR92 / FR30：** 记分板 / 有界夹具 checker 为配套；**alone ≠ FR112**（亦 ≠ FR100）。
- **≠ FR107：** SystemC AT / `nb_transport` **不是**本 epic 完成面。
- **设计依赖：** 设计 crate 只依赖 `bitloom-prelude`；形式/生成路径在工具链 crate。
- **品牌：** Bitloom / `bitloom-*`。

### (b) 粗工期带

- **预计：** Epic 54 整体约 **1–6 人周**（54.1 ≤0.25；54.2 选定分支 0.75–5；54.3 收口 0.25–0.75）。置信度：**中**（分支 B 中等；若改选 A 则外挂工具依赖升高）。
- **假设：** 未选分支保持 deferred（NFR47）；FR100/103 回归不破。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **54.2–54.3** 标 `ready`。
- **不得以 FR92 记分板 alone 关闭 FR112。**
- **不得以 FR78/FR92 adapter 模板 alone 关闭 FR112。**
- **不得以 FR100 F1-(i) alone / FR103 SyncFifo MVP alone 冒充 FR112。**
- **不得把 FR107 SystemC AT 写成 FR112。**
- **不得仅改文档关闭 FR112**（须有选定分支可运行流程 + 夹具 + ATDD）。
- **不得 silent 宣称未选分支（A/C）已交付**（NFR47）。
- 不得改写 FR100/103「已关闭」为失败（NFR44）。
- 不得冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR47** 共同责任人；NFR44/45 共同注意人。

---

### 加深路径三选一（本 epic 钉死）

| 分支 | 内容 | 本 epic |
| --- | --- | --- |
| **(A) F1-(ii) SymbiYosys/SMT** | 外挂形式工具绑定产品入口 | **deferred**（未选；须新合同或后续 epic） |
| **(B) GeneratedFunctional MemRead ≡ tick** | 生成功能视图对 SyncReadMem/MemRead 与周期 `tick` 语义对齐可检查 | **选定 = FR112 完成面** |
| **(C) 更多一级 IP 手写 FL** | 超出 SyncFifo 的更多 IP 手写功能模型完整面 | **deferred**（未选） |

### 选定分支 (B) — 证明义务 / 夹具 / 工具依赖

| 项 | 钉死 |
| --- | --- |
| **证明义务** | 至少一夹具：含 SyncReadMem（或文档等价 MemRead）的 FrozenHir，经 **GeneratedFunctional**（或文档钉死的生成功能路径）与 **`Sim::tick`** 在文档化刺激下输出一致（或显式等价谓词通过）；ATDD 可复现 |
| **夹具范围** | ≥1 可运行夹具（小 Mem 深度即可）；负向/边界至少一类可读失败（例如未接线 MemRead 路径不得 silent Ok 宣称 FR112） |
| **工具依赖** | **树内** Bitloom 工具链（`just test` / cargo test）；**不**要求 CI 安装 SymbiYosys（分支 A 未选） |
| **超出 MVP** | 明确超出 FR100 F1-(i) 有界穷举与 FR103「SyncFifo 手写 FL」诚实面——本面针对 **生成功能路径的 MemRead≡tick** |

### Epic 54 关闭条件（Story 54.3 勾选）

- [x] **54.2 / FR112：** 分支 B 可运行 + ATDD + ≥1 夹具；负向可读
- [x] **文档 / deferred / FR100·FR103 交叉链**（未选 A/C 保持 deferred）
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖：** Bitloom；prelude 边界
- [x] **FR100/103 MVP 关闭仍有效**（NFR44）

---

## 门禁一句话

**缺 NFR14（或缺 a–d / 未钉死三选一）⇒ 不得将 54.2–54.3 标 `ready`。**  
**FR112 完成面 = 选定分支 (B) GeneratedFunctional MemRead≡tick；不得以 FR92 / F1-(i) / adapter alone 关闭；A/C 未交付不得 silent 宣称。**
