# NFR14 风险记录 — Epic 42 Idiomatic Chisel 往返（FR97）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；**AD-27**（修订后：机械 + idiomatic）；**AD-3**（FIRRTL 文本契约）；Phase 12 **NFR40 / NFR41 / NFR43**；交付 **FR97**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic41-in-tree-hls.md`；`nfr14-risk-epic40-literal-path-b.md`；历史 Chisel 双向门禁（Epic 20）。  
> **前置：** Epic 40 **closed**（FR94；`65b4a37` 修订 AD-5/25/27）；Epic 41 **closed**（FR95/FR96；`032646b`）— 非硬依赖，仅时间序。Path B 字面绿合同已开闸；**须引用修订后 AD-27**。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 42 后续故事 **42.2–42.3** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR97 / Epic 42；NFR14、NFR40、NFR41、NFR43；对照 FR28/FR46（机械可编译）/ FR88 |
| 记录日期 | 2026-09-09 |
| 状态 | accepted — Story 42.1；关闭条件留给 Story 42.3 |

### (a) 上游约束

- **Epic 40 / FR94 已关闭：** Path B 字面绿合同翻转完成；ARCHITECTURE-SPINE **AD-27** 已修订：在继续合法满足 **FR28 / FR46**「机械可编译 + 端口/层次谓词」之外，**另增** **FR97** idiomatic / 可维护 Scala 验收面。**NFR41：** 本 epic 实现须引用修订后 AD-27。
- **既有机械路径（FR28 / FR46 / FR88）：** `emit_chisel` / 往返 = **可编译 ≠ idiomatic**；钉死 Chisel↔firtool 对（AD-9 / NFR12）；历史诚实声明仍约束机械面，**不得**因本 epic 开工而回滚机械回归。
- **FIRRTL→Scala Circuit 官方不支持现实（必读）：**
  - CIRCT 时代 Chisel **不**提供「`.fir` → idiomatic / 可维护 Scala Circuit」的官方产品路径；历史 Scala `Parser.parse` / `firrtl.Parser` API 已废弃（chipsalliance/chisel#4899）。
  - **AD-3 / AD-27：** 交换边界仍为 `.fir` + firtool；**不**要求恢复已删除的 Scala FIRRTL Parser。
  - FR97 因此是 **Bitloom 工具链自有合同**：自 FrozenHir/`.fir` **生成**可维护 idiomatic Chisel Scala（或文档等价往返），**不是**「等待上游恢复官方 FIRRTL→Scala」交差。
- **Idiomatic 验收条定义（本记录钉死；42.2 实现须可测）：** 至少覆盖下列之一组合，并在实现故事文档化具体谓词（风格黄金 / 结构断言 / 官方子集检查）：
  1. **命名：** 模块/端口/实例名可读且与 FrozenHir 公开名一致或有稳定映射；禁止无意义机械乱码名作为 FR97 通过标准。
  2. **结构：** 层次与 `Module` / `IO` / 连接组织接近手写 Chisel 可读结构（非单一巨型扁平语句堆）；实例图可映射。
  3. **可读性：** 生成 Scala 可供人类审阅维护（合理缩进、分组、注释可选）；不得以「仅能过 `sbt compile`」冒充可读。
  4. **官方风格子集（可选钉死）：** 若选用 Chisel 官方 Style Guide / 文档化子集，须在 42.2 文档写明所采纳条款与检查方式；未采纳则不得口头宣称「符合官方风格」。
- **机械面 vs FR97 完成面：**
  - **FR28/FR46** = 可编译 + 端口/层次谓词（机械风格合法）。
  - **FR97** = 超出机械面的 idiomatic / 可维护验收；**不得**仅以 FR28/FR46 已绿宣称 FR97 关闭。
- **NFR40：** idiomatic 往返为多年/高维护字面条之一；不得用短期「改文档重贴标签」假绿交差。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。

### (b) 粗工期带

