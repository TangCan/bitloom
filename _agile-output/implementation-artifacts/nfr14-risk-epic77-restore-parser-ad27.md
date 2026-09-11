# NFR14 风险记录 — Epic 77 恢复废弃 Parser / 再修订 AD-27（FR138）

> **权威：** PRD NFR14；AD-28；Phase 16 **NFR56 / NFR57 / NFR58 / NFR59**；交付 **FR138**。  
> **前置：** Epic 72 **closed**（FR133）；FR130 S1–S4 **closed**（Epic 70；Style Guide；**Parser 未恢复**）；FR122 / FR111 / FR97 **closed**；AD-27 现行仍**禁止**静默恢复 `Parser.parse`。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **77.2–77.3** 标 `ready`。  
> **隔离：** ≠ FR97 alone；≠ FR111 alone；≠ FR122 O1–O4 alone；≠ FR130 Style Guide（Parser 未恢复）alone；≠ docs-only；≠ 未修订 AD-27 宣称关闭。  
> **NFR58：** 实现前**必须** Correct Course 痕迹（可复用 Phase 16 闸门）+ **再修订 AD-27**；未修订不得标 77.2 done / 宣称 FR138 关闭。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR138 / Epic 77；对照 FR130 / FR122 / FR111 / FR97 |
| 记录日期 | 2026-09-11 |
| 状态 | **closed** — Story **77.3**（P1–P4 + AD-27 修订已于 77.2；文档/deferred/NFR14 收口） |
| **选定加深子集** | **恢复 Scala `Parser.parse`（或文档等价）为产品关闭条件 + 再修订 AD-27（P1–P4）** |

### (a) 上游约束

- Epic 72 / FR133 已关闭；Phase 12–15 关闭仍有效（**NFR56**）；不得改写 FR94–132「已关闭」为失败。
- FR130 S1–S4（Style Guide；`emit_chisel_style_guide_fr130`；**S3 = Parser 未恢复**）**仍有效**；alone ≠ FR138。
- FR122 O1–O4 / FR111 D1+D3 / FR97 idiomatic MVP **仍有效**；alone ≠ FR138。
- **AD-27 现行：** 默认仍禁止把恢复 `Parser.parse` / `firrtl.Parser` 写成产品关闭条件（见 chipsalliance/chisel#4899）；**本 epic 必须再修订**后才可合法关闭 FR138（**NFR58**）。
- **Correct Course 痕迹（可复用）：** Phase 16 闸门已批准 FR138 升格 — `correctCoursePhase16Approved: 2026-09-11`；提案 `_agile-output/planning-artifacts/sprint-change-proposal-2026-09-11-phase16-nfr55-final-closeout.md`。复用该痕迹**不**豁免 AD-27 再修订。
- **Stack / 版本配对：** Chisel **7.14.0** + firtool **1.155.0**（AD-9）；Parser 恢复产物须与该配对可检查对齐；升钉须等 Chisel 正式配对并改 Stack / AD-9 / AD-27。
- 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；Parser 恢复属 CLI/互操作/夹具层，不得把 Scala 运行时依赖塞进设计 crate。
- 品牌 Bitloom / `bitloom-*`；禁止发布 `rhdl` / `rhdl-bits`。

### (b) 粗工期带

- Epic 77：约 **1–3 人周**（77.1 ≤0.25；77.2 0.75–2.5；77.3 ≤0.5）。置信度：中–低（上游 Parser 已删；可能需 shim/compat）。
- 假设：77.2 交付下表 **P1–P4** + AD-27 再修订；未列入更深 Chisel/Parser 生态 / 社区 linter 全家桶仍 **NFR59**。

### (c) 禁止的静默降级清单

- 不得缺本记录将 **77.2–77.3** 标 `ready`。
- **不得仅 FR97 关闭 FR138。**
- **不得仅 FR111 关闭 FR138。**
- **不得仅 FR122 O1–O4 关闭 FR138。**
- **不得仅 FR130 Style Guide（Parser 未恢复）关闭 FR138。**
- **不得 docs-only**（仅评估 defer / 文档声明 ≠ Parser 产品路径本身）。
- **不得未修订 AD-27 即宣称 FR138 关闭**（NFR58）。
- 不得改写 FR97 / FR111 / FR122 / FR130「已关闭」为失败（NFR56）。
- 不得因「终局」口号静默吞并 **NFR59**（未列更深 Parser 生态仍须新合同）。
- 不得冒充 **NFR14-crates**（本门禁 = NFR14 / AD-28）。
- 不得在无 Correct Course 痕迹时开工把 Parser 写成产品关闭条件（本记录允许复用 Phase 16 闸门痕迹）。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR56** / **NFR57** / **NFR58（AD-27 再修订）** / **NFR59**。
- 消歧：**NFR14-crates**（crates.io 名占用）≠ 本门禁。

---

### Parser 恢复形状（API / 工作流 / 版本配对）

