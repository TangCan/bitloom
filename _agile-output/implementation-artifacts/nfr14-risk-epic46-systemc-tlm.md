# NFR14 风险记录 — Epic 46 SystemC TLM-2.0 产品路径（FR101）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 12 **NFR40 / NFR41 / NFR43**；交付 **FR101**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic45-formal-equiv-dual-model.md`；`nfr14-risk-dual-sim-generation.md`（FR47 ≠ TLM）。  
> **前置：** Epic 40 **closed**（FR94；Path B 字面绿）；Epic 45 **closed**（FR100/102/103 — 形式等价 / 双模型；**≠** TLM 产品）；修订后 **AD-5**（2026-09-09；允许 FR101；**NFR41**）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 46 后续故事 **46.2–46.3** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR101 / Epic 46；NFR14、NFR40、NFR41、NFR43；推翻 FR93「不承诺 TLM」；对照 FR47 / FR100 / AD-5 |
| 记录日期 | 2026-09-09 |
| 状态 | accepted — Story 46.1（门禁记录；产品实现 → 46.2；收口 → 46.3） |

### (a) 上游约束

- **Epic 40 / FR94 已关闭：** Path B 字面绿合同已开闸；本 epic 实现 FR93「不承诺 SystemC TLM」排除项 → **FR101**（SystemC TLM-2.0 **产品路径**）。
- **修订后 AD-5（NFR41 · 强制引用）：** ARCHITECTURE-SPINE AD-5（2026-09-09 Path B 修订）**允许**工具链交付 SystemC TLM-2.0 产品路径（FR101）；废止「不承诺 / 不要求 SystemC TLM-2.0」作为阻断 FR101 的依据。实现形状由本 epic 钉死。周期精确仍仅从 `FrozenHir` `tick`。Rust 功能模拟器（**FR47**）仍为合法功能视图，**≠** SystemC TLM。
- **FR47 / Rust 功能 sim（硬对照）：** 既有生成 Rust 功能模拟器 crate / 手写 `#[functional_model]` 是 AD-5 合法功能视图；**不得**把 host Rust FL 标成 SystemC TLM 或单独关闭 FR101。
- **FR100 / Epic 45（已关闭 · 隔离）：** 默认 TLM≡CA / 自动 FL≡RTL 形式等价属 **FR100**；关闭 Epic 45 **不**关闭 TLM **产品**。本 epic **不得**把形式等价口号并入 FR101 完成面，亦不得因交付 TLM 产品自动宣称 TLM≡CA。
- **IEEE / 工具现实：** TLM-2.0 为 IEEE 1666 SystemC 事务级接口（generic payload、`b_transport` / `nb_transport_*`、quantum / DMI 等）。工业 LT/AT 与 cycle-accurate 目标不同；混合靠 adapter / 共仿，非默认 bit-identical 锁步。本记录钉死 MVP 交付物形状，不要求全芯片 TLM≡CA 证明。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。SystemC/TLM 工具链依赖属工具链 / 示例 / 可选 CI，不得泄漏进设计 crate。
- **NFR40：** SystemC/TLM 工具链与示例维护为多年/高维护字面条；不得用文档口号或 Rust FL 假绿交差。

### (b) 粗工期带

