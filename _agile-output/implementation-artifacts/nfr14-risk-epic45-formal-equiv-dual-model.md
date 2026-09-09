# NFR14 风险记录 — Epic 45 形式等价与双模型齐全（FR100 / FR102 / FR103）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**（风险门禁）；Phase 12 **NFR40 / NFR43**；交付 **FR100、FR102、FR103**。  
> **模板：** `nfr14-risk-record-template.md`（Story 19.1）。  
> **体例对照：** `nfr14-risk-epic44-full-elaborate-lsp.md`；`nfr14-risk-epic39-ide-multiview.md`；`nfr14-risk-dual-sim-generation.md`。  
> **前置：** Epic 40 **closed**（FR94；Path B 字面绿）；Epic 39 **closed**（**FR92** SharedStimulusScoreboard + adapter 模板——**≠** 形式等价产品）；Epic 43 **closed**（FR98 近 VIP UART/SPI/I2C/AXI）；Epic 44 **closed**（FR99）。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 45 后续故事 **45.2–45.4** 标为 `ready`，亦不得开工实现。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR100, FR102, FR103 / Epic 45；NFR14、NFR40、NFR43；推翻 FR93#3；对照 FR92 / FR29 / FR30 / FR47 / FR78 / FR98 |
| 记录日期 | 2026-09-09 |
| 状态 | accepted — Story 45.1（开门禁；**未**勾选 Epic 45 关闭） |

### (a) 上游约束

- **Epic 40 / FR94 已关闭：** Path B 字面绿合同已开闸；本 epic 实现 FR93#3 → **FR100**（自动 FL≡RTL / 形式等价产品），并交付 **FR102**（多视图属性全矩阵）与 **FR103**（一级 IP 双模型齐全）。
- **FR92（软依赖 · Epic 39）：** `SharedStimulusScoreboard` + bridge adapter **模板**已交付；一致性仍是随机/对照 / PortValues 记分板（见 `docs/fr92-shared-stimulus-adapter.md`）。FR92 **明确不承诺**自动形式 FL≡RTL——本 epic **升格**该排除项为产品交付，**不得**继续用「FR92 已绿」宣称 FR100。
- **FR30 / FR47：** 既有双视图等价检查与生成功能模拟器路径是**前件**，不是 FR100「形式等价产品」完成定义（FR30 文档自称 bounded checker / 非 SMT）。
- **FR29 / FR78：** 手写 `#[bridge]` / `#[abstraction]` / `#[both]` 与 `start_wait_complete` adapter 模板已存在；FR102 完成面是**属性全矩阵**（见下方清单），**不得**仅靠模板 adapter 交差。
- **FR98 / docs/ip：** UART/SPI/I2C/AXI 近 VIP + SyncFifo 一级面已有；FR103 要求清单内每类具备**功能视图 + 周期精确**双模型完整面（扩大「仅同刺激模板」）。
- **AD-5 / Epic 46 隔离：** SystemC TLM-2.0 产品路径属 **FR101 / Epic 46**；本 epic **不得**把 TLM 产品或默认 TLM≡CA 口号并入 FR100 关闭口径（形式等价对象是 FL≡RTL / 功能↔周期，除非 45.2 另钉合同）。
- **设计依赖边界：** 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；公开品牌 **Bitloom**（crates.io / CLI：`bitloom`）。
- **NFR40：** 形式工具链与全矩阵维护为多年/高维护字面条；不得用随机记分板或模板 adapter 假绿交差。

### (b) 粗工期带

- **预计：** Epic 45 整体约 **5–14 人周**（45.1 本风险记录 ≤0.25 人周；45.2 FR100 形式等价产品路径 2–5 人周；45.3 FR102 属性全矩阵 1.5–4 人周；45.4 FR103 一级 IP 双模型 + 收口 1.5–4.5 人周）。置信度：**低–中**（形式工具选型与 IP 双模型深度方差大；NFR40）。
- **假设：** 不回滚 FR92 同刺激路径；不把 SystemC TLM（Epic 46）或富波形（Epic 47）并入本 epic；不并行冒充 Epic 46–47 关闭。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **45.2–45.4** 标 `ready` 或开工实现。
- **不得把随机共测记分板（FR92 `SharedStimulusScoreboard` / 仅随机对照）单独标成形式等价产品 / 关闭 FR100。**
- **不得仅以模板 adapter（FR78 `start_wait_complete` / FR92 adapter 文档）关闭 FR102 或 FR103。**
- 不得在未改 PRD / 本记录的前提下砍掉下方「双模型齐全」一级 IP 清单中的任一类却宣称 FR103 全绿。
- 不得把 **SystemC TLM-2.0 / Epic 46** 冒充本 epic 已交付，或把「默认 TLM≡CA」口号当作 FR100 完成证据。
- 不得允许 `#[functional_state]`（或等价）泄漏进 HIR / `freeze`（除非本记录日后另开合同修订）。
- 不得把本记录冒充 **NFR14-crates**（crates.io FCFS）完成定义。

