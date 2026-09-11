# NFR14 风险记录 — Epic 74 更多 IP 手写 FL（超 Gpio）（FR135）

> **权威：** PRD NFR14；AD-28；Phase 16 **NFR56 / NFR57 / NFR59**；交付 **FR135**。  
> **前置：** Epic 72 **closed**（FR133；`1a726c8`）；Epic 78 **closed**（FR139；`42cdffe` — 软序满足）；FR126 选定 Gpio 手写 FL **closed**（Epic 66）；FR103 / FR112 / FR119 / FR92 / FR100 **closed**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **74.2–74.3** 标 `ready`。  
> **隔离：** ≠ FR92 alone；≠ FR100 F1-(i) alone；≠ FR103/112/119 alone；≠ FR126 Gpio alone；≠ docs-only。  
> **软序：** Epic **78 → 74/75**（同触 `ip/`）；Epic 78 已关闭；相对 Epic 75 避免并行改同一 `ip/` 树。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR135 / Epic 74；对照 FR126 / FR103 / FR112 / FR119 / FR100 / FR92 |
| 记录日期 | 2026-09-11 |
| 状态 | **accepted** — Story **74.1**（门禁；实现→74.2；收口→74.3） |
| **选定加深子集** | **超 Gpio 更多协议手写 FL ≡ tick（F1–F3）** |

### (a) 上游约束

- Epic 72 / FR133 已关闭；Phase 12–15 关闭仍有效（**NFR56**）；不得改写 FR94–132「已关闭」为失败。
- FR126 `GpioFunctional` ≡ tick（Epic 66 F1–F3）**仍有效**；alone ≠ FR135。
- FR103 SyncFifo 手写 + UART/SPI/I2C/AXI **GeneratedFunctional** **仍有效**；GeneratedFunctional alone ≠ FR135 手写加深。
- FR112 MemRead≡tick / FR119 sby / FR100 F1-(i) / FR92 记分板 **仍有效**（NFR56）；alone ≠ FR135。
- 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；手写 FL / 双模型矩阵在 **`bitloom-sim`**（不进设计 crate）。
- 品牌 Bitloom / `bitloom-*`；禁止发布 `rhdl` / `rhdl-bits`。
- **软序：** Epic 78（FR139 `ip/` VIP/SocPad 细拆）已关闭；本 epic 可开工。相对 **Epic 75**（全芯片 pad）建议串行或分目录，避免同触 `ip/` 冲突。

### (b) 粗工期带

- Epic 74：约 **1–3 人周**（74.1 ≤0.25；74.2 0.75–2.5；74.3 ≤0.5）。置信度：中。
- 假设：74.2 至少交付清单中 **一项** 可验收协议手写 FL ≡ tick；未列入协议仍 **NFR59**。

### (c) 禁止的静默降级清单

- 不得缺本记录将 **74.2–74.3** 标 `ready`。
- **不得仅 FR92 记分板关闭 FR135。**
- **不得仅 FR100 F1-(i) 关闭 FR135。**
- **不得仅 FR103 / FR112 / FR119 关闭 FR135。**
- **不得仅 FR126 选定 Gpio 手写 FL 关闭 FR135。**
- **不得 docs-only**（仅评估 defer ≠ 更多协议手写 FL 本身）。
- 不得以 **GeneratedFunctional alone**（UART/SPI/I2C/AXI 生成路径）冒充本 FR 手写加深。
- 不得改写 FR92/100/103/112/119/126「已关闭」为失败（NFR56）。
- 不得因「终局」口号静默吞并 **NFR59**（未列入协议仍须新合同）。
- 不得冒充 **NFR14-crates**（本门禁 = NFR14 / AD-28）。
- 不得将手写 FL 运行时依赖塞进设计 crate（须保持 `bitloom-sim` / 工具链侧）。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR56** / **NFR57** / **NFR59**。
- 消歧：**NFR14-crates**（crates.io 名占用）≠ 本门禁。

---

### 新增协议清单（超 Gpio；≥1 可验收）

| 角色 | 协议 / IP | 74.2 义务 |
| --- | --- | --- |
| **选定（MVP）** | **`UartTx`**（`bitloom_prelude::ip::UartTx`） | 交付手写 FL ≡ `Sim::tick`（或等价对拍）+ ATDD；**超出** FR126 `Gpio` |
| 候选（未强制） | `UartRx` / `Spi*` / `I2c*` / `Axi*` | 可在 74.2 同批加深；否则仍 **NFR59** |
| 已关闭对照 | `Gpio` / `GpioFunctional`（FR126） | **仍有效；alone ≠ FR135** |
| 已关闭对照 | `SyncFifo` 手写（FR103） | **仍有效；alone ≠ FR135** |

### 验收谓词（F1–F3）与 GeneratedFunctional / 手写边界

| ID | 钉死 |
| --- | --- |
| **F1 手写 FL + API** | 选定协议（至少 **`UartTx`**）手写功能模型（例：`UartTxFunctional` 或等价）+ 钉死刺激夹具 + `IpDualModelMatrix::verify_*_handwritten`（或等价公开 API）；落在 **`bitloom-sim`** |
| **F2 FL≡tick** | 在钉死架构端口上 Pass（FL ≡ `settle`+`tick` 或文档化等价）；故意错模型 → Fail；可检查 |
| **F3 边界 / ATDD** | ATDD 覆盖 F1–F2；**≠ GeneratedFunctional alone**；**≠ Gpio / FR126 alone**；**≠ SyncFifo / FR103 alone**；关闭条件 = 清单 ≥1 协议落地 + F1–F3，**≠** docs-only |

**GeneratedFunctional / 手写边界：**

| 层 | 完成面 | FR135 关系 |
| --- | --- | --- |
| **FR103 GeneratedFunctional** | UART/SPI/I2C/AXI 生成功能路径 ≡ tick | **仍有效；alone ≠ FR135** |
| **FR103 SyncFifo 手写** | `SyncFifoFunctional` | **仍有效；alone ≠ FR135** |
| **FR126 Gpio 手写** | `GpioFunctional` | **仍有效；alone ≠ FR135** |
| **FR135 F1–F3** | 超 Gpio **更多协议手写** FL ≡ tick | **本 epic** |

### 与 Epic 75/78 的 `ip/` 软序

- **Epic 78 → Epic 74：** 软序假设已满足（Epic 78 / FR139 closed @ `42cdffe`）。
- **Epic 74 vs Epic 75：** 同触 `bitloom-prelude` `ip/` 时建议串行；本 epic FL 主体在 `bitloom-sim`，仍避免与 75.2 pad 环并行改同一 prelude IP 文件而无协调。

### 未列入（NFR59）

- 未在上表「选定（MVP）」强制的其余协议手写 FL；全芯片 pad（→ Epic 75 / FR136）；任意第三方 IP FL 生态；以 GeneratedFunctional 替换全部手写义务。

### Epic 74 关闭条件（74.3 勾选）

- [ ] **74.2 / FR135：** F1–F3 产品路径 + ATDD（≥1 超 Gpio 协议）
- [ ] **文档 / deferred / Phase 16 故事清单指针**
- [ ] **禁止事项未触发**
- [ ] **品牌 / 依赖：** Bitloom；设计 crate → `bitloom-prelude`；FL 在工具链/`bitloom-sim`
- [ ] **FR126 / FR103 / FR112 / FR119 关闭仍有效**（NFR56）
