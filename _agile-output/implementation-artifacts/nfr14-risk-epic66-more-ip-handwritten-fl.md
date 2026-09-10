# NFR14 风险记录 — Epic 66 更多 IP 手写 FL（FR126）

> **前置：** Epic 64 closed；FR103/FR112-B/FR119 closed。  
> **门禁：** 无本记录 ⇒ 不得标 66.2–66.3 ready。

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR126 / Epic 66 |
| 记录日期 | 2026-09-10 |
| 状态 | **closed** — Story **66.3** |
| **选定 IP** | **`Gpio` 手写 FL**（`GpioFunctional` ≡ tick） |

### (a) 上游约束

- FR103 SyncFifo 手写 + UART/SPI/I2C/AXI GeneratedFunctional **仍有效**；alone ≠ FR126。
- FR112 MemRead≡tick / FR119 sby alone ≠ FR126。
- 设计依赖 `bitloom-prelude`；FL 在 `bitloom-sim`。
- 品牌 Bitloom。

### (b) 粗工期带

- 0.75–3 人周。置信度：中。

### (c) 禁止的静默降级清单

- 不得缺本记录标 66.2–66.3 ready。
- **不得仅 FR92 记分板关闭。**
- **不得仅 FR100 F1-(i) 关闭。**
- **不得仅 MemRead≡tick / FR112-B 关闭。**
- **不得仅 sby 绑定 / FR119 关闭。**
- 未列入 IP 仍 deferred（NFR55）。
- 不得改写 FR103/112/119 为失败（NFR52）。
- 不得冒充 NFR14-crates。

### (d) 负责人

- Richard — NFR14 / NFR52 / NFR55。

### 验收谓词（F1–F3）

| ID | 钉死 |
| --- | --- |
| F1 | `GpioFunctional` + `gpio_dual_stimulus` + `IpDualModelMatrix::verify_gpio_handwritten` |
| F2 | Pass on arch ports `pad_out`/`rd_data`；故意错模型 → Fail |
| F3 | ATDD；≠ GeneratedFunctional alone；≠ SyncFifo alone |

### Epic 66 关闭条件（66.3）

- [x] **66.2 / FR126**
- [x] **文档 / deferred / README**
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖**
- [x] **FR112-B / FR119 关闭仍有效**
