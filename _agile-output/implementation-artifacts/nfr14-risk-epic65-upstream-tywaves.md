# NFR14 风险记录 — Epic 65 上游 Tywaves 一等集成（FR125）

> **权威：** PRD NFR14；AD-28；Phase 15 **NFR52 / NFR53 / NFR55**；交付 **FR125**。  
> **前置：** Epic 64 **closed**（FR124）；Epic 58 **closed**（FR117 自研 typed 子集 B）。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **65.2–65.3** 标 `ready`。  
> **隔离：** ≠ FR104 alone；≠ FR114 alone；≠ FR117 `typed-wave.html` alone；≠ docs-only。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR125 / Epic 65；对照 FR117 / FR104 / FR114 |
| 记录日期 | 2026-09-10 |
| 状态 | **closed** — Story **65.3**（Epic 65 / FR125 收口；T1–T4 已交付） |
| **选定加深子集** | **上游 Tywaves 一等集成形状（T1–T4）** |

### (a) 上游约束

- Epic 64 / FR124 已关闭；Phase 12–14 关闭仍有效（NFR52）。
- FR117 自研 `typed-wave.html` / `wave.typed.json` **仍有效**；alone ≠ FR125。
- 设计 crate 只依赖 `bitloom-prelude`；Tywaves 运行时 **不**进入设计依赖。
- 品牌 Bitloom / `bitloom-*`。

### (b) 粗工期带

- Epic 65：约 **1–4 人周**（65.1 ≤0.25；65.2 1–3；65.3 ≤0.75）。置信度：中。

### (c) 禁止的静默降级清单

- 不得缺本记录将 65.2–65.3 标 ready。
- **不得仅 FR104 `interactive.html` 关闭 FR125。**
- **不得仅 FR114 LCOV/`coverage.html` 关闭 FR125。**
- **不得仅 FR117 自研 typed-wave 关闭 FR125。**
- **不得 docs-only。**
- 缺上游/元数据不得 silent 宣称 FR125 绿。
- 不得改写 FR104/114/117「已关闭」为失败（NFR52）。
- 不得冒充 NFR14-crates。

### (d) 负责人

- Richard（Dev）— NFR14 / NFR52 / NFR55。

---

### 选定集成形状（T1–T4）

| ID | 钉死 |
| --- | --- |
| **T1 契约** | `cargo bitloom wave --tywaves` 写出上游可消费 sidecar：`wave.tywaves.json`（schemaVersion + Bitloom brand + typed signals + samples）及 `tywaves.launch.sh` |
| **T2 版本/入口** | 环境变量 `BITLOOM_TYWAVES_BIN` 钉死上游 viewer 可执行文件路径（文档允许 stub）；未设置时仍写出 sidecar，但 launch 路径失败可读 |
| **T3 失败语义** | `BITLOOM_TYWAVES_FORCE_MISSING=1` 或 launch 目标不存在 → **非零**退出且消息含 `bitloom.tywaves`；禁止 silent 绿 |
| **T4 验收** | ATDD：sidecar 含 `data-bitloom-tywaves` / schema 字段；≠ 仅有 `typed-wave.html`；FR117 工件仍发射（NFR52） |

### 未列入（NFR55）

- IDE 插件商店发布；完整 ChiselSim 耦合；替换默认 VCD/`typed-wave.html`。

### Epic 65 关闭条件（65.3 勾选）

- [x] **65.2 / FR125：** T1–T4 产品路径 + ATDD
- [x] **文档 / deferred / README**
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖：** Bitloom / bitloom-prelude
- [x] **FR117 关闭仍有效**