| 角色 | 钉死 | 77.2 义务 |
| --- | --- | --- |
| **选定 API** | Scala **`Parser.parse`** / **`firrtl.Parser`**（或文档钉死的**产品等价** API，名称须在 `docs/fr138-*` 与 AD-27 修订中显式对照） | 产品路径实际暴露/调用该 API（或等价）；可检查 |
| **选定工作流** | 文档化产品路径：代表性 `.fir`（或 Chisel/FIRRTL 文本输入）→ Parser（或等价）→ 可检查结果（FrozenHir / emit / tick 往返 **与/或** 可编译 Scala 夹具消费 Parser 输出） | 路径可复现；失败非零可读 |
| **版本配对** | **Chisel 7.14.0** + **firtool-1.155.0**（AD-9 Stack）；若恢复依赖独立 Scala FIRRTL/parser 构件，须在 AD-27 修订中钉死构件坐标/版本并与该配对对齐 | ≠ 未配对 Chisel HEAD；≠ PATH 随机 `scala`/`firrtl`；升钉须改 Stack |
| 覆盖 / 钉死路径 | CLI / 文档夹具 / ATDD（例：`cargo bitloom …` 或 `just` / `docs/fr138-*` 钉死命令） | 缺 Parser / 版本不符 → 非零可读失败 |
| 已关闭对照 | FR130 S3「Parser 未恢复」；FR122/111/97 idiomatic/Style 完成面 | **仍有效；alone ≠ FR138** |

### 验收谓词（P1–P4）

| ID | 钉死 |
| --- | --- |
| **P1 API/工作流/配对** | 钉死 Parser（或文档等价）API + 文档化工作流 + Chisel **7.14.0** / firtool **1.155.0** 配对；可检查 |
| **P2 AD-27 + Correct Course** | **再修订 AD-27** 明确允许本产品关闭条件；Correct Course 痕迹存在（可复用 Phase 16 `correctCoursePhase16Approved: 2026-09-11`）；**未修订 AD-27 ⇒ 不得**标 77.2 done / 宣称 FR138 关闭（NFR58） |
| **P3 失败语义** | 缺 Parser / API 不可用 / 版本或配对不符 → **非零** + **可读**失败；未修订 AD-27 时验收必须拒绝关闭宣称 |
| **P4 ATDD / 边界** | ATDD 覆盖 P1–P3；**≠ FR97 / FR111 / FR122 O1–O4 alone**；**≠ FR130 Style Guide alone**；关闭条件 = P1–P4，**≠** docs-only |

### 失败语义

| 场景 | 钉死行为 |
| --- | --- |
| Parser / 等价 API 缺失或不可调用 | 产品路径 **非零退出**；stderr/日志 **可读**说明缺 Parser 或 API |
| Chisel / firtool / parser 构件版本与钉死配对不符 | **非零** + **可读**；禁止 silent skip |
| AD-27 尚未再修订即宣称 FR138 关闭 | **验收失败**；不得标 77.2 done / Epic 77 关闭 |
| 仅 FR130 Style Guide（Parser 未恢复）或仅 FR97/111/122 | **不得**勾选 FR138 |

### 与 FR130 Style Guide / Parser 未恢复边界

| 层 | 完成面 | FR138 关系 |
| --- | --- | --- |
| **FR130 S1** | Style Guide 宣称头 + `scalafmt-style` | **仍有效；alone ≠ FR138** |
| **FR130 S2** | `withClockAndReset` 纪律 + naming | **仍有效；alone ≠ FR138** |
| **FR130 S3** | **不恢复** Parser（文档 + 验收谓词） | **仍有效；alone ≠ FR138**（本 FR **翻转**为恢复 Parser 关闭条件） |
| **FR130 S4** | `emit_chisel_style_guide_fr130` / `check_chisel_style_guide_fr130` | **仍有效；alone ≠ FR138** |
| **FR122 O1–O4** | 官方风格全家桶 | **仍有效；alone ≠ FR138** |
| **FR111 / FR97** | idiomatic 加深 / MVP | **仍有效；alone ≠ FR138** |
| **FR138 P1–P4** | Parser API/工作流/配对 + AD-27 再修订 + 失败语义 + ATDD | **本 epic** |

### Correct Course + AD-27（NFR58）— 实现前门禁

| 项 | 钉死 |
| --- | --- |
| Correct Course | **必须**有痕迹；**允许复用** Phase 16 闸门（`correctCoursePhase16Approved: 2026-09-11` + sprint-change-proposal Phase 16） |
| AD-27 | **必须再修订**（允许 Parser 作为产品关闭条件）；修订发生在 **77.2**（本故事 **不**改脊柱） |
| 顺序 | 先/同步修订 AD-27 → 交付 Parser 产品路径 + ATDD → 方可标 77.2 done |

### 未列入（NFR59）

- 完整社区 Style Guide / linter 全家桶超出 P1–P4；任意 Chisel 版本 Parser 回迁；macos/windows Parser 资产；以 FR130 S3「未恢复」继续宣称 Parser 产品关闭；未配对 firtool-1.156.0+ / Chisel HEAD。

### Epic 77 关闭条件（77.3 勾选）

- [x] **77.2 / FR138：** P1–P4 产品路径 + ATDD + **AD-27 再修订**（NFR58）+ Correct Course 痕迹
- [x] **文档 / deferred / Phase 16 故事清单指针**
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖：** Bitloom；设计 crate → `bitloom-prelude`
- [x] **FR130 / FR122 / FR111 / FR97 关闭仍有效**（NFR56）