- **预计：** Epic 46 整体约 **4–12 人周**（46.1 本风险记录 ≤0.25 人周；46.2 TLM-2.0 产品面 3–9 人周；46.3 文档/deferred 收口 0.5–2 人周）。置信度：**低–中**（SystemC 安装面、LT vs AT 深度、生成 vs 一等集成选型方差大；NFR40）。
- **假设：** 不回滚修订后 AD-5；不把 FR47 Rust FL 升格为 FR101 唯一完成面；不并行冒充 Epic 47 关闭；不要求本 epic 交付 TLM≡CA 形式证明（仍属 FR100）。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **46.2–46.3** 标 `ready` 或开工实现。
- **不得仅以文档口号 / README 宣称 / 空 stub 头文件关闭 FR101**（须可构建或可运行的 TLM-2.0 路径 + 钉死依赖/版本；见下方交付物）。
- **不得把 host Rust 功能模拟器 / FR47 生成 crate / `#[functional_model]` 标成 SystemC TLM 或单独关闭 FR101。**
- 不得在未改 PRD / 本记录的前提下把下方「TLM-2.0 交付物」四元（库/生成器/示例/工具链依赖）砍到「仅文档」却宣称 FR101 全绿。
- 不得因交付 TLM 产品自动宣称 **默认 TLM≡CA** / 关闭 FR100 口径已覆盖本 epic（FR100 已关；本 epic 是产品路径，不是形式证明）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。
- 不得以历史「AD-5 不承诺 TLM」旧读法阻断 FR101 而不引用 **修订后 AD-5**（违反 **NFR41**）。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR41**（实现 epic 须引用修订后 AD-5）共同责任人；**NFR40** / **NFR43** 共同注意人
- 备份 / 升级路径：改选「仅集成第三方 TLM 库、不自生成」或砍 LT/AT 任一支须升级至产品 / Correct Course；与 FR47 / FR100 边界冲突升级至 AD-5 / AD-28 维护者。

---

### TLM-2.0 交付物（本记录钉死 · 供 46.2 / FR101）

> 完成面 = 下列四元均可检查；**未列能力默认非目标**。静默砍到「仅文档」禁止。

| # | 交付元 | 钉死值（MVP 合同） |
| --- | --- | --- |
| **D1 · 库** | 可链接 / 可 `#include`（或等价）的 SystemC TLM-2.0 模型库或 Bitloom 发射的 TLM 侧工件，覆盖至少一条事务级接口（generic payload 或文档等价） | 46.2 须落盘路径 + 版本钉死 |
| **D2 · 生成器或一等集成** | 工具链**生成** TLM 侧代码，**或**文档化的一等集成路径（CLI/`cargo bitloom` 子命令或等价产品入口）把 Bitloom 设计接到 TLM 世界 | 二选一钉死；不得无入口宣称 FR101 |
| **D3 · 示例** | ≥1 可构建/可运行示例夹具（LT 或 AT 风格至少一条；见 L1–L2） | ATDD / CI 烟测可复现 |
| **D4 · 工具链依赖** | 文档钉死 SystemC / TLM 相关依赖与版本（或「CI 可选 + 本机安装说明」）；失败须可读（缺依赖时提示，不得静默假绿） | README / docs 交叉链 |

### LT / AT 范围（本记录钉死）

| # | 抽象 | MVP 义务 | 非目标（除非日后改本记录） |
| --- | --- | --- | --- |
| **L1 · LT** | Loosely-Timed：`b_transport` +（可选）quantum / temporal decoupling | **至少一条** LT 风格夹具 **或** 文档明确「MVP 仅 AT」并交付 AT | 全 SoC VP / 完整 OS 启动 |
| **L2 · AT** | Approximately-Timed：`nb_transport_fw/bw` 相位准确 | **至少一条** AT 风格夹具 **或** 文档明确「MVP 仅 LT」并交付 LT | 全协议相位完备 / 性能模型认证 |
| **L3 · 选择合同** | 46.2 实现时须**钉死**「LT-only / AT-only / LT+AT」并写入 docs | 未钉死却宣称双风格全绿 | |

**当前显式裁剪：** 允许 46.2 选择 **LT-only 或 AT-only** 作为 MVP，但须在产品文档写明；**不得**两者皆无。

### 与周期精确路径的关系（本记录钉死）

