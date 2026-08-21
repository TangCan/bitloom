# NFR14 风险记录 — Chisel 双向（Epic 20 / FR28 + FR46）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（Phase 7 风险门禁）。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 20 后续故事 **20.2–20.5** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR28 + FR46（及 FR40 `import` 产品入口）/ Epic 20 |
| 记录日期 | 2026-08-21 |
| 状态 | accepted |

### (a) 上游约束

- **钉死版本对（脊柱 Stack / AD-9）：** Chisel **7.14.0** ↔ CIRCT firtool **1.155.0**；升钉须等 Chisel 正式配对后再改 AD-9 / Stack（NFR12）；不得默认信任 PATH 上的随机 firtool。
- **CIRCT / Chisel 交换边界：** 自 Chisel 5 起上游**不**支持 FIRRTL 文本 → Scala `Circuit` 解析；交换边界为 `.fir` + `firtool`（chipsalliance/chisel **#4899**）。本工具链须自建「生成可编译 Scala / 导入路径」，不得假设恢复 Scala `firrtl.Parser` / `Parser.parse` API。
- **FIRRTL 文本契约仍属 AD-3：** FrozenHir ↔ `FIRRTL version 6.0.0` 文本；Chisel Scala 产品路径属 **AD-27**，不替代 AD-3。
- **许可 / CI：** 生成 Scala 的编译验收可能需要 JVM + 钉死 Chisel 工件；CI 可标可选 runner，但「无 JVM 环境」不得静默把 FR28 验收降为尽力失败。

### (b) 粗工期带

- **预计：** Epic 20 整体约 **3–5 人周**（20.2 AD 核对/补缺 ≤0.5 人周；20.3 正向生成器 1.5–2.5 人周；20.4 反向导入 0.5–1 人周；20.5 CLI+混合夹具 0.5–1 人周；缓冲含 firtool/Chisel 夹具与文档）。
- **置信度 / 假设：** 中；假设钉死栈不在本 epic 中途升钉，且机械风格 Scala 可通过编译+端口/层次谓词验收（Open Q5 已关闭）。若需 idiomatic 手写风格或绑 Chisel 内部 API，工期显著上修。

### (c) 禁止的静默降级清单

- 不得把 **FR28** 改回「结构化尽力失败 / 尽力生成但不保证编译」而不改 PRD 与脊柱。
- 不得把 **FR46** 双向合同降回「仅 `.fir` 交换、无产品化 Scala 路径 / 无导入腿」而不改 PRD。
- 不得以 NFR10 **调试用** HIR→源码再生冒充 Chisel 双向完成。
- 不得要求恢复已删除的 Scala FIRRTL Parser API 作为验收前提，或以「上游删了 API」自动豁免 FR28/FR46。
- 不得把「可维护」偷换成必须 idiomatic 手写 Chisel（验收条=可编译+端口/层次谓词；机械风格可接受）。
- 不得在无本记录（或缺 a–d）时将 **20.2–20.5** 标 `ready` 或开工实现。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）
- 备份 / 升级路径：架构争议升级至脊柱 AD-27 维护者；钉死升版跟 NFR12 流程。

### FR46 实现方向（addendum 选项 A/B/C）

| 选项 | 说明 | 本记录拟选 |
| --- | --- | --- |
| A. 自研 FIRRTL→Scala pretty printer + 测试编译 | 不依赖已删 Parser | **拟选** `[ASSUMPTION]` — 与 AD-27 一致 |
| B. 仅保证 `.fir` 交换 + 薄 Scala wrapper 工程 | 较弱，可能不满足「可维护 Chisel」字面 | 不选 |
| C. 绑定特定 Chisel 版本内部 API | 脆弱，随 CIRCT bump 破碎 | 不选 |

若架构 AD 最终改选 B/C，须先修订本记录与脊柱，不得静默切换。

### 并行 / 维护叠加（Chipyard 式）

- 与 Epic 21–24 并行时：每次 Chisel/firtool 升钉都会膨胀 FR28/FR46 回归面（生成 Scala 编译 + 往返谓词）；须与 NFR12 升钉节奏绑定，避免多 epic 各拉一套版本。
- 语义最低公分母：机械可编译 Scala ≠ 团队手写 idiomatic；文档须写清验收条，避免评审用风格否决合同。
- 混合夹具与树内 IP（Epic 22）耦合时，禁止把 Chisel 互操作失败归因到「IP 未就绪」而跳过 FR46 验收。

### 引用

- AD-28 — Phase 7 风险门禁（NFR14）
- AD-27 — Bitloom ↔ Chisel 产品互操作
- AD-9 / Stack — Chisel 7.14.0 ↔ firtool 1.155.0
- PRD NFR14（`prd-rhdl-2026-08-19/prd.md` §6）
- Addendum：FR46 选项 A/B/C；chipsalliance/chisel#4899
- 历史别名消歧：**NFR14-crates**（crates.io FCFS）≠ 本门禁 **NFR14**

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 20 故事 20.2–20.5 标 `ready`。**
