# NFR14 风险记录 — Epic 75 多外设 / 全芯片 pad 环（FR136）

> **权威：** PRD NFR14；AD-28；Phase 16 **NFR56 / NFR57 / NFR59**；交付 **FR136**。  
> **前置：** Epic 72 **closed**（FR133；`1a726c8`）；Epic 78 **closed**（FR139；`42cdffe` — 软序满足）；Epic 74 **closed**（FR135；`d14ab99` — 同触 `ip/` 串行假设满足）；FR128 `GpioSocPad` D1–D4 **closed**（Epic 68）；FR108 / FR120 **closed**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **75.2–75.3** 标 `ready`。  
> **隔离：** ≠ FR108 alone；≠ FR120 C1–C4 alone；≠ FR128 D1–D4 alone；≠ docs-only。  
> **软序：** Epic **78 → 74/75**（同触 `ip/`）；Epic 78 / 74 已关闭；本 epic 可开工。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR136 / Epic 75；对照 FR128 / FR120 / FR108 / FR98 |
| 记录日期 | 2026-09-11 |
| 状态 | **closed** — Story **75.3**（R1–R4 @ 75.2；文档/deferred/Phase 16 指针收口） |
| **选定加深子集** | **多外设 pad 环 + 全芯片环形状（R1–R4）** |

### (a) 上游约束

- Epic 72 / FR133 已关闭；Phase 12–15 关闭仍有效（**NFR56**）；不得改写 FR94–132「已关闭」为失败。
- FR128 `GpioSocPad` D1–D4（双 bank×8＝16、降沿 IRQ、AXI CSR、对拍 ATDD）**仍有效**；alone ≠ FR136。
- FR120 `GpioVip` C1–C4 / FR108 `Gpio` / FR98 四类近 VIP **仍有效**；alone ≠ FR136。
- 设计 crate 只依赖 **`bitloom-prelude`**（AD-6）；pad / SoC 环 API 落在 prelude `ip/`（现行 `ip/gpio/socpad.rs`；FR139 拆分后路径稳定）。
- 品牌 Bitloom / `bitloom-*`；禁止发布 `rhdl` / `rhdl-bits`。
- **软序：** Epic 78（FR139 `ip/gpio/{base,vip,socpad}`）与 Epic 74（FR135 手写 FL，主体在 `bitloom-sim`）均已关闭；本 epic 可开工，避免与未关闭的 `ip/` 并行冲突。

### (b) 粗工期带

- Epic 75：约 **1–3 人周**（75.1 ≤0.25；75.2 0.75–2.5；75.3 ≤0.5）。置信度：中。
- 假设：75.2 交付下表 **R1–R4**；未列入更广 pad/外设仍 **NFR59**。

### (c) 禁止的静默降级清单

- 不得缺本记录将 **75.2–75.3** 标 `ready`。
- **不得仅 FR108 关闭 FR136。**
- **不得仅 FR120 C1–C4 关闭 FR136。**
- **不得仅 FR128 D1–D4（GpioSocPad）关闭 FR136。**
- **不得 docs-only**（仅评估 defer ≠ 多外设/全芯片 pad 环本身）。
- 不得改写 FR98/108/120/128「已关闭」为失败（NFR56）。
- 不得因「终局」口号静默吞并 **NFR59**（未列入更广 pad/外设仍须新合同）。
- 不得冒充 **NFR14-crates**（本门禁 = NFR14 / AD-28）。
- 不得将 pad 环运行时 / 仿真专用依赖塞进设计 crate 之外的偷偷路径而破坏「设计 crate → `bitloom-prelude` only」。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR56** / **NFR57** / **NFR59**。
- 消歧：**NFR14-crates**（crates.io 名占用）≠ 本门禁。

---

### Pad 环范围（多外设集合与/或全芯片环形状）