### (d) 负责人

- 姓名 / 角色：Richard（实现负责人 / Dev）— **NFR14** 门禁与 **NFR40**（多年维护 / 字面绿诚实）共同责任人；**NFR43** 共同注意人
- 备份 / 升级路径：改选形式工具族、砍 IP 清单、或把 FR92 记分板升格为 FR100 唯一完成面须升级至产品 / Correct Course；与 AD-5 / Epic 46 边界冲突升级至 AD-28 维护者。

---

### 形式等价产品边界（本记录钉死 · 供 45.2 / FR100）

> 未列能力默认**非目标**；静默扩大未列模式禁止。随机共测可作**配套**路径，**不得单独**关闭 FR100。

| # | 边界项 | 钉死值（MVP 合同） |
| --- | --- | --- |
| **F1 · 工具** | 产品入口须文档化并可选一：**(i)** 树内形式/有界证明检查器 API（超出「仅 PortValues 随机记分板」）；或 **(ii)** 绑定的外部形式工具（如 SymbiYosys / SMT 族）+ Bitloom CLI/`cargo bitloom` 或文档化产品入口。45.2 须在实现时**钉死选定支**并写入 docs；不得无入口宣称 FR100 |
| **F2 · 证明义务** | 对每个验收夹具：检查/证明 **功能视图（FL）≡ 周期精确 RTL/`tick`**（PortValues 或文档等价观测面）；故意不一致须 **Fail + 可读诊断**；通过须可复现 |
| **F3 · 夹具** | 至少 **一条**自动随机/对照路径（可复用 FR92 刺激，但完成叙事须写明「配套非充分」），以及 **一条**形式等价（或绑定形式工具）**产品入口**夹具；二者均须 ATDD 可复现 |
| **F4 · 范围** | MVP = 文档钉死的模块/IP 夹具规模；**不**要求全芯片无界证明；超出范围须明确失败/降级提示 |
| **F5 · 非充分** | **随机共测记分板 alone ≠ 形式等价产品**；FR30 bounded checker / FR92 scoreboard **可保留**，但**不得**单独勾选 FR100 |

### 属性宏矩阵清单（本记录钉死 · 供 45.3 / FR102）

> 完成面 = 下列属性（或文档等价）作为**可验收矩阵**，含合法组合与非法组合门禁；**不是**「仅有 adapter 模板文档」。

| 属性 / 表面 | 角色 | 进入 HIR/`freeze`？ | 矩阵义务 |
| --- | --- | --- | --- |
| `#[functional_model]`（或文档等价） | 主机功能模型 `cycle` | **否** | 必选完成面行 |
| `#[abstraction]`（或文档等价） | 非定时 / 事务形抽象 | **否** | 必选完成面行 |
| `#[functional_state]`（或文档等价） | 功能侧状态 | **否**（不得泄漏进 HIR） | 必选完成面行 + 负向 ATDD |
| `#[bridge]`（或文档等价） | 引脚↔抽象适配 | **否** | 矩阵须覆盖与 abstraction/both 的合法组合 |
| `#[both]`（或文档等价） | 混合夹具（RTL + 手写视图） | **否** | 矩阵须覆盖混合路径 |
| FR78 / FR92 adapter **模板** | 可复用握手骨架 | N/A | **配套**；**单独不足**关闭 FR102 |

**非法组合（至少须文档+ATDD）：** `functional_state`（或等价）出现在 synthesizable / FrozenHir 路径；未文档化的属性静默 no-op 却宣称矩阵完成。

### 「双模型齐全」一级 IP 集合（本记录钉死 · 供 45.4 / FR103）

| IP 类 | prelude 类型（代表） | 双模型义务（功能 + 周期精确） | 关闭必选？ |
| --- | --- | --- | --- |
| **FIFO** | `SyncFifo` | 可运行功能模型 + 周期 `tick`/生成路径；与刺激/等价路径联验 | **是** |
| **UART** | `UartTx` + `UartRx`（全双工合同可分夹具） | 同上 | **是** |
| **SPI** | `SpiMaster` | 同上 | **是** |
| **I2C** | `I2cMaster` | 同上 | **是** |
| **AXI4-Lite** | `Axi4LiteSlave` | 同上 | **是** |
| **GPIO** | （若有） | — | **否**（与 FR98 G1 一致；未交付不构成 FR103 失败） |
| **Crc8Lut / 黑盒** | `Crc8Lut` / `ExtBlackBox` | — | **否**（非本清单一级关闭集） |

