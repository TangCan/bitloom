# NFR14 风险记录 — Epic 68 全 SoC pad / 商业对拍（FR128）

> **前置：** Epic 64 closed；FR120 / Epic 61 closed；Epic 71 / FR131 closed（软序）。  
> **门禁：** 无本记录 ⇒ 不得标 68.2–68.3 ready。

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR128 / Epic 68；NFR14 / NFR52 / NFR55 |
| 记录日期 | 2026-09-10 |
| 状态 | **closed** — Story **68.3** |

### (a) 上游约束

- FR120 `GpioVip` C1–C4 **仍有效**（NFR52）；alone ≠ FR128。
- FR108 `Gpio` / FR98 四类近 VIP 回归不破。
- 设计 crate 只依赖 `bitloom-prelude`；实现落在 `ip/gpio.rs`（FR131 拆分后）。
- 品牌 Bitloom。

### (b) 粗工期带

- 1–3 人周。

### (c) 禁止的静默降级清单

- 不得缺本记录标 68.2–68.3 ready。
- **不得仅 FR108。**
- **不得仅 GpioVip C1–C4。**
- **不得 docs-only。**
- 不得改写 FR120 关闭为失败（NFR52）。
- 不得冒充 NFR14-crates。

### (d) 负责人

- Richard — NFR14 / NFR52 / NFR55。消歧：NFR14-crates ≠ 本门禁。

### 全 SoC pad 深度（D1–D4）

| ID | 钉死 |
| --- | --- |
| D1 | **双 bank pad 环** — `GpioSocPad`：bank0=[7:0]、bank1=[15:8]；`pad_in`/`pad_out`/`dir`/`rd_data` 宽 16 |
| D2 | **降沿 IRQ** — `irq_fall_en` / `irq_fall_status`；与上升沿并存；`irq_out` = rise∨fall armed |
| D3 | **AXI 风格 CSR 窗** — `csr_wen`/`csr_addr`/`csr_wdata`/`csr_rdata`（addr0=out 写；读回 `rd_data`/`irq_status`） |
| D4 | **对拍记分板 ATDD** — 期望 vs 观测 `pad_out` 序列；`cargo test -p bitloom --test fr128_soc_pad` |

### 明确非目标（NFR55）

Debounce / 驱动强度 / 模拟差分；第三方 VIP 二进制对拍；电平敏感全家桶（本 FR 钉降沿增量，非全模式库）。

### Epic 68 关闭条件（68.3）

- [x] **68.2 / FR128**
- [x] **文档 / deferred**
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖**
- [x] **FR120 关闭仍有效**
