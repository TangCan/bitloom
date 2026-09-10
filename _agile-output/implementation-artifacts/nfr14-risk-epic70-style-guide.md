# NFR14 风险记录 — Epic 70 完整 Style Guide ± Parser（FR130）

> **前置：** Epic 64 closed；FR122 O1–O4 closed；AD-27 默认禁止 Parser。  
> **门禁：** 无本记录 ⇒ 不得标 70.2–70.3 ready。

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR130 / Epic 70；NFR14 / NFR52 / NFR54 / NFR55 |
| 记录日期 | 2026-09-10 |
| 状态 | accepted — Story 70.1 |

### (a) 上游约束

- FR122 O1–O4 **仍有效**（NFR52）；alone ≠ FR130。
- FR97 / FR111 关闭仍有效；alone ≠ FR130。
- **默认不恢复** `Parser.parse`；若恢复须 Correct Course + 再修订 AD-27。
- 公开品牌 **Bitloom** / `bitloom-*`；设计 crate 只依赖 `bitloom-prelude`。

### (b) 粗工期带

- 0.5–2 人周。

### (c) 禁止的静默降级清单

- 不得缺本记录标 70.2–70.3 ready。
- **不得仅 FR97。**
- **不得仅 FR111。**
- **不得仅 FR122 O1–O4 alone。**
- **不得 docs-only。**
- **不得静默恢复 Parser。**
- 不得改写 FR122 关闭为失败（NFR52）。
- 不得冒充 NFR14-crates。

### (d) 负责人

- Richard — NFR14 / NFR54 / NFR52 / NFR55。消歧：NFR14-crates ≠ 本门禁。

### Style Guide 深度（G1–G4 ≡ S1–S4）

| ID | 钉死 |
| --- | --- |
| G1 / S1 | Style Guide 宣称头 + `scalafmt-style` |
| G2 / S2 | `withClockAndReset` 纪律注释 + naming |
| G3 / S3 | **不恢复** Parser（文档 + 验收谓词） |
| G4 / S4 | `emit_chisel_style_guide_fr130` / `check_chisel_style_guide_fr130`；每模块 `// --- FR130 style-guide ---` |

### AD-27

**须修订** — 允许 FR130 Style Guide 验收面；默认仍禁止 Parser。

### Epic 70 关闭条件（70.3）

- [ ] **70.2 / FR130**
- [ ] **文档 / deferred / AD-27**
- [ ] **禁止事项未触发**
- [ ] **品牌 / 依赖**
- [ ] **FR122 关闭仍有效**
