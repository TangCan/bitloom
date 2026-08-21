# NFR14 风险记录 — 双模拟器生成（Epic 21 / FR47 + FR30）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（Phase 7 风险门禁）。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 21 后续故事 **21.2–21.5** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR47 + FR30（及手写回归 FR29 作为生成基座）/ Epic 21 |
| 记录日期 | 2026-08-21 |
| 状态 | accepted |

### (a) 上游约束

- **功能模拟器形态（PRD Open Q6 已关闭）：** 工具链须 **生成 Rust 功能模拟器 crate**；**不**强制 / **不**以 SystemC TLM-2.0 为合同或交付物（AD-5）。
- **双模型值与对照面（AD-17）：** 功能视图与周期精确视图的对照只比较 `PortValues`；周期精确路径仍只从 `FrozenHir` `tick`（AD-5）。
- **工程现实（addendum）：** 工业 TLM↔RTL 自动等价极少；FR47/FR30 字面「生成双模拟器 + 产品等价」完成成本高——须按工期带规划，不得用「同业未做满」永久豁免（PRD 反指标）。
- **既有手写基座：** 阶段二已有 / 可恢复的 `#[bridge]` / `#[abstraction]` / mixed `both` 与 `PortValues` 对照（FR29）；生成路径建立其上，但 **不得**用「仅手写对照绿」冒充 FR47/SM-7 完成。
- **设计依赖边界：** 设计 crate 仍只依赖 `bitloom-prelude`；生成器属工具链 / CLI crate。

### (b) 粗工期带

- **预计：** Epic 21 整体约 **3–5 人周**（21.1 风险记录 ≤0.25 人周；21.2 手写回归整理/ATDD 0.25–0.5 人周；21.3 功能模拟器生成 1–1.5 人周；21.4 周期精确工件 + 桥接对照 1–1.5 人周；21.5 FR30 接入生成路径 0.5–1 人周；缓冲含文档与故意不一致 fail 夹具）。
- **置信度 / 假设：** 中；假设沿用既有 `tick` / equiv / PortValues，且功能生成以 Rust crate 为唯一形态。若中途要求 SystemC 或全自动形式证明级 EC，工期显著上修。

### (c) 禁止的静默降级清单

- 不得删除 / 绕过 **生成路径**，改回「仅手写 functional / bridge 对照」并宣称 FR47 或 SM-7 完成，而不改 PRD 与脊柱。
- 不得宣称 **SystemC TLM**（或 TLM-2.0）已交付，或以 SystemC 替代「生成 Rust crate」而不改 PRD Open Q6 / AD-5。
- 不得把 **FR30** 收口验收永久停在「仅手写模型对 tick」，而不接入 FR47 生成产物路径。
- 不得以「工业无 TLM↔RTL 自动等价」为由把 FR47/FR30 永久标 unsupported / 尽力失败。
- 不得在无本记录（或缺 a–d）时将 **21.2–21.5** 标 `ready` 或开工实现。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）
- 备份 / 升级路径：架构争议升级至 AD-5 / AD-17 / AD-28 维护者；生成器 API 形态变更须先改本记录与 PRD 再动实现。

### 功能模拟器形态（Open Q6）

| 形态 | 说明 | 本记录 |
| --- | --- | --- |
| 生成 **Rust** 功能模拟器 crate | PRD FR47 success；Open Q6 已关闭 | **钉死** |
| SystemC TLM-2.0 | 非合同；AD-5 明确不承诺 | **禁止冒充交付** |

### 并行 / 维护叠加（Chipyard 式）

- 与 Epic 20/22–24 并行时：生成器 + 手写多视图 + equiv 回归面叠加；升钉 / 语言表面变更须同步手写夹具与生成黄金夹具，避免「只修一侧」。
- 语义最低公分母：生成功能 crate 的事务语义须与手写 `#[functional_model]` / `PortValues` 对照约定一致；禁止各 epic 各定一套 join 类型。
- 禁止把生成路径失败归因到「手写 FR29 未就绪」而跳过 FR47 验收（21.2 为基座回归，不豁免生成腿）。

### 引用

- AD-28 — Phase 7 风险门禁（NFR14）
- AD-5 — 双模型仿真（允许生成 Rust 功能模拟器；不承诺 SystemC TLM-2.0）
- AD-17 — PortValues 对照面
- PRD NFR14（`prd-rhdl-2026-08-19/prd.md` §6）
- PRD FR47 / FR30 / FR29；Open Q6 已关闭（生成 Rust crate）
- Addendum：TLM↔RTL 成本；FR47 实现草图
- 历史别名消歧：**NFR14-crates**（crates.io FCFS）≠ 本门禁 **NFR14**

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 21 故事 21.2–21.5 标 `ready`。**