**显式裁剪合同（当前）：无。** 上表五类全交付；裁剪须改本记录并改 PRD。

### FR92 / FR30 / 模板 vs FR100/102/103（摘要）

| 路径 | 角色 | 可否单独关闭对应 FR |
| --- | --- | --- |
| FR92 SharedStimulusScoreboard | 同刺激随机/对照 | **否** → FR100 |
| FR30 bounded checker | 夹具刺激集上的 PortValues 比对 | **否** → FR100（可作前件） |
| FR78/FR92 adapter 模板 | 握手骨架 | **否** → FR102 / FR103 |
| 本记录 F1–F5 形式等价产品 + ATDD | FR100 完成面 | **是**（须 45.2） |
| 本记录属性宏全矩阵 + ATDD | FR102 完成面 | **是**（须 45.3） |
| 本记录五类 IP 双模型 + ATDD | FR103 完成面 | **是**（须 45.4） |

### Epic 45 故事分工（本记录不开工实现）

| 故事 | 交付 | 本记录角色 |
| --- | --- | --- |
| **45.1** | 本 NFR14 风险记录 + ATDD | **本故事** |
| **45.2** | 自动 FL≡RTL / 形式等价产品路径（FR100；F1–F5） | Gate：须本记录后才可 ready |
| **45.3** | 多视图属性全矩阵（FR102） | Gate：须本记录后才可 ready |
| **45.4** | 一级 IP 双模型齐全 + Epic 45 收口（FR103） | Gate：须本记录后才可 ready |

### 并行 / 维护叠加（Chipyard 式 · NFR40 / NFR43）

- 可与 Epic 46–47 **并行规划**，但各自须独立 NFR14；本 epic 不得冒充 TLM / 波形字面条关闭。
- `docs/fr92-shared-stimulus-adapter.md` / deferred-work「默认 TLM≡CA / 自动形式等价 → FR100」须同一叙事；禁止「记分板已有却宣称形式等价产品已绿」。
- prelude IP 双模型 churn：与 Epic 43 近 VIP 加深叠加——保持「近 VIP 协议深度」与「双模型齐全」合同边界清晰。

### 引用

- AD-28 — 风险门禁（NFR14）
- PRD NFR14 / FR100 / FR102 / FR103；NFR40 / NFR43；推翻 FR93#3
- 对照：FR92；FR29；FR30；FR47；FR78；FR98；`docs/fr92-shared-stimulus-adapter.md`；`docs/ip/README.md`
- 前置：Epic 40 / FR94；Epic 39 / FR92；Epic 43 / FR98；Epic 44 / FR99
- 历史别名消歧：**NFR14-crates** ≠ 本门禁 **NFR14**

---

### Epic 45 关闭条件（Story 45.4 勾选）

- [x] **45.2 / FR100：** F1–F5 形式等价产品入口 + 随机/对照配套路径 + ATDD — Story 45.2
- [x] **45.3 / FR102：** 属性宏全矩阵（含非法组合门禁）+ 正/负向 ATDD — Story 45.3
- [ ] **45.4 / FR103：** 清单内 FIFO/UART/SPI/I2C/AXI 功能+周期双模型联验 — Story 45.4
- [ ] **文档 / deferred：** 撤销「默认 TLM≡CA / 自动形式等价」永久非目标话术（FR100 口径；TLM 产品面仍见 Epic 46）— Story 45.4
- [ ] **禁止事项未触发：** 无随机记分板单独关闭 FR100；无仅模板 adapter 关闭 FR102/103 — Story 45.4
- [ ] **品牌 / 依赖：** 仍为 Bitloom；设计 crate 只依赖 `bitloom-prelude` — Story 45.4

---

## 门禁一句话

**缺 NFR14 风险记录（或缺字段 a–d）⇒ 不得将 Epic 45 故事 45.2–45.4 标 `ready`。**  
**FR100 完成面 = 形式等价产品（工具/证明义务/夹具见 F1–F5）+ ATDD；不得以随机共测记分板单独关闭。**  
**FR102/103 完成面 = 属性宏全矩阵 + 清单内一级 IP 双模型；不得仅以模板 adapter 关闭。**  
**Epic 45 / FR100–103：仍 open — 待 45.2–45.4。**
