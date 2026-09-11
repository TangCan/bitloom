# NFR14 风险记录 — Epic 82 预 1.0 表面卫生（可选 / FR145）

> **权威：** PRD NFR14；AD-28；Phase 17 **NFR60 / NFR61 / NFR63**；交付 **FR145**。  
> **前置：** Epic 81 **closed**（FR143/FR144；`e7da518`）。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **82.2–82.3** 标 `ready`。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR145 / Epic 82；对照 FR142 表面 / FR144 semver |
| 记录日期 | 2026-09-11 |
| 状态 | closed — Story 82.3；FR145-skip；Epic 82 关闭 |
| **选定** | **skip** 路径：相对 crates.io 0.1.2 的 prelude `PortField` 漂移由即将到来的 **1.0.0 major** 吸收；无额外阻塞卫生 PR 必做（Q3） |

### (a) 上游约束

- Epic 80/81 已关闭；表面清单与 semver 政策/CI 就位。
- 阻塞项候选：`cargo-semver-checks` vs crates.io 在 **minor** 模式下对 `bitloom-prelude` 报 major（`PortField::describe`→`flatten`）；pre-1.0 门禁默认 **`--release-type major`** 已允许该漂移进入 1.0。
- **Skip 条件（满足）：** 无必须在 1.0.0 前单独合并的表面卫生修复；漂移文档化并由 FR146 major 吸收。
- **禁止：** 借机做 NFR59 产品加深；无清单扩大重构。

### (b) 粗工期带

- Epic 82 skip 路径：≤0.5 人周。置信度：高。

### (c) 禁止的静默降级清单

- 不得缺本记录将 **82.2–82.3** 标 `ready`。
- 不得把 FR145 skip 写成「无任何 API 变化」。
- 不得借机关闭 NFR59。
- 不得冒充 NFR14-crates。

### (d) 负责人

- Richard（Dev）— NFR14 / NFR63。

---

### Epic 82 关闭条件（Story 82.3 勾选）

- [x] **阻塞毛刺处理或 skip 文档化** — Story 82.2（`docs/fr145-pre-1-0-hygiene-skip.md`）
- [x] **README / deferred 指针** — Story 82.3
- [x] **禁止事项未触发**
- [x] **品牌：** Bitloom

### Epic 82 关闭声明

**FR145-skip 已关闭：** 无阻塞卫生 PR；crates.io 0.1.2→本地 prelude 漂移由 **FR146 / 1.0.0 major** 吸收。Epic 83 可在 NFR14 后标 ready。NFR59 仍 deferred（NFR63）。
