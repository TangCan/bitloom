# NFR14 风险记录 — Epic 32 Bundle 嵌套与 derive（FR80 / AD-20）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；**AD-20**（`Bundle`/`Vec` 允许可综合路径，宽/向 emit 前失败）；Phase 10 **NFR37**（规划 done ≠ 深度 done）。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **历史最小合同：** Epic 19 / **FR51** — 文档化 `Bundle` / `HwVec`（或等价）进入可综合路径；**ground 叶子** flatten 为标量 HIR 端口；位宽/方向不匹配 emit 前失败。prelude 明确 **nested Bundle / `HwVec<Bundle,_>` / `#[derive(Bundle)]` = OUT OF SCOPE (MVP)**。  
> **体例对照：** `nfr14-risk-epic31-cdc-true-rtl.md`；`nfr14-risk-epic30-bridge-adapter-closures.md`；`nfr14-risk-epic34-ip-baseline.md`。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 32 后续故事 **32.2–32.4** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR80（加深 FR51 / AD-20）/ Epic 32；NFR14、NFR37 |
| 记录日期 | 2026-09-09 |
| 状态 | accepted — 门禁有效；Epic 32 关闭条件待 Story 32.4 勾选 |

### (a) 上游约束

- **AD-20 / FR51 现状（诚实基线）：** `Bundle` 与 `HwVec`（文档等价 `Vec<T,N>`）**允许**进入可综合路径；实现以 ground 叶子 flatten（`{field}_{member}` / `{field}_{i}`）为主，**不**要求公开 HIR 扩展 Bundle 节点。宽/向不匹配须在 emit 前失败。这是 **FR51 最小合同**，sprint 可将 Epic 19 标 `done`，但 **≠** FR80 嵌套/derive 深度（NFR37）。
- **prelude 文档边界（强制承认）：** FR51 / Epic 19 时 `bitloom-prelude` 写明 nested `Bundle` 成员、`HwVec<Bundle, _>`、以及 `#[derive(Bundle)]` 均 **OUT OF SCOPE (MVP)**。**Story 32.2 后：** 至少一层嵌套经 `Bundle::nested_bundles` 已进入可综合路径（FR80）。**Story 32.3 后：** `#[derive(Bundle)]` 经 prelude 可用（支持/限制见 language-surface；稳定 `rhdl::E0180`）。`HwVec<Bundle,_>` 与 ≥2 层递归仍 OUT OF SCOPE / defer（32.4 文档收口）。历史 OUT OF SCOPE 注释**不是** FR80 完成证明；亦不得仅靠删除注释假装已支持嵌套（NFR37）。
- **嵌套深度上限假设（本 epic 钉死，留给 32.2–32.4 兑现）：**
  - **合同下限：** 至少 **一层**文档化嵌套 Bundle（子 Bundle 作为父 Bundle 成员）可 elaborate → emit `.v` → tick（或文档等价）（Story 32.2 / FR80）。
  - **本 epic 默认上限：** 验收与文档以 **一层嵌套**为主路径；**更深嵌套（≥2 层）** 若未在 32.2–32.4 实现，须书面列为延期/非目标，**不得**静默声称「任意深度」。
  - **derive：** Story 32.3 提供 `#[derive(Bundle)]` 或文档等价；须文档化支持/限制集合；不支持的嵌套/字段形态有稳定诊断。
  - **继承：** 嵌套字段宽/向不匹配仍须 emit 前失败（FR8 / FR51 精神）。
- **与 FR51 最小合同的区别（强制写清）：** FR51 = flatten-only ground leaves + 宽/向门控 + nested OUT OF SCOPE。FR80 = 嵌套层次可表达（至少一层）且/或 derive，**不再**仅以 OUT OF SCOPE / flatten-only 交差。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**；不得让设计 crate 依赖 CLI 包 `bitloom`。
- **NFR37：** 规划/sprint `done` ≠ 深度 done；深度缺口以 **FR80** 验收；不得用 Epic 19 / FR51「done」关闭嵌套深度。

### (b) 粗工期带

