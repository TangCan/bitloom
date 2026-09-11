# NFR14 风险记录 — Epic 83 发布 Bitloom 1.0.0（FR146）

> **权威：** PRD NFR14；AD-28；Phase 17 **NFR60 / NFR61 / NFR62 / NFR63**；交付 **FR146**。  
> **前置：** Epic 81+82 **closed**（含 FR145-skip；`89aaa07`）。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **83.2–83.3** 标 `ready`。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR146 / Epic 83；对照 FR142 表面 / FR147 |
| 记录日期 | 2026-09-11 |
| 状态 | accepted — Story 83.1 |
| **选定** | 升 workspace **1.0.0**；CHANGELOG；annotated tag `v1.0.0`；`cargo publish --dry-run` + 手动 publish 清单；真实 crates.io publish **可延后**若缺凭证 |

### (a) 上游约束

- 表面承诺 crate（FR142）：`bitloom` / `bitloom-prelude` / `bitloom-macro` / `bitloom-sim`（workspace 锁步版本）。
- hir/builder/vlog 同 workspace 版本可升至 1.0.0 但 **out-of-promise**（Q2）。
- Publish 顺序建议：macro → hir → builder → vlog → prelude → sim → bitloom（按依赖）。
- Dry-run 策略：每包 `cargo publish -p … --dry-run`；失败则不得宣称 FR146 完成。
- Tag：annotated `v1.0.0` 指向发版提交；CHANGELOG 须有 `[1.0.0]` 节。
- **禁止**未关 FR141–145（含 145-skip）即 publish。
- NFR59 仍 deferred；README 须保留列表。

### (b) 粗工期带

- Epic 83：0.5–1.5 人周（含凭证/dry-run 反复）。置信度：中。

### (c) 禁止的静默降级清单

- 不得缺本记录将 **83.2–83.3** 标 `ready`。
- 不得无 dry-run/清单即声称已 publish。
- 不得用 Phase 16 alone 宣称 1.0（FR147）。
- 不得静默吞并 NFR59。
- 不得冒充 NFR14-crates。

### (d) 负责人

- Richard（Dev）— NFR14 / NFR62 / 发版。

---

### Epic 83 关闭条件（Story 83.3 勾选）

- [ ] **发版 1.0.0 + tag + CHANGELOG** — Story 83.2
- [ ] **README / deferred / Phase 17 指针** — Story 83.3
- [ ] **禁止事项未触发**
- [ ] **品牌：** Bitloom
