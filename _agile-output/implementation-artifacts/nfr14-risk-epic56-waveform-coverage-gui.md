# NFR14 风险记录 — Epic 56 富波形 / 覆盖率 GUI 加深（FR114）

> **权威：** PRD NFR14；AD-28；Phase 13 **NFR44 / NFR45 / NFR47**；交付 **FR114**。  
> **前置：** Epic 48 **closed**（FR106）；Epic 47 **closed**（FR104 `interactive.html` I1–I3 + FR105 coverage v2 Mux）；Epic 51 **closed**（FR109 C3 FSM；≠ 本 epic GUI）。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **56.2–56.3** 标 `ready`。  
> **隔离：** ≠ 仅静态 timing / VCD /「请开 GTKWave」；≠ FR104/105 MVP alone。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR114 / Epic 56；NFR14、NFR44、NFR45、NFR47；对照 FR104、FR105、FR109、FR34 |
| 记录日期 | 2026-09-10 |
| 状态 | **closed** — Story **56.3** / Epic 56 / FR114 |
| **选定加深子集** | **(B) LCOV + 树内覆盖率 GUI 一等路径** |

### (a) 上游约束

- **Epic 48 / FR106 已关闭：** Phase 13 加深合同已开闸。
- **FR104 / Epic 47（已关闭 · 隔离）：** `interactive.html` I1–I3 **仍有效**（NFR44）；**不得**改写为失败，亦不得把 I1–I3 alone 冒充 FR114。
- **FR105：** coverage v2 Mux 分支 **仍有效**；**alone ≠ FR114**。
- **FR109 / Epic 51：** C3 FSM/state-visit 记录器 **已关闭且独立**；本 epic 是 **GUI/可观测性加深**，不得把「仅有 FR109 文本报告」写成 FR114。
- **FR34：** toggle 基线为配套；**alone ≠ FR114**。
- **设计依赖：** 设计 crate 只依赖 `bitloom-prelude`；波形/覆盖率工具在工具链 / CLI。
- **品牌：** Bitloom / `bitloom-*`。

### (b) 粗工期带

- **预计：** Epic 56 整体约 **1–5 人周**（56.1 ≤0.25；56.2 选定子集 0.75–4；56.3 收口 0.25–0.75）。置信度：**中**。
- **假设：** 未选 (A) Tywaves 级 typed IDE 波形保持 deferred（NFR47）；FR104/105/109 回归不破。

### (c) 禁止的静默降级清单

- 不得在缺本记录时将 **56.2–56.3** 标 `ready`。
- **不得仅静态 timing.html / VCD /「请开 GTKWave」关闭 FR114。**
- **不得以 FR104 `interactive.html` I1–I3 alone 冒充 FR114。**
- **不得以 FR105 Mux v2 alone（或 FR109 文本报告 alone）冒充 FR114。**
- **不得仅改文档关闭 FR114**（须有可复现产品路径 + 夹具/步骤 + ATDD 或文档化手动验收清单）。
- **不得 silent 宣称未选子集 (A) 已交付**（NFR47）。
- 不得改写 FR104/105「已关闭」为失败（NFR44）。
- 不得冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR47** 共同责任人；NFR44/45 共同注意人。

---

### 加深子集（至少钉死一类）

| 子集 | 内容 | 本 epic |
| --- | --- | --- |
| **(A) Tywaves 级 typed IDE 波形** | 一等集成 Tywaves（或文档等价 typed 源级 IDE 波形）/ 自研等价 | **deferred**（未选；须新合同） |
| **(B) LCOV / 第三方覆盖率 GUI** | 一等：Bitloom 导出 LCOV（或文档等价）+ **树内**覆盖率 HTML GUI（或文档钉死的第三方 LCOV GUI 一等打开路径） | **选定 = FR114 完成面** |

### 选定子集 (B) — 证明义务 / 夹具 / 工具依赖

| 项 | 钉死 |
| --- | --- |
| **证明义务** | 从仿真覆盖率（至少 Mux 分支；可含已关 FR109 FSM 若夹具注册）生成 **LCOV** 工件；提供 **可打开的树内覆盖率 GUI**（Bitloom 品牌 HTML）展示 hit/miss；ATDD 或手动验收清单可检查 |
| **夹具** | ≥1 可复现 `cargo bitloom` / 库 API 路径写出 `coverage.lcov` + `coverage.html`（名可文档等价）；负向：缺覆盖数据不得 silent 宣称 FR114 绿 |
| **工具依赖** | **树内** Bitloom（`just test`）；**不**要求 CI 安装外部 Tywaves；可选第三方 LCOV 查看器仅为补充，不得单独关闭 |
| **超出 MVP** | 明确超出 FR104 I1–I3 与 FR105 Mux v2 文本报告 / FR109 文本 state 报告——本面是 **LCOV + GUI** |
| **不回归** | `interactive.html` / VCD / Mux v2 默认路径仍可用 |

### Epic 56 关闭条件（Story 56.3 勾选）

- [x] **56.2 / FR114：** 子集 B 可运行 + ATDD/验收清单 + ≥1 夹具；负向可读
- [x] **文档 / deferred / FR104·FR105 交叉链**（未选 A 保持 deferred）
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖：** Bitloom；prelude 边界
- [x] **FR104/105 MVP 关闭仍有效**（NFR44）
- [x] **Phase 13 规划故事齐**（Epic 48–56 实现故事清单完整指针）

---

## 门禁一句话

**缺 NFR14（或缺 a–d / 未钉死 A/B 子集）⇒ 不得将 56.2–56.3 标 `ready`。**  
**FR114 完成面 = (B) LCOV + 树内覆盖率 GUI；不得以 FR104 I1–I3 / FR105 Mux v2 / timing·VCD·GTKWave alone 关闭；A 未交付不得 silent 宣称。**
