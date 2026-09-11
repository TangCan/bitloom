# NFR14 风险记录 — Epic 80 公开 API 表面清单钉死（FR142）

> **权威：** PRD NFR14；AD-28；Phase 17 **NFR60 / NFR61 / NFR63**；交付 **FR142**。  
> **前置：** Epic 79 **closed**（FR141 / Story 79.4；`ce707b3`）；`correctCoursePhase17Approved: 2026-09-11`；Q1–Q5 默认。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **80.2–80.3** 标 `ready`。  
> **隔离：** ≠ Phase 16 终局 alone；≠ NFR59 产品加深；≠ 未文档化内部 API 写入 in-surface。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR142 / Epic 80；对照 FR141 / FR147 / NFR15 / Q1–Q2 |
| 记录日期 | 2026-09-11 |
| 状态 | accepted — Story 80.1；80.2–80.3 待本记录后 ready |
| **选定交付** | 成文 `docs/public-api-1-0-surface.md` + ATDD（in/out 分区） |

### (a) 上游约束

- Epic 79 / FR141 已关闭；Phase 12–16 关闭仍有效（**NFR60**）。
- Q1：`bitloom-sim` **纳入** 1.0 表面；Q2：`bitloom-hir` / `bitloom-builder` / `bitloom-vlog` 可 publish，**不**进 1.0 稳定承诺。
- 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；宏经 prelude 再导出；品牌 Bitloom。
- 建议路径：`docs/public-api-1-0-surface.md`；ATDD 锁必填节与 in/out 分区。
- 禁止把未文档化内部 API / `pub` 实现细节默认可写入 in-surface。

### (b) 粗工期带

- Epic 80：约 **0.5–1.5 人周**（80.1 ≤0.25；80.2 0.25–1；80.3 ≤0.25）。置信度：高。

### (c) 禁止的静默降级清单

- 不得缺本记录将 **80.2–80.3** 标 `ready`。
- **不得**把未文档化内部 API 写入 in-surface。
- **不得**把 hir/builder/vlog 写入 1.0 稳定承诺（Q2）。
- **不得**把 LSP / `bitloom-lsp` 写入 1.0 稳定承诺（本批默认 out）。
- **不得**静默吞并 NFR59（NFR63）。
- 不得改写 FR94–140「已关闭」为失败（NFR60）。
- 不得冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR60** / **NFR61** / **NFR63**。

---

### 建议纳入 / 排除表（Q1–Q2）

| Crate / 面 | 分区 | 1.0 承诺？ |
| --- | --- | --- |
| `bitloom` CLI 文档化子命令 | **in-surface** | 是 |
| `bitloom-prelude` 主路径（含 `rhdl` facade / `ip`） | **in-surface** | 是 |
| `bitloom-macro` 文档化属性（经 prelude 使用） | **in-surface** | 是 |
| `bitloom-sim` 公开 API（tick / VCD / 双模型） | **in-surface**（Q1） | 是 |
| `bitloom-hir` / `bitloom-builder` / `bitloom-vlog` | **out-of-promise**（可 publish） | 否（Q2） |
| `bitloom-lsp` / LSP | **out-of-surface** | 否 |
| 设计 crate → 仅 `bitloom-prelude` | **边界**（AD-6） | 必须遵守 |

### ATDD 策略

- Story 80.2：`fr142_public_api_1_0_surface.rs` 断言文件存在、in/out 分区、prelude 边界、Q1/Q2。
- 缺节或分区缺失 → 测试失败。

### Epic 80 关闭条件（Story 80.3 勾选）

- [ ] **表面清单成文 + ATDD** — Story 80.2
- [ ] **README / deferred 指针** — Story 80.3
- [ ] **禁止事项未触发**（未文档化 API 未入 in-surface；NFR59 未吞并）
- [ ] **品牌 / 依赖：** Bitloom / `bitloom-prelude`
