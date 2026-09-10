# NFR14 风险记录 — Epic 51 FSM / state-visit 仿真覆盖率（FR109）

> **权威：** PRD NFR14；AD-28；Phase 13 **NFR44 / NFR45 / NFR47**；交付 **FR109**。  
> **前置：** Epic 48 **closed**（FR106）；Epic 47 **closed**（FR105 Mux 分支 v2；**C3 FSM cropped**）。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **51.2–51.3** 标 `ready`。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR109 / Epic 51；NFR14、NFR44、NFR45、NFR47；对照 FR105 C2/C3、FR34 |
| 记录日期 | 2026-09-10 |
| 状态 | **closed** — Story 51.3 / Epic 51（2026-09-10） |

### (a) 上游约束

- **Epic 48 / FR106 已关闭：** Phase 13 加深合同已开闸；本 epic 将 FR105 **C3 cropped** 升格为 **FR109** 必选。
- **FR105 / Epic 47（已关闭 · 隔离）：** Mux 分支覆盖率 **v2**（C2）**仍有效**（NFR44）；**不得**改写为失败，亦不得把「Mux v2 已关」冒充 C3 / state-visit 已交付。
- **C3（历史 cropped）：** 状态枚举 / FSM state-visit 覆盖 — 现为本 epic **必选验收面**（见下方度量钉死）。
- **FR34：** toggle hit/miss 文本是 P2a 基线；**单独不足以**关闭 FR109 / 冒充 C3。
- **设计依赖：** 覆盖率记录器落在仿真 / 工具链侧（`bitloom-sim` 或文档钉死的等价路径）；设计 crate 只依赖 `bitloom-prelude`。
- **品牌：** Bitloom / `bitloom-*`。
- **Epic 56 / FR114：** Tywaves / LCOV GUI **不在**本 epic 关闭集（NFR47）；保持 deferred，除非 Epic 56 另行关闭。

### (b) 粗工期带

- **预计：** Epic 51 整体约 **1–4 人周**（51.1 ≤0.25；51.2 记录器+夹具 0.75–3；51.3 收口 0.25–0.75）。置信度：**中**。
- **假设：** 不要求商业覆盖率 GUI / Tywaves；FR105 Mux v2 回归不破。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **51.2–51.3** 标 `ready`。
- **不得仅改文档 / README 口号关闭 FR109**（须有可运行记录器 + 报告 + ≥1 夹具）。
- **不得把 FR105 Mux 分支 v2 alone 写成 C3 / FR109 完成面。**
- **不得把 FR34 toggle hit/miss alone 写成 C3 / FR109。**
- 不得改写 FR105「已关闭」为失败（NFR44）。
- 不得静默扩大至 Tywaves / LCOV GUI（属 Epic 56；NFR47）。
- 不得冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR47** 共同责任人；NFR44/45 共同注意人。

---

### C3 / FR109 度量类型与报告格式（钉死 · 供 51.2）

| # | 项 | 钉死内容 |
| --- | --- | --- |
| **M1 · 度量类型** | **FSM / state-visit** | 对夹具钉死的显式状态枚举（或等价有限状态标签集）：记录每个状态是否被访问（visit）；至少支持「已访问 / 未访问」布尔或计数 ≥1 |
| **M2 · 报告格式** | 稳定可读文本（或 JSON 等价且 ATDD 可解析） | 须含：覆盖率族标识（如 `bitloom-sim coverage` + **FSM/state-visit** / **C3** / **FR109** 字样）；状态名列表与 visit 结果；未覆盖状态可读 |
| **M3 · 夹具范围** | ≥1 可运行夹具 | 小 FSM（或文档等价状态机）经仿真路径产出报告；ATDD 断言相对「仅 Mux v2」本路径执行 C3 合同 |
| **M4 · 失败可读** | 负向 / 边界 | 未覆盖状态或缺失 C3 段时，报告或诊断须可读（ATDD 至少一类） |

**明确非目标（NFR47）：** Tywaves 级 IDE 波形；LCOV / 第三方覆盖率 GUI（→ Epic 56 / FR114）；全芯片无限状态；商业覆盖率产品对拍；静默把 Mux v2 报告冒充 state-visit。

### 与 FR105 / FR34 对照

| 路径 | 角色 | 可否单独关闭 FR109 |
| --- | --- | --- |
| FR105 Mux 分支 v2（C2） | Phase 12 MVP；仍须回归 | **否** |
| FR34 toggle hit/miss | P2a 基线 | **否** |
| 本记录 M1–M4 + ATDD | C3 / FR109 | **是**（须 51.2→51.3） |

### Epic 51 关闭条件（Story 51.3 勾选）

- [x] **51.2 / FR109：** M1–M4 + ATDD + ≥1 夹具报告
- [x] **文档 / deferred / FR105 交叉链**
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖：** Bitloom；prelude 边界
- [x] **FR105 Mux v2 MVP 关闭仍有效**（NFR44）
- [x] **Tywaves/LCOV GUI 仍属 Epic 56**（未开则 deferred）

---

## 门禁一句话

**缺 NFR14（或缺 a–d）⇒ 不得将 51.2–51.3 标 `ready`。**  
**不得以 FR105 Mux v2 或 FR34 toggle alone 冒充 C3（FR109）已交付；不得仅改文档关闭。**  
**Epic 51 / FR109 已关闭（Story 51.3）：** C3 state-visit 完成面；FR105 Mux v2 MVP 关闭仍有效；Tywaves/LCOV GUI → Epic 56。
