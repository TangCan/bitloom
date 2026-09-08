# NFR14 风险记录 — Epic 30 桥接适配器闭包模板（FR78）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）。  
> **模板：** `nfr14-risk-record-template.md`（Story 26.1 / 19.1）。  
> **Phase 9 总览：** `nfr14-risk-phase9-closures.md`（Epic 26.1；本记录为 Epic 30 专用收紧）。  
> **双视图基座：** `nfr14-risk-dual-sim-generation.md`（Epic 21 / FR47）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 30 后续故事 **30.2–30.4** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR78 / Epic 30；NFR14、NFR36；衔接 FR47 / FR16 |
| 记录日期 | 2026-09-08 |
| 状态 | accepted — Epic 30 开工门禁（Story 30.1）；关闭条件待 30.4 |

### (a) 上游约束

- **依赖 Epic 27（FR73）：** 桥接适配器闭包模板建立在 elaborate-time 生成器闭包通路上；无 Epic 27 不得假装 FR78 完成。
- **依赖 FR47 双视图路径：** 模板须与既有 `generate_*` / bridge / PortValues 对照路径联验（Story 30.3）；不得另起一套事务语义。功能模拟器形态仍为 **生成 Rust crate**（AD-5）；**不**以 SystemC TLM-2.0 为合同。
- **AD-18 / FR16 / NFR35：** 周期精确 / `tick` 路径继续拒绝**捕获闭包**与未消解的 Rust 闭包对象；允许的是 elaborate-time **非捕获** `Fn`，且必须在 `freeze` 前消解为 HIR。本 epic **不得**把「功能视图自由闭包」写成周期精确路径也允许捕获。
- **NFR36 / Cap-R-58：** freeze 后 FrozenHir / Verilog / FIRRTL / Chisel / tick **无**闭包残留；后端不编码闭包 IR。周期精确侧仅见模板消解后的**信号级接口**（Cap-R-65…68）。
- **视图边界（强制）：**
  - **功能视图：** 可继续使用自由闭包发事务 / 激励（Cap-R-66/67）。
  - **周期精确侧：** 仅经 `start_wait_complete`（或文档等价）模板消解后的普通信号握手；**不得**把功能侧闭包对象带进 `tick`。
- **TLM↔信号泄漏风险：** 事务级（功能/桥接）闭包若未在进入周期精确路径前消解，会把 TLM/事务语义泄漏为「假信号级」或闭包对象进 HIR/`tick`——视为合同失败，须诊断或联验 fail，不得 silent 成功。
- **品牌 / 依赖：** 公开产品 **Bitloom**；设计 crate 只依赖 **`bitloom-prelude`**（AD-6）。

### (b) 粗工期带

- **预计：** Epic 30 整体约 **0.5–1.5 人周**（30.1 本风险记录 ≤0.25 人周；30.2 桥接模板 API 0.25–0.75 人周；30.3 FR47 联验 0.25–0.5 人周；30.4 文档收口 ≤0.25 人周）。
- **置信度 / 假设：** 中；假设 Epic 27 生成器通路与 Epic 21 FR47 生成/对照路径可用；主要不确定项是模板 API 落点（prelude vs 验证辅助层）与 TLM↔信号边界文档是否被误读为「周期侧也可自由闭包」。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **30.2–30.4** 标 `ready` 或开工实现。
- 不得让功能视图**自由闭包**泄漏进周期精确 / `tick` / FrozenHir（**TLM↔信号泄漏**）；周期侧须仅为消解后信号级接口。
- 不得把桥接**模板误用**为 comb/seq 可综合闭包（FR74/FR75）或生成器 LUT 闭包（FR73）的完成定义；模板职责是事务↔周期握手，不是硬件内联。
- 不得与 **FR16** 混淆：不得宣称「Epic 30 允许捕获闭包进 tick」；周期精确路径捕获禁令仍在（AD-18 / NFR35）。
- 不得把 **FR47** sim generators / Phase 7「闭环」冒充 **FR78** 桥接适配器闭包模板完成定义。
- 不得以 SystemC TLM-2.0 替代 Rust 功能路径或宣称 TLM 已交付（AD-5）。
- 不得在 FIRRTL / Chisel / Verilog / tick 中残留闭包语义或编码闭包节点（NFR36）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）
- 备份 / 升级路径：视图边界 / FR16 语义争议升级至 ARCHITECTURE-SPINE（AD-18）维护者；FR47 联验争议升级至双视图 / AD-5·AD-17 维护者。

### 视图与泄漏边界摘要（实现指针）

| 视图 / 路径 | 允许的闭包 | 进入周期精确侧之前 | 禁止 |
| --- | --- | --- | --- |
| 功能视图 / 桥接激励 | 自由 Rust 闭包（不进 FrozenHir） | 经模板消解为信号时序 | 闭包对象进 `tick` |
| 桥接适配器模板（FR78） | `start_wait_complete`（或等价）展开 | 输出普通信号握手 | 冒充 FR73/FR75 硬件闭包 |
| 周期精确 / `tick` | 无闭包对象（仅信号） | N/A | 捕获闭包（FR16）；未消解 Fn |
| emit / 后端 | 无闭包 IR（NFR36） | N/A | FIRRTL/Chisel「闭包节点」 |

### 并行 / 维护叠加（Chipyard 式）

- 与 Epic 29（HLS/IP 闭包）及 Phase 10（Epic 31–35）并行时：文档须区分 **功能自由闭包** vs **桥接模板** vs **SynthesizableClosure** vs **生成器闭包**；避免「闭包」一词混成同一完成定义。
- 与 FR47 生成/对照夹具叠加时：模板联验失败不得 `continue-on-error` 假装绿；故意不一致须 fail（衔接 FR30 精神）。
- FR16 负例矩阵须与桥接正例共存回归，防止「模板可用」冲掉捕获禁令。

### 引用

- AD-28 — 风险门禁（NFR14）
- AD-18（Revised）— 禁捕获；elaborate-time 非捕获须冻前消解
- AD-5 / AD-17 — 双视图；PortValues；不承诺 SystemC TLM
- PRD NFR14 / FR78 / NFR36；FR47 / FR16 / NFR35
- Cap-R-65…68 — 桥接适配器闭包模板
- Phase 9 总览：`nfr14-risk-phase9-closures.md`
- 双视图：`nfr14-risk-dual-sim-generation.md`
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 30 关闭条件（Story 30.4 勾选）

- [ ] **FR78：** `start_wait_complete`（或等价）模板 API 文档化可用
- [ ] **视图边界：** 功能侧自由闭包；周期侧仅消解后信号（无闭包对象进 tick）
- [ ] **FR47 联验：** 模板驱动夹具与生成/对照路径一致；故意破坏则 fail
- [ ] **NFR36 / FR16：** emit/HIR 无闭包 IR；捕获闭包负例仍失败
- [ ] **禁止事项未触发：** 无 TLM↔信号泄漏静默成功；无 FR47 冒充 FR78；无 SystemC 冒充交付

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 30 故事 30.2–30.4 标 `ready`。**  
**Epic 30 关闭条件（上节）待 Story 30.4 勾选。**
