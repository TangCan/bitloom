# NFR14 风险记录 — Epic 71 `ip.rs` 按协议拆分（FR131）

> **前置：** Epic 64 closed；FR98/108/120 closed；`ip.rs` ~3213 LOC。  
> **门禁：** 无本记录 ⇒ 不得标 71.2–71.3 ready。

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR131 / Epic 71 |
| 记录日期 | 2026-09-10 |
| 状态 | **closed** — Story **71.3** |

### (a) 上游约束

- 公开 API `bitloom_prelude::ip::*` 必须保持稳定。
- 回归：FR98 四类 + FR108 + FR120。
- 设计 crate 只依赖 `bitloom-prelude`；品牌 Bitloom。

### (b) 粗工期带

- 0.5–2 人周。

### (c) 禁止的静默降级清单

- 不得缺本记录标 71.2–71.3 ready。
- **不得仅文档评估关闭。**
- **不得破坏双模型/近 VIP。**
- **不得 silent 改导出。**
- 不得冒充 NFR14-crates。

### (d) 负责人

- Richard — NFR14 / NFR52 / NFR55。消歧：NFR14-crates ≠ 本门禁。

### 切分图（P1–P4）

| ID | 模块 |
| --- | --- |
| P1 | `ip/{sync_fifo,uart,spi,i2c,axi,gpio,blackbox,crc}.rs` + `mod.rs` re-export |
| P2 | `pub use` 保持 `bitloom_prelude::ip::GpioVip` 等路径 |
| P3 | FR98/108/120 + prelude `--lib` 回归绿 |
| P4 | 拆分本身 = 关闭条件（≠ 仅评估 defer） |

### Epic 71 关闭条件（71.3）

- [x] **71.2 / FR131**
- [x] **文档 / deferred**
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖**
- [x] **Phase 15 故事清单指针**
