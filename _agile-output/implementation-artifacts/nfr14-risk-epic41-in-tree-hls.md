# NFR14 风险记录 — Epic 41 树内 HLS 调度与闭包变换（FR95 / FR96）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；**AD-25**（修订后：树内+外挂）；**AD-18**（闭包溶解）；Phase 12 **NFR40 / NFR41 / NFR43**；交付 **FR95 / FR96**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic40-literal-path-b.md`；`nfr14-risk-hls.md`；`nfr14-risk-epic37-interop-hls.md`。  
> **前置：** Epic 40 **closed**（FR94；`65b4a37` 修订 AD-5/25/27）；Path B 字面绿合同已开闸。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 41 后续故事 **41.2–41.4** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR95 / FR96 / Epic 41；NFR14、NFR40、NFR41、NFR43；对照 FR35/FR86（外挂诚实路径） |
| 记录日期 | 2026-09-09 |
| 状态 | closed — Story 41.4；Epic 41（FR95/FR96）已关闭 |

### (a) 上游约束

- **Epic 40 / FR94 已关闭：** Path B 字面绿合同翻转完成；ARCHITECTURE-SPINE **AD-25** 已修订：允许 bitloom/rhdl crate 实现树内 `#[hls]`（或等价）**scheduling/allocation** 作为 **FR95** 产品主路径之一；外挂可保留为可选/对照，但**不得单独**满足 FR95。**NFR41：** 本 epic 实现须引用修订后 AD-25。
- **既有外挂路径（FR35 / FR86）：** 发射宿主 IR/C + **Bambu**（钉死后端；CI 默认 stub；真机 `BITLOOM_HLS_USE_REAL=1`）仍为诚实可选/对照路径。Epic 37 Path B 已钉「stub ≠ HLS 质量」。
- **树内调度与外挂路径共存策略：**
  - **FR95 完成面** = 树内调度 MVP：带 `#[hls]` 的设计在**不调用**外挂 Bambu 的情况下产出可检查的调度结果或下游 RTL/IR（Story 41.2）。
  - **外挂路径** = 保留为可选/对照/FR35 诚实接线；可用于回归「外挂仍可用」，**不得**单独勾选 FR95 done。
  - 文档须同时写明两条路径的角色；禁止用「Bambu 仍可跑」话术关闭字面 FR95。
- **调度 / allocation 进入 crate 的范围边界：**
  - **允许进入** bitloom/rhdl crates：文档化子集的 scheduling / allocation（pipeline 或 loop-unroll 等可演示构造）、以及调度前闭包数据流变换（FR96）。
  - **不在本 epic 默认可综合语义内：** Handshake / 未合同的动态数据流作为默认可综合 RTL 语义（AD-25 Prevents）。
  - 调度结果须可经 elaborate/emit 或文档化 IR 产物验收；禁止「仅有宏/属性表面、无调度产物」交差。
- **闭包变换（FR96）与 AD-18 溶解规则关系：**
  - FR96：调度前将闭包作为数据流变换单元**内联**。
  - AD-18：**允许** elaborate-time **非捕获** `Fn`（或等价），**必须在 `freeze` 前消解**为 HIR；**禁止**捕获闭包 / 未消解闭包对象进入周期精确/`tick`。
  - Epic 41 实现须把 FR96 内联落在 AD-18 溶解边界内；负向：捕获闭包在 freeze 前失败可读（Story 41.3）。
- **NFR40：** 树内 HLS 为多年/高维护字面条之一；不得用短期 stub 假绿交差。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。

### (b) 粗工期带