- **预计：** Epic 42 整体约 **1.5–5 人周**（42.1 本风险记录 ≤0.25 人周；42.2 idiomatic 发射与验收 1–4 人周；42.3 收口/回归 0.25–0.75 人周）。置信度：**低–中**（idiomatic 谓词细节 Deferred 至本 epic；NFR40）。
- **假设：** 不回滚 FR28/FR46 机械路径；不要求上游恢复 `Parser.parse`；不并行冒充 Epic 43–47 关闭。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **42.2–42.3** 标 `ready` 或开工实现。
- **不得仅改文案 / 文档把机械 `emit_chisel` 重标为 idiomatic / FR97 已交付**，而无风格/结构/可读性自动化断言或可检查产物。
- **不得要求上游恢复已废弃 Scala `Parser.parse` / FIRRTL Parser，而不写明替代合同**（替代合同 = 本工具链生成 idiomatic Scala，或文档化的非 Parser 往返路径）；禁止把「等官方恢复」当作 FR97 完成面。
- 不得在未引用修订后 **AD-27** 时声称 FR97 实现合法（NFR41）。
- 不得静默降低 idiomatic 验收条至「能编译即可」而不改本记录 / PRD。
- 不得用 NFR10 调试用 HIR→源码再生冒充 FR97。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR41**（须引用修订 AD-27）共同责任人；**NFR40 / NFR43** 共同注意人
- 备份 / 升级路径：撤回 idiomatic 验收面 / 改回「仅机械满足」须升级至产品 / Correct Course；是否恢复对上游 Parser 依赖的争议升级至架构维护者（须先写替代或修订合同）。

---

### Idiomatic 验收条（摘要 · 供 42.2）

| 维度 | 最低期望 | 备注 |
| --- | --- | --- |
| 命名 | 可读 + 与公开 HIR 名稳定对应 | 禁乱码名过关 |
| 结构 | 可映射的 Module/IO/层次组织 | 非巨型扁平堆即可交差 |
| 可读性 | 人类可审阅维护 | 超出 `sbt compile` |
| 官方风格子集 | 可选；若宣称须文档钉死条款 | 未钉死不得口头宣称 |

### 机械 vs FR97（摘要）

| 路径 | 角色 | 可否单独关闭 FR97 |
| --- | --- | --- |
| 机械 `emit_chisel`（FR28/FR46） | 可编译 + 端口/层次谓词；继续回归 | **否** |
| Idiomatic / 可维护 Scala（FR97） | 超出机械的命名/结构/可读性（或官方子集） | **是**（须可验收） |

### FIRRTL→Scala 官方现实（摘要）

| 事实 | 对本 epic 含义 |
| --- | --- |
| 官方不支持 `.fir`→idiomatic Scala Circuit | FR97 ≠ 等待上游 |
| `Parser.parse` 已废弃（#4899） | 禁止「恢复 Parser」而无替代合同 |
| 交换边界 = `.fir` + firtool（AD-3/27） | Bitloom 自生成 idiomatic Scala |

### Epic 42 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **42.1** | 本 NFR14 风险记录 + ATDD | **本故事** |
| **42.2** | Idiomatic Chisel 发射与验收（FR97） | Gate：须本记录后才可 ready |
| **42.3** | FR97 收口与回归；勾选关闭 | Gate：须本记录后才可 ready |

### 并行 / 维护叠加（Chipyard 式 · NFR40 / NFR43）

- 可与 Epic 43–47 **并行规划**，但各自须独立 NFR14；本 epic 不得冒充其他字面条关闭。
- 文档面：`docs/fr28-chisel-compilable.md`（或等价）须区分机械 FR28/FR46 vs FR97 idiomatic；禁止混用「Chisel 已交付」话术。
- 回归：机械路径不得被本 epic 改写成「已废弃」或「不算完成」（Story 42.3 保留机械回归）。

### 引用

- AD-28 — 风险门禁（NFR14）
- AD-27（修订 2026-09-09）— 机械 FR28/FR46 + FR97 idiomatic
- AD-3 — FIRRTL 文本契约；无 Scala Parser 要求
- AD-9 / NFR12 — Chisel↔firtool 钉死对
- PRD NFR14 / FR97；NFR40 / NFR41 / NFR43
- 对照：FR28 / FR46 / FR88；chipsalliance/chisel#4899
- 前置：Epic 40 / FR94（`nfr14-risk-epic40-literal-path-b.md`）
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 42 关闭条件（Story 42.3 勾选）

- [ ] **FR97：** 至少一夹具满足本记录 idiomatic 验收条 + 自动化断言 — Story 42.2
- [ ] **文档：** 区分机械可编译 vs FR97 idiomatic；`docs/fr28-chisel-*.md`（或等价）与 deferred/README 更新 — Story 42.3
- [ ] **回归：** 既有机械 FR28/FR46 路径不回退 — Story 42.3
- [ ] **禁止事项未触发：** 无仅文档把机械标成 idiomatic；无「恢复 Parser」而无替代合同
- [ ] **品牌 / 依赖：** 仍为 Bitloom；设计 crate 只依赖 `bitloom-prelude`
- [ ] **NFR41：** 实现故事引用修订后 AD-27

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 42 故事 42.2–42.3 标 `ready`。**  
**FR97 完成面 = idiomatic / 可维护验收；机械 `emit_chisel` / FR28/FR46 不得单独关闭 FR97；不得把「恢复上游 Parser」当作完成定义。**
