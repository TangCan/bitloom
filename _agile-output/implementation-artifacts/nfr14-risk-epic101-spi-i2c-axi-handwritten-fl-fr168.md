# NFR14 风险记录 — Epic 101 SPI / I2C / AXI 手写 FL（FR168）

> **权威：** PRD `prd-rhdl-2026-08-19` **NFR14**；架构脊柱 **AD-28**；Phase 20 **NFR73–NFR77**；实现面 **FR168**；宣称须引对应 FR。  
> **模板：** `nfr14-risk-record-template.md`。  
> **体例对照：** `nfr14-risk-epic95-unlisted-protocol-handwritten-fl-fr163.md`。  
> **前置：** Epic 99 / FR166 **closed**；Epic 95 / **FR163** `UartRx` handwritten FL **closed**；Epic 74 / **FR135** `UartTx` **closed**；Epic 66 / **FR126** Gpio **closed**。  
> **门禁：** 无本有效记录（或缺字段 a–d）⇒ **不得**将 Epic 101 后续故事 **101.2–101.3** 标为 `ready`，亦不得开工实现。**不得**以 FR163 `UartRx` alone / FR135 alone / FR126 alone 勾选本 FR。**SPI+I2C+AXI 三者皆须交付**（禁止「至少一项」交差）。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR168 / Epic 101；NFR14、NFR73、NFR74、NFR76、NFR77；对照 FR163 / FR135 / FR126 / FR103 |
| 记录日期 | 2026-09-12 |
| 状态 | open / in-progress — Story 101.1；Epic 101 进行中；101.2–101.3 须本记录后才可 ready |
| **选定** | 在保留 FR163 `UartRxFunctional` / FR135 `UartTxFunctional` / FR126 `GpioFunctional` 关闭面的前提下，授权 **`SpiMaster`（或文档等价）、`I2cMaster`（或等价）、`Axi4LiteSlave`（或等价）各自手写 FL ≡ tick** — **三者皆交付**；**≠ FR163 alone**；**≠「至少一项」** |

### Phase 12–19 / FR163 关闭面 vs Epic 101 实现边界（NFR73 · 必读）

| 层 | 含义 | 本阶段状态 |
| --- | --- | --- |
| **FR103** | SyncFifo 手写 + UART/SPI/I2C/AXI **GeneratedFunctional** | **仍有效**；GeneratedFunctional alone ≠ FR168 |
| **FR126 / Epic 66** | `GpioFunctional` 手写 | **仍有效**；alone ≠ FR168 |
| **FR135 / Epic 74** | `UartTxFunctional` 手写 | **仍有效**；alone ≠ FR168 |
| **FR163 / Epic 95** | `UartRxFunctional` 手写 | **仍有效；不得改写为失败**；**alone ≠ FR168** |
| **Epic 99 / FR166** | Phase 20 闸门 | **已关闭** |
| **Epic 101 / FR168** | SPI + I2C + AXI 手写 FL | **本实现 epic** |
| **Epic 100 / 102–104** | 其它 Phase 20 | **本 epic 不做** |

**选定：关闭 FR163 明示未做的 SPI/I2C/AXI 手写缺口；≠ 重做 FR163；≠ 仅交付三者之一。**

### 本批钉死（101.2 必须 · 三者皆须）

| 维度 | MVP | 明确不在本 epic |
| --- | --- | --- |
| **SPI** | `SpiMaster`（或 docs 钉死等价 SPI IP）手写 FL ≡ tick + 矩阵/对拍 API | GeneratedFunctional alone；docs-only |
| **I2C** | `I2cMaster`（或等价）手写 FL ≡ tick + API | 仅 SPI 或仅 AXI 交差 |
| **AXI** | `Axi4LiteSlave`（或等价）手写 FL ≡ tick + API | 仅「至少一项」 |
| **crate 边界** | 设计 crate → **`bitloom-prelude`**；手写 FL / 矩阵在 **`bitloom-sim`**（或现有对拍面） | 强制设计 crate 依赖 `bitloom-sim` |
| **与 FR163 边界** | FR163 = `UartRx`；FR168 = **SPI+I2C+AXI 三者** | 把 FR163 alone 写成已含 SPI/I2C/AXI |

### 验收谓词 / 失败语义（101.2）

| 项 | 钉死 |
| --- | --- |
| **正向** | 三者手写 FL ≡ `Sim::settle`+`tick`（或文档等价）均 Pass；`docs/fr168-*`；ATDD 可复现 |
| **负向** | 故意错模型 → Fail / 可读；**不得** silent-Ok 宣称 FR168 |
| **禁止勾选** | FR163 alone；FR135 alone；FR126 alone；GeneratedFunctional alone；「至少一项」；仅 docs |

### `ip/` 布局与软序

- 触碰 `crates/bitloom-prelude/src/ip/`（SPI/I2C/AXI）与/或 `crates/bitloom-sim` 双模型面。
- **软序：** 与其它并行改 `ip/` 的 epic（Epic 100 GUI 面除外）**串行或分目录**，避免同 PR 冲突。
- **Epic 78 / FR139** crate 边界仍有效（NFR73）。

### 故事分工

| 故事 | 交付 |
| --- | --- |
| **101.1** | 本 NFR14 |
| **101.2** | SPI+I2C+AXI 三者手写 FL + ATDD；`docs/fr168-*` |
| **101.3** | README / `docs/ip/` / deferred 收口；勾选 Epic 101；写明未纳入其它协议仍 **NFR76** |

### (a) 上游约束

- **Epic 99 已关闭。**
- **FR163 / FR135 / FR126 / FR103：** **仍有效**；不得改写为失败；alone ≠ FR168。
- **NFR73：** 不得改写 FR94–165「已关闭」。
- **NFR76：** 禁止静默扩大到未钉死协议；禁止缩成「至少一项」。
- **品牌：** **Bitloom**；设计 crate → **`bitloom-prelude`**（AD-6）。
- **软序：** 101.2 → 101.3；`ip/` 触碰面互斥见上。

### (b) 粗工期带

- **预计：** Epic 101 整体约 **1.5–4 人周**（101.1 ≤0.25；101.2 三者 FL+ATDD 1–3.5；101.3 收口 0.25）。
- **置信度 / 假设：** 中（可复用 FR163/FR135 体例；RTL IP 已存在）。假设不改 prelude 公共 elaboratable 表面契约。

### (c) 禁止的静默降级清单

- 不得在缺本记录（或缺 a–d）时将 **101.2–101.3** 标 `ready` 或开工实现。
- **不得以 FR163 `UartRx` alone / FR135 alone / FR126 alone 勾选 FR168。**
- **不得以「至少一项」SPI 或 I2C 或 AXI 交差冒充三者皆交付。**
- **不得以 GeneratedFunctional alone / docs-only 冒充 FR168。**
- **不得**静默扩大 FR142；**不得**把 `git push` 当 FR。
- 不得把本记录冒充 **NFR14-crates**。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR73** / **NFR76**
- 升级：缩回「至少一项」或砍掉任一协议须 Correct Course / 产品批准。

### 引用

- AD-28；FR168；对照 FR163 / FR135 / FR126；Phase 20 Correct Course approved 2026-09-12（Q1–Q5：FR168 = SPI+I2C+AXI 三者皆交付）
- **NFR14-crates** ≠ 本门禁
