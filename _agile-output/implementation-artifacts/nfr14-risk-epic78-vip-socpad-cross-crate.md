# NFR14 风险记录 — Epic 78 VIP/SocPad 再细拆或跨 crate（FR139）

> **权威：** PRD NFR14；AD-28；Phase 16 **NFR56 / NFR57 / NFR58 / NFR59**；交付 **FR139**。  
> **前置：** Epic 72 **closed**（FR133；`1a726c8`）；FR131 `ip/` 协议拆分 **closed**（Epic 71 P1–P4）；FR98/108/120/128 **closed**。  
> **门禁：** 无本有效记录 ⇒ **不得**将 **78.2–78.3** 标 `ready`。  
> **隔离：** ≠ FR131 P1–P4 alone；≠ FR128 `GpioSocPad` D1–D4 alone；≠ docs-only。  
> **软序：** 建议 **Epic 78 先于 Epic 74/75**（同触 `bitloom-prelude` `ip/`）。

---

### 元数据

| 项 | 填写 |
| --- | --- |
| 覆盖 FR / Epic | FR139 / Epic 78；对照 FR131 / FR128 / FR120 / FR108 / FR98 |
| 记录日期 | 2026-09-11 |
| 状态 | **closed** — Story **78.3** |
| **选定加深子集** | **VIP/SocPad 再细拆与/或跨 crate 切分图（C1–C4）** |

### (a) 上游约束

- Epic 72 / FR133 已关闭；Phase 12–15 关闭仍有效（**NFR56**）；不得改写 FR94–132「已关闭」为失败。
- FR131 P1–P4（`ip/{uart,spi,i2c,axi,gpio,…}.rs` + `mod.rs` re-export）**仍有效**；alone ≠ FR139。
- FR128 `GpioSocPad` D1–D4 / FR120 `GpioVip` / FR108 / FR98 四类近 VIP **仍有效**（NFR56）。
- 现行公开路径：`bitloom_prelude::ip::*`（含 `Gpio` / `GpioVip` / `GpioSocPad` 等）；设计 crate 只依赖 **`bitloom-prelude`**（**AD-6**），除非本记录 / FR139 显式批准新边界并提供迁移说明（**NFR58**）。
- 品牌 Bitloom / `bitloom-*`；禁止发布 `rhdl` / `rhdl-bits`。
- **软序假设：** Epic 78 建议先于 Epic 74（更多 IP FL）与 Epic 75（全芯片 pad），避免同触 `ip/` 冲突。

### (b) 粗工期带

- Epic 78：约 **1–3 人周**（78.1 ≤0.25；78.2 0.75–2.5；78.3 ≤0.5）。置信度：中。
- 假设：默认优先 prelude 内 VIP/SocPad 子模块细拆；跨 crate 仅在 C2 合同勾选后执行。

### (c) 禁止的静默降级清单

- 不得缺本记录将 **78.2–78.3** 标 `ready`。
- **不得仅 FR131 P1–P4（协议模块拆）关闭 FR139。**
- **不得 silent 改导出**（删除/改名 `bitloom_prelude::ip::*` 公开路径而无迁移说明）。
- **不得未文档化即破坏设计 crate 只依赖 `bitloom-prelude`（AD-6）**；跨 crate 须本记录 C2 显式批准 + 迁移说明 + 脊柱/文档同步（**NFR58**）。
- **不得仅 FR128 `GpioSocPad` D1–D4 alone / FR120 alone 关闭 FR139。**
- **不得 docs-only**（仅评估 defer ≠ 拆分/搬迁本身）。
- 不得改写 FR98/108/120/128/131「已关闭」为失败（NFR56）。
- 不得因「终局」口号静默吞并 **NFR59**（未列入更深 IP 布局仍须新合同）。
- 不得冒充 **NFR14-crates**（本门禁 = NFR14 / AD-28）。

### (d) 负责人

- Richard（Dev）— **NFR14** / **NFR56** / **NFR57** / **NFR58**（crate 边界 / AD-6 同步）/ **NFR59**。
- 消歧：**NFR14-crates**（crates.io 名占用）≠ 本门禁。

---

### 切分图（C1–C4）

| ID | 钉死 |
| --- | --- |
| **C1 再细拆** | 超出 FR131 协议层：将 VIP / SocPad（至少 `GpioVip` / `GpioSocPad`，可含基线 `Gpio`）从单体 `ip/gpio.rs` 再细拆为子模块（例：`ip/gpio/{mod,base,vip,socpad}.rs` 或等价），`mod.rs` / 子 `mod` **re-export** 保持可检查；切分本身可检查 ≠ 仅文档评估 |
| **C2 跨 crate（可选合同）** | 若搬出 `bitloom-prelude`：须新 crate 名、依赖方向相对 **AD-6** 的显式合同、设计 crate 迁移路径与文档；**未勾选 C2 则默认保留 prelude 内细拆**，不得 silent 跨 crate |
| **C3 公开 API / 迁移义务** | `bitloom_prelude::ip::{Gpio,GpioVip,GpioSocPad,…}`（及 FR98 四类等既有路径）**稳定**，或提供迁移说明（deprecate → 新路径；禁止 silent 断导出） |
| **C4 回归义务** | 至少：**FR98** 四类（UART/SPI/I2C/AXI）+ **FR108** + **FR120** + **FR128** + **FR131**；`bitloom-prelude` `--lib` / 既有 IP ATDD 绿；关闭条件 = C1（及若选则 C2）落地 + C3 + C4，**≠** FR131 P1–P4 alone |

### 与 Epic 74/75 的软序

- **Epic 78 → Epic 74 / Epic 75**：本 epic 建议先行（共享 `ip/`）；74/75 NFR14 须引用本假设，避免并行改同一文件树。

### 未列入（NFR59）

- 多外设全芯片 pad 环产品化（→ Epic 75 / FR136）；更多 IP 手写 FL 加深（→ Epic 74 / FR135）；任意第三方 IP crate 生态；恢复 Scala `Parser.parse`（→ Epic 77）。

### Epic 78 关闭条件（78.3 勾选）

- [x] **78.2 / FR139：** C1（及若选 C2）+ C3 + C4 产品路径 + ATDD
- [x] **文档 / deferred / Phase 16 故事清单指针**
- [x] **禁止事项未触发**
- [x] **品牌 / 依赖：** Bitloom；AD-6 边界或本记录批准的迁移合同（NFR58）
- [x] **FR131 / FR128 / FR120 关闭仍有效**（NFR56）