- **预计：** Epic 41 整体约 **2–6 人周**（41.1 本风险记录 ≤0.25 人周；41.2 树内调度 MVP 1–3 人周；41.3 闭包数据流变换 0.75–2 人周；41.4 收口/回归 0.25–0.75 人周）。置信度：**低–中**（调度器实现形状未钉；NFR40）。
- **假设：** 不回滚 FR35 外挂诚实路径；不把 Handshake 默认语义塞进本 epic；不并行冒充 Epic 42–47 关闭。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **41.2–41.4** 标 `ready` 或开工实现。
- **不得仅改文档声称树内 HLS / FR95 已交付**，而无可检查的树内调度产物或 ATDD。
- **不得把 stub / `BITLOOM_HLS_USE_REAL` 外挂路径标成 FR95 done**（外挂可保留，但不得单独满足 FR95）。
- **不得 silent 扩大到未合同的动态数据流 / Handshake 默认语义**（AD-25）。
- 不得在未引用修订后 **AD-25** 时声称 FR95/FR96 实现合法（NFR41）。
- 不得把捕获闭包或未消解闭包对象送入 `tick`（AD-18）；不得用 FR72–78 表面冒充 FR96 已关。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR41**（须引用修订 AD-25）共同责任人；**NFR40 / NFR43** 共同注意人
- 备份 / 升级路径：撤回树内主路径 / 改回「仅外挂满足 FR95」须升级至产品 / Correct Course；AD-18 溶解边界争议升级至架构维护者。

---

### 树内 ↔ 外挂共存（摘要）

| 路径 | 角色 | 可否单独关闭 FR95 |
| --- | --- | --- |
| 树内 `#[hls]` scheduling/allocation | **FR95 产品主路径之一**（Epic 41） | **是**（须可验收） |
| 外挂 Bambu（stub / `BITLOOM_HLS_USE_REAL`） | 可选/对照 / FR35·FR86 诚实路径 | **否** |

### FR96 ↔ AD-18（摘要）

| 规则 | 含义 |
| --- | --- |
| FR96 | 调度前闭包 → 数据流变换单元内联 |
| AD-18 | 非捕获 elaborate-time `Fn` 须 freeze 前消解入 HIR；捕获闭包禁入 tick |
| 本 epic | 41.3 实现须同时满足两者；负向可读失败 |

### Epic 41 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **41.1** | 本 NFR14 风险记录 + ATDD | **本故事** |
| **41.2** | 树内 `#[hls]` 调度 MVP（FR95） | Gate：须本记录后才可 ready |
| **41.3** | HLS 闭包数据流变换（FR96） | Gate：须本记录后才可 ready |
| **41.4** | FR95/FR96 收口与回归；勾选关闭 | Gate：须本记录后才可 ready |

### 并行 / 维护叠加（Chipyard 式 · NFR40 / NFR43）

- 可与 Epic 42–47 **并行规划**，但各自须独立 NFR14；本 epic 不得冒充其他字面条关闭。
- 文档面：`docs/fr35-hls.md`（或等价）须区分外挂诚实路径 vs 树内 FR95 完成面；禁止混用「HLS 已交付」话术。
- 回归：外挂路径不得被改写成「唯一完成定义」（Story 41.4）。

### 引用

- AD-28 — 风险门禁（NFR14）
- AD-25（修订 2026-09-09）— 树内 FR95 + 外挂可选
- AD-18 — 闭包溶解 / 禁捕获入 tick
- PRD NFR14 / FR95 / FR96；NFR40 / NFR41 / NFR43
- 对照：FR35 / FR86；Epic 37 stub / `BITLOOM_HLS_USE_REAL`
- 前置：Epic 40 / FR94（`nfr14-risk-epic40-literal-path-b.md`）
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 41 关闭条件（Story 41.4 勾选）

- [x] **FR95：** 树内调度 MVP 可验收且不依赖外挂 Bambu — Story 41.2（`f4257b2`）
- [x] **FR96：** 调度前闭包内联遵守 AD-18 — Story 41.3（`bb8b256`）
- [x] **回归 / 文档：** 外挂诚实路径不回退为「唯一完成定义」；README/deferred 不再把树内 HLS 列为永久非目标 — Story 41.4
- [x] **禁止事项未触发：** 无仅文档假交付；无 stub/`BITLOOM_HLS_USE_REAL` 冒充 FR95；无 silent 动态数据流默认
- [x] **品牌 / 依赖：** 仍为 Bitloom；设计 crate 只依赖 `bitloom-prelude`
- [x] **NFR41：** 实现故事引用修订后 AD-25

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 41 故事 41.2–41.4 标 `ready`。**  
**FR95 完成面 = 树内调度；外挂 Bambu / stub / `BITLOOM_HLS_USE_REAL` 不得单独关闭 FR95。**
