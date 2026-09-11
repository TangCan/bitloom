# NFR14 风险记录 — Epic 73 真实上游 Tywaves GUI / IDE 插件深度（FR134）

> **权威：** PRD NFR14；AD-28；Phase 16 **NFR56 / NFR57 / NFR59**；交付 **FR134**。  
> **前置：** Epic 72 **closed**（FR133；`1a726c8`）；FR125 T1–T4 **closed**（Epic 65）；FR117 / FR114 / FR104 **closed**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **73.2–73.3** 标 `ready`。  
> **隔离：** ≠ FR104 alone；≠ FR114 alone；≠ FR117 alone；≠ FR125 T1–T4 alone；≠ docs-only。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR134 / Epic 73；对照 FR125 / FR117 / FR114 / FR104 |
| 记录日期 | 2026-09-11 |
| 状态 | **closed** — Story **73.3** |
| **选定加深子集** | **真实上游 GUI 安装包与/或 IDE 插件深度（G1–G4）** |

### (a) 上游约束

- Epic 72 / FR133 已关闭；Phase 12–15 关闭仍有效（**NFR56**）；不得改写 FR94–132「已关闭」为失败。
- FR125 T1–T4（`wave.tywaves.json` / `tywaves.launch.sh` / `BITLOOM_TYWAVES_BIN` / 失败语义 / ATDD）**仍有效**；alone ≠ FR134。
- FR117 自研 `typed-wave.html` / FR114 LCOV GUI / FR104 `interactive.html` **仍有效**（NFR56）；alone ≠ FR134。
- 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；**Tywaves 运行时不得进入设计 crate 依赖**（延续 standing honesty）。
- 品牌 Bitloom / `bitloom-*`；禁止发布 `rhdl` / `rhdl-bits`。
- 上游 GUI / IDE 插件版本、发行渠道与元数据契约须在本记录 G1–G2 钉死后方可宣称 FR134 绿。

### (b) 粗工期带

- Epic 73：约 **1–4 人周**（73.1 ≤0.25；73.2 1–3；73.3 ≤0.75）。置信度：中。
- 假设：优先文档化真实上游 GUI 安装包 **或** IDE 插件之一（G1）；两者皆交付亦可；缺上游工件时须走 G3 失败语义，不得 silent 绿。

### (c) 禁止的静默降级清单

- 不得缺本记录将 **73.2–73.3** 标 `ready`。
- **不得仅 FR104 `interactive.html` 关闭 FR134。**
- **不得仅 FR114 LCOV/`coverage.html` 关闭 FR134。**
- **不得仅 FR117 自研 typed-wave 关闭 FR134。**
- **不得仅 FR125 T1–T4 关闭 FR134。**
- **不得 docs-only**（仅评估 defer ≠ GUI/插件深度本身）。
- 缺上游 GUI/插件/元数据不得 silent 宣称 FR134 绿。
- 不得改写 FR104/114/117/125「已关闭」为失败（NFR56）。
- 不得因「终局」口号静默吞并 **NFR59**（未列入更深 GUI/IDE 子集仍须新合同）。
- 不得冒充 **NFR14-crates**（本门禁 = NFR14 / AD-28）。
- 不得将 Tywaves 运行时塞进设计 crate 依赖。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR56** / **NFR57** / **NFR59**。
- 消歧：**NFR14-crates**（crates.io 名占用）≠ 本门禁。

---

### 选定集成形状（G1–G4）

| ID | 钉死 |
| --- | --- |
| **G1 版本/发行渠道** | 钉死真实上游 Tywaves **GUI 安装包**与/或 **IDE 插件**的版本标识 + 发行渠道（例：上游 release URL、文档化安装步骤、IDE marketplace / VSIX 标识，或树内可复现 stub 安装描述符）；**超出** FR125 `BITLOOM_TYWAVES_BIN` 单二进制 launch |
| **G2 元数据契约** | 可检查元数据：至少含 Bitloom brand、`schemaVersion`、以及 GUI/插件能力标记（例：`tywaves.gui` / `tywaves.ide-plugin` 字段或等价 manifest）；契约本身可 ATDD；≠ 仅有 `wave.tywaves.json` sidecar（FR125 T1） |
| **G3 失败语义** | 缺上游 GUI 安装包 / IDE 插件 / 元数据（或强制缺失夹具）→ **非零**退出或文档化手动清单失败；消息须可识别（含 `bitloom.tywaves` 或等价）；**禁止 silent 绿** |
| **G4 验收谓词** | ATDD 与/或文档化手动清单覆盖 G1–G3；回归 FR104 / FR114 / FR117 / FR125 不破（NFR56）；关闭条件 = G1–G4 落地，**≠** FR125 T1–T4 alone |

### 与 FR125 T1–T4 边界

| 层 | 完成面 | FR134 关系 |
| --- | --- | --- |
| **FR125 T1–T4** | sidecar + launch.sh + `BITLOOM_TYWAVES_BIN` + 失败语义 + ATDD | **仍有效；alone ≠ FR134** |
| **FR134 G1–G4** | 真实上游 GUI 安装包与/或 IDE 插件深度 + 元数据契约 + 更深失败/验收 | **本 epic** |

### 未列入（NFR59）

- 替换默认 VCD / `typed-wave.html` 为唯一波形面；完整 ChiselSim 耦合；任意第三方波形 GUI 生态；未在 G1 钉死的额外 IDE 商店多端发布。

### Epic 73 关闭条件（73.3 勾选）

- [x] **73.2 / FR134：** G1–G4 产品路径 + ATDD（或文档化手动清单）
- [x] **文档 / deferred / Phase 16 故事清单指针**
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖：** Bitloom；Tywaves 运行时不进设计 crate
- [x] **FR125 / FR117 / FR114 / FR104 关闭仍有效**（NFR56）