- **预计：** Epic 32 整体约 **1–2.5 人周**（32.1 本风险记录 ≤0.25 人周；32.2 一层嵌套 Bundle → HIR → emit 0.5–1 人周；32.3 derive 或文档等价 0.5–1 人周；32.4 ATDD + 文档限制表 0.25–0.5 人周）。
- **置信度 / 假设：** 中；假设沿用既有 flatten / `ElaborateSession` / emit / `Sim::tick`，且「嵌套」可先展开为更深命名叶子而不强制公开 HIR Bundle 节点。若要求任意深度、`HwVec<Bundle,_>` 全家桶、或 FIRRTL 级 Bundle 保留到后端，工期显著上修，须改本记录而非静默缩范围。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **32.2–32.4** 标 `ready` 或开工实现。
- **不得仅删除（或改写）prelude「OUT OF SCOPE」注释 / README 而无嵌套实现**，并宣称 FR80 / 嵌套 Bundle 已完成（NFR37）。
- 不得把 FR51 flatten-only / Epic 19「done」冒充 FR80 深度关闭（**NFR37：规划 done ≠ 深度 done**）。
- 不得只交付 derive 宏而无一层嵌套路径（或反过来只改文档声称 derive），却宣称 FR80 全完成，而不改 PRD / 本记录。
- 不得静默把「至少一层」缩成「仍仅 ground leaves、无嵌套类型表面」，或把深度上限偷换成未文档化的任意深度验收。
- 不得弱化嵌套字段宽/向 emit 前失败来换取「看起来能编过」。
- 不得让设计 crate 依赖非 `bitloom-prelude` 的工具链包交差（AD-6）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR37** 深度诚实度共同责任人
- 备份 / 升级路径：嵌套深度上限或「一层 vs 任意深度」争议升级至 ARCHITECTURE-SPINE（AD-20）维护者；与 FR51 历史话术冲突升级至产品 / Phase 10 排程负责人。

### FR51 最小合同 vs FR80 深度（对照）

| 维度 | Epic 19 / FR51（AD-20） | Epic 32 / FR80 |
| --- | --- | --- |
| 完成话术 | ground-leaf Bundle/`HwVec` flatten + 宽/向 emit 前失败 | **至少一层**嵌套 Bundle 可综合路径 +/或 derive |
| prelude | nested / derive **OUT OF SCOPE** | nested 不再仅以 OUT OF SCOPE 交差；derive 或文档等价 |
| 深度 | 扁平聚合 | 合同下限一层；更深须文档钉死或书面非目标 |
| 诚实度 | sprint 可 `done`（最小合同） | **不得**用该 done 关闭深度（NFR37） |

### 并行 / 维护叠加（Chipyard 式）

- 与 Epic 31（CDC）/ Epic 34（IP）并行时：嵌套 Bundle 是语言表面深度，不得与 CDC/IP 完成话术混用。
- 与 Phase 9 闭包 epic 并行时：嵌套/derive **不得**引入捕获闭包进 `tick`（AD-18 / NFR35）。
- 升钉 flatten 命名或宽/向诊断时须同步嵌套正/负例夹具，禁止「只修扁平 Bundle、文档仍写嵌套已支持」。

### 引用

- AD-28 — 风险门禁（NFR14）
- AD-20 — Bundle/Vec 可综合；宽/向 emit 前失败；与 FR22 边界
- AD-6 — 设计 crate 只依赖 `bitloom-prelude`
- PRD NFR14 / FR51 / FR80；NFR37（规划 done ≠ 深度 done）
- `bitloom-prelude` — `Bundle` / `HwVec`；FR51 曾 nested/derive OUT OF SCOPE；32.2 一层 `nested_bundles`；32.3 `#[derive(Bundle)]` via prelude；`HwVec<Bundle,_>` 仍 OOS
- 体例：`nfr14-risk-epic31-cdc-true-rtl.md`；`nfr14-risk-epic34-ip-baseline.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 32 关闭条件（Story 32.4 勾选）

- [ ] **FR80 嵌套：** 至少一层文档化嵌套 Bundle → elaborate → emit `.v` → tick（或文档等价）
- [ ] **FR80 derive（或等价）：** 文档化支持/限制；正例可综合；不支持形态有稳定诊断
- [ ] **宽/向负例：** 嵌套字段不匹配仍 emit 前失败（继承 FR51）
- [ ] **NFR37：** 相对 FR51 / OUT OF SCOPE 历史已文档化；不得用最小合同或仅删注释冒充深度关闭
- [ ] **ATDD / 配方：** 嵌套正例 + 宽/向负例（亦含于 `just test` 或文档化配方）
- [ ] **禁止事项未触发：** 无仅删除 OUT OF SCOPE 注释交差；无静默砍掉嵌套或 derive 却宣称 FR80 全完成

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 32 故事 32.2–32.4 标 `ready`。**  
**Epic 32 关闭条件（上节）待 Story 32.4 勾选。**