| 角色 | 钉死 | 75.2 义务 |
| --- | --- | --- |
| **选定（MVP）多外设** | **≥2 外设族** 共享/协调同一 pad 环 API（例：`ChipPadRing` / `MultiPeripheralPadRing` 或扩展 `GpioSocPad`）：至少 **GPIO（SocPad 面）+ UART（例 `UartTx` pad 侧）** | 公开可 elaboratable 路径 + 文档化端口集合；**超出** 单一 `GpioSocPad` |
| **选定（MVP）全芯片环形状** | **超出 FR128 双 bank×8＝16**：≥**3** bank **或** 环宽 ≥**24** 的全芯片环拓扑 + 公开 bank/脚位索引 | 形状可检查；≠ 仅改名双 bank 16 |
| 候选（未强制） | SPI/I2C/AXI pad 复用；驱动强度 / debounce / 模拟差分 | 可同批加深；否则仍 **NFR59** |
| 已关闭对照 | `GpioSocPad` D1–D4（FR128） | **仍有效；alone ≠ FR136** |
| 已关闭对照 | `GpioVip` C1–C4（FR120） / `Gpio`（FR108） | **仍有效；alone ≠ FR136** |

### 断言/对拍深度与验收谓词（R1–R4）

| ID | 钉死 |
| --- | --- |
| **R1 多外设集合** | 环 API 暴露 ≥2 外设族 pad 端口（GPIO + UART）；可 elaboratable；设计 crate 仅依赖 `bitloom-prelude` |
| **R2 全芯片环形状** | 超出 FR128 双 bank 16：≥3 bank 或宽度 ≥24 + 公开索引；形状 ATDD 可断言 |
| **R3 断言/对拍深度** | 环级记分板或对拍：期望 vs 观测 **pad 序列**，且至少覆盖 **一外设副作用**（例 UART 位流 / GPIO `pad_out`）；故意错模型 → Fail；可检查 |
| **R4 ATDD / 边界** | ATDD 覆盖 R1–R3；**≠ FR128 D1–D4 alone**；**≠ FR120 C1–C4 alone**；**≠ FR108 alone**；关闭条件 = R1–R4，**≠** docs-only |

### 与 FR128 D1–D4 边界

| 层 | 完成面 | FR136 关系 |
| --- | --- | --- |
| **FR128 D1** | 双 bank pad 环 `GpioSocPad`（bank0=[7:0]、bank1=[15:8]；宽 16） | **仍有效；alone ≠ FR136** |
| **FR128 D2** | 降沿 IRQ | **仍有效；alone ≠ FR136** |
| **FR128 D3** | AXI 风格 CSR 窗 | **仍有效；alone ≠ FR136** |
| **FR128 D4** | 对拍记分板 ATDD（单 `GpioSocPad` `pad_out`） | **仍有效；alone ≠ FR136**（本 FR 须 **环级/多外设** 加深对拍） |
| **FR136 R1–R4** | 多外设集合 + 全芯片环形状 + 加深断言/对拍 | **本 epic** |

### 与 Epic 74/78 的 `ip/` 软序

- **Epic 78 → Epic 75：** 软序假设已满足（Epic 78 / FR139 closed @ `42cdffe`）。
- **Epic 74 → Epic 75：** 软序 / 串行假设已满足（Epic 74 / FR135 closed @ `d14ab99`）；74 主体在 `bitloom-sim`，75.2 预计改 prelude `ip/` — 仍避免无协调并行改同一 `ip/gpio/**` 文件。

### 未列入（NFR59）

- 未在上表「选定（MVP）」强制的其余外设 pad 复用；驱动强度 / debounce / 模拟差分；第三方 VIP 二进制对拍；任意全芯片物理封装库；以 FR128 D1–D4 alone 宣称全芯片完成。

### Epic 75 关闭条件（75.3 勾选）

- [x] **75.2 / FR136：** R1–R4 产品路径 + ATDD（多外设 + 全芯片形状 + 加深对拍）
- [x] **文档 / deferred / Phase 16 故事清单指针**
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖：** Bitloom；设计 crate → `bitloom-prelude`
- [x] **FR128 / FR120 / FR108 关闭仍有效**（NFR56）