| 路径 | 权威语义 | 与 FR101 关系 |
| --- | --- | --- |
| **Cycle-accurate** | 仅 `FrozenHir` → `Sim::tick`（AD-5）；VCD 同源 | **保留**；TLM 产品**不替代** CA |
| **Rust FL（FR47）** | 手写 / 生成 Rust 功能视图；PortValues 对照（AD-17） | **合法并行**；**≠** SystemC TLM |
| **SystemC TLM（FR101）** | IEEE 1666 TLM-2.0 事务级产品路径（本 epic） | **本完成面** |
| **FL≡RTL / TLM≡CA 形式证明（FR100）** | Epic 45 已关闭 | **不**因 FR101 自动重开或冒充；adapter/共仿可作配套，**alone ≠** 形式证明 |

### FR47 / FR100 vs FR101（摘要）

| 路径 | 角色 | 可否单独关闭 FR101 |
| --- | --- | --- |
| FR47 生成 / 手写 Rust FL | 功能视图 | **否** |
| 仅 README / 口号 / 空 stub | 文档 | **否** |
| FR100 形式等价产品 | FL≡RTL（已关） | **否**（不同 FR） |
| 本记录 D1–D4 + L1/L2 + ATDD | TLM-2.0 产品 | **是**（须 46.2） |

### Epic 46 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **46.1** | 本 NFR14 风险记录 + ATDD | **本故事** |
| **46.2** | SystemC TLM-2.0 生成或集成产品面（FR101；D1–D4 + L1/L2） | Gate：须本记录后才可 ready |
| **46.3** | FR101 收口；撤销「不承诺 TLM」；区分 Rust FL vs SystemC TLM | Gate：须本记录后才可 ready |

### 并行 / 维护叠加（Chipyard 式 · NFR40 / NFR43）

- 可与 Epic 47 **并行规划**，但各自须独立 NFR14；本 epic 不得冒充波形 / 覆盖率字面条关闭。
- `nfr14-risk-dual-sim-generation.md` 历史「不承诺 TLM」叙述与修订后 AD-5 / 本记录须同一叙事：FR47 仍钉 Rust；FR101 另开 TLM 产品腿。
- SystemC 安装面与 CI 可选性：缺依赖失败须可读；禁止「本机有库才绿、CI 静默 skip 却宣称 FR101」。

### 引用

- **AD-5（修订后 · 2026-09-09）** — 允许 SystemC TLM-2.0 产品路径（FR101）；NFR41
- AD-28 — 风险门禁（NFR14）
- AD-6 / AD-17 — prelude-only；PortValues 对照面
- PRD NFR14 / FR101；NFR40 / NFR41 / NFR43；推翻 FR93「不承诺 TLM」
- 对照：FR47；FR100；`nfr14-risk-dual-sim-generation.md`；`nfr14-risk-epic45-formal-equiv-dual-model.md`
- 前置：Epic 40 / FR94；Epic 45 / FR100–103 closed
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 46 关闭条件（Story 46.3 勾选 · 本故事不勾）

- [ ] **46.2 / FR101：** D1–D4 交付物 + L1/L2（LT 与/或 AT）+ 可复现烟测 — Story 46.2
- [ ] **文档 / deferred / doc-19：** 撤销「不承诺 SystemC TLM-2.0」作为产品完成排除项；明确区分 Rust FL（FR47）vs SystemC TLM（FR101）— Story 46.3
- [ ] **禁止事项未触发：** 无仅文档口号关闭；无 host Rust FL 冒充 TLM — Story 46.3
- [ ] **修订后 AD-5 / NFR41：** 实现与文档交叉引用修订后 AD-5 — Story 46.2 / 46.3
- [ ] **品牌 / 依赖：** 仍为 Bitloom；设计 crate 只依赖 `bitloom-prelude` — Story 46.3

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 46 故事 46.2–46.3 标 `ready`。**  
**FR101 完成面 = SystemC TLM-2.0 产品路径（库/生成器或一等集成/示例/工具链依赖 + LT/AT 范围）+ ATDD；不得以文档口号或 host Rust FL（FR47）单独关闭。**  
**须引用修订后 AD-5（NFR41）。周期精确路径仍仅 FrozenHir `tick`；本 epic 不替代 CA，亦不自动关闭 TLM≡CA 形式证明。**
